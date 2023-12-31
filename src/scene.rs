use std::fs;
use std::str::FromStr;

use na::{Isometry3, Point3, Vector3, point, vector};
use crate::{Viewport, Shape, Object, Material, Light};

enum _Operand {
    Float(f64),
    Char(u8),
    Vector(Vector3<f64>),
    Point(Point3<f64>),
    Shape(Shape),
    Object(Object),
    Material(Material),
}

fn try_pop<T: FromStr>(stack: &mut Vec<&str>) -> Result<T, String> {
    Ok(stack.pop()
        .ok_or("Empty stack")?
        .parse::<T>()
        .map_err(|_| "Unable to parse")?)
}

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}
impl Scene {

    pub fn to_vrc(&mut self, Viewport{ eye, target, up }: &Viewport) {
        let tfm = Isometry3::look_at_rh(&eye, &target, &up);
        for object in self.objects.iter_mut() {
            match object.shape {
                Shape::Sphere { ref mut p, .. } => {
                    *p = tfm * *p;
                }
                Shape::Plane { ref mut p, ref mut n } => {
                    *p = tfm * *p;
                    *n = tfm * *n;
                }
                Shape::Triangle { 
                    ref mut p0,
                    ref mut p1,
                    ref mut p2,
                } => {
                    *p0 = tfm * *p0;
                    *p1 = tfm * *p1;
                    *p2 = tfm * *p2;
                }
            }
        }
        for light in self.lights.iter_mut() {
            light.0 = tfm * light.0;
        }
    }

    pub fn from_file(filepath: &str) -> Result<Self, String> {
        let mut stack: Vec<&str> = vec![];

        let mut objects: Vec<Object> = vec![];
        let mut lights: Vec<Light> = vec![];
        let s = fs::read_to_string(filepath)
            .map_err(|e| e.to_string())?;

        for tok in s.split_whitespace() {
            match tok {
                "light" => {
                    let z: f64 = try_pop(&mut stack)?;
                    let y: f64 = try_pop(&mut stack)?;
                    let x: f64 = try_pop(&mut stack)?;
                    lights.push(Light(point![x, y, z]));
                }

                "sphere" => {
                    let r: f64 = try_pop(&mut stack)?;
                    let z: f64 = try_pop(&mut stack)?;
                    let y: f64 = try_pop(&mut stack)?;
                    let x: f64 = try_pop(&mut stack)?;
                    let p = point![x, y, z];
                
                    objects.push(Object{
                        shape: Shape::Sphere { p, r },
                        material: Material::default()
                    });
                },
                "plane" => {
                    let nz: f64 = try_pop(&mut stack)?;
                    let ny: f64 = try_pop(&mut stack)?;
                    let nx: f64 = try_pop(&mut stack)?;
                    let n = vector![nx, ny, nz];
                    let pz: f64 = try_pop(&mut stack)?;
                    let py: f64 = try_pop(&mut stack)?;
                    let px: f64 = try_pop(&mut stack)?;
                    let p = point![px, py, pz];
                
                    objects.push(Object{
                        shape: Shape::Plane { p, n },
                        material: Material::default()
                    });
                },
                "triangle" => {
                    let p2z: f64 = try_pop(&mut stack)?;
                    let p2y: f64 = try_pop(&mut stack)?;
                    let p2x: f64 = try_pop(&mut stack)?;
                    let p2 = point![p2x, p2y, p2z];
                    let p1z: f64 = try_pop(&mut stack)?;
                    let p1y: f64 = try_pop(&mut stack)?;
                    let p1x: f64 = try_pop(&mut stack)?;
                    let p1 = point![p1x, p1y, p1z];
                    let p0z: f64 = try_pop(&mut stack)?;
                    let p0y: f64 = try_pop(&mut stack)?;
                    let p0x: f64 = try_pop(&mut stack)?;
                    let p0 = point![p0x, p0y, p0z];

                    objects.push(Object{
                        shape: Shape::Triangle { p0, p1, p2 },
                        material: Material::default()
                    });
                },

                "solid" => {
                    let b: u8 = try_pop(&mut stack)?;
                    let g: u8 = try_pop(&mut stack)?;
                    let r: u8 = try_pop(&mut stack)?;
                    objects.last_mut()
                        .ok_or("No object")?
                        .material = Material::Solid {
                            color: point![r, g, b]
                        };
                },
                "reflective" => {
                    objects.last_mut()
                        .ok_or("No object")?
                        .material = Material::Reflective 
                },
                "checkered" => {
                    let scale: f64 = try_pop(&mut stack)?;
                    let upz: f64 = try_pop(&mut stack)?;
                    let upy: f64 = try_pop(&mut stack)?;
                    let upx: f64 = try_pop(&mut stack)?;
                    let up = vector![upx, upy, upz];
                    let b1: u8 = try_pop(&mut stack)?;
                    let g1: u8 = try_pop(&mut stack)?;
                    let r1: u8 = try_pop(&mut stack)?;
                    let color1 = point![r1, g1, b1];
                    let b0: u8 = try_pop(&mut stack)?;
                    let g0: u8 = try_pop(&mut stack)?;
                    let r0: u8 = try_pop(&mut stack)?;
                    let color0 = point![r0, g0, b0];

                    objects.last_mut()
                        .ok_or("No object")?
                        .material = Material::Checkered {
                            color0, color1, up, scale
                        }
                },
                
                "|" => {},

                _ => stack.push(tok),
            }
        }

        Ok(Self { objects, lights })
    }
}
