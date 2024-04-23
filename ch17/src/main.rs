fn main() {
    // println!("{}", latin1_to_char(255));
    // println!("{}", '七'.is_alphabetic());
    // println!("{}", char_to_latin1('a').unwrap());
    // println!("{}", '1'.to_digit(2).unwrap());
    // let mut letter = String::new();
    // writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap();
    // println!("{}", letter);
    // writeln!(letter, "His house is in the village though;").unwrap();
    // println!("{}", letter);
    // assert_eq!(
    //     letter,
    //     "Whose rutabagas these are I think I know\n\
    // His house is in the village though;\n"
    // );
    // println!("{:?}", ch17_3_3().unwrap());
    // let fuga = "hoge".to_string();
    // println!("{}", fuga[0]);
    // let mut beverage = "a piña colada".to_string();
    // beverage.truncate(4);
    // println!("{}", beverage);
    assert_eq!(
        format!(
            "{mode} {3} {} {} {}",
            "people",
            "eater",
            "purple",
            mode = "flying"
        ),
        "flying flying people eater purple"
    );
}

// fn latin1_to_char(latin1: u8) -> char {
//     latin1 as char
// }

// fn char_to_latin1(c: char) -> Option<u8> {
//     if c as u32 <= 0xff {
//         Some(c as u8)
//     } else {
//         None
//     }
// }

// use std::fmt::Write;
// fn ch17_3_3() -> Result<(), std::fmt::Error> {
//     let mut letter = String::new();
//     writeln!(letter, "Whose {} these are I think I know", "hogehoge")?;
//     println!("{}", letter);
//     Ok(())
// }
