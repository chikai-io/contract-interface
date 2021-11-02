use super::init_attr;
use super::inputs::Inputs;
use super::item_generics::Generics;
use super::meta_attrs;
use crate::error;
use crate::replace_ident::replace_ident_from_self_to_state;
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

    pub ret: syn::ReturnType,
}

#[derive(Debug, FromMeta)]
pub struct RawAttrs {
    /// The name that will be used for the module that will contain
    /// the generated items.
    #[darling(default, rename = "mod")]
    pub method_mod_name: Option<syn::Ident>,

    #[darling(default)]
    pub init: Option<init_attr::InitAttr>,

    /// Forward attributes to be attached into the `Args` structure.
    #[darling(default)]
    pub args_attr: Option<syn::Meta>,

    /// Forward attributes to be attached into the `Return` structure.
    #[darling(default)]
    pub return_attr: Option<syn::Meta>,
}

#[derive(Debug)]
pub struct Attrs {
    /// The name that will be used for the module that will contain
    /// the generated items.
    pub method_mod_name: syn::Ident,

    pub init: Option<init_attr::InitAttr>,

    // TODO: use value on code gen
    /// Forward attributes to be attached into the `Args` structure.
    args_attr: Vec<syn::NestedMeta>,

    // TODO: use value on code gen
    /// Forward attributes to be attached into the `Return` structure.
    return_attr: Vec<syn::NestedMeta>,
}

impl TraitItemMethodInfo {
    pub fn new(original: &mut syn::TraitItemMethod) -> error::Result<Self> {
        let (contract_attr, non_contract_attr) =
            meta_attrs::partition_attrs(&original.attrs, "contract");
        original.attrs.clear();
        original.attrs = non_contract_attr;
        let (doc_attrs, forward_attrs) = meta_attrs::partition_attrs(&original.attrs, "doc");

        let attrs = {
            let meta_attrs = meta_attrs::into_meta_attrs(contract_attr)?;
            let nested = meta_attrs::remove_first_layer(meta_attrs, "contract")?;

            let attrs = RawAttrs::from_list(&nested)?;

            let args_attr = if let Some(fa) = attrs.args_attr {
                meta_attrs::remove_first_layer(vec![fa], "args_attr")?
            } else {
                vec![]
            };
            let return_attr = if let Some(fa) = attrs.return_attr {
                meta_attrs::remove_first_layer(vec![fa], "return_attr")?
            } else {
                vec![]
            };

            Attrs {
                method_mod_name: attrs.method_mod_name.unwrap_or_else(|| {
                    let res = original.sig.ident.to_string();
                    syn::Ident::new(&res, proc_macro2::Span::call_site())
                }),
                init: attrs.init,
                args_attr,
                return_attr,
            }
        };

        let generics = Generics::new(&original.sig.generics).replace_from_self_to_state();

        let inputs = Inputs::new(original.sig.inputs.iter_mut(), attrs.init.is_some())?
            .replace_from_self_to_state();

        if attrs.init.is_some() && inputs.receiver.is_some() {
            use syn::spanned::Spanned;
            return Err(syn::Error::new(
                inputs.receiver.span(),
                "Init methods can't have `self` parameter as it implies an existing state",
            )
            .into());
        };

        let mut ret: syn::ReturnType = original.sig.output.clone();
        replace_ident_from_self_to_state(&mut ret);

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
