use crate::prelude::*;


pub fn variant_name_to_str(input: DeriveInput) -> MacroResult {
    let name = &input.ident;
    let variants = match input.data {
        syn::Data::Enum(ref data) => {
            data.variants
                .iter()
                .map(|v| {
                    let variant = &v.ident;
                    match &v.fields {
                        syn::Fields::Named(_) => quote! {
                            #name::#variant(_) => stringify!(#variant).into()                            
                        },
                        syn::Fields::Unnamed(_) => quote! {
                            #name::#variant(_) => stringify!(#variant).into()
                        },
                        syn::Fields::Unit => quote! {
                            #name::#variant => stringify!(#variant).into()
                        }
                    }
                })
        }
        _ => panic!("Variants can only be used with Enums")
    };

    let func = quote! {
        impl std::string::ToString for #name {
            fn to_string(&self) -> String {
                match self {
                    #(#variants),*
                }
            }
        }
    };

    Ok(func.into())
}


pub fn variants_to_iter(input: DeriveInput) -> MacroResult {
    let name = &input.ident;
    let variants = match input.data {
        syn::Data::Enum(ref data) => 
            data.variants
                .iter()
                .map(|v| &v.ident)
                .collect::<Vec<_>>(),
        _ => panic!("")
    };
    
    let func = quote! {
        impl #name {
            pub fn variants() -> impl Iterator<Item = #name> {
                vec![#(Self::#variants),*].into_iter()
            }
        }
    };

    Ok(func.into())
}