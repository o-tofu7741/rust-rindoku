fn main() {
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    r[0]; // bad: uses `v`, which is now uninitialized
    let aside = v; // move vector to aside ベクタをasideに移動
}
