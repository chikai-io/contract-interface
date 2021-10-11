use super::attr_sig_info_called_in::AttrSigInfo;
use crate::replace_ident::replace_ident_from_self_to_state;
use std::collections::HashMap;
use syn::export::Span;
use syn::spanned::Spanned;
use syn::{Error, LitStr, TraitItemMethod};

/// Information extracted from trait method.
pub struct TraitItemMethodInfo {
    /// The original AST of the trait item method.
    pub original: TraitItemMethod,

    /// Method name.
    pub ident: syn::Ident,

    /// The method documentation.
    /// eg. `#[doc = "My Documentation"] fn f() {}`
    pub docs: Vec<syn::Lit>,

    /// The method lifetimes generics.  
    /// eg. `fn f<'a>(){}`.
    pub generic_lifetimes: indexmap::IndexMap<syn::Lifetime, syn::LifetimeDef>,
    /// The method type generics.  
    /// eg. `fn f<T>(){}`.
    pub generic_types: indexmap::IndexMap<syn::Ident, syn::TypeParam>,
    /// The trait const generics.  
    /// eg. `f f<const N: usize>(){}`
    pub generic_consts: indexmap::IndexMap<syn::Ident, syn::ConstParam>,

    /// Lifetime bounds from the `where` clause.  
    /// eg. `fn f<'a, T>() where T: 'a`.
    pub lifetime_bounds: indexmap::IndexMap<syn::Ident, syn::PredicateLifetime>,
    /// Type bounds from the `where` clause.  
    /// eg. `fn f<T>() where T: Clone`.
    pub type_bounds: indexmap::IndexMap<syn::Type, syn::PredicateType>,

    /// The `self`, or `&mut self`, or `&self` part.
    pub receiver: Option<syn::Receiver>,

    pub args: Vec<syn::PatType>,
    // pub args: indexmap::IndexMap<syn::Ident, syn::PatType>,
    // pub args_sets: ArgsSets,
    //
    // /// Attributes and signature information.
    // pub attr_sig_info: AttrSigInfo,
    // /// String representation of method name, e.g. `"my_method"`.
    // pub ident_byte_str: LitStr,
}

impl TraitItemMethodInfo {
    pub fn new(
        original: &TraitItemMethod,
        trait_info: &super::item_trait_info_called_in::ItemTraitInfo,
    ) -> syn::Result<Self> {
        let ident = original.sig.ident.clone();

        let mut docs = vec![];
        for attr in &original.attrs {
            if !matches!(attr.style, syn::AttrStyle::Outer) {
                continue;
            }

            if attr.path.is_ident("doc") {
                match attr.parse_meta()? {
                    syn::Meta::NameValue(mnv) => docs.push(mnv.lit),
                    bad => return Err(Error::new_spanned(bad, "unrecognized doc attribute")),
                };
            }
        }

        let generic_lifetimes: indexmap::IndexMap<syn::Lifetime, syn::LifetimeDef> =
            original.sig.generics.lifetimes().map(|lt| (lt.lifetime.clone(), lt.clone())).collect();
        let generic_types: indexmap::IndexMap<syn::Ident, syn::TypeParam> = original
            .sig
            .generics
            .type_params()
            .map(|tp| {
                (tp.ident.clone(), {
                    let mut tp = tp.clone();
                    for b in tp.bounds.iter_mut() {
                        // for `T: Trait<Self>` case
                        replace_ident_from_self_to_state(b);
                    }
                    if let Some(d) = tp.default.as_mut() {
                        // for `T: Trait<Item=Self>` case
                        replace_ident_from_self_to_state(d)
                    }
                    tp
                })
            })
            .collect();
        let generic_consts: indexmap::IndexMap<syn::Ident, syn::ConstParam> =
            original.sig.generics.const_params().map(|cp| (cp.ident.clone(), cp.clone())).collect();

        let lifetime_bounds = if let Some(ref wc) = original.sig.generics.where_clause {
            wc.predicates
                .iter()
                .filter_map(|wp| {
                    if let syn::WherePredicate::Lifetime(pl) = wp {
                        Some(pl)
                    } else {
                        None
                    }
                })
                .map(|pl| (pl.lifetime.ident.clone(), pl.clone()))
                .collect()
        } else {
            indexmap::IndexMap::new()
        };
        let type_bounds = if let Some(ref wc) = original.sig.generics.where_clause {
            wc.predicates
                .iter()
                .filter_map(|wp| {
                    if let syn::WherePredicate::Type(pt) = wp {
                        let mut pt = pt.clone();
                        // for `Self: Trait` cases
                        replace_ident_from_self_to_state(&mut pt.bounded_ty);
                        for b in pt.bounds.iter_mut() {
                            // for `T: Trait<Self>` cases
                            replace_ident_from_self_to_state(b);
                        }
                        Some(pt)
                    } else {
                        None
                    }
                })
                .map(|pt| (pt.bounded_ty.clone(), pt))
                .collect()
        } else {
            indexmap::IndexMap::new()
        };

        let mut receiver = None;
        let mut args = Vec::new();
        for arg in &original.sig.inputs {
            match arg {
                syn::FnArg::Receiver(r) => {
                    assert!(receiver.is_none());
                    receiver = Some(r.clone())
                }
                syn::FnArg::Typed(pty) => {
                    let mut pty = pty.clone();
                    replace_ident_from_self_to_state(&mut pty);
                    args.push(pty);
                }
            }
        }

        // let attr_sig_info = AttrSigInfo::new(attrs, sig)?;

        // let ident_byte_str = LitStr::new(&attr_sig_info.ident.to_string(), Span::call_site());

        Ok(Self {
            ident,
            original: original.clone(),

            docs,
            generic_lifetimes,
            generic_types,
            generic_consts,
            lifetime_bounds,
            type_bounds,
            receiver,
            args,
            // args_sets,
            // attr_sig_info,
            // ident_byte_str
        })
    }
}
