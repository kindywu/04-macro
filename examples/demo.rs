use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let i = "100".parse::<i32>()?;
    println!("{i}");
    Ok(())
}

// fn main() {
//     match "100".parse::<i32>() {
//         Ok(i) => {
//             println!("{}", i);
//         }
//         Err(e) => {
//             eprintln!("解析错误: {}", e);
//         }
//     }
// }
