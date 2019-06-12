/**
 * Query by tag, manipulate as a field, push changes, and more.
 * @author David Hand (sqykly@gmail.com)
 */

#[macro_use]
use hdk;
#[macro_use]
use hdk::api as api;
use hdk::error::ZomeApiResult;

use crate::link_rules::LinkRules;
use crate::HoloPtr;
use crate::Tag;
use crate::with_entry;

use hdk::holochain_core_types::entry::Entry;

/// The elements of a LinkSet are given during the filter operation as this struct, which can load
/// the data you're interested in without a goofy parameter.
pub struct Linked {
    addr: HoloPtr,
}


/// When you filter a LinkSet, you will need to return one of these.
pub enum TakeOrLeave {
    /// New set should keep this element
    Take,
    /// New set should not contain this element, but leave the link on the DHT
    Leave,
    /// Remove this element from the DHT and from the set
    Remove,
    /// Remove this element and replace it with another
    Replace(HoloPtr)
}

/// An element of a LinkSet that is undergoing a filter operation via `select()`
impl Linked {

    /// ```
    /// Linked::of(addr)
    /// ```
    /// Construct a Linked from the address `addr`
    /// Returns a new `Linked`
    pub fn of(addr: HoloPtr) -> Linked {
        Linked { addr }
    }

    /// ```
    /// # let linked = Linked { addr: "whatevs" }
    /// let addr: Address = linked.hash();
    /// ```
    /// Returns the Address that is linked.
    pub fn hash(&self) -> HoloPtr {
        self.addr
    }

    /// ```
    /// let entry: ZomeApiResult<Option<Entry>> = linked.entry();
    /// ```
    /// returns the entry pointed to by this Linked.
    pub fn entry(&self) -> ZomeApiResult<Option<Entry>> {
        // When HoloPtr is defined, this is a fix me
        api::get_entry(&self.addr)
    }

    /// Return `linked.leave()` to omit the link from the `select`
    pub fn leave(&self) -> TakeOrLeave { TakeOrLeave::Leave }

    /// Return `linked.take()` to include the link in the `select`
    pub fn take(&self) -> TakeOrLeave { TakeOrLeave::Take }

    /// Return `linked.remove()` to remove the link from the DHT
    pub fn remove(&self) -> TakeOrLeave { TakeOrLeave::Remove }

    /// Return `linked.replace(address)` to replace this link with another Address in the selected
    /// set
    pub fn replace(&self, with: HoloPtr) -> TakeOrLeave { TakeOrLeave::Replace(with) }
}

/// The result of a query of links.  Mostly immutable, but there are methods for treating this
/// query as if mutating a field of a struct, `set()`, `add()`, and `push()`.  Apart from that
/// LinkSet does transformations like mapping and filtering specific to Holochain links.
pub struct LinkSet<'a> {
    /// The address of the object being linked _from_.
    pub base: HoloPtr,
    /// An identifier for the role of the link, i.e. how the base is related to the targets.
    pub tag: Tag,
    hashes: Vec<HoloPtr>,
    /// A `LinkRules` object that maintains the tag's relationships
    pub rules: &'a LinkRules
}

impl<'a> LinkSet<'a> {

    /// `LinkSet::new(base, tag, rules, hashes)`
    /// Constructs a new LinkSet from scratch.  There is no reason to do this from outside this
    /// file.
    pub fn new(base: HoloPtr, tag: Tag, rules: &'a LinkRules, hashes: Vec<HoloPtr>) -> LinkSet<'a> {
        LinkSet {
            base,
            tag: tag,
            rules,
            hashes: hashes.clone()
        }
    }

    /// `let links: LinkSet = LinkSet::load(base, tag, rules)`
    /// Queries the DHT for links from the Address `base` with the string `tag`.
    /// `rules` is a `LinkRules` object that holds the structure of this and related tags.
    /// Maybe prefer the method on your `rules` object
    pub fn load(base: &HoloPtr, tag: &Tag, rules: &'a LinkRules) -> LinkSet<'a> {
        // Another type-swapping Fix Me
        LinkSet {
            base: *base,
            tag: *tag,
            hashes: match api::get_links(&base, *tag) {
                Ok(hashes) => { hashes.addresses().clone() }
                _ => { Vec::new() }
            },
            rules
        }
    }

    /// Clones this LinkSet with the hashes replaced with `hashes`.
    /// Don't use outside this module.
    fn but_with(&self, hashes: Vec<HoloPtr>) -> LinkSet<'a> {
        LinkSet {
            base: self.base,
            tag: self.tag,
            rules: self.rules,
            hashes
        }
    }

    /// ```let entries: Vec<Entry> = link_set.entries()```
    /// Retrieves the entries that are linked on this base by this tag.
    /// Entries which do not load successfully are omitted.
    pub fn entries(&self) -> Vec<Entry> {
        // type fix me
        let them: Vec<Entry> = Vec::new();

        for addr in self.hashes {
            with_entry!((&addr) {
                hit (entry) {
                    them.push(entry);
                }
                miss { (); }
                fail (dont_care) { (); }
            });
        }

        them
    }

    /// Iterates over the linked elements using the callback you supply and returns a new LinkSet
    /// as it specifies.  For each element, your callback receives an instance of Linked, from
    /// which you can load the `.hash()` or `.entry()`.  Return one of the following:
    /// - `linked.take()` to include the same link in the selection.
    /// - `linked.leave()` to omit the link from the selection.
    /// - `linked.replace(Address)` to omit this link, but replace it with another in the selection.
    ///     - Note that this doesn't affect the DHT until you `save()`.
    ///     - This will be present in the selection _even if it is not in the DHT_.
    /// - `linked.remove()` to omit this link _and_ remove it from the DHT.
    pub fn select(&self, cb: &Fn(Linked)->self::TakeOrLeave) -> LinkSet<'a> {
        let results: Vec<HoloPtr> = Vec::new();

        for addr in self.hashes.iter() {
            match cb(Linked::of(*addr)) {
                TakeOrLeave::Take => {
                    results.push(*addr);
                }
                TakeOrLeave::Remove => {
                    self.rules.unlink(&self.base, &self.tag, addr);
                }
                TakeOrLeave::Replace(with_addr) => {
                    results.push(with_addr);
                }
                TakeOrLeave::Leave => { ; }
            }
        }

        self.but_with(results)
    }

    /// Returns true if the LinkSet has a link from its base to the target address.
    /// It doesn't matter if the link is in the DHT; use `LinkSet::load(...).contains(...)` if
    /// that is your concern.
    pub fn contains(&self, target: &HoloPtr) -> bool {
        self.hashes.contains(target)
    }

    /// Returns an iterator over the addresses of the targets of the links.
    pub fn iter(&self) -> std::slice::Iter<HoloPtr> {
        self.hashes.iter()
    }

    /// Returns a Vec containing the addresses of the targets of these links.
    pub fn hashes(&self) -> Vec<HoloPtr> {
        self.hashes.clone()
    }

    /// Construct a new `LinkSet` containing the targets of this set's links that are also targets
    /// of another's links.  The result will have this object's base, tag, and rules, but the other
    /// object does not need to have these identical fields.
    pub fn and_in(&self, other: &LinkSet) -> LinkSet<'a> {
        let mut results = Vec::new();
        for addr in self.iter() {
            if other.contains(&addr) {
                results.push(*addr);
            }
        }

        self.but_with(results)
    }

    /// Construct a new `LinkSet` containing the targets of this set that are not in some other
    /// `LinkSet`.  The product will have the same base, tag, and rules as this one, but the other
    /// `LinkSet` may have any attributes.
    pub fn not_in(&self, other: &LinkSet) -> LinkSet<'a> {
        let mut results = Vec::new();
        for addr in self.iter() {
            if !other.contains(&addr) {
                results.push(*addr);
            }
        }

        self.but_with(results)
    }

    /// Don't use this.
    fn save_links(&self) {
        for addr in self.iter() {
            self.rules.link(&self.base, &self.tag, addr);
        }
    }

    /// Removes all links in this set from the DHT.  The targets are still stored in this object.
    pub fn remove_all(&self) {
        for addr in self.hashes.iter() {
            self.rules.unlink(&self.base, &self.tag, addr);
        }
    }

    /// Commit the links in this set, including whatever modifications led to it, to the DHT.  Note
    /// that both additions and deletions will be saved; these links will be the only links from
    /// this base with this tag.
    pub fn save(&self) -> &LinkSet {
        let current = LinkSet::load(&self.base, &self.tag, self.rules);
        self.not_in(&current).save_links();
        current.not_in(self).remove_all();

        self
    }

    /// Add an address to the `LinkSet` and return this object for chaining.
    pub fn push(&mut self, addr: HoloPtr) -> &Self {
        self.hashes.push(addr);

        self
    }

    /// ```
    /// while link_set.replace(remove_addr, add_addr) {}
    /// ```
    /// Replace one link target, with `Address` `remove_addr`, with `add_addr` in this set.
    /// Returns `true` if `remove_addr` was found in the set.
    pub fn replace(&mut self, rem: HoloPtr, add: HoloPtr) -> bool {

        for (i, addr) in self.hashes.iter_mut().enumerate() {
            if *addr == rem {
                *addr = add;
                return true;
            }
        }

        false
    }

    /// After `link_set.set(to: Address)`, `to` will be the only link target in the set.  `.save()`
    /// is required to commit the scenario to the DHT.
    pub fn set(&mut self, to: &HoloPtr) -> &LinkSet {
        self.hashes = vec![to.clone()];

        self
    }

    /// Append the targets of another `LinkSet` to this one.  The DHT is not yet affected.
    pub fn add(&mut self, other: &LinkSet) -> &LinkSet {
        let common = self.and_in(&other);
        let other_xor = other.not_in(&common);
        let mut all = self.hashes.clone();
        all.append(&mut other_xor.hashes());
        //self.iter().chain(other_xor.iter()).map(|a| a).collect();
        self.hashes = all;

        self
    }

    pub fn save_as(&self, tag: &Tag) -> LinkSet<'a> {
        let foo = LinkSet::new(self.base.clone(), tag.clone(), self.rules, self.hashes.clone());
        foo.save();
        // No way did that just work.  That can't be right.
        foo
    }

    pub fn len(&self) -> usize {
        self.hashes.len()
    }
}
