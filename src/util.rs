use nalgebra::{Point2, Vector2};

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

// This algorithm will only detect on entry
pub fn rectangle_line_intersect(p: Point2<f32>, v: Vector2<f32>, r: Rect) -> bool {
    // There's 4 lines our segment can intersect with:
    // y = r.y
    // y = r.y + r.h
    // x = r.x
    // x = r.x + r.w
    //
    // We're looking for the line with the intersection closest to p, in the direction of v, only
    // inclu including ones between p and p+v
    let bottom_intersect = intersect(p, v, r.y, false);
    let top_intersect = intersect(p, v, r.y + r.h, false);
    let left_intersect = intersect(p, v, r.x, true);
    let right_intersect = intersect(p, v, r.x + r.w, true);

    // For each, check if they're withing the acceptable part of the intersection
    if let Some(bottom_intesect) = bottom_intersect {
        if bottom_intesect.0 >= 0.
            && bottom_intesect.0 <= 1.
            && bottom_intesect.1.x >= r.x
            && bottom_intesect.1.x <= r.x + r.w
        {
            return true;
        }
    }
    if let Some(top_intersect) = top_intersect {
        if top_intersect.0 >= 0.
            && top_intersect.0 <= 1.
            && top_intersect.1.x >= r.x
            && top_intersect.1.x <= r.x + r.w
        {
            return true;
        }
    }
    if let Some(left_intersect) = left_intersect {
        if left_intersect.0 >= 0.
            && left_intersect.0 <= 1.
            && left_intersect.1.y >= r.y
            && left_intersect.1.y <= r.y + r.h
        {
            return true;
        }
    }
    if let Some(right_intersect) = right_intersect {
        if right_intersect.0 >= 0.
            && right_intersect.0 <= 1.
            && right_intersect.1.y >= r.y
            && right_intersect.1.y <= r.y + r.h
        {
            return true;
        }
    }

    false
}

// Return value is t, where f(t) = p +vt, as well as the point of intersection
// If None is returned, there is no intersection point (i.e. the lines are parallel).
pub fn intersect(
    p: Point2<f32>,
    v: Vector2<f32>,
    // These two encode whether this is a y = c or x = c equation (if vert = true, then x is
    // constant)
    c: f32,
    vert: bool,
) -> Option<(f32, Point2<f32>)> {
    // Then x is constant
    if vert {
        if v.x == 0.0 {
            None
        } else {
            let t = (c - p.x) / v.x;
            Some((t, p + v * t))
        }
    } else {
        if v.y == 0.0 {
            None
        } else {
            let t = (c - p.y) / v.y;
            Some((t, p + v * t))
        }
    }
}
