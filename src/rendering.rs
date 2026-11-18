use crate::constants::*;
use crate::types::{MapTheme, Particle, Platform, Scenery, SceneryKind};
use macroquad::prelude::*;

// ---------- Visual Themes & Effects ----------

pub const MAP_THEMES: [MapTheme; 5] = [
    MapTheme {
        // 0: Forest
        name: "Forest",
        bg_top: Color {
            r: 0.6,
            g: 0.8,
            b: 0.9,
            a: 1.0,
        },
        bg_bot: Color {
            r: 0.8,
            g: 0.9,
            b: 0.8,
            a: 1.0,
        },
        plat_base: Color {
            r: 0.35,
            g: 0.25,
            b: 0.15,
            a: 1.0,
        },
        plat_top: Color {
            r: 0.3,
            g: 0.7,
            b: 0.3,
            a: 1.0,
        },
        plat_shadow: Color {
            r: 0.1,
            g: 0.2,
            b: 0.1,
            a: 0.4,
        },
        text_col: Color {
            r: 0.1,
            g: 0.3,
            b: 0.1,
            a: 1.0,
        },
        particle_col: YELLOW,
    },
    MapTheme {
        // 1: Desert
        name: "Desert",
        bg_top: Color {
            r: 1.0,
            g: 0.6,
            b: 0.3,
            a: 1.0,
        },
        bg_bot: Color {
            r: 1.0,
            g: 0.8,
            b: 0.5,
            a: 1.0,
        },
        plat_base: Color {
            r: 0.75,
            g: 0.45,
            b: 0.25,
            a: 1.0,
        },
        plat_top: Color {
            r: 0.95,
            g: 0.75,
            b: 0.45,
            a: 1.0,
        },
        plat_shadow: Color {
            r: 0.4,
            g: 0.2,
            b: 0.1,
            a: 0.4,
        },
        text_col: Color {
            r: 0.4,
            g: 0.2,
            b: 0.1,
            a: 1.0,
        },
        particle_col: WHITE,
    },
    MapTheme {
        // 2: Cyber
        name: "Cyber",
        bg_top: Color {
            r: 0.05,
            g: 0.0,
            b: 0.1,
            a: 1.0,
        },
        bg_bot: Color {
            r: 0.2,
            g: 0.0,
            b: 0.3,
            a: 1.0,
        },
        plat_base: Color {
            r: 0.1,
            g: 0.1,
            b: 0.15,
            a: 1.0,
        },
        plat_top: Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        plat_shadow: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.6,
        },
        text_col: Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        particle_col: Color::new(0.0, 1.0, 1.0, 1.0),
    },
    MapTheme {
        // 3: Ocean
        name: "Ocean",
        bg_top: Color {
            r: 0.0,
            g: 0.2,
            b: 0.4,
            a: 1.0,
        },
        bg_bot: Color {
            r: 0.0,
            g: 0.4,
            b: 0.6,
            a: 1.0,
        },
        plat_base: Color {
            r: 0.0,
            g: 0.15,
            b: 0.25,
            a: 1.0,
        },
        plat_top: Color {
            r: 0.9,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        },
        plat_shadow: Color {
            r: 0.0,
            g: 0.1,
            b: 0.2,
            a: 0.5,
        },
        text_col: Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.0,
        },
        particle_col: WHITE,
    },
    MapTheme {
        // 4: Ice
        name: "Ice",
        bg_top: Color {
            r: 0.5,
            g: 0.6,
            b: 0.7,
            a: 1.0,
        },
        bg_bot: Color {
            r: 0.8,
            g: 0.9,
            b: 1.0,
            a: 1.0,
        },
        plat_base: Color {
            r: 0.2,
            g: 0.4,
            b: 0.6,
            a: 1.0,
        },
        plat_top: Color {
            r: 0.95,
            g: 0.98,
            b: 1.0,
            a: 1.0,
        },
        plat_shadow: Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 0.4,
        },
        text_col: Color {
            r: 0.1,
            g: 0.3,
            b: 0.5,
            a: 1.0,
        },
        particle_col: WHITE,
    },
];

// ---------- Drawing Utilities ----------

pub fn draw_gradient(top: Color, bot: Color) {
    let steps = 12.0;
    let h = REF_H / steps;
    for i in 0..steps as i32 {
        let t = i as f32 / steps;
        let c = Color::new(
            top.r * (1.0 - t) + bot.r * t,
            top.g * (1.0 - t) + bot.g * t,
            top.b * (1.0 - t) + bot.b * t,
            1.0,
        );
        draw_rectangle(0.0, i as f32 * h, REF_W, h + 1.0, c);
    }
}

pub fn draw_rotated_rect(cx: f32, cy: f32, w: f32, h: f32, angle: f32, color: Color) {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let c = angle.cos();
    let s = angle.sin();
    let p1 = vec2(cx - hw * c + hh * s, cy - hw * s - hh * c);
    let p2 = vec2(cx + hw * c + hh * s, cy + hw * s - hh * c);
    let p3 = vec2(cx + hw * c - hh * s, cy + hw * s + hh * c);
    let p4 = vec2(cx - hw * c - hh * s, cy - hw * s + hh * c);

    draw_triangle(p1, p2, p3, color);
    draw_triangle(p1, p3, p4, color);
}

pub fn draw_pixel_rect(
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    angle: f32,
    color: Color,
    outline: Option<Color>,
) {
    if let Some(oc) = outline {
        draw_rotated_rect(cx, cy, w + 2.0, h + 2.0, angle, oc);
    }
    draw_rotated_rect(cx, cy, w, h, angle, color);
}

pub fn draw_particles(idx: usize, time: f64, col: Color) {
    match idx {
        0 => {
            // Forest
            for i in 0..60 {
                let x = ((i as f64 * 137.0 + (time * 0.5 + i as f64).sin() * 50.0) % REF_W as f64)
                    .abs() as f32;
                let y =
                    (REF_H as f64 - (time * 30.0 + i as f64 * 73.0) % REF_H as f64).abs() as f32;
                let a = ((time * 3.0 + i as f64).sin() * 0.4 + 0.4) as f32;
                draw_rectangle(x, y, 4.0, 4.0, Color::new(col.r, col.g, col.b, a));
            }
        }
        1 => {
            // Desert
            for i in 0..50 {
                let x = ((time * 60.0 + i as f64 * 111.0) % REF_W as f64).abs() as f32;
                let y = ((i as f64 * 47.0) % REF_H as f64).abs() as f32;
                draw_rectangle(x, y, 12.0, 4.0, Color::new(1.0, 1.0, 1.0, 0.15));
            }
        }
        2 => {
            // Cyber
            for i in 0..40 {
                let x = ((i as f64 * 40.0) % REF_W as f64).abs() as f32;
                let y = (((time * 150.0 + i as f64 * 231.0) % (REF_H as f64 * 1.5))
                    - REF_H as f64 * 0.25) as f32;
                draw_rectangle(x, y, 4.0, 40.0, Color::new(col.r, col.g, col.b, 0.3));
            }
        }
        3 => {
            // Ocean
            for i in 0..70 {
                let x = ((i as f64 * 89.0 + (time * 2.0 + i as f64).sin() * 15.0) % REF_W as f64)
                    .abs() as f32;
                let y =
                    (REF_H as f64 - (time * 60.0 + i as f64 * 51.0) % REF_H as f64).abs() as f32;
                let r = (i % 4 + 2) as f32;
                draw_rectangle_lines(
                    x - r,
                    y - r,
                    r * 2.0,
                    r * 2.0,
                    2.0,
                    Color::new(1.0, 1.0, 1.0, 0.3),
                );
            }
        }
        4 => {
            // Ice
            for i in 0..100 {
                let x = ((time * 30.0 + i as f64 * 67.0 + (time + i as f64).sin() * 20.0)
                    % REF_W as f64)
                    .abs() as f32;
                let y = ((time * 80.0 + i as f64 * 43.0) % REF_H as f64).abs() as f32;
                draw_rectangle(x, y, 4.0, 4.0, Color::new(1.0, 1.0, 1.0, 0.6));
            }
        }
        _ => {}
    }
}

// ---------- Platform Rendering ----------

impl Platform {
    pub fn draw(&self, theme: &MapTheme) {
        let outline_col = Color::new(
            theme.plat_base.r * 0.4,
            theme.plat_base.g * 0.4,
            theme.plat_base.b * 0.4,
            1.0,
        );

        // Shadow
        draw_rotated_rect(
            self.center.x + 4.0,
            self.center.y + 6.0,
            self.size.x,
            self.size.y,
            self.angle,
            theme.plat_shadow,
        );

        // Base Body with Outline
        draw_pixel_rect(
            self.center.x,
            self.center.y,
            self.size.x,
            self.size.y,
            self.angle,
            theme.plat_base,
            Some(outline_col),
        );

        // Tile Pattern (Slanted Bricks)
        let s = self.angle.sin();
        let c = self.angle.cos();
        let brick_w = 40.0;
        let stripe_slant = 0.25; // Constant radians for a stylish tilt
        let stripe_h = self.size.y - 4.0; // Padding from top/bottom

        // Only draw stripes if platform is wide enough
        if self.size.x > 30.0 {
            let num_bricks = (self.size.x / brick_w).floor() as i32;
            for i in 0..num_bricks {
                let offset = (i as f32 * brick_w) - (self.size.x / 2.0) + (brick_w / 2.0);

                // Edge padding: skip stripes too close to left/right
                if offset.abs() > (self.size.x / 2.0 - 5.0) {
                    continue;
                }

                let bx = self.center.x + offset * c;
                let by = self.center.y + offset * s;

                // Draw slanted stripe as a rotated rect
                draw_rotated_rect(
                    bx,
                    by,
                    1.5,
                    stripe_h,
                    self.angle + stripe_slant,
                    outline_col,
                );

                // Scaled Grit/Texture (Sub-pixel detail)
                let (gx, gy, gw, gh) = if i % 2 == 0 {
                    (4.0, -4.0, 4.0, 4.0)
                } else {
                    (-4.0, 8.0, 4.0, 4.0)
                };

                let rcx = bx + gx * c - gy * s;
                let rcy = by + gx * s + gy * c;
                draw_rotated_rect(
                    rcx,
                    rcy,
                    gw,
                    gh,
                    self.angle,
                    Color::new(1.0, 1.0, 1.0, 0.05),
                );

                if i % 2 == 0 {
                    let (gx2, gy2, gw2, gh2) = (-10.0, 6.0, 2.0, 2.0);
                    let rcx2 = bx + gx2 * c - gy2 * s;
                    let rcy2 = by + gx2 * s + gy2 * c;
                    draw_rotated_rect(
                        rcx2,
                        rcy2,
                        gw2,
                        gh2,
                        self.angle,
                        Color::new(0.0, 0.0, 0.0, 0.1),
                    );
                }
            }
        }

        let top_thickness = self.size.y * 0.3;
        let dist = (self.size.y - top_thickness) / 2.0;
        let dx = s * dist;
        let dy = -c * dist;

        if self.is_jump_pad {
            let time = get_time() as f32 * 8.0;
            let pulse = (time.sin() * 0.5 + 0.5) * 0.6 + 0.4;
            let pad_col = Color::new(1.0, 0.4 * pulse, 0.0, 1.0);
            draw_rotated_rect(
                self.center.x + dx,
                self.center.y + dy,
                self.size.x,
                top_thickness,
                self.angle,
                pad_col,
            );

            // Glowing core
            let up_dx = s * 5.0;
            let up_dy = -c * 5.0;
            draw_rectangle(
                self.center.x + up_dx - 3.0,
                self.center.y + up_dy - 3.0,
                6.0,
                6.0,
                YELLOW,
            );
            draw_rectangle(
                self.center.x - self.size.x * 0.25 + up_dx - 3.0,
                self.center.y + up_dy - 3.0,
                6.0,
                6.0,
                YELLOW,
            );
            draw_rectangle(
                self.center.x + self.size.x * 0.25 + up_dx - 3.0,
                self.center.y + up_dy - 3.0,
                6.0,
                6.0,
                YELLOW,
            );
        } else {
            draw_rotated_rect(
                self.center.x + dx,
                self.center.y + dy,
                self.size.x,
                top_thickness,
                self.angle,
                theme.plat_top,
            );
            // Subtle 1px highlight line on top
            draw_line(
                self.center.x + dx - (self.size.x / 2.0 - 2.0) * c,
                self.center.y + dy - (self.size.x / 2.0 - 2.0) * s,
                self.center.x + dx + (self.size.x / 2.0 - 2.0) * c,
                self.center.y + dy + (self.size.x / 2.0 - 2.0) * s,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.2),
            );
        }

        // Icy Overlay
        if self.iced_timer > 0.0 {
            let ice_alpha = (self.iced_timer.min(1.0) * 0.6) + (get_time().sin() as f32 * 0.05);
            draw_rotated_rect(
                self.center.x,
                self.center.y,
                self.size.x,
                self.size.y,
                self.angle,
                Color::new(0.9, 0.95, 1.0, ice_alpha),
            );
            // Sparkle
            let count = 3;
            for i in 0..count {
                let t = get_time() as f32 * 2.0 + (i as f32);
                let x_off = t.sin() * self.size.x * 0.4;
                let y_off = (t.cos() * 4.0) - 8.0;
                draw_rectangle(
                    self.center.x + x_off,
                    self.center.y + y_off,
                    2.0,
                    2.0,
                    Color::new(1.0, 1.0, 1.0, 0.4),
                );
            }
        }
    }
}

// ---------- Scenery Rendering ----------

impl Scenery {
    pub fn draw(&self, theme: &MapTheme) {
        let x = self.pos.x;
        let y = self.pos.y;
        match self.kind {
            SceneryKind::Tree => {
                let trunk = Color::new(0.35, 0.2, 0.1, 1.0);
                let trunk_dark = Color::new(0.25, 0.15, 0.05, 1.0);
                let trunk_light = Color::new(0.45, 0.3, 0.15, 1.0);

                // Trunk with detail
                draw_rectangle(x - 8.0, y - 70.0, 16.0, 70.0, trunk);
                draw_rectangle(x - 8.0, y - 70.0, 4.0, 70.0, trunk_dark);
                draw_rectangle(x + 4.0, y - 70.0, 2.0, 70.0, trunk_light);
                // Roots
                draw_rectangle(x - 12.0, y - 10.0, 24.0, 10.0, trunk);

                let leaf_base = theme.plat_top;
                let leaf_mid =
                    Color::new(leaf_base.r * 0.8, leaf_base.g * 0.8, leaf_base.b * 0.8, 1.0);
                let leaf_dark =
                    Color::new(leaf_base.r * 0.6, leaf_base.g * 0.6, leaf_base.b * 0.6, 1.0);
                let leaf_bright = Color::new(
                    (leaf_base.r + 0.2).min(1.0),
                    (leaf_base.g + 0.2).min(1.0),
                    (leaf_base.b + 0.2).min(1.0),
                    1.0,
                );

                // Tiered Leaves (Bottom to Top)
                // Tier 1 (Bottom)
                draw_rectangle(x - 45.0, y - 60.0, 90.0, 25.0, leaf_dark);
                draw_rectangle(x - 40.0, y - 62.0, 80.0, 20.0, leaf_mid);
                draw_rectangle(x - 30.0, y - 65.0, 10.0, 5.0, leaf_bright); // Highlight

                // Tier 2 (Middle)
                draw_rectangle(x - 35.0, y - 85.0, 70.0, 30.0, leaf_dark);
                draw_rectangle(x - 30.0, y - 87.0, 60.0, 25.0, leaf_mid);
                draw_rectangle(x - 20.0, y - 90.0, 10.0, 5.0, leaf_bright); // Highlight

                // Tier 3 (Top)
                draw_rectangle(x - 25.0, y - 110.0, 50.0, 35.0, leaf_dark);
                draw_rectangle(x - 20.0, y - 112.0, 40.0, 30.0, leaf_base);
                draw_rectangle(x - 10.0, y - 115.0, 15.0, 8.0, leaf_bright); // Highlight
            }
            SceneryKind::Bush => {
                let leaf_base = theme.plat_top;
                let leaf_dark = Color::new(
                    leaf_base.r * 0.65,
                    leaf_base.g * 0.65,
                    leaf_base.b * 0.65,
                    1.0,
                );
                let leaf_bright = Color::new(
                    (leaf_base.r + 0.15).min(1.0),
                    (leaf_base.g + 0.15).min(1.0),
                    (leaf_base.b + 0.15).min(1.0),
                    1.0,
                );

                // Left Cluster
                draw_rectangle(x - 35.0, y - 30.0, 40.0, 30.0, leaf_dark);
                draw_rectangle(x - 30.0, y - 32.0, 30.0, 25.0, leaf_base);
                draw_rectangle(x - 25.0, y - 35.0, 8.0, 4.0, leaf_bright);

                // Right Cluster
                draw_rectangle(x - 5.0, y - 25.0, 40.0, 25.0, leaf_dark);
                draw_rectangle(x, y - 27.0, 30.0, 20.0, leaf_base);
                draw_rectangle(x + 5.0, y - 30.0, 8.0, 4.0, leaf_bright);

                // Berries
                let berry_col = if theme.plat_top.g > 0.5 { RED } else { YELLOW };
                draw_rectangle(x - 20.0, y - 15.0, 4.0, 4.0, berry_col);
                draw_rectangle(x + 15.0, y - 20.0, 4.0, 4.0, berry_col);
                draw_rectangle(x + 2.0, y - 10.0, 4.0, 4.0, berry_col);
            }
            SceneryKind::Cactus => {
                let green = Color::new(0.25, 0.45, 0.2, 1.0);
                let green_dark = Color::new(0.15, 0.3, 0.1, 1.0);
                let green_light = Color::new(0.35, 0.6, 0.3, 1.0);

                // Main body
                draw_rectangle(x - 12.0, y - 80.0, 24.0, 80.0, green_dark);
                draw_rectangle(x - 8.0, y - 78.0, 12.0, 76.0, green);
                draw_rectangle(x, y - 76.0, 4.0, 72.0, green_light);

                // Arm Left
                draw_rectangle(x - 32.0, y - 55.0, 20.0, 16.0, green_dark);
                draw_rectangle(x - 36.0, y - 75.0, 16.0, 36.0, green_dark);
                draw_rectangle(x - 34.0, y - 73.0, 8.0, 32.0, green);

                // Arm Right
                draw_rectangle(x + 12.0, y - 40.0, 20.0, 16.0, green_dark);
                draw_rectangle(x + 20.0, y - 60.0, 16.0, 36.0, green_dark);
                draw_rectangle(x + 24.0, y - 58.0, 8.0, 32.0, green);

                // Spines (random-ish pixels)
                for i in 0..12 {
                    let ox = (i as f32 * 13.0 % 20.0) - 10.0;
                    let oy = (i as f32 * 7.0 % 70.0) - 75.0;
                    draw_rectangle(x + ox, y + oy, 2.0, 2.0, WHITE);
                }

                // Flower
                draw_rectangle(x - 6.0, y - 85.0, 12.0, 6.0, PINK);
                draw_rectangle(x - 2.0, y - 88.0, 4.0, 4.0, YELLOW);
            }
            SceneryKind::CyberPillar => {
                let h = 90.0;
                let neon = theme.particle_col;
                let dark = Color::new(0.02, 0.02, 0.05, 1.0);
                let mid = Color::new(0.1, 0.1, 0.15, 1.0);

                // Pillar structure
                draw_rectangle(x - 18.0, y - h, 36.0, h, dark);
                draw_rectangle(x - 14.0, y - h, 28.0, h, mid);

                // Neon bands
                for i in 0..4 {
                    let by = y - 15.0 - (i as f32 * 20.0);
                    draw_rectangle(x - 18.0, by, 36.0, 4.0, dark);
                    draw_rectangle(x - 18.0, by + 1.0, 36.0, 2.0, neon);
                }

                // Vertical data lines
                draw_rectangle(x - 4.0, y - h + 5.0, 2.0, h - 10.0, neon);
                draw_rectangle(x + 2.0, y - h + 5.0, 2.0, h - 10.0, neon);

                // Top cap
                draw_rectangle(x - 22.0, y - h - 4.0, 44.0, 8.0, neon);
                draw_rectangle(x - 18.0, y - h - 6.0, 36.0, 4.0, WHITE);
            }
            SceneryKind::Coral => {
                let pink = Color::new(0.7, 0.2, 0.4, 1.0);
                let light_pink = Color::new(0.9, 0.4, 0.6, 1.0);
                let deep_pink = Color::new(0.5, 0.1, 0.3, 1.0);

                // Branching Logic (Blocky)
                // Main trunk
                draw_rectangle(x - 6.0, y - 60.0, 12.0, 60.0, deep_pink);
                draw_rectangle(x - 4.0, y - 58.0, 4.0, 56.0, pink);

                // Left branch
                draw_rectangle(x - 24.0, y - 40.0, 18.0, 10.0, deep_pink);
                draw_rectangle(x - 28.0, y - 55.0, 10.0, 20.0, deep_pink);
                draw_rectangle(x - 26.0, y - 53.0, 4.0, 16.0, light_pink);

                // Right branch
                draw_rectangle(x + 6.0, y - 30.0, 20.0, 10.0, deep_pink);
                draw_rectangle(x + 20.0, y - 50.0, 10.0, 25.0, deep_pink);
                draw_rectangle(x + 22.0, y - 48.0, 4.0, 20.0, light_pink);

                // Texture dots
                draw_rectangle(x - 2.0, y - 20.0, 3.0, 3.0, light_pink);
                draw_rectangle(x + 10.0, y - 10.0, 3.0, 3.0, deep_pink);
            }
            SceneryKind::IceSpike => {
                let ice_blue = Color::new(0.6, 0.8, 1.0, 0.7);
                let ice_dark = Color::new(0.4, 0.6, 0.9, 0.8);
                let ice_white = Color::new(0.9, 0.95, 1.0, 0.9);

                // Jagged Spike Main
                draw_rectangle(x - 20.0, y - 40.0, 40.0, 40.0, ice_dark);
                draw_rectangle(x - 15.0, y - 80.0, 25.0, 60.0, ice_blue);
                draw_rectangle(x - 5.0, y - 100.0, 10.0, 100.0, ice_white);

                // Side splinters
                draw_rectangle(x - 30.0, y - 30.0, 15.0, 30.0, ice_dark);
                draw_rectangle(x + 15.0, y - 50.0, 12.0, 50.0, ice_blue);

                // Refraction Highlights
                draw_rectangle(x - 2.0, y - 90.0, 2.0, 40.0, WHITE);
                draw_rectangle(x - 10.0, y - 40.0, 4.0, 4.0, WHITE);
                draw_rectangle(x + 8.0, y - 20.0, 3.0, 3.0, ice_white);
            }
        }
    }
}

impl Particle {
    pub fn draw(&self) {
        let alpha = self.life / self.max_life;
        let mut color = self.color;
        color.a *= alpha;
        draw_rectangle(
            self.pos.x - self.size * alpha / 2.0,
            self.pos.y - self.size * alpha / 2.0,
            self.size * alpha,
            self.size * alpha,
            color,
        );
    }
}
