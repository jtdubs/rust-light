use std::fmt::{Show,Formatter,Result};

use geometry::point::Point;
use geometry::transform::{Transform,Trans,TransMut};
use geometry::ray::Ray;

pub struct AABB {
    empty : bool,
    min   : Point,
    max   : Point,
}

impl AABB {
    pub fn new() -> AABB {
        AABB { empty: true, min: Point::origin(), max: Point::origin() }
    }

    pub fn for_point(p : &Point) -> AABB {
        AABB { empty: false, min: *p, max: *p }
    }

    pub fn for_points(ps : &[Point]) -> AABB {
        let mut r = AABB::new();
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
        

    pub fn add_point(&self, p : &Point) -> AABB {
        if self.empty {
            AABB::for_point(p)
        } else {
            AABB { empty: false, min: Point::new(self.min.x.min(p.x), self.min.y.min(p.y), self.min.z.min(p.z)), max: Point::new(self.max.x.max(p.x), self.max.y.max(p.y), self.max.z.max(p.z)) }
        }
    }

    pub fn add_points(&self, ps : &[Point]) -> AABB {
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

    pub fn union(&self, a : &AABB) -> AABB {
        if self.empty {
            *a
        } else if a.empty {
            *self
        } else {
            AABB { empty: false, min: Point::new(self.min.x.min(a.min.x), self.min.y.min(a.min.y), self.min.z.min(a.min.z)), max: Point::new(self.max.x.max(a.max.x), self.max.y.max(a.max.y), self.max.z.max(a.max.z)) }
        }
    }

    pub fn add_self_aabb(&mut self, a : &AABB) {
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

    pub fn overlaps(&self, a : &AABB) -> bool {
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
            let d = self.max.sub_p(&self.min);
            2f32 * (d.x * d.y + d.x * d.y + d.y * d.z)
        }
    }

    pub fn volume(&self) -> f32 {
        if self.empty {
            0f32
        } else {
            let d = self.max.sub_p(&self.min);
            d.x * d.y * d.z
        }
    }

    pub fn corners(&self) -> Option<[Point, ..8]> {
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
        let mut tmin : f32;
        let mut tmax : f32;

        let tx1 = (self.min.x - r.origin.x) / r.direction.x;
        let tx2 = (self.max.x - r.origin.x) / r.direction.x;
        if tx1 < tx2 {
            tmin = tx1;
            tmax = tx2;
        } else {
            tmin = tx2;
            tmax = tx1;
        };

        let ty1 = (self.min.y - r.origin.y) / r.direction.y;
        let ty2 = (self.max.y - r.origin.y) / r.direction.y;
        if ty1 < ty2 {
            tmin = if tmin > ty1 { tmin } else { ty1 };
            tmax = if tmax < ty2 { tmax } else { ty2 };
        } else {
            tmin = if tmin > ty2 { tmin } else { ty2 };
            tmax = if tmax < ty1 { tmax } else { ty1 };
        };
        if tmin > tmax { return false; }

        let tz1 = (self.min.z - r.origin.z) / r.direction.z;
        let tz2 = (self.max.z - r.origin.z) / r.direction.z;
        if tz1 < tz2 {
            tmin = if tmin > tz1 { tmin } else { tz1 };
            tmax = if tmax < tz2 { tmax } else { tz2 };
        } else {
            tmin = if tmin > tz2 { tmin } else { tz2 };
            tmax = if tmax < tz1 { tmax } else { tz1 };
        };

        tmax >= tmin && tmin >= 0f32
    }
}

impl Show for AABB {
    fn fmt(&self, f : &mut Formatter) -> Result {
        writeln!(f, "AABB {{ min: {}, max: {} }}", self.min, self.max)
    }
}

impl Clone for AABB {
    fn clone(&self) -> AABB {
        AABB { empty: self.empty, min: self.min, max: self.max }
    }

    fn clone_from(&mut self, source: &AABB) {
        self.empty = source.empty;
        self.min.clone_from(&source.min);
        self.max.clone_from(&source.max);
    }
}

impl PartialEq for AABB {
    fn eq(&self, other: &AABB) -> bool {
        (self.empty && other.empty) || (self.min == other.min && self.max == other.max)
    }

    fn ne(&self, other: &AABB) -> bool {
        self.empty != other.empty || self.min != other.min || self.max != other.max
    }
}

impl Trans for AABB {
    fn transform(&self, t : &Transform) -> AABB {
        match self.corners() {
            None => *self,
            Some(cs) => AABB::for_points(cs.iter().map(|c| { c.transform(t) }).collect::<Vec<Point>>().as_slice())
        }
    }
}

impl TransMut for AABB {
    fn transform_self(&mut self, t : &Transform) {
        let c = self.clone();
        self.clone_from(&c.transform(t))
    }
}

// TODO: test AABB
