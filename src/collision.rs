use macroquad::prelude::*;

// ---------- Collision Math (SAT) ----------

pub fn circle_obb_resolve(
    c_pos: Vec2,
    r: f32,
    p_center: Vec2,
    p_size: Vec2,
    p_angle: f32,
) -> Option<Vec2> {
    let s = p_angle.sin();
    let c = p_angle.cos();

    let dx = c_pos.x - p_center.x;
    let dy = c_pos.y - p_center.y;

    let local_x = dx * c + dy * s;
    let local_y = -dx * s + dy * c;

    let half_w = p_size.x / 2.0;
    let half_h = p_size.y / 2.0;

    let closest_x = local_x.clamp(-half_w, half_w);
    let closest_y = local_y.clamp(-half_h, half_h);

    let dist_x = local_x - closest_x;
    let dist_y = local_y - closest_y;
    let dist_sq = dist_x * dist_x + dist_y * dist_y;

    if dist_sq < r * r {
        let dist = dist_sq.sqrt();
        let (nx, ny, overlap) = if dist > 0.0001 {
            (dist_x / dist, dist_y / dist, r - dist)
        } else {
            let d_x = half_w - local_x.abs();
            let d_y = half_h - local_y.abs();
            if d_x < d_y {
                let sign = if local_x > 0.0 { 1.0 } else { -1.0 };
                (sign, 0.0, d_x + r)
            } else {
                let sign = if local_y > 0.0 { 1.0 } else { -1.0 };
                (0.0, sign, d_y + r)
            }
        };

        let world_nx = nx * c - ny * s;
        let world_ny = nx * s + ny * c;

        Some(vec2(world_nx * overlap, world_ny * overlap))
    } else {
        None
    }
}
