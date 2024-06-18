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

pub const WATER_COLOR: GColor = &[
    Color::rgb(0.0, 0.624, 0.784),
    Color::rgb(0.0, 0.671, 0.843),
    Color::rgb(0.0, 0.71, 0.894),
    Color::rgb(0.122, 0.757, 0.918),
    Color::rgb(0.224, 0.816, 0.969),
];

pub const STONE_COLOR: GColor = &[
    Color::rgb(0.313, 0.313, 0.313),
    Color::rgb(0.345, 0.345, 0.345),
    Color::rgb(0.392, 0.392, 0.392),
    Color::rgb(0.254, 0.254, 0.254),
    Color::rgb(0.196, 0.196, 0.196),
];
