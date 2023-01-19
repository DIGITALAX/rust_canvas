use nannou::prelude::*;
use nannou_egui::Egui;
pub(crate) mod elements;
use elements::{Elements, Ellipse, Forms, Line, Pencil, Rectangle, Rectangle_Custom, Tool};

#[derive(Clone)]
pub struct Settings {
    pub color: Hsv,
    pub weight: f32,
    shapes: bool,
}

impl Settings {
    pub fn new(color: Hsv, weight: f32, shapes: bool) -> Self {
        Self {
            color,
            weight,
            shapes,
        }
    }

    pub fn set_shapes(&mut self, shape: bool) {
        self.shapes = shape
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_color(&self) -> Hsv {
        self.color
    }

    pub fn get_shapes(&self) -> bool {
        self.shapes
    }
}

pub struct Model {
    pub egui: Egui,
    pub settings: Settings,
    pub line: Line,
    pub ellipse: Ellipse,
    pub rect: Rectangle,
    pub rect_custom: Rectangle_Custom,
    pub elements: Vec<Elements>,
    pub tool: Tool,
    pub drawing: bool,
}

impl Model {
    pub fn new(
        egui: Egui,
        settings: Settings,
        line: Line,
        ellipse: Ellipse,
        rect: Rectangle,
        rect_custom: Rectangle_Custom,
        elements: Vec<Elements>,
        tool: Tool,
        drawing: bool,
    ) -> Self {
        Model {
            egui,
            settings,
            line,
            ellipse,
            rect,
            rect_custom,
            elements,
            tool,
            drawing,
        }
    }

    pub fn display(&self, draw: &Draw, _app: &App) {
        match self.tool {
            Tool::Pencil => {
                draw.polyline()
                    .start_cap_round()
                    .caps_round()
                    .end_cap_round()
                    .join_round()
                    .stroke_weight(self.get_line().get_weight())
                    .points_colored(self.get_line().get_pixels());
            }
            Tool::Ellipse => {
                draw.ellipse()
                    .xy(self.ellipse.get_center())
                    .color(self.get_settings().get_color())
                    .radius(self.ellipse.get_radius());
            }
            Tool::Rect => {
                draw.rect()
                    .xy(self.rect.get_center())
                    .width(self.rect.get_wh().x)
                    .height(self.rect.get_wh().y)
                    .color(self.rect.get_color());
            }
            Tool::Rect_Custom => {
                draw.rect()
                    .xy(self.rect_custom.get_center())
                    .width(self.rect_custom.get_wh().x)
                    .height(self.rect_custom.get_wh().y)
                    .color(self.rect_custom.get_color());
            }
            Tool::Rubber => {}
        }
    }

    pub fn get_drawing(&self) -> bool {
        self.drawing
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_tool(&mut self) -> &Tool {
        &self.tool
    }

    pub fn get_line(&self) -> &Line {
        &self.line
    }

    pub fn get_rect_line(&self) -> &Rectangle_Custom {
        &self.rect_custom
    }

    pub fn get_mut_line(&mut self) -> &mut Line {
        &mut self.line
    }

    pub fn get_mut_rect_line(&mut self) -> &mut Rectangle_Custom {
        &mut self.rect_custom
    }

    pub fn set_line(&mut self) {
        self.elements.push(Elements::L(Box::new(self.line.clone())))
    }

    pub fn set_rect_line(&mut self) {
        self.elements
            .push(Elements::L(Box::new(self.rect_custom.clone())))
    }

    pub fn set_drawing(&mut self, drawing: bool) {
        self.drawing = drawing
    }

    pub fn update(&mut self, app: &App) {
        let mouse_pos = pt2(app.mouse.x, app.mouse.y);
        if self.get_drawing() {
            match self.tool {
                Tool::Pencil => self
                    .line
                    .pixels
                    .push((mouse_pos, self.get_settings().get_color())),
                Tool::Rect => self
                    .rect
                    .set_wh((mouse_pos - self.rect.get_center()).abs() * 2.),
                Tool::Ellipse => self.ellipse.set_radius(
                    ((mouse_pos.x - self.ellipse.get_center().x).pow(2) as f32
                        - (mouse_pos.y - self.ellipse.get_center().y).pow(2) as f32)
                        .sqrt()
                        * 2.,
                ),
                Tool::Rect_Custom => {
                    self.rect_custom
                        .set_wh((mouse_pos - self.rect_custom.get_center()).abs() * 2.);
                }

                _ => {}
            }
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
