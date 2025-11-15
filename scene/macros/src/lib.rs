use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NodeComponent)]
pub fn derive_node_component(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;

	let expanded = quote! {
		impl fatum_scene::NodeComponent for #name {
			fn name(&self) -> &str {
				std::any::type_name::<Self>()
			}

			fn enter_scene(&mut self, owner: fatum_scene::NodeId, scene: fatum_scene::SharedSceneGraph) {
				self.owner = owner;
				self.scene = Some(scene);
			}

			fn exit_scene(&mut self) {
				self.owner = Default::default();
				self.scene = Default::default();
			}

			fn clone_component(&self) -> Box<dyn NodeComponent> {
				std::boxed::Box::new(std::clone::Clone::clone(self))
			}

			fn as_any(&self) -> &dyn std::any::Any {
				self
			}

			fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
				self
			}
		}
	};

	TokenStream::from(expanded)
}
