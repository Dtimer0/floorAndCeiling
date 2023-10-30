pub fn floor64(num: f64) -> i64 {
    (num - (num % 1.0)) as i64
}
pub fn floor32(num: f32) -> i32 {
    (num - (num % 1.0)) as i32
}
pub fn ceiling64(num: f64) -> i64 {
    floor64(num) + 1
}
pub fn ceiling32(num: f32) -> i32 {
    floor32(num) + 1
}