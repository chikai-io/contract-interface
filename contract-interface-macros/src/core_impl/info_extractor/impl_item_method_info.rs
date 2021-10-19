use super::attr_sig_info::AttrSigInfo;
use super::inputs::Inputs;
use super::item_generics::Generics;
use super::meta_attrs;
use crate::error;
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
    // /// Information on the attributes and the signature of the method.
    // pub attr_signature_info: AttrSigInfo,
    // /// Whether method has `pub` modifier.
    // pub is_public: bool,
    // /// The type of the contract struct.
    // pub struct_type: Type,
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

impl ImplItemMethodInfo {
    /// Process the method and extract information important for near-sdk.
    pub fn new(original: &mut syn::ImplItemMethod) -> error::Result<Self> {
        let ident = original.sig.ident.clone();

        let (raw_attrs, forward_attrs) =
            meta_attrs::meta_attrs::<RawAttrs>(&original.attrs, vec![], "contract")?;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let attrs = Attrs {
            module_name: raw_attrs.module_name.unwrap_or_else(|| {
                let res = original.sig.ident.to_string();
                syn::Ident::new(&res, syn::export::Span::call_site())
            }),
        };

        let generics = Generics::new(&original.sig.generics);

        let inputs = Inputs::new(original.sig.inputs.iter_mut())?;

        // let attr_sig_info = AttrSigInfo::new(attrs, sig)?;

        // let ident_byte_str = LitStr::new(&attr_sig_info.ident.to_string(), Span::call_site());

        Ok(Self {
            original: original.clone(),
            attrs,
            doc_attrs,
            forward_attrs,
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
