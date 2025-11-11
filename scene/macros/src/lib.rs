use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NodeBehaviour, attributes(node_base))]
pub fn derive_node_behaviour(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;

	let expanded = quote! {
		impl fatum_scene::NodeBehaviour for #name {
			fn setup(&mut self) {
				
			}

			fn dispatcher(&self) -> &fatum_signals::SignalDispatcher {
				&self.signal_dispatcher
			}

			fn as_any(&self) -> &dyn std::any::Any {
				self
			}

			fn as_any_mut(&mut self) -> &mut std::any::Any {
				self
			}
		}
	};

	TokenStream::from(expanded)
}

#[proc_macro_derive(NodeComponent)]
pub fn derive_node_component(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;

	let expanded = quote! {
		impl fatum_scene::NodeComponent for #name {
			fn enter_scene(&mut self, owner: fatum_scene::NodeId, scene: fatum_scene::SharedSceneGraph) {
				self.owner = owner;
				self.scene = Some(scene);
			}

			fn exit_scene(&mut self) {
				self.owner = Default::default();
				self.scene = Default::default();
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
