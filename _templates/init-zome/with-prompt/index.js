
// see types of prompts:
// https://github.com/enquirer/enquirer/tree/master/examples
//
module.exports = [
  {
    type: 'input',
    name: 'zome_name',
    message: 'Name of the new zome? (eg. `rea_economic_event`)',
    required: true,
  }, {
    type: 'input',
    name: 'zome_friendly_name',
    message: 'Human-readable short name for the zome, to use in file comments (eg. "Holo-REA economic event")',
    required: true,
  }, {
    type: 'input',
    name: 'package_author_name',
    message: 'Initial author name for published Rust crate?',
    required: true,
  }, {
    type: 'input',
    name: 'package_author_email',
    message: 'Initial author email address for published Rust crate?',
    required: true,
  }, {
    type: 'input',
    name: 'record_primary_key_type',
    message: 'Type ID used for record primary keys (eg. `EventAddress`)- see vf_core/type_aliases.rs',
    required: true,
  }, {
    type: 'input',
    name: 'record_response_attr',
    message: 'Attribute used in the response payload that returns the record data? (eg. `economic_event`)',
    required: true,
  },
]
