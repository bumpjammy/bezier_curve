use std::cmp::min;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::utils::BezierPoints;

pub(crate) fn render(canvas: &mut WindowCanvas, current_bezier_points: &BezierPoints, current_points: &mut Vec<Point>, frame: i32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    render_demo_points(canvas, &current_bezier_points, current_points, frame);
    render_bezier_points(canvas, &current_bezier_points);
    render_lines(canvas, &current_bezier_points, frame);

    canvas.present();
}

fn render_bezier_points(canvas: &mut WindowCanvas, current_points: &BezierPoints) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    render_full_circle_pixels(canvas, &current_points.start.location, 5);
    render_full_circle_pixels(canvas, &current_points.end.location, 5);
    render_full_circle_pixels(canvas, &current_points.control.location, 5);

    canvas.set_draw_color((40, 40, 255));
    if current_points.start.pressed {
        render_full_circle_pixels(canvas, &current_points.start.location, 5);
    }
    if current_points.end.pressed {
        render_full_circle_pixels(canvas, &current_points.end.location, 5);
    }
    if current_points.control.pressed {
        render_full_circle_pixels(canvas, &current_points.control.location, 5);
    }
}

fn render_lines(canvas: &mut WindowCanvas, current_points: &BezierPoints, frame: i32) {
    if frame > 120 {
        return;
    }
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.draw_line(current_points.start.location, current_points.control.location).expect("Failed to draw line!");
    canvas.draw_line(current_points.end.location, current_points.control.location).expect("Failed to draw line!");
}

fn render_demo_points(canvas: &mut WindowCanvas, current_bezier_points: &BezierPoints, current_points: &mut Vec<Point>, frame: i32) {
    let percentage = (min(frame, 120) as f32) / 120.0; // 2 seconds to complete animation

    let start_line_point= Point::new(
        ((1.0 - percentage) * current_bezier_points.start.location.x as f32
            + percentage * current_bezier_points.control.location.x as f32) as i32,
        ((1.0 - percentage) * current_bezier_points.start.location.y as f32
            + percentage * current_bezier_points.control.location.y as f32) as i32,
    );

    let end_line_point = Point::new(
        ((1.0 - percentage) * current_bezier_points.control.location.x as f32
            + percentage * current_bezier_points.end.location.x as f32) as i32,
        ((1.0 - percentage) * current_bezier_points.control.location.y as f32
            + percentage * current_bezier_points.end.location.y as f32) as i32,
    );

    let mid_point = Point::new(
        ((1.0 - percentage) * start_line_point.x as f32
            + percentage * end_line_point.x as f32) as i32,
        ((1.0 - percentage) * start_line_point.y as f32
            + percentage * end_line_point.y as f32) as i32,
    );
    current_points.push(mid_point);

    canvas.set_draw_color(Color::RGB(150, 120, 0));
    render_full_circle_pixels(canvas, &start_line_point, 2);
    render_full_circle_pixels(canvas, &end_line_point, 2);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    if frame < 120 {
        canvas.draw_line(start_line_point, end_line_point).expect("Failed to draw line!");
    }

    let mut previous_point = current_bezier_points.start.location;
    for point in &mut *current_points {
        canvas.draw_line(previous_point, *point).expect("Failed to draw line!");
        previous_point = *point;
    }
}

fn render_full_circle_pixels(canvas: &mut WindowCanvas, point: &Point, radius: i32) {
    let mut x = radius;
    let mut y = 0;
    let mut x_change = 1 - (radius << 1);
    let mut y_change = 0;
    let mut radius_error = 1 - x;

    while x >= y {
        for i in point.x - x..point.x + x {
            canvas.draw_point(Point::new(i, point.y + y)).expect("Failed to draw point");
            canvas.draw_point(Point::new(i, point.y - y)).expect("Failed to draw point");
        }
        for i in point.x - y..point.x + y {
            canvas.draw_point(Point::new(i, point.y + x)).expect("Failed to draw point");
            canvas.draw_point(Point::new(i, point.y - x)).expect("Failed to draw point");
        }

        y += 1;
        radius_error += y_change;
        y_change += 2;
        if ((radius_error << 1) + x_change) > 0 {
            x -= 1;
            radius_error += x_change;
            x_change += 2;
        }
    }
}