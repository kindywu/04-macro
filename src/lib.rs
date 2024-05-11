use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);
    // get the ident
    let ident = input.ident;

    // get generics
    let generics = input.generics;

    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };

    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        // ty=DirectionUp #ident=Direction #var=Up
                        // ty=i32 #ident=Direction #var=Down
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }

            _ => quote! {},
        }
    });

    // quote return proc_macro2 TokenTtream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
    .into()

    // quote! {}.into()
}
