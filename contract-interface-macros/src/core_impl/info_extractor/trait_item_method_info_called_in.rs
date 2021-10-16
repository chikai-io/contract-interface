use super::inputs::Inputs;
use super::item_generics::Generics;
use super::meta_attrs;
use crate::error;
use darling::FromMeta;

/// Information extracted from trait method.
pub struct TraitItemMethodInfo {
    /// The original AST of the trait item method.
    pub original: syn::TraitItemMethod,

    pub attrs: Attrs,
    pub doc_attrs: Vec<syn::Attribute>,
    pub forward_attrs: Vec<syn::Attribute>,

    /// The method generics information.
    pub generics: Generics,

    pub inputs: Inputs,
    // pub args: indexmap::IndexMap<syn::Ident, syn::PatType>,
    // pub args_sets: ArgsSets,
    //
    // /// Attributes and signature information.
    // pub attr_sig_info: AttrSigInfo,
    // /// String representation of method name, e.g. `"my_method"`.
    // pub ident_byte_str: LitStr,
}

#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    #[darling(default, rename = "name")]
    method_name: Option<syn::Ident>,
}

#[derive(Debug)]
pub struct Attrs {
    /// The method name.  
    /// eg. `fn name() {}`
    pub method_name: syn::Ident,
}

impl TraitItemMethodInfo {
    pub fn new(original: &syn::TraitItemMethod) -> error::Result<Self> {
        let (raw_attrs, forward_attrs) =
            meta_attrs::meta_attrs::<RawAttrs>(&original.attrs, vec![], "contract")?;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let attrs = Attrs {
            method_name: raw_attrs.method_name.unwrap_or_else(|| {
                let res = original.sig.ident.to_string();
                syn::Ident::new(&res, syn::export::Span::call_site())
            }),
        };

        let generics = Generics::replace_from_self_to_state(&original.sig.generics);

        let inputs = Inputs::replace_from_self_to_state(original.sig.inputs.iter());

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
    }
}
