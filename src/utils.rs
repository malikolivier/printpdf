//! Utility / conveniece functions for commonly use graphical shapes

use scale::Pt;
use Point;

// PDF doesn't understand what a "circle" is, so we have to 
// approximate it.
const C: f64 = 0.551915024494;

/// Calculates and returns the points for an approximated circle, given a radius and an 
/// offset into the page from the lower left corner.
#[inline]
pub fn calculate_points_for_circle(radius: Pt, offset_x: Pt, offset_y: Pt) -> Vec<(Point, bool)> {

    let radius = radius.0;

    let p10 = Point { x: Pt(0.0 * radius), y: Pt(1.0 * radius) };
    let p11 = Point { x: Pt(C   * radius), y: Pt(1.0 * radius) };
    let p12 = Point { x: Pt(1.0 * radius), y: Pt(C   * radius) };
    let p13 = Point { x: Pt(1.0 * radius), y: Pt(0.0 * radius) };
    
    let p20 = Point { x: Pt(1.0 * radius), y: Pt(0.0 * radius) };
    let p21 = Point { x: Pt(1.0 * radius), y: Pt(-C  * radius) };
    let p22 = Point { x: Pt(C   * radius), y: Pt(-1.0 * radius) };
    let p23 = Point { x: Pt(0.0 * radius), y: Pt(-1.0 * radius) };
    
    let p30 = Point { x: Pt(0.0 * radius),  y: Pt(-1.0 * radius) };
    let p31 = Point { x: Pt(-C  * radius),  y: Pt(-1.0 * radius) };
    let p32 = Point { x: Pt(-1.0 * radius), y: Pt(-C   * radius) };
    let p33 = Point { x: Pt(-1.0 * radius), y: Pt(0.0 * radius) };
    
    let p40 = Point { x: Pt(-1.0 * radius), y: Pt(0.0 * radius) };
    let p41 = Point { x: Pt(-1.0 * radius), y: Pt(C * radius) };
    let p42 = Point { x: Pt(-C * radius),   y: Pt(1.0  * radius) };
    let p43 = Point { x: Pt(0.0 * radius),  y: Pt(1.0 * radius) };

    let mut pts  = vec![(p10, true), (p11, true), (p12, true), (p13, false),
                        (p20, true), (p21, true), (p22, true), (p23, false),
                        (p30, true), (p31, true), (p32, true), (p33, false),
                        (p40, true), (p41, true), (p42, true), (p43, false),
    ];

    for &mut (ref mut p, _) in pts.iter_mut() {
        p.x.0 += offset_x.0;
        p.y.0 += offset_y.0;
    }

    pts
}

/// Calculates and returns the points for a rectangle, given a horizontal and vertical scale,
/// and an offset into the page from the lower left corner.
#[inline]
pub fn calculate_points_for_rect(scale_x: Pt, scale_y: Pt, offset_x: Pt, offset_y: Pt) -> Vec<(Point, bool)> {
    let top = Pt(offset_y.0 + (scale_y.0 / 2.0));
    let bottom = Pt(offset_y.0 - (scale_y.0 / 2.0));
    let left = Pt(offset_x.0 - (scale_x.0 / 2.0));
    let right = Pt(offset_x.0 + (scale_x.0 / 2.0));

    let top_left_pt = Point { x: left, y: top };
    let top_right_pt = Point { x: right, y: top };
    let bottom_right_pt = Point { x: right, y: bottom };
    let bottom_left_pt = Point { x: left, y: bottom };

    vec![(top_left_pt, false), (top_right_pt, false), (bottom_right_pt, false), (bottom_left_pt, false)]
}