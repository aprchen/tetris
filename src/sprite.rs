use bevy::math::Vec2;

pub const CONVEX: i32 = 0;
pub const UP: i32 = 0;
pub const RIGHT: i32 = 1;
pub const DOWN: i32 = 2;
pub const LEFT: i32 = 3;

pub fn convex_up_match(self_loc: Vec2, tar_loc: Vec2, direction: i32) -> bool {
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