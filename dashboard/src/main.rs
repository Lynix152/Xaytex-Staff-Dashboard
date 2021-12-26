//! # Xaytex-dashboard

mod contributor;
mod data;
mod widgets;


fn main() {
    contributor::contributor();
    widgets::gui::widget();
}
