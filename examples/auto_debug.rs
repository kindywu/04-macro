use std::fmt::Debug;

use macros::AutoDebug;

#[derive(AutoDebug)]
// #[derive(std::fmt::Debug)]
pub struct RespResult<T: Debug> {
    code: u16,
    #[debug(skip)]
    inner_code: u16,
    value: T,
}
// #[derive(std::fmt::Debug)]
// #[automatically_derived]
// #[allow(unused)]
// impl<T: ::core::fmt::Debug> ::core::fmt::Debug for RespResult<T> {
//     #[inline]
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         ::core::fmt::Formatter::debug_struct_field3_finish(
//             f,
//             "RespResult",
//             "code",
//             &self.code,
//             "inner_code",
//             &self.inner_code,
//             "value",
//             &&self.value,
//         )
//     }
// }

fn main() {
    let result = RespResult {
        code: 100,
        inner_code: 10000,
        value: "value".to_owned(),
    };
    println!("inner_code is {}", result.inner_code);
    println!("{:?}", result)
}
