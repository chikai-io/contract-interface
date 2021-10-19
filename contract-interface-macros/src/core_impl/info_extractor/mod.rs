pub(crate) mod inputs;
pub(crate) mod item_generics;
pub(crate) mod meta_attrs;

pub(crate) mod arg_info;
pub(crate) mod attr_sig_info;
pub(crate) mod impl_item_method_info;
pub(crate) mod init_attr;
pub(crate) mod item_impl_info;
pub(crate) mod item_trait_info;
pub(crate) mod serializer_attr;
pub(crate) mod trait_item_method_info;

/// Type of serialization we use.
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum SerializerType {
    JSON,
    Borsh,
}

/// Type of the method.
#[derive(PartialEq, Eq)]
pub enum MethodType {
    Regular,
    View,
    Init,
    InitIgnoreState,
}

/// Whether the input struct is used for serialization or deserialization.
#[derive(PartialEq, Eq)]
pub enum InputStructType {
    Serialization,
    Deserialization,
}
