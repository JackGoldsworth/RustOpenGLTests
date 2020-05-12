extern crate sdl2;

struct Thing { x: i32, y: i32 }

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Test Window", 500, 500).resizable().build().unwrap();
    let mut event = sdl.event_pump().unwrap();
    'main: loop {
        for event in event.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    }
}