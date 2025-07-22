use proc_macro::TokenStream;
use macro_cores::derives::enum_util::*;
use macro_cores::prelude::*;


/// This procedural macro which can implementation `ToString` for Enum.
/// 
/// Usage:
/// 
/// ```rust
/// #[derive(VariantToString)]
/// enum Alphabet {
///    Aaa,
///    Bbb,
///    Ccc(#[allow(unused)] String)
///}
/// 
/// let a = Alphabet::Aaa;
/// assert_eq!("Aaa", a.to_string());        
/// let b = Alphabet::Bbb;
/// assert_eq!("Bbb", b.to_string());
/// 
/// let c = Alphabet::Ccc("aaa".into());
/// assert_eq!("Ccc", c.to_string());
/// ```
/// 
#[proc_macro_derive(VariantToString)]
pub fn v_to_s(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match variant_name_to_str(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into()
    }
}

