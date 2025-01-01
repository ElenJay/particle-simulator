pub fn hypot(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

pub fn fact(x: usize) -> usize {
    let mut result = 1;
    for i in 1..=x {
        result *= i;
    }
    result
}