use nannou_egui::egui::Button;

use crate::model::{Ellipse, Rectangle, Tools};

pub fn open_shapes(shapes: &mut bool) {
    if *shapes {
        *shapes = false;
    } else {
        *shapes = true;
    }
}

pub fn set_shape(
    ell_clicked: bool,
    rect_clicked: bool,
    tool: &mut Tools,
    ellipse: &mut Ellipse,
    rect: &mut Rectangle,
) {
    if ell_clicked {
        if ellipse.clicked {
            *tool = Tools::Pencil;
            ellipse.clicked = false;
        } else {
            *tool = Tools::Ellipse;
            ellipse.clicked = true;
        }
    } else if rect_clicked {
        if rect.clicked {
            *tool = Tools::Pencil;
            rect.clicked = false;
        } else {
            *tool = Tools::Rect;
            rect.clicked = true;
        }
    }
}
