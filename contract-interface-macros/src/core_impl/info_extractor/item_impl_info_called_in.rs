use super::attr_docs;
use super::impl_item_method_info_called_in::ImplItemMethodInfo;
use super::item_generics::Generics;
use crate::replace_ident::replace_ident_from_self_to_state;
use syn::spanned::Spanned;
use syn::{Error, ImplItem, ItemImpl, Type};

/// Information extracted from `impl` section.
pub struct ItemImplInfo {
    /// The original AST.
    pub original: ItemImpl,

    /// The struct name that will be used to generate the module.  
    /// eg. `mod name`
    pub ident: syn::Ident,

    /// The impl documentation.
    /// eg. `#[doc = "My Documentation"] impl Struct {}`
    pub docs: Vec<syn::Lit>,

    /// The impl's generics information.
    pub generics: Generics,

    /// The trait which is being impl (in case).
    /// eg. `impl Trait for Struct {}`
    pub trait_path: Option<syn::Path>,

    /// The type for which this `impl` is written.
    /// eg. `impl Struct {}`
    pub self_ty: Type,

    pub items: ImplItems,
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
    pub fn replace_from_self_to_state(items: &[syn::ImplItem]) -> syn::Result<Self> {
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
            .map(|tic| (tic.ident.clone(), tic.clone()))
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
            .collect::<Result<_, syn::Error>>()?;

        Ok(Self {
            consts,
            types,
            methods,
        })
    }
}

impl ItemImplInfo {
    pub fn new(original: &ItemImpl) -> syn::Result<Self> {
        let ident = {
            use crate::get_ident::GetIdent;
            original.self_ty.get_ident()
        }
        .expect("expecting mod name");

        let docs = attr_docs::parse_attr_docs(&original.attrs)?;

        let generics = Generics::replace_from_self_to_state(&original.generics);

        let self_ty = (*original.self_ty.as_ref()).clone();
        let trait_path = original.trait_.as_ref().map(|(_, p, _)| p);

        let items = ImplItems::replace_from_self_to_state(&original.items)?;

        Ok(Self {
            original: original.clone(),
            ident,
            self_ty,
            docs,
            generics,
            trait_path: trait_path.cloned(),
            items,
        })
    }
}
