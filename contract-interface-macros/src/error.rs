pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Syn(syn::Error),
    Darling(darling::Error),
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Self::Syn(e)
    }
}

impl From<darling::Error> for Error {
    fn from(e: darling::Error) -> Self {
        Self::Darling(e)
    }
}

impl Error {
    pub fn into_token_stream(self) -> proc_macro::TokenStream {
        let ts2: proc_macro2::TokenStream = self.into_token_stream2();
        ts2.into()
    }
    pub fn into_token_stream2(self) -> proc_macro2::TokenStream {
        match self {
            Error::Syn(s) => s.to_compile_error(),
            Error::Darling(d) => d.write_errors(),
        }
    }
}

// impl Into<proc_macro2::TokenStream> for Error {
//     fn into(self) -> proc_macro2::TokenStream {
//         match self {
//             Error::Syn(s) => s.to_compile_error(),
//             Error::Darling(d) => d.write_errors(),
//         }
//     }
// }

// impl Into<proc_macro::TokenStream> for Error {
//     fn into(self) -> proc_macro::TokenStream {
//         let ts2: proc_macro2::TokenStream = self.into();
//         ts2.into()
//     }
// }
