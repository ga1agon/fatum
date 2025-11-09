use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NodeBehaviour, attributes(node_base))]
pub fn derive_node_behaviour(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Find the field marked with `#[node_base]` (default: `base`)
    let base_field = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .find(|f| {
                    f.attrs.iter().any(|attr| {
                        attr.path()
                            .segments
                            .last()
                            .map_or(false, |s| s.ident == "node_base")
                    })
                })
                .map(|f| f.ident.clone())
                .unwrap_or_else(|| Some(syn::Ident::new("base", name.span()))),
            _ => Some(syn::Ident::new("base", name.span())),
        },
        _ => Some(syn::Ident::new("base", name.span())),
    };

    let expanded = quote! {
        impl fatum_scene::NodeBehaviour for #name {
            fn enter_scene(
                &mut self,
                scene: std::sync::Arc<std::sync::Mutex<fatum_scene::SceneTree>>
            ) {
                self.#base_field.enter_scene(scene);
            }

            fn exit_scene(&mut self) {
                self.#base_field.exit_scene();
            }

            fn ready(&mut self) {
                self.#base_field.ready();
            }

            fn update(&mut self, delta: std::time::Duration) {
                self.#base_field.update(delta);
            }
        }
    };

    TokenStream::from(expanded)
}
