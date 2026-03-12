use macroquad::prelude::*;
use crate::types::{Platform, Scenery, SceneryKind, PlatformData, LevelData};
use std::fs;
use std::path::{Path, PathBuf};

fn get_levels_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let p = Path::new(&home).join(".local/share/touch_me/custom_levels");
            if let Ok(_) = fs::create_dir_all(&p) {
                return p;
            }
        }
    }
    PathBuf::from("custom_levels")
}

// ---------- Scenery Spawning ----------

pub fn spawn_scen(p: &Platform, offset_x: f32, kind: SceneryKind, fg: bool) -> Scenery {
    let c = p.angle.cos();
    let s = p.angle.sin();
    let local_y = -p.size.y / 2.0 + 2.0; // Sink into the floor slightly to prevent 1px air gaps
    let world_x = p.center.x + offset_x * c - local_y * s;
    let world_y = p.center.y + offset_x * s + local_y * c;
    Scenery { pos: vec2(world_x, world_y), kind, fg }
}

// ---------- Level Building ----------

pub fn build_scenery(idx: usize, platforms: &[Platform]) -> Vec<Scenery> {
    let mut scenery = Vec::new();
    
    // Choose primary decoration based on theme
    let (primary_kind, has_bushes) = match idx {
        0 => (SceneryKind::Tree, true),      // Forest
        1 => (SceneryKind::Cactus, true),    // Desert
        2 => (SceneryKind::CyberPillar, false), // Cyber
        3 => (SceneryKind::Coral, false),    // Ocean
        4 => (SceneryKind::IceSpike, false),  // Ice
        _ => (SceneryKind::Tree, true),
    };

    for (i, p) in platforms.iter().enumerate() {
        // Skip arena boundaries (0-3) and jump pads
        if i < 4 || p.is_jump_pad {
            continue;
        }

        // Only place on horizontal platforms (near-zero angle)
        if p.angle.abs() > 0.01 {
            continue;
        }

        let width = p.size.x;
        // Skip platforms that are too narrow
        if width < 60.0 {
            continue;
        }

        // Determine spacing based on width
        // Small platforms get 1, larger ones get more
        let count = (width / 180.0).max(1.0) as i32;
        
        for j in 0..count {
            // Pseudo-random distribution using iterator index
            let seed = (i * 13 + j as usize * 7) % 100;
            
            // Only place scenery on ~30% of eligible "slots" to avoid over-crowding
            if seed < 35 {
                let t = if count > 1 {
                    (j as f32 / (count - 1) as f32) * 2.0 - 1.0
                } else {
                    0.0
                };
                
                // Keep away from the very edges
                let margin = 20.0;
                let offset_x = t * (width * 0.5 - margin);
                
                // For forest/desert, alternate between primary and bushes
                let kind = if has_bushes && seed % 2 == 0 {
                    SceneryKind::Bush
                } else {
                    primary_kind
                };

                // Alternate foreground/background
                let fg = seed % 3 == 0;
                
                scenery.push(spawn_scen(p, offset_x, kind, fg));
            }
        }
    }
    scenery
}

pub fn build_map(idx: usize) -> Vec<Platform> {
    // 1280x720 Arena Bounds
    let mut p = vec![
        Platform { center: vec2(640.0, -10.0), size: vec2(1300.0, 20.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // 0: Top
        Platform { center: vec2(640.0, 730.0), size: vec2(1300.0, 20.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // 1: Bottom (Floor)
        Platform { center: vec2(-10.0, 360.0), size: vec2(20.0, 740.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 },  // 2: Left
        Platform { center: vec2(1290.0, 360.0), size: vec2(20.0, 740.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // 3: Right
    ];
    match idx {
        0 => {
            // Forest - Custom Level (hardcoded from custom_1.json)
            p.push(Platform { center: vec2(40.0, 480.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(280.0, 600.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(100.0, 600.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(240.0, 360.0), size: vec2(240.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(380.0, 440.0), size: vec2(322.4903, 24.0), angle: -0.5191461, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 520.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(700.0, 400.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(670.0, 720.0), size: vec2(1220.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(30.0, 720.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(480.0, 620.0), size: vec2(126.491104, 24.0), angle: 0.32175055, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(760.0, 640.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(850.0, 560.0), size: vec2(220.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1020.0, 460.0), size: vec2(240.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1080.0, 590.0), size: vec2(212.60292, 24.0), angle: -0.71883, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1210.0, 640.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1270.0, 640.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1250.0, 120.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1280.0, 170.0), size: vec2(128.06248, 24.0), angle: 0.8960554, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1180.0, 200.0), size: vec2(160.0, 24.0), angle: 1.5707964, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(670.0, 640.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 360.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 120.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(380.0, 120.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(700.0, 120.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(880.0, 120.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1110.0, 120.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(900.0, 320.0), size: vec2(322.4903, 24.0), angle: 0.5191461, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1070.0, 280.0), size: vec2(220.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1080.0, 200.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1040.0, 160.0), size: vec2(80.0, 24.0), angle: -1.5707964, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(400.0, 240.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(110.0, 200.0), size: vec2(272.02942, 24.0), angle: 0.6287963, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(680.0, 240.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(950.0, 170.0), size: vec2(205.9126, 24.0), angle: -0.5070985, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(50.0, 280.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(200.0, 440.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
        }
        1 => {
            // Desert - Custom Level (hardcoded from custom_2.json)
            p.push(Platform { center: vec2(160.0, 600.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(420.0, 600.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(680.0, 600.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(920.0, 600.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1140.0, 600.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(280.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(560.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(800.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1040.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1240.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(160.0, 720.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(420.0, 720.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(680.0, 720.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(920.0, 720.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1140.0, 720.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 280.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(230.0, 320.0), size: vec2(384.18747, 24.0), angle: -0.67474097, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 380.0), size: vec2(360.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(650.0, 260.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(780.0, 180.0), size: vec2(178.88544, 24.0), angle: 0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(490.0, 280.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(240.0, 480.0), size: vec2(144.22205, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(400.0, 480.0), size: vec2(144.22205, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(560.0, 480.0), size: vec2(144.22205, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(700.0, 480.0), size: vec2(144.22205, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(840.0, 490.0), size: vec2(100.0, 24.0), angle: 0.6435011, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(770.0, 440.0), size: vec2(72.11102, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(940.0, 520.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(820.0, 320.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1100.0, 400.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1130.0, 490.0), size: vec2(116.61904, 24.0), angle: 0.5404195, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1220.0, 160.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(900.0, 320.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(120.0, 180.0), size: vec2(89.44272, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(240.0, 160.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(410.0, 100.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(610.0, 80.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
        }
        2 => {
            // Cyber - Hardcoded from custom_3.json
            p.push(Platform { center: vec2(660.0, 720.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(480.0, 240.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(780.0, 240.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(280.0, 340.0), size: vec2(312.40997, 24.0), angle: -0.69473827, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1040.0, 300.0), size: vec2(341.76016, 24.0), angle: 0.35877067, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1240.0, 440.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1080.0, 540.0), size: vec2(312.40997, 24.0), angle: -0.69473827, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1150.0, 640.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1190.0, 560.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1250.0, 720.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(800.0, 640.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(840.0, 560.0), size: vec2(240.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(880.0, 480.0), size: vec2(320.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(820.0, 400.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(950.0, 410.0), size: vec2(63.245552, 24.0), angle: 0.32175055, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(780.0, 320.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(860.0, 320.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1010.0, 370.0), size: vec2(278.56778, 24.0), angle: 0.36717382, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(300.0, 720.0), size: vec2(600.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(970.0, 720.0), size: vec2(500.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(490.0, 340.0), size: vec2(145.6022, 24.0), angle: -0.27829966, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(390.0, 440.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(490.0, 440.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(310.0, 360.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1060.0, 140.0), size: vec2(304.63092, 24.0), angle: -0.4048918, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1220.0, 200.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1240.0, 280.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(780.0, 160.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(640.0, 90.0), size: vec2(400.4997, 24.0), angle: 0.049958397, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 80.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(200.0, 80.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(350.0, 80.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 240.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(240.0, 240.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(120.0, 300.0), size: vec2(144.22205, 24.0), angle: -0.98279375, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(50.0, 420.0), size: vec2(107.70329, 24.0), angle: 0.38050637, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(130.0, 620.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 620.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(80.0, 620.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(230.0, 440.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(300.0, 540.0), size: vec2(200.0, 24.0), angle: 1.5707964, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(360.0, 500.0), size: vec2(120.0, 24.0), angle: 1.5707964, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(430.0, 640.0), size: vec2(260.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(490.0, 550.0), size: vec2(152.31546, 24.0), angle: -0.4048918, is_jump_pad: false, iced_timer: 0.0 });
        }
        3 => {
            // Ocean - Replaced with custom_1.json
            p.push(Platform { center: vec2(640.0, 720.0), size: vec2(1280.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(457.4389, 640.4969), size: vec2(125.12617, 24.0), angle: -0.007942262, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(822.5611, 640.4969), size: vec2(125.12614, 24.0), angle: 0.007942264, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(640.0, 640.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(230.0, 570.0), size: vec2(384.70767, 24.0), angle: -0.48689923, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1000.0, 560.0), size: vec2(288.4441, 24.0), angle: 0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(101.0943, 559.5032), size: vec2(42.200302, 24.0), angle: -0.023554057, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(270.0, 480.0), size: vec2(340.0, 24.0), angle: -0.48995733, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 480.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(380.0, 650.0), size: vec2(44.72136, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(900.0, 650.0), size: vec2(44.72136, 24.0), angle: 0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(730.0, 640.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(550.0, 640.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1080.0, 440.0), size: vec2(240.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1240.0, 640.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1140.0, 520.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 350.0), size: vec2(134.16408, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(750.0, 320.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(730.0, 420.0), size: vec2(72.11102, 24.0), angle: -0.5880026, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(940.0, 360.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1180.0, 360.0), size: vec2(200.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 440.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(90.0, 360.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 280.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(300.0, 360.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(300.0, 200.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(90.0, 200.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(30.0, 360.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(32.40979, 80.24844), size: vec2(64.82149, 24.0), angle: 0.007665546, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(170.0, 110.0), size: vec2(228.03508, 24.0), angle: 0.26625204, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(430.0, 280.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(550.0, 520.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(730.0, 520.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(900.0, 280.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1073.3179, 239.13043), size: vec2(167.8787, 24.0), angle: -0.5085303, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1066.9498, 109.5031), size: vec2(158.3206, 24.0), angle: 0.39548415, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1210.0, 200.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1200.0, 280.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 200.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(422.14206, 120.00001), size: vec2(124.284, 24.0), angle: 1.2277356e-7, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(860.0, 90.0), size: vec2(280.71338, 24.0), angle: -0.071307465, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(559.90686, 112.91925), size: vec2(157.98334, 24.0), angle: -0.099237755, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(750.0, 200.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1011.4086, 168.81989), size: vec2(100.09873, 24.0), angle: 0.1419514, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1220.0, 60.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
        }
        4 => {
            // Ice - Replaced with custom_2.json
            p.push(Platform { center: vec2(240.0, 720.0), size: vec2(320.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(460.0, 720.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(640.0, 720.0), size: vec2(240.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(820.0, 720.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1040.0, 720.0), size: vec2(320.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(563.6089, 619.25464), size: vec2(96.58353, 24.0), angle: -0.44403046, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(716.7637, 619.25464), size: vec2(95.91137, 24.0), angle: 0.447367, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(640.0, 600.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1160.0, 540.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(122.32831, 560.6211), size: vec2(84.66573, 24.0), angle: 0.014673636, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1016.8917, 539.3789), size: vec2(95.573044, 24.0), angle: 0.44618365, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(240.0, 600.0), size: vec2(178.88544, 24.0), angle: 0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1050.0, 650.0), size: vec2(152.31546, 24.0), angle: -0.4048918, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1160.0, 620.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1240.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(640.0, 360.0), size: vec2(1120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 720.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
            p.push(Platform { center: vec2(133.87662, 641.98755), size: vec2(107.82654, 24.0), angle: 0.03687516, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(190.0, 650.0), size: vec2(28.284271, 24.0), angle: 0.7853982, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 220.0), size: vec2(89.44272, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(480.0, 240.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(720.0, 200.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(940.0, 130.0), size: vec2(44.72136, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1120.0, 100.0), size: vec2(40.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1250.0, 80.0), size: vec2(60.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1180.0, 260.0), size: vec2(203.96078, 24.0), angle: -0.19739556, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(720.0, 460.0), size: vec2(89.44272, 24.0), angle: -0.4636476, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(940.0, 600.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(930.0, 520.0), size: vec2(100.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1110.0, 440.0), size: vec2(180.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(320.0, 500.0), size: vec2(164.92422, 24.0), angle: -0.24497867, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(400.0, 560.0), size: vec2(164.92422, 24.0), angle: -0.24497867, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(150.0, 460.0), size: vec2(140.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(540.0, 460.0), size: vec2(120.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(920.0, 440.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(360.0, 420.0), size: vec2(160.0, 24.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(1022.39813, 279.75156), size: vec2(124.79726, 24.0), angle: -0.003981566, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(180.0, 110.0), size: vec2(82.46211, 24.0), angle: -0.24497867, is_jump_pad: false, iced_timer: 0.0 });
            p.push(Platform { center: vec2(40.0, 180.0), size: vec2(80.0, 24.0), angle: 0.0, is_jump_pad: true, iced_timer: 0.0 });
        }
        _ => {}
    }
    p
}

pub fn build_spawns(idx: usize) -> Vec<Vec2> {
    match idx {
        0 => {
            // Forest spawns (from custom_1.json)
            vec![
                vec2(176.5, 81.5),
                vec2(1085.0, 165.00002),
                vec2(1056.0, 560.5),
                vec2(124.5, 683.0),
            ]
        }
        1 => {
            // Desert spawns (from custom_2.json) - 4 spawn points spread across the level
            vec![
                vec2(160.0, 540.0),
                vec2(1140.0, 540.0),
                vec2(420.0, 660.0),
                vec2(920.0, 660.0),
            ]
        }
        2 => {
            // Cyber spawns (from custom_3.json)
            vec![
                vec2(466.989, 189.375),
                vec2(779.2528, 197.8125),
                vec2(374.15387, 595.3125),
                vec2(805.50916, 599.0625),
            ]
        }
        3 => {
            // New Ocean Spawns (copied from Forest as they are for custom_1.json)
            vec![
                vec2(176.5, 81.5),
                vec2(1085.0, 165.00002),
                vec2(1056.0, 560.5),
                vec2(124.5, 683.0),
            ]
        }
        4 => {
            // New Ice Spawns (from custom_2.json)
            vec![
                vec2(163.9115, 317.51553),
                vec2(1160.7916, 319.75156),
                vec2(118.46332, 520.2484),
                vec2(1157.8114, 503.85095),
            ]
        }
        _ => {
            // Default spawns for other levels
            vec![
                vec2(640.0 * 0.1, 360.0 * 0.3), vec2(640.0 * 0.9, 360.0 * 0.3),
                vec2(640.0 * 0.1, 360.0 * 0.7), vec2(640.0 * 0.9, 360.0 * 0.7),
            ]
        }
    }
}

// ---------- Level Export/Import ----------

pub fn export_level(platforms: &[Platform], spawns: &[Vec2], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = get_levels_dir();
    fs::create_dir_all(&dir)?;
    
    // Skip boundary platforms (first 4)
    let user_platforms: Vec<PlatformData> = platforms.iter()
        .skip(4)
        .map(|p| PlatformData::from(*p))
        .collect();
    
    let spawn_data: Vec<[f32; 2]> = spawns.iter().map(|s| [s.x, s.y]).collect();
    
    let level_data = LevelData {
        platforms: user_platforms,
        spawns: spawn_data,
    };
    
    let json = serde_json::to_string_pretty(&level_data)?;
    let path = dir.join(format!("{}.json", name));
    fs::write(path, json)?;
    
    Ok(())
}

pub fn import_level(name: &str) -> Result<(Vec<Platform>, Vec<Vec2>), Box<dyn std::error::Error>> {
    let path = get_levels_dir().join(format!("{}.json", name));
    let json = fs::read_to_string(path)?;
    let level_data: LevelData = serde_json::from_str(&json)?;
    
    // Build the full platform list with arena boundaries
    let mut platforms = vec![
        Platform { center: vec2(640.0, -10.0), size: vec2(1300.0, 20.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // Top
        Platform { center: vec2(640.0, 730.0), size: vec2(1300.0, 20.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // Bottom (Floor)
        Platform { center: vec2(-10.0, 360.0), size: vec2(20.0, 740.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 },  // Left
        Platform { center: vec2(1290.0, 360.0), size: vec2(20.0, 740.0), angle: 0.0, is_jump_pad: false, iced_timer: 0.0 }, // Right
    ];
    
    for data in level_data.platforms {
        platforms.push(Platform::from(data));
    }
    
    let spawns: Vec<Vec2> = level_data.spawns.iter().map(|s| vec2(s[0], s[1])).collect();
    
    Ok((platforms, spawns))
}

pub fn list_custom_levels() -> Vec<String> {
    let mut levels = Vec::new();
    if let Ok(entries) = fs::read_dir(get_levels_dir()) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if let Some(name) = filename.strip_suffix(".json") {
                    levels.push(name.to_string());
                }
            }
        }
    }
    levels.sort();
    levels
}
