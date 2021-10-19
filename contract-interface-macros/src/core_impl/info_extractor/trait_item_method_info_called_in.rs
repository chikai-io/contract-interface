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
    /// The name that will be used for the module that will contain
    /// the generated items.
    #[darling(default, rename = "mod")]
    method_mod_name: Option<syn::Ident>,
}

#[derive(Debug)]
pub struct Attrs {
    /// The name that will be used for the module that will contain
    /// the generated items.
    pub method_mod_name: syn::Ident,
}

impl TraitItemMethodInfo {
    pub fn new(original: &mut syn::TraitItemMethod) -> error::Result<Self> {
        let (raw_attrs, forward_attrs) =
            meta_attrs::meta_attrs::<RawAttrs>(&original.attrs, vec![], "contract")?;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let attrs = Attrs {
            method_mod_name: raw_attrs.method_mod_name.unwrap_or_else(|| {
                let res = original.sig.ident.to_string();
                syn::Ident::new(&res, syn::export::Span::call_site())
            }),
        };

        let generics = Generics::new(&original.sig.generics).replace_from_self_to_state();

        let inputs = Inputs::new(original.sig.inputs.iter_mut())?.replace_from_self_to_state();

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
