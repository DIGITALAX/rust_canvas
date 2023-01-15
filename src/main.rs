use nannou::{color, prelude::*};
use nannou_egui::{egui, Egui};
mod helpers;
mod model;
use helpers::{open_shapes, set_shape};
use model::{Elements, Ellipse, Model, PixelVec, Rectangle, Settings, Tools};

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
            tool: Tools::Pencil,
        },
        PixelVec::new(),
        Ellipse::new(),
        Rectangle::new(),
        Vec::new(),
        Vec::new(),
        false,
        Vec::new(),
    )
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut pencil,
        ref mut elements,
        ref mut ellipse,
        ref mut rect,
        ref mut tools_type,
        ref mut drawing,
        ref mut weight,
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings")
        .default_size(egui::vec2(0.0, 600.0))
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
                ui.add(egui::Slider::new(&mut settings.weight, 1.0..=100.0));
                ui.add_space(10.);
                ui.separator();
                let shape_button = egui::Button::new("Add Shape").fill(egui::Color32::BLACK);
                let clicked = ui.add(shape_button).clicked();

                if clicked {
                    open_shapes(&mut settings.shapes);
                }
                ui.add_space(5.);
                if settings.shapes {
                    let ell = ui.button("Ellipse").clicked();
                    ui.add_space(5.);
                    let rectangle = ui.button("Rectangle").clicked();

                    if rectangle || ell {
                        set_shape(ell, rectangle, &mut settings.tool, ellipse, rect);
                    }
                }
            })
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    for (index, elem) in model.elements.iter().enumerate() {
        match elem {
            Elements::Pencil(pencil) => {
                draw.polyline()
                    .weight(model.weight[index])
                    .start_cap_round()
                    .caps_round()
                    .end_cap_round()
                    .join_round()
                    .points_colored(pencil.vec_pix.clone());
            }
            Elements::Rect(rect) => {
                draw.rect()
                    .xy(rect.start)
                    .w_h(rect.width, rect.height)
                    .color(rect.color);
            }
            Elements::Ellipse(ellipse) => {
                draw.ellipse()
                    .xy(ellipse.center)
                    .color(ellipse.color)
                    .radius(ellipse.radius);
            }
            Elements::Rubber => {}
        }
    }
    model.display(&draw, &app);
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

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            if model.drawing {
                match model.settings.tool {
                    Tools::Pencil => {
                        model
                            .pencil
                            .vec_pix
                            .push((pt2(pos.x, pos.y), model.settings.color));
                    }
                    Tools::Ellipse => {
                        model.ellipse.radius = ((app.mouse.x - model.ellipse.center.x).pow(2)
                            as f32
                            - (app.mouse.y - model.ellipse.center.y).pow(2) as f32)
                            .sqrt();
                        model.ellipse.weight = model.settings.weight;
                        model.ellipse.color = model.settings.color;
                    }
                    Tools::Rect => {
                        model.rect.color = model.settings.color;
                        model.rect.weight = model.settings.weight;
                        model.rect.width = ((app.mouse.x) - (model.rect.start.x)).abs();
                        model.rect.height = ((app.mouse.y) - (model.rect.start.y)).abs();
                    }
                    Tools::Rubber => {}
                }
            }
        }
        MousePressed(pos) => match pos {
            MouseButton::Left => {
                model.tools_type.push(model.settings.tool.clone());
                model.drawing = true;
                model.weight.push(model.settings.weight);
                match model.settings.tool {
                    Tools::Pencil => {}
                    Tools::Ellipse => model.ellipse.center = pt2(app.mouse.x, app.mouse.y),
                    Tools::Rect => model.rect.start = pt2(app.mouse.x, app.mouse.y),
                    Tools::Rubber => {}
                }
            }

            _ => {}
        },
        MouseReleased(pos) => match pos {
            MouseButton::Left => {
                model.drawing = false;
                match model.settings.tool {
                    Tools::Pencil => {
                        model.elements.push(Elements::Pencil(model.pencil.clone()));
                        model.pencil.vec_pix.clear();
                    }
                    Tools::Ellipse => {
                        model
                            .elements
                            .push(Elements::Ellipse(model.ellipse.clone()));
                    }
                    Tools::Rect => {
                        model.elements.push(Elements::Rect(model.rect.clone()));
                    }
                    Tools::Rubber => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
}
