use macros::AutoDefef;

#[derive(AutoDefef)]
#[deref(mutable = true)]
#[deref(field = "v")]
struct MyBox<T> {
    name: String,
    v: T,
}

impl<T> MyBox<T> {
    fn new(name: impl Into<String>, v: T) -> MyBox<T> {
        MyBox {
            name: name.into(),
            v,
        }
    }
}

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.v
//     }
// }

// use std::ops::DerefMut;

// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.v
//     }
// }

// cargo expand --example auto_deref

fn main() {
    let my_box: MyBox<f64> = MyBox::new("my_box", -100.9);
    println!(
        "box name is {}, value is {} abs() is {}",
        my_box.name,
        my_box.v,
        my_box.v.abs()
    );

    let mut s = MyBox::new("my_box", String::from("hello, "));
    s.push_str("world");
    display(&mut s)
}

fn display(s: &mut String) {
    s.push_str(" !");
    println!("{}", s)
}
