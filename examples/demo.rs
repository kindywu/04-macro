use anyhow::Result;
#[derive(Debug)]
struct A {
    name: String,
    data: Data<Variants, ()>,
}

#[derive(Debug)]
enum Data<T, U> {
    Enum(Vec<T>),
    Struct(U),
}

#[derive(Debug)]
struct Variants {
    name: String,
}

fn main() {
    let A {
        name,
        data: Data::Enum(data),
    } = new_a().expect("")
    else {
        panic!("EnumFromDarling only works on enums");
    };
    println!("{}, {:?}", name, data);
    let arr: Vec<String> = data.iter().map(|e| e.name.clone()).collect();
    println!("{:?}", arr);

    let b = new_b();
    println!("{:#?}", b)
}

fn new_a() -> Result<A> {
    let a = A {
        name: "A".to_owned(),
        data: Data::Enum(vec![
            Variants {
                name: "a".to_string(),
            },
            Variants {
                name: "b".to_string(),
            },
            Variants {
                name: "c".to_string(),
            },
        ]),
    };
    // println!("{:#?}", a);
    Ok(a)
}

fn new_b() -> Result<A> {
    let b = A {
        name: "A".to_owned(),
        data: Data::Struct(()),
    };
    // println!("{:#?}", a);
    Ok(b)
}
