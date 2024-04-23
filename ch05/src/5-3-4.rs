fn main() {
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if r < s {
                s = r;
            }
        }
        s
    }
}
