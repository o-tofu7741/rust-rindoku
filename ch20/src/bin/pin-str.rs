fn main() {
    use async_std::pin::Pin;
    let mut string = "Pinned?".to_string();
    let mut pinned: Pin<&mut String> = Pin::new(&mut string);
    pinned.push_str(" Not");
    // string.push_str("hoge");
    Pin::into_inner(pinned);//.push_str(" so much.");
    // let new_home = string;
    // assert_eq!(new_home, "Pinned? Not so much.");
    // println!("{:?}", Pin::into_inner(pinned));
}
