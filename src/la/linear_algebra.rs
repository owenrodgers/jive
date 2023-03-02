
// vec3f
// mat3x3
// mat4x4 (for projection)

// ----- vec3f -----

/*
Want:
dot
cross
normalize

operator overloads
new(a,b,c)
from([a,b,c])
*/
use std::ops::MulAssign;

pub struct Vec3f{
    pub e : [f32; 3],
}
impl Vec3f{
    pub fn new(data: [f32; 3]) -> Vec3f {
        Vec3f{e: data}
    }
    pub fn from(a: f32, b: f32, c: f32) -> Vec3f {
        Vec3f{e: [a,b,c]}
    }
    pub fn normalize(&self) -> Vec3f {
        // returns another vector, doesnt mutate self
        let mag = self.magnitude();
        Vec3f{e: [self.e[0] / mag, self.e[1] / mag, self.e[2] / mag]}
    }
    pub fn magnitude(&self) -> f32 {
        let mag_squared: f32 = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        return mag_squared.sqrt();
    }
}

pub fn vec3f_dot(vec_a: Vec3f, vec_b: Vec3f) -> f32 {
    return vec_a.e[0] * vec_b.e[0] + vec_a.e[1] * vec_b.e[1] + vec_a.e[2] * vec_b.e[2];
}

pub fn vec3f_cross(vec_a: Vec3f, vec_b: Vec3f) -> Vec3f {
    let nx = vec_a.e[1]*vec_b.e[2] - vec_a.e[2]*vec_b.e[1];
    let ny = vec_a.e[2]*vec_b.e[0] - vec_a.e[0]*vec_b.e[2];
    let nz = vec_a.e[0]*vec_b.e[1] - vec_a.e[1]*vec_b.e[0];
    return Vec3f::from(nx, ny, nz);
}

impl MulAssign<&Mat3x3> for Vec3f {
    fn mul_assign(&mut self, mat: &Mat3x3){
        self.e[0] = mat.e[0]*self.e[0] + mat.e[1]*self.e[1] + mat.e[2]*self.e[2];
        self.e[1] = mat.e[3]*self.e[0] + mat.e[4]*self.e[1] + mat.e[5]*self.e[2];
        self.e[2] = mat.e[6]*self.e[0] + mat.e[7]*self.e[1] + mat.e[8]*self.e[2];
    }
}

impl MulAssign<&Mat4x4> for Vec3f {
    fn mul_assign(&mut self, matrix: &Mat4x4){
    
        let w: f32;
        self.e[0] = self.e[0]*matrix.e[0] + self.e[1]*matrix.e[4] + self.e[2]*matrix.e[8] + matrix.e[12];
        self.e[1] = self.e[0]*matrix.e[1] + self.e[1]*matrix.e[5] + self.e[2]*matrix.e[9] + matrix.e[13];
        self.e[2] = self.e[0]*matrix.e[2] + self.e[1]*matrix.e[6] + self.e[2]*matrix.e[10] + matrix.e[14];
        w = self.e[0]*matrix.e[3] + self.e[1]*matrix.e[7] + self.e[2]*matrix.e[11] + matrix.e[15];
        if w != 0.0{
            self.e[0] /= w; self.e[1] /= w; self.e[2] /= w;
        }
    }
}


// ----- Mat3x3 -----

pub struct Mat3x3{
    pub e: [f32; 9],
}
impl Mat3x3{
    pub fn new() -> Mat3x3{
        let dat: [f32; 9] = [0.0; 9];
        Mat3x3{ e: dat}
    }
    pub fn identity() -> Mat3x3 {
        let mut dat: [f32; 9] = [0.0; 9];
        dat[0] = 1.0; dat[4] = 1.0; dat[8] = 1.0;
        Mat3x3{ e : dat }
    }
    pub fn roll(theta: f32) -> Mat3x3 {
        let mut dat: [f32; 9] = [0.0; 9];
        dat[0] = 1.0;
        dat[4] = theta.cos();
        dat[5] = -1.0 * theta.sin();
        dat[7] = theta.sin();
        dat[8] = theta.cos();
        Mat3x3{ e : dat }
    }
}
impl MulAssign<&Mat3x3> for Mat3x3 {
    fn mul_assign(&mut self, mat: &Mat3x3) {
        let mut dat: [f32; 9] = [0.0; 9];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    dat[i * 3 + j] += self.e[i * 3 + k] * mat.e[k * 3 + j];
                }
            }
        }
        self.e = dat;

    }
}

// ---- 4x4 Matrices ----
pub struct Mat4x4{
    pub e: [f32; 16],
}
impl Mat4x4 {
    pub fn new() -> Mat4x4{
        let dat: [f32; 16] = [0.0; 16];
        Mat4x4{ e : dat }
    }
    pub fn projection(&mut self, h: &f32, w: &f32, fov: &f32, zfar: &f32, znear: &f32){
        let a: f32 = h / w;
        let f: f32 = 1.0 / (fov*0.5).tan();
        let q: f32 = zfar / (zfar - znear);
    
        self.e[0] = a * f;
        self.e[5] = f;
        self.e[10] = q;
        self.e[11] = 1.0;
        self.e[14] = (-1.0 * zfar * znear) / (zfar - znear);
    }
}





