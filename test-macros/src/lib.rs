use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn preview(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemFn);

    // Extract the function name and visibility
    let name = &input.sig.ident;
    let vis = &input.vis;

    // Generate the expanded code
    let expanded = quote! {
        #vis fn #name() {
            use tracing;
            tracing::info!("hello world 2, {:?}", stringify!(#name));
            #input
        }
    };

    expanded.into()
}
