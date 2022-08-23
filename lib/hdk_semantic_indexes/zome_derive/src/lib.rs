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
    // Override the generated query API function name. Useful for grammatically correct pluralisation.
    #[darling(default)]
    query_fn_name: Option<String>,
    // Override the generated 'read all' API function name. Useful for grammatically correct pluralisation.
    #[darling(default)]
    read_all_fn_name: Option<String>,
    // Override the API method name in the associated CRUD zome that will be called with `ByAddress` to
    // retrieve associated records. Useful for record types with nonstandard (non-`DnaAddressable`) identifiers.
    #[darling(default)]
    record_read_fn_name: Option<String>,
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
    let record_type_str_ident = format_ident!("{}", record_type_str_attribute);

    let record_type_index_attribute = format_ident!("{}_index", record_type_str_attribute);
    let record_read_api_method_name = match &args.record_read_fn_name {
        None => format_ident!("get_{}", record_type_str_attribute),
        Some(read_fn) => format_ident!("{}", read_fn),
    };

    let exposed_query_api_method_name = match &args.query_fn_name {
        None => format_ident!("query_{}s", record_type_str_attribute),
        Some(query_fn) => format_ident!("{}", query_fn),
    };
    let exposed_read_api_method_name = match &args.read_all_fn_name {
        None => format_ident!("read_all_{}s", record_type_str_attribute),
        Some(read_fn) => format_ident!("{}", read_fn),
    };
    let exposed_append_api_name = format_ident!("record_new_{}", record_type_str_attribute);
    let creation_time_index_name = [record_type_str_attribute.clone(), ".created".to_string()].concat();
    let record_index_field_type = format_ident!("{}Address", record_type.to_string().to_case(Case::UpperCamel));

    // build iterators for generating index update methods and query conditions
    let all_indexes = fields.iter()
        .map(|field| {
            let relationship_name = field.ident.as_ref().unwrap().to_string().to_case(Case::Snake);

            // find first segment of field `Type` portion
            let path = match &field.ty {
                Type::Path(TypePath { path, .. }) => path,
                _ => panic!("expected index type of Local or Remote, with optional index-type casting (eg. String)"),
            };
            // parse the index type and its arguments
            let (index_type, args) = match path.segments.first() {
                // Default (hash-based) index.
                // `index_type` is "Local" or "Remote" depending on the *calling context* of the CRUD
                // zome these data updates are bound to.
                // Record identifiers are of type `DnaAddressable<T>` and the arguments map to the indexed entry
                // types' foreign CRUD zome names / datatypes.
                Some(PathSegment { arguments: AngleBracketed(AngleBracketedGenericArguments { args, .. }), ident, .. }) => (ident, args),
                _ => panic!("expected parameterised index with <related_record_type, relationship_name>"),
            };
            // set flag for injecting index datatype translation logic if typecast syntax is present
            let index_datatype = if path.segments.len() == 2 {
                match path.segments.last() {
                    Some(PathSegment { ident, .. }) => Some(ident),
                    None => None
                }
            } else { None };

            // parse definition for related Record entity names
            assert_eq!(args.len(), 2, "expected 2 args to index defs");
            let mut these_args = args.to_owned();
            let related_relationship_name: String = next_generic_type_as_string(&mut these_args).to_case(Case::Snake);
            let related_record_type: String = next_generic_type_as_string(&mut these_args);

            // generate identifiers for substituion
            let related_index_field_type = format_ident!("{}Address", related_record_type.to_case(Case::UpperCamel));
            let related_index_name = format_ident!("{}_{}", record_type_str_attribute, relationship_name);
            let related_record_type_str_attribute = related_record_type.to_case(Case::Snake);
            let reciprocal_index_name = format_ident!("{}_{}", related_record_type_str_attribute, related_relationship_name);
            let remote_record_time_index_id: String = format!("{}_{}.indexed", record_type_str_attribute, relationship_name);

            (
                index_type, index_datatype, relationship_name,
                related_record_type_str_attribute,
                related_index_field_type, related_index_name,
                reciprocal_index_name, remote_record_time_index_id,
            )
        });

    // generate all public API accessor interfaces
    let index_accessors = all_indexes.clone()
        .map(|(
            _index_type, _index_datatype, relationship_name,
            _related_record_type_str_attribute,
            related_index_field_type, related_index_name,
            _reciprocal_index_name, remote_record_time_index_id,
        )| {
            let local_dna_read_method_name = format_ident!("_internal_read_{}_{}", record_type_str_attribute, relationship_name);

            quote! {
                #[hdk_extern]
                fn #local_dna_read_method_name(ByAddress { address }: ByAddress<#record_index_field_type>) -> ExternResult<Vec<#related_index_field_type>> {
                    Ok(read_index(
                        &address,
                        &stringify!(#related_index_name),
                        &#remote_record_time_index_id,
                    )?)
                }
            }
        });

    // generate all public APIs for index updates / mutation
    let index_mutators = all_indexes.clone()
        .map(|(
            index_type, _index_datatype, relationship_name,
            _related_record_type_str_attribute,
            related_index_field_type, related_index_name,
            reciprocal_index_name, remote_record_time_index_id,
        )| {
            // :TODO: differentiate Local/Remote indexes as necessitated by final HC core APIs
            let dna_update_method_name = match index_type.to_string().as_ref() {
                "Local" => format_ident!("_internal_index_{}_{}", record_type_str_attribute, relationship_name),
                "Remote" => format_ident!("index_{}_{}", record_type_str_attribute, relationship_name),
                _ => panic!("expected index type of Local or Remote"),
            };

            // Standard logic for *Addressable-based indexes.
            // Note that String-based indexes are transparently converted to *Addressable ones in the client
            // macros and passed through to this method as normal.
            quote! {
                #[hdk_extern]
                fn #dna_update_method_name(indexes: RemoteEntryLinkRequest<#related_index_field_type, #record_index_field_type>) -> ExternResult<RemoteEntryLinkResponse> {
                    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

                    Ok(sync_index(
                        &remote_entry,
                        target_entries.as_slice(),
                        removed_entries.as_slice(),
                        &stringify!(#reciprocal_index_name), &stringify!(#related_index_name),
                        &#remote_record_time_index_id,
                    )?)
                }
            }
        });

    // generate query API method code to handle filtered read requests
    let query_handlers = all_indexes
        .map(|(
            _index_type, index_datatype, relationship_name,
            _related_record_type_str_attribute,
            related_index_field_type, _related_index_name,
            reciprocal_index_name, _remote_record_time_index_id,
        )| {
            let query_field_ident = format_ident!("{}", relationship_name);

            // custom adapter logic for indexes based on non-`DnaAddressable` data
            match index_datatype {
                Some(string_ident) => match string_ident.to_string().as_ref() {
                    "String" => quote! {
                        match &params.#query_field_ident {
                            Some(#query_field_ident) => {
                                // adapt the externally passed String identifier to an EntryHash for indexing engine
                                let index_anchor_path = Path::from(#query_field_ident);
                                let index_anchor_id: #related_index_field_type = DnaAddressable::new(dna_info()?.hash, index_anchor_path.path_entry_hash()?);

                                entries_result = query_index::<ResponseData, #record_index_field_type, _,_,_,_,_,_>(
                                    &index_anchor_id,
                                    &stringify!(#reciprocal_index_name),
                                    &LOCAL_TIME_INDEX_ID,
                                    &read_index_target_zome,
                                    &QUERY_FN_NAME,
                                );
                            },
                            _ => (),
                        };
                    },
                    _ => panic!("String is currently the only valid index datatype"),
                },
                // standard logic for *Addressable-based indexes
                None => quote! {
                    match &params.#query_field_ident {
                        Some(#query_field_ident) => {
                            entries_result = query_index::<ResponseData, #record_index_field_type, _,_,_,_,_,_>(
                                #query_field_ident,
                                &stringify!(#reciprocal_index_name),
                                &LOCAL_TIME_INDEX_ID,
                                &read_index_target_zome,
                                &QUERY_FN_NAME,
                            );
                        },
                        _ => (),
                    };
                },
            }
        });

    // combine everything to generate the toplevel zome definition code
    TokenStream::from(quote! {
        use hdk::prelude::*;
        use hdk_semantic_indexes_zome_lib::*;
        use hdk::hash_path::path::TypedPath;
        use hdk_semantic_indexes_core::LinkTypes;

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
        const QUERY_FN_NAME: &str = stringify!(#record_read_api_method_name);
        const LOCAL_TIME_INDEX_ID: &str = #creation_time_index_name;

        // pagination constants
        const PAGE_SIZE: usize = 30;

        // public zome API for reading indexes to determine related record IDs
        #(
            #index_accessors
        )*

        // public zome API for updating indexes when associated records change
        #(
            #index_mutators
        )*

        // query input parameters mimicing Relay's pagination spec
        // @see https://relay.dev/graphql/connections.htm
        // :TODO: extend to allow for filtering with `QueryParams`
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PagingParams {
            // :TODO: forwards pagination
            // first: Option<usize>,
            // after: Option<EntryHash>,
            last: Option<usize>,
            before: Option<EntryHash>,
        }

        // query results structure mimicing Relay's pagination format
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct QueryResults {
            pub page_info: PageInfo,
            #[serde(default)]
            pub edges: Vec<Edge>,
            #[serde(default)]
            #[serde(skip_serializing_if = "Vec::is_empty")]
            pub errors: Vec<WasmError>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Edge {
            node: Response,
            cursor: String,
        }

        // declare public list API
        #[hdk_extern]
        fn #exposed_read_api_method_name(PagingParams { /*first, after,*/ last, before }: PagingParams) -> ExternResult<QueryResults> {
            let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

            entries_result = query_time_index::<ResponseData, #record_index_field_type,_,_,_>(
                &read_index_target_zome,
                &QUERY_FN_NAME,
                &LOCAL_TIME_INDEX_ID,
                before,
                last.unwrap_or(PAGE_SIZE),
            );

            Ok(handle_list_output(entries_result?.as_slice())?)
        }

        // declare API for global list API management
        #[hdk_extern]
        fn #exposed_append_api_name(AppendAddress { address, timestamp }: AppendAddress<#record_index_field_type>) -> ExternResult<()> {
            Ok(append_to_time_index(&LOCAL_TIME_INDEX_ID, &address, timestamp)?)
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

            Ok(handle_list_output(entries_result?.as_slice())?)
        }

        fn handle_list_output(entries: &[RecordAPIResult<ResponseData>]) -> RecordAPIResult<QueryResults>
        {
            let valid_edges = entries.iter()
                .cloned()
                .filter_map(Result::ok);

            let edge_cursors = valid_edges
                .clone()
                .map(|node| {
                    node.#record_type_str_ident.id.to_string()
                });

            let formatted_edges = valid_edges.zip(edge_cursors)
                .map(|(node, cursor)| {
                    Edge {
                        node: node.#record_type_str_ident,
                        cursor,
                    }
                });

            let mut edge_cursors = formatted_edges.clone().map(|e| { e.cursor });
            let first_cursor = edge_cursors.next().unwrap_or("0".to_string());

            Ok(QueryResults {
                edges: formatted_edges.collect(),
                page_info: PageInfo {
                    end_cursor: edge_cursors.last().unwrap_or(first_cursor.clone()),
                    start_cursor: first_cursor,
                    // :TODO:
                    has_next_page: true,
                    has_previous_page: true,
                    page_limit: None,
                    total_count: None,
                },
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
