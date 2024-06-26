// use egui::ahash::HashMap;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::EventPump;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

// use crate::Keycode;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::Button;

pub fn handle_events(
    mut event_pump: &mut EventPump,
    buttons: &mut Vec<Button>,
    viewport_widgets: Rect,
    viewport_drawing: Rect,
    draw_codes: &mut HashMap<String, bool>,
    points_drawing: &mut Vec<Point>,
    mouse_drawing_position: &mut Point,
) -> (bool, bool) {
    let mut needs_render = false;
    let mut can_break = false;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return (true, true),
            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => {
                let (x, y) = (x as i32, y as i32);
                let mouse_position = Point::new(x, y);
                if viewport_widgets.contains_point(mouse_position) {       // Widgets viewport
                    // Gestion des événements pour les widgets
                    needs_render = true;
                    for button in &mut buttons.iter_mut() {
                        if button.rect.contains_point(mouse_position) {
                            if mouse_btn == MouseButton::Left {
                                // Clic gauche sur les widgets
                                button.set_color(Color::RGB(100, 100, 100)); 
                                button.set_moussdown();
                            }
                        }
                    }
                } else if viewport_drawing.contains_point(mouse_position) { // Drawing viewport
                    // Gestion des événements pour le dessin
                    // needs_render = true;

                    if mouse_btn == MouseButton::Left {
                        // Clic gauche pour dessiner
                        points_drawing.push(Point::new(x,y));
                        // needs_render = true;
                    }
                }
            }
            Event::MouseButtonUp {
                x, y, mouse_btn, ..
            } => {
                let (x, y) = (x as i32, y as i32);
                let mouse_position = Point::new(x, y);
                if viewport_widgets.contains_point(mouse_position) {       // Widgets viewport
                    // Gestion des événements pour les widgets
                    needs_render = true;
                    for button in &mut buttons.iter_mut() {
                        // Clic gauche sur les widgets
                        button.set_color(Color::RGB(50, 50, 50));
                        if button.moussdown{

                        }
                    }
                } else if viewport_drawing.contains_point(mouse_position) { // Drawing viewport
                    // Gestion des événements pour le dessin
                    // needs_render = true;
                    if mouse_btn == MouseButton::Left {
                        // Clic gauche pour dessiner
                    }
                }
            }
            Event::MouseMotion { x, y, .. } => {
                let (x, y) = (x as i32, y as i32);
                let mouse_position = Point::new(x, y);
                if viewport_drawing.contains_point(mouse_position) {  // Drawing viewport
                    *mouse_drawing_position = mouse_position;
   
                }
            }
            _ => {}
        }
    }
    (needs_render, false)
}
