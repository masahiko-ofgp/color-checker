use sdl2;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::error::Error;

const TITLE: &'static str = "Color checker";
const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 300;

struct App {
    canvas: Canvas<Window>,
    event: sdl2::EventPump,
}

impl App {
    fn init() -> Result<Self, String> {
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;
        
        let win = video_subsys.window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        
        let cvs = win.into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        
        let event_pump = sdl_ctx.event_pump()?;

        Ok(App {
            canvas: cvs,
            event: event_pump,
        })
    }
    fn run_loop(&mut self) {
        let mut color = Color::RGB(0, 0, 0);

        'running: loop {
            self.canvas.set_draw_color(color);
            self.canvas.clear();
            self.canvas.present();
            
            for ev in self.event.poll_iter() {
                match ev {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode: Some(k), .. } => {
                        match k {
                            Keycode::Escape => break 'running,
                            Keycode::R => {
                                if color.r < 255 {
                                    color.r += 1;
                                } else {
                                    continue;
                                }
                            },
                            Keycode::G => {
                                if color.g < 255 {
                                    color.g += 1;
                                } else {
                                    continue;
                                }
                            },
                            Keycode::B => {
                                if color.b < 255 {
                                    color.b += 1;
                                } else {
                                    continue;
                                }
                            },
                            Keycode::E => {
                                if color.r > 0 {
                                    color.r -= 1;
                                } else {
                                    continue;
                                }
                            },
                            Keycode::F => {
                                if color.g > 0 {
                                    color.g -= 1;
                                } else {
                                    continue;
                                }
                            },
                            Keycode::V => {
                                if color.b > 0 {
                                    color.b -= 1;
                                } else {
                                    continue;
                                }
                            },
                            _ => {},
                        }
                    },
                    Event::KeyUp { keycode: Some(k), .. } => {
                        match k {
                            Keycode::Escape => break 'running,
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
        }
        println!("R: {} G: {} B: {}", color.r, color.g, color.b);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut app = App::init()?;
    app.run_loop();
    Ok(())
}
