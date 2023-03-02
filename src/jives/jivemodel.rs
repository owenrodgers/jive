
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

    ----- Add axes to scene -----

pub struct JiveModel {

}
*/

pub fn jivecontainer_add(a: f32, b: f32) -> f32 {
    a + b
}