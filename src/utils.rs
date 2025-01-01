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

pub fn point_to_segment_distance_v(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let ab_x: f32 = x2 - x1;
    let ab_y: f32 = y2 - y1;

    let ap_x: f32 = px - x1;
    let ap_y: f32 = py - y1;

    let bp_x: f32 = px - x2;
    let bp_y: f32 = py - y2;

    let ab_ap: f32 = ab_x * ap_x + ab_y * ap_y;
    let ab_ab: f32 = ab_x * ab_x + ab_y * ab_y;
    let t: f32 = ab_ap / ab_ab;

    // Project point P ont the line segment AB
    if t < 0.0 {
        // P is closer to A
        return hypot(ap_x, ap_y);
    }
    else if t > 1.0 {
        // P is closer to B
        return hypot(bp_x, bp_y);
    }
    else {
        // projection point is on the segment
        let proj_x: f32 = x1 + t * ab_x;
        let proj_y: f32 = y1 + t * ab_y;
        return hypot(px - proj_x, py - proj_y);
    }
}