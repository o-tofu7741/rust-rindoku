#[repr(C)]
enum Tag {
    Float = 0,
    Int = 1,
}
#[repr(C)]
union FloatOrInt {
    f: f32,
    i: i32,
}
#[repr(C)]
struct Value {
    tag: Tag,
    union: FloatOrInt,
}
fn is_zero(v: Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value {
                tag: Int,
                union: FloatOrInt { i: 0 },
            } => true,
            Value {
                tag: Float,
                union: FloatOrInt { f: 0.0 },
            } => true,
            _ => false,
        }
    }
}
fn main() {
    
}
