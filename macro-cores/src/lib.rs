pub mod derives;
pub mod procedure;


pub mod prelude {
    pub use proc_macro2::TokenStream;
    pub use syn::{DeriveInput, parse_macro_input};   
    pub use quote::quote;
    
    pub type MacroResult = Result<TokenStream, syn::Error>;
}

