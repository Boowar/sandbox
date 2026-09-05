use crate::sim::{Role, Species, Season, Terrain, TownIdea, Weather, DAY_LEN, H, Sim, W};
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

fn paint_farm(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 158, 140, 80);
    let h = hash2(x, y);
    fill(buf, ax, ay, ART, 1, 128, 112, 62);
    fill(buf, ax, ay + 3, ART, 1, 128, 112, 62);
    set_px(buf, ax + (h >> 3) as usize % 4, ay + ((h >> 5) % 2) as usize, 96, 84, 44);
    set_px(buf, ax + (h >> 7) as usize % 4, ay + 2 + ((h >> 9) % 2) as usize, 230, 214, 130);
}

fn paint_desert(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 210, 190, 120);
    let h = hash2(x, y);
    if h % 3 == 0 {
        set_px(buf, ax + (h >> 2) as usize % 4, ay + (h >> 4) as usize % 4, 230, 210, 140);
    }
    if h % 5 == 0 {
        set_px(buf, ax + (h >> 6) as usize % 4, ay + (h >> 8) as usize % 4, 180, 160, 100);
    }
    if h % 11 == 0 {
        set_px(buf, ax + (h >> 10) as usize % 3, ay + (h >> 12) as usize % 3, 160, 140, 80);
        set_px(buf, ax + (h >> 10) as usize % 3, ay + 1, 80, 140, 60);
    }
}

fn paint_tundra(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 180, 195, 190);
    let h = hash2(x, y);
    if h % 3 == 0 {
        set_px(buf, ax + (h >> 2) as usize % 4, ay + (h >> 4) as usize % 4, 160, 175, 170);
    }
    if h % 4 == 0 {
        set_px(buf, ax + (h >> 6) as usize % 4, ay + (h >> 8) as usize % 4, 200, 210, 205);
    }
    if h % 7 == 0 {
        set_px(buf, ax + (h >> 10) as usize % 4, ay + (h >> 12) as usize % 4, 140, 160, 130);
    }
}

fn paint_jungle(buf: &mut [u8], ax: usize, ay: usize, x: i32, y: i32) {
    fill(buf, ax, ay, ART, ART, 20, 80, 20);
    let h = hash2(x, y);
    fill(buf, ax + (h % 2) as usize, ay, 3, 2, 30, 100, 28);
    fill(buf, ax + 2, ay + 2, 2, 2, 40, 115, 35);
    set_px(buf, ax + (h >> 3) as usize % 4, ay + (h >> 5) as usize % 4, 15, 65, 15);
    if (h >> 7) % 3 == 0 {
        set_px(buf, ax + (h >> 9) as usize % 4, ay + (h >> 11) as usize % 4, 60, 140, 50);
    }
    if (h >> 13) % 5 == 0 {
        set_px(buf, ax + (h >> 15) as usize % 3, ay + (h >> 17) as usize % 3, 180, 60, 40);
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
                Terrain::Farm => paint_farm(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Desert => paint_desert(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Tundra => paint_tundra(&mut buf, ax, ay, x as i32, y as i32),
                Terrain::Jungle => paint_jungle(&mut buf, ax, ay, x as i32, y as i32),
            }
        }
    }
    for y in 0..H {
        for x in 0..W {
            let (ax, ay) = (x * ART, y * ART);
            let c = &sim.grid[y * W + x];
            let h = hash2(x as i32, y as i32);
            if c.burn > 0 {
                fill(&mut buf, ax, ay, ART, ART, 92, 42, 18);
                set_px(&mut buf, ax + (h % 3) as usize, ay + ((h >> 3) % 3) as usize, 255, 128, 16);
                set_px(&mut buf, ax + ((h >> 5) % 4) as usize, ay + ((h >> 7) % 4) as usize, 255, 200, 40);
            } else if c.gold > 0.0 {
                set_px(&mut buf, ax + 1 + (h % 2) as usize, ay + 1 + ((h >> 3) % 2) as usize, 240, 210, 90);
                set_px(&mut buf, ax + 2 + ((h >> 5) % 2) as usize, ay + 2 + ((h >> 7) % 2) as usize, 255, 232, 130);
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

fn draw_ruin(ctx: &CanvasRenderingContext2d, ox: f64, oy: f64) {
    ctx.set_fill_style_str("rgb(88,93,100)");
    ctx.fill_rect(ox + 3.0, oy + 1.0, 16.0, 11.0);
    ctx.fill_rect(ox + 9.0, oy - 3.0, 8.0, 4.0);
    ctx.set_fill_style_str("rgb(48,52,58)");
    ctx.fill_rect(ox + 4.0, oy + 1.0, 14.0, 2.0);
    ctx.fill_rect(ox + 4.0, oy + 10.0, 14.0, 2.0);
    ctx.fill_rect(ox + 10.0, oy - 3.0, 6.0, 2.0);
    ctx.set_fill_style_str("rgb(66,70,76)");
    ctx.fill_rect(ox + 8.0, oy + 4.0, 2.0, 5.0);
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

fn draw_building(
    ctx: &CanvasRenderingContext2d,
    px: f64,
    py: f64,
    kind: crate::sim::BuildingKind,
    r: u8,
    g: u8,
    b: u8,
) {
    match kind {
        crate::sim::BuildingKind::House => {
            let lr = shade(r, g, b, 1.5);
            ctx.set_fill_style_str(&lr);
            ctx.fill_rect(px + 1.0, py, 5.0, 1.0);
            ctx.set_fill_style_str(&shade(r, g, b, 0.55));
            ctx.fill_rect(px + 2.0, py + 1.0, 4.0, 3.0);
            ctx.set_fill_style_str("rgb(38,34,30)");
            ctx.fill_rect(px + 3.0, py + 2.0, 2.0, 2.0);
        }
        crate::sim::BuildingKind::Well => {
            ctx.set_fill_style_str("rgb(105,105,110)");
            ctx.fill_rect(px + 1.0, py, 4.0, 1.0);
            ctx.fill_rect(px + 1.0, py + 3.0, 4.0, 2.0);
            ctx.set_fill_style_str("rgb(88,166,255)");
            ctx.fill_rect(px + 2.0, py + 3.0, 2.0, 2.0);
            ctx.set_fill_style_str("rgb(70,70,76)");
            ctx.fill_rect(px + 2.0, py + 1.0, 2.0, 2.0);
        }
        crate::sim::BuildingKind::TradePost => {
            ctx.set_fill_style_str("rgb(232,214,120)");
            ctx.fill_rect(px, py, 6.0, 1.0);
            ctx.set_fill_style_str("rgb(150,120,70)");
            ctx.fill_rect(px, py + 1.0, 6.0, 3.0);
            ctx.set_fill_style_str("rgb(232,214,120)");
            ctx.fill_rect(px, py + 3.0, 6.0, 1.0);
            ctx.set_fill_style_str("rgb(92,74,44)");
            ctx.fill_rect(px + 1.0, py + 2.0, 4.0, 2.0);
        }
        crate::sim::BuildingKind::Farm => {
            ctx.set_fill_style_str("rgb(158,140,80)");
            ctx.fill_rect(px, py + 1.0, 6.0, 2.0);
            ctx.set_fill_style_str("rgb(128,112,62)");
            ctx.fill_rect(px, py + 1.0, 6.0, 1.0);
            ctx.set_fill_style_str("rgb(230,214,130)");
            ctx.fill_rect(px + 1.0, py + 3.0, 1.0, 1.0);
            ctx.fill_rect(px + 4.0, py + 3.0, 1.0, 1.0);
        }
        crate::sim::BuildingKind::Sanctuary => {
            ctx.set_fill_style_str("rgb(118,108,130)");
            ctx.fill_rect(px, py + 3.0, 6.0, 2.0);
            ctx.set_fill_style_str("rgb(148,138,162)");
            ctx.fill_rect(px + 1.0, py + 1.0, 4.0, 3.0);
            ctx.set_fill_style_str("rgb(255,222,120)");
            ctx.fill_rect(px + 1.0, py, 4.0, 1.0);
        }
        crate::sim::BuildingKind::Clinic => {
            ctx.set_fill_style_str("rgb(230,234,238)");
            ctx.fill_rect(px, py, 6.0, 4.0);
            ctx.set_fill_style_str("rgb(64,180,170)");
            ctx.fill_rect(px, py, 1.0, 4.0);
            ctx.fill_rect(px + 5.0, py, 1.0, 4.0);
            ctx.set_fill_style_str("rgb(224,60,56)");
            ctx.fill_rect(px + 2.0, py + 1.0, 1.0, 2.0);
            ctx.fill_rect(px + 3.0, py + 1.0, 1.0, 2.0);
        }
        crate::sim::BuildingKind::Wall => {
            ctx.set_fill_style_str("rgb(120,112,104)");
            ctx.fill_rect(px, py, 6.0, 3.0);
            ctx.set_fill_style_str("rgb(88,82,76)");
            ctx.fill_rect(px, py, 1.0, 3.0);
            ctx.fill_rect(px + 5.0, py, 1.0, 3.0);
            ctx.fill_rect(px + 2.0, py, 1.0, 3.0);
            ctx.set_fill_style_str("rgb(150,142,132)");
            ctx.fill_rect(px + 1.0, py, 1.0, 1.0);
        }
        crate::sim::BuildingKind::Barracks => {
            ctx.set_fill_style_str("rgb(140,96,84)");
            ctx.fill_rect(px, py + 2.0, 6.0, 3.0);
            ctx.set_fill_style_str("rgb(104,72,62)");
            ctx.fill_rect(px, py, 6.0, 2.0);
            ctx.set_fill_style_str("rgb(210,120,120)");
            ctx.fill_rect(px + 1.0, py + 1.0, 4.0, 1.0);
            ctx.fill_rect(px + 2.0, py, 2.0, 1.0);
        }
        crate::sim::BuildingKind::University => {
            ctx.set_fill_style_str("rgb(96,74,140)");
            ctx.fill_rect(px + 1.0, py + 2.0, 4.0, 3.0);
            ctx.set_fill_style_str("rgb(60,48,92)");
            ctx.fill_rect(px, py + 1.0, 6.0, 1.0);
            ctx.set_fill_style_str("rgb(216,180,255)");
            ctx.fill_rect(px + 1.0, py, 4.0, 1.0);
        }
        crate::sim::BuildingKind::Smithy => {
            ctx.set_fill_style_str("rgb(70,66,60)");
            ctx.fill_rect(px, py, 6.0, 4.0);
            ctx.set_fill_style_str("rgb(255,160,60)");
            ctx.fill_rect(px + 1.0, py + 1.0, 2.0, 1.0);
            ctx.set_fill_style_str("rgb(190,110,40)");
            ctx.fill_rect(px + 3.0, py + 2.0, 2.0, 1.0);
        }
        crate::sim::BuildingKind::Library => {
            ctx.set_fill_style_str("rgb(110,96,70)");
            ctx.fill_rect(px, py, 6.0, 4.0);
            ctx.set_fill_style_str("rgb(232,214,160)");
            ctx.fill_rect(px + 1.0, py, 4.0, 1.0);
            ctx.fill_rect(px + 2.0, py + 1.0, 2.0, 2.0);
        }
    }
}

fn draw_caravan(ctx: &CanvasRenderingContext2d, cx: f64, cy: f64) {
    ctx.set_fill_style_str("rgba(0,0,0,0.15)");
    ctx.fill_rect(cx, cy + 4.0, 7.0, 1.0);
    ctx.set_fill_style_str("rgb(150,120,70)");
    ctx.fill_rect(cx, cy + 2.0, 7.0, 2.0);
    ctx.set_fill_style_str("rgb(120,96,56)");
    ctx.fill_rect(cx, cy + 2.0, 1.0, 2.0);
    ctx.fill_rect(cx + 6.0, cy + 2.0, 1.0, 2.0);
    ctx.set_fill_style_str("rgb(255,222,120)");
    ctx.fill_rect(cx + 2.0, cy + 1.0, 3.0, 1.0);
    ctx.fill_rect(cx + 3.0, cy, 1.0, 1.0);
}

fn draw_scaffold(
    ctx: &CanvasRenderingContext2d,
    px: f64,
    py: f64,
    progress: f32,
    cost: f32,
    r: u8,
    g: u8,
    b: u8,
) {
    ctx.set_fill_style_str("rgba(0,0,0,0.15)");
    ctx.fill_rect(px, py + 6.0, 7.0, 2.0);
    ctx.set_fill_style_str("rgb(150,120,70)");
    ctx.fill_rect(px, py, 7.0, 1.0);
    ctx.fill_rect(px, py + 4.0, 7.0, 1.0);
    ctx.fill_rect(px, py, 1.0, 5.0);
    ctx.fill_rect(px + 6.0, py, 1.0, 5.0);
    let k = (progress / cost).clamp(0.0, 1.0) as f64;
    ctx.set_fill_style_str(&shade(r, g, b, 1.1));
    ctx.fill_rect(px + 1.0, py + 1.0, 5.0 * k, 1.0);
}

fn draw_animal(
    ctx: &CanvasRenderingContext2d,
    cx: f64,
    cy: f64,
    species: Species,
    dir_x: i32,
    domestic: bool,
) {
    match species {
        Species::Deer => {
            ctx.set_fill_style_str("rgb(196,128,74)");
            ctx.fill_rect(cx, cy + 1.0, 4.0, 2.0);
            ctx.fill_rect(cx + 3.0, cy, 2.0, 2.0);
            ctx.set_fill_style_str("rgb(150,96,54)");
            ctx.fill_rect(cx + 1.0, cy + 2.0, 4.0, 1.0);
            ctx.set_fill_style_str("rgb(240,214,168)");
            ctx.fill_rect(cx + 1.0, cy + 1.0, 1.0, 1.0);
        }
        Species::Boar => {
            ctx.set_fill_style_str("rgb(122,102,74)");
            ctx.fill_rect(cx - 1.0, cy, 5.0, 3.0);
            ctx.set_fill_style_str("rgb(94,76,54)");
            ctx.fill_rect(cx - 1.0, cy + 1.0, 5.0, 1.0);
            ctx.set_fill_style_str("rgb(196,164,120)");
            ctx.fill_rect(cx + 1.0, cy + 2.0, 2.0, 1.0);
        }
        Species::Wolf => {
            ctx.set_fill_style_str("rgb(116,124,134)");
            ctx.fill_rect(cx, cy + 1.0, 5.0, 2.0);
            ctx.fill_rect(cx + 3.0, cy, 2.0, 3.0);
            ctx.set_fill_style_str("rgb(84,92,102)");
            ctx.fill_rect(cx, cy + 2.0, 5.0, 1.0);
            ctx.set_fill_style_str("rgb(238,240,244)");
            ctx.fill_rect(cx + 4.0, cy, 1.0, 1.0);
        }
        Species::Cow => {
            ctx.set_fill_style_str("rgb(238,238,244)");
            ctx.fill_rect(cx - 1.0, cy, 6.0, 3.0);
            ctx.set_fill_style_str("rgb(40,38,48)");
            ctx.fill_rect(cx, cy, 1.0, 1.0);
            ctx.fill_rect(cx + 3.0, cy + 2.0, 1.0, 1.0);
            ctx.fill_rect(cx, cy + 2.0, 1.0, 1.0);
            ctx.fill_rect(cx + 4.0, cy + 1.0, 1.0, 1.0);
            ctx.set_fill_style_str("rgb(172,118,96)");
            ctx.fill_rect(cx - 1.0, cy + 1.0, 1.0, 2.0);
        }
    }
    let _ = dir_x;
    if domestic {
        ctx.set_fill_style_str("rgb(255,222,120)");
        ctx.fill_rect(cx + 1.0, cy - 1.0, 2.0, 1.0);
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
    build_flash: Option<(usize, f64)>,
    selected_town: Option<usize>,
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
            if (c.terrain == Terrain::Forest || c.terrain == Terrain::Jungle) && c.food < 3.0 {
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

    let town_col: Vec<(u8, u8, u8)> = sim
        .towns
        .iter()
        .map(|t| {
            t.empire
                .and_then(|e| sim.empires.get(e))
                .map(|emp| (emp.r, emp.g, emp.b))
                .unwrap_or((t.r, t.g, t.b))
        })
        .collect();

    for (i, t) in sim.towns.iter().enumerate() {
        if !t.alive {
            draw_ruin(ctx, t.x as f64 * CELL - 3.0, t.y as f64 * CELL - 2.0);
            ctx.set_fill_style_str("rgb(238,90,74)");
            ctx.fill_rect(t.x as f64 * CELL + 8.0, t.y as f64 * CELL - 8.0, 2.0, 6.0);
            ctx.fill_rect(t.x as f64 * CELL + 4.0, t.y as f64 * CELL - 4.0, 10.0, 2.0);
            ctx.set_font("11px ui-monospace, monospace");
            ctx.set_fill_style_str("rgb(150,156,164)");
            let _ = ctx.fill_text("0", t.x as f64 * CELL + 13.0, t.y as f64 * CELL - 4.0);
            continue;
        }
        let (tr, tg, tb) = town_col[i];
        draw_house(ctx, t.x as f64 * CELL - 3.0, t.y as f64 * CELL - 2.0, tr, tg, tb);
        if t.at_war {
            ctx.set_fill_style_str("rgb(232,68,64)");
            ctx.fill_rect(t.x as f64 * CELL - 5.0, t.y as f64 * CELL - 7.0, 10.0, 7.0);
            ctx.set_fill_style_str("rgb(255,214,96)");
            ctx.fill_rect(t.x as f64 * CELL - 5.0, t.y as f64 * CELL - 7.0, 2.0, 2.0);
            ctx.fill_rect(t.x as f64 * CELL + 3.0, t.y as f64 * CELL - 7.0, 2.0, 2.0);
            ctx.fill_rect(t.x as f64 * CELL - 5.0, t.y as f64 * CELL - 1.0, 10.0, 1.0);
        }
        if t.idea != TownIdea::None {
            let (px, py) = (t.x as f64 * CELL + 5.0, t.y as f64 * CELL - 8.0);
            match t.idea {
                TownIdea::War => {
                    ctx.set_fill_style_str("rgb(255,90,70)");
                    ctx.fill_rect(px + 1.0, py - 2.0, 2.0, 2.0);
                    ctx.fill_rect(px, py, 4.0, 2.0);
                    ctx.set_fill_style_str("rgb(255,214,96)");
                    ctx.fill_rect(px + 1.0, py, 2.0, 1.0);
                }
                TownIdea::Prosperity => {
                    ctx.set_fill_style_str("rgb(126,231,135)");
                    ctx.fill_rect(px + 1.0, py - 1.0, 1.0, 2.0);
                    ctx.fill_rect(px + 2.0, py + 1.0, 1.0, 1.0);
                    ctx.set_fill_style_str("rgb(255,222,120)");
                    ctx.fill_rect(px + 2.0, py - 1.0, 1.0, 2.0);
                }
                TownIdea::Toil => {
                    ctx.set_fill_style_str("rgb(228,190,84)");
                    ctx.fill_rect(px, py + 1.0, 4.0, 1.0);
                    ctx.fill_rect(px + 1.0, py, 2.0, 1.0);
                    ctx.set_fill_style_str("rgb(238,243,247)");
                    ctx.fill_rect(px, py - 1.0, 4.0, 1.0);
                }
                TownIdea::None => {}
            }
        }
        let (bx, by) = (t.x as f64 * CELL - 14.0, t.y as f64 * CELL + 15.0);
        for (i, k) in t.built.iter().enumerate() {
            let sx = bx + (i % 6) as f64 * 8.0;
            let sy = by + (i / 6) as f64 * 8.0;
            draw_building(ctx, sx, sy, *k, tr, tg, tb);
        }
        if let Some((fl, ft)) = build_flash {
            if fl == i && ft > 0.0 {
                ctx.set_stroke_style_str(&format!("rgba(255,255,255,{:.2})", ft * 0.9));
                ctx.stroke_rect(t.x as f64 * CELL - 8.0, t.y as f64 * CELL - 8.0, 24.0, 24.0);
            }
        }
        if let Some((kind, progress)) = t.queue.first() {
            let slot = t.built.len();
            let sx = bx + (slot % 6) as f64 * 8.0;
            let sy = by + (slot / 6) as f64 * 8.0;
            draw_scaffold(ctx, sx, sy, *progress, kind.cost(), tr, tg, tb);
        }
        let _ = i;
        ctx.set_font("11px ui-monospace, monospace");
        ctx.set_fill_style_str("rgb(238,243,247)");
        let _ = ctx.fill_text(&sim.pop(i).to_string(), t.x as f64 * CELL + 13.0, t.y as f64 * CELL - 4.0);
        if let Some(rul) = sim
            .families
            .iter()
            .filter(|f| f.town == i && !f.extinct)
            .max_by_key(|f| f.members)
        {
            if !rul.name.is_empty() {
                ctx.set_font("10px ui-monospace, monospace");
                ctx.set_fill_style_str("rgb(215,225,236)");
                let _ = ctx.fill_text(&rul.name, t.x as f64 * CELL - 16.0, t.y as f64 * CELL + 8.0);
            }
        }
    }

    if let Some(si) = selected_town {
        if let Some(t) = sim.towns.get(si) {
            let pulse = 0.5 + 0.5 * ((tick as f64) * 0.15).sin();
            let alpha = 0.4 + pulse * 0.4;
            ctx.set_stroke_style_str(&format!("rgba(255,255,100,{:.2})", alpha));
            ctx.set_line_width(2.0 / zoom);
            ctx.begin_path();
            ctx.arc(t.x as f64 * CELL + 4.0, t.y as f64 * CELL + 4.0, 14.0, 0.0, std::f64::consts::TAU).ok();
            ctx.stroke();
            ctx.set_line_width(1.0);
        }
    }

    for emp in sim.empires.iter().filter(|e| e.members.len() > 1) {
        ctx.set_stroke_style_str(&format!("rgba({},{},{},0.55)", emp.r, emp.g, emp.b));
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for w in emp.members.windows(2) {
            let t0 = &sim.towns[w[0]];
            let t1 = &sim.towns[w[1]];
            ctx.move_to(t0.x as f64 * CELL, t0.y as f64 * CELL);
            ctx.line_to(t1.x as f64 * CELL, t1.y as f64 * CELL);
        }
        ctx.stroke();
        ctx.set_line_width(1.0);
    }

    for (i, a) in sim.agents.iter().enumerate() {
        let t = &sim.towns[a.home];
        let (tr, tg, tb) = if a.home < town_col.len() { town_col[a.home] } else { (t.r, t.g, t.b) };
        let fx = a.x as f64 * CELL + 2.0;
        let fy = a.y as f64 * CELL + 1.0;
        if let Some(fam) = sim.families.get(a.family) {
            let (fr, fg, fb) = fam.accent;
            let alpha = if a.founder { 0.9 } else { 0.45 };
            ctx.set_fill_style_str(&format!("rgba({},{},{},{:.2})", fr, fg, fb, alpha));
            ctx.fill_rect(fx - 1.0, fy - 1.0, 4.0, 4.0);
        }
        draw_agent(ctx, fx, fy, tr, tg, tb, a.dir_x.clamp(-1, 1), i & 1);
        if a.founder {
            ctx.set_fill_style_str("rgb(255,222,120)");
            ctx.fill_rect(fx, fy - 1.0, 2.0, 1.0);
        }
        let role_col = match a.role {
            Role::Worker => "rgb(200,205,215)",
            Role::Farmer => "rgb(126,231,135)",
            Role::Miner => "rgb(228,190,84)",
            Role::Hunter => "rgb(255,140,80)",
            Role::Priest => "rgb(248,242,220)",
            Role::Healer => "rgb(120,220,214)",
            Role::Guard => "rgb(210,120,120)",
            Role::Scholar => "rgb(202,166,255)",
            Role::Builder => "rgb(255,194,92)",
        };
        ctx.set_fill_style_str(role_col);
        ctx.fill_rect(fx + 3.0, fy + 2.0, 1.0, 1.0);
        if a.age < crate::sim::CHILD_AGE {
            ctx.set_fill_style_str("rgb(255,214,196)");
            ctx.fill_rect(fx + 1.0, fy + 2.0, 1.0, 1.0);
        }
        if a.age > crate::sim::OLD_AGE {
            ctx.set_fill_style_str("rgb(238,242,247)");
            ctx.fill_rect(fx, fy - 1.0, 2.0, 1.0);
        }
        if a.sick > 0 {
            ctx.set_fill_style_str("rgb(255,64,64)");
            ctx.fill_rect(fx + 3.0, fy + 4.0, 1.0, 1.0);
        }
        if a.raider {
            ctx.set_fill_style_str("rgb(232,68,64)");
            ctx.fill_rect(fx + 3.0, fy - 1.0, 2.0, 1.0);
            ctx.set_fill_style_str("rgb(255,214,96)");
            ctx.fill_rect(fx + 3.0, fy - 2.0, 1.0, 1.0);
        }
        if let Some((kind, _)) = a.carry {
            let col = match kind {
                crate::sim::ResourceKind::Food => "rgb(126,231,135)",
                crate::sim::ResourceKind::Water => "rgb(88,166,255)",
                crate::sim::ResourceKind::Ore => "rgb(228,190,84)",
                crate::sim::ResourceKind::Meat => "rgb(232,120,96)",
                crate::sim::ResourceKind::Gold => "rgb(255,222,120)",
            };
            ctx.set_fill_style_str(col);
            ctx.fill_rect(fx + 1.0, fy - 1.0, 3.0, 1.0);
        }
        if a.mood < -0.5 {
            ctx.set_fill_style_str("rgba(220,60,60,0.7)");
            ctx.fill_rect(fx + 1.0, fy - 3.0, 2.0, 1.0);
        } else if a.mood > 0.5 {
            ctx.set_fill_style_str("rgba(80,220,100,0.7)");
            ctx.fill_rect(fx + 1.0, fy - 3.0, 2.0, 1.0);
        }
    }

for a in &sim.animals {
        draw_animal(
            &ctx,
            a.x as f64 * CELL,
            a.y as f64 * CELL,
            a.species,
            0,
            a.home.is_some(),
        );
    }

    for c in &sim.caravans {
        draw_caravan(&ctx, c.x as f64 * CELL, c.y as f64 * CELL);
    }

    match sim.weather {
        Weather::Clear => {}
        Weather::Rain => {
            ctx.set_fill_style_str("rgba(150,190,255,0.18)");
            let ph = tick as usize % 9;
            for i in 0..100usize {
                let wx = (i as f64 * 173.0 + ph as f64 * 5.0) % cw;
                let wy = (i as f64 * 97.0 + ph as f64 * 8.0) % ch;
                ctx.fill_rect(wx, wy, 1.0, 4.0);
            }
            ctx.set_fill_style_str("rgba(96,140,220,0.10)");
            ctx.fill_rect(0.0, 0.0, cw, ch);
        }
        Weather::Heat => {
            ctx.set_fill_style_str(&format!(
                "rgba(255,140,40,{:.3})",
                0.04 + (tick % 30) as f64 * 0.002
            ));
            ctx.fill_rect(0.0, 0.0, cw, ch);
        }
        Weather::Frost => {
            ctx.set_fill_style_str("rgba(240,245,255,0.35)");
            let ph = tick as usize % 7;
            for i in 0..70usize {
                let wx = (i as f64 * 131.0 + ph as f64 * 2.0) % cw;
                let wy = (i as f64 * 61.0 + ph as f64 * 6.0) % ch;
                ctx.fill_rect(wx, wy, 1.0, 1.0);
                ctx.fill_rect(wx + 5.0, wy + 3.0, 1.0, 1.0);
            }
            ctx.set_fill_style_str("rgba(210,225,255,0.12)");
            ctx.fill_rect(0.0, 0.0, cw, ch);
        }
    }

    if sim.is_night() {
        ctx.set_fill_style_str("rgba(8,12,26,0.34)");
        ctx.fill_rect(0.0, 0.0, cw, ch);
    }
    ctx.set_fill_style_str("rgba(140,120,90,0.35)");
    for y in 0..H {
        for x in 0..W {
            if sim.roads[y * W + x] {
                ctx.fill_rect(
                    x as f64 * CELL + 1.0,
                    y as f64 * CELL + 1.0,
                    CELL - 2.0,
                    CELL - 2.0,
                );
            }
        }
    }
    let dawn_len = DAY_LEN as f64 * 0.1;
    let phase = sim.day_phase as f64;
    let dusk = if phase < dawn_len {
        (dawn_len - phase) / dawn_len
    } else if phase > DAY_LEN as f64 - dawn_len {
        (phase - (DAY_LEN as f64 - dawn_len)) / dawn_len
    } else {
        0.0
    };
    if dusk > 0.0 {
        ctx.set_fill_style_str(&format!("rgba(255,150,60,{:.2})", dusk * 0.18));
        ctx.fill_rect(0.0, 0.0, cw, ch);
    }
    let season_tint = match sim.season {
        Season::Winter => "rgba(195,220,255,0.13)",
        Season::Spring => "rgba(140,240,170,0.06)",
        Season::Summer => "rgba(255,225,130,0.05)",
        Season::Autumn => "rgba(220,150,70,0.11)",
    };
    ctx.set_fill_style_str(season_tint);
    ctx.fill_rect(0.0, 0.0, cw, ch);

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

    let houses: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::House).count())
        .sum();
    let wells: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Well).count())
        .sum();
    let farms: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Farm).count())
        .sum();
    let posts: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::TradePost).count())
        .sum();
    let sanctuaries: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Sanctuary).count())
        .sum();
    let clinics: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Clinic).count())
        .sum();
    let walls: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Wall).count())
        .sum();
    let barracks: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Barracks).count())
        .sum();
    let unis: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::University).count())
        .sum();
    let smiths: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Smithy).count())
        .sum();
    let libs: usize = sim
        .towns
        .iter()
        .map(|t| t.built.iter().filter(|b| **b == crate::sim::BuildingKind::Library).count())
        .sum();
    let science: f32 = sim.towns.iter().map(|t| t.dev).sum();
    let scholars: usize = sim.agents.iter().filter(|a| a.role == Role::Scholar).count();
    let builders: usize = sim.agents.iter().filter(|a| a.role == Role::Builder).count();
    let sick = sim.agents.iter().filter(|a| a.sick > 0).count();
    let pending: usize = sim.towns.iter().map(|t| t.queue.len()).sum();
    let dynasty = sim.families.iter().filter(|f| !f.extinct).count();
    let extinct = sim.families.len() - dynasty;
    let ideas = sim.towns.iter().filter(|t| t.idea != TownIdea::None).count();
    let weather_name = match sim.weather {
        Weather::Clear => "☀ ясно",
        Weather::Rain => "🌧 дождь",
        Weather::Heat => "🔥 жара",
        Weather::Frost => "❄ мороз",
    };
    let season_name = match sim.season {
        Season::Spring => "весна",
        Season::Summer => "лето",
        Season::Autumn => "осень",
        Season::Winter => "зима",
    };
    let day_name = if sim.is_day() { "☀ день" } else { "🌙 ночь" };
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("tick {}  {}  {}", sim.tick_count, season_name, day_name));
    lines.push(format!("pop {}  🏠{} ⛲{} 🌾{} 🏦{} ⛪{} ⛑{} ⛋{} ⛩{}  uni{} sm{} lib{}  in_queue {}", sim.agents.len(), houses, wells, farms, posts, sanctuaries, clinics, walls, barracks, unis, smiths, libs, pending));
    lines.push(format!("science {}  scholars {}  builders {}", science as u32, scholars, builders));
    let avg_mood: f32 = if sim.agents.is_empty() { 0.0 } else { sim.agents.iter().map(|a| a.mood).sum::<f32>() / sim.agents.len() as f32 };
    let mood_icon = if avg_mood > 0.3 { "😊" } else if avg_mood < -0.3 { "😠" } else { "😐" };
    lines.push(format!("mood {:.2} {}  links {}", avg_mood, mood_icon, sim.social_links.len()));
    let burning: usize = sim.grid.iter().filter(|c| c.burn > 0).count();
    lines.push(format!("events  fires {}  invades {}  veins {}", burning, sim.invades, sim.gold_veins.len()));
    let max_faith = sim.towns.iter().map(|t| t.faith as usize).max().unwrap_or(0);
    lines.push(format!("вера {}  больны {}", max_faith, sick));
    let plague = sim.towns.iter().any(|t| t.plague_until > 0);
    if plague {
        lines.push("☠ ЧУМА! Постройте лечебницу ⛑ и целителей".to_string());
    }
    let bless_name = match sim
        .towns
        .iter()
        .find(|t| t.blessing != crate::sim::Blessing::None)
        .map(|t| t.blessing)
    {
        Some(crate::sim::Blessing::Fertility) => "7 плодородие",
        Some(crate::sim::Blessing::Abundance) => "7 изобилие",
        Some(crate::sim::Blessing::Protection) => "7 защита",
        _ => "",
    };
    if !bless_name.is_empty() {
        lines.push(format!("благословение: {}", bless_name));
    }
    let mut town_rows: Vec<(usize, usize)> = Vec::new();
    for (i, t) in sim.towns.iter().enumerate() {
        let ruler = sim
            .families
            .iter()
            .filter(|f| f.town == i && !f.extinct)
            .max_by_key(|f| f.members)
            .map(|f| f.name.as_str())
            .filter(|n| !n.is_empty())
            .unwrap_or("?");
        if !t.alive {
            let row = lines.len();
            lines.push(format!("☠ {}  разорён", ruler));
            town_rows.push((row, i));
            continue;
        }
        let mark = if t.at_war {
            "  ⚔"
        } else {
            match t.idea {
                TownIdea::None => "",
                TownIdea::War => "  ⚔",
                TownIdea::Prosperity => "  🌿",
                TownIdea::Toil => "  🔨",
            }
        };
        let emp_mark = t
            .empire
            .and_then(|e| sim.empires.get(e))
            .map(|emp| format!("  ⚑{}", emp.name))
            .unwrap_or_default();
        let row = lines.len();
        lines.push(format!(
            "{} {}  pop {}  f{} w{} o{} m{} g{}{}{}",
            "◆", ruler, sim.pop(i),
            t.stocks.food as i32, t.stocks.water as i32, t.stocks.ore as i32, t.stocks.meat as i32, t.stocks.gold as i32, mark, emp_mark
        ));
        town_rows.push((row, i));
    }
    let empires_line = sim
        .empires
        .iter()
        .filter(|e| !e.members.is_empty())
        .map(|e| format!("{} ({} городов)", e.name, e.members.len()))
        .collect::<Vec<_>>()
        .join("   ");
    if !empires_line.is_empty() {
        lines.push(format!("империи: {}", empires_line));
    }
    let (mut deer, mut boar, mut wolf, mut cow) = (0, 0, 0, 0);
    for a in &sim.animals {
        match a.species {
            Species::Deer => deer += 1,
            Species::Boar => boar += 1,
            Species::Wolf => wolf += 1,
            Species::Cow => cow += 1,
        }
    }
    lines.push(format!(
        "animals deer {}  boar {}  wolf {}  cow {}  (herds {})",
        deer, boar, wolf, cow,
        sim.animals
            .iter()
            .filter(|a| a.species == Species::Cow && a.home.is_some())
            .count()
    ));
    lines.push(format!("dynasties {}  extinct {}  ideas {}  wars {}  ruins {}  migrants {}  allies {}  treaties {}  gifts {}", dynasty, extinct, ideas, sim.towns.iter().filter(|t| t.at_war).count(), sim.towns.iter().filter(|t| !t.alive).count(), sim.migrations, sim.alliances.len(), sim.treaties.len(), sim.gifts_sent));
    let gold_total: i32 = sim.towns.iter().map(|t| t.stocks.gold as i32).sum();
    let gold_in_route: i32 = sim.caravans.iter().map(|c| c.goods.iter().map(|(k, q)| q * crate::sim::trade_price(*k)).sum::<f32>() as i32).sum();
    lines.push(format!("gold {}  caravan {}  (+{} in route)", gold_total, sim.caravans.len(), gold_in_route));
    lines.push(format!("workers {}  farmers {}  miners {}  hunters {}", sim.agents.iter().filter(|a| a.role == Role::Worker).count(), sim.agents.iter().filter(|a| a.role == Role::Farmer).count(), sim.agents.iter().filter(|a| a.role == Role::Miner).count(), sim.agents.iter().filter(|a| a.role == Role::Hunter).count()));
    lines.push(format!("{} {:>3.0}s  fps {:.0}  speed x{:.1}{}", weather_name, sim.weather_left * 0.08, fps, speed, if paused { "  [PAUSED]" } else { "" }));
    lines.push(String::new());
    lines.push("Space: пауза   B: 🌱   I: 💡 идея городу   W: ⛅ погода".to_string());
    lines.push("1: 🏠  2: ⛲  3: 🏦  4: 🌾  5: ⛪  6: ⛑  7: ⛋  8: ⛩  9: 🎓  0: 🔨  Q: 📚   C: 🐄   R: новый мир".to_string());
    ctx.set_fill_style_str("rgba(10,14,18,0.72)");
    ctx.fill_rect(4.0, 4.0, 346.0, 14.0 + lines.len() as f64 * 15.0);
    ctx.set_stroke_style_str("rgb(70,78,90)");
    ctx.begin_path();
    ctx.rect(4.0, 4.0, 346.0, 14.0 + lines.len() as f64 * 15.0);
    ctx.stroke();
    ctx.set_font("13px ui-monospace, monospace");
    let mut chips: Vec<Option<(u8, u8, u8)>> = vec![None; lines.len()];
    for (row, ti) in town_rows {
        let t = &sim.towns[ti];
        let (r, g, b) = if t.alive { town_col[ti] } else { (120, 126, 134) };
        chips[row] = Some((r, g, b));
    }
    for (i, l) in lines.iter().enumerate() {
        if let Some((r, g, b)) = chips[i] {
            ctx.set_fill_style_str(&format!("rgb({},{},{})", r, g, b));
            ctx.fill_rect(10.0, 13.0 + i as f64 * 15.0, 10.0, 10.0);
        }
        ctx.set_fill_style_str(if i == 2 { "rgb(255,222,120)" } else { "rgb(238,243,247)" });
        let _ = ctx.fill_text(l, if chips[i].is_some() { 26.0 } else { 10.0 }, 22.0 + i as f64 * 15.0);
    }

    if let Some(si) = selected_town {
        if let Some(t) = sim.towns.get(si) {
            let ruler = sim.families.iter()
                .filter(|f| f.town == si && !f.extinct)
                .max_by_key(|f| f.members)
                .map(|f| f.name.as_str())
                .filter(|n| !n.is_empty())
                .unwrap_or("?");
            let pop = sim.pop(si);
            let (cr, cg, cb) = town_col[si];
            let agents: Vec<_> = sim.agents.iter().filter(|a| a.home == si).collect();
            let sick_n = agents.iter().filter(|a| a.sick > 0).count();
            let avg_mood: f32 = if agents.is_empty() { 0.0 } else {
                agents.iter().map(|a| a.mood).sum::<f32>() / agents.len() as f32
            };
            let mut role_counts = [(Role::Worker, 0u32), (Role::Farmer, 0u32), (Role::Miner, 0u32),
                (Role::Hunter, 0u32), (Role::Builder, 0u32), (Role::Healer, 0u32),
                (Role::Priest, 0u32), (Role::Scholar, 0u32), (Role::Guard, 0u32)];
            for a in &agents {
                for (r, c) in role_counts.iter_mut() {
                    if a.role == *r { *c += 1; }
                }
            }
            let build_counts = |kind: crate::sim::BuildingKind| -> usize {
                t.built.iter().filter(|b| **b == kind).count()
            };
            let houses = build_counts(crate::sim::BuildingKind::House);
            let wells = build_counts(crate::sim::BuildingKind::Well);
            let farms = build_counts(crate::sim::BuildingKind::Farm);
            let posts = build_counts(crate::sim::BuildingKind::TradePost);
            let sanct = build_counts(crate::sim::BuildingKind::Sanctuary);
            let clinic = build_counts(crate::sim::BuildingKind::Clinic);
            let wall = build_counts(crate::sim::BuildingKind::Wall);
            let barracks = build_counts(crate::sim::BuildingKind::Barracks);
            let uni = build_counts(crate::sim::BuildingKind::University);
            let smith = build_counts(crate::sim::BuildingKind::Smithy);
            let lib = build_counts(crate::sim::BuildingKind::Library);
            let bless_name = match t.blessing {
                crate::sim::Blessing::Fertility => "плодородие",
                crate::sim::Blessing::Abundance => "изобилие",
                crate::sim::Blessing::Protection => "защита",
                crate::sim::Blessing::None => "",
            };
            let idea_name = match t.idea {
                crate::sim::TownIdea::War => "война",
                crate::sim::TownIdea::Prosperity => "процветание",
                crate::sim::TownIdea::Toil => "труд",
                crate::sim::TownIdea::None => "",
            };
            let emp_name = t.empire.and_then(|e| sim.empires.get(e)).map(|emp| emp.name.as_str()).unwrap_or("");
            let war_mark = if t.at_war {
                let enemy_name = t.enemy.and_then(|ei| sim.towns.get(ei))
                    .and_then(|_| sim.families.iter().filter(|f| f.town == t.enemy.unwrap() && !f.extinct)
                        .max_by_key(|f| f.members).map(|f| f.name.as_str()))
                    .unwrap_or("?");
                format!("  ⚔ vs {}", enemy_name)
            } else { String::new() };
            let queue_str = if t.queue.is_empty() { String::new() } else {
                t.queue.iter().map(|(k, _)| match k {
                    crate::sim::BuildingKind::House => "🏠", crate::sim::BuildingKind::Well => "⛲",
                    crate::sim::BuildingKind::TradePost => "🏦", crate::sim::BuildingKind::Farm => "🌾",
                    crate::sim::BuildingKind::Sanctuary => "⛪", crate::sim::BuildingKind::Clinic => "⛑",
                    crate::sim::BuildingKind::Wall => "⛋", crate::sim::BuildingKind::Barracks => "⛩",
                    crate::sim::BuildingKind::University => "🎓", crate::sim::BuildingKind::Smithy => "🔨",
                    crate::sim::BuildingKind::Library => "📚",
                }).collect::<Vec<_>>().join("")
            };
            let families: Vec<_> = sim.families.iter()
                .filter(|f| f.town == si && !f.extinct)
                .collect();

            let mut p: Vec<String> = Vec::new();
            p.push(format!("═══ {} ═══", ruler));
            p.push(format!("pop {} / {}  |  mood {:.2}{}", pop, t.cap, avg_mood,
                if sick_n > 0 { format!("  sick {}", sick_n) } else { String::new() }));
            p.push(String::new());
            p.push(format!(" еда {:>5.0}  вода {:>5.0}  руда {:>5.0}", t.stocks.food, t.stocks.water, t.stocks.ore));
            p.push(format!(" мясо {:>5.0}  золото {:>5.0}", t.stocks.meat, t.stocks.gold));
            p.push(String::new());
            p.push(format!("🏠{} ⛲{} 🌾{} 🏦{} ⛪{} ⛑{}", houses, wells, farms, posts, sanct, clinic));
            p.push(format!("⛋{} ⛩{} 🎓{} 🔨{} 📚{}", wall, barracks, uni, smith, lib));
            if !t.queue.is_empty() {
                p.push(format!("  build: {} ({:.0}%)", queue_str, t.queue[0].1 * 100.0));
            }
            p.push(String::new());
            for (r, c) in &role_counts {
                if *c > 0 {
                    let icon = match r {
                        Role::Worker => "⚙", Role::Farmer => "🌱", Role::Miner => "⛏",
                        Role::Hunter => "🏹", Role::Builder => "🔨", Role::Healer => "💚",
                        Role::Priest => "🙏", Role::Scholar => "🎓", Role::Guard => "⚔",
                    };
                    p.push(format!(" {} {}×{}", icon, format!("{:?}", r), c));
                }
            }
            p.push(String::new());
            if !families.is_empty() {
                let fam_str = families.iter().map(|f| {
                    format!("{} ({}{}, {}{})", f.name, f.members, if f.children > 0 { format!("+{}", f.children) } else { String::new() },
                        if f.role != Role::Worker { format!("{:?}", f.role) } else { String::new() },
                        "")
                }).collect::<Vec<_>>().join("  ");
                p.push(format!("династии: {}", fam_str));
            }
            if !emp_name.is_empty() {
                p.push(format!("империя: {}", emp_name));
            }
            if !idea_name.is_empty() {
                p.push(format!("идея: {}  ({:.0} тиков)", idea_name, t.idea_left));
            }
            if !bless_name.is_empty() {
                p.push(format!("благословение: {}  ({:.0} тиков)", bless_name, t.blessing_left));
            }
            if t.plague_until > 0 {
                p.push("☠ ЧУМА!".to_string());
            }
            if t.faith > 0.0 {
                p.push(format!("вера: {:.0}", t.faith));
            }
            if !war_mark.is_empty() {
                p.push(war_mark);
            }
            p.push(format!("наука: {:.1}", t.dev));
            if !t.alive {
                p.push("☠ РАЗОРЁН".to_string());
            }

            let pw = 220.0;
            let ph = 14.0 + p.len() as f64 * 15.0;
            let px = 4.0;
            let py = 4.0 + 14.0 + lines.len() as f64 * 15.0 + 8.0;
            ctx.set_fill_style_str("rgba(10,14,18,0.88)");
            ctx.fill_rect(px, py, pw, ph);
            ctx.set_stroke_style_str(&format!("rgb({},{},{})", cr, cg, cb));
            ctx.begin_path();
            ctx.rect(px, py, pw, ph);
            ctx.stroke();
            ctx.set_font("13px ui-monospace, monospace");
            for (i, l) in p.iter().enumerate() {
                if i == 0 {
                    ctx.set_fill_style_str(&format!("rgb({},{},{})", cr, cg, cb));
                } else {
                    ctx.set_fill_style_str("rgb(238,243,247)");
                }
                let _ = ctx.fill_text(l, px + 8.0, py + 14.0 + i as f64 * 15.0);
            }
        }
    }
}