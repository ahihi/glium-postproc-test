#[macro_use]
extern crate glium;

use std::error::Error;

use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::glutin::ElementState::*;
use glium::glutin::Event::*;
use glium::glutin::VirtualKeyCode::*;
use glium::glutin::WindowBuilder;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{EmptyUniforms, Sampler};
use glium::texture::texture2d::Texture2d;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex {
        Vertex { position: [x, y] }
    }
}

implement_vertex!(Vertex, position);

fn main_result() -> Result<(), Box<Error>> {
    let display = try!(WindowBuilder::new().build_glium());
    let window = try!(display.get_window().ok_or("Failed to get window"));
    let (width, height) = try!(window.get_inner_size_pixels().ok_or("Failed to get inner size"));
    
    let default_vs_src = include_str!("default.vert");
    
    let triangle = vec![Vertex::new(0.0, 0.5), Vertex::new(-0.5, -0.5), Vertex::new(0.5, -0.5)];
    let triangle_vbuf = try!(VertexBuffer::new(&display, &triangle));
    let triangle_ixs = NoIndices(PrimitiveType::TrianglesList);
    let triangle_fs_src = include_str!("triangle.frag");
    let triangle_prog = try!(Program::from_source(&display, default_vs_src, triangle_fs_src, None));
    let triangle_unis = EmptyUniforms;
    
    let triangle_tex = try!(Texture2d::empty(&display, width, height));
    
    let rect = vec![Vertex::new(-1.0, -1.0), Vertex::new(1.0, -1.0), Vertex::new(1.0, 1.0), Vertex::new(-1.0, 1.0)];
    let rect_vbuf = try!(VertexBuffer::new(&display, &rect));
    let rect_ixs = NoIndices(PrimitiveType::TriangleFan);
    let rect_fs_src = include_str!("rect.frag");
    let rect_prog = try!(Program::from_source(&display, default_vs_src, rect_fs_src, None));
    let rect_unis = uniform! {
        resolution: [width as f32, height as f32],
        tex: Sampler::new(&triangle_tex)
    };

    'outer: loop {
        for ev in display.poll_events() {
            match ev {
                Closed | KeyboardInput(Pressed, _, Some(Escape)) =>
                    break 'outer,
                
                _ => ()
            }
        }
        
        { // Draw triangle to texture
            let mut target = triangle_tex.as_surface();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            try!(target.draw(&triangle_vbuf, &triangle_ixs, &triangle_prog, &triangle_unis, &Default::default()));
        }
                
        { // Draw rectangle with texture to display
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            try!(target.draw(&rect_vbuf, &rect_ixs, &rect_prog, &rect_unis, &Default::default()));
            try!(target.finish());
        }
    }
    
    Ok(())
}

fn main() {
    main_result().unwrap();
}
