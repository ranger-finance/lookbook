use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, Expr, FnArg, ItemFn, Lit, Meta};

#[proc_macro_attribute]
pub fn preview(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let ident = item.sig.ident.clone();
    let vis = item.vis.clone();
    let block = item.block.clone();

    let docs = collect_docs(&item.attrs);

    let s = ident.to_string();
    let name = s.strip_suffix("Preview").unwrap_or(&s);

    let mut states = Vec::new();
    let mut from_states = Vec::new();
    // let mut controls = Vec::new();

    for arg in item.sig.inputs.into_iter().skip(1) {
        match arg {
            FnArg::Typed(typed_arg) => {
                let mut docs = String::new();
                let mut default: Option<Expr> = None;

                for attr in typed_arg.attrs {
                    let path = attr.path().get_ident().unwrap().to_string();
                    if path == "doc" {
                        let meta = attr.meta.require_name_value().unwrap();
                        if let Expr::Lit(expr) = &meta.value {
                            if let Lit::Str(lit) = &expr.lit {
                                docs.push_str(&lit.value());
                                docs.push('\n');
                            }
                        }
                    }
                    // else if path == "lookbook" {
                    //     let path = attr.meta.require_list().unwrap();
                    //     let meta: Meta = syn::parse2(path.tokens.clone()).unwrap();

                    //     if let Meta::NameValue(meta_name_value) = meta {
                    //         if meta_name_value.path.is_ident("default") {
                    //             let value = meta_name_value.value;
                    //             default = Some(value);
                    //         }
                    //     }
                    // }
                }

                let ty = typed_arg.ty;
                let pat = typed_arg.pat.clone();
                let pat_name = pat.to_token_stream().to_string();

                states.push(quote! {
                    let default = #default;
                    let #pat = use_signal(|| default);
                });

                // from_states.push(quote! {let #pat = <#ty>::from_state(&**#pat);});

                // from_states.push(quote! {let #pat = <#ty>::from_state(#pat);});
                from_states.push(quote! {
                    // tracing::info!("hello world 4, {:?}, {:?}", stringify!(#pat), stringify!(#ty));
                });

                // let ty_name = ty.span().source_text().unwrap_or_else(|| "".to_string());
                // let default_string = default
                //     .map(|expr| expr.span().source_text().unwrap_or_default())
                //     .unwrap_or_default();

                // controls.push(quote! {{ rsx! {
                //     tr {
                //         border_bottom: "2px solid #e7e7e7",
                //         td { padding_left: "20px", p { color: "#222", font_weight: 600, #pat_name } }
                //         td { code { #ty_name } }
                //         td { p { #docs } }
                //         td { code { #default_string } }
                //         // td { padding_right: "20px", <#ty>::control(#pat_name, #pat) }
                //     }
                // }}});
            }
            _ => todo!(),
        }
    }

    // let controls = render_with_location(
    //     quote! {
    //         #(#controls)*
    //     },
    //     name,
    //     0,
    // );

    let look = render_with_location(
        quote! {
            lookbook::Look {
                name: #name,
                docs: #docs,
                // controls: controls,
                controls: None,
                children: #block
            }
        },
        name,
        1,
    );

    // let debug_stuff = vec![quote! {
    //     tracing::info!("look------------------ {:?}", #look);
    // }];

    let name_literal = quote! { #name };

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #vis static #ident: lookbook::Preview = lookbook::Preview::new(#name_literal, |()| {
            use dioxus::prelude::*;
            use lookbook::Control;
            use tracing;

            // tracing::info!("hello world 4, {:?}", stringify!(#controls));

            fn f() -> Element {
                #(#states)*

                // let controls = rsx! { #controls };

                // #(#debug_stuff)*

                #(#from_states)*
                // #debug_stuff

                // rsx! { div { "Hello, world!" } }
                // rsx! { {#look} }
                #look
            }

            f()
        });
    };

    //     let expanded = quote! {
    //         #[allow(non_upper_case_globals)]
    //         #vis static #ident: lookbook::Preview = lookbook::Preview::new(#name, |()| {
    //             use dioxus::prelude::*;
    //             use lookbook::Control;
    //             use tracing;

    //             fn f() -> Element {
    //                 #(#states)*

    //                 let controls = rsx! { #controls };

    //                 #(#from_states)*

    //                 tracing::info!("hello world 2, {:?}", stringify!(#ident));

    //                 rsx! { #look }
    //             }
    //             f()
    //         });
    //     };

    //     let expanded = quote! {
    //     #[allow(non_upper_case_globals)]
    //     #vis static #ident: lookbook::Preview = lookbook::Preview::new(stringify!(#ident), |()| {
    //         use dioxus::prelude::*;
    //         use tracing;

    //         tracing::info!("hello world 2, {:?}", stringify!(#ident));

    //         fn f() -> Element {
    //             rsx! { div { "Hello, world!" } }
    //         }
    //         f()
    //     });
    // };
    expanded.into()
}

fn render_with_location(
    tokens: proc_macro2::TokenStream,
    name: &str,
    idx: u8,
) -> proc_macro2::TokenStream {
    let location = format!("__lookbook/{name}.rs:0:0:{idx}");
    let rsx: dioxus_rsx::CallBody = syn::parse2(tokens).unwrap();
    rsx.render_with_location(location)
}

fn collect_docs(attrs: &[Attribute]) -> String {
    let mut docs = String::new();
    for attr in attrs {
        if attr.path().get_ident().unwrap().to_string() == "doc" {
            let meta = attr.meta.require_name_value().unwrap();
            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit) = &expr.lit {
                    docs.push_str(&lit.value());
                    docs.push('\n');
                }
            }
        }
    }
    docs
}
