use crate::sim::{Terrain, H, Sim, W};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const CELL: f64 = 8.0;

pub fn draw_terrain(ctx: &CanvasRenderingContext2d, sim: &Sim) {
    for y in 0..H {
        for x in 0..W {
            let c = &sim.grid[y * W + x];
            let (r, g, b) = match c.terrain {
                Terrain::Water => (42, 111, 176),
                Terrain::Forest => (47, 107, 35),
                Terrain::Grass => (93, 140, 58),
            };
            ctx.set_fill_style_str(&format!("#{:02x}{:02x}{:02x}", r, g, b));
            ctx.fill_rect(x as f64 * CELL, y as f64 * CELL, CELL, CELL);
        }
    }
}

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    terrain: &HtmlCanvasElement,
    sim: &Sim,
    paused: bool,
    speed: f64,
    fps: f64,
) {
    let _ = ctx.draw_image_with_html_canvas_element(terrain, 0.0, 0.0);

    for t in &sim.towns {
        let color = format!("#{:02x}{:02x}{:02x}", t.r, t.g, t.b);
        ctx.set_fill_style_str(&color);
        ctx.fill_rect(t.x as f64 * CELL - 3.0, t.y as f64 * CELL - 3.0, 18.0, 18.0);
        ctx.set_fill_style_str("#dfe7ef");
    }

    for (ti, t) in sim.towns.iter().enumerate() {
        let color = format!("#{:02x}{:02x}{:02x}", t.r, t.g, t.b);
        ctx.set_fill_style_str(&color);
        let pop = sim.pop(ti);
        ctx.set_font("13px ui-monospace, monospace");
        let _ = ctx.fill_text(&pop.to_string(), (t.x as f64 + 0.3) * CELL, (t.y as f64 - 0.8) * CELL);
    }

    for a in &sim.agents {
        let t = &sim.towns[a.home];
        let color = format!("#{:02x}{:02x}{:02x}aa", t.r, t.g, t.b);
        ctx.set_fill_style_str(&color);
        ctx.fill_rect((a.x as f64 + 0.25) * CELL, (a.y as f64 + 0.25) * CELL, 4.5, 4.5);
    }

    let total: f32 = sim.towns.iter().map(|t| t.stockpile).sum();
    let lines = [
        format!("tick {}", sim.tick_count),
        format!("pop {}  food {}", sim.agents.len(), total as i32),
        format!("fps {:.0}  speed x{:.1}{}", fps, speed, if paused { "  [PAUSED]" } else { "" }),
        String::new(),
        "Space: пауза   +/-: скорость   R: новый мир".to_string(),
    ];
    ctx.set_font("13px ui-monospace, monospace");
    ctx.set_fill_style_str("#0c1012aa");
    ctx.fill_rect(4.0, 4.0, 360.0, 14.0 + lines.len() as f64 * 16.0);
    ctx.set_fill_style_str("#eef3f7");
    for (i, l) in lines.iter().enumerate() {
        let _ = ctx.fill_text(l, 10.0, 24.0 + i as f64 * 16.0);
    }
}