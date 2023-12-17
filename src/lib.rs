extern crate nalgebra as na;

pub mod imbuf;

use na::{Point3, Vector3, point};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>,
}

pub enum Geometry {
    Plane,
    Sphere,
    Triangle,
}

impl Ray {
    pub fn trace(&self, scene: &Scene, max_bounces: usize) -> Option<Point3<u8>> {
        let mut ixn: Option<(Point3<f64>, &Plane)> = None;
        for plane in &scene.planes {
            if let Some(p) = plane.intersect(&self) {
                if let Some((q, _)) = ixn {
                    if na::distance(&p, &self.origin) < na::distance(&q, &self.origin) {
                        ixn = Some((p, &plane));
                    }
                } else {
                    ixn = Some((p, &plane));
                }
            }
        }

        if let Some((pt, plane)) = ixn {
            let (u, v) = plane.proj_coords(&pt);
            let (u, v) = (f64::floor(u) as i64, f64::floor(v) as i64);
            let w = u.rem_euclid(2) == v.rem_euclid(2);

            let dist = 32.0 + (255.0-32.0)/(0.1*na::distance_squared(&pt, &self.origin));
            let dist = f64::from(dist).clamp(0.0, 255.0) as u8;

            if w {
                Some(point![dist, 0, 0])
            } else {
                Some(point![0, dist, 0])
            }
        } else {
            None
        }
    }
}

pub struct Sphere {
    pub p: Point3<f64>,
    pub r: Point3<f64>,
}
impl Sphere {
}

pub trait RayIntersect {
}

#[derive(Debug)]
pub struct Plane {
    pub p0: Point3<f64>,
    pub p1: Point3<f64>,
    pub p2: Point3<f64>,
    /// p1 - p0
    u: Vector3<f64>,
    /// p2 - p0
    v: Vector3<f64>,
    /// u X v
    n: Vector3<f64>,
}
impl Plane {
    pub fn new(p0: Point3<f64>, p1: Point3<f64>, p2: Point3<f64>) -> Self {
        let u = p1 - p0;
        let v = p2 - p0;
        let n = u.cross(&v);
        Self {
            p0,
            p1,
            p2,
            u,
            v,
            n,
        }
    }
    pub fn proj_coords(&self, pt: &Point3<f64>) -> (f64, f64) {
        let dp = pt - self.p0;
        let u = dp.dot(&self.u)/self.u.dot(&self.u);
        let v = dp.dot(&self.v)/self.v.dot(&self.v);
        (u, v)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
        let denom = ray.dir.dot(&self.n);
        if denom != 0.0 {
            let t = -(ray.origin - self.p0).dot(&self.n)/denom;
            if t >= 0.0 {
                Some(ray.origin + t*ray.dir)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub planes: Vec<Plane>,
}
/* pub fn trace(&self, scene: &Scene, maxbounces: usize) ->  */
