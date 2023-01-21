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
    fn get_clicked(&self) -> bool;
    fn set_clicked(&mut self, clicked: bool);
}

pub enum Elements {
    // pointer to the trait
    L(Box<dyn Pencil>),
    F(Box<dyn Forms>),
}

trait Elems {
    fn test_elem(&self) -> bool;
}

impl<T: Pencil + Forms> Elems for T {
    fn test_elem(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Tool {
    Pencil,
    Rect,
    Rect_Custom,
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
        draw.rect()
            .xy(self.center)
            .wh(self.wh)
            .color(self.color)
            .no_fill()
            .stroke_color(self.get_color());

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
    fn get_clicked(&self) -> bool {
        self.clicked
    }
    fn set_center(&mut self, center: Vec2) {
        self.center = center
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

    pub fn get_wh(&self) -> Vec2 {
        self.wh
    }

    pub fn set_wh(&mut self, center: Vec2) {
        self.wh = center
    }
}

#[derive(Clone, Default)]
pub struct Ellipse {
    center: Point2,
    color: Hsv,
    radius: f32,
    clicked: bool,
}

impl Forms for Ellipse {
    fn draw_elem(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.center)
            .radius(self.radius)
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
    fn get_clicked(&self) -> bool {
        self.clicked
    }
    fn set_center(&mut self, center: Vec2) {
        self.center = center
    }
    fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked
    }
}

impl Ellipse {
    pub fn new(center: Vec2, color: Hsv, radius: f32, clicked: bool) -> Self {
        Self {
            center,
            color,
            radius,
            clicked,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius
    }
}

#[derive(Clone)]
pub struct Line {
    pub pixels: Vec<(Point2, Hsv)>,
    weight: f32,
    color: Hsv,
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

#[derive(Clone, Default)]
pub struct Rectangle_Custom {
    pub center: Point2,
    pub wh: Point2,
    pub weight: f32,
    pub pixels: Vec<(Point2, Hsv)>,
    pub color: Hsv,
    pub clicked: bool,
}

impl Pencil for Rectangle_Custom {
    fn draw_elem(&self, draw: &Draw) {
        draw.polyline()
            .color(self.get_color())
            .start_cap_square()
            .caps_square()
            .end_cap_square()
            // .stroke_weight(self.get_weight())
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

impl Rectangle_Custom {
    pub fn new(center: Vec2, wh: Vec2, color: Hsv, clicked: bool, weight: f32) -> Self {
        Self {
            center,
            wh,
            weight,
            pixels: Vec::new(),
            color,
            clicked,
        }
    }

    pub fn rect_to_pixels(&mut self, rb_corner: Point2) {
        // need to put this in a new thread, now it is blocking

        let d = rb_corner - self.get_center();
        let x1 = rb_corner.x - d.x * 2.;
        let x2 = rb_corner.x;
        let y1 = rb_corner.y - d.y * 2.;
        let y2 = rb_corner.y;

        let lt = pt2(x1, y1);
        let lb = pt2(x1, y2);
        let rt = pt2(x2, y1);

        // create new lines
        let height: i32 = ((lb.x - lt.x).pow(2) as f32 + (lb.y - lt.y).pow(2) as f32)
            .sqrt()
            .floor() as i32;
        let width: i32 = ((rt.x - lt.x).pow(2) as f32 + (rt.y - lt.y).pow(2) as f32)
            .sqrt()
            .floor() as i32;
        let mut height_counter = 0;
        while height_counter < height {
            let mut width_counter = 0;
            while width_counter < width {
                self.pixels.push((
                    (lt.x + width_counter as f32, lt.y - height_counter as f32).into(),
                    self.get_color(),
                ));
                width_counter += 1;
            }
            height_counter += 1;
        }
    }

    pub fn get_wh(&self) -> Vec2 {
        self.wh
    }

    pub fn set_wh(&mut self, center: Vec2) {
        self.wh = center
    }

    pub fn clear_line(&mut self) {
        self.pixels.clear()
    }
    pub fn get_line(&self) -> &Vec<(Point2, Hsv)> {
        &self.pixels
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }
    pub fn get_clicked(&self) -> bool {
        self.clicked
    }
    pub fn set_center(&mut self, center: Vec2) {
        self.center = center
    }

    pub fn set_clicked(&mut self, clicked: bool) {
        self.clicked = clicked
    }
}
