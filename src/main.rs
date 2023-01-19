// use diffusion::text_2_image;
use nannou::{color, prelude::*};
use nannou_egui::{egui, Egui};
mod diffusion;
mod helpers;
mod model;
use helpers::open_shapes;
use model::{
    elements::{Elements, Ellipse, Forms, Line, Pencil, Rectangle, Rectangle_Custom, Tool},
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
        Rectangle_Custom::default(),
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
        ref mut rect_custom,
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
                    ui.add_space(5.);
                    ui.button("Custom Rect").clicked().then(|| {
                        if rect_custom.get_clicked() {
                            *tool = Tool::Pencil;
                            rect_custom.set_clicked(false);
                        } else {
                            *tool = Tool::Rect_Custom;
                            rect_custom.set_clicked(true);
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
    // text_2_image("mountain and flowers");
    // let texture = wgpu::Texture::from_path(app, "result.png").expect("load file error");
    // draw.texture(&texture);

    println!("elem length {}", model.elements.len());

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
                    Tool::Rect_Custom => {
                        model.rect_custom.set_center(pt2(app.mouse.x, app.mouse.y));
                        model
                            .rect_custom
                            .set_color(model.get_settings().get_color());
                    }
                    Tool::Ellipse => {
                        model.ellipse.set_center(pt2(app.mouse.x, app.mouse.y));
                        model.ellipse.set_color(model.get_settings().get_color())
                    }
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
                    Tool::Rect_Custom => {
                        // transfer from rect into pixels here and push into the line i.e. the pixels and then clear it
                        model
                            .rect_custom
                            .rect_to_pixels(pt2(app.mouse.x, app.mouse.y));
                        model.set_rect_line();
                        model.get_mut_rect_line().clear_line();
                        app.set_loop_mode(LoopMode::RefreshSync)
                    }
                    Tool::Rubber => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
}
