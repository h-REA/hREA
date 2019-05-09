/**
 * Base ValueFlows record fields, behaviours & helper macros
 *
 * Shared by all VF record types.
 */

#[macro_export]
macro_rules! vfRecord {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        // setup base traits
        #[derive(Debug, Default, Clone)]
        struct $name {

            // inject common fields shared by all records
            id: Option<Address>,
            note: Option<String>,

            $($field_name: $field_type,)*
        }
    };
}
