use crate::types::MapTheme;
use macroquad::prelude::*;
use std::cell::RefCell;

// ---------- UI Helpers ----------

// Track if a slider is currently being dragged
thread_local! {
    static SLIDER_DRAGGING: RefCell<bool> = RefCell::new(false);
}

pub fn mouse_hit(mpos: Vec2, x: f32, y: f32, w: f32, h: f32) -> bool {
    mpos.x >= x && mpos.x <= x + w && mpos.y >= y && mpos.y <= y + h
}

pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, _r: f32, col: Color) {
    // Pixel-art style "rounded" rect just has the corners clipped or is just a rect
    draw_rectangle(x, y, w, h, col);
    // Optional: Draw a 1-pixel detail at corners or just keep it sharp
}

pub fn draw_m3_btn(
    mpos: Vec2,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    sel: bool,
    font_sz: f32,
    theme: &MapTheme,
) -> bool {
    let hover = mouse_hit(mpos, x, y, w, h);
    let clicked = hover && is_mouse_button_down(MouseButton::Left);

    let y_offset = if clicked { 4.0 } else { 0.0 };
    let shadow_h = 6.0;

    let mut bg = if sel {
        theme.plat_top
    } else if hover {
        Color::new(
            (theme.plat_top.r + 0.15).min(1.0),
            (theme.plat_top.g + 0.15).min(1.0),
            (theme.plat_top.b + 0.15).min(1.0),
            1.0,
        )
    } else {
        Color::new(
            theme.plat_top.r * 0.85,
            theme.plat_top.g * 0.85,
            theme.plat_top.b * 0.85,
            1.0,
        )
    };

    if sel && !clicked {
        let s = (get_time() * 5.0).sin() as f32 * 0.1 + 0.9;
        bg.r *= s;
        bg.g *= s;
        bg.b *= s;
    }

    let brightness = bg.r * 0.299 + bg.g * 0.587 + bg.b * 0.114;
    let text_col = if brightness > 0.6 {
        Color::new(0.05, 0.05, 0.05, 1.0)
    } else {
        Color::new(0.98, 0.98, 0.98, 1.0)
    };

    // Shadow
    if !clicked {
        draw_rectangle(x, y + shadow_h, w, h, theme.plat_shadow);
    }

    // Main body
    draw_rectangle(x, y + y_offset, w, h, bg);

    // Outline
    draw_rectangle_lines(x, y + y_offset, w, h, 2.0, theme.plat_shadow);

    // Hover glow
    if hover && !clicked {
        draw_rectangle(x, y + y_offset, w, h, Color::new(1.0, 1.0, 1.0, 0.1));
    }

    let td = measure_text(label, None, font_sz as u16, 1.0);
    draw_text(
        label,
        x + (w - td.width) / 2.0,
        y + y_offset + (h + td.height) / 2.0 - 2.0,
        font_sz,
        text_col,
    );

    hover && is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_slider(
    mpos: Vec2,
    x: f32,
    y: f32,
    w: f32,
    val: f32,
    lo: f32,
    hi: f32,
    step: f32,
    theme: &MapTheme,
) -> f32 {
    let t = (val - lo) / (hi - lo);
    let kw = 20.0;
    let kh = 30.0;

    // Track
    draw_rectangle(x, y - 4.0, w, 8.0, theme.plat_shadow);
    draw_rectangle(x, y - 4.0, w * t, 8.0, theme.plat_top);

    let kx = x + w * t - kw / 2.0;
    let ky = y - kh / 2.0;
    let is_hovering = mpos.x >= kx && mpos.x <= kx + kw && mpos.y >= ky && mpos.y <= ky + kh;

    // Knob shadow
    draw_rectangle(kx + 2.0, ky + 2.0, kw, kh, theme.plat_shadow);

    // Knob body
    let knob_col = if is_hovering { WHITE } else { theme.plat_top };
    draw_rectangle(kx, ky, kw, kh, knob_col);
    draw_rectangle_lines(kx, ky, kw, kh, 2.0, theme.plat_shadow);

    // Handle slider dragging with off-screen support
    let mut result = val;

    SLIDER_DRAGGING.with(|dragging| {
        let mut is_dragging = dragging.borrow_mut();

        // Start dragging if we're hovering and button pressed
        if is_hovering && is_mouse_button_pressed(MouseButton::Left) {
            *is_dragging = true;
        }

        // Stop dragging if button released
        if !is_mouse_button_down(MouseButton::Left) {
            *is_dragging = false;
        }

        // Update slider value while dragging (even off-screen)
        if *is_dragging && is_mouse_button_down(MouseButton::Left) {
            let raw = ((mpos.x - x) / w).clamp(0.0, 1.0);
            let v = lo + raw * (hi - lo);
            result = ((v / step).round() * step).clamp(lo, hi);
        }
    });

    result
}
