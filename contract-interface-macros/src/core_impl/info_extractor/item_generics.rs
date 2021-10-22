use crate::replace_ident::replace_ident_from_self_to_ident;
use crate::replace_ident::replace_ident_from_self_to_state;

/// Generics for vairous kinds of items.  
#[derive(Debug, Clone)]
pub struct Generics {
    /// The item's lifetimes generics.  
    /// eg. `trait Trait<'a> {}`.
    /// eg. `impl<'a> Struct<'a> {}`.
    /// eg. `fn f<'a>(){}`.
    pub lifetimes: indexmap::IndexMap<syn::Lifetime, syn::LifetimeDef>,
    /// The item's type generics.  
    /// eg. `trait Trait<T> {}`.
    /// eg. `impl<T> Struct<T> {}`.
    /// eg. `fn f<T>(){}`.
    pub types: indexmap::IndexMap<syn::Ident, syn::TypeParam>,
    /// The item's const generics.  
    /// eg. `trait Trait<const N: usize> {}`
    /// eg. `impl<const N: usize> Struct<N> {}`
    /// eg. `fn f<const N: usize>(){}`
    pub consts: indexmap::IndexMap<syn::Ident, syn::ConstParam>,

    /// Lifetime bounds from the `where` clause.  
    /// eg. `trait Trait<'a, T> where T: 'a {}`.
    /// eg. `impl<'a, T> Struct<'a, T> where T: 'a {}`.
    /// eg. `fn f<'a, T>() where T: 'a {}`.
    pub lifetime_bounds: indexmap::IndexMap<syn::Ident, syn::PredicateLifetime>,
    /// Type bounds from the `where` clause.  
    /// eg. `trait Trait<T> where T: Clone {}`.
    /// eg. `impl<T> Struct where T: Clone {}`.
    /// eg. `fn f<T>() where T: Clone {}`.
    pub type_bounds: indexmap::IndexMap<syn::Type, syn::PredicateType>,
}

impl Generics {
    /// Gets information and replaces the `Self` identifier to `_State`.
    pub fn new(generics: &syn::Generics) -> Self {
        let lifetimes = generics
            .lifetimes()
            .map(|lt| (lt.lifetime.clone(), lt.clone()))
            .collect();
        let types = generics
            .type_params()
            .map(|tp| (tp.ident.clone(), tp.clone()))
            .collect();
        let consts = generics
            .const_params()
            .map(|cp| (cp.ident.clone(), cp.clone()))
            .collect();

        let lifetime_bounds = if let Some(ref wc) = generics.where_clause {
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
        let type_bounds = if let Some(ref wc) = generics.where_clause {
            wc.predicates
                .iter()
                .filter_map(|wp| {
                    if let syn::WherePredicate::Type(pt) = wp {
                        Some(pt.clone())
                    } else {
                        None
                    }
                })
                .map(|pt| (pt.bounded_ty.clone(), pt))
                .collect()
        } else {
            indexmap::IndexMap::new()
        };

        Self {
            lifetimes,
            types,
            consts,
            lifetime_bounds,
            type_bounds,
        }
    }
    pub fn replace_from_self_to_state(mut self) -> Self {
        use proc_macro2::Span;
        let _state = syn::Ident::new("_State", Span::call_site());
        self.replace_from_self_to_ident(&_state)
    }
    pub fn replace_from_self_to_ident(mut self, ident: &syn::Ident) -> Self {
        for tp in self.types.values_mut() {
            for b in tp.bounds.iter_mut() {
                // for `T: Trait<Self>` case
                replace_ident_from_self_to_ident(b, ident);
            }
            if let Some(d) = tp.default.as_mut() {
                // for `T: Trait<Item=Self>` case
                replace_ident_from_self_to_ident(d, ident)
            }
        }

        let mut new_type_bounds = indexmap::IndexMap::new();
        for pt in self.type_bounds.values_mut() {
            // for `Self: Trait` cases
            replace_ident_from_self_to_ident(&mut pt.bounded_ty, ident);
            for b in pt.bounds.iter_mut() {
                // for `T: Trait<Self>` cases
                replace_ident_from_self_to_ident(b, ident);
            }

            new_type_bounds.insert(pt.bounded_ty.clone(), pt.clone());
        }
        self.type_bounds = new_type_bounds;

        self
    }
}
