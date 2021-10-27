use super::meta_attrs;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
use darling::FromMeta;

#[derive(Debug)]
pub struct Inputs {
    /// The `self`, or `&mut self`, or `&self` part.
    pub receiver: Option<syn::Receiver>,
    pub receiver_kind: ReceiverKind,

    pub args: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub enum ReceiverKind {
    RefMut,
    Ref,
    Owned,
    Stateless,
}

impl From<Option<syn::Receiver>> for ReceiverKind {
    fn from(r: Option<syn::Receiver>) -> Self {
        match r {
            Some(r) => match (r.reference.is_some(), r.mutability.is_some()) {
                (true, true) => ReceiverKind::RefMut,
                (true, false) => ReceiverKind::Ref,
                (false, true) => ReceiverKind::Owned,
                (false, false) => ReceiverKind::Owned,
            },
            None => ReceiverKind::Stateless,
        }
    }
}

impl ReceiverKind {
    pub fn quote_trait_name(&self) -> proc_macro2::TokenStream {
        use quote::quote;
        match self {
            ReceiverKind::RefMut => quote!(_interface::ServeRefMut),
            ReceiverKind::Ref => quote!(_interface::ServeRef),
            ReceiverKind::Owned => quote!(_interface::ServeOwned),
            ReceiverKind::Stateless => quote!(_interface::ServeStateless),
        }
    }
    pub fn quote_trait_link_str(&self) -> &str {
        match self {
            ReceiverKind::RefMut => "[`ServeRefMut`](_interface::ServeRefMut)",
            ReceiverKind::Ref => "[`ServeRef`](_interface::ServeRef)",
            ReceiverKind::Owned => "[`ServeOwned`](_interface::ServeOwned)",
            ReceiverKind::Stateless => "[`ServeStateless`](_interface::ServeStateless)",
        }
    }
    pub fn quote_self_argument(&self) -> proc_macro2::TokenStream {
        use quote::quote;
        match self {
            ReceiverKind::RefMut => quote!(&mut Self::State,),
            ReceiverKind::Ref => quote!(&Self::State,),
            ReceiverKind::Owned => quote!(Self::State,),
            ReceiverKind::Stateless => quote!(),
        }
    }
}

#[derive(Debug)]
pub struct Arg {
    pub contract_attr: Vec<syn::Attribute>,
    pub attr: Attrs,
    pub arg: syn::PatType,
}

/// Attributes for a single argument.
#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    /// Forward attributes to be attached into the `Args` structure.
    #[darling(rename = "attr", default)]
    pub forward_attr: Option<syn::Meta>,
}

#[derive(Debug, Clone)]
pub struct Attrs {
    /// Forward attributes to be attached into the `Args` structure.
    pub forward_attr: Vec<syn::NestedMeta>,
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
                    let (contract_attr, non_contract_attr) =
                        meta_attrs::partition_attrs(&pty.attrs, "contract");
                    pty.attrs.clear();
                    pty.attrs = non_contract_attr;

                    #[allow(clippy::let_and_return)]
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
        let receiver_kind = receiver.clone().into();

        Ok(Self {
            receiver,
            receiver_kind,
            args,
        })
    }

    pub fn replace_from_self_to_state(mut self) -> Self {
        for pty in self.args.iter_mut() {
            replace_ident_from_self_to_state(&mut pty.arg);
        }
        self
    }
}
