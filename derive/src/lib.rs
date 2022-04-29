use proc_macro::TokenStream;

mod input;
mod scene;

#[proc_macro_derive(Input, attributes(sceneify))]
pub fn derive_input(input: TokenStream) -> TokenStream {
    input::derive(input)
}

#[proc_macro_derive(Scene, attributes(sceneify))]
pub fn derive_scene(input: TokenStream) -> TokenStream {
    scene::derive(input)
}
