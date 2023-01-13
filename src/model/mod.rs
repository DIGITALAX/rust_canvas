use nannou::{draw::primitive::Ellipse, prelude::*};
use nannou_egui::Egui;

// tool mapping
// 0 -> drawing, 1 -> ellipse, 2 -> square

pub struct Settings {
    pub color: Hsv,
    pub weight: f32,
    pub shapes: bool,
    pub tool: i32,
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
    pub pixel_vec: PixelVec,
    pub ellipse: Ellipse,
    pub global_vec: Vec<PixelVec>,
    pub tools_vec: Vec<i32>,
    pub drawing: bool,
}

impl Model {
    pub fn new(
        egui: Egui,
        settings: Settings,
        pixel_vec: PixelVec,
        ellipse: Ellipse,
        global_vec: Vec<PixelVec>,
        tools_vec: Vec<i32>,
        drawing: bool,
    ) -> Self {
        Model {
            egui,
            settings,
            pixel_vec,
            ellipse,
            global_vec,
            tools_vec,
            drawing,
        }
    }

    pub fn display(&self, draw: &Draw) {
        for (index, elem_type) in self.global_vec.iter().enumerate() {
            if self.tools_vec[index] == 0 {
                free_draw(self, draw)
            } else if self.tools_vec[index] == 1 {
                println!("now")
            }
        }
    }
}

pub fn free_draw(model: &Model, draw: &Draw) {
    if !model.pixel_vec.vec_pix.is_empty() {
        draw.polyline()
            .weight(model.pixel_vec.weight[model.pixel_vec.weight.len() - 1])
            .points_colored(model.pixel_vec.vec_pix.clone());
    }

    for (index, elem) in model.global_vec.iter().enumerate() {
        draw.polyline()
            .weight(elem.weight[index])
            .points_colored(elem.vec_pix.clone());
    }
}
