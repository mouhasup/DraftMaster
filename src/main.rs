extern crate sdl2;
mod shapes;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use shapes::DmPoint;
use std::time::Duration;
use std::time::Instant;

const WAIT_DURATION: Duration = Duration::from_millis(800);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut points: Vec<DmPoint> = Vec::new();
    let mut anime = false;
    let mut step = 0;
    let mut last_event_time = Instant::now();
    let mut old_point: Vec<DmPoint> = Vec::new();

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            // Réinitialiser le temps du dernier événement
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::KpEnter),
                    ..
                } => {
                    if points.len() > 1 {
                        anime = true;
                        let points_clone = &points;
                        old_point = points_clone.clone();
                        println!("len:{}", "points.len()");
                    }

                    // Ajoutez ici ce que vous voulez faire lors d'un clic gauche
                }

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        if !anime {
                            points.push(DmPoint::new(x, y));
                            println!("len: {}", points.len());
                        }
                        // Ajoutez ici ce que vous voulez faire lors d'un clic gauche
                    }
                }

                _ => {}
            }
        }

        if anime {
            if step == 7 {
                // anime = false;
                // points = Vec::new();
                points = old_point.clone();
                step = 0;
            }

            if Instant::now().duration_since(last_event_time) >= WAIT_DURATION {
                last_event_time = Instant::now();
                points = chaiks(&points);
                step += 1;
            }
            canvas.set_draw_color(Color::WHITE);

            

            if !points.is_empty() {
                for k in 0..points.len() - 1 {
                    let _ = canvas.draw_line(
                        Point::new(points[k].x as i32, points[k].y as i32),
                        Point::new(points[k + 1].x as i32, points[k + 1].y as i32),
                    );
                }
            }
            // The rest of the game loop goes here...
        } else {
            canvas.set_draw_color(Color::WHITE);
            for k in 0..points.len() {
                // let _ = canvas.draw_rect(Rect::new(
                //     points[k].x as i32 - 3,
                //     points[k].y as i32 - 3,
                //     6,
                //     6,
                // ));
                draw_circle(&mut canvas, points[k].x as i32, points[k].y as i32, 3)
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn chaiks(points: &Vec<DmPoint>) -> Vec<DmPoint> {
    let mut new_points = Vec::new();
    new_points.push(points[0]); // Ajouter le premier point

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        // Ajouter les points intermédiaires
        let q1 = p1.intermediate_point(p2, 1.0 / 4.0);
        let q2 = p1.intermediate_point(p2, 3.0 / 4.0);

        new_points.push(q1);
        new_points.push(q2);
    }

    new_points.push(points[points.len() - 1]); // Ajouter le dernier point
    new_points
}

fn draw_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius;

    while y >= x {
        draw_octant(canvas, center_x, center_y, x, y);
        x += 1;
        if d > 0 {
            y -= 1;
            d += 4 * (x - y) + 10;
        } else {
            d += 4 * x + 6;
        }
        draw_octant(canvas, center_x, center_y, x, y);
    }
}
fn draw_octant(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, x: i32, y: i32) {
    canvas
        .draw_point(Point::new(center_x + x, center_y + y))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x - x, center_y + y))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x + x, center_y - y))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x - x, center_y - y))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x + y, center_y + x))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x - y, center_y + x))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x + y, center_y - x))
        .unwrap();
    canvas
        .draw_point(Point::new(center_x - y, center_y - x))
        .unwrap();
}
