use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn graphics(input: TokenStream) -> TokenStream {
	let obj = parse_macro_input!(input as Expr);
	let expanded = quote! {
		{
			let mut g = #obj.graphics_engine();
			g.get()
		}
	};
	
	TokenStream::from(expanded)
}

#[proc_macro]
pub fn resources(input: TokenStream) -> TokenStream {
	let obj = parse_macro_input!(input as Expr);
	let expanded = quote! {
		{
			let mut r = #obj.resource_engine();
			g.get()
		}
	};

	TokenStream::from(expanded)
}
