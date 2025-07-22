#[cfg(test)]
mod tests {
    use macro_cores::impl_value;

    #[tokio::test]
    async fn impl_value_works() {
        impl_value! {
            enum Pref {
                Hokkaido => "JP-01",
                Tokyo => "JP-13"
            } 
        }

        let pref = Pref::Tokyo;
        assert_eq!(pref.value(), "JP-13");

        let pref = Pref::Hokkaido;
        assert_eq!(pref.value(), "JP-01");
    }
}
