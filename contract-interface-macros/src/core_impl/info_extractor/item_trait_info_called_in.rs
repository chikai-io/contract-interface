use super::attr_docs;
use super::item_generics::Generics;
use super::meta_attrs::meta_attrs;
use super::trait_item_method_info_called_in::TraitItemMethodInfo;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
use darling::FromMeta;
use inflector::Inflector;
use syn::export::Span;

/// Information extracted from `ItemTrait`.
pub struct ItemTraitInfo {
    /// The original AST of the trait.
    pub original: syn::ItemTrait,

    pub attrs: Attrs,
    pub forward_attrs: Vec<syn::Attribute>,

    /// The trait name.  
    /// eg. `trait Name`
    pub original_ident: syn::Ident,

    /// The trait generics information.
    pub generics: Generics,

    /// Self lifetime bounds (from supertrait syntax).  
    /// eg. `trait Trait<'a>: 'a`.
    pub self_lifetime_bounds: Vec<syn::Lifetime>,
    /// Self trait bounds (from supertrait syntax).  
    /// eg. `trait Trait: OtherTrait`.
    pub self_trait_bounds: Vec<syn::TraitBound>,

    /// The trait associated items.
    pub items: TraitItems,
}

#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    #[darling(default, rename = "name")]
    module_name: Option<syn::Ident>,
}

#[derive(Debug)]
pub struct Attrs {
    /// The trait name that will be used to generate the module.  
    /// eg. `mod name {}`
    pub module_name: syn::Ident,
}

pub struct TraitItems {
    /// The trait associated consts.  
    /// eg. `trait Trait {const T: u8}`.
    pub consts: indexmap::IndexMap<syn::Ident, syn::TraitItemConst>,
    /// The trait associated types.  
    /// eg. `trait Trait {type T}`.
    pub types: indexmap::IndexMap<syn::Ident, syn::TraitItemType>,
    /// The trait methods.  
    /// eg. `trait Trait {fn f();}`
    pub methods: indexmap::IndexMap<syn::Ident, TraitItemMethodInfo>,
}

impl TraitItems {
    pub fn replace_from_self_to_state(items: &[syn::TraitItem]) -> error::Result<Self> {
        let consts = items
            .iter()
            .filter_map(|item| {
                if let syn::TraitItem::Const(tic) = item {
                    let mut tic = tic.clone();

                    // for the `const C<T: Trait<Self>>` cases
                    replace_ident_from_self_to_state(&mut tic.ty);

                    if let Some((_, e)) = tic.default.as_mut() {
                        // for the `const X = C<Self>` cases
                        replace_ident_from_self_to_state(e);
                    }

                    Some(tic)
                } else {
                    None
                }
            })
            .map(|tic| (tic.ident.clone(), tic.clone()))
            .collect();

        let types = items
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

        let methods = items
            .iter()
            .filter_map(|ti| {
                if let syn::TraitItem::Method(tim) = ti {
                    Some(tim)
                } else {
                    None
                }
            })
            .map(|tim| Ok((tim.sig.ident.clone(), TraitItemMethodInfo::new(tim)?)))
            .collect::<Result<_, error::Error>>()?;

        Ok(Self {
            consts,
            types,
            methods,
        })
    }
}

impl ItemTraitInfo {
    pub(crate) fn new(
        original: &syn::ItemTrait,
        attr_args: syn::AttributeArgs,
    ) -> error::Result<Self> {
        let original_ident = original.ident.clone();

        let (raw_attrs, forward_attrs) =
            meta_attrs::<RawAttrs>(&original.attrs, attr_args, "contract")?;

        let attrs = Attrs {
            module_name: raw_attrs.module_name.unwrap_or_else(|| {
                let res = original.ident.to_string().to_snake_case();
                syn::Ident::new(&res, Span::call_site())
            }),
        };

        let generics = Generics::replace_from_self_to_state(&original.generics);

        let self_lifetime_bounds = original
            .supertraits
            .iter()
            .filter_map(|tpb| {
                if let syn::TypeParamBound::Lifetime(l) = tpb {
                    Some(l)
                } else {
                    None
                }
            })
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

        let items = TraitItems::replace_from_self_to_state(&original.items)?;

        Ok(Self {
            original_ident,
            attrs,
            forward_attrs,
            original: original.clone(),
            generics,
            self_lifetime_bounds,
            self_trait_bounds,
            items,
        })
    }
}
