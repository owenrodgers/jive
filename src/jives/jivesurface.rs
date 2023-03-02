/*
----- The jive surface -----
The main goal of jivesurface is to take the mathematial object passed in and based 
on a flag, generate a surface using parametrics

the JiveSurface struct needs to:
    ----- Be Consistent -----
    All of the mathematical objects need to be rendered in -10,10 space
    adhere to a list of surface flags like:
    CONE: 1
    SPHERE: 2
    etc...

    ----- Transform and project -----
    When the jivemesh is passed off to the jive surface everything should be normalized
    to -1,1 space

    projection matrices

pub struct JiveSurface{

}
*/

use core::f32::consts::PI;
use crate::la::linear_algebra::Vec3f;
use crate::la::linear_algebra::Mat3x3;

const SPHERE: u8 = 1;
const ELLIPSOID: u8 = 2;
const HYPERBOLOID: u8 = 3;
const PARABOLOID: u8 = 4;


const PLANE: u8 = 5;
const CONE: u8 = 6;


pub struct JiveSurface{
    surface_flag: u8,
    surface_coefficients: [f32; 6],
    pub surface_transformation: Mat3x3,

    vmin: usize, vmax: usize, vstep: usize,
    umin: usize, umax: usize, ustep: usize,

}
impl JiveSurface{

    pub fn new(flag: u8, coefficients: [f32; 6]) -> JiveSurface {
        let identity: Mat3x3 = Mat3x3::identity();
        match flag {
            SPHERE => {
                JiveSurface{surface_flag: SPHERE, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} 
            }
            ELLIPSOID => {
                JiveSurface{surface_flag: ELLIPSOID, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 180, vstep: 10, umin: 0, umax: 360, ustep: 4}
            }
            HYPERBOLOID => {
                JiveSurface{surface_flag: HYPERBOLOID, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 4}
            }
            PARABOLOID => {
                JiveSurface{surface_flag: PARABOLOID, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 4}
            }
            PLANE => {
                JiveSurface{surface_flag: PLANE, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 10, ustep: 1}
            }
            CONE => { 
                JiveSurface{surface_flag: CONE, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 10, vstep: 1, umin: 0, umax: 360, ustep: 1} 
            }
            _ => {
                JiveSurface{surface_flag: SPHERE, surface_coefficients: coefficients, surface_transformation: identity,
                            vmin: 0, vmax: 180, vstep: 30, umin: 0, umax: 360, ustep: 4} }
            }
    }
    pub fn render(&self) -> Result<(), String> {
        println!("{:?}", self.surface_flag);
        println!("{:?}", self.surface_coefficients);
        Ok(())
    }
    pub fn surface_data(&self) -> Vec<Vec3f> {
        let mut vertices = Vec::new();

        for v in (self.vmin..self.vmax).step_by(self.vstep) {
            let v_param = v as f32;
                
            for u in (self.umin..self.umax).step_by(self.ustep) {
                let u_param = u as f32;
                    
                let (x,y,z) = Self::solve(self.surface_flag, v_param, u_param, self.surface_coefficients);
                    vertices.push(Vec3f::from(x,y,z));
            }
        }
        return vertices;
    }
    pub fn solve(surface_type: u8, v_parameter: f32, u_parameter: f32, surface_coefficients: [f32; 6]) -> (f32, f32, f32) {
        let scale: f32 = 1.0 / 20.0;

        match surface_type {
            CONE => {
                // in this case v = t (y coordinate) and u = theta
                // so theta (v) needs to be converted to radians
                // the optional parameters come in as the steepness and height

                let height = surface_coefficients[1];
                let steepness = surface_coefficients[0];
                let scale = height / (200.0 * steepness);

                let theta = Self::d2rad(u_parameter);
                let x = scale * v_parameter * theta.cos();
                let y = scale * v_parameter * theta.sin();
                let z = scale * surface_coefficients[0] * v_parameter;
                (x,y,z)
            }
            PLANE => {
                let k: f32 = 1.0 / 20.0;
                //let xy_scale: f32 = 1.0 / 10.0;
                let a = surface_coefficients[0];
                let b = surface_coefficients[1];
                let c = surface_coefficients[2];
                let d = surface_coefficients[3];

                let mut x = 0.0;
                let mut y = 0.0;
                let mut z = 0.0;
                // if else is a hammer and everything is a nail

                if a == 0.0 && b == 0.0 && c == 0.0 {
                    println!("all zero, problem");
                } else if a != 0.0 && b != 0.0 && c != 0.0 {
                    // ax + by + cz = d
                    x = u_parameter; y = v_parameter; z = (1.0 / c) * (d - a*u_parameter - b*v_parameter);
                } else if a == 0.0 || b == 0.0 || c == 0.0 {
                    if a == 0.0 && b == 0.0 {
                        // cz = d
                        x = u_parameter; y = v_parameter; z = d;
                    } else if b == 0.0 && c == 0.0 {
                        // ax = d
                        x = u_parameter; y = d; z = v_parameter;
                    } else if a == 0.0 && c == 0.0 {
                        // by = d
                        x = d; y = v_parameter; z = u_parameter;
                    } else {
                        // ax + by + cz = d
                        x = u_parameter; y = v_parameter; z = (1.0 / c) * (d - a*u_parameter - b*v_parameter);
                    }
                }
                x *= k; y *= k; z *= k;
                (x,y,z)
            }
            ELLIPSOID => {
                // u = psi and v = theta
                //
                // both angles so need to go to radians
                let theta = Self::d2rad(v_parameter);
                let psi = Self::d2rad(u_parameter);
                let a: f32 = surface_coefficients[0] / 2.0;
                let b: f32 = surface_coefficients[2] / 2.0;
                let c: f32;
                let d = ((surface_coefficients[5]).abs()).sqrt() / 2.0;

                if a > b { 
                    c = b 
                } else {
                    c = a
                }
                let x = a * d * scale * psi.cos() * theta.sin();
                let y = b * d * scale * psi.sin() * theta.sin();
                let z = c * d * scale * theta.cos();
                (x,y,z)

            }
            HYPERBOLOID => {
                let theta = Self::d2rad(u_parameter);
                let v: f32 = v_parameter as f32 / 10.0;
                //let scale: f32 = 0.1;
                let a: f32 = surface_coefficients[0];
                let b: f32 = surface_coefficients[2];

                let x = scale * a * scale * v.cosh() * theta.cos();
                let y = scale * b * scale * v.cosh() * theta.sin();
                let z = scale * v.sinh();
                (x,z,y)
            }
            PARABOLOID => {
                let theta = Self::d2rad(u_parameter);
                let v: f32 = v_parameter;
                //let scale: f32 = 0.2;
                let a: f32 = surface_coefficients[0];
                let b: f32 = surface_coefficients[2];

                let x = a * scale * v * theta.cos();
                let y = b * scale * v * theta.sin();
                let z = scale * v * v;
                (x,y,z)
            }
            SPHERE => {
                // u = psi and v = theta
                // both angles so need to go to radians
                let theta = Self::d2rad(v_parameter);
                let psi = Self::d2rad(u_parameter);
                let d = ((surface_coefficients[5]).abs()).sqrt() / 2.0; 

                let x = d * scale * psi.cos() * theta.sin();
                let y = d * scale * psi.sin() * theta.sin();
                let z = d * scale * theta.cos();
                (x,y,z)
            }
            _ => {
                // have a nice cone
                let theta = Self::d2rad(v_parameter);
                let x = u_parameter * theta.cos();
                let y = u_parameter * theta.sin();
                let z = u_parameter;
                (x,y,z)
            }
        }
    }

    pub fn rotate_roll(&mut self, theta: f32) {
        let roll_mat = Mat3x3::roll(theta);
        self.surface_transformation *= &roll_mat;
    }


    fn d2rad(degrees: f32) -> f32 {
        return degrees * (PI / 180.0);
    }
    
    
}



