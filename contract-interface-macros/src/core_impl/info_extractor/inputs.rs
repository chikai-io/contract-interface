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
    StatelessInit,
}

impl ReceiverKind {
    pub fn from_syn(r: &Option<syn::Receiver>, init: bool) -> Self {
        match (init, r) {
            (true, _) => ReceiverKind::StatelessInit,
            (false, Some(r)) => match (r.reference.is_some(), r.mutability.is_some()) {
                (true, true) => ReceiverKind::RefMut,
                (true, false) => ReceiverKind::Ref,
                (false, true) => ReceiverKind::Owned,
                (false, false) => ReceiverKind::Owned,
            },
            (false, None) => ReceiverKind::Stateless,
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
            ReceiverKind::StatelessInit => quote!(_interface::ServeStatelessInit),
        }
    }
    pub fn quote_trait_link_str(&self) -> &str {
        match self {
            ReceiverKind::RefMut => "[`ServeRefMut`](_interface::ServeRefMut)",
            ReceiverKind::Ref => "[`ServeRef`](_interface::ServeRef)",
            ReceiverKind::Owned => "[`ServeOwned`](_interface::ServeOwned)",
            ReceiverKind::Stateless => "[`ServeStateless`](_interface::ServeStateless)",
            ReceiverKind::StatelessInit => "[`ServeStatelessInit`](_interface::ServeStatelessInit)",
        }
    }
    pub fn quote_self_argument(&self) -> proc_macro2::TokenStream {
        use quote::quote;
        match self {
            ReceiverKind::RefMut => quote!(&mut Self::State,),
            ReceiverKind::Ref => quote!(&Self::State,),
            ReceiverKind::Owned => quote!(Self::State,),
            ReceiverKind::Stateless => quote!(),
            ReceiverKind::StatelessInit => quote!(),
        }
    }
}

#[derive(Debug)]
pub struct Arg {
    pub contract_attr: Vec<syn::Attribute>,
    pub attr: Attrs,
    pub arg: syn::PatType,
    pub fake_arg: Option<FakeArg>,
}

#[derive(Debug)]
pub struct FakeArg {
    pub fake_arg: syn::PatType,
    pub modification: ArgModification,
}

/// Linked list of modifications/adaptions made to the argument.
#[derive(Debug)]
pub struct ArgModification {
    kind: ArgModificationKind,
    next: Option<Box<ArgModification>>,
}

impl ArgModification {
    pub fn modify_pat(&self, s: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use quote::quote;
        let s = if let Some(next) = self.next.as_ref() {
            next.modify_pat(s)
        } else {
            s
        };
        match self.kind {
            ArgModificationKind::Ref => quote!(& #s),
            ArgModificationKind::RefMut => quote!(&mut #s),
            ArgModificationKind::PtrConst => quote!(& #s),
            ArgModificationKind::PtrMut => quote!(&mut #s),
        }
    }
}

/// A single kind of modification/adaption.
#[derive(Debug)]
pub enum ArgModificationKind {
    /// for definition: `x: &T` -> `x: T`
    /// for usage: `x` -> `&x`
    Ref,
    /// for definition: `x: &mut T` -> `x: T`
    /// for usage: `x` -> `&mut x`
    RefMut,
    /// for definition: `x: *const T` -> `x: T`
    /// for usage: `x` -> `&x`
    PtrConst,
    /// for definition: `x: *mut T` -> `x: T`
    /// for usage: `x` -> `&mut x`
    PtrMut,
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

fn append_modification(
    modification_head: &mut Option<Box<ArgModification>>,
    next_modification: ArgModification,
) -> &mut Option<Box<ArgModification>> {
    match modification_head {
        Some(m) => {
            m.next = Some(Box::new(next_modification));
            &mut m.next
        }
        None => {
            *modification_head = Some(Box::new(next_modification));
            modification_head
        }
    }
}

fn adapt_argument(
    pty: &mut syn::PatType,
    modification_head: &mut Option<Box<ArgModification>>,
) -> error::Result<()> {
    use syn::spanned::Spanned;

    // TODO
    match pty.pat.as_ref() {
        syn::Pat::Box(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Box is not currently supported.",
        )),
        syn::Pat::Ident(_p) => Ok(()),
        syn::Pat::Lit(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Lit is not currently supported.",
        )),
        syn::Pat::Macro(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Macro is not currently supported.",
        )),
        syn::Pat::Or(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Or is not currently supported.",
        )),
        syn::Pat::Path(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Path is not currently supported.",
        )),
        syn::Pat::Range(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Range is not currently supported.",
        )),
        syn::Pat::Reference(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Reference is not currently supported.",
        )),
        syn::Pat::Rest(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Rest is not currently supported.",
        )),
        syn::Pat::Slice(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Slice is not currently supported.",
        )),
        syn::Pat::Struct(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Struct is not currently supported.",
        )),
        syn::Pat::Tuple(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Tuple is not currently supported.",
        )),
        syn::Pat::TupleStruct(p) => Err(syn::Error::new(
            p.span(),
            "Pat::TuTupleStructple is not currently supported.",
        )),
        syn::Pat::Type(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Type is not currently supported.",
        )),
        syn::Pat::Verbatim(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Verbatim is not currently supported.",
        )),
        syn::Pat::Wild(p) => Err(syn::Error::new(
            p.span(),
            "Pat::Wild is not currently supported.",
        )),
        p => Err(syn::Error::new(
            p.span(),
            "Unknown Pat is not currently supported.",
        )),
    }?;
    match pty.ty.as_ref() {
        syn::Type::BareFn(ty) => {
            Err(syn::Error::new(ty.span(), "Type::BareFn is not currently supported.").into())
        }
        syn::Type::ImplTrait(ty) => {
            Err(syn::Error::new(ty.span(), "Type::ImplTrait is not currently supported.").into())
        }
        syn::Type::Ptr(ty) => {
            let kind = if ty.mutability.is_some() {
                ArgModificationKind::PtrMut
            } else {
                assert!(ty.const_token.is_some());
                ArgModificationKind::PtrConst
            };
            *pty.ty = *ty.elem.clone();
            let next_modification = ArgModification { kind, next: None };
            let new_last = append_modification(modification_head, next_modification);
            adapt_argument(pty, new_last)?;
            Ok(())
        }
        syn::Type::Reference(ty) => {
            let kind = if ty.mutability.is_some() {
                ArgModificationKind::RefMut
            } else {
                ArgModificationKind::Ref
            };
            *pty.ty = *ty.elem.clone();
            let next_modification = ArgModification { kind, next: None };
            let new_last = append_modification(modification_head, next_modification);
            adapt_argument(pty, new_last)?;
            Ok(())
        }
        syn::Type::Array(_) => Ok(()),
        syn::Type::Group(ty) => {
            Err(syn::Error::new(ty.span(), "Type::Group is not currently supported.").into())
        }
        syn::Type::Infer(ty) => {
            Err(syn::Error::new(ty.span(), "Type::Infer is not currently supported.").into())
        }
        syn::Type::Macro(ty) => {
            Err(syn::Error::new(ty.span(), "Type::Macro is not currently supported.").into())
        }
        syn::Type::Never(ty) => {
            Err(syn::Error::new(ty.span(), "Type::Never is not currently supported.").into())
        }
        syn::Type::Paren(_) => Ok(()),
        syn::Type::Path(_) => Ok(()),
        syn::Type::Slice(ty) => {
            Err(syn::Error::new(ty.span(), "Type::Slice is not currently supported.").into())
        }
        syn::Type::TraitObject(ty) => {
            Err(syn::Error::new(ty.span(), "Type::TraitObject is not currently supported.").into())
        }
        syn::Type::Tuple(_) => Ok(()),
        syn::Type::Verbatim(ts) => {
            Err(syn::Error::new(ts.span(), "Type::Verbatim is not currently supported.").into())
        }
        ty => Err(syn::Error::new(ty.span(), "Unknown Type is not currently supported.").into()),
    }
}

impl Inputs {
    pub fn new<'a>(
        inputs: impl Iterator<Item = &'a mut syn::FnArg>,
        method_is_init: bool,
    ) -> error::Result<Self> {
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

                    let mut fake_arg = None;
                    let mut modifications = None;
                    let arg = pty.clone();
                    let mut pty = pty.clone();
                    adapt_argument(&mut pty, &mut modifications)?;
                    if let Some(m) = modifications {
                        fake_arg = Some(FakeArg {
                            fake_arg: pty.clone(),
                            modification: *m,
                        });
                    }

                    let arg = Arg {
                        contract_attr,
                        arg,
                        fake_arg,
                        attr,
                    };

                    args.push(arg);
                }
            }
        }
        let receiver_kind = ReceiverKind::from_syn(&receiver, method_is_init);

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
