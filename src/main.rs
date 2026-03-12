use macroquad::prelude::*;
use touch_me::*;
use touch_me::physics::resolve_tags;
use std::path::Path;

fn get_asset_path(path: &str) -> String {
    if Path::new(path).exists() {
        path.to_string()
    } else {
        #[cfg(target_os = "linux")]
        {
            let system_path = format!("/usr/share/touch_me/{}", path);
            if Path::new(&system_path).exists() {
                return system_path;
            }
        }
        path.to_string()
    }
}

fn all_controls() -> Vec<Controls> {
    vec![
        Controls { up: KeyCode::W, left: KeyCode::A, right: KeyCode::D },
        Controls { up: KeyCode::Up, left: KeyCode::Left, right: KeyCode::Right },
        Controls { up: KeyCode::I, left: KeyCode::J, right: KeyCode::L },
        Controls { up: KeyCode::T, left: KeyCode::F, right: KeyCode::H },
    ]
}













async fn load_fixed_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
    let actual_path = get_asset_path(path);
    let img = load_image(&actual_path).await?;
    let mut fixed = img.clone();
    for y in 0..img.height {
        for x in 0..img.width {
            let p = img.get_pixel(x as u32, y as u32);
            // Replace white or near-white with transparency
            if p.r > 0.95 && p.g > 0.95 && p.b > 0.95 {
                fixed.set_pixel(x as u32, y as u32, Color::new(0.0, 0.0, 0.0, 0.0));
            }
        }
    }
    let tex = Texture2D::from_image(&fixed);
    tex.set_filter(FilterMode::Nearest);
    Ok(tex)
}

#[macroquad::main("Touch Me Reforged")]
async fn main() {
    // Enable linear filtering for smoother text rendering
    set_default_filter_mode(FilterMode::Linear);
    
    // Load player textures with chroma-keying
    let player_textures = [
        load_fixed_texture("players/red.png").await,
        load_fixed_texture("players/blue.png").await,
        load_fixed_texture("players/green.png").await,
        load_fixed_texture("players/yellow.png").await,
    ];

    // Load powerup textures
    let pu_textures = [
        load_fixed_texture("images/sneakers.png").await,
        load_fixed_texture("images/cloud.png").await,
        load_fixed_texture("images/ghost.png").await,
        load_fixed_texture("images/ice.png").await,
        load_fixed_texture("images/shield.png").await,
        load_fixed_texture("images/portal blue.png").await,
        load_fixed_texture("images/portal green.png").await,
    ];
    
    let mut state = GameState::Menu;
    let mut nplayers: usize = 2;
    let mut map_idx: usize = 0; // 0-4 = presets, 5 = custom (editor level), 6+ = loaded custom levels
    let mut theme_idx: usize = 0; // 0-4 for theming
    let mut timer_secs: f32 = 60.0;
    let mut time_left: f32 = 0.0;
    let mut spawn_timer = 0.0;
    let mut players: Vec<Player> = Vec::new();
    let mut platforms: Vec<Platform> = Vec::new();
    let mut scenery: Vec<Scenery> = Vec::new();
    let mut powerups: Vec<Powerup> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    
    let mut editor_plats: Vec<Platform> = build_map(4);
    let mut editor_spawns: Vec<Vec2> = vec![
        vec2(REF_W * 0.1, REF_H * 0.3), vec2(REF_W * 0.9, REF_H * 0.3),
        vec2(REF_W * 0.1, REF_H * 0.7), vec2(REF_W * 0.9, REF_H * 0.7),
    ];
    let mut drag = DragMode::None;
    let mut edit_mode_pad = false; // Toggle for Editor jump pads
    let mut custom_levels = list_custom_levels();
    let mut editing_custom_level: Option<usize> = None; // Track which custom level is being edited
    let mut export_message = "";
    let mut export_message_timer = 0.0;
    
    let mut powerups_enabled = false;

    loop {
        // --- Virtual Resolution Setup ---
        let sw = screen_width();
        let sh = screen_height();
        let aspect = REF_W / REF_H;
        let screen_aspect = sw / sh;
        let (vw, vh, vx, vy) = if screen_aspect > aspect {
            let vw = sh * aspect;
            (vw, sh, (sw - vw) / 2.0, 0.0)
        } else {
            let vh = sw / aspect;
            (sw, vh, 0.0, (sh - vh) / 2.0)
        };

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, REF_H, REF_W, -REF_H));
        camera.viewport = Some((vx as i32, vy as i32, vw as i32, vh as i32));

        let (smx, smy) = mouse_position();
        let mpos = camera.screen_to_world(vec2(smx, smy));
        
        set_default_camera();
        clear_background(BLACK);
        
        set_camera(&camera);
        let current_theme = &MAP_THEMES[theme_idx.min(4)];
        
        let dt = get_frame_time().min(0.033);

        match state {
            GameState::Menu => {
                draw_gradient(current_theme.bg_top, current_theme.bg_bot);
                draw_particles(map_idx, get_time(), current_theme.particle_col);

                let cx = REF_W / 2.0;

                // Title
                let td = measure_text("TOUCH ME", None, 60, 1.0);
                draw_text("TOUCH ME", cx - td.width / 2.0 + 3.0, 50.0 + 3.0, 60.0, Color::new(0.0, 0.0, 0.0, 0.3));
                draw_text("TOUCH ME", cx - td.width / 2.0, 50.0, 60.0, current_theme.text_col);
                
                let sd = measure_text("A game of tag", None, 20, 1.0);
                draw_text("A game of tag", cx - sd.width / 2.0, 75.0, 20.0, current_theme.text_col);

                // Players
                let pd = measure_text("Players", None, 24, 1.0);
                draw_text("Players", cx - pd.width / 2.0, 115.0, 24.0, current_theme.text_col);
                let bw = 70.0; let gap = 15.0;
                let bsx = cx - (3.0 * bw + 2.0 * gap) / 2.0;
                for i in 0..3 {
                    let n = i + 2;
                    if draw_m3_btn(mpos, bsx + i as f32 * (bw + gap), 140.0, bw, 40.0, &n.to_string(), nplayers == n, 24.0, current_theme) {
                        nplayers = n;
                    }
                }

                // Map - Preset levels + Custom
                let md = measure_text("Preset Maps", None, 24, 1.0);
                draw_text("Preset Maps", cx - md.width / 2.0, 200.0, 24.0, current_theme.text_col);
                let map_names = ["Forest", "Desert", "Cyber", "Ocean", "Ice", "Custom"];
                let mw = 95.0; let mgap = 8.0;
                let msx = cx - (6.0 * mw + 5.0 * mgap) / 2.0;
                for i in 0..6 {
                    if i < 5 {
                        if draw_m3_btn(mpos, msx + i as f32 * (mw + mgap), 225.0, mw, 38.0, map_names[i], map_idx == i, 18.0, current_theme) {
                            map_idx = i;
                            theme_idx = i;
                        }
                    } else {
                        if draw_m3_btn(mpos, msx + i as f32 * (mw + mgap), 225.0, mw, 38.0, "Custom", map_idx == 5, 18.0, current_theme) {
                            map_idx = 5;
                        }
                    }
                }
                
                // Custom Loaded Levels
                if !custom_levels.is_empty() {
                    let cl = measure_text("Saved Custom Levels", None, 24, 1.0);
                    draw_text("Saved Custom Levels", cx - cl.width / 2.0, 280.0, 24.0, current_theme.text_col);
                    let cmw = 100.0; let cmgap = 8.0; let edit_btn_w = 28.0;
                    let max_custom_per_row = 4;
                    for (i, level) in custom_levels.iter().enumerate().take(8) {
                        let row = i / max_custom_per_row;
                        let col = i % max_custom_per_row;
                        let cx_offset = cx - (max_custom_per_row as f32 * (cmw + edit_btn_w) + (max_custom_per_row as f32 - 1.0) * cmgap) / 2.0;
                        let cx_btn = cx_offset + col as f32 * (cmw + edit_btn_w + cmgap);
                        let cy_btn = 305.0 + row as f32 * 43.0;
                        
                        // Main level button
                        if draw_m3_btn(mpos, cx_btn, cy_btn, cmw, 35.0, level, map_idx == 6 + i, 16.0, current_theme) {
                            map_idx = 6 + i;
                        }
                        
                        // Edit button (pencil icon)
                        let edit_x = cx_btn + cmw + 4.0;
                        if draw_m3_btn(mpos, edit_x, cy_btn, edit_btn_w, 35.0, "✏", false, 16.0, current_theme) {
                            // Load this custom level into the editor
                            if let Ok((loaded_plats, loaded_spawns)) = import_level(&custom_levels[i]) {
                                editor_plats = loaded_plats;
                                editor_spawns = loaded_spawns;
                                editing_custom_level = Some(i);
                                state = GameState::Editor;
                            }
                        }
                    }
                }

                // --- Game Settings Section ---
                let settings_y = if custom_levels.is_empty() { 300.0 } else { 410.0 };
                
                // Draw a subtle separator
                draw_line(cx - 300.0, settings_y - 15.0, cx + 300.0, settings_y - 15.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.1));

                let sth = measure_text("Game Settings", None, 22, 1.0);
                draw_text("Game Settings", cx - sth.width / 2.0, settings_y, 22.0, current_theme.text_col);
                
                // Timer Slider
                let tl = format!("Match Timer: {}s", timer_secs as i32);
                let _tld = measure_text(&tl, None, 18, 1.0);
                draw_text(&tl, cx - 150.0, settings_y + 35.0, 18.0, current_theme.text_col);
                timer_secs = draw_slider(mpos, cx - 150.0, settings_y + 55.0, 300.0, timer_secs, 15.0, 300.0, 5.0, current_theme);

                // Powerups Toggle
                let pu_y = settings_y + 90.0;
                let pu_label = if powerups_enabled { "Powerups: ENABLED" } else { "Powerups: DISABLED" };
                if draw_m3_btn(mpos, cx - 100.0, pu_y, 200.0, 35.0, pu_label, powerups_enabled, 18.0, current_theme) {
                    powerups_enabled = !powerups_enabled;
                }

                // --- Start Section ---
                let btn_y = REF_H - 100.0;
                let btn_gap = 20.0;
                let start_w = 160.0;
                let editor_w = 120.0;
                let total_w = start_w + editor_w + btn_gap;
                let start_x_base = cx - total_w / 2.0;

                if draw_m3_btn(mpos, start_x_base, btn_y, editor_w, 45.0, "Editor", false, 22.0, current_theme) {
                    state = GameState::Editor;
                }
                if draw_m3_btn(mpos, start_x_base + editor_w + btn_gap, btn_y, start_w, 45.0, "START GAME", false, 26.0, current_theme) {
                    if map_idx < 5 {
                        platforms = build_map(map_idx);
                        scenery = build_scenery(map_idx, &platforms);
                        editor_spawns = build_spawns(map_idx);
                    } else if map_idx == 5 {
                        platforms = editor_plats.clone();
                        scenery = build_scenery(4, &platforms);
                    } else if map_idx - 6 < custom_levels.len() {
                        if let Ok((loaded_platforms, loaded_spawns)) = import_level(&custom_levels[map_idx - 6]) {
                            platforms = loaded_platforms;
                            scenery = build_scenery(4, &platforms);
                            editor_spawns = if loaded_spawns.is_empty() {
                                vec![
                                    vec2(REF_W * 0.1, REF_H * 0.3), vec2(REF_W * 0.9, REF_H * 0.3),
                                    vec2(REF_W * 0.1, REF_H * 0.7), vec2(REF_W * 0.9, REF_H * 0.7),
                                ]
                            } else {
                                loaded_spawns
                            };
                        } else {
                            platforms = build_map(0);
                            scenery = build_scenery(0, &platforms);
                        }
                    } else {
                        platforms = build_map(0);
                        scenery = build_scenery(0, &platforms);
                    }
                    let ctrls = all_controls();
                    players.clear();
                    for (i, c) in ctrls.into_iter().enumerate() {
                        if i >= nplayers || i >= editor_spawns.len() { break; }
                        players.push(Player::new(editor_spawns[i], PLAYER_COLORS[i], c, PLAYER_RADIUS));
                    }
                    if !players.is_empty() {
                        players[0].is_it = true;
                    }

                    // Populate Powerups (Empty initially, dynamic spawn will handle it)
                    powerups.clear();
                    spawn_timer = 2.0; // Wait 2 seconds before first spawn

                    time_left = timer_secs;
                    state = GameState::Playing;
                }
            }

            GameState::Playing | GameState::Paused => {
                // Background
                draw_gradient(current_theme.bg_top, current_theme.bg_bot);
                draw_particles(map_idx, get_time(), current_theme.particle_col);
                
                // Draw Background Scenery
                for s in &scenery { if !s.fg { s.draw(current_theme); } }
                
                // Draw Platforms
                for p in &platforms { p.draw(current_theme); }

                // Draw Particles
                for p in &particles { p.draw(); }

                // Draw Powerups
                for pu in &powerups { pu.draw(&pu_textures); }
                
                // Draw Players
                for (i, p) in players.iter().enumerate() {
                    let tex = if i < player_textures.len() {
                        player_textures[i].as_ref().ok()
                    } else {
                        None
                    };
                    p.draw(tex);
                }
                
                // Draw Foreground Scenery (Hiding spots)
                for s in &scenery { if s.fg { s.draw(current_theme); } }

                let tt = format!("{}s", time_left.ceil() as i32);
                let ttd = measure_text(&tt, None, 48, 1.0);
                draw_text(&tt, REF_W / 2.0 - ttd.width / 2.0 + 2.0, 54.0, 48.0, Color::new(0.0, 0.0, 0.0, 0.3));
                draw_text(&tt, REF_W / 2.0 - ttd.width / 2.0, 52.0, 48.0, current_theme.text_col);

                let ct = match players.len() {
                    2 => "P1: WASD | P2: Arrows",
                    3 => "P1: WASD | P2: Arrows | P3: IJKL",
                    _ => "P1: WASD | P2: Arrows | P3: IJKL | P4: TFGH",
                };
                draw_text(ct, 30.0, REF_H - 15.0, 22.0, Color::new(1.0, 1.0, 1.0, 0.5));

                if state == GameState::Playing {
                    if is_key_pressed(KeyCode::Escape) {
                        state = GameState::Paused;
                    } else {
                        // Platform logic (timers)
                        for p in &mut platforms {
                            p.iced_timer = (p.iced_timer - dt).max(0.0);
                        }
                        
                        // Particle Logic
                        for p in &mut particles {
                            p.pos += p.vel * dt;
                            p.life -= dt;
                        }
                        particles.retain(|p| p.life > 0.0);

                        // Game Logic
                        for i in 0..players.len() { 
                            players[i].update(dt, &platforms); 
                            
                            // Powerup collection
                            let mut pu_to_remove = Vec::new();
                            for pu_idx in 0..powerups.len() {
                                if !powerups[pu_idx].active { continue; }
                                
                                let pu_pos = powerups[pu_idx].pos;
                                let pu_kind = powerups[pu_idx].kind;

                                if (players[i].pos - pu_pos).length() < 30.0 {
                                    // Prevent immediate portal consumption if we just teleported
                                    if (pu_kind == PowerupKind::PortalBlue || pu_kind == PowerupKind::PortalGreen) && players[i].teleport_timer > 0.0 {
                                        continue;
                                    }

                                    // Mark both portals for removal
                                    if pu_kind == PowerupKind::PortalBlue || pu_kind == PowerupKind::PortalGreen {
                                        // Collection feedback
                                        for _ in 0..15 {
                                            particles.push(Particle {
                                                pos: pu_pos,
                                                vel: vec2(rand::gen_range(-100.0, 100.0), rand::gen_range(-100.0, 100.0)),
                                                color: if pu_kind == PowerupKind::PortalBlue { BLUE } else { GREEN },
                                                life: 0.6,
                                                max_life: 0.6,
                                                size: 4.0,
                                            });
                                        }

                                        // Mark both portals for removal
                                        for k in 0..powerups.len() {
                                            if powerups[k].kind == PowerupKind::PortalBlue || powerups[k].kind == PowerupKind::PortalGreen {
                                                pu_to_remove.push(k);
                                            }
                                        }
                                    } else {
                                        // Collection feedback
                                        for _ in 0..10 {
                                            let pcolor = match pu_kind {
                                                PowerupKind::Turbo => YELLOW,
                                                PowerupKind::DoubleJump => SKYBLUE,
                                                PowerupKind::Ghost => PURPLE,
                                                PowerupKind::IcePatch => BLUE,
                                                PowerupKind::Shield => GOLD,
                                                _ => WHITE,
                                            };
                                            particles.push(Particle {
                                                pos: pu_pos,
                                                vel: vec2(rand::gen_range(-150.0, 150.0), rand::gen_range(-150.0, 150.0)),
                                                color: pcolor,
                                                life: 0.5,
                                                max_life: 0.5,
                                                size: 3.0,
                                            });
                                        }
                                        powerups[pu_idx].active = false;
                                    }
                                    
                                    match pu_kind {
                                        PowerupKind::Turbo => players[i].turbo_timer = 6.0,
                                        PowerupKind::DoubleJump => players[i].extra_jumps = 1,
                                        PowerupKind::Ghost => players[i].ghost_timer = 5.0,
                                        PowerupKind::IcePatch => {
                                            // Ice AOE: Freeze platforms in 250px radius
                                            let radius = 250.0;
                                            for p in &mut platforms {
                                                if (p.center - pu_pos).length() < radius {
                                                    p.iced_timer = 8.0;
                                                }
                                            }
                                        },
                                        PowerupKind::Shield => {
                                            players[i].shield_active = true;
                                            players[i].shield_timer = 8.0;
                                        },
                                        PowerupKind::PortalBlue | PowerupKind::PortalGreen => {
                                            if players[i].teleport_timer <= 0.0 {
                                                let target_kind = if pu_kind == PowerupKind::PortalBlue { PowerupKind::PortalGreen } else { PowerupKind::PortalBlue };
                                                let dest = powerups.iter().find(|p| p.kind == target_kind);
                                                if let Some(d) = dest {
                                                    players[i].pos = d.pos;
                                                    players[i].vel = Vec2::ZERO;
                                                    players[i].teleport_timer = 1.0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Process removals
                            pu_to_remove.sort();
                            pu_to_remove.dedup();
                            for idx in pu_to_remove.into_iter().rev() {
                                powerups.remove(idx);
                            }
                        }

                        // Dynamic Spawning
                        if powerups_enabled {
                            spawn_timer -= dt;
                            // Only spawn if active powerups < 3
                            let active_count = powerups.iter().filter(|p| p.active).count();
                            if spawn_timer <= 0.0 && active_count < 3 {
                                let kinds = [
                                    PowerupKind::Turbo, PowerupKind::DoubleJump, PowerupKind::Ghost,
                                    PowerupKind::IcePatch, PowerupKind::Shield, PowerupKind::PortalBlue
                                ];
                                let kind = kinds[rand::gen_range(0, kinds.len())];
                                
                                // Better reachability check
                                let mut valid_plats: Vec<usize> = Vec::new();
                                for (idx, p) in platforms.iter().enumerate() {
                                    // Skip boundary platforms (0-3) and ceiling (y < 120) and steep angles AND Jump Pads
                                    if idx > 3 && p.center.y > 120.0 && p.center.y < 700.0 && p.angle.abs() < 0.8 && !p.is_jump_pad {
                                        valid_plats.push(idx);
                                    }
                                }

                                if !valid_plats.is_empty() {
                                    let p1_idx = valid_plats[rand::gen_range(0, valid_plats.len())];
                                    let p1 = &platforms[p1_idx];
                                    let pos1 = p1.center + vec2(0.0, -25.0);
                                    
                                    if kind == PowerupKind::PortalBlue {
                                        if valid_plats.len() > 1 {
                                            let mut p2_idx = p1_idx;
                                            while p2_idx == p1_idx {
                                                p2_idx = valid_plats[rand::gen_range(0, valid_plats.len())];
                                            }
                                            let p2 = &platforms[p2_idx];
                                            let pos2 = p2.center + vec2(0.0, -25.0);
                                            powerups.push(Powerup { pos: pos1, kind: PowerupKind::PortalBlue, active: true, respawn_timer: 0.0, spawn_in_timer: 0.5 });
                                            powerups.push(Powerup { pos: pos2, kind: PowerupKind::PortalGreen, active: true, respawn_timer: 0.0, spawn_in_timer: 0.5 });
                                        }
                                    } else {
                                        powerups.push(Powerup { pos: pos1, kind, active: true, respawn_timer: 0.0, spawn_in_timer: 0.5 });
                                    }
                                }
                                spawn_timer = rand::gen_range(8.0, 15.0); // Slower spawn
                            }
                        }

                        // Update powerup animations
                        for pu in &mut powerups {
                            pu.spawn_in_timer = (pu.spawn_in_timer - dt).max(0.0);
                        }

                        // Cleanup inactive powerups
                        powerups.retain(|pu| pu.active);

                        // Tagging Logic
                        resolve_tags(&mut players, TAG_COOLDOWN);

                        time_left -= dt;
                        if time_left <= 0.0 {
                            time_left = 0.0;
                            let loser = players.iter().position(|p| p.is_it).unwrap_or(0);
                            state = GameState::GameOver { loser };
                        }
                    }
                } else {
                    // Paused Menu Overlay
                    if is_key_pressed(KeyCode::Escape) {
                        state = GameState::Playing;
                    }

                    draw_rectangle(0.0, 0.0, REF_W, REF_H, Color::new(0.0, 0.0, 0.0, 0.6));
                    
                    let cx = REF_W / 2.0;
                    let pd = measure_text("PAUSED", None, 80, 1.0);
                    draw_text("PAUSED", cx - pd.width / 2.0, REF_H * 0.35, 80.0, WHITE);

                    if draw_m3_btn(mpos, cx - 120.0, REF_H * 0.5, 240.0, 55.0, "Resume", false, 28.0, current_theme) {
                        state = GameState::Playing;
                    }
                    if draw_m3_btn(mpos, cx - 120.0, REF_H * 0.62, 240.0, 55.0, "Quit to Menu", false, 28.0, current_theme) {
                        state = GameState::Menu;
                    }
                }
            }

            GameState::GameOver { loser } => {
                draw_gradient(current_theme.bg_top, current_theme.bg_bot);
                draw_particles(map_idx, get_time(), current_theme.particle_col);

                let cx = REF_W / 2.0;
                let cy = REF_H / 2.0;

                let td = measure_text("TIME'S UP!", None, 80, 1.0);
                draw_text("TIME'S UP!", cx - td.width / 2.0 + 4.0, cy - 100.0 + 4.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.3));
                draw_text("TIME'S UP!", cx - td.width / 2.0, cy - 100.0, 80.0, current_theme.text_col);

                let lt = format!("{} was IT!", PLAYER_NAMES[loser]);
                let ld = measure_text(&lt, None, 40, 1.0);
                draw_text(&lt, cx - ld.width / 2.0 + 3.0, cy - 30.0 + 3.0, 40.0, Color::new(0.0, 0.0, 0.0, 0.5));
                draw_text(&lt, cx - ld.width / 2.0, cy - 30.0, 40.0, PLAYER_COLORS[loser]);
                
                // Draw loser player texture
                let size = PLAYER_RADIUS * 2.0;
                if loser < player_textures.len() {
                    if let Ok(tex) = &player_textures[loser] {
                        draw_texture_ex(tex, cx - size/2.0, cy + 50.0 - size/2.0, WHITE, DrawTextureParams {
                            dest_size: Some(vec2(size, size)),
                            ..Default::default()
                        });
                        draw_rectangle_lines(cx - size/2.0 - 3.0, cy + 50.0 - size/2.0 - 3.0, size + 6.0, size + 6.0, 4.0, WHITE);
                    }
                }

                if draw_m3_btn(mpos, cx - 180.0, cy + 120.0, 160.0, 55.0, "Play Again", false, 26.0, current_theme) {
                    let spawns = [
                        vec2(REF_W * 0.1, REF_H * 0.3), vec2(REF_W * 0.9, REF_H * 0.3),
                        vec2(REF_W * 0.1, REF_H * 0.7), vec2(REF_W * 0.9, REF_H * 0.7),
                    ];
                    for (i, p) in players.iter_mut().enumerate() {
                        p.pos = spawns[i]; p.vel = Vec2::ZERO; p.trail.clear();
                        p.is_it = false; p.on_ground = false; p.tag_timer = 0.0;
                    }
                    players[0].is_it = true;
                    time_left = timer_secs;
                    state = GameState::Playing;
                }
                if draw_m3_btn(mpos, cx + 20.0, cy + 120.0, 160.0, 55.0, "Menu", false, 26.0, current_theme) {
                    state = GameState::Menu;
                }
            }

// Replace your GameState::Editor match block in main.rs with this:

            GameState::Editor => {
                draw_gradient(current_theme.bg_top, current_theme.bg_bot);
                let edit_scenery = build_scenery(4, &editor_plats);
                
                let grid = 40.0;
                for x in 0..(REF_W / grid) as i32 + 1 {
                    draw_line(x as f32 * grid, 0.0, x as f32 * grid, REF_H, 1.0, Color::new(0.0, 0.0, 0.0, 0.1));
                }
                for y in 0..(REF_H / grid) as i32 + 1 {
                    draw_line(0.0, y as f32 * grid, REF_W, y as f32 * grid, 1.0, Color::new(0.0, 0.0, 0.0, 0.1));
                }

                let mut snap_pos = mpos;
                let is_snapping = is_key_down(KeyCode::LeftShift);
                if is_snapping {
                    snap_pos.x = (snap_pos.x / 20.0).round() * 20.0;
                    snap_pos.y = (snap_pos.y / 20.0).round() * 20.0;
                }

                for s in &edit_scenery { if !s.fg { s.draw(current_theme); } }

                for p in editor_plats.iter() {
                    p.draw(current_theme);
                    let (s, e) = p.endpoints();
                    draw_circle(s.x, s.y, 6.0, WHITE);
                    draw_circle(e.x, e.y, 6.0, WHITE);
                }
                
                // Draw spawn points
                let spawn_colors = [RED, BLUE, GREEN, YELLOW];
                for (i, spawn) in editor_spawns.iter().enumerate() {
                    let color = spawn_colors[i % 4];
                    draw_circle(spawn.x, spawn.y, 12.0, color);
                    draw_circle_lines(spawn.x, spawn.y, 14.0, 2.0, WHITE);
                    let num_text = (i + 1).to_string();
                    draw_text(&num_text, spawn.x - 4.0, spawn.y + 6.0, 16.0, WHITE);
                }
                
                for s in &edit_scenery { if s.fg { s.draw(current_theme); } }

                let tb_y = 12.0;
                if draw_m3_btn(mpos, 10.0, tb_y, 80.0, 40.0, "Back", false, 20.0, current_theme) {
                    drag = DragMode::None;
                    editing_custom_level = None;
                    state = GameState::Menu;
                }
                if draw_m3_btn(mpos, 100.0, tb_y, 80.0, 40.0, "Clear", false, 20.0, current_theme) {
                    editor_plats.clear();
                    editor_plats = build_map(4); 
                    drag = DragMode::None;
                }
                
                // Mode Toggle button
                // 0: Normal Pad, 1: Jump Pad, 2: Place Spawn
                // We'll use edit_mode_pad as a hacky state: false = normal, true = jump
                // We need a proper state variable for spawn placement, let's just cycle through
                let mode_lbl = if is_key_down(KeyCode::P) { "Mode: Spawn" } else if edit_mode_pad { "Mode: Jump Pad" } else { "Mode: Platform" };
                
                if draw_m3_btn(mpos, 190.0, tb_y, 160.0, 40.0, mode_lbl, false, 18.0, current_theme) {
                    edit_mode_pad = !edit_mode_pad; // Just toggle pads for now, spawn is still P key since adding a new state variable requires restructuring main
                }

                // Theme Selector
                let theme_names = ["Forest", "Desert", "Cyber", "Ocean", "Ice"];
                if draw_m3_btn(mpos, 360.0, tb_y, 140.0, 40.0, theme_names[theme_idx % 5], false, 20.0, current_theme) {
                    theme_idx = (theme_idx + 1) % 5;
                }
                
                if draw_m3_btn(mpos, 510.0, tb_y, 100.0, 40.0, "Export", false, 20.0, current_theme) {
                    let level_name = if let Some(idx) = editing_custom_level {
                        custom_levels[idx].clone()
                    } else {
                        let level_num = custom_levels.len() + 1;
                        format!("custom_{}", level_num)
                    };
                    match export_level(&editor_plats, &editor_spawns, &level_name) {
                        Ok(_) => {
                            export_message = "Level exported!";
                            export_message_timer = 2.0;
                            custom_levels = list_custom_levels();
                        }
                        Err(_) => {
                            export_message = "Export failed!";
                            export_message_timer = 2.0;
                        }
                    }
                }
                
                // Display export message
                if export_message_timer > 0.0 {
                    export_message_timer -= dt;
                    let msg_color = if export_message.contains("failed") { RED } else { GREEN };
                    draw_text(export_message, 10.0, 60.0, 18.0, msg_color);
                }

                let in_toolbar = mpos.y < 60.0;

                if is_mouse_button_pressed(MouseButton::Right) && !in_toolbar {
                    // Check if clicking on a spawn point
                    let mut remove_spawn = None;
                    for (i, spawn) in editor_spawns.iter().enumerate() {
                        if (*spawn - mpos).length() < 16.0 {
                            remove_spawn = Some(i);
                            break;
                        }
                    }
                    if let Some(idx) = remove_spawn {
                        editor_spawns.remove(idx);
                        drag = DragMode::None;
                    } else {
                        // Remove platform (ignore walls 0-3)
                        let mut remove_idx = None;
                        for (i, p) in editor_plats.iter().enumerate().skip(4) {
                            if circle_obb_resolve(mpos, 5.0, p.center, p.size, p.angle).is_some() {
                                remove_idx = Some(i);
                                break;
                            }
                        }
                        if let Some(idx) = remove_idx {
                            editor_plats.remove(idx);
                            drag = DragMode::None;
                        }
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) && !in_toolbar {
                    let grab_r = 15.0;
                    let mut grabbed = false;
                    
                    // Check spawn points first
                    for (i, spawn) in editor_spawns.iter().enumerate() {
                        if (*spawn - snap_pos).length() < 16.0 {
                            drag = DragMode::PlacingSpawn { id: i };
                            grabbed = true;
                            break;
                        }
                    }
                    
                    if !grabbed {
                        // Grab endpoints (ignore walls 0-3)
                        for (i, p) in editor_plats.iter().enumerate().skip(4) {
                            let (s, e) = p.endpoints();
                            if (s - snap_pos).length() < grab_r {
                                drag = DragMode::GrabStart { idx: i }; grabbed = true; break;
                            }
                            if (e - snap_pos).length() < grab_r {
                                drag = DragMode::GrabEnd { idx: i }; grabbed = true; break;
                            }
                        }
                    }
                    
                    if !grabbed && is_key_down(KeyCode::P) {
                        if editor_spawns.len() < 4 {
                            editor_spawns.push(snap_pos);
                        }
                    } else if !grabbed {
                        drag = DragMode::Creating { start: snap_pos };
                    }
                }

                if is_mouse_button_down(MouseButton::Left) {
                    match drag {
                        DragMode::PlacingSpawn { id } => {
                            if id < editor_spawns.len() {
                                editor_spawns[id] = snap_pos;
                            }
                        }
                        DragMode::GrabStart { idx } => {
                            if idx < editor_plats.len() && idx >= 4 {
                                let (_, e) = editor_plats[idx].endpoints();
                                // If snapping, align the other end too for cleaner lines
                                let mut start = snap_pos;
                                if is_snapping {
                                    start.x = (start.x / 20.0).round() * 20.0;
                                    start.y = (start.y / 20.0).round() * 20.0;
                                }
                                editor_plats[idx] = Platform::new_from_endpoints(start, e, editor_plats[idx].size.y, editor_plats[idx].is_jump_pad);
                            }
                        }
                        DragMode::GrabEnd { idx } => {
                            if idx < editor_plats.len() && idx >= 4 {
                                let (s, _) = editor_plats[idx].endpoints();
                                let mut end = snap_pos;
                                if is_snapping {
                                    end.x = (end.x / 20.0).round() * 20.0;
                                    end.y = (end.y / 20.0).round() * 20.0;
                                }
                                editor_plats[idx] = Platform::new_from_endpoints(s, end, editor_plats[idx].size.y, editor_plats[idx].is_jump_pad);
                            }
                        }
                        DragMode::Creating { start } => {
                            let temp = Platform::new_from_endpoints(start, snap_pos, 24.0, edit_mode_pad);
                            temp.draw(current_theme);
                        }
                        DragMode::None => {}
                    }
                }

                if is_mouse_button_released(MouseButton::Left) {
                    if let DragMode::Creating { start } = drag {
                        if (snap_pos - start).length() > 15.0 {
                            editor_plats.push(Platform::new_from_endpoints(start, snap_pos, 24.0, edit_mode_pad));
                        }
                    }
                    drag = DragMode::None;
                }

                draw_text("Drag plats | Grab spawns/ends to move | R-Click delete | Shift snap | P+Click place spawn",
                    5.0, REF_H - 15.0, 18.0, current_theme.text_col);
            }
        }
        next_frame().await;
    }
}