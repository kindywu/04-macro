use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(dead_code)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down(i32),
    Left,
    Right(i32, i32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    // let up = Direction::Up(DirectionUp::new(42));
    let up: Direction<i32> = DirectionUp::new(42).into();
    println!("{:?}", up);
    let down: Direction<i32> = 42.into();
    println!("{:?}", down);

    // let right: Direction<i32> = (42, 34).into();
    // println!("{:?}", right);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(value: DirectionUp<T>) -> Self {
//         Direction::Up(value)
//     }
// }

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::Up(value)
//     }
// }

// DeriveInput {
//     attrs: [ ],
//     vis: Visibility::Inherited,
//     ident: Ident {
//         ident: "Direction",
//         span: #0 bytes(75..84),
//     },
//     generics: Generics {
//         lt_token: None,
//         params: [],
//         gt_token: None,
//         where_clause: None,
//     },
//     data: Data::Enum {
//         enum_token: Enum,
//         brace_token: Brace,
//         variants: [
//             Variant {
//                 attrs: [],
//                 ident: Ident {
//                     ident: "Up",
//                     span: #0 bytes(91..93),
//                 },
//                 fields: Fields::Unnamed {
//                     paren_token: Paren,
//                     unnamed: [
//                         Field {
//                             attrs: [],
//                             vis: Visibility::Inherited,
//                             mutability: FieldMutability::None,
//                             ident: None,
//                             colon_token: None,
//                             ty: Type::Path {
//                                 qself: None,
//                                 path: Path {
//                                     leading_colon: None,
//                                     segments: [
//                                         PathSegment {
//                                             ident: Ident {
//                                                 ident: "DirectionUp",
//                                                 span: #0 bytes(94..105),
//                                             },
//                                             arguments: PathArguments::None,
//                                         },
//                                     ],
//                                 },
//                             },
//                         },
//                     ],
//                 },
//                 discriminant: None,
//             },
//             Comma,
//             Variant {
//                 attrs: [],
//                 ident: Ident {
//                     ident: "Down",
//                     span: #0 bytes(112..116),
//                 },
//                 fields: Fields::Unnamed {
//                     paren_token: Paren,
//                     unnamed: [
//                         Field {
//                             attrs: [],
//                             vis: Visibility::Inherited,
//                             mutability: FieldMutability::None,
//                             ident: None,
//                             colon_token: None,
//                             ty: Type::Path {
//                                 qself: None,
//                                 path: Path {
//                                     leading_colon: None,
//                                     segments: [
//                                         PathSegment {
//                                             ident: Ident {
//                                                 ident: "i32",
//                                                 span: #0 bytes(117..120),
//                                             },
//                                             arguments: PathArguments::None,
//                                         },
//                                     ],
//                                 },
//                             },
//                         },
//                     ],
//                 },
//                 discriminant: None,
//             },
//             Comma,
//         ],
//     },
// }
