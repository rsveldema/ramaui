use crate::syn::ImplItem::Fn;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{self, parse_macro_input, ItemImpl};

struct FoundFuncs {
    name: String,
    //inputs : Punctuated<FnArg, Comma>
}

impl ToTokens for FoundFuncs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ident = format_ident!("{}", self.name);
        let toks = quote! {
            MethodInfo{ name: #name.to_string(), func: SelfType::#ident}
        };

        tokens.append_all(toks);
    }
}

fn get_name_from_impl_block(input: &ItemImpl) -> syn::Ident {
    let impl_path = &input.self_ty;
    let impl_ref = impl_path.as_ref();
    match impl_ref {
        syn::Type::Path(p) => {
            let segments = &p.path.segments;
            let name = segments[0].ident.to_string();
            let impl_name = name;
            let name = format_ident!("{}", impl_name);
            name
        }

        _ => {
            panic!("unhandled assoc type: {}", impl_path.to_token_stream());
        }
    }
}

#[proc_macro_attribute]
pub fn inspectable(_meta: TokenStream, code: TokenStream) -> TokenStream {
    let input = parse_macro_input!(code as ItemImpl);

    //println!(" PARSED MACRO:{:#?}", input);

    let mut funcs = Vec::<FoundFuncs>::new();

    let name = get_name_from_impl_block(&input);

    for item in &input.items {
        if let Fn(impl_item) = item {
            // println!("found impl item !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:#?}", impl_item);

            let sig = &impl_item.sig;
            let ident = &sig.ident;
            let generics = &sig.generics;
            let inputs = &sig.inputs;

            if !generics.params.is_empty() {
                // no introspection for generics
                // as we can't direcly call it
                continue;
            }

            if inputs.len() != 1 {
                continue;
            }

            funcs.push(FoundFuncs {
                name: ident.to_string(),
                //inputs: inputs.clone()
            });
        }
    }

    let method_info = format_ident!("MethodInfo_{}", name);
    let mut expanded = quote! {
        // The generated impl.
        struct #method_info {
            name: String,
            func: fn(&#name)
        }
        struct #name {
            tree: Option<UITreeRef>,
            info: Vec<#method_info>
        }

        impl #name {
            pub fn new() -> #name {
                type MethodInfo = #method_info;
                type SelfType = #name;
                #name {
                    tree: Option::None,
                    info: vec![#(#funcs),*]
                }
            }

            fn find_method(&self, name: &str) -> Option<&#method_info> {
                for it in self.info.iter() {
                    //println!("examine: {} vs {}", it.name, name);
                    if it.name.to_lowercase() == name.to_lowercase() {
                        return Option::Some(it);
                    }
                }
                return Option::None;
            }
        }

        use callable::CallableByName;

        impl CallableByName for #name {

            fn get_tree(&self) -> Option<UITreeRef> {
                return self.tree;
            }

            fn set_tree(&mut self, tree: Option<UITreeRef>)
            {
                self.tree = tree;
            }

            fn call_method(&self, name: &str) {
                let opt = self.find_method(name);
                match (opt) {
                    Some(m) => {
                        let func = m.func;
                        func(self);
                    }

                    None => {
                        println!("failed to find method: {}", name);
                    }
                }
            }
        }
    };

    let new_toks = quote!(#input);

    // TokenStream::from(expanded.append_all(new_toks))
    expanded.append_all(new_toks);

    // println!("expanded = {}", expanded);

    TokenStream::from(expanded)
}
