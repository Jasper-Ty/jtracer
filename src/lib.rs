extern crate nalgebra as na;

pub mod imbuf;
mod scene;
pub use scene::Scene;

pub fn sigmoid(x: f64) -> f64 {
    1.0/(1.0 + f64::exp(-x))
}

pub struct Viewport {
    pub target: Point3<f64>,
    pub eye: Point3<f64>,
    pub up: Vector3<f64>,
}

use na::{Point3, Vector3, point};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>,
}

/// returns a new iterator that skips the ith element
pub fn except<U, T>(iter: U, i: usize) -> impl Iterator<Item = T> 
where
    U: Iterator<Item = T>
{
    iter
        .enumerate()
        .filter_map(move |(j, e)| (j != i).then_some(e))
}

#[derive(Debug)]
pub enum Shape {
    Plane {
        p: Point3<f64>,
        n: Vector3<f64>,
    },
    Sphere {
        p: Point3<f64>,
        r: f64,
    },
    Triangle {
        p0: Point3<f64>,
        p1: Point3<f64>,
        p2: Point3<f64>,
    },
}

//TODO: intersect -> cast -> trace functions
impl Shape {
    /// Attempts to intersect a ray with the shape, returning the point of
    /// intersection if one exists
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Self::Plane { p, n, .. } => {
                let denom = ray.dir.dot(n);
                if denom != 0.0 {
                    let t = -(ray.origin - p).dot(&n)/denom;
                    if t > 0.0 {
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Self::Sphere { p, r, .. } => { 
                let a = ray.dir.norm_squared();
                let b = 2.0*(ray.origin - p).dot(&ray.dir);
                let c = (ray.origin - p).norm_squared() - r*r;

                let discriminant = b*b - 4.0*a*c;

                if discriminant >= 0.0 {
                    let t0 = (-b - f64::sqrt(discriminant))/(2.0*a);
                    let t1 = (-b + f64::sqrt(discriminant))/(2.0*a);
                    let t = f64::min(t0, t1);
                    if t > 0.0 {
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Self::Triangle { .. } => { todo!() },
        }
    }
    /// returns the normal at a point
    pub fn normal(&self, pt: &Point3<f64>) -> Vector3<f64> {
        match self {
            Self::Plane { n, .. } => n.clone(),
            Self::Sphere { p, .. } => (pt - p).try_normalize(10e-6).unwrap(),
            Self::Triangle { .. } => { todo!() },
        }
    }
}

#[derive(Debug)]
pub enum Material {
    Solid {
        color: Point3<u8>,
    },
    Checkered {
        color0: Point3<u8>,
        color1: Point3<u8>,
        up: Vector3<f64>,
        scale: f64,
    },
    Reflective,
}
impl Material {
    pub fn compute() {
    }
}
impl Default for Material {
    fn default() -> Self {
        Self::Solid{ color: point![255, 0, 0] }
    }
}

#[derive(Debug)]
pub struct Object {
    shape: Shape,
    material: Material,
}


impl Ray {
    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t*self.dir
    }

    pub fn cast<'a, T>(
        &self, 
        objects: T
    ) 
    -> Option<(Point3<f64>, usize)> 
    where
        T: Iterator<Item = &'a Object>
    {
        let mut min = f64::INFINITY;
        let mut object_hit = None;

        for (i, object) in objects.enumerate() {
            let Some(curr) = object.shape.intersect(&self) else { continue };
            if curr < min {
                min = curr;
                object_hit = Some(i);
            }
            min = f64::min(curr, min);
        }

        object_hit.map(|obj| (self.at(min), obj))
    }

    pub fn trace(&self, scene: &Scene) -> Option<Point3<u8>> {
        let (p, i) = self.cast(scene.objects.iter())?;
        let color = match scene.objects[i].material {
            Material::Solid { color } => Some(color),
            _ => None
        }?;
        let mut brightness: f64 = 0.0;
        for light in &scene.lights {
            if !light.occluded(p, scene.objects.iter()) {
                brightness += 10.0 / (1.0 + 0.5*na::distance(&light.0, &p));
            }
        }
        let v = sigmoid(brightness);
        let (mut cx, mut cy, mut cz) = (color.x, color.y, color.z);
        cx = ((cx as f64) * v) as u8;
        cy = ((cy as f64) * v) as u8;
        cz = ((cz as f64) * v) as u8;
        let newcolor = point![cx, cy, cz];
        // println!("{} * {} = {}", color, v, newcolor);
        Some(point![cx, cy, cz])

        /* cast to light */
        /*
        let (p, object) = loop {
            let (p, object) = ray.cast(&scene.objects[..])?;
            if let Material::Reflective = object.material {
                let normal = object.shape.normal(&p);
                let proj = self.dir.dot(&normal) * normal;
                ray = Ray {
                    origin: p,
                    dir: self.dir - 2.0*proj 
                };
            } else {
                break (p, object);
            }
        };

        // TODO: cast rays to light sources
        for _ in &scene.lights {
        }

        match object.material {
            Material::Solid { color } => Some(color),
            Material::Checkered { 
                color0,
                color1,
                up,
                scale,
            } => { 
                todo!()
            },
            _ => None,
        }
        */
    }

        /*

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
        */
}


#[derive(Debug)]
pub struct Light(Point3<f64>);
impl Light {

    pub fn occluded<'a, T>(
        &self, 
        p: Point3<f64>, 
        objects: T 
    ) -> bool 
    where
        T: Iterator<Item = &'a Object>
    {
        let ray = Ray {
            origin: self.0,
            dir: p - self.0,
        };
        objects
            .filter_map(|obj| obj.shape.intersect(&ray))
            .any(|t| t >= 0.0 && t < 1.0 - 10e-6)
    }
}

