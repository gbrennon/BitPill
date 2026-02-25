#![allow(dead_code)]

mod application;
mod domain;
mod infrastructure;
mod presentation;

fn main() {
    presentation::run();
}
