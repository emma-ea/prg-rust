use glium;
use glium::{glutin, Surface};
extern crate nalgebra_glm as glm;

#[derive(Copy, Clone)]
struct Vertex3t2 {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

glium::implement_vertex!(Vertex3t2, position, tex_coords);

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 800.0))
        .with_title("generating boxes");
    let ctx = glutin::ContextBuilder::new().with_depth_buffer(24);
    let window = glium::Display::new(wb, ctx, &events_loop).unwrap();
    
    events_loop.run(move|ev, _, control_flow|{
        let mut target = window.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let program = 
            glium::Program::from_source(&window, vertex_shader_src(), fragment_shader_src(), None)
            .unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut verts: Vec<Vertex3t2> = Vec::new();
        for x in -2..3 {
            for y in -2..3 {
                for z in -2..3 {
                    add_cube(
                        &mut verts, 
                        &glm::Vec3::new(x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0),
                    );
                }
            }
        }
        let vertex_buffer = glium::VertexBuffer::new(&window, &verts).unwrap();
        
        let t: f32 = 0.559; // 32 deg

        let mut model_view = glm::rotate_z(&glm::identity(), t);
        model_view = glm::translate(&model_view, &glm::vec3(0.0, 0.0, -12.0));
        model_view = glm::rotate_x(&model_view, t / 2.0);
        model_view = glm::rotate_y(&model_view, t / 2.0);
        let view: [[f32; 4]; 4] = model_view.into();

        let perspective = glm::perspective(1.0, 3.14 / 2.0, 0.1, 1000.0);
        let p:[[f32; 4]; 4] = perspective.into();

        let uniforms = glium::uniform!{matrix: view, perspective: p};

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();

        target.finish().unwrap();

        match ev {
            glutin::event::Event::WindowEvent {event, ..} => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                _ => return,
            },
            _ => (),
        }
    });

}

fn add_cube(verts: &mut Vec<Vertex3t2>, pos: &glm::Vec3) {
    add_quad(
        verts,
        glm::vec3(-0.5, -0.5, 0.5) + pos,
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
    );
}

fn add_quad(dest: &mut Vec<Vertex3t2>, bottom_left: glm::Vec3, up: glm::Vec3, right: glm::Vec3) {
    let top_left: glm::Vec3 = (bottom_left + up).into();
    let top_right: glm::Vec3 = (top_left + right).into();
    let bottom_right: glm::Vec3 = (bottom_left + right).into();

    dest.push(Vertex3t2 {
        position: bottom_left.into(),
        tex_coords: [0.0, 0.0],
    });
    dest.push(Vertex3t2 {
        position: top_left.into(),
        tex_coords: [0.0, 1.0],
    });
    dest.push(Vertex3t2 {
        position: top_right.into(),
        tex_coords: [1.0, 1.0],
    });
    dest.push(Vertex3t2 {
        position: bottom_left.into(),
        tex_coords: [0.0, 0.0],
    });
    dest.push(Vertex3t2 {
        position: top_right.into(),
        tex_coords: [1.0, 1.0],
    });
    dest.push(Vertex3t2 {
        position: bottom_right.into(),
        tex_coords: [1.0, 0.0],
    });
}

fn vertex_shader_src() -> &'static str {
    r#"
        #version 140

        in vec3 position;
        in vec2 tex_coords;
        out vec2 v_tex_position;

        uniform mat4 matrix;
        uniform mat4 perspective;

        void main() {
            v_tex_position = tex_coords;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
        "#
}

fn fragment_shader_src() -> &'static str {
    r#"
        #version 140
        
        in vec2 v_tex_position;
        out vec4 color;

        void main() {
            float dst = min(v_tex_position.y, v_tex_position.x);
            dst = min(dst, min(2.0 - v_tex_position.y, 2.0 - v_tex_position.x));

            float intensity = smoothstep(0.1, 0.0, dst);
            vec3 col = vec3(0.0, intensity, 0.0);
            color = vec4(col, 1.0);
        }
        "#
}
