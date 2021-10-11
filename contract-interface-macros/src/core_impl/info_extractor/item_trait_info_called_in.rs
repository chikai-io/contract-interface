use std::collections::HashMap;

use super::trait_item_method_info_called_in::TraitItemMethodInfo;
use crate::replace_ident::replace_ident_from_self_to_state;
use inflector::Inflector;
use syn::export::Span;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemTrait, TraitItem};

/// Information extracted from `ItemTrait`.
pub struct ItemTraitInfo {
    /// The original AST.
    pub original: ItemTrait,

    /// The trait name.  
    /// eg. `trait Name`
    pub original_ident: Ident,

    /// The trait name that will be used to generate the module.  
    /// eg. `mod name`
    pub ident: Ident,
    /// The trait documentation.
    /// eg. `#[doc = "My Documentation"] trait Trait {}`
    pub docs: Vec<syn::Lit>,

    /// The trait lifetimes generics.  
    /// eg. `trait Trait<'a>`.
    pub generic_lifetimes: indexmap::IndexMap<syn::Lifetime, syn::LifetimeDef>,
    /// The trait type generics.  
    /// eg. `trait Trait<T>`.
    pub generic_types: indexmap::IndexMap<syn::Ident, syn::TypeParam>,
    /// The trait const generics.  
    /// eg. `trait Trait<const N: usize>`
    pub generic_consts: indexmap::IndexMap<syn::Ident, syn::ConstParam>,

    /// Self lifetime bounds (from supertrait syntax).  
    /// eg. `trait Trait<'a>: 'a`.
    pub self_lifetime_bounds: Vec<syn::Lifetime>,
    /// Self trait bounds (from supertrait syntax).  
    /// eg. `trait Trait: OtherTrait`.
    pub self_trait_bounds: Vec<syn::TraitBound>,

    /// Lifetime bounds from the `where` clause.  
    /// eg. `trait Trait<'a, T> where T: 'a`.
    pub lifetime_bounds: indexmap::IndexMap<syn::Ident, syn::PredicateLifetime>,
    /// Type bounds from the `where` clause.  
    /// eg. `trait Trait<T> where T: Clone`.
    pub type_bounds: indexmap::IndexMap<syn::Type, syn::PredicateType>,

    /// The trait associated consts.  
    /// eg. `trait Trait {const T: u8}`.
    pub const_items: indexmap::IndexMap<syn::Ident, syn::TraitItemConst>,
    /// The trait associated types.  
    /// eg. `trait Trait {type T}`.
    pub assoc_type_items: indexmap::IndexMap<syn::Ident, syn::TraitItemType>,
    /// The trait methods.  
    /// eg. `trait Trait {fn f();}`
    pub method_items: indexmap::IndexMap<syn::Ident, TraitItemMethodInfo>,
}

impl ItemTraitInfo {
    pub fn new(original: &mut ItemTrait, trait_name_override: Option<Ident>) -> syn::Result<Self> {
        let original_ident = original.ident.clone();
        let ident = trait_name_override.unwrap_or({
            let res = original.ident.to_string().to_snake_case();
            Ident::new(&res, Span::call_site())
        });

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

        let generic_lifetimes =
            original.generics.lifetimes().map(|lt| (lt.lifetime.clone(), lt.clone())).collect();
        let generic_types = original
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
        let generic_consts =
            original.generics.const_params().map(|cp| (cp.ident.clone(), cp.clone())).collect();

        let self_lifetime_bounds = original
            .supertraits
            .iter()
            .filter_map(
                |tpb| if let syn::TypeParamBound::Lifetime(l) = tpb { Some(l) } else { None },
            )
            .cloned()
            .collect();
        let self_trait_bounds = original
            .supertraits
            .iter()
            .filter_map(|tpb| {
                if let syn::TypeParamBound::Trait(tb) = tpb {
                    let mut tb = tb.clone();
                    // for `Trait<Self>` (trait bound) case
                    replace_ident_from_self_to_state(&mut tb.path);
                    Some(tb)
                } else {
                    None
                }
            })
            .collect();

        let lifetime_bounds = if let Some(ref wc) = original.generics.where_clause {
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
        let type_bounds = if let Some(ref wc) = original.generics.where_clause {
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

        let const_items = original
            .items
            .iter()
            .filter_map(|item| if let syn::TraitItem::Const(tic) = item { Some(tic) } else { None })
            .map(|tic| (tic.ident.clone(), tic.clone()))
            .collect();

        let assoc_type_items = original
            .items
            .iter()
            .filter_map(|item| {
                if let syn::TraitItem::Type(tit) = item {
                    let mut tit = tit.clone();

                    // for the `type X<T: Trait<Self>>` cases
                    replace_ident_from_self_to_state(&mut tit.generics);
                    for b in tit.bounds.iter_mut() {
                        // for `T: Trait<Self>` cases
                        replace_ident_from_self_to_state(b);
                    }
                    if let Some((_, t)) = tit.default.as_mut() {
                        // for the `type X = Self` cases
                        replace_ident_from_self_to_state(t);
                    }

                    Some(tit)
                } else {
                    None
                }
            })
            .map(|tit| (tit.ident.clone(), tit))
            .collect();

        let mut partial_self = {
            Self {
                original_ident,
                original: original.clone(),
                ident,
                docs,
                generic_lifetimes,
                generic_types,
                generic_consts,
                self_lifetime_bounds,
                self_trait_bounds,
                lifetime_bounds,
                type_bounds,
                const_items,
                assoc_type_items,
                method_items: indexmap::IndexMap::new(),
            }
        };

        let method_items = original
            .items
            .iter()
            .filter_map(|ti| if let syn::TraitItem::Method(tim) = ti { Some(tim) } else { None })
            .map(|tim| Ok((tim.sig.ident.clone(), TraitItemMethodInfo::new(tim, &partial_self)?)))
            .collect::<Result<_, syn::Error>>()?;

        partial_self.method_items = method_items;
        Ok(partial_self)
    }
}
