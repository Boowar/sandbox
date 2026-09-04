use crate::sim::{Terrain, H, Sim, W};
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const CELL: f64 = 8.0;
const ART: usize = 4;
const PX: usize = 2;
const PW: usize = W * ART * PX;
const PH: usize = H * ART * PX;

pub struct Fx {
    pub x: f64,
    pub y: f64,
    pub life: f32,
}

fn hash2(x: i32, y: i32) -> u32 {
    let mut h = (x as u32).wrapping_mul(0x9E37_79B9) ^ (y as u32).wrapping_mul(0x85EB_CA6B);
    h ^= h >> 16;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 13;
    h = h.wrapping_mul(0x27D4_EB2F);
    h ^= h >> 16;
    h
}

fn set_px(buf: &mut [u8], ax: usize, ay: usize, r: u8, g: u8, b: u8) {
    for py in 0..PX {
        for px in 0..PX {
            let x = ax * PX + px;
            let y = ay * PX + py;
            let i = (y * PW + x) * 4;
            buf[i] = r;
            buf[i + 1] = g;
            buf[i + 2] = b;
            buf[i + 3] = 255;
        }
    }
}

fn fill(buf: &mut [u8], ax: usize, ay: usize, w: usize, hgt: usize, r: u8, g: u8, b: u8) {
    for dy in 0..hgt {
        for dx in 0..w {
            set_px(buf, ax + dx, ay + dy, r, g, b);
        }
    }
}

fn paint_water(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 28, 92, 140);
    let h = hash2(x, y);
    if h % 3 == 0 {
        fill(buf, ax + 1, ay + 1, 2, 2, 36, 112, 168);
    }
    if (h >> 4) % 4 == 0 {
        set_px(buf, ax + (h >> 6) as usize % 4, ay + (h >> 8) as usize % 4, 46, 130, 186);
    }
}

fn paint_grass(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 76, 140, 58);
    let h = hash2(x, y);
    fill(buf, ax + ((h >> 3) % 2) as usize * 2, ay + ((h >> 5) % 2) as usize * 2, 2, 2, 60, 113, 45);
    set_px(buf, ax + (h >> 7) as usize % 4, ay + (h >> 9) as usize % 4, 100, 172, 76);
    if (h >> 11) % 7 == 0 {
        set_px(buf, ax + (h >> 13) as usize % 4, ay + (h >> 15) as usize % 4, 248, 236, 138);
    }
    if (h >> 17) % 11 == 0 {
        set_px(buf, ax + (h >> 19) as usize % 4, ay + (h >> 21) as usize % 4, 246, 168, 190);
    }
}

fn paint_forest(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 42, 92, 32);
    let h = hash2(x, y);
    fill(buf, ax + (h % 2) as usize, ay, 3, 2, 50, 110, 38);
    fill(buf, ax + 2, ay + 2, 2, 2, 64, 128, 46);
    set_px(buf, ax + (h >> 3) as usize % 4, ay + (h >> 5) as usize % 4, 32, 72, 26);
    if (h >> 7) % 5 == 0 {
        set_px(buf, ax + (h >> 9) as usize % 4, ay + (h >> 11) as usize % 4, 84, 150, 58);
    }
}

fn paint_hills(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 96, 94, 100);
    let h = hash2(x, y);
    fill(buf, ax + ((h >> 3) % 2) as usize * 2, ay + ((h >> 5) % 2) as usize * 2, 2, 2, 76, 74, 82);
    fill(buf, ax + 1, ay + 1, 2, 2, 112, 110, 118);
    if (h >> 7) % 3 == 0 {
        set_px(buf, ax + (h >> 9) as usize % 3, ay + (h >> 11) as usize % 3, 132, 130, 138);
    }
}

pub fn draw_terrain(ctx: &CanvasRenderingContext2d, sim: &Sim) {
    let mut buf = vec![0u8; PW * PH * 4];
    for y in 0..H {
        for x in 0..W {
            let (ax, ay) = (x * ART, y * ART);
            match sim.grid[y * W + x].terrain {
                Terrain::Water => paint_water(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Grass => paint_grass(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Forest => paint_forest(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Hills => paint_hills(&mut buf, ax, ay, x as i32, y as i32),
            }
        }
    }
    for y in 0..H {
        for x in 0..W {
            if sim.grid[y * W + x].terrain != Terrain::Water {
                continue;
            }
            let (ax, ay) = (x * ART, y * ART);
            let is_water = |xx: i32, yy: i32| -> bool {
                xx >= 0 && xx < W as i32 && yy >= 0 && yy < H as i32 && sim.grid[yy as usize * W + xx as usize].terrain == Terrain::Water
            };
            if !is_water(x as i32, y as i32 - 1) {
                fill(&mut buf, ax, ay, ART, 1, 206, 184, 96);
            }
            if !is_water(x as i32, y as i32 + 1) {
                fill(&mut buf, ax, ay + ART - 1, ART, 1, 206, 184, 96);
            }
            if !is_water(x as i32 - 1, y as i32) {
                fill(&mut buf, ax, ay, 1, ART, 206, 184, 96);
            }
            if !is_water(x as i32 + 1, y as i32) {
                fill(&mut buf, ax + ART - 1, ay, 1, ART, 206, 184, 96);
            }
        }
    }
    let img = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buf[..]), PW as u32, PH as u32)
        .expect("image data");
    let _ = ctx.put_image_data(&img, 0.0, 0.0);
}

fn shade(r: u8, g: u8, b: u8, f: f32) -> String {
    format!(
        "rgb({},{},{})",
        (r as f32 * f).clamp(0.0, 255.0) as u8,
        (g as f32 * f).clamp(0.0, 255.0) as u8,
        (b as f32 * f).clamp(0.0, 255.0) as u8
    )
}

fn draw_house(ctx: &CanvasRenderingContext2d, ox: f64, oy: f64, r: u8, g: u8, b: u8) {
    ctx.set_fill_style_str("rgba(0,0,0,0.18)");
    ctx.fill_rect(ox - 1.0, oy + 6.0, 12.0, 2.0);
    ctx.set_fill_style_str("rgb(96,80,50)");
    ctx.fill_rect(ox - 1.0, oy + 4.0, 12.0, 2.0);
    ctx.set_fill_style_str(&shade(r, g, b, 0.55));
    ctx.fill_rect(ox, oy - 2.0, 10.0, 6.0);
    ctx.set_fill_style_str("rgb(38,34,30)");
    ctx.fill_rect(ox + 4.0, oy - 1.0, 2.0, 5.0);
    let lr = shade(r, g, b, 1.35);
    ctx.set_fill_style_str(&lr);
    ctx.fill_rect(ox + 4.0, oy - 6.0, 2.0, 1.0);
    ctx.fill_rect(ox + 3.0, oy - 5.0, 4.0, 1.0);
    ctx.fill_rect(ox + 2.0, oy - 4.0, 6.0, 1.0);
    ctx.fill_rect(ox + 1.0, oy - 3.0, 8.0, 1.0);
    let br = shade(r, g, b, 1.8);
    ctx.set_fill_style_str(&br);
    ctx.fill_rect(ox + 9.0, oy - 10.0, 2.0, 3.0);
    ctx.set_fill_style_str("rgb(105,105,110)");
    ctx.fill_rect(ox + 11.0, oy - 9.0, 1.0, 7.0);
}

fn draw_agent(ctx: &CanvasRenderingContext2d, cx: f64, cy: f64, r: u8, g: u8, b: u8, facing: i32, leg: usize) {
    ctx.set_fill_style_str("rgba(0,0,0,0.15)");
    ctx.fill_rect(cx, cy + 6.0, 4.0, 1.0);
    let lift = if leg == 0 { 0 } else { 1 };
    ctx.set_fill_style_str("rgb(31,35,40)");
    ctx.fill_rect(cx, cy + 5.0 - lift as f64, 1.0, 2.0);
    ctx.fill_rect(cx + 3.0, cy + 4.0 + lift as f64, 1.0, 2.0);
    ctx.set_fill_style_str(&shade(r, g, b, 0.7));
    ctx.fill_rect(cx, cy + 2.0, 4.0, 3.0);
    ctx.set_fill_style_str("rgb(24,25,29)");
    ctx.fill_rect(cx, cy + 4.0, 4.0, 1.0);
    let hd = shade(r, g, b, 1.25);
    ctx.set_fill_style_str(&hd);
    if facing == 0 {
        ctx.fill_rect(cx + 1.0, cy, 2.0, 3.0);
    } else {
        ctx.fill_rect(cx + 0.5, cy, 2.0, 3.0);
    }
    ctx.set_fill_style_str("rgb(255,255,255)");
    if facing == 0 {
        ctx.fill_rect(cx + 1.0, cy + 1.0, 1.0, 1.0);
        ctx.fill_rect(cx + 2.0, cy + 1.0, 1.0, 1.0);
    } else if facing > 0 {
        ctx.fill_rect(cx + 1.0, cy + 1.0, 1.0, 1.0);
        ctx.fill_rect(cx + 3.0, cy + 1.0, 1.0, 1.0);
    } else {
        ctx.fill_rect(cx, cy + 1.0, 1.0, 1.0);
        ctx.fill_rect(cx + 2.0, cy + 1.0, 1.0, 1.0);
    }
}

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    terrain: &HtmlCanvasElement,
    sim: &Sim,
    tick: u64,
    paused: bool,
    speed: f64,
    fps: f64,
    fxs: &[Fx],
    zoom: f64,
    cam_x: f64,
    cam_y: f64,
) {
    let cw = W as f64 * CELL;
    let ch = H as f64 * CELL;
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    ctx.set_fill_style_str("rgb(10,14,18)");
    ctx.fill_rect(0.0, 0.0, cw, ch);
    let tx = cw / 2.0 - cam_x * zoom;
    let ty = ch / 2.0 - cam_y * zoom;
    let _ = ctx.set_transform(zoom, 0.0, 0.0, zoom, tx, ty);
    ctx.set_image_smoothing_enabled(false);
    let _ = ctx.draw_image_with_html_canvas_element(terrain, 0.0, 0.0);

    let phase = (tick / 2) as usize % 6;
    ctx.set_fill_style_str("rgba(195,228,255,0.35)");
    for y in 0..H {
        for x in 0..W {
            if sim.grid[y * W + x].terrain != Terrain::Water {
                continue;
            }
            let p = ((x as usize * 7 + y as usize * 13 + phase) % 4) as f64;
            ctx.fill_rect(
                x as f64 * CELL + p - 1.0,
                (y as f64 + 0.35) * CELL + (x % 2) as f64,
                2.0,
                1.0,
            );
        }
    }

    ctx.set_fill_style_str("rgba(215,205,140,0.5)");
    for y in 0..H {
        for x in 0..W {
            let c = &sim.grid[y * W + x];
            if c.terrain == Terrain::Forest && c.food < 3.0 {
                ctx.fill_rect(x as f64 * CELL, y as f64 * CELL, CELL, CELL);
            }
        }
    }

    ctx.set_fill_style_str("rgb(228,190,84)");
    for y in 0..H {
        for x in 0..W {
            let c = &sim.grid[y * W + x];
            if c.terrain == Terrain::Hills && c.ore > 0.5 {
                ctx.fill_rect(x as f64 * CELL + 1.0, y as f64 * CELL + 3.0, 2.0, 2.0);
                ctx.fill_rect(x as f64 * CELL + 4.0, y as f64 * CELL + 5.0, 2.0, 2.0);
            }
        }
    }

    for (i, t) in sim.towns.iter().enumerate() {
        draw_house(ctx, t.x as f64 * CELL - 3.0, t.y as f64 * CELL - 2.0, t.r, t.g, t.b);
        ctx.set_font("11px ui-monospace, monospace");
        ctx.set_fill_style_str("rgb(238,243,247)");
        let _ = ctx.fill_text(&sim.pop(i).to_string(), t.x as f64 * CELL + 13.0, t.y as f64 * CELL - 4.0);
    }

    for (i, a) in sim.agents.iter().enumerate() {
        let t = &sim.towns[a.home];
        let fx = a.x as f64 * CELL + 2.0;
        let fy = a.y as f64 * CELL + 1.0;
        draw_agent(ctx, fx, fy, t.r, t.g, t.b, a.dir_x.clamp(-1, 1), i & 1);
        if let Some((kind, _)) = a.carry {
            let col = match kind {
                crate::sim::ResourceKind::Food => "rgb(126,231,135)",
                crate::sim::ResourceKind::Water => "rgb(88,166,255)",
                crate::sim::ResourceKind::Ore => "rgb(228,190,84)",
            };
            ctx.set_fill_style_str(col);
            ctx.fill_rect(fx + 1.0, fy - 1.0, 3.0, 1.0);
        }
    }

    for e in fxs {
        let k = (e.life / 26.0).clamp(0.0, 1.0) as f64;
        let rad = 2.0 + (1.0 - k) * 9.0;
        ctx.set_fill_style_str(&format!(
            "rgba(234,246,140,{:.2})",
            k * 0.85
        ));
        for (dx, dy) in [(rad, 0.0), (-rad, 0.0), (0.0, rad), (0.0, -rad)] {
            ctx.fill_rect(e.x + dx - 1.0, e.y + dy - 1.0, 2.0, 2.0);
        }
        ctx.set_fill_style_str("rgba(255,255,255,0.85)");
        ctx.fill_rect(e.x - 1.0, e.y - 1.0, 2.0, 2.0);
    }

    let food: f32 = sim.towns.iter().map(|t| t.stocks.food).sum();
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let water: f32 = sim.towns.iter().map(|t| t.stocks.water).sum();
    let ore: f32 = sim.towns.iter().map(|t| t.stocks.ore).sum();
    let lines = [
        format!("tick {}", sim.tick_count),
        format!("pop {}  food {}  water {}  ore {}", sim.agents.len(), food as i32, water as i32, ore as i32),
        format!("fps {:.0}  speed x{:.1}{}", fps, speed, if paused { "  [PAUSED]" } else { "" }),
        String::new(),
        "Space: пауза   B: 🌱  R: мир".to_string(),
    ];
    ctx.set_fill_style_str("rgba(10,14,18,0.72)");
    ctx.fill_rect(4.0, 4.0, 300.0, 14.0 + lines.len() as f64 * 15.0);
    ctx.set_stroke_style_str("rgb(70,78,90)");
    ctx.begin_path();
    ctx.rect(4.0, 4.0, 300.0, 14.0 + lines.len() as f64 * 15.0);
    ctx.stroke();
    ctx.set_font("13px ui-monospace, monospace");
    ctx.set_fill_style_str("rgb(238,243,247)");
    for (i, l) in lines.iter().enumerate() {
        let _ = ctx.fill_text(l, 10.0, 22.0 + i as f64 * 15.0);
    }
}