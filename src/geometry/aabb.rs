use std::fmt::{Show,Formatter,Result};

use geometry::point::Point;
use geometry::transform::{Transform,Trans,TransMut};

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
        self.min.x = self.min.x.min(a.min.x);
        self.min.y = self.min.y.min(a.min.y);
        self.min.z = self.min.z.min(a.min.z);
        self.max.x = self.max.x.max(a.max.x);
        self.max.y = self.max.y.max(a.max.y);
        self.max.z = self.max.z.max(a.max.z);
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

    pub fn surface_area(&self) -> f64 {
        if self.empty {
            0f64
        } else {
            let d = self.max.sub_p(&self.min);
            2f64 * (d.x * d.y + d.x * d.y + d.y * d.z)
        }
    }

    pub fn volume(&self) -> f64 {
        if self.empty {
            0f64
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
