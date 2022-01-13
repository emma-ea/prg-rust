use std::time;
use glium::glutin::{event, event_loop, window, dpi};
use glium::{Surface, glutin, SwapBuffersError};
use glium;


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

fn init_window<T>(events_loop: &event_loop::EventLoop<T>) -> glium::Display {
    let wb = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(400.0, 400.0))
        .with_title("tic-tac-toe");
    let ctx = glutin::ContextBuilder::new();
    glium::Display::new(wb, ctx, events_loop).unwrap()
}

fn set_window_attrib(display: &glium::Display) -> Result<(), SwapBuffersError> {
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.1, 1.0);
    frame.finish()
}
