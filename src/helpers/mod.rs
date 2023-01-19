use crate::model::{
    elements::{Ellipse, Forms, Pencil, Rectangle, Rectangle_Custom, Tool},
    Settings,
};
use nannou::prelude::*;

pub fn open_shapes(settings: &mut Settings) {
    if settings.get_shapes() {
        settings.set_shapes(false);
    } else {
        settings.set_shapes(true);
    }
}
