pub mod en;
pub mod jp;


const BRANK: &'static str = "";

#[non_exhaustive]
#[derive(macros::VariantToString)]
pub enum Country {
    JP(Option<jp::Prefecture>),
    EN,
}

impl Country {
    /// func name is like: 
    ///     Japan  -> 県
    ///     US     -> State
    ///     Canada -> Province
    ///     China  -> 省
    pub fn subnational_division_code(&self) -> &'static str {
        match self {
            Country::JP(prefecture) => prefecture
                .map_or(BRANK, |p| p.value()),
            Country::EN => unimplemented!("I WILL IMPL IT"),
        }
    }
}
