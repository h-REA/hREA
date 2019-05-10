use hdk::holochain_core_types::{
    cas::content::Address,
};

vfRecord! {
    struct Location {
        name: String,
        address: Option<String>,
        latitude: Option<f32>,
        longitude: Option<f32>,
    }
}
