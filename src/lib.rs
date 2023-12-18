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
                        ixn = Some((p, plane));
                    }
                } else {
                    ixn = Some((p, plane));
                }
            }
        }

        let mut ixn_s: Option<(Point3<f64>, &Sphere)> = None;
        for sphere in &scene.spheres {
            if let Some(p) = sphere.intersect(&self) {
                if let Some((q, _)) = ixn_s {
                    if na::distance(&p, &self.origin) < na::distance(&q, &self.origin) {
                        ixn_s = Some((p, sphere));
                    }
                } else {
                    ixn_s = Some((p, sphere));
                }
            }
        }


        if let Some((pt, sphere)) = ixn_s {
            let normal = (pt - sphere.origin).normalize();
            let proj = 2.0 * self.dir.dot(&normal) * normal;
            let ray = Ray {
                origin: pt,
                dir: self.dir - proj,
            };
            let scene = Scene {
                planes: scene.planes.clone(),
                spheres: vec![]
            };
            if max_bounces > 0 {
                return ray.trace(&scene, max_bounces-1);
            } else {
                return None;
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

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point3<f64>,
    pub radius: f64,
}
impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
        let a = ray.dir.norm_squared();
        let b = 2.0*(ray.origin - self.origin).dot(&ray.dir);
        let c = (ray.origin - self.origin).norm_squared() - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;

        if discriminant >= 0.0 {
            let t0 = (-b - f64::sqrt(discriminant))/(2.0*a);
            let t1 = (-b + f64::sqrt(discriminant))/(2.0*a);
            let t = f64::min(t0, t1);
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


#[derive(Debug, Clone)]
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
    pub spheres: Vec<Sphere>,
}
/* pub fn trace(&self, scene: &Scene, maxbounces: usize) ->  */
