fn main() {
    // use std::collections::HashMap;
    // let mut major_cities = HashMap::new();
    // major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    // major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    // major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    // major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    // major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    // let countries = ["Japan", "Brazil", "Kenya"];
    // for city in countries.iter().flat_map(|country| &major_cities[country]) {
    //     println!("{}", &city);
    // }
    // let a = &"Japan";
    // let b = "hoge";
    // major_cities.get(&a);
    // assert!(a);
    // println!("{}", a);

    // for i in countries.iter(){
    // }

    // let message = "To: jimb\r\n\
    // From: id\r\n\
    // \r\n\
    // Oooooh, donuts!!\r\n";
    // let mut lines = message.lines();
    // println!("Headers:");
    // for header in lines.by_ref().take_while(|l| !l.is_empty()) {
    //     println!("{}", header);
    // }
    // println!("\nBody:");
    // for body in lines {
    //     println!("{}", body);
    // }
    // let big_city_with_volcano_park = populations.iter().find_map(|(&city, _)| {
    //     if let Some(park) = find_volcano_park(city, &parks) {
    //         // find_map returns this value, so our caller knows find_mapはこの値を返すので、
    //         // *which* park we found. 見つけたのが「どの」公園なのかが呼び出し元にわかる
    //         return Some((city, park.name));
    //     }
    //     // Reject this item, and continue the search. このアイテムを捨てて検索を継続
    //     None
    // });
    // assert_eq!(
    //     big_city_with_volcano_park,
    //     Some(("Portland", "Mt. Tabor Park"))
    // );
    // use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};
    // let args: HashSet<String> = std::env::args().collect(); //collectでMapを作るには、(キー, 値)
    // let args: BTreeSet<String> = std::env::args().collect(); //のペアが必要になるので、この例では、
    // let args: LinkedList<String> = std::env::args().collect(); //文字列と整数の列をzipしたものを与えている
    //                                                            // Collecting a map requires (key, value) pairs, so for this example,
    //                                                            // zip the sequence of strings with a sequence of integers
    // let a = vec!["hoge", "fuga", "baka"];
    // let args: HashMap<String, usize> = a.iter().zip(0..).collect();
    // // let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
}
