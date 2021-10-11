pub(crate) mod serializer_attr;
pub(crate) mod serializer_attr_call_out;
pub(crate) mod serializer_attr_called_in;
// pub use serializer_attr::SerializerAttr;

pub(crate) mod arg_info;
pub(crate) mod arg_info_call_out;
pub(crate) mod arg_info_called_in;
// pub use arg_info::{ArgInfo, BindgenArgType};

pub(crate) mod attr_sig_info;
pub(crate) mod attr_sig_info_call_out;
pub(crate) mod attr_sig_info_called_in;
// pub use attr_sig_info::AttrSigInfo;

pub(crate) mod impl_item_method_info;
pub(crate) mod impl_item_method_info_call_out;
pub(crate) mod impl_item_method_info_called_in;
// pub use impl_item_method_info::ImplItemMethodInfo;

pub(crate) mod trait_item_method_info;
pub(crate) mod trait_item_method_info_call_out;
pub(crate) mod trait_item_method_info_called_in;
// pub use trait_item_method_info::*;

pub(crate) mod item_trait_info;
pub(crate) mod item_trait_info_call_out;
pub(crate) mod item_trait_info_called_in;
// pub use item_trait_info::ItemTraitInfo;

pub(crate) mod item_impl_info;
pub(crate) mod item_impl_info_call_out;
pub(crate) mod item_impl_info_called_in;

pub(crate) mod init_attr;
pub(crate) mod init_attr_call_out;
pub(crate) mod init_attr_called_in;
// pub use init_attr::InitAttr;

// pub use item_impl_info::ItemImplInfo;

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
