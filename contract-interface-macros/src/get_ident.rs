pub trait GetIdent {
    /// Gets an identifier that can be used as default
    /// module name for the given structure.
    fn get_ident(&self) -> Option<syn::Ident>;
}

impl GetIdent for syn::Type {
    fn get_ident(&self) -> Option<syn::Ident> {
        use syn::Type;

        match self {
            Type::Array(_t) => None,
            Type::BareFn(_t) => None,
            Type::Group(t) => t.elem.as_ref().get_ident(),
            Type::ImplTrait(_t) => None,
            Type::Infer(_t) => None,
            Type::Macro(_t) => None,
            Type::Never(_t) => None,
            Type::Paren(t) => t.elem.as_ref().get_ident(),
            Type::Path(t) => {
                if let Some(_q) = t.qself.as_ref() {
                    None
                } else {
                    t.path.get_ident().cloned()
                }
            }
            Type::Ptr(t) => t.elem.as_ref().get_ident(),
            Type::Reference(t) => t.elem.as_ref().get_ident(),
            Type::Slice(_t) => None,
            Type::TraitObject(_t) => None,
            Type::Tuple(_t) => None,
            Type::Verbatim(_t) => None,
            _ => None,
        }
    }
}
