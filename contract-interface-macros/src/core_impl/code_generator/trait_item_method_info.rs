use crate::core_impl::{
    info_extractor::attr_sig_info::AttrSigInfo,
    info_extractor::{
        item_trait_info::ItemTraitInfo, trait_item_method_info::TraitItemMethodInfo,
        InputStructType, SerializerType,
    },
};
use crate::error;
use quote::quote;
use syn::export::TokenStream2;

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
        let method_type_where_clauses = self.generics.lifetime_bounds.values().collect::<Vec<_>>();

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

        /*
        let ident = &self.attr_sig_info.ident;
        let ident_byte_str = &self.ident_byte_str;
        let pat_type_list = self.attr_sig_info.pat_type_list();
        let serialize = TraitItemMethodInfo::generate_serialier(
            &self.attr_sig_info,
            &self.attr_sig_info.result_serializer,
        );
        quote! {
            pub fn #ident(#pat_type_list __account_id: AccountId, __balance: near_sdk::Balance, __gas: near_sdk::Gas) -> near_sdk::Promise {
                #serialize
                near_sdk::Promise::new(__account_id)
                .function_call(
                    #ident_byte_str.to_string(),
                    args,
                    __balance,
                    __gas,
                )
            }
        }
        */

        let near_sdk = crate::crate_name("near-sdk")?;

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

        let mod_doc_msg = format!(
            " Generated code based on [`{}::{}()`].",
            &trait_info.original_ident, original_method_name
        );

        let q = Ok(quote! {
            #[doc = #mod_doc_msg]
            #[doc = ""]
            #(#attr_docs)*
            #[allow(non_camel_case_types)]
            pub mod #method_mod_name {
                use super::*;
                use #near_sdk as _near_sdk;

                #[doc = #mod_doc_msg]
                #[doc = ""]
                #(#attr_docs)*
                #[derive(_near_sdk::serde::Deserialize)]
                #[serde(crate = "_near_sdk::serde")]
                pub struct
                Args< //
                    #args_generics_with_bounds
                >
                #where_clause
                {
                    #( #args_forward_attrs pub #args,)*
                    #[serde(skip)]
                    pub _phantom: CalledIn< //
                        #args_generics_idents
                    >,
                }

                #[doc = #mod_doc_msg]
                #[doc = ""]
                #(#attr_docs)*
                pub type Return<Z> = Z;

                #[doc = #mod_doc_msg]
                #[doc = ""]
                #(#attr_docs)*
                #[derive(Default)]
                pub struct CalledIn< //
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
        });

        // debugging
        // panic!("{}", q.unwrap());

        q
    }

    pub fn generate_serialier(
        attr_sig_info: &AttrSigInfo,
        serializer: &SerializerType,
    ) -> error::Result<TokenStream2> {
        let has_input_args = attr_sig_info.input_args().next().is_some();
        if !has_input_args {
            return Ok(quote! { let args = vec![]; });
        }
        let struct_decl = attr_sig_info.input_struct(InputStructType::Serialization)?;
        let constructor_call = attr_sig_info.constructor_expr()?;
        let constructor = quote! { let args = #constructor_call; };
        let value_ser = match serializer {
            SerializerType::JSON => quote! {
                let args = near_sdk::serde_json::to_vec(&args).expect("Failed to serialize the cross contract args using JSON.");
            },
            SerializerType::Borsh => quote! {
                let args = near_sdk::borsh::BorshSerialize::try_to_vec(&args).expect("Failed to serialize the cross contract args using Borsh.");
            },
        };

        Ok(quote! {
          #struct_decl
          #constructor
          #value_ser
        })
    }
}
