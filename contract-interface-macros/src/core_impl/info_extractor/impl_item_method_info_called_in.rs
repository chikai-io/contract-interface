use super::attr_docs;
use super::attr_sig_info_called_in::AttrSigInfo;
use super::inputs::Inputs;
use super::item_generics::Generics;
use crate::error;

/// Information extracted from `ImplItemMethod`.
pub struct ImplItemMethodInfo {
    /// The original AST of the impl item method.
    pub original: syn::ImplItemMethod,

    /// Method name.
    pub ident: syn::Ident,

    /// The method documentation.
    /// eg. `#[doc = "My Documentation"] fn f() {}`
    pub docs: Vec<syn::Lit>,

    /// The method generics information.
    pub generics: Generics,

    pub inputs: Inputs,
    // /// Information on the attributes and the signature of the method.
    // pub attr_signature_info: AttrSigInfo,
    // /// Whether method has `pub` modifier.
    // pub is_public: bool,
    // /// The type of the contract struct.
    // pub struct_type: Type,
}

impl ImplItemMethodInfo {
    /// Process the method and extract information important for near-sdk.
    pub fn new(original: &syn::ImplItemMethod) -> error::Result<Self> {
        let ident = original.sig.ident.clone();

        let docs = attr_docs::parse_attr_docs(&original.attrs)?;

        let generics = Generics::replace_from_self_to_state(&original.sig.generics);

        let inputs = Inputs::replace_from_self_to_state(original.sig.inputs.iter());

        // let attr_sig_info = AttrSigInfo::new(attrs, sig)?;

        // let ident_byte_str = LitStr::new(&attr_sig_info.ident.to_string(), Span::call_site());

        Ok(Self {
            original: original.clone(),
            ident,
            docs,
            generics,
            inputs,
            // args_sets,
            // attr_sig_info,
            // ident_byte_str
        })

        // let ImplItemMethod { attrs, sig, .. } = original;
        // let attr_signature_info = AttrSigInfo::new(attrs, sig)?;
        // let is_public = matches!(original.vis, Visibility::Public(_));
        // Ok(Self {
        //     attr_signature_info,
        //     is_public,
        //     struct_type,
        // })
    }
}
