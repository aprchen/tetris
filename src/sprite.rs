use bevy::math::Vec2;
use bevy::prelude::Color;

pub const SHAPE_BG: i32 = -1;
pub const SHAPE_PRE: i32 = 0;
pub const SHAPE_I: i32 = 1;
pub const SHAPE_J: i32 = 2;
pub const SHAPE_L: i32 = 3;
pub const SHAPE_O: i32 = 4;
pub const SHAPE_S: i32 = 5;
pub const SHAPE_T: i32 = 6;
pub const SHAPE_Z: i32 = 7;

pub const UP: i32 = 0;
pub const RIGHT: i32 = 1;
pub const DOWN: i32 = 2;
pub const LEFT: i32 = 3;
pub const DEFAULT_COLOR: Color = Color::rgb(0.4, 0.4, 1.0);
const PRE_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);


pub fn get_color(shape: i32) -> Color {
    if shape == SHAPE_T {
        return Color::PURPLE;
    }
    if shape == SHAPE_PRE {
        return PRE_COLOR;
    }
    return DEFAULT_COLOR;
}

pub fn shape_t_match(self_loc: Vec2, tar_loc: Vec2, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    if direction == UP {
        if (y == sy) && (sx - 1. == x || sx == x || sx + 1. == x) {
            return true;
        }
        if y == sy + 1. && sx == x {
            return true;
        }
    }
    if direction == DOWN {
        if (y == sy) && (sx - 1. == x || sx == x || sx + 1. == x) {
            return true;
        }
        if y == sy - 1. && sx == x {
            return true;
        }
    }
    if direction == RIGHT {
        if (x == sx) && (y == sy + 1. || y == sy || y == sy - 1.) {
            return true;
        }
        if sx == x - 1. && y == sy {
            return true;
        }
    }
    if direction == LEFT {
        if (x == sx) && (y == sy + 1. || y == sy || y == sy - 1.) {
            return true;
        }
        if sx == x + 1. && y == sy {
            return true;
        }
    }
    false
}