/**
 * Rust version of the original LinkRepo library.
 * @author David Hand (sqykly@gmail.com)
 */

use std::collections::{ HashMap, HashSet };

#[macro_use]
use hdk;
#[macro_use]
use hdk::api as api;

use crate::link_set::LinkSet;
use crate::{
    HoloPtr,
    Tag
};

struct PredicateRule {
    query: Tag,
    dependent: Tag
}

/// A LinkRules object represents a coherent system
/// of links and the rules that govern the relationships between tags therein.  The rules are
/// declared once, then the object will do the boiler plate for you when you add and delete links.
pub struct LinkRules {
    reciprocals: HashMap<Tag, Vec<Tag>>,
    singulars: HashSet<Tag>,
    predicates: HashMap<Tag, Vec<PredicateRule>>,
    rec_guard: HashSet<String>
}

macro_rules! guard {
    (($rec_guard:expr, $base:expr, $op:expr, $tag:expr, $link:expr) $what_do:block) => {
        let desc = format!("{} {}{} {}", $base, $op, $tag, $link);
        if !$rec_guard.contains(&desc) {
            $rec_guard.insert(desc.clone());
            $what_do;
            $rec_guard.remove(&desc);
        }
    }
}

impl LinkRules {
    pub fn new() -> LinkRules {
        LinkRules {
            reciprocals: HashMap::new(),
            singulars: HashSet::new(),
            predicates: HashMap::new(),
            rec_guard: HashSet::new(),
        }
    }

    // This method may be a no-go.
    /// Create a LinkSet (a query) that, if mutated, will observe the rules encapsulated in this
    /// object.
    /// `HoloPtr base`: The subject of the links to query
    /// `&Tag tag`: The tag or type of link to query
    /// returns `LinkSet`
    pub fn load(&mut self, base: &HoloPtr, tag: &Tag) -> LinkSet {
        LinkSet::load(base, tag, self)
    }

    fn get_origin(&self, of: &HoloPtr) -> HoloPtr {
        let initial = LinkSet::load(of, &"initial_entry", self);
        if initial.len() > 0 {
            initial.hashes()[0].clone()
        } else {
            of.clone()
        }
    }

    /// Create a link from an entry.  If the rules of this object specify any additional action for
    /// a valid link as specified, it will be done for you.
    /// `&HoloPtr base`: the subject of the link
    /// `&Tag tag`: the link type
    /// `&HoloPtr` target: the object of the link.
    /// returns `&self` for chaining, e.g. `rules.link(...).link(...).unlink(...)` etc.
    pub fn link(&mut self, base: &HoloPtr, tag: &Tag, target: &HoloPtr) -> &Self {
        let base_id = self.get_origin(base);
        let target_id = self.get_origin(target);

        //self.guard(&base_id, '+', tag, &target_id, &|| {
        guard!((self.rec_guard, &base_id, '+', tag, &target_id) {

            if self.singulars.contains(tag) {
                LinkSet::load(&base_id, tag, self).remove_all();
            }

            api::link_entries(&base_id, &target_id, *tag);

            let Self {predicates, reciprocals, ..} = self;

            match predicates.get(tag) {
                Some(preds) => {
                    for PredicateRule {query, dependent} in preds {
                        let subj: LinkSet = self.load(&target_id, query);
                        //LinkSet::load(&target_id, query, self);
                        for obj in subj.hashes() {
                            self.link(&base_id, dependent, &obj);
                        }
                    }
                }
                None => {}
            }

            match reciprocals.get(tag) {
                Some(recip_tags) => {
                    for recip in recip_tags.iter() {
                        self.link(&target_id, recip, &base_id);
                    }
                }
                None => {}
            }

        });

        self
    }

    /// Remove the specified link from the DHT and clean up any links that were added as a result
    /// of the rules for this object.  Reciprocals will be removed, predicated links will be
    /// removed automagically.
    /// `&HoloPtr base`: the subject of the link to remove.
    /// `&Tag tag`: the type of the link to remove.
    /// `&HoloPtr target`: the object of the link to remove.
    /// returns `&self` for chaining.
    pub fn unlink(&mut self, base: &HoloPtr, tag: &Tag, target: &HoloPtr) -> &Self {
        let base_id = self.get_origin(base);
        let target_id = self.get_origin(target);

        guard!((self.rec_guard, &base_id, '-', tag, &target_id) {

            api::remove_link(&base_id, &target_id, *tag);

            let Self {predicates, reciprocals, ..} = self;

            match predicates.get(tag) {
                Some(preds) => {
                    for PredicateRule { query, dependent } in preds.iter() {
                        let sibs: LinkSet = self.load(&target_id, query);   //LinkSet::load(&target_id, query, self);
                        for addr in sibs.hashes().iter() {
                            self.unlink(&base_id, dependent, addr);
                        }
                    }
                }
                None => {}
            }

            match reciprocals.get(tag) {
                Some(reciprocal) => {
                    for recip_tag in reciprocal.iter() {
                        self.unlink(&target_id, recip_tag, &base_id);
                    }
                }
                None => {}
            }
        });

        self
    }

    /// Declare a one-way reciprocal.  This means that `object_A tag object_B` will trigger
    /// `object_B recip object_A`, but `object_B recip object_A` will not trigger
    /// `object_A tag object_B`.  I personally see few use cases, but maybe I'm wrong.
    /// `&Tag tag`: the tag that _does_ trigger its reciprocal
    /// `&Tag recip`: the tag that will be triggered on the target, but won't trigger `tag` itself.
    /// returns `&self`
    pub fn reciprocal_one_way(&mut self, tag: &Tag, recip: &Tag) -> &LinkRules {
        match self.reciprocals.get_mut(tag) {
            None => {
                self.reciprocals.insert(tag, vec![recip]);
            }
            Some(existing) => {
                existing.push(recip);
            }
        }

        self
    }

    /// Declare a tag to have a reciprocal tag from the object back to the subject.
    ///
    /// More specifically:
    /// - All (`B`, `A`) where `A tag B` => `B recip A`
    /// - All (`A`, `B`) where `B recip A` => `A tag B`
    /// `&Tag tag`: The forward tag from subject to object
    /// `&Tag recip`: The reciprocal tag from object to subject
    /// returns `&self` for chaining.
    pub fn reciprocal(&mut self, tag: &Tag, recip: &Tag) -> &LinkRules {
        self.reciprocal_one_way(tag, recip);
        self.reciprocal_one_way(recip, tag)
    }

    /// Declare a tag to have only object at a time:
    /// - All (`A`, `B`, `C`) where `A tag B` => `A !tag C`
    ///
    /// `&Tag tag`: The tag that can only have one object
    /// returns `&self` for chaining
    pub fn singular(&mut self, tag: &Tag) -> &LinkRules {
        if !self.singulars.contains(tag) {
            self.singulars.insert(tag);
        }

        self
    }

    /// Declares a rule that applies to entries _related to_ the object.  The most familiar example
    /// is that if `A child_of B`, anything else `C child_of B` must also be `A sibling C`.  More
    /// generally:
    /// - All (`A`, `B`, `C`) where `A tag B` & `B query C` => `A dependent C`
    ///
    /// `&Tag tag`: The application of `tag` will trigger this arrangement to play out.
    /// `&Tag query`: The tag used to find the target entries from the object of the link.
    /// `&Tag dependent`: The tag that will link the subject of `tag` to the object of `query`.
    /// returns `&self` for chaining.
    pub fn predicate(&mut self, tag: &Tag, query: &Tag, dependent: &Tag) -> &LinkRules {
        match self.predicates.get_mut(tag) {
            Some(existing) => {
                existing.push(PredicateRule {
                    query: query,
                    dependent: dependent
                });
            }
            None => {
                self.predicates.insert(tag, vec![PredicateRule {
                    query: query,
                    dependent: dependent
                }]);
            }
        };

        self
    }
}
