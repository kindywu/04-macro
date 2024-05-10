use std::fmt::{Debug, Display};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // let ret = my_try!(f2(my_try!(f1("hello"))));
    let ret = my_try!(f3(my_try!(f2(my_try!(f1("hello"))))));

    println!("final => {ret}");
    Ok(())
}

fn f1(str: impl Into<String> + Display + Debug) -> Result<impl Into<String> + Display + Debug> {
    Ok(format!("f1 => {}", str))
}

fn f2(str: impl Into<String> + Display + Debug) -> Result<impl Into<String> + Display + Debug> {
    Ok(format!("f2 => {}", str))
}

// fn f3(str: impl Into<String> + Display + Debug) -> Result<String> {
//     Err(anyhow!("f3 => {}", str))
// }

fn f3(str: impl Into<String> + Display + Debug) -> Result<impl Into<String> + Display + Debug> {
    Result::<String, _>::Err(anyhow!("f3 => {}", str))
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}
