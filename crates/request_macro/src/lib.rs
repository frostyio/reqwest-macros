use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, Expr, Ident, ItemStruct};

const EXPECTED_URL: &str = "Expected url: &str.";
const EXPECTED_METHOD: &str = "Expected method: Ident.";

#[proc_macro_attribute]
pub fn request(attrs: TokenStream, inp: TokenStream) -> TokenStream {
	let mut attribute_ast = parse_macro_input!(attrs as syn::AttributeArgs);

	let url_stream = attribute_ast.pop().expect(EXPECTED_URL).into_token_stream();
	let url: Expr = parse2(url_stream).expect(EXPECTED_URL);

	let method_stream = attribute_ast
		.pop()
		.expect(EXPECTED_METHOD)
		.into_token_stream();
	let method: Ident = parse2(method_stream).expect(EXPECTED_METHOD);

	let input = parse_macro_input!(inp as ItemStruct);
	let name = &input.ident;

	let expanded = quote! {
		#input
		impl #name {
			fn new() -> reqwest::RequestBuilder {
				let client = reqwest::Client::new();
				client.#method(#url)
			}

			fn from(client: &reqwest::Client) -> reqwest::RequestBuilder {
				client.#method(#url)
			}

			async fn json(resp: reqwest::Response) -> Result<#name, reqwest::Error> {
				Ok(resp.json().await?)
			}
		}
	};

	expanded.into()
}
