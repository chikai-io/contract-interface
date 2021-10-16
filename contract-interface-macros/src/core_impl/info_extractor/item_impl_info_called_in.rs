use super::impl_item_method_info_called_in::ImplItemMethodInfo;
use super::item_generics::Generics;
use super::meta_attrs;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
use darling::FromMeta;

/// Information extracted from `impl` section.
pub struct ItemImplInfo {
    /// The original AST.
    pub original: syn::ItemImpl,

    pub attrs: Attrs,
    pub doc_attrs: Vec<syn::Attribute>,
    pub forward_attrs: Vec<syn::Attribute>,

    /// The impl's generics information.
    pub generics: Generics,

    /// The trait which is being impl (in case).
    /// eg. `impl Trait for Struct {}`
    pub trait_path: Option<syn::Path>,

    /// The type for which this `impl` is written.
    /// eg. `impl Struct {}`
    pub self_ty: syn::Type,

    pub items: ImplItems,
}

#[derive(Debug, FromMeta)]
pub struct Attrs {
    /// The struct name that will be used to generate the module.  
    /// eg. `mod name {}`
    #[darling(rename = "name")]
    pub module_name: syn::Ident,
}

pub struct ImplItems {
    /// The trait associated consts.  
    /// eg. `trait Trait {const T: u8}`.
    pub consts: indexmap::IndexMap<syn::Ident, syn::ImplItemConst>,
    /// The trait associated types.  
    /// eg. `trait Trait {type T}`.
    pub types: indexmap::IndexMap<syn::Ident, syn::ImplItemType>,
    /// The trait methods.  
    /// eg. `trait Trait {fn f();}`
    pub methods: indexmap::IndexMap<syn::Ident, ImplItemMethodInfo>,
}

impl ImplItems {
    pub fn replace_from_self_to_state(items: &[syn::ImplItem]) -> error::Result<Self> {
        let consts = items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Const(tic) = item {
                    let mut tic = tic.clone();

                    // for the `const C<T: Trait<Self>>` cases
                    replace_ident_from_self_to_state(&mut tic.ty);

                    // for the `const X = C<Self>` cases
                    replace_ident_from_self_to_state(&mut tic.expr);

                    Some(tic)
                } else {
                    None
                }
            })
            .map(|tic| (tic.ident.clone(), tic))
            .collect();

        let types = items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Type(tit) = item {
                    let mut tit = tit.clone();

                    // for the `type X<T: Trait<Self>>` cases
                    replace_ident_from_self_to_state(&mut tit.generics);

                    // for the `type X = Self` cases
                    replace_ident_from_self_to_state(&mut tit.ty);

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
                if let syn::ImplItem::Method(tim) = ti {
                    Some(tim)
                } else {
                    None
                }
            })
            .map(|tim| Ok((tim.sig.ident.clone(), ImplItemMethodInfo::new(tim)?)))
            .collect::<Result<_, error::Error>>()?;

        Ok(Self {
            consts,
            types,
            methods,
        })
    }
}

impl ItemImplInfo {
    pub(crate) fn new(
        original: &syn::ItemImpl,
        attr_args: syn::AttributeArgs,
    ) -> error::Result<Self> {
        let (attrs, forward_attrs) =
            meta_attrs::meta_attrs::<Attrs>(&original.attrs, attr_args, "contract")?;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let generics = Generics::replace_from_self_to_state(&original.generics);

        let self_ty = (*original.self_ty.as_ref()).clone();
        let trait_path = original.trait_.as_ref().map(|(_, p, _)| p);

        let items = ImplItems::replace_from_self_to_state(&original.items)?;

        Ok(Self {
            original: original.clone(),
            attrs,
            doc_attrs,
            forward_attrs,
            self_ty,
            generics,
            trait_path: trait_path.cloned(),
            items,
        })
    }
}
