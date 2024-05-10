use anyhow::Result;

fn main() -> Result<()> {
    let my_vec: Vec<i32> = my_vec! {};
    println!("{:?}", my_vec);
    let my_vec = my_vec! [1;3];
    println!("{:?}", my_vec);
    let my_vec = my_vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", my_vec);
    let my_vec: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
        "7".parse()?,
    ];
    println!("{:?}", my_vec);
    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // ($($elem:expr),*) => {
    //     {
    //         let mut tmp = Vec::new();
    //         $(
    //             tmp.push($elem);
    //         )*
    //         tmp
    //     }
    // };
    ($($x:expr),* $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
