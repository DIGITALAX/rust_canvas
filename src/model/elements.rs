use nannou::prelude::*;

pub trait Pencil {
    fn draw_elem(&self, draw: &Draw);
    fn trait_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut (Point2, Hsv)> + '_>;
    fn set_color(&mut self, color: Hsv);
    fn get_color(&self) -> Hsv;
    fn set_weight(&mut self, weight: f32);
    fn get_weight(&self) -> f32;
    fn get_pixels(&self) -> Vec<(Point2, Hsv)>;
}

pub trait Forms {
    fn draw_elem(&self, draw: &Draw);
    fn set_color(&mut self, color: Hsv);
    fn get_color(&self) -> Hsv;
    fn set_center(&mut self, center: Vec2);
    fn get_center(&self) -> Vec2;
    fn set_wh(&mut self, wh: Vec2);
    fn get_wh(&self) -> Vec2;
    fn get_clicked(&self) -> bool;
    fn set_clicked(&mut self, clicked: bool);
}

pub enum Elements {
    L(Box<dyn Pencil>),
    F(Box<dyn Forms>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Tool {
    Pencil,
    Rect,
    Ellipse,
    Rubber,
}

#[derive(Clone, Default)]
pub struct Rectangle {
    center: Point2,
    wh: Point2,
    color: Hsv,
    clicked: bool,
}

impl Forms for Rectangle {
    fn draw_elem(&self, draw: &Draw) {
        draw.rect().xy(self.center).wh(self.wh).color(self.color);
    }
    fn set_color(&mut self, color: Hsv) {
        self.color = color;
    }
    fn get_color(&self) -> Hsv {
        self.color
    }
    fn get_center(&self) -> Vec2 {
        self.center
    }
    fn get_wh(&self) -> Vec2 {
        self.wh
    }
    fn get_clicked(&self) -> bool {
        self.clicked
    }
    fn set_center(&mut self, center: Vec2) {
        self.center = center
    }
    fn set_wh(&mut self, center: Vec2) {
        self.wh = center
    }
    fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked
    }
}

impl Rectangle {
    pub fn new(center: Vec2, wh: Vec2, color: Hsv, clicked: bool) -> Self {
        Self {
            center,
            wh,
            color,
            clicked,
        }
    }
}

#[derive(Clone, Default)]
pub struct Ellipse {
    center: Point2,
    color: Hsv,
    wh: Point2,
    clicked: bool,
}

impl Forms for Ellipse {
    fn draw_elem(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.center)
            .radius(self.wh.x)
            .color(self.color);
    }
    fn set_color(&mut self, color: Hsv) {
        self.color = color;
    }
    fn get_color(&self) -> Hsv {
        self.color
    }
    fn get_center(&self) -> Vec2 {
        self.center
    }
    fn get_wh(&self) -> Vec2 {
        self.wh
    }
    fn get_clicked(&self) -> bool {
        self.clicked
    }
    fn set_center(&mut self, center: Vec2) {
        self.center = center
    }
    fn set_wh(&mut self, center: Vec2) {
        self.wh = center
    }
    fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked
    }
}

impl Ellipse {
    pub fn new(center: Vec2, color: Hsv, wh: Vec2, clicked: bool) -> Self {
        Self {
            center,
            color,
            wh,
            clicked,
        }
    }
}

#[derive(Clone)]
pub struct Line {
    pub pixels: Vec<(Point2, Hsv)>,
    pub weight: f32,
    pub color: Hsv,
}

impl Pencil for Line {
    fn draw_elem(&self, draw: &Draw) {
        draw.polyline()
            .weight(self.get_weight())
            .color(self.get_color())
            .start_cap_round()
            .caps_round()
            .end_cap_round()
            .join_round()
            .points_colored(self.get_line().clone());
    }
    fn trait_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut (Point2, Hsv)> + '_> {
        Box::new(self.pixels.iter_mut())
    }
    fn set_color(&mut self, color: Hsv) {
        self.color = color;
    }
    fn get_color(&self) -> Hsv {
        self.color
    }
    fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }
    fn get_pixels(&self) -> Vec<(Point2, Hsv)> {
        self.pixels.clone()
    }
}

impl Line {
    pub fn new() -> Self {
        Self {
            pixels: Vec::new(),
            weight: 0.,
            color: hsv(10.0, 0.5, 1.0),
        }
    }
    pub fn new_param(pixels: Vec<(Point2, Hsv)>, weight: f32, color: Hsv) -> Self {
        Self {
            pixels,
            weight,
            color,
        }
    }
    pub fn clear_line(&mut self) {
        self.pixels.clear()
    }
    pub fn get_line(&self) -> &Vec<(Point2, Hsv)> {
        &self.pixels
    }
}
