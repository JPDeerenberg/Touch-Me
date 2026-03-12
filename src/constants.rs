use macroquad::prelude::*;

// ---------- Physics Constants ----------
pub const PLAYER_RADIUS: f32 = 14.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const JUMP_FORCE: f32 = 550.0;
pub const GRAVITY: f32 = 1400.0;
pub const MAX_FALL_SPEED: f32 = 900.0;
pub const TAG_COOLDOWN: f32 = 1.0;
pub const ACCELERATION: f32 = 2200.0;
pub const FRICTION: f32 = 1600.0;

// ---------- Display Constants ----------
pub const REF_W: f32 = 1280.0;
pub const REF_H: f32 = 720.0;

// ---------- Player Configuration ----------
pub const PLAYER_COLORS: [Color; 4] = [
    Color::new(0.90, 0.22, 0.21, 1.0), // Red 500
    Color::new(0.13, 0.59, 0.95, 1.0), // Blue 500
    Color::new(0.30, 0.69, 0.31, 1.0), // Green 500
    Color::new(1.00, 0.76, 0.03, 1.0), // Amber 500
];

pub const PLAYER_NAMES: [&str; 4] = ["P1 (WASD)", "P2 (Arrows)", "P3 (IJKL)", "P4 (TFGH)"];
