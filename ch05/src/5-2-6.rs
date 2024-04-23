fn main() {
    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }
    let r = &factorial(6);
    // Arithmetic operators can see through one level of references.
    assert_eq!(r + 1009, 1729);
}
