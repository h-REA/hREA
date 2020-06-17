// see types of prompts:
// https://github.com/enquirer/enquirer/tree/master/examples
//
module.exports = [
  {
    type: 'input',
    name: 'dna_path',
    message: 'Project-relative directory name of the destination DNA? (eg. `happs/observation`)',
    required: true,
  }, {
    type: 'input',
    name: 'local_dna_name',
    message: 'Name of the local DNA? (eg. `observation`) ...sorry, kinda duplicate question...',
    required: true,
  }, {
    type: 'input',
    name: 'foreign_record_name',
    message: 'Name of the foreign record type in the remote DNA? (eg. `Economic Event`)',
    required: true,
  }, {
    type: 'input',
    name: 'local_record_name',
    message: 'Name of the record type in the local DNA? (eg. `Agreement`)',
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
  },
]
