use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn describe_struct(name: &syn::Ident, fields: Vec<&syn::Ident>) -> proc_macro2::TokenStream {
    quote!(
        impl Describe for #name {
            fn describe(&self) -> String {
                let mut description = String::new();
                description.push_str("This is a struct object\n");
                description.push_str("Fields:\n");
                #(description.push_str(&format!("{} -> {:?}\n", stringify!(#fields), self.#fields));)*
                description
            }
        }
    )
}

fn describe_enum(name: &syn::Ident, variants: Vec<syn::Ident>) -> proc_macro2::TokenStream {
    quote!(
        impl Describe for #name {
            fn describe(&self) -> String {
                let mut description = String::new();
                description.push_str("This is an enum object\n");
                let variant = match self {
                    #(Self::#variants => format!("{}::{}", stringify!(#name), stringify!(#variants)),)*
                };
                description.push_str(&format!("Current variant: {}\n", variant));
                description.push_str("Variants:\n");
                #(description.push_str(&format!("{}::{}\n", stringify!(#name), stringify!(#variants)));)*
                description
            }
        }
    )
}

#[proc_macro_derive(Describe)]
pub fn json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let obj_type = match input.data {
        syn::Data::Struct(ref data) => {
            let fields = data.fields.iter().filter_map(|f| f.ident.as_ref()).collect::<Vec<_>>();
            describe_struct(name, fields)
        }
        syn::Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| v.ident.clone()).collect();
            describe_enum(name, variants)
        }
        _ => panic!("Only structs and enums are supported"),
    };

    TokenStream::from(obj_type)
}
