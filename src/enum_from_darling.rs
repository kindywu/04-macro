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
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // let EnumFromDarling {
    //     ident,
    //     generics,
    //     data: Data::Enum(data),
    // } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    // else {
    //     panic!("EnumFromDarling only works on enums");
    // };

    let EnumFromDarling {
        ident,
        generics,
        data,
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input");

    let data = match data {
        Data::Enum(data) => data,
        Data::Struct(_) => panic!("EnumFromDarling only works on enums"),
    };

    let from_impls = data.iter().map(|variant| {
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