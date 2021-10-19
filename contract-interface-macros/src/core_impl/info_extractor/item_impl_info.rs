use super::impl_item_method_info::ImplItemMethodInfo;
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
    /// The name that will be used for the module that will contain
    /// the generated items.
    #[darling(rename = "mod")]
    pub module_name: syn::Ident,

    // TODO: decide if this attribute is necessary
    // (may still be useful, even with `serve` set,
    // to check that no default implementation is being used)
    //
    /// The path to the module (generated from the trait) being
    /// implemented.
    ///
    /// Use this if you intend to specialize the state of the items
    /// from that module.  
    ///
    /// To define the generated module's items independently,
    /// you can still set the `serve` attribute.
    #[darling(rename = "trait", default)]
    pub trait_mod_path: Option<syn::Path>,

    /// Whether this struct/trait's methods should potentially be
    /// served/exposed by the generated wasm.
    ///
    /// Use this if other users or contracts shall call or make
    /// requests to this struct/trait's methods of your deployed
    /// wasm file.
    #[darling(default)]
    serve: bool,

    /// Whether this struct/trait's methods should potentially be
    /// callable by the generated wasm.
    ///
    /// Use this if you intend to make requests into a deployed
    /// contract that is serving this struct/trait's methods.
    #[darling(default)]
    request: bool,
}

#[derive(Debug)]
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
    pub fn get_items(items: &mut [syn::ImplItem]) -> error::Result<Self> {
        let consts = items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Const(tic) = item {
                    Some(tic.clone())
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
                    Some(tit.clone())
                } else {
                    None
                }
            })
            .map(|tit| (tit.ident.clone(), tit))
            .collect();

        let methods = items
            .iter_mut()
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
        original: &mut syn::ItemImpl,
        attr_args: syn::AttributeArgs,
    ) -> error::Result<Self> {
        let (attrs, forward_attrs) =
            meta_attrs::meta_attrs::<Attrs>(&original.attrs, attr_args, "contract")?;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let generics = Generics::new(&original.generics);

        let self_ty = (*original.self_ty.as_ref()).clone();
        let trait_path = original.trait_.as_ref().map(|(_, p, _)| p);

        let items = ImplItems::get_items(&mut original.items)?;

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
