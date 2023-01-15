use nannou::prelude::*;
use nannou_egui::Egui;

// tool mapping
// 0 -> drawing, 1 -> ellipse, 2 -> square

pub enum Elements {
    Pencil(PixelVec),
    Rect(Rectangle),
    Ellipse(Ellipse),
    Rubber,
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

#[derive(Clone)]
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

#[derive(Clone)]
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
}

impl PixelVec {
    pub fn new() -> Self {
        Self {
            vec_pix: Vec::new(),
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
    pub weight: Vec<f32>,
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
        weight: Vec<f32>,
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
            weight,
        }
    }

    pub fn display(&self, draw: &Draw, app: &App) {
        match self.settings.tool {
            Tools::Pencil => {
                draw.polyline()
                    .start_cap_round()
                    .caps_round()
                    .end_cap_round()
                    .join_round()
                    .stroke_weight(self.settings.weight)
                    .points_colored(self.pencil.vec_pix.clone());
            }
            Tools::Ellipse => {
                draw.ellipse()
                    .xy(self.ellipse.center)
                    .color(self.settings.color)
                    .radius(self.ellipse.radius);
            }
            Tools::Rect => {
                draw.rect()
                    .xy(self.rect.start)
                    .width(self.rect.width)
                    .height(self.rect.height)
                    .color(self.rect.color);
            }
            Tools::Rubber => {}
        }
    }
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

// pub fn erase_elements(pencil: &mut Model, mouse_pos: Vec2) {
//     model.elements.iter_mut().for_each(|pencil| {
//         pencil.vec_pix.iter_mut().for_each(|(p, c)| {
//             if check_range(p, mouse_pos, 5.) {
//                 *c = color::hsv(0., 0., 0.)
//             }
//         });
//     });
// }
