use std::time;
use glium::glutin::{event, event_loop, window, dpi};
use glium::{Surface, glutin, SwapBuffersError, Program, implement_vertex, index};
use glium;

const VHLINE: f64 = 133.33;     
const WSIZE: f64 = 400.0;      

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
}

implement_vertex!(Vertex, pos);

#[allow(unused_variables)]
fn main() {

    let events_loop = event_loop::EventLoop::new();
    let display = init_window(&events_loop); 

    events_loop.run(move | ev, wtarget, control_flow | {
        let hmonitor = wtarget.primary_monitor();
        set_window_attrib(&display).expect("SET_WINDOW_ATTRIB_ERROR");

        let fps = time::Instant::now() + time::Duration::from_nanos(16_666_667);
        *control_flow = event_loop::ControlFlow::WaitUntil(fps);

        match ev {
            event::Event::WindowEvent {event, ..} => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}

fn draw_vhlines() -> Vec<Vertex> { 
    let p1 = Vertex { pos: [0.0, 0.7] };
    let p2 = Vertex { pos: [0.0, -0.7] };
    vec![p1, p2]
}

fn init_window<T>(events_loop: &event_loop::EventLoop<T>) -> glium::Display {
    let wb = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(WSIZE,WSIZE))
        .with_resizable(false)
        .with_title("tic-tac-toe");
    let ctx = glutin::ContextBuilder::new();
    glium::Display::new(wb, ctx, events_loop).unwrap()
}

fn vertex_shader() -> &'static str {
    r#"
        #version 140
        in vec2 pos;
        void main() { 
            gl_Position = vec4(pos, 0.0, 0.1); 
        }
    "#
}

fn fragment_shader() -> &'static str {
    r#"
        #version 140
        out vec4 color;
        void main() { 
            color = vec4(1.0, 0.0, 0.0, 1.0); 
        }
    "#
}

fn set_window_attrib(display: &glium::Display) -> Result<(), SwapBuffersError> {
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.1, 1.0);
    
    // vertex, index, program, uniforms, draw_parameters..
    let vertex_buffer = glium::VertexBuffer::new(display, &draw_vhlines()).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::LinesList);
    let vertex_shader_src = vertex_shader(); 
    let fragment_shader_src = fragment_shader(); 
    let program = Program::from_source(
        display, vertex_shader_src, fragment_shader_src, None).unwrap();
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    frame.finish()
}
