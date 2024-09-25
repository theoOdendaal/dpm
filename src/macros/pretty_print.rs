// Macro used to convert struct into a dataframe.

// First create a macros that checks that all struct field are of type Vec, and all are of equal length.

//#[macro_export]
//macro_rules! pretty_print {}
/*
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ExtractFieldNames)]
pub fn extract_field_names_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let name = &input.ident;

    // Ensure it's a struct with named fields
    let field_names = if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            // Extract field names as identifiers
            fields_named
                .named
                .iter()
                .map(|f| &f.ident)
                .collect::<Vec<_>>()
        } else {
            panic!("Expected named fields");
        }
    } else {
        panic!("Expected a struct");
    };

    // Generate the implementation for the macro
    let expanded = quote! {
        impl #name {
            pub fn field_names() -> Vec<String> {
                vec![
                    #(#field_names.as_ref().unwrap().to_string(),)*
                ]
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
    */
