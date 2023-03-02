
// bolt on for sdl2 pretty much
// what do I want this library to do?
/*

frdim is a math library
jive is a graphics library

*/

/*

----- The jive model -----

the JiveModel struct needs to:
    ----- House important SCREEN SPACE Translations -----
    by the time jive mesh is invoked the computation for vertex data is done

    ----- Process events -----
    Need a blanket implementation of standard events that jive can interpret
    like arrow keys, and ** moving the camera with the mouse **

    ----- Handle Errors -----
    Now that you know how to program in rust theres no reason this program ever panics
    Result, Option

    ----- Apply Shaders -----
    This is going to look good.

pub struct JiveModel {

}


----- The jive surface -----
Jivesurface is going to be specifically made for frdim, it will adhere to a list
of surface flags that indicate which shapes to render

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

----- The jive library -----
The main goal of this lib is to put all of the rendering under the hood
All that you are concerned with is the math

----- Functions -----
blanket render function

pub fn render( surface: &JiveSurface ) -> Result<(), String> {
    let renderable_container = surface.to_JiveContainer()
    let rotations = renderable_container.rotations;
    let translations = renderable_container.translations;

    for vertex in renderable_container() {
        apply_transformations(rotations, translations, vertex)
        set_color( some data to give to the shader )
        point( x, y, z )
    }
}

extern crate jive;
use "render_jive_mesh"

pub fn main() -> Result<(), String> {
    // hyperconic sections
    let hyperconic_section = intersection(cone, plane)
    render( hyperconic_section.to_surface() )
}
*/



pub fn main() -> Result<(), String> {
    println!("New shitty graphics lib");
    
    Ok(())

}
