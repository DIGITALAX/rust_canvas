use nannou::{color, prelude::*};
use nannou_egui::Egui;

// tool mapping
// 0 -> drawing, 1 -> ellipse, 2 -> square

pub enum Elements {
    Pencil(PixelVec),
    Rect(Rectangle),
    Ellipse(Ellipse),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Tools {
    Pencil,
    Rect,
    Ellipse,
    Rubber,
}

#[derive(Clone)]
pub struct Settings {
    pub color: Hsv,
    pub weight: f32,
    pub shapes: bool,
    pub tool: Tools,
}

pub struct Ellipse {
    pub center: Vec2,
    pub color: Hsv,
    pub weight: f32,
    pub radius: f32,
    pub clicked: bool,
}

impl Ellipse {
    pub fn new() -> Self {
        Self {
            center: pt2(0., 0.),
            color: hsv(10.0, 0.5, 1.0),
            weight: 1.,
            radius: 0.,
            clicked: false,
        }
    }
}

pub struct Rectangle {
    pub start: Point2,
    pub width: f32,
    pub height: f32,
    pub color: Hsv,
    pub weight: f32,
    pub clicked: bool,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            start: pt2(0., 0.),
            width: 0.,
            height: 0.,
            color: hsv(10.0, 0.5, 1.0),
            weight: 1.,
            clicked: false,
        }
    }
}

#[derive(Clone)]
pub struct PixelVec {
    pub vec_pix: Vec<(Point2, Hsv)>,
    pub weight: Vec<f32>,
}

impl PixelVec {
    pub fn new() -> Self {
        Self {
            vec_pix: Vec::new(),
            weight: Vec::new(),
        }
    }
}

pub struct Model {
    pub egui: Egui,
    pub settings: Settings,
    pub pencil: PixelVec,
    pub ellipse: Ellipse,
    pub rect: Rectangle,
    pub elements: Vec<Elements>,
    pub tools_type: Vec<Tools>,
    pub drawing: bool,
}

impl Model {
    pub fn new(
        egui: Egui,
        settings: Settings,
        pencil: PixelVec,
        ellipse: Ellipse,
        rect: Rectangle,
        elements: Vec<Elements>,
        tools_type: Vec<Tools>,
        drawing: bool,
    ) -> Self {
        Model {
            egui,
            settings,
            pencil,
            ellipse,
            rect,
            elements,
            tools_type,
            drawing,
        }
    }

    pub fn display(&self, draw: &Draw, app: &App) {
        let mouse_pos = pt2(app.mouse.x, app.mouse.y);
        for index in self.elements.iter().enumerate() {
            println!("here {:?}", self.tools_type[index.0]);
            match self.tools_type[index.0] {
                Tools::Pencil => free_draw(self, draw),
                Tools::Ellipse => draw_ellipse(self, draw),
                Tools::Rect => draw_rect(self, draw),
                Tools::Rubber => {}
            }
        }
    }
}

pub fn free_draw(model: &Model, draw: &Draw) {
    if !model.pencil.vec_pix.is_empty() {
        draw.polyline()
            .start_cap_round()
            .caps_round()
            .end_cap_round()
            .join_round()
            .stroke_weight(model.pencil.weight[model.pencil.weight.len() - 1])
            .points_colored(model.pencil.vec_pix.clone());
    }

    for (index, elem) in model.elements.iter().enumerate() {
        draw.polyline()
            .start_cap_round()
            .caps_round()
            .end_cap_round()
            .join_round()
            .stroke_weight(model.pencil.weight[index])
            .points_colored(elem.vec_pix.clone());
    }
}

pub fn draw_ellipse(model: &Model, draw: &Draw) {
    draw.ellipse()
        .xy(model.ellipse.center)
        .color(model.ellipse.color)
        .stroke_weight(model.ellipse.weight)
        .stroke_color(model.ellipse.color)
        .radius(model.ellipse.radius);
}

pub fn draw_rect(model: &Model, draw: &Draw) {
    draw.rect()
        .xy(model.rect.start)
        .width(model.rect.width)
        .height(model.rect.height)
        .stroke_weight(model.rect.weight)
        .stroke_color(model.ellipse.color)
        .color(model.rect.color);
}

fn check_range(p: &Vec2, m: Vec2, r: f32) -> bool {
    let range_x: std::ops::Range<f32> = m.x - r..m.x + r;
    let range_y = m.y - r..m.y + r;
    if range_x.contains(&p.x) && range_y.contains(&p.y) {
        true
    } else {
        false
    }
}

pub fn erase_elements(model: &mut Model, mouse_pos: Vec2) {
    model.elements.iter_mut().for_each(|pencil| {
        pencil.vec_pix.iter_mut().for_each(|(p, c)| {
            if check_range(p, mouse_pos, 5.) {
                *c = color::hsv(0., 0., 0.)
            }
        });
    });
}
