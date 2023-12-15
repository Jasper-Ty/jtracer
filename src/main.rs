extern crate nalgebra as na;

use std::fs::File;
use std::io::prelude::*;

use na::{ Point3, Vector3, Matrix4, Isometry3 };
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("out.ppm"))]
    filename: String
}


struct Viewport {
    target: Point3<f64>,
    eye: Point3<f64>,
    up: Vector3<f64>,
}

#[derive(Debug)]
struct Ray {
    origin: Point3<f64>,
    dir: Vector3<f64>,
}

#[derive(Debug)]
struct Plane {
    p0: Point3<f64>,
    p1: Point3<f64>,
    p2: Point3<f64>,
}
impl Plane {
    pub fn u(&self) -> Vector3<f64> { self.p1 - self.p0 }
    pub fn v(&self) -> Vector3<f64> { self.p2 - self.p0 }
    pub fn n(&self) -> Vector3<f64> { self.u().cross(&self.v()) }

    pub fn proj_coords(&self, pt: &Point3<f64>) -> (f64, f64) {
        let dp = pt - self.p0;
        let u = dp.dot(&self.u())/self.u().dot(&self.u());
        let v = dp.dot(&self.v())/self.v().dot(&self.v());
        (u, v)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
        if ray.dir.dot(&self.n()) != 0.0 {
            let t = -(ray.origin - self.p0).dot(&self.n())/ray.dir.dot(&self.n());
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

fn main() {
    let args = Args::parse();

    let umin: f64 = -1.0;
    let umax: f64 = 1.0;
    let vmin: f64 = -1.0;
    let vmax: f64 = 1.0;

    let image_width = 500;
    let image_height = 500;

    let target = Point3::new(100.4, 0.0, -101.0);
    let eye = Point3::new(100.0, 0.0, -100.0);
    let up = Vector3::new(0.3, -1.0, 0.0);

    let to_vrc = Isometry3::look_at_rh(&eye, &target, &up);
    let ground = Plane {
        p0: to_vrc * Point3::new(0.0, -1.0, 0.0),
        p1: to_vrc * Point3::new(1.0, -1.0, 0.0),
        p2: to_vrc * Point3::new(0.0, -1.0, -1.0),
    };

    let mut file = File::create(&args.filename).unwrap();
    write!(&mut file, "P6\n").unwrap();
    write!(&mut file, "{} {}\n", image_width, image_height).unwrap();
    write!(&mut file, "255\n").unwrap();

    let du = (umax - umin)/f64::from(image_width);
    let dv = (vmax - vmin)/f64::from(image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pix = [0u8; 3];
            let dir = Vector3::new(umin + f64::from(i)*du, vmin + f64::from(j)*dv, 1.0);
            let ray = Ray {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir, 
            };

            let intersection = ground.intersect(&ray);
            match intersection {
                Some(pt) => {

                    let (u, v) = ground.proj_coords(&pt);
                    let (u, v) = (u as i64, v as i64);
                    let w = (u.wrapping_add(v)) % 2 == 0;

                    let dist = 255.0/f64::log2(na::distance(&pt, &ray.origin));
                    let dist = f64::from(dist).clamp(0.0, 255.0) as u8;
                    if w {
                        pix[0] = dist;
                        pix[1] = 0;
                        pix[2] = 0;
                    } else {
                        pix[0] = 0;
                        pix[1] = dist;
                        pix[2] = 0;
                    }
                },
                None => {
                    pix[0] = 0;
                    pix[1] = 0;
                    pix[2] = 0;
                },
            }

            file.write_all(&pix).unwrap();
        }
    }

    

    println!("Hello! {}", args.filename);
}
