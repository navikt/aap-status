#![warn(clippy::all, rust_2018_idioms)]

pub use app::Application;

mod app;
pub mod github;
mod ui;

// pub fn vertical_scroll<R>(id: &str, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> ScrollAreaOutput<R>) -> ScrollAreaOutput<R> {
//     ScrollArea::vertical()
//         .id_source(hash(id))
//         .show(ui, |ui| content(ui))
// }

