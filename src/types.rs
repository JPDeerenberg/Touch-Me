use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

// ---------- Controls ----------
#[allow(dead_code)]
pub struct Controls {
    pub up: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

// ---------- Level Data Serialization Format ----------
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PlatformData {
    pub center_x: f32,
    pub center_y: f32,
    pub size_x: f32,
    pub size_y: f32,
    pub angle: f32,
    pub is_jump_pad: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelData {
    pub platforms: Vec<PlatformData>,
    pub spawns: Vec<[f32; 2]>,
}

impl From<Platform> for PlatformData {
    fn from(p: Platform) -> Self {
        PlatformData {
            center_x: p.center.x,
            center_y: p.center.y,
            size_x: p.size.x,
            size_y: p.size.y,
            angle: p.angle,
            is_jump_pad: p.is_jump_pad,
        }
    }
}

impl From<PlatformData> for Platform {
    fn from(d: PlatformData) -> Self {
        Platform {
            center: vec2(d.center_x, d.center_y),
            size: vec2(d.size_x, d.size_y),
            angle: d.angle,
            is_jump_pad: d.is_jump_pad,
            iced_timer: 0.0,
        }
    }
}

// ---------- Platform ----------
#[derive(Clone, Copy)]
pub struct Platform {
    pub center: Vec2,
    pub size: Vec2,
    pub angle: f32, // Radians
    pub is_jump_pad: bool, // High-velocity kinetic repulsion surface
    pub iced_timer: f32,   // AOE Slipping Effect
}

impl Platform {
    pub fn new_from_endpoints(start: Vec2, end: Vec2, thickness: f32, is_jump_pad: bool) -> Self {
        let center = (start + end) / 2.0;
        let length = (end - start).length().max(10.0);
        let angle = (end.y - start.y).atan2(end.x - start.x);
        Self { center, size: vec2(length, thickness), angle, is_jump_pad, iced_timer: 0.0 }
    }

    pub fn endpoints(&self) -> (Vec2, Vec2) {
        let c = self.angle.cos();
        let s = self.angle.sin();
        let hw = self.size.x / 2.0;
        let dx = hw * c;
        let dy = hw * s;
        (self.center - vec2(dx, dy), self.center + vec2(dx, dy))
    }
}

// ---------- Scenery ----------
#[derive(Clone, Copy)]
pub enum SceneryKind {
    Tree,
    Bush,
    Cactus,
    CyberPillar,
    Coral,
    IceSpike,
}

#[derive(Clone, Copy)]
pub struct Scenery {
    pub pos: Vec2,
    pub kind: SceneryKind,
    pub fg: bool,
}

// ---------- Powerups ----------
#[derive(Clone, Copy, PartialEq)]
pub enum PowerupKind {
    Turbo,      // Buff: Speed boost
    DoubleJump, // Buff: Extra jump
    Ghost,      // Buff: Untaggable
    IcePatch,   // Debuff: Slippery
    Shield,     // Defensive: Absorb tag
    PortalBlue, // Utility: Linked portal A
    PortalGreen,// Utility: Linked portal B
}

#[derive(Clone, Copy)]
pub struct Powerup {
    pub pos: Vec2,
    pub kind: PowerupKind,
    pub active: bool,
    pub respawn_timer: f32,
    pub spawn_in_timer: f32, // For "pop in" animation
}

// ---------- Particles ----------
#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
}

// ---------- Theme ----------
#[allow(dead_code)]
pub struct MapTheme {
    pub name: &'static str,
    pub bg_top: Color,
    pub bg_bot: Color,
    pub plat_base: Color,
    pub plat_top: Color,
    pub plat_shadow: Color,
    pub text_col: Color,
    pub particle_col: Color,
}

// ---------- Player ----------
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub is_it: bool,
    pub on_ground: bool,
    pub controls: Controls,
    pub tag_timer: f32,
    pub coyote_timer: f32,
    pub render_angle: f32,
    pub radius: f32,
    pub trail: Vec<Vec2>,
    
    // Powerup states
    pub turbo_timer: f32,
    pub ghost_timer: f32,
    pub shield_active: bool,
    pub shield_timer: f32,
    pub extra_jumps: u32,
    pub slip_timer: f32,
    pub teleport_timer: f32,
}

impl Player {
    pub fn new(pos: Vec2, color: Color, controls: Controls, radius: f32) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            color,
            is_it: false,
            on_ground: false,
            controls,
            tag_timer: 0.0,
            coyote_timer: 0.0,
            render_angle: 0.0,
            radius,
            trail: Vec::new(),
            turbo_timer: 0.0,
            ghost_timer: 0.0,
            shield_active: false,
            shield_timer: 0.0,
            extra_jumps: 0,
            slip_timer: 0.0,
            teleport_timer: 0.0,
        }
    }
}

// ---------- Game States ----------
pub enum DragMode {
    None,
    Creating { start: Vec2 },
    GrabStart { idx: usize },
    GrabEnd { idx: usize },
    PlacingSpawn { id: usize },
}

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver { loser: usize },
    Editor,
}
