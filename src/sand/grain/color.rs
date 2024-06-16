use bevy::prelude::*;

type GColor = &'static [Color];

pub const DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);

pub const SAND_COLOR: GColor = &[
    Color::rgb(0.965, 0.843, 0.69),
    Color::rgb(0.949, 0.824, 0.663),
    Color::rgb(0.925, 0.8, 0.635),
    Color::rgb(0.906, 0.769, 0.588),
    Color::rgb(0.882, 0.749, 0.573),
];
