/// Takes a f64 and returns the i64 FLOOR() of the f64.
pub fn floor64(num: f64) -> i64 {
    if num < 0.0 { (num - (num % 1.0) - 1.0) as i64 } else if num == 0.0 { 0 } else {(num - (num % 1.0)) as i64}
}
/// Takes a f32 and returns the i32 FLOOR() of the f32.
pub fn floor32(num: f32) -> i32 {
    if num < 0.0 { (num - (num % 1.0) - 1.0) as i32 } else if num == 0.0 { 0 } else {(num - (num % 1.0)) as i32 }
}
/// Takes a f64 and returns the i64 CEIL() of the f64.
pub fn ceiling64(num: f64) -> i64 {
    if num == 0.0 { 0 } else { floor64(num) + 1 }
}
/// Takes a f32 and returns the i32 CEIL() of the f32.
pub fn ceiling32(num: f32) -> i32 {
    if num == 0.0 { 0 } else { floor32(num) + 1 }
}