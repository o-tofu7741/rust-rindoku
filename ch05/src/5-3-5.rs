fn main() {
    // This does not compile. このコードはコンパイルできない
    struct S<'a> {
        r: &'a i32,
    }
    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}
