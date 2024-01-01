extern crate nalgebra as na;

use std::error::Error;
use std::fs::File;

use na::{ Point3, Vector3, Isometry3 };
use clap::Parser;

use jtracer::imbuf::ImBuf;
use jtracer::{Ray, Scene, Viewport};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("scene.txt"))]
    scene_file: String,
    #[arg(long, default_value_t = String::from("out.ppm"))]
    filename: String,
    #[arg(long, default_value_t = 500)]
    width: usize,
    #[arg(long, default_value_t = 500)]
    height: usize 
}


// TODO: separate trace and bounce code?
// TODO: how to avoid specific object?
// TODO: one big vector?
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut imbuf = ImBuf::new(args.width, args.height);

    let umin: f64 = -1.0;
    let umax: f64 = 1.0;
    let vmin: f64 = -1.0;
    let vmax: f64 = 1.0;

    let image_width = 500;
    let image_height = 500;

    let target = Point3::new(0.0, 0.0, 1.0);
    let eye = Point3::new(0.0, -5.0, 1.0);
    let up = Vector3::new(0.0, 0.0, 1.0);
    let viewport = Viewport { target, eye, up };

    let mut scene = Scene::from_file(&args.scene_file)?;
    scene.to_vrc(&viewport);

    let du = (umax - umin)/f64::from(image_width);
    let dv = (vmax - vmin)/f64::from(image_height);
    for j in 0..args.height {
        for i in 0..args.width {
            let u = umin + (i as f64)*du;
            let v = vmin + (j as f64)*dv;
            let dir = Vector3::new(u, v, -1.0);
            let ray = Ray {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir, 
            };
            let p = ray.trace(&scene);

            if let Some(pix) = p {
                imbuf[(i, j)] = pix;
            }
        }
    }

    let mut f = File::create(&args.filename)?;
    imbuf.to_ppm(&mut f)?;

    Ok(())
}
