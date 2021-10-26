use crate::core_impl::{
    info_extractor::attr_sig_info::AttrSigInfo,
    info_extractor::{
        item_trait_info::ItemTraitInfo, trait_item_method_info::TraitItemMethodInfo,
        InputStructType, SerializerType,
    },
};
use crate::error;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl TraitItemMethodInfo {
    /// Generate code that wraps the method.
    pub fn method_wrapper(
        &self,
        original_method_name: &syn::Ident,
        trait_info: &ItemTraitInfo,
    ) -> error::Result<TokenStream2> {
        let method_mod_name = &self.attrs.method_mod_name;
        let attr_docs = &self.doc_attrs;

        //

        let internal_interface = crate::crate_name("contract-interface")?;

        //

        let args_trait_lifetime_idents = trait_info.generics.lifetimes.keys().collect::<Vec<_>>();
        let args_trait_lifetimes = trait_info.generics.lifetimes.values().collect::<Vec<_>>();

        let args_method_lifetime_idents = self.generics.lifetimes.keys().collect::<Vec<_>>();
        let args_method_lifetimes = self.generics.lifetimes.values().collect::<Vec<_>>();

        //

        let args_trait_generic_type_idents = trait_info.generics.types.keys().collect::<Vec<_>>();
        let args_trait_generic_types = trait_info.generics.types.values().collect::<Vec<_>>();

        let args_method_generic_type_idents = self.generics.types.keys().collect::<Vec<_>>();
        let args_method_generic_types = self.generics.types.values().collect::<Vec<_>>();

        //

        let args_trait_generic_const_idents = trait_info.generics.consts.keys().collect::<Vec<_>>();
        let args_trait_generic_consts = trait_info.generics.consts.values().collect::<Vec<_>>();

        //

        let args_method_generic_const_idents = self.generics.consts.keys().collect::<Vec<_>>();
        let args_method_generic_consts = self.generics.consts.values().collect::<Vec<_>>();

        let outer_args = &self.inputs.args;
        let (args, args_forward_attrs): (Vec<_>, Vec<_>) = outer_args
            .iter()
            .map(|a| {
                let mut arg = a.arg.clone();
                arg.attrs.clear();
                let forwarded_attr = &a.attr.forward_attr;
                (arg, quote! { #( # [ #forwarded_attr ] )* })
            })
            .unzip();

        let self_lifetime_bounds = &trait_info.self_lifetime_bounds;
        let self_lifetime_bounds_q = if self_lifetime_bounds.is_empty() {
            quote! {}
        } else {
            quote! {_State: #(#self_lifetime_bounds )+*,}
        };

        let implicit_self_trait_bound = {
            let trait_name = &trait_info.original_ident;
            if !args_trait_lifetime_idents.is_empty()
                || !args_trait_generic_type_idents.is_empty()
                || !args_trait_generic_const_idents.is_empty()
            {
                quote! {
                    _State: #trait_name < //
                      #(#args_trait_lifetime_idents,)*
                      #(#args_trait_generic_type_idents,)*
                      #(#args_trait_generic_const_idents,)*
                    >,
                }
            } else {
                quote! {_State: #trait_name ,}
            }
        };

        let self_trait_bounds = &trait_info.self_trait_bounds;
        let self_trait_bounds_q = if self_trait_bounds.is_empty() {
            quote! {}
        } else {
            quote! {_State: #(#self_trait_bounds )+*,}
        };

        let trait_lifetime_where_clauses = trait_info
            .generics
            .lifetime_bounds
            .values()
            .collect::<Vec<_>>();
        let trait_type_where_clauses = trait_info.generics.type_bounds.values().collect::<Vec<_>>();

        let method_lifetime_where_clauses =
            self.generics.lifetime_bounds.values().collect::<Vec<_>>();
        let method_type_where_clauses = self.generics.type_bounds.values().collect::<Vec<_>>();

        let where_clause = quote! {
            where
                #self_lifetime_bounds_q
                #self_trait_bounds_q
                #implicit_self_trait_bound
                #(#trait_lifetime_where_clauses,)*
                #(#method_lifetime_where_clauses,)*
                #(#method_type_where_clauses,)*
                #(#trait_type_where_clauses,)*
        };

        let near_sdk = crate::crate_name("near-sdk")?;
        let near_sdk_str = crate::crate_name_str("near-sdk")?;
        let account_id_str = format!("{}::AccountId", &near_sdk_str);
        let balance_str = format!("{}::Balance", &near_sdk_str);
        let gas_str = format!("{}::Gas", &near_sdk_str);
        let promise_str = format!("{}::Promise", &near_sdk_str);
        let promise_function_call_str = format!("{}::Promise::function_call()", &near_sdk_str);

        let args_generics_with_bounds = quote! {
            #(#args_trait_lifetimes,)*
            #(#args_method_lifetimes,)*
            _State,
            #(#args_method_generic_types,)*
            #(#args_trait_generic_types,)*
            #(#args_trait_generic_consts,)*
            #(#args_method_generic_consts,)*
        };

        let args_generics_idents = quote! {
            #(#args_trait_lifetime_idents,)*
            #(#args_method_lifetime_idents,)*
            _State,
            #(#args_method_generic_type_idents,)*
            #(#args_trait_generic_type_idents,)*
            #(#args_trait_generic_const_idents,)*
            #(#args_method_generic_const_idents,)*
        };

        let trait_name_str = format!("{}", &trait_info.original_ident);
        let method_name_str = format!("{}", &trait_info.original_ident);
        let method_link_str = format!(
            "[`{}::{}()`]",
            &trait_info.original_ident, original_method_name
        );
        let method_link_dot_str = format!("{}.", &method_link_str);
        let mod_doc_str = format!(" Generated code based on {}.  ", &method_link_str);
        let builder_doc_str = format!(" Builder that can be used by a client contract for making a request into a server contract's {} method.", &method_link_str);

        let return_type = match &self.ret {
            syn::ReturnType::Default => quote! {()},
            syn::ReturnType::Type(_t, ty) => quote! {#ty},
        };

        let receiver_kind = &self.inputs.receiver_kind;
        // let receiver_kind_trait_name = receiver_kind.quote_trait_name();
        let receiver_kind_trait_link_str = receiver_kind.quote_trait_link_str();
        // let receiver_kind_state = receiver_kind.quote_self_argument();

        let args_pats = self
            .inputs
            .args
            .iter()
            .map(|a| a.arg.pat.as_ref())
            .collect::<Vec<_>>();

        let return_serializer = {
            use crate::core_impl::info_extractor::inputs;
            let recv_kind = &self.inputs.receiver_kind;
            if matches!(recv_kind, inputs::ReceiverKind::Owned) {
                quote! {
                    #[derive(_near_sdk::borsh::BorshSerialize)]
                }
            } else {
                quote! {
                    #[derive(_near_sdk::serde::Serialize)]
                    #[serde(crate = "_near_sdk::serde")]
                    #[serde(transparent)]
                }
            }
        };
        let return_serializer_skip = {
            use crate::core_impl::info_extractor::inputs;
            let recv_kind = &self.inputs.receiver_kind;
            if matches!(recv_kind, inputs::ReceiverKind::Owned) {
                quote! {
                    #[borsh_skip]
                }
            } else {
                quote! {
                    #[serde(skip)]
                }
            }
        };
        let return_serializer_bounds = {
            use crate::core_impl::info_extractor::inputs;
            let recv_kind = &self.inputs.receiver_kind;
            if matches!(recv_kind, inputs::ReceiverKind::Owned) {
                quote! {
                    _State: near_sdk::borsh::BorshSerialize
                }
            } else {
                quote!()
            }
        };

        let q = Ok(quote! {
            #[doc = #mod_doc_str]
            #[doc = ""]
            #(#attr_docs)*
            #[allow(non_camel_case_types)]
            pub mod #method_mod_name {
                use super::*;
                use #near_sdk as _near_sdk;
                use #internal_interface as _interface;

                pub use serve::Serve;
                pub use request::Request;

                #[doc = #mod_doc_str]
                #[doc = ""]
                #[doc = " Represents the arguments required by "]
                #[doc = #method_link_dot_str]
                #[doc = ""]
                #(#attr_docs)*
                #[derive(_near_sdk::serde::Serialize, _near_sdk::serde::Deserialize)]
                #[serde(crate = "_near_sdk::serde")]
                pub struct
                Args< //
                    #args_generics_with_bounds
                >
                #where_clause
                {
                    #( #args_forward_attrs pub #args,)*
                    #[serde(skip)]
                    pub _phantom: serve::Serve< //
                        #args_generics_idents
                    >,
                }

                impl <#args_generics_with_bounds> Args<#args_generics_idents>
                #where_clause
                serve::Serve<#args_generics_idents>: Default
                {
                    pub fn new(#(#args,)*) -> Args<#args_generics_idents> {
                        Args {
                            _phantom: serve::Serve::default(),
                            #(#args_pats),*
                        }
                    }
                }

                #[doc = #mod_doc_str]
                #[doc = ""]
                #[doc = " Represents the return given by "]
                #[doc = #method_link_dot_str]
                #[doc = ""]
                #(#attr_docs)*
                #return_serializer
                pub struct Return< //
                    #args_generics_with_bounds
                >
                #where_clause
                #return_serializer_bounds
                {
                    pub value: #return_type,
                    // phantom datas
                    #return_serializer_skip
                    pub _phantom: serve::Serve< //
                        #args_generics_idents
                    >
                }

                #[doc = #mod_doc_str]
                #[doc = ""]
                #(#attr_docs)*
                pub mod serve {
                    use super::*;

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = " Represents all generics information required by "]
                    #[doc = #method_link_dot_str]
                    #[doc = ""]
                    #[doc = " For a server contract to serve the method "]
                    #[doc = #method_link_str]
                    #[doc = ", ie. making an `extern \"C\"` wasm function from it,"]
                    #[doc = " [`Serve`] should implement "]
                    #[doc = #receiver_kind_trait_link_str]
                    #[doc = ", which can be derived from an implementation of [`"]
                    #[doc = #trait_name_str]
                    #[doc = "`] for the server contract struct."]
                    #[doc = ""]
                    #(#attr_docs)*
                    #[derive(Default)]
                    pub struct Serve< //
                        #args_generics_with_bounds
                    >
                    #where_clause
                    {
                        _trait_lifetimes: ( //
                            #(std::marker::PhantomData<&#args_trait_lifetime_idents ()>,)*
                        ),
                        _method_lifetimes: ( //
                            #(std::marker::PhantomData<&#args_method_lifetime_idents ()>,)*
                        ),
                        _state_type: std::marker::PhantomData<_State>,
                        _trait_types: ( //
                            #(std::marker::PhantomData<#args_trait_generic_type_idents>,)*
                        ),
                        _method_types: ( //
                            #(std::marker::PhantomData<#args_method_generic_type_idents>,)*
                        ),
                    }
                }

                #[doc = #mod_doc_str]
                #[doc = ""]
                #[doc = #builder_doc_str]
                #[doc = ""]
                #(#attr_docs)*
                pub mod request {
                    use super::*;

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = #builder_doc_str]
                    #[doc = ""]
                    #[doc = " This represents a request where [the contract]("]
                    #[doc = #account_id_str]
                    #[doc = ") and (possibly) the method being called"]
                    #[doc = " still need to be defined."]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub struct Request<#args_generics_with_bounds>(serve::Serve<#args_generics_idents>)
                    #where_clause;

                    impl<#args_generics_with_bounds> Request<#args_generics_idents>
                    #where_clause
                    serve::Serve<#args_generics_idents>: Default
                    {
                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the `contract_being_called` ([AccountId]("]
                        #[doc = #account_id_str]
                        #[doc = "))"]
                        #[doc = " and the `method_name` (`"]
                        #[doc = #method_name_str]
                        #[doc = "`) being called."]
                        #[doc = ""]
                        #[doc = " See also [`Self::contract_with_renamed_method()`] if the method's name has been renamed."]
                        #(#attr_docs)*
                        pub fn contract(contract_being_called: _near_sdk::AccountId) -> MethodRequest<#args_generics_idents> {
                            MethodRequest {
                                contract_being_called,
                                method_name: "#original_method_name".to_string(),
                                _phantom: serve::Serve::default(),
                            }
                        }

                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the `contract_being_called` ([`AccountId`]("]
                        #[doc = #account_id_str]
                        #[doc = "))"]
                        #[doc = " and the `method_name` being called."]
                        #[doc = ""]
                        #[doc = " See also [`Self::contract()`] if the method's name has not been renamed."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn contract_with_renamed_method(contract_being_called: _near_sdk::AccountId, method_name: String) -> MethodRequest<#args_generics_idents> {
                            MethodRequest {
                                contract_being_called,
                                method_name,
                                _phantom: serve::Serve::default(),
                            }
                        }
                    }

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = #builder_doc_str]
                    #[doc = ""]
                    #[doc = " This represents a request where the arguments for the method being called"]
                    #[doc = " still need to be defined."]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub struct MethodRequest<#args_generics_with_bounds>
                    #where_clause
                    {
                        contract_being_called: _near_sdk::AccountId,
                        method_name: String,
                        _phantom: serve::Serve<#args_generics_idents>
                    }

                    impl<#args_generics_with_bounds> MethodRequest<#args_generics_idents>
                    #where_clause
                    serve::Serve<#args_generics_idents>: Default
                    {
                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the arguments for the call."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn args(self, #(#args,)*) -> ArgsRequest<#args_generics_idents> {
                            let args = Args::new(#(#args_pats),*);
                            ArgsRequest::new(
                                self.method_name,
                                self.contract_being_called,
                                args
                            )
                        }
                    }

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = #builder_doc_str]
                    #[doc = ""]
                    #[doc = " This represents a request where the [amount of `Near`]("]
                    #[doc = #balance_str]
                    #[doc = ") to be sent and"]
                    #[doc = " the [`Gas` quantity]("]
                    #[doc = #gas_str]
                    #[doc = ") to be attached still need to be defined."]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub struct ArgsRequest<#args_generics_with_bounds>
                    #where_clause
                    {
                        method_name: String,
                        contract_being_called: _near_sdk::AccountId,
                        args: Args<#args_generics_idents>,
                    }

                    impl<#args_generics_with_bounds> ArgsRequest<#args_generics_idents>
                    #where_clause
                    {
                        pub fn new(
                            method_name: String,
                            contract_being_called: _near_sdk::AccountId,
                            args: Args<#args_generics_idents>
                        ) -> Self {
                            Self {
                                method_name,
                                contract_being_called,
                                args,
                            }
                        }

                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the [amount of `Near`]("]
                        #[doc = #balance_str]
                        #[doc = ") to be sent for the call."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn send_amount(self, send_amount: _near_sdk::Balance) -> AmountRequest<#args_generics_idents> {
                            AmountRequest {
                                method_name: self.method_name,
                                contract_being_called: self.contract_being_called,
                                args: self.args,
                                send_amount,
                            }
                        }

                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the [`Gas` quantity]("]
                        #[doc = #gas_str]
                        #[doc = ") to be attached for the call,"]
                        #[doc = " while also setting the [amount of `Near`]("]
                        #[doc = #balance_str]
                        #[doc = ") to be sent to zero."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn prepaid_gas(self, maximum_allowed_consumption: _near_sdk::Gas) -> GasRequest<#args_generics_idents> {
                            GasRequest {
                                method_name: self.method_name,
                                contract_being_called: self.contract_being_called,
                                args: self.args,
                                send_amount: 0,
                                prepaid_gas: maximum_allowed_consumption,
                            }
                        }
                    }

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = #builder_doc_str]
                    #[doc = ""]
                    #[doc = " This represents a request where the [`Gas` quantity]("]
                    #[doc = #gas_str]
                    #[doc = ") to be attached still need to be defined."]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub struct AmountRequest<#args_generics_with_bounds>
                    #where_clause
                    {
                        method_name: String,
                        contract_being_called: _near_sdk::AccountId,
                        args: Args<#args_generics_idents>,
                        send_amount: _near_sdk::Balance,
                    }

                    impl<#args_generics_with_bounds> AmountRequest<#args_generics_idents>
                    #where_clause
                    {
                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sets the [`Gas` quantity]("]
                        #[doc = #gas_str]
                        #[doc = ") to be attached for the call."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn prepaid_gas(self, maximum_allowed_consumption: _near_sdk::Gas) -> GasRequest<#args_generics_idents> {
                            GasRequest {
                                method_name: self.method_name,
                                contract_being_called: self.contract_being_called,
                                args: self.args,
                                send_amount: self.send_amount,
                                prepaid_gas: maximum_allowed_consumption,
                            }
                        }
                    }

                    #[doc = #mod_doc_str]
                    #[doc = ""]
                    #[doc = #builder_doc_str]
                    #[doc = ""]
                    #[doc = " This represents a request ready to be sent into the server contract."]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub struct GasRequest<#args_generics_with_bounds>
                    #where_clause
                    {
                        method_name: String,
                        contract_being_called: _near_sdk::AccountId,
                        args: Args<#args_generics_idents>,
                        send_amount: _near_sdk::Balance,
                        prepaid_gas: _near_sdk::Gas,
                    }

                    impl<#args_generics_with_bounds> GasRequest<#args_generics_idents>
                    #where_clause
                    Args<#args_generics_idents>: _interface::ToBytes<_interface::Json>
                    {

                        #[doc = #mod_doc_str]
                        #[doc = ""]
                        #[doc = #builder_doc_str]
                        #[doc = ""]
                        #[doc = " Sends the request into the server contract."]
                        #[doc = ""]
                        #[doc = " Creates a [`Promise`]("]
                        #[doc = #promise_str]
                        #[doc = ") with a "]
                        #[doc = " [`function_call()`]("]
                        #[doc = #promise_function_call_str]
                        #[doc = ") to the server contract."]
                        #[doc = ""]
                        #(#attr_docs)*
                        pub fn request(self) -> _near_sdk::Promise {
                            use _interface::ToBytes;
                            _near_sdk::Promise::new(self.contract_being_called).function_call(
                                self.method_name.to_string(),
                                self.args
                                    .to_bytes()
                                    .expect("Failed to serialize the cross contract args."),
                                self.send_amount,
                                self.prepaid_gas,
                            )
                        }

                    }
                }

            }
        });

        // debugging
        // panic!("{}", q.unwrap());

        q
    }
}
