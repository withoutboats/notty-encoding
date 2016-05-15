use std::cmp;

use super::{Argument, Coords, SplitKind, ResizeRule};
use super::SplitKind::*;
use super::ResizeRule::*;

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
        assert!(right > left);
        assert!(bottom > top);
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

    pub fn offset(&self, Coords {x, y}: Coords) -> Coords {
        Coords { x: x.saturating_sub(self.left), y: y.saturating_sub(self.top) }
    }

    pub fn move_to_contain(&self, coords: Coords) -> Region {
        match (coords.x < self.left, self.right <= coords.x,
               coords.y < self.top, self.bottom <= coords.y) {
            // Left and above
            (true, false, true, false)  =>
                Region::new(coords.x, coords.y, coords.x + self.width(), coords.x + self.height()),
            // Left and below
            (true, false, false, true)  =>
                Region::new(coords.x,
                            (coords.y + 1).saturating_sub(self.height()),
                            coords.x + self.width(),
                            (coords.y + 1).saturating_sub(self.height()) + self.height()),
            // Right and above
            (false, true, true, false)  =>
                Region::new((coords.x + 1).saturating_sub(self.width()),
                            coords.y,
                            (coords.x + 1).saturating_sub(self.width()) + self.width(),
                            coords.y + self.height()),
            // Right and below
            (false, true, false, true)  =>
                Region::new((coords.x + 1).saturating_sub(self.width()),
                            (coords.y + 1).saturating_sub(self.height()),
                            (coords.x + 1).saturating_sub(self.width()) + self.width(),
                            (coords.y + 1).saturating_sub(self.height()) + self.height()),
            // Left only
            (true, false, false, false) =>
                Region::new(coords.x, self.top, coords.x + self.width(), self.bottom),
            // Right only
            (false, true, false, false) =>
                Region::new((coords.x + 1).saturating_sub(self.width()),
                            self.top,
                            (coords.x + 1).saturating_sub(self.width()) + self.width(),
                            self.bottom),
            // Above only
            (false, false, true, false) =>
                Region::new(self.left, coords.y, self.right, coords.y + self.height()),
            // Below only
            (false, false, false, true) =>
                Region::new(self.left,
                            (coords.y + 1).saturating_sub(self.height()),
                            self.right,
                            (coords.y + 1).saturating_sub(self.height()) + self.height()),
            _                           => *self
        }
    }

    pub fn split(self, kind: SplitKind, rule: ResizeRule) -> (SplitKind, Region, Region) {
        match (kind, rule) {
            (Horizontal(n), MaxLeftTop) | (Horizontal(n), Percentage)   => {
                let n = cmp::min(self.top + n, self.bottom - 1);
                (Horizontal(n), Region { bottom: n, ..self }, Region { top: n, ..self })
            }
            (Horizontal(n), MaxRightBottom)                             => {
                let n = cmp::min(n, cmp::max(self.bottom.saturating_sub(n), self.top + 1));
                (Horizontal(n), Region { bottom: n, ..self }, Region { top: n, ..self })
            }
            (Vertical(n), MaxLeftTop) | (Vertical(n), Percentage)       => {
                let n = cmp::min(self.left + n, self.right - 1);
                (Vertical(n), Region { right: n, ..self }, Region { left: n, ..self })
            }
            (Vertical(n), MaxRightBottom)                               => {
                let n = cmp::min(n, cmp::max(self.right.saturating_sub(n), self.left + 1));
                (Vertical(n), Region { right: n, ..self }, Region { left: n, ..self })
            }
        }
    }

}

impl Argument for Region {
    fn from_nums<T>(mut args: T, default: Option<Region>) -> Option<Region>
    where T: Iterator<Item=u64> {
        match (args.next(), args.next(), args.next(), args.next()) {
            (Some(l), Some(t), Some(r), Some(b)) => Some(Region::new(l as u32, t as u32,
                                                                     r as u32, b as u32)),
            _                                    => default
        }
    }

    fn encode(&self) -> String {
        format!("{:x}.{:x}.{:x}.{:x}", self.left, self.top, self.right, self.bottom)
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
