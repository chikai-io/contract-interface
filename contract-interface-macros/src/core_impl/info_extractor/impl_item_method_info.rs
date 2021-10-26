use super::attr_sig_info::AttrSigInfo;
use super::inputs::Inputs;
use super::item_generics::Generics;
use super::meta_attrs;
use crate::error;
use darling::util::SpannedValue;
use darling::FromMeta;

/// Information extracted from `ImplItemMethod`.
#[derive(Debug)]
pub struct ImplItemMethodInfo {
    /// The original AST of the impl item method.
    pub original: syn::ImplItemMethod,

    pub attrs: Attrs,
    pub doc_attrs: Vec<syn::Attribute>,
    pub forward_attrs: Vec<syn::Attribute>,

    /// The method generics information.
    pub generics: Generics,

    pub inputs: Inputs,

    pub ret: syn::ReturnType,
}

#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    // TODO: use in code generation
    #[darling(default, rename = "mod")]
    module_name: Option<syn::Ident>,

    #[darling(default)]
    init: Option<InitAttr>,

    #[darling(default)]
    payable: Option<bool>,

    #[darling(default)]
    private: Option<bool>,

    #[darling(default)]
    allow_temporary_state: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub struct InitAttr {
    #[darling(default)]
    pub ignore_state: Option<bool>,
}

#[derive(Debug)]
pub struct Attrs {
    /// The trait name that will be used to generate the module.  
    /// eg. `mod name {}`
    pub module_name: syn::Ident,

    pub init: Option<InitAttr>,

    pub payable: bool,

    pub private: bool,

    pub allow_temporary_state: bool,
}

impl ImplItemMethodInfo {
    /// Process the method and extract information important for near-sdk.
    pub fn new(original: &mut syn::ImplItemMethod) -> error::Result<Self> {
        let (contract_attr, non_contract_attr) =
            meta_attrs::partition_attrs(&original.attrs, "contract");
        original.attrs.clear();
        original.attrs = non_contract_attr;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let attrs = {
            let meta_attrs = meta_attrs::into_meta_attrs(contract_attr)?;
            let nested = meta_attrs::remove_first_layer(meta_attrs, "contract")?;
            let attrs = RawAttrs::from_list(&nested)?;

            Attrs {
                module_name: attrs.module_name.unwrap_or_else(|| {
                    let res = original.sig.ident.to_string();
                    syn::Ident::new(&res, proc_macro2::Span::call_site())
                }),
                init: attrs.init,
                payable: matches!(attrs.payable, Some(true)),
                private: matches!(attrs.payable, Some(true)),
                allow_temporary_state: matches!(attrs.allow_temporary_state, Some(true)),
            }
        };

        let generics = Generics::new(&original.sig.generics);

        let inputs = Inputs::new(original.sig.inputs.iter_mut())?;

        if attrs.init.is_some() && inputs.receiver.is_some() {
            use syn::spanned::Spanned;
            return Err(syn::Error::new(
                inputs.receiver.span(),
                "Init methods can't have `self` parameter as it implies an existing state",
            )
            .into());
        };

        if attrs.payable
            && !matches!(
                inputs.receiver_kind,
                super::inputs::ReceiverKind::RefMut | super::inputs::ReceiverKind::Owned
            )
        {
            use syn::spanned::Spanned;
            return Err(syn::Error::new(
                inputs.receiver.span(),
                "Payable methods must be able to change state (`&mut self`, `mut self`, `self`)",
            )
            .into());
        };

        if attrs.allow_temporary_state
            && !matches!(inputs.receiver_kind, super::inputs::ReceiverKind::Ref)
        {
            use syn::spanned::Spanned;
            return Err(syn::Error::new(
                inputs.receiver.span(),
                "States can only be temporary (ie. possibly created by default and then discarded) on methods based on `&self`",
            )
            .into());
        };

        let ret: syn::ReturnType = original.sig.output.clone();

        Ok(Self {
            original: original.clone(),
            attrs,
            doc_attrs,
            forward_attrs,
            generics,
            inputs,
            ret,
        })
    }
}
