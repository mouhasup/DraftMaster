use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::time::Duration;
// Widgets module
mod buttons;
use buttons::Button;
// Events module
mod events;
use events::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("DraftMaster", 1200, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // les variables d'Etats

    // Drawing
    // let mut drawing_line: bool = false;
    // let mut drawing_poly_line: bool = false;
    // let mut drawing_axe: bool = false;
    // let mut drawing_rect: bool = false;
    // let mut drawing_point: bool = false;
    // let mut drawing_circle: bool = false;
    // let mut drawing_polygone: bool = false;
    // // Modification
    // let mut copied: bool = false;
    // let mut moving: bool = false;
    // let mut miror: bool = false;

    // Fill buttons -----
    let mut buttons = Button::fill_buttons();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window_width = canvas.output_size().unwrap().0;
    let window_height = canvas.output_size().unwrap().1;
    let widgets_box_height: u32 = 200;

    let viewport_widgets = Rect::new(0, 0, window_width, widgets_box_height);
    let viewport_drawing = Rect::new(
        0,
        widgets_box_height as i32,
        window_width,
        window_height - widgets_box_height,
    );

    let mut draw_codes: HashMap<String, bool> = HashMap::new();

    // Initialize widgets -------
    for button in buttons.iter_mut() {
        canvas.set_draw_color(button.color);
        canvas.fill_rect(button.rect).unwrap();
        draw_codes.insert(button.code.clone(), false);
    }
    draw_codes.insert("first".to_string(), false);
    canvas.present();

    // All the cliqued points durring the drawing
    let mut points_drawing: Vec<Point> = Vec::new();
    let mut mouse_position: Point = Point::new(0, 0);

    'running: loop {
        let mut needs_render = false;
        let mut can_break = false;

        // Handle Events
        (needs_render, can_break) = handle_events(
            &mut event_pump,
            &mut buttons,
            viewport_widgets,
            viewport_drawing,
            &mut draw_codes,
            &mut points_drawing,
            &mut mouse_position,
        );
        // break the loop according to the handle events
        if can_break {
            break 'running;
        }

        // Render if needed
        if needs_render {
            // Effacer les deux viewports
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // Dessiner """----les widgets----""" dans le viewport des widgets
            canvas.set_viewport(viewport_widgets);
            for button in &buttons {
                canvas.set_draw_color(button.color);
                canvas.fill_rect(button.rect).unwrap();
            }

            // Dessiner """----les dessins----""" dans le viewport du dessin
            canvas.set_draw_color(Color::RGB(250, 250, 250));
            canvas.set_viewport(viewport_drawing);
            let _ = canvas.draw_line(Point::new(0, 0), Point::new(60, 80));
            let _ = canvas.draw_line(Point::new(0, 0), Point::new(61, 81));

            // Dessiner d'autres éléments ici
            for button in buttons {
                if let Some(tsss) = draw_codes.get(&button.code) {}
            }
            canvas.present();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
