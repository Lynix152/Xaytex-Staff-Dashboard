//! # Xaytex-dashboard

#[warn(dead_code)]

mod contributor;
mod data;
mod widgets;
mod infrastructure;
mod logging;

fn main() {
    contributor::contributor();

}
