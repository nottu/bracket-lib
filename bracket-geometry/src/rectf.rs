use crate::prelude::PointF;
use std::convert::TryInto;
use std::ops;

/// Defines a rectangle with floating-point coordinates.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RectF {
    /// The X position of the first point (typically the left)
    pub x1: f32,
    /// The X position of the second point (typically the right)
    pub x2: f32,
    /// The Y position of the first point (typically the top)
    pub y1: f32,
    /// The Y position of the second point (typically the bottom)
    pub y2: f32,
}

#[cfg(feature = "specs")]
impl specs::prelude::Component for RectF {
    type Storage = specs::prelude::VecStorage<Self>;
}

impl Default for RectF {
    fn default() -> RectF {
        RectF::zero()
    }
}

impl RectF {
    /// Create a new rectangle, specifying X/Y Width/Height
    #[allow(clippy::missing_panics_doc)]
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> RectF
    where
        T: TryInto<f32>,
    {
        let x_f32: f32 = x.try_into().ok().unwrap();
        let y_f32: f32 = y.try_into().ok().unwrap();
        RectF {
            x1: x_f32,
            y1: y_f32,
            x2: x_f32 + w.try_into().ok().unwrap(),
            y2: y_f32 + h.try_into().ok().unwrap(),
        }
    }

    /// Create a new rectangle, specifying exact dimensions
    #[allow(clippy::missing_panics_doc)]
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> RectF
    where
        T: TryInto<f32>,
    {
        RectF {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }

    /// Creates a zero rectangle
    #[must_use]
    pub fn zero() -> RectF {
        RectF {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        }
    }

    /// Returns true if this overlaps with other
    #[must_use]
    pub fn intersect(&self, other: &RectF) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Returns the center of the rectangle
    #[must_use]
    pub fn center(&self) -> PointF {
        PointF {
            x: (self.x1 + self.x2) / 2.0,
            y: (self.y1 + self.y2) / 2.0,
        }
    }

    /// Returns true if a point is inside the rectangle
    #[must_use]
    pub fn point_in_rect(&self, point: PointF) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    /// Returns the rectangle's width
    #[must_use]
    pub fn width(&self) -> f32 {
        f32::abs(self.x2 - self.x1)
    }

    /// Returns the rectangle's height
    #[must_use]
    pub fn height(&self) -> f32 {
        f32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<RectF> for RectF {
    type Output = RectF;
    fn add(mut self, rhs: RectF) -> RectF {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}
