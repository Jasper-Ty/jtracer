extern crate nalgebra as na;

use std::error::Error;
use std::fs::File;

use na::{ Point3, Vector3, Isometry3 };
use clap::Parser;

use jtracer::imbuf::ImBuf;
use jtracer::{Ray, Plane, Scene};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("out.ppm"))]
    filename: String,
    #[arg(long, default_value_t = 500)]
    width: usize,
    #[arg(long, default_value_t = 500)]
    height: usize 
}


struct Viewport {
    target: Point3<f64>,
    eye: Point3<f64>,
    up: Vector3<f64>,
}
/*
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
*/

// TODO: Figure out why TF planes look like that
//TODO: Generic intersect trace code with Option
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut imbuf = ImBuf::new(args.width, args.height);

    let umin: f64 = -1.0;
    let umax: f64 = 1.0;
    let vmin: f64 = -1.0;
    let vmax: f64 = 1.0;

    let image_width = 500;
    let image_height = 500;

    let target = Point3::new(0.0, 0.0, 0.0);
    let eye = Point3::new(0.0, 0.0, 1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let to_vrc = Isometry3::look_at_rh(&eye, &target, &up);

    let plane1 = Plane::new(
        to_vrc * Point3::new(0.0, -1.0, 0.0),
        to_vrc * Point3::new(1.0, -1.0, 0.0),
        to_vrc * Point3::new(0.0, -1.0, 1.0),
    );

    let plane2 = Plane::new(
        to_vrc * Point3::new(0.0, 0.0, 5.0),
        to_vrc * Point3::new(0.0, 1.0, 5.0),
        to_vrc * Point3::new(0.7, 0.0, 5.7),
    );

    let plane3 = Plane::new(
        to_vrc * Point3::new(0.0, 0.0, 6.0),
        to_vrc * Point3::new(0.0, 1.0, 5.4),
        to_vrc * Point3::new(-0.9, 0.0, 6.3),
    );

    let scene = Scene {
        planes: vec![plane1, plane2, plane3]
    };



    let du = (umax - umin)/f64::from(image_width);
    let dv = (vmax - vmin)/f64::from(image_height);
    for j in 0..args.height {
        for i in 0..args.width {
            let u = umin + (i as f64)*du;
            let v = vmin + (j as f64)*dv;
            let dir = Vector3::new(u, v, 1.0);
            let ray = Ray {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir, 
            };
            let p = ray.trace(&scene, 1);

            if let Some(pix) = p {
                imbuf[(i, j)] = pix;
            }
        }
    }

    let mut f = File::create(&args.filename)?;
    imbuf.to_ppm(&mut f)?;

    Ok(())
}
