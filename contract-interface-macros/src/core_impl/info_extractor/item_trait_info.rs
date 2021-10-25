use super::item_generics::Generics;
use super::meta_attrs;
use super::trait_item_method_info::TraitItemMethodInfo;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
use darling::FromMeta;
use inflector::Inflector;

/// Information extracted from `ItemTrait`.
pub struct ItemTraitInfo {
    // TODO: ahve an alternative to original,
    // with de #[contract] attributes filtered out
    //
    /// The original AST of the trait.
    pub original: syn::ItemTrait,

    pub attrs: Attrs,
    pub doc_attrs: Vec<syn::Attribute>,
    pub non_contract_attrs: Vec<syn::Attribute>,

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
    /// The name that will be used for the module that will contain
    /// the generated items.
    #[darling(default, rename = "mod")]
    module_name: Option<syn::Ident>,

    /// Whether this trait's methods should potentially be
    /// served/extern by the generated wasm.
    ///
    /// Use this if other users or contracts shall call or make
    /// requests to this trait's methods of your deployed wasm file.
    #[darling(default)]
    serve: bool,

    /// Whether this trait's methods should potentially be callable
    /// by the generated wasm.
    ///
    /// Use this if you intend to make requests into a deployed
    /// contract that is serving this trait's methods.
    #[darling(default)]
    request: bool,
}

#[derive(Debug)]
pub struct Attrs {
    /// The name that will be used for the module that will contain
    /// the generated items.
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
    pub fn replace_from_self_to_state(items: &mut [syn::TraitItem]) -> error::Result<Self> {
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
            .iter_mut()
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
        original: &mut syn::ItemTrait,
        attr_args: syn::AttributeArgs,
    ) -> error::Result<Self> {
        let original_ident = original.ident.clone();

        let (raw_attrs, non_contract__attrs) =
            meta_attrs::meta_attrs::<RawAttrs>(&original.attrs, attr_args, "contract")?;
        let (doc_attrs, non_contract_attrs) =
            meta_attrs::partition_attrs(&non_contract__attrs, "doc");

        let attrs = Attrs {
            module_name: raw_attrs.module_name.unwrap_or_else(|| {
                let res = original.ident.to_string().to_snake_case();
                syn::Ident::new(&res, proc_macro2::Span::call_site())
            }),
        };

        let generics = Generics::new(&original.generics).replace_from_self_to_state();

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

        let items = TraitItems::replace_from_self_to_state(&mut original.items)?;

        Ok(Self {
            original_ident,
            attrs,
            doc_attrs,
            non_contract_attrs,
            original: original.clone(),
            generics,
            self_lifetime_bounds,
            self_trait_bounds,
            items,
        })
    }
}
