use nannou::prelude::*;
use nannou_egui::Egui;

pub struct Settings {
    pub color: Hsv,
    pub weight: f32,
    pub shapes: bool,
}

#[derive(Clone)]
pub struct PixelVec {
    pub vec_pix: Vec<(Point2, Hsv)>,
    pub drawing: bool,
    pub weight: Vec<f32>,
}

impl PixelVec {
    pub fn new() -> Self {
        Self {
            vec_pix: Vec::new(),
            drawing: false,
            weight: Vec::new(),
        }
    }
}

pub struct Model {
    pub egui: Egui,
    pub settings: Settings,
    pub pixel_vec: PixelVec,
    pub global_vec: Vec<PixelVec>,
}

impl Model {
    pub fn new(
        egui: Egui,
        settings: Settings,
        pixel_vec: PixelVec,
        global_vec: Vec<PixelVec>,
    ) -> Self {
        Model {
            egui,
            settings,
            pixel_vec,
            global_vec,
        }
    }

    pub fn display(&self, draw: &Draw) {
        if !self.pixel_vec.vec_pix.is_empty() {
            draw.polyline()
                .weight(self.pixel_vec.weight[self.pixel_vec.weight.len() - 1])
                .points_colored(self.pixel_vec.vec_pix.clone());
        }

        for (index, elem) in self.global_vec.iter().enumerate() {
            draw.polyline()
                .weight(elem.weight[index])
                .points_colored(elem.vec_pix.clone());
        }
    }
}
