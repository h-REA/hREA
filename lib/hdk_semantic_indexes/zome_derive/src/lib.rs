/**
 * Derive macro for index zome code generator.
 *
 * Generates a complete, self-contained zome def.
 *
 * @package hdk_semantic_indexes
 * @author  pospi <pospi@spadgos.com>
 * @since   2021-10-10
 */

extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input,
    AttributeArgs,
    Data, DataStruct, DeriveInput,
    Fields, Type, TypePath, PathSegment,
    PathArguments::AngleBracketed,
    AngleBracketedGenericArguments, GenericArgument,
    punctuated::Punctuated, token::Comma,
};
use darling::FromMeta;
use convert_case::{Case, Casing};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    query_fn_name: Option<String>,
}

#[proc_macro_attribute]
pub fn index_zome(attribs: TokenStream, input: TokenStream) -> TokenStream {
    let raw_args = parse_macro_input!(attribs as AttributeArgs);
    let args = match MacroArgs::from_list(&raw_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let input = parse_macro_input!(input as DeriveInput);
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    // build toplevel variables for generated code
    let record_type = &input.ident;
    let record_type_str_attribute = record_type.to_string().to_case(Case::Snake);

    let record_type_index_attribute = format_ident!("{}_index", record_type_str_attribute);
    let record_read_api_method_name = format_ident!("get_{}", record_type_str_attribute);

    let exposed_query_api_method_name = match &args.query_fn_name {
        None => format_ident!("query_{}s", record_type_str_attribute),
        Some(query_fn) => format_ident!("{}", query_fn),
    };
    let record_index_field_type = format_ident!("{}Address", record_type.to_string().to_case(Case::UpperCamel));

    // build iterators for generating index update methods and query conditions
    let all_indexes = fields.iter()
        .map(|field| {
            let relationship_name = field.ident.as_ref().unwrap().to_string().to_case(Case::Snake);

            let path = match &field.ty {
                Type::Path(TypePath { path, .. }) => path,
                _ => panic!("expected index type of Local or Remote"),
            };
            let (index_type, args) = match path.segments.first() {
                Some(PathSegment { arguments: AngleBracketed(AngleBracketedGenericArguments { args, .. }), ident, .. }) => (ident, args),
                _ => panic!("expected parameterised index with <related_record_type, relationship_name>"),
            };

            assert_eq!(args.len(), 2, "expected 2 args to index defs");
            let mut these_args = args.to_owned();

            let related_relationship_name: String = next_generic_type_as_string(&mut these_args).to_case(Case::Snake);
            let related_record_type: String = next_generic_type_as_string(&mut these_args);
            let related_index_field_type = format_ident!("{}Address", related_record_type.to_case(Case::UpperCamel));
            let related_index_name = format_ident!("{}_{}", record_type_str_attribute, relationship_name);
            let related_record_type_str_attribute = related_record_type.to_case(Case::Snake);
            let reciprocal_index_name = format_ident!("{}_{}", related_record_type_str_attribute, related_relationship_name);

            (
                index_type, relationship_name,
                related_record_type_str_attribute,
                related_index_field_type, related_index_name,
                reciprocal_index_name,
            )
        });

    let index_accessors = all_indexes.clone()
        .map(|(
            _index_type, relationship_name,
            _related_record_type_str_attribute,
            related_index_field_type, related_index_name,
            _reciprocal_index_name,
        )| {
            let local_dna_read_method_name = format_ident!("_internal_read_{}_{}", record_type_str_attribute, relationship_name);

            quote! {
                #[hdk_extern]
                fn #local_dna_read_method_name(ByAddress { address }: ByAddress<#record_index_field_type>) -> ExternResult<Vec<#related_index_field_type>> {
                    Ok(read_index(&stringify!(#record_type_str_attribute), &address, &stringify!(#related_index_name))?)
                }
            }
        });

    let index_mutators = all_indexes.clone()
        .map(|(
            index_type, relationship_name,
            related_record_type_str_attribute,
            related_index_field_type, related_index_name,
            reciprocal_index_name,
        )| {
            // :TODO: differentiate Local/Remote indexes as necessitated by final HC core APIs
            let dna_update_method_name = match index_type.to_string().as_ref() {
                "Local" => format_ident!("_internal_index_{}_{}", record_type_str_attribute, relationship_name),
                "Remote" => format_ident!("index_{}_{}", record_type_str_attribute, relationship_name),
                _ => panic!("expected index type of Local or Remote"),
            };

            quote! {
                #[hdk_extern]
                fn #dna_update_method_name(indexes: RemoteEntryLinkRequest<#related_index_field_type, #record_index_field_type>) -> ExternResult<RemoteEntryLinkResponse> {
                    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

                    Ok(sync_index(
                        &stringify!(#related_record_type_str_attribute), &remote_entry,
                        &stringify!(#record_type_str_attribute),
                        target_entries.as_slice(),
                        removed_entries.as_slice(),
                        &stringify!(#reciprocal_index_name), &stringify!(#related_index_name),
                    )?)
                }
            }
        });

    let query_handlers = all_indexes
        .map(|(
            _index_type, relationship_name,
            related_record_type_str_attribute,
            _related_index_field_type, _related_index_name,
            reciprocal_index_name,
        )| {
            let query_field_ident = format_ident!("{}", relationship_name);

            quote! {
                match &params.#query_field_ident {
                    Some(#query_field_ident) => {
                        entries_result = query_index::<ResponseData, #record_index_field_type, _,_,_,_,_,_>(
                            &stringify!(#related_record_type_str_attribute),
                            #query_field_ident,
                            &stringify!(#reciprocal_index_name),
                            &read_index_target_zome,
                            &READ_FN_NAME,
                        );
                    },
                    _ => (),
                };
            }
        });

    TokenStream::from(quote! {
        use hdk::prelude::*;
        use hdk_semantic_indexes_zome_lib::*;

        // unrelated toplevel zome boilerplate
        entry_defs![Path::entry_def()];

        // :TODO: obviate this with zome-specific configs
        #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
        pub struct DnaConfigSlice {
            pub #record_type_index_attribute: IndexingZomeConfig,
        }

        // zome properties access helper
        fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
            Some(conf.#record_type_index_attribute.record_storage_zome)
        }

        // define struct to wrap query parameter inputs, so that other meta-args (eg. pagination) can be added later
        #[derive(Debug, Serialize, Deserialize)]
        struct SearchInputs {
            pub params: QueryParams,
        }

        // define zome API function name to read indexed records
        const READ_FN_NAME: &str = stringify!(#record_read_api_method_name);

        // public zome API for reading indexes to determine related record IDs
        #(
            #index_accessors
        )*

        // public zome API for updating indexes when associated records change
        #(
            #index_mutators
        )*

        // define query results structure as a flat array which separates errors into own list
        #[derive(Debug, Serialize, Deserialize)]
        struct QueryResults {
            #[serde(default)]
            pub results: Vec<ResponseData>,
            // :TODO: pagination metadata
            #[serde(default)]
            #[serde(skip_serializing_if = "Vec::is_empty")]
            pub errors: Vec<WasmError>,
        }

        // declare public query method with injected handler logic
        #[hdk_extern]
        fn #exposed_query_api_method_name(SearchInputs { params }: SearchInputs) -> ExternResult<QueryResults>
        {
            let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

            // :TODO: proper search combinator logic, this just does exclusive boolean ops
            #(
                #query_handlers
            )*

            let entries = entries_result?;

            Ok(QueryResults {
                results: entries.iter()
                    .cloned()
                    .filter_map(Result::ok)
                    .collect(),
                errors: entries.iter()
                    .cloned()
                    .filter_map(Result::err)
                    .map(|err| { WasmError::from(err) })
                    .collect(),
            })
        }
    })
}

fn next_generic_type_as_string(args: &mut Punctuated<GenericArgument, Comma>) -> String {
    match args.pop().unwrap().value() {
        GenericArgument::Type(Type::Path(TypePath { path, .. })) => path.get_ident().unwrap().to_string(),
        _ => panic!("expecting a Type argument of length 1"),
    }
}
