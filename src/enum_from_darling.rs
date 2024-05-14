use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantFields>,
}

#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);

    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(variants),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };

    // let EnumFromDarling {
    //     ident,
    //     generics,
    //     data,
    // } = EnumFromDarling::from_derive_input(&input).expect("can not parse input");

    // let variants = match data {
    //     Data::Enum(data) => data,
    //     _ => panic!("EnumFromDarling only works on enums"),
    // };

    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_enum_from_darling() {
        let derive_input = syn::parse_str(
            r#"
            #[derive(Debug, EnumFromDarling)]
            #[allow(dead_code)]
            enum Direction {
                Up(DirectionUp),
                Down(i32),
            }
        "#,
        )
        .unwrap();

        if let EnumFromDarling {
            ident,
            generics,
            data: Data::Enum(variants),
        } = EnumFromDarling::from_derive_input(&derive_input).unwrap()
        {
            println!("{:#?}", generics);
            // println!("{:#?}", variants);
            assert_eq!(ident, "Direction");
            let mut variants = variants.iter();
            match (variants.next(), variants.next()) {
                (Some(up), Some(down)) => {
                    assert_eq!(up.ident, "Up");
                    assert_eq!(down.ident, "Down");
                }
                _ => panic!("Here we should get two fields (up and down)"),
            }
        } else {
            panic!("EnumFromDarling only works on enums");
        }
    }
}
