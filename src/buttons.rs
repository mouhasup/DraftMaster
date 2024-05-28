use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub struct Button {
    pub rect: Rect,
    pub color: Color,
    pub code: String,
    pub moussdown: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32,code: String) -> Button {
        Button {
            rect: Rect::new(x, y, width, height),
            color: Color::RGB(50, 50, 50),
            code,
            moussdown:false,
        }
    }
    pub fn fill_buttons() -> Vec<Button> {
        let mut buttons: Vec<Button> = Vec::new();
        let width: u32 = 50;
        let height: u32 = 30;
        //drawing buttons
        buttons.push(Button::new(0, 0, width, 30,"line".to_string())); //line
        buttons.push(Button::new((width as i32 + 2) * 1, 0, 50, 30,"poly_line".to_string())); //poly_line
        buttons.push(Button::new((width as i32 + 2) * 2, 0, 50, 30,"axe".to_string())); //axe
        buttons.push(Button::new(
            (width as i32 + 2) * 0,
            (height as i32 + 2) * 1,
            50,
            30,
            "rectangle".to_string()
        )); //rectangle
        buttons.push(Button::new(
            (width as i32 + 2) * 1,
            (height as i32 + 2) * 1,
            50,
            30,
            "polygone".to_string()
        )); //polygone
        buttons.push(Button::new(
            (width as i32 + 2) * 2,
            (height as i32 + 2) * 1,
            50,
            30,
            "point".to_string()
        )); //point
        buttons
    }
    // Set a new color to the button
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    // Set true to the moussdown
    // It means that the moussdown events is detected on the button
    pub fn set_moussdown(&mut self){
        self.moussdown = true;
    }
}
