use nannou::{color, prelude::*};
use nannou_egui::{egui, Egui};
mod helpers;
mod model;
use helpers::open_shapes;
use model::{
    elements::{Elements, Ellipse, Forms, Line, Pencil, Rectangle, Tool},
    Model, Settings,
};

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
        Settings::new(hsv(10.0, 0.5, 1.0), 1., false),
        Line::new(),
        Ellipse::default(),
        Rectangle::default(),
        Vec::new(),
        Tool::Pencil,
        false,
    )
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.update(app);

    let Model {
        ref mut egui,
        ref mut settings,
        ref mut line,
        ref mut elements,
        ref mut ellipse,
        ref mut rect,
        ref mut tool,
        ref mut drawing,
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
                ui.add(shape_button)
                    .clicked()
                    .then(|| open_shapes(settings));
                ui.add_space(5.);
                if settings.get_shapes() {
                    ui.button("Ellipse").clicked().then(|| {
                        if ellipse.get_clicked() {
                            *tool = Tool::Pencil;
                            ellipse.set_clicked(false);
                        } else {
                            *tool = Tool::Ellipse;
                            ellipse.set_clicked(true);
                        }
                    });
                    ui.add_space(5.);
                    ui.button("Rectangle").clicked().then(|| {
                        if rect.get_clicked() {
                            *tool = Tool::Pencil;
                            rect.set_clicked(false);
                        } else {
                            *tool = Tool::Rect;
                            rect.set_clicked(true);
                        }
                    });
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

    for elem in model.elements.iter() {
        match elem {
            Elements::L(line) => line.draw_elem(&draw),
            Elements::F(form) => form.draw_elem(&draw),
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
        // MouseMoved(pos) => {
        //     if model.drawing {
        //         match model.get_settings().get_tool() {
        //             Tool::Pencil => {
        //                 model
        //                    .set_line();
        //                 app.set_loop_mode(LoopMode::Rate {
        //                     update_interval: std::time::Duration::from_millis(1),
        //                 })
        //             }

        //             Tool::Ellipse => {
        //                 // model.get_ell().clone().set_radius(
        //                 //     (((app.mouse.x - model.get_ell().clone().get_center().x).pow(2)
        //                 //         as f32)
        //                 //         - ((app.mouse.y - model.get_ell().clone().get_center().y).pow(2)
        //                 //             as f32))
        //                 //         .sqrt(),
        //                 // );
        //                 // model
        //                 //     .get_ell()
        //                 //     .clone()
        //                 //     .set_weight(model.get_settings().clone().get_weight());
        //                 // model
        //                 //     .get_ell()
        //                 //     .clone()
        //                 //     .set_color(model.get_settings().get_color());
        //             }
        //             Tool::Rect => {
        //                 // model.rect.set_color(model.get_settings().get_color());
        //                 // model.rect.set_weight(model.get_settings().get_weight());
        //                 // model
        //                 //     .rect
        //                 //     .set_width(((app.mouse.x) - (model.rect.get_start().x)).abs());
        //                 // model
        //                 //     .rect
        //                 //     .set_height(((app.mouse.y) - (model.rect.get_start().y)).abs());
        //             }
        //             Tool::Rubber => {}
        //         }
        //     }
        // }
        MousePressed(pos) => match pos {
            MouseButton::Left => {
                model.set_drawing(true);
                match model.get_tool() {
                    Tool::Pencil => {
                        model.line.set_color(model.get_settings().get_color());
                        model.line.set_weight(model.get_settings().get_weight());
                    }
                    Tool::Rect => {
                        model.rect.set_center(pt2(app.mouse.x, app.mouse.y));
                        model.rect.set_color(model.get_settings().get_color())
                    }
                    Tool::Ellipse => model.ellipse.set_center(pt2(app.mouse.x, app.mouse.y)),
                    _ => {}
                }
            }

            _ => {}
        },
        MouseReleased(pos) => match pos {
            MouseButton::Left => {
                model.set_drawing(false);
                match model.get_tool() {
                    Tool::Pencil => {
                        model.set_line();
                        model.get_mut_line().clear_line();
                        app.set_loop_mode(LoopMode::RefreshSync)
                    }
                    Tool::Ellipse => model
                        .elements
                        .push(Elements::F(Box::new(model.ellipse.clone()))),
                    Tool::Rect => model
                        .elements
                        .push(Elements::F(Box::new(model.rect.clone()))),
                    Tool::Rubber => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
}
