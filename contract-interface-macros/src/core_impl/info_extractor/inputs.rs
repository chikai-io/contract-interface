use super::meta_attrs;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
use darling::FromMeta;

#[derive(Debug)]
pub struct Inputs {
    /// The `self`, or `&mut self`, or `&self` part.
    pub receiver: Option<syn::Receiver>,

    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg {
    pub contract_attr: Vec<syn::Attribute>,
    pub attr: Attrs,
    pub arg: syn::PatType,
}

#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    #[darling(rename = "attr", default)]
    pub forward_attr: Option<syn::Meta>,
    // #[darling(rename = "attr", default)]
    // pub forward_attr: Option<bool>,

    // #[darling(default, rename = "mod")]
    // method_mod_name: Option<syn::Ident>,
}

#[derive(Debug, Clone)]
pub struct Attrs {
    pub forward_attr: Vec<syn::NestedMeta>,
    // #[darling(rename = "attr", default)]
    // pub forward_attr: Option<bool>,

    // #[darling(default, rename = "mod")]
    // method_mod_name: Option<syn::Ident>,
}

impl Inputs {
    pub fn new<'a>(inputs: impl Iterator<Item = &'a mut syn::FnArg>) -> error::Result<Self> {
        let mut receiver = None;
        let mut args = Vec::new();
        for arg in inputs {
            match arg {
                syn::FnArg::Receiver(r) => {
                    assert!(receiver.is_none());
                    receiver = Some(r.clone())
                }
                syn::FnArg::Typed(pty) => {
                    // TODO: use darling
                    // it appears that darling doesn't work with
                    // attributes on method args

                    let (contract_attr, non_contract_attr) =
                        meta_attrs::partition_attrs(&pty.attrs, "contract");
                    pty.attrs.clear();
                    pty.attrs = non_contract_attr;

                    // let tokens = contract_attr.iter().map(|a| a.tokens.parse).collect::<Vec<_>>();

                    let attr = {
                        let meta_attrs = meta_attrs::into_meta_attrs(contract_attr.clone())?;
                        let nested =
                            meta_attrs::remove_first_layer(meta_attrs.clone(), "contract")?;

                        // TODO: check if works with multiple attr's
                        // eg. #[contract(attr(..))] #[contract(attr(..))]
                        // TODO: also check if works with multiple inner attrs
                        // eg. #[contract(attr(.., ..))]
                        let attrs = RawAttrs::from_list(&nested)?;
                        let attrs = if let Some(fa) = attrs.forward_attr {
                            let nested = meta_attrs::remove_first_layer(vec![fa], "attr")?;

                            Attrs {
                                forward_attr: nested,
                            }
                        } else {
                            Attrs {
                                forward_attr: vec![],
                            }
                        };

                        // let nested = &attrs.forward_attr;

                        // let q = quote::quote! {#(# [ #nested ])*};
                        attrs
                    };
                    // if let Some(a) = attrs.forward_attr {
                    //     panic!("{:#?}", a);
                    // }

                    let arg = Arg {
                        contract_attr,
                        arg: pty.clone(),
                        attr,
                    };

                    args.push(arg);
                }
            }
        }

        Ok(Self { receiver, args })
    }

    pub fn replace_from_self_to_state<'a>(mut self) -> Self {
        for pty in self.args.iter_mut() {
            replace_ident_from_self_to_state(&mut pty.arg);
        }
        self
    }
}
