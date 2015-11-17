use std::cmp;

use args::Coords;

/// A concrete, rectangular region of the screen.
///
/// The region is incluse of the top and left boundary and exclusive of the bottom and right
/// boundary.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Region {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Region {
    /// Creates a region. Note that x1/x2 and y1/y2 need not be properly ordered, but one of them 
    /// __must__ be greater than the other. This function will panic otherwise.
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Region {
        let (left, right) = (cmp::min(x1, x2), cmp::max(x1, x2));
        let (top, bottom) = (cmp::min(y1, y2), cmp::max(y1, y2));
        assert!(right > 0);
        assert!(bottom > 0);
        Region {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    /// Returns true if a given coordinates is contained within this region.
    pub fn contains(&self, coords: Coords) -> bool {
        self.left <= coords.x && coords.x < self.right
            && self.top <= coords.y && coords.y < self.bottom
    }

    /// Returns the width of this region.
    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    /// Returns the height of this region.
    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }

    /// Calculate the nearest coordinate within the region.
    pub fn xy_within(&self, Coords {x, y}: Coords) -> Coords {
        Coords {
            x: self.x_within(x),
            y: self.y_within(y),
        }
    }

    /// Calculate the nearest x value within the region.
    pub fn x_within(&self, x: u32) -> u32 {
        cmp::max(cmp::min(x, self.right - 1), self.left)
    }

    /// Calculate the naerest y value within the region.
    pub fn y_within(&self, y: u32) -> u32 {
        cmp::max(cmp::min(y, self.bottom - 1), self.top)
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    use args::Coords;

    static REGION: Region = Region { left: 0, top: 10, right: 100, bottom: 100 }; 

    static COORDS: &'static [(Coords, bool, Coords)] = &[
        (Coords { x: 0, y: 0 }, false, Coords { x: 0, y: 10 }),
        (Coords { x: 0, y: 10 }, true, Coords { x: 0, y: 10 }),
        (Coords { x: 50, y: 50 }, true, Coords { x: 50, y: 50 }),
        (Coords { x: 99, y: 99 }, true, Coords { x: 99, y: 99 }),
        (Coords { x: 100, y: 0 }, false, Coords { x: 99, y: 10 }),
        (Coords { x: 100, y: 100 }, false, Coords { x: 99, y: 99 }),
        (Coords { x: 200, y: 200 }, false, Coords { x: 99, y: 99 }),
    ];

    #[test]
    fn region_contains_coords() {
        for &(coords, b, _) in COORDS {
            assert!(REGION.contains(coords) == b, "{:?}", coords);
        }
    }

    #[test]
    fn region_coords_within() {
        for &(coords, _, within) in COORDS {
            assert_eq!(REGION.xy_within(coords), within);
        }
    }

}
