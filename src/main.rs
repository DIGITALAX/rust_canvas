use nannou::{color, prelude::*};
use nannou_egui::{egui, Egui};
mod model;
use model::{Model, PixelVec, Settings};

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title("Canvas")
        .min_size(900, 700)
        .resizable(true)
        .size(1350, 850)
        .event(event)
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    Model::new(
        Egui::from_window(&window),
        Settings {
            color: hsv(10.0, 0.5, 1.0),
            weight: 1.,
            shapes: false,
        },
        PixelVec::new(),
        Vec::new(),
    )
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut pixel_vec,
        ref mut global_vec,
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings")
        .default_size(egui::vec2(0.0, 200.0))
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10., -10.))
        .resizable(false)
        .show(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.separator();
                ui.label("Choose a Color");
                edit_hsv(ui, &mut settings.color);
                ui.add_space(10.);
                ui.separator();
                ui.label("Brush Size");
                ui.add(egui::Slider::new(&mut settings.weight, 0.0..=100.0));
                ui.add_space(10.);
                ui.vertical(|ui| {
                    ui.separator();
                    let clicked = ui.button("Add Shape").clicked();
                    if clicked {
                        open_shapes(&mut settings.shapes);
                    }
                    ui.add_space(5.);
                    if settings.shapes {
                        ui.horizontal(|ui| {
                            ui.button("Ellipse");
                            ui.add_space(5.);
                            ui.button("Square");
                        });
                    }
                })
            })
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(BLACK);
    model.display(&draw);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv = egui::color::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
        color.saturation,
        color.value,
        1.0,
    );

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        Moved(_) => {}
        KeyPressed(_) => {}
        KeyReleased(_) => {}
        ReceivedCharacter(_) => {}
        MouseMoved(pos) => {
            if model.pixel_vec.drawing {
                model
                    .pixel_vec
                    .vec_pix
                    .push((pt2(pos.x, pos.y), model.settings.color));
            }
        }
        MousePressed(pos) => match pos {
            MouseButton::Left => {
                model.pixel_vec.drawing = true;
                model.pixel_vec.weight.push(model.settings.weight);
            }
            _ => {}
        },
        MouseReleased(pos) => match pos {
            MouseButton::Left => {
                model.pixel_vec.drawing = false;
                model.global_vec.push(model.pixel_vec.clone());
                model.pixel_vec.vec_pix.clear();
            }
            _ => {}
        },
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_, _) => {}
        Resized(_) => {}
        HoveredFile(_) => {}
        DroppedFile(_) => {}
        HoveredFileCancelled => {}
        Touch(_) => {}
        TouchPressure(_) => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn open_shapes(shapes: &mut bool) {
    if *shapes {
        *shapes = false
    } else {
        *shapes = true
    }
}
