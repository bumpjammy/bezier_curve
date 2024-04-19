use sdl2::EventPump;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::rendering;
use crate::utils::{BezierPoints, BezierPoint, get_distance_between_points};

pub(crate) fn run_app_loop(canvas: &mut WindowCanvas, event_pump: &mut EventPump) {
    let mut current_points = BezierPoints {
        start: BezierPoint { location: Point::new(300, 300), pressed: false},
        end: BezierPoint { location: Point::new(800, 300), pressed: false },
        control: BezierPoint { location: Point::new(450, 750), pressed: false}
    };

    let mut frame = 1;
    let mut points: Vec<Point> = vec![];

    'running: loop {
        rendering::render(&mut *canvas, &current_points, &mut points, frame);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                sdl2::event::Event::MouseButtonDown {x, y, ..} => {
                    let mouse_point = Point::new(x, y);
                    if get_distance_between_points(&current_points.start.location, &mouse_point) <= 5 {
                        current_points.start.pressed = true;
                    }
                    else if get_distance_between_points(&current_points.end.location, &mouse_point) <= 5 {
                        current_points.end.pressed = true;
                    }
                    else if get_distance_between_points(&current_points.control.location, &mouse_point) <= 5 {
                        current_points.control.pressed = true;
                    }
                },
                sdl2::event::Event::MouseButtonUp {..} => {
                    if current_points.start.pressed {
                        current_points.start.pressed = false;
                    }
                    if current_points.end.pressed {
                        current_points.end.pressed = false;
                    }
                    if current_points.control.pressed {
                        current_points.control.pressed = false;
                    }
                },
                sdl2::event::Event::MouseMotion {x, y, ..} => {
                    let mouse_point = Point::new(x, y);
                    if current_points.start.pressed {
                        current_points.start.location = mouse_point;
                    }
                    else if current_points.end.pressed {
                        current_points.end.location = mouse_point;
                    }
                    else if current_points.control.pressed {
                        current_points.control.location = mouse_point;
                    }
                }
                _ => { },
            }
        }

        if current_points.start.pressed || current_points.end.pressed || current_points.control.pressed {
            frame = 1;
            points = vec![];
        } else {
            frame += 1;
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 120));
    }
}