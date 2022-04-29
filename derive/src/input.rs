use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(sceneify))]
struct InputAttrs {
    kind: String,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let InputAttrs { kind } = InputAttrs::from_derive_input(&input).expect("bruh");
    let DeriveInput { ident, data, .. } = input;
    let data = match data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let settings_ident = format_ident!("{}Settings", ident);

    let settings_fields = data
        .fields
        .iter()
        .map(|field| {
            let ident = field
                .ident
                .as_ref()
                .map(|ident| format_ident!("{}", ident.to_string().to_case(Case::Pascal)))
                .unwrap();
            let ty = &field.ty;
            quote! {
                #ident(#ty)
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        enum #settings_ident {
            #(#settings_fields),*
        }

        impl sceneify::Source for #ident {
            type Settings = #settings_ident;
            type SceneItemType = sceneify::SceneItemInstance<Self, Self::Settings>;

            fn get_kind() -> &'static str {
                #kind
            }
        }

        impl sceneify::Input for #ident {
        }
    };
    output.into()
}
