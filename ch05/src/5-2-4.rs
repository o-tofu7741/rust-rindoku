fn main() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    // assert!(rx == rrx);
    assert!(rx == *rrx);
    assert!(x == **rrx);
}
