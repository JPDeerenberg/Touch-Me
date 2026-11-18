use crate::collision::circle_obb_resolve;
use crate::constants::*;
use crate::types::{Platform, Player};
use macroquad::prelude::*;

// ---------- Player Physics & Updates ----------

impl Player {
    pub fn update(&mut self, dt: f32, platforms: &[Platform]) {
        if self.tag_timer > 0.0 {
            self.tag_timer -= dt;
        }

        // Powerup Timers
        self.turbo_timer = (self.turbo_timer - dt).max(0.0);
        self.ghost_timer = (self.ghost_timer - dt).max(0.0);
        self.slip_timer = (self.slip_timer - dt).max(0.0);
        self.shield_timer = (self.shield_timer - dt).max(0.0);
        self.teleport_timer = (self.teleport_timer - dt).max(0.0);

        if self.shield_timer <= 0.0 {
            self.shield_active = false;
        }

        self.trail.insert(0, self.pos);
        self.trail.truncate(8); // Keep last 8 positions for motion blur

        let wants_l = is_key_down(self.controls.left);
        let wants_r = is_key_down(self.controls.right);

        let mut on_iced_platform = false;
        if self.on_ground {
            for p in platforms {
                if p.iced_timer > 0.0 {
                    // Check distance to platform center or collision.
                    // To keep it simple, if we are grounded on it and it's iced.
                    if let Some(_) =
                        circle_obb_resolve(self.pos, self.radius + 2.0, p.center, p.size, p.angle)
                    {
                        on_iced_platform = true;
                        break;
                    }
                }
            }
        }

        let is_slipping = self.slip_timer > 0.0 || on_iced_platform;
        let speed_mult = if self.turbo_timer > 0.0 { 1.6 } else { 1.0 };
        let current_speed = PLAYER_SPEED * speed_mult;
        let current_accel_mult = if is_slipping { 0.15 } else { 1.0 };

        // Apply acceleration or realistic friction
        if wants_l && !wants_r {
            self.vel.x -= ACCELERATION * current_accel_mult * dt;
        } else if wants_r && !wants_l {
            self.vel.x += ACCELERATION * current_accel_mult * dt;
        } else {
            let friction_mult = if is_slipping { 0.1 } else { 1.0 };
            if self.vel.x > 0.0 {
                self.vel.x = (self.vel.x - FRICTION * friction_mult * dt).max(0.0);
            } else if self.vel.x < 0.0 {
                self.vel.x = (self.vel.x + FRICTION * friction_mult * dt).min(0.0);
            }
        }
        self.vel.x = self.vel.x.clamp(-current_speed, current_speed);

        let wants_jump = is_key_pressed(self.controls.up);
        let can_jump = self.on_ground || self.coyote_timer > 0.0 || self.extra_jumps > 0;

        if wants_jump && can_jump {
            if !self.on_ground && self.coyote_timer <= 0.0 {
                self.extra_jumps -= 1;
            }
            self.vel.y = -JUMP_FORCE;
            self.on_ground = false;
            self.coyote_timer = 0.0;
        }

        // Only apply gravity if not grounded OR if moving/jumping
        if !self.on_ground || wants_l || wants_r || wants_jump || self.vel.length() > 10.0 {
            self.vel.y += GRAVITY * dt;
        }
        self.vel.y = self.vel.y.clamp(-2000.0, MAX_FALL_SPEED);

        self.pos += self.vel * dt;

        let mut grounded = false;
        let mut ground_normal = Vec2::ZERO;

        for _ in 0..4 {
            for p in platforms {
                // Use a slightly larger radius for the grounded check to avoid flickering
                let check_radius = self.radius + 2.0;
                if let Some(mtv) =
                    circle_obb_resolve(self.pos, self.radius, p.center, p.size, p.angle)
                {
                    self.pos += mtv;
                    let n = mtv.normalize();
                    let v_dot = self.vel.dot(n);
                    if v_dot < 0.0 {
                        self.vel -= n * v_dot;
                    }

                    // If we have a collision from above (n.y < -0.3), it counts as ground
                    if n.y < -0.3 {
                        if p.is_jump_pad {
                            self.vel.y = -JUMP_FORCE * 2.2;
                            self.on_ground = false;
                            self.coyote_timer = 0.0;
                            return; // Exit update after jump pad boost
                        } else {
                            grounded = true;
                            ground_normal = n;
                        }
                    }
                } else {
                    // Check if we are "just above" the platform to maintain grounded state
                    if let Some(_) =
                        circle_obb_resolve(self.pos, check_radius, p.center, p.size, p.angle)
                    {
                        // We check the normal by doing a tiny projection
                        if let Some(mtv_near) = circle_obb_resolve(
                            self.pos + vec2(0.0, 1.0),
                            self.radius,
                            p.center,
                            p.size,
                            p.angle,
                        ) {
                            let n = mtv_near.normalize();
                            if n.y < -0.3 && !p.is_jump_pad {
                                grounded = true;
                                ground_normal = n;
                            }
                        }
                    }
                }
            }
        }

        if grounded {
            self.on_ground = true;
            self.coyote_timer = 0.1;

            // Absolute stop if no input
            if !wants_l && !wants_r {
                self.vel = Vec2::ZERO;
            }
        } else {
            self.coyote_timer = (self.coyote_timer - dt).max(0.0);
            self.on_ground = false;
        }

        // --- Visual Tilt Logic ---
        let target_angle = if self.on_ground && ground_normal != Vec2::ZERO {
            // normal.y is negative (upwards). normal.x is the slope.
            // atan2(y, x) gives angle from X axis.
            // We want 0 degrees when normal is (0, -1).
            ground_normal.x.atan2(-ground_normal.y)
        } else {
            0.0
        };

        // Smoothly interpolate angle
        let lerp_speed = 15.0;
        let mut diff = target_angle - self.render_angle;
        while diff > std::f32::consts::PI {
            diff -= std::f32::consts::TAU;
        }
        while diff < -std::f32::consts::PI {
            diff += std::f32::consts::TAU;
        }
        self.render_angle += diff * (lerp_speed * dt).min(1.0);
    }

    pub fn draw(&self, texture: Option<&Texture2D>) {
        // Draw trail
        for (i, p) in self.trail.iter().enumerate() {
            let t = i as f32 / 8.0;
            let r = self.radius * (1.0 - t) * 0.4;
            let a = 0.4 * (1.0 - t);
            draw_rectangle(
                p.x - r,
                p.y - r,
                r * 2.0,
                r * 2.0,
                Color::new(self.color.r, self.color.g, self.color.b, a),
            );
        }

        let size = self.radius * 2.0;

        let alpha = if self.ghost_timer > 3.0 {
            0.0 // Completely invisible
        } else if self.ghost_timer > 0.0 {
            // Flickering back into existence
            (self.ghost_timer * 15.0).sin().abs() * 0.4
        } else {
            1.0
        };

        let mut color = self.color;
        color.a = alpha;

        // Double Jump Aura (Cloud effect at feet)
        if self.extra_jumps > 0 && alpha > 0.0 {
            let t = get_time() as f32;
            for i in 0..3 {
                let offset = (i as f32 - 1.0) * 10.0;
                let bounce = (t * 5.0 + offset).sin() * 3.0;
                let cx = self.pos.x + offset;
                let cy = self.pos.y + self.radius + 2.0 + bounce;

                // Draw small cloud puffs
                draw_circle(cx, cy, 5.0, Color::new(1.0, 1.0, 1.0, 0.4 * alpha));
                draw_circle(cx, cy, 3.0, Color::new(1.0, 1.0, 1.0, 0.6 * alpha));
            }
        }

        // Draw player texture or fallback to programmatic drawing
        if let Some(tex) = texture {
            // Use WHITE tint instead of player color to avoid tinting background/preserving sprite colors
            let tint = Color::new(1.0, 1.0, 1.0, alpha);
            draw_texture_ex(
                tex,
                self.pos.x - size / 2.0,
                self.pos.y - size / 2.0,
                tint,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    rotation: self.render_angle,
                    ..Default::default()
                },
            );
        } else {
            // Fallback: Pixel-art style box with eyes
            draw_rectangle_ex(
                self.pos.x,
                self.pos.y,
                size,
                size,
                DrawRectangleParams {
                    color,
                    rotation: self.render_angle,
                    offset: vec2(0.5, 0.5),
                    ..Default::default()
                },
            );

            // Rotating Eyes
            let (s, c) = self.render_angle.sin_cos();
            let eye_size = 4.0;

            // Offset from center for each eye
            let e1_off = vec2(-6.0, -4.0);
            let e2_off = vec2(2.0, -4.0);

            // Rotate offsets
            let e1_rot = vec2(e1_off.x * c - e1_off.y * s, e1_off.x * s + e1_off.y * c);
            let e2_rot = vec2(e2_off.x * c - e2_off.y * s, e2_off.x * s + e2_off.y * c);

            draw_rectangle_ex(
                self.pos.x + e1_rot.x,
                self.pos.y + e1_rot.y,
                eye_size,
                eye_size,
                DrawRectangleParams {
                    color: WHITE,
                    rotation: self.render_angle,
                    offset: vec2(0.5, 0.5),
                    ..Default::default()
                },
            );
            draw_rectangle_ex(
                self.pos.x + e2_rot.x,
                self.pos.y + e2_rot.y,
                eye_size,
                eye_size,
                DrawRectangleParams {
                    color: WHITE,
                    rotation: self.render_angle,
                    offset: vec2(0.5, 0.5),
                    ..Default::default()
                },
            );
        }

        // Shield Effect
        if self.shield_active {
            let r = self.radius + 5.0;
            draw_rectangle_lines(
                self.pos.x - r,
                self.pos.y - r,
                r * 2.0,
                r * 2.0,
                2.0,
                SKYBLUE,
            );
        }

        // Do not draw "IT" marker if invisible
        if self.is_it && alpha > 0.1 {
            // "IT" marker - Pixelated arrow
            let arrow_y = self.pos.y - size / 2.0 - 24.0;
            let arrow_x = self.pos.x;
            draw_rectangle(arrow_x - 4.0, arrow_y, 8.0, 12.0, RED);
            draw_rectangle(arrow_x - 8.0, arrow_y + 8.0, 16.0, 4.0, RED);
            draw_rectangle(arrow_x - 4.0, arrow_y + 12.0, 8.0, 4.0, RED);
        }
        if self.tag_timer > 0.0 {
            draw_rectangle(
                self.pos.x - size / 2.0,
                self.pos.y - size / 2.0 - 12.0,
                size * (self.tag_timer / TAG_COOLDOWN),
                5.0,
                YELLOW,
            );
        }
    }
}

pub fn resolve_tags(players: &mut [Player], tag_cooldown: f32) {
    let n = players.len();
    for i in 0..n {
        if !players[i].is_it || players[i].tag_timer > 0.0 {
            continue;
        }

        for j in 0..n {
            if i == j {
                continue;
            }

            // Check collision
            let dist = (players[i].pos - players[j].pos).length();
            if dist < (players[i].radius + players[j].radius) {
                // Potential tag!

                // Ghost check
                if players[j].ghost_timer > 0.0 {
                    continue;
                }

                // Shield check
                if players[j].shield_active {
                    players[j].shield_active = false;
                    players[i].tag_timer = tag_cooldown; // IT gets penalized/cooldown for hitting a shield
                    continue;
                }

                // Successful tag!
                players[i].is_it = false;
                players[j].is_it = true;
                players[j].tag_timer = tag_cooldown;
                return; // Only one tag per frame
            }
        }
    }
}

// ---------- Powerup Drawing ----------
use crate::types::{Powerup, PowerupKind};

impl Powerup {
    pub fn draw(&self, textures: &[Texture2D; 7]) {
        if !self.active {
            return;
        }

        // Spawn/Pop animation scale
        // spawn_in_timer ranges from 0.5 down to 0.0
        let scale = if self.spawn_in_timer > 0.0 {
            let t = 1.0 - (self.spawn_in_timer / 0.5);
            // Elastic out effect
            1.2 * (1.0 - (1.0 - t).powi(3))
        } else {
            1.0
        };

        let (color, tex_idx) = match self.kind {
            PowerupKind::Turbo => (YELLOW, 0),
            PowerupKind::DoubleJump => (SKYBLUE, 1),
            PowerupKind::Ghost => (PURPLE, 2),
            PowerupKind::IcePatch => (BLUE, 3),
            PowerupKind::Shield => (GOLD, 4),
            PowerupKind::PortalBlue => (BLUE, 5),
            PowerupKind::PortalGreen => (GREEN, 6),
        };

        let label = match self.kind {
            PowerupKind::Turbo => "SNK",
            PowerupKind::DoubleJump => "CLD",
            PowerupKind::Ghost => "GST",
            PowerupKind::IcePatch => "ICE",
            PowerupKind::Shield => "SHD",
            PowerupKind::PortalBlue => "PRT",
            PowerupKind::PortalGreen => "PRT",
        };

        // Gentle float animation
        let time = get_time() as f32;
        let y_off = (time * 3.0).sin() * 5.0;
        let pos = self.pos + vec2(0.0, y_off);

        // Glow behind
        draw_rectangle(
            pos.x - 22.0 * scale,
            pos.y - 22.0 * scale,
            44.0 * scale,
            44.0 * scale,
            Color::new(color.r, color.g, color.b, 0.3),
        );

        // Portal Swirl Effect
        if self.kind == PowerupKind::PortalBlue || self.kind == PowerupKind::PortalGreen {
            let swirl_count = 4;
            for i in 0..swirl_count {
                let angle = time * 4.0 + (i as f32 * std::f32::consts::TAU / swirl_count as f32);
                let swirl_pos = pos + vec2(angle.cos(), angle.sin()) * 20.0 * scale;
                let swirl_c = Color::new(color.r, color.g, color.b, 0.6);
                draw_rectangle(
                    swirl_pos.x - 4.0,
                    swirl_pos.y - 4.0,
                    8.0 * scale,
                    8.0 * scale,
                    swirl_c,
                );

                // Second ring
                let angle2 = -time * 3.0 + (i as f32 * std::f32::consts::TAU / swirl_count as f32);
                let swirl_pos2 = pos + vec2(angle2.cos(), angle2.sin()) * 14.0 * scale;
                draw_rectangle(
                    swirl_pos2.x - 3.0,
                    swirl_pos2.y - 3.0,
                    6.0 * scale,
                    6.0 * scale,
                    swirl_c,
                );
            }
        }

        if let Some(tex) = textures.get(tex_idx) {
            let size = 32.0 * scale;
            draw_texture_ex(
                tex,
                pos.x - size / 2.0,
                pos.y - size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback to square and text if texture missing
            draw_rectangle(
                pos.x - 18.0 * scale,
                pos.y - 20.0 * scale,
                36.0 * scale,
                36.0 * scale,
                color,
            );
            let td = measure_text(label, None, (14.0 * scale) as u16, 1.0);
            draw_text(
                label,
                pos.x - td.width / 2.0,
                pos.y + td.height / 2.0 - 2.0,
                14.0 * scale,
                WHITE,
            );
        }
    }
}
