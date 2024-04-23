fn main() {
    // ok
    {
        let x = 1;
        {
            let r = &x;
            assert_eq!(*r, 1);
        }
    }
    // ng
    let r;
    {
        let x = 1;

        r = &x;
    }
    assert_eq!(*r, 1);
}
