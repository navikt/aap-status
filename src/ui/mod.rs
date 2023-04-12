use egui::{ScrollArea, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

pub mod pull_requests;
pub mod workflows;
pub mod deployments;
pub mod panels;
pub mod repositories;

trait Table {
    fn create<'b>(ui: &'b mut Ui, columns: Vec<&'b str>) -> egui_extras::Table<'b>;
}

trait Scroll {
    fn horizontal<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R);
    fn vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R);
}

struct Scrollbar;

pub struct FixedField;

impl FixedField {
    fn minimum_width(width: f32, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(width))
            .vertical(|mut strip| strip.cell(|ui| add_contents(ui)));
    }

    fn remaining_width(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        StripBuilder::new(ui)
            .size(Size::remainder())
            .vertical(|mut strip| strip.cell(|ui| add_contents(ui)));
    }

    fn height(height: f32, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        StripBuilder::new(ui)
            .size(Size::exact(height))
            .horizontal(|mut strip| strip.cell(|ui| add_contents(ui)));
    }
}

impl Table for TableBuilder<'_> {
    fn create<'b>(ui: &'b mut Ui, columns: Vec<&'b str>) -> egui_extras::Table<'b> {
        let mut table_builder = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0);

        for _ in 0..columns.len() {
            table_builder = table_builder.column(Column::auto())
        }

        table_builder
            .header(20.0, |mut header| {
                columns.into_iter().for_each(|column| {
                    header.col(|ui| { ui.strong(column); });
                })
            })
    }
}

impl Scroll for Scrollbar {
    fn horizontal<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
        ScrollArea::horizontal().show(ui, |ui| add_contents(ui));
    }

    fn vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
        ScrollArea::vertical().show(ui, |ui| add_contents(ui));
    }
}
