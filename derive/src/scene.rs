use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Field};

fn default_scene_item_type() -> String {
    "sceneify::SceneItemInstance".to_string()
}

#[derive(FromDeriveInput)]
#[darling(attributes(sceneify))]
struct SceneArgs {
    #[darling(default = "default_scene_item_type")]
    scene_item_type: String,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let SceneArgs { scene_item_type } = SceneArgs::from_derive_input(&input).expect("bruh");
    let DeriveInput { ident, data, .. } = input;
    let data = match data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let field_args = data
        .fields
        .iter()
        .map(|field| {
            // panic!("{:?}", field);
            match &field.ty {
                syn::Type::Path(ty) => {
                    let ident = field.ident.as_ref().unwrap();
                    quote! {
                        , #ident: <#ty as sceneify::Source>::SceneItemType
                    }
                }
                _ => panic!("Only verbatim types are supported"),
            }
        })
        .collect::<Vec<_>>();

    let instance_ident = format_ident!("{}Instance", ident);
    let scene_item_type: proc_macro2::TokenStream = scene_item_type.parse().unwrap();

    let output = quote! {
        struct #instance_ident {
            internals: sceneify::SceneInternals
        }

        impl #ident {
            pub fn new(name: &str #(#field_args)*) -> #instance_ident {
                #instance_ident {
                    internals: sceneify::SceneInternals {
                        name: name.to_string(),
                    }
                }
            }
        }

        impl sceneify::Scene for #instance_ident {
            type SceneItemType = #scene_item_type<Self, Self::Settings>;
        }
    };
    output.into()
}
