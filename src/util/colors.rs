

// shitty "shader"
use crate::la::linear_algebra::Vec3f;

pub fn find_color(position: &Vec3f) -> (u8, u8, u8) {
    // 0 < x,y,z < 10
    let norm_position = position.normalize();
    let nr = (norm_position.e[0] * 255.0) as u8;
    let ng = (norm_position.e[1] * 255.0) as u8;
    let nb = (norm_position.e[2] * 255.0) as u8;
    // for high absolute value of z, 
    
    return (nr, ng, nb);
}
