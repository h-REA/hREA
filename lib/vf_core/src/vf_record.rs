/**
 * Base ValueFlows record fields, behaviours & helper macros
 *
 * Shared by all VF record types.
 */

#[macro_export]
macro_rules! vfRecord {
    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        // setup base traits
        #[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
        pub struct $name {

            // inject common fields shared by all records
            id: Option<Address>,
            note: Option<String>,

            $($field_name: $field_type,)*
        }
    };
}
