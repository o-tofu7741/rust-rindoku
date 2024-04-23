fn main() {
    static mut STASH: &i32 = &128;
    fn f(p: &'static i32) {
        // still not good enough これでもまだダメ
        unsafe {
            STASH = p;
        }
    }
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}
