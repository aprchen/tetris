use bevy::math::{Vec2, Vec3};
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
pub const DEFAULT_COLOR: Color = Color::rgb(0.2, 0.2, 1.0);
const PRE_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);


pub fn get_color(shape: i32) -> Color {
    match shape {
        SHAPE_T => Color::PURPLE,
        SHAPE_O => Color::GOLD,
        SHAPE_I => Color::VIOLET,
        SHAPE_J => Color::YELLOW,
        SHAPE_L => Color::PINK,
        SHAPE_S => Color::TOMATO,
        SHAPE_Z => Color::AQUAMARINE,
        SHAPE_PRE => PRE_COLOR,
        _ => DEFAULT_COLOR
    }
}

pub fn shape_z_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    if direction == UP || direction == DOWN {
        if (sy == y) && (sx == x || sx == x - 1.) {
            return true;
        }
        if (sy == y - 1.) && (sx == x || sx == x + 1.) {
            return true;
        }
    }
    if direction == RIGHT || direction == LEFT {
        if (sx == x) && (sy == y + 1. || sy == y) {
            return true;
        }
        if (sx == x - 1.) && (sy == y - 1. || sy == y) {
            return true;
        }
    }
    false
}

pub fn shape_s_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    if direction == UP || direction == DOWN {
        if (sy == y) && (sx == x || sx == x - 1.) {
            return true;
        }
        if (sy == y + 1.) && (sx == x || sx == x + 1.) {
            return true;
        }
    }
    if direction == RIGHT || direction == LEFT {
        if (sx == x) && (sy == y + 1. || sy == y) {
            return true;
        }
        if (sx == x + 1.) && (sy == y - 1. || sy == y) {
            return true;
        }
    }
    false
}

pub fn shape_l_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    if direction == UP {
        if (sy == y) && (sx == x - 1. || sx == x || sx == x - 2.) {
            return true;
        }
        if sy == y + 1. && (sx == x) {
            return true;
        }
    }
    if direction == RIGHT {
        if (sy == y) && (sx == x + 1. || sx == x) {
            return true;
        }
        if (sx == x) && (sy == y + 1. || sy == y + 2.) {
            return true;
        }
    }
    if direction == DOWN {
        if (sy == y) && (sx == x + 1. || sx == x || sx == x + 2.) {
            return true;
        }
        if sy == y - 1. && sx == x {
            return true;
        }
    }
    if direction == LEFT {
        if (sy == y) && (sx == x - 1. || sx == x) {
            return true;
        }
        if (sx == x) && (sy == y - 1. || sy == y - 2.) {
            return true;
        }
    }
    false
}

pub fn shape_j_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    if direction == UP {
        if (sy == y) && (sx == x + 1. || sx == x || sx == x + 2.) {
            return true;
        }
        if sy == y + 1. && sx == x {
            return true;
        }
    }
    if direction == RIGHT {
        if (sy == y) && (sx == x + 1. || sx == x) {
            return true;
        }
        if (sx == x) && (sy == y - 1. || sy == y - 2.) {
            return true;
        }
    }
    if direction == DOWN {
        if (sy == y) && (sx == x - 1. || sx == x || sx == x - 2.) {
            return true;
        }
        if sy == y - 1. && sx == x {
            return true;
        }
    }
    if direction == LEFT {
        if (sy == y) && (sx == x - 1. || sx == x) {
            return true;
        }
        if (sx == x) && (sy == y + 1. || sy == y + 2.) {
            return true;
        }
    }
    false
}


pub fn shape_i_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    match direction {
        | UP | DOWN => {
            if (sy == y) && (sx == x - 1. || sx == x || sx == x + 1. || sx == x + 2.) {
                return true;
            }
            false
        }
        | LEFT | RIGHT => {
            if (sx == x) && (sy == y || sy == y - 1. || sy == y + 1. || sy == y + 2.) {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn shape_o_match(self_loc: Vec2, tar_loc: Vec3) -> bool {
    let x = tar_loc.x;
    let y = tar_loc.y;
    let sy = self_loc.y;
    let sx = self_loc.x;
    //同一行
    if (sy == y) && (sx == x || sx == x + 1.) {
        return true;
    }
    //上一行
    if (sy == y - 1.) && (sx == x || sx == x + 1.) {
        return true;
    }
    false
}

pub fn shape_t_match(self_loc: Vec2, tar_loc: Vec3, direction: i32) -> bool {
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