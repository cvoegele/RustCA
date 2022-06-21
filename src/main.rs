// use ndarray::{Dim, Ix3};
// use crate::cellular_automata::CellularAutomata;
//
// mod cellular_automata;
//
// fn update_conway(ca: &CellularAutomata, x: i32, index: (usize, usize, usize)) -> i32 {
//     //let neighbours = ca.get_neighbours(index);
//     if x == 0 {
//         return 2;
//     }
//     return 0;
// }
//
// fn main() {
//     let mut machine = CellularAutomata::new(100);
//     machine.set_value(19, (10, 10, 10));
//
//     machine.update(&update_conway);
//     let nn = machine.get_neighbours((10, 10, 10));
//     print!("{}", nn)
// }

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
