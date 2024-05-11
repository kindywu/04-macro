mod enum_from;

use enum_from::process_enum_from;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    process_enum_from(input)
}
