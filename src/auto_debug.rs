use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldsInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    // ty: syn::Type,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);

    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs");
    };

    let fields = fields.iter().filter(|f| !f.skip).map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! {
            .field(stringify!(#ident), &self.#ident)
        }
    });

    let mut generics_without_bound = generics.clone();
    generics_without_bound
        .type_params_mut()
        .for_each(|tp| tp.bounds.clear());

    quote! {
        impl #generics core::fmt::Debug for #ident #generics_without_bound {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result{
                f.debug_struct(stringify!(#ident))
                #(#fields)*
                .finish()
            }
        }
    }
    .into()
}
