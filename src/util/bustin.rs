extern crate sdl2;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;


use crate::jives::jivesurface::JiveSurface;
use crate::la::linear_algebra::Vec3f;
use crate::la::linear_algebra::Mat4x4;

use crate::util::colors::find_color;
use std::f32::consts::PI;

pub fn util_function() {
    println!("Need some utils to bust jives");
}
pub fn degrees_to_radians(degrees: &f32) -> f32 {
    return degrees * (PI / 180.0);
}

//event pump and canvas
pub fn jive_render_init(screen_width: u32, screen_height: u32) -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust sdl2 window", screen_width as u32, screen_height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let canvas = window
        .into_canvas()
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    return (canvas, event_pump);
}

// points
pub struct ScreenData{
    //contains useful info
    pub screen_width: f32,
    pub screen_height: f32,
    pub field_of_view: f32,
    pub f_near: f32,
    pub f_far: f32,
}

impl ScreenData{
    pub fn projection_matrix(&self) -> Mat4x4 {
        let mut mat: Mat4x4 = Mat4x4::new();
        let a: f32 = self.screen_height / self.screen_width;
        let f: f32 = 1.0 / (degrees_to_radians(&(self.field_of_view*0.5))).tan();
        let q: f32 = self.f_far / (self.f_far - self.f_near);
    
        mat.e[0] = a * f;
        mat.e[5] = f;
        mat.e[10] = q;
        mat.e[11] = 1.0;
        mat.e[14] = (-1.0 * self.f_far * self.f_near) / (self.f_far - self.f_near);
        return mat;
    }
}


pub fn render_jive_surface(canvas: &mut WindowCanvas, jive_surface: &JiveSurface, screen_data: &ScreenData ) -> Result<(), String> {
    let raw_vertex_data: Vec<Vec3f> = jive_surface.surface_data();
    let mut render_vertex: Vec3f;
    let mut position_vertex: Vec3f;
    let projection_matrix = screen_data.projection_matrix();

    for vertex in raw_vertex_data.iter() {
        render_vertex = Vec3f{e : vertex.e};
        position_vertex = Vec3f{e : vertex.e};
        
        // apply the surfaces transformation matrix
        render_vertex *= &jive_surface.surface_transformation;
        // multiply by projection mat
        render_vertex *= &projection_matrix;
        
        // add 1 to x and y
        render_vertex.e[0] += 1.0; render_vertex.e[1] += 1.0;

        // scale x by half screen width
        render_vertex.e[0] *= screen_data.screen_width / 2.0;

        // scale y by half screen height
        render_vertex.e[1] *= screen_data.screen_height / 2.0;

        // render the point and find the color
        let (r,g,b) = find_color(&position_vertex);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.fill_rect(Rect::new(render_vertex.e[0] as i32, render_vertex.e[1] as i32, 4, 4))?;
    }
    Ok(())
}