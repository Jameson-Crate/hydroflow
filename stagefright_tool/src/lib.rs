use std::path::Path;
use std::{env, fs};

use proc_macro2::Span;
use quote::ToTokens;
use syn::parse_quote;
use syn::visit_mut::VisitMut;

struct GenMacroVistor {
    exported_macros: Vec<syn::Path>,
    current_mod: syn::Path,
}

// marks everything as pub(crate) because proc-macros cannot actually export anything
impl VisitMut for GenMacroVistor {
    fn visit_item_enum_mut(&mut self, i: &mut syn::ItemEnum) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            i.vis = parse_quote!(pub(crate));
        }
    }

    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        let old_mod = self.current_mod.clone();
        self.current_mod = parse_quote!(#old_mod::#i.ident);

        syn::visit_mut::visit_item_mod_mut(self, i);

        self.current_mod = old_mod;
    }

    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        let is_entry = i
            .attrs
            .iter()
            .any(|a| a.path().to_token_stream().to_string() == "stagefright :: entry");

        if is_entry {
            let cur_path = &self.current_mod;
            let i_name = &i.sig.ident;
            self.exported_macros.push(parse_quote!(#cur_path::#i_name));
        }

        if matches!(i.vis, syn::Visibility::Public(_)) {
            i.vis = parse_quote!(pub(crate));
        }
    }

    fn visit_item_mut(&mut self, i: &mut syn::Item) {
        syn::visit_mut::visit_item_mut(self, i);
    }
}

pub fn gen_macro(flow_path: &Path, final_crate: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib.rs");

    let mut flow_lib =
        syn_inline_mod::parse_and_inline_modules(&flow_path.join("src").join("lib.rs"));
    let mut visitor = GenMacroVistor {
        exported_macros: vec![],
        current_mod: parse_quote!(crate::__flow),
    };
    visitor.visit_file_mut(&mut flow_lib);

    for exported in visitor.exported_macros {
        let underscored_path = syn::Ident::new(
            &exported
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("_"),
            Span::call_site(),
        );

        let proc_macro_wrapper: syn::ItemFn = parse_quote!(
            #[proc_macro]
            #[allow(non_snake_case)]
            pub fn #underscored_path(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                let input = ::stagefright::internal::TokenStream::from(input);
                let out = ::stagefright::runtime_support::CURRENT_FINAL_CRATE.with(|f| {
                    let mut f = f.borrow_mut();
                    *f = Some(#final_crate);
                    drop(f);
                    #exported(input)
                });
                ::proc_macro::TokenStream::from(out)
            }
        );

        flow_lib.items.push(syn::Item::Fn(proc_macro_wrapper));
    }

    fs::write(&dest_path, flow_lib.to_token_stream().to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");

    let flow_path_absolute = fs::canonicalize(&flow_path).unwrap();
    println!(
        "cargo:rerun-if-changed={}",
        flow_path_absolute.to_string_lossy()
    );
}

struct GenFinalVistor {
    current_mod: syn::Path,
}

impl VisitMut for GenFinalVistor {
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        let old_mod = self.current_mod.clone();
        self.current_mod = parse_quote!(#old_mod::#i.ident);

        syn::visit_mut::visit_item_mod_mut(self, i);

        self.current_mod = old_mod;
    }

    fn visit_item_mut(&mut self, i: &mut syn::Item) {
        match i {
            syn::Item::Fn(fun) => {
                let is_entry = fun
                    .attrs
                    .iter()
                    .any(|a| a.path().to_token_stream().to_string() == "stagefright :: entry");

                if is_entry {
                    let cur_path = &self.current_mod;
                    let i_name = &fun.sig.ident;
                    let path: syn::Path = parse_quote!(#cur_path::#i_name);

                    let underscored_path = syn::Ident::new(
                        &path
                            .segments
                            .iter()
                            .map(|s| s.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("_"),
                        Span::call_site(),
                    );

                    *i = parse_quote!(pub use crate::__macro::#underscored_path as #i_name;);
                }
            }
            syn::Item::Enum(e) => {
                if matches!(e.vis, syn::Visibility::Public(_)) {
                    let cur_path = &self.current_mod;
                    let e_name = &e.ident;
                    *i = parse_quote!(pub use #cur_path::#e_name;);
                }
            }
            _ => {}
        }

        syn::visit_mut::visit_item_mut(self, i);
    }
}

struct GenFinalPubVistor {
    current_mod: syn::Path,
}

impl VisitMut for GenFinalPubVistor {
    fn visit_item_enum_mut(&mut self, i: &mut syn::ItemEnum) {
        i.vis = parse_quote!(pub);
    }

    fn visit_item_use_mut(&mut self, i: &mut syn::ItemUse) {
        i.vis = parse_quote!(pub);
    }

    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        let old_mod = self.current_mod.clone();
        self.current_mod = parse_quote!(#old_mod::#i.ident);

        i.vis = parse_quote!(pub);

        syn::visit_mut::visit_item_mod_mut(self, i);

        self.current_mod = old_mod;
    }
}

pub fn gen_final(flow_path: &Path) {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut flow_lib =
        syn_inline_mod::parse_and_inline_modules(&flow_path.join("src").join("lib.rs"));
    let mut flow_lib_pub = flow_lib.clone();

    let mut final_visitor = GenFinalVistor {
        current_mod: parse_quote!(crate::__flow),
    };
    final_visitor.visit_file_mut(&mut flow_lib);

    let mut final_pub_visitor = GenFinalPubVistor {
        current_mod: parse_quote!(crate::__flow),
    };
    final_pub_visitor.visit_file_mut(&mut flow_lib_pub);

    fs::write(
        &Path::new(&out_dir).join("lib.rs"),
        flow_lib.to_token_stream().to_string(),
    )
    .unwrap();

    fs::write(
        &Path::new(&out_dir).join("lib_pub.rs"),
        flow_lib_pub.to_token_stream().to_string(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    let flow_path_absolute = fs::canonicalize(&flow_path).unwrap();
    println!(
        "cargo:rerun-if-changed={}",
        flow_path_absolute.to_string_lossy()
    );
}
