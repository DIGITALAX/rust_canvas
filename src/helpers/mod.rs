use crate::model::{
    elements::{Ellipse, Forms, Rectangle, Tool},
    Settings,
};

pub fn open_shapes(settings: &mut Settings) {
    if settings.get_shapes() {
        settings.set_shapes(false);
    } else {
        settings.set_shapes(true);
    }
}