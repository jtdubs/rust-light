use std::default::Default;
use std::fmt::{Display, Formatter, Result};

use crate::geometry::{Point, Ray, HasTransform};

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    empty : bool,
    min   : Point,
    max   : Point,
}

impl BoundingBox {
    pub fn empty() -> BoundingBox {
        BoundingBox { 
            empty: true,
            min: Point::origin(),
            max: Point::origin(),
        }
    }

    pub fn for_point(p : &Point) -> BoundingBox {
        BoundingBox { empty: false, min: *p, max: *p }
    }

    pub fn for_points(ps : &[Point]) -> BoundingBox {
        let mut r = BoundingBox::empty();
        for p in ps.iter() {
            r.add_self_point(p);
        }
        r
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn range_x(&self) -> Option<(f32, f32)> {
        if self.empty {
            None
        } else {
            Some((self.min.x, self.max.x))
        }
    }

    pub fn range_y(&self) -> Option<(f32, f32)> {
        if self.empty {
            None
        } else {
            Some((self.min.y, self.max.y))
        }
    }

    pub fn range_z(&self) -> Option<(f32, f32)> {
        if self.empty {
            None
        } else {
            Some((self.min.z, self.max.z))
        }
    }
        

    pub fn add_point(&self, p : &Point) -> BoundingBox {
        if self.empty {
            BoundingBox::for_point(p)
        } else {
            BoundingBox { empty: false, min: Point::new(self.min.x.min(p.x), self.min.y.min(p.y), self.min.z.min(p.z)), max: Point::new(self.max.x.max(p.x), self.max.y.max(p.y), self.max.z.max(p.z)) }
        }
    }

    pub fn add_points(&self, ps : &[Point]) -> BoundingBox {
        let mut r = self.clone();
        for p in ps.iter() {
            r.add_self_point(p);
        }
        r
    }

    pub fn add_self_point(&mut self, p : &Point) {
        if self.empty {
            self.empty = false;
            self.min = *p;
            self.max = *p;
        } else {
            self.min.x = self.min.x.min(p.x);
            self.min.y = self.min.y.min(p.y);
            self.min.z = self.min.z.min(p.z);
            self.max.x = self.max.x.max(p.x);
            self.max.y = self.max.y.max(p.y);
            self.max.z = self.max.z.max(p.z);
        }
    }

    pub fn add_self_points(&mut self, ps : &[Point]) {
        for p in ps.iter() {
            self.add_self_point(p)
        }
    }

    pub fn union(&self, a : &BoundingBox) -> BoundingBox {
        if self.empty {
            *a
        } else if a.empty {
            *self
        } else {
            BoundingBox { empty: false, min: Point::new(self.min.x.min(a.min.x), self.min.y.min(a.min.y), self.min.z.min(a.min.z)), max: Point::new(self.max.x.max(a.max.x), self.max.y.max(a.max.y), self.max.z.max(a.max.z)) }
        }
    }

    pub fn add_self_bounding_box(&mut self, a : &BoundingBox) {
        if self.empty {
            self.clone_from(a)
        } else { 
            self.min.x = self.min.x.min(a.min.x);
            self.min.y = self.min.y.min(a.min.y);
            self.min.z = self.min.z.min(a.min.z);
            self.max.x = self.max.x.max(a.max.x);
            self.max.y = self.max.y.max(a.max.y);
            self.max.z = self.max.z.max(a.max.z);
        }
    }

    pub fn overlaps(&self, a : &BoundingBox) -> bool {
        if self.empty || a.empty {
            false
        } else {
            self.contains(&a.min) || self.contains(&a.max)
        }
    }

    pub fn contains(&self, p : &Point) -> bool {
        if self.empty {
            false
        } else {
            p.x >= self.min.x && p.y >= self.min.y && p.z >= self.min.z && p.x <= self.max.x && p.y <= self.max.y && p.z <= self.max.z
        }
    }

    pub fn surface_area(&self) -> f32 {
        if self.empty {
            0f32
        } else {
            let d = self.max - self.min;
            2f32 * (d.x * d.y + d.x * d.y + d.y * d.z)
        }
    }

    pub fn volume(&self) -> f32 {
        if self.empty {
            0f32
        } else {
            let d = self.max - self.min;
            d.x * d.y * d.z
        }
    }

    pub fn corners(&self) -> Option<[Point; 8]> {
        if self.empty {
            None
        } else {
            let n = self.min;
            let x = self.max;
            Some([Point::new(n.x, n.y, n.z), Point::new(n.x, n.y, x.z), Point::new(n.x, x.y, n.z), Point::new(n.x, x.y, x.z),
                  Point::new(x.x, n.y, n.z), Point::new(x.x, n.y, x.z), Point::new(x.x, x.y, n.z), Point::new(x.x, x.y, x.z)])
        }
    }

    pub fn intersects(&self, r : &Ray) -> bool {
        if self.empty { return false; }

        let mut tmin : f32 = (self.min.x - r.origin.x) / r.direction.x;
        let mut tmax : f32 = (self.max.x - r.origin.x) / r.direction.x;
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let ty1 = (self.min.y - r.origin.y) / r.direction.y;
        let ty2 = (self.max.y - r.origin.y) / r.direction.y;
        if ty1 < ty2 {
            tmin = tmin.max(ty1);
            tmax = tmax.min(ty2);
        } else {
            tmin = tmin.max(ty2);
            tmax = tmax.min(ty1);
        };
        if tmin > tmax { return false; }

        let tz1 = (self.min.z - r.origin.z) / r.direction.z;
        let tz2 = (self.max.z - r.origin.z) / r.direction.z;
        if tz1 < tz2 {
            tmin = tmin.max(tz1);
            tmax = tmax.min(tz2);
        } else {
            tmin = tmin.max(tz2);
            tmax = tmax.min(tz1);
        };

        tmax >= tmin && tmin >= 0f32
    }

    pub fn to<T : HasTransform>(&self, t : &T) -> BoundingBox {
        match self.corners() {
            None => *self,
            Some(cs) => BoundingBox::for_points(cs.iter().map(|c| { c.to(t) }).collect::<Vec<Point>>().as_slice())
        }
    }

    pub fn from<T : HasTransform>(&self, t : &T) -> BoundingBox {
        match self.corners() {
            None => *self,
            Some(cs) => BoundingBox::for_points(cs.iter().map(|c| { c.from(t) }).collect::<Vec<Point>>().as_slice())
        }
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "BoundingBox {{ min: {}, max: {} }}", self.min, self.max)
    }
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &BoundingBox) -> bool {
        (self.empty && other.empty) || (self.min == other.min && self.max == other.max)
    }

    fn ne(&self, other: &BoundingBox) -> bool {
        self.empty != other.empty || self.min != other.min || self.max != other.max
    }
}

impl Default for BoundingBox {
    fn default() -> BoundingBox {
        BoundingBox::empty()
    }
}
