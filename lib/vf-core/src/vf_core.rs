/**
 * Core types for the ValueFlows system
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-02-06
 */

/**
 * VfEntry is the base class for entities that have to do with VF.
 * The standard says that there are a few fields that any object could have.
 */
#[derive(Debug, Default, Clone)]
pub struct VfEntry {
  name: Option<String>,
  image: Option<String>,
  note: Option<String>,
  url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_creation() {
        let e: VfEntry = VfEntry { name: Some("Billy".into()), ..VfEntry::default() };
        assert_eq!(e.name, Some("Billy".into()))
    }
}
