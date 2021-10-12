use super::attr_docs;
use super::inputs::Inputs;
use super::item_generics::Generics;

/// Information extracted from trait method.
pub struct TraitItemMethodInfo {
    /// The original AST of the trait item method.
    pub original: syn::TraitItemMethod,

    /// Method name.
    pub ident: syn::Ident,

    /// The method documentation.
    /// eg. `#[doc = "My Documentation"] fn f() {}`
    pub docs: Vec<syn::Lit>,

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

impl TraitItemMethodInfo {
    pub fn new(original: &syn::TraitItemMethod) -> syn::Result<Self> {
        let ident = original.sig.ident.clone();

        let docs = attr_docs::parse_attr_docs(&original.attrs)?;

        let generics = Generics::replace_from_self_to_state(&original.sig.generics);

        let inputs = Inputs::replace_from_self_to_state(original.sig.inputs.iter());

        // let attr_sig_info = AttrSigInfo::new(attrs, sig)?;

        // let ident_byte_str = LitStr::new(&attr_sig_info.ident.to_string(), Span::call_site());

        Ok(Self {
            ident,
            original: original.clone(),
            docs,
            generics,
            inputs,
            // args_sets,
            // attr_sig_info,
            // ident_byte_str
        })
    }
}
