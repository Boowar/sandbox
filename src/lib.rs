mod render;
mod sim;

use js_sys::{Date, Function, Uint8Array};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Blob, BlobPropertyBag, CanvasRenderingContext2d, Element, EventTarget,
    HtmlAnchorElement, HtmlCanvasElement, HtmlInputElement, KeyboardEvent,
    PointerEvent, Url, WheelEvent,
};

const CELL: f64 = 8.0;
const BLESS_R: i32 = 3;
const ZOOM_MIN: f64 = 0.5;
const ZOOM_MAX: f64 = 8.0;

fn world_size() -> (f64, f64) {
    (sim::W as f64 * CELL, sim::H as f64 * CELL)
}

struct App {
    sim: Rc<RefCell<sim::Sim>>,
    ctx: CanvasRenderingContext2d,
    terrain: HtmlCanvasElement,
    terrain_ctx: CanvasRenderingContext2d,
    paused: bool,
    bless_mode: bool,
    inspire_mode: bool,
    build_mode: bool,
    build_terrain: sim::Terrain,
    road_mode: bool,
    pending_bless: Vec<(usize, usize)>,
    pending_build: Vec<(usize, usize)>,
    pending_road: Vec<(usize, usize)>,
    effects: Vec<render::Fx>,
    btn_pause: Element,
    btn_bless: Element,
    btn_inspire: Element,
    _btn_build: Element,
    _btn_road: Element,
    speed_lbl: Element,
    speed: f64,
    acc: f64,
    last: f64,
    fps_frames: u32,
    fps_time: f64,
    fps: f64,
    seed: u64,
    zoom: f64,
    cam_x: f64,
    cam_y: f64,
    pointers: Vec<(i32, i32, i32)>,
    drag_ptr: Option<i32>,
    pinch_prev_dist: f64,
    pinch_prev_mx: f64,
    pinch_prev_my: f64,
    build_cursor: usize,
    build_flash: Option<(usize, f64)>,
    selected_town: Option<usize>,
    hud: render::HudConfig,
}

impl App {
    fn step(&mut self, now: f64) {
        if !self.pending_bless.is_empty() {
            let pts: Vec<(usize, usize)> = self.pending_bless.drain(..).collect();
            let mut sim = self.sim.borrow_mut();
            for (x, y) in pts {
                sim.bless(x as i32, y as i32, BLESS_R);
                self.effects.push(render::Fx {
                    x: x as f64 * CELL + CELL / 2.0,
                    y: y as f64 * CELL + CELL / 2.0,
                    life: 26.0,
                });
            }
            render::draw_terrain(&self.terrain_ctx, &sim);
        }
        if !self.pending_build.is_empty() {
            let pts: Vec<(usize, usize)> = self.pending_build.drain(..).collect();
            let terrain = self.build_terrain;
            let mut sim = self.sim.borrow_mut();
            for (x, y) in pts {
                if x < sim::W && y < sim::H {
                    let i = y * sim::W + x;
                    let old = sim.grid[i].terrain;
                    if old != sim::Terrain::Water && old != sim::Terrain::Farm {
                        sim.grid[i].terrain = terrain;
                        match terrain {
                            sim::Terrain::Forest => { sim.grid[i].food = 10.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Hills => { sim.grid[i].food = 0.0; sim.grid[i].ore = 60.0; }
                            sim::Terrain::Water => { sim.grid[i].food = 0.0; sim.grid[i].ore = 0.0; sim.grid[i].water = 240.0; }
                            sim::Terrain::Desert => { sim.grid[i].food = 1.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Tundra => { sim.grid[i].food = 4.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Jungle => { sim.grid[i].food = 15.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Grass => { sim.grid[i].food = 10.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Farm => {}
                            sim::Terrain::Swamp => { sim.grid[i].food = 8.0; sim.grid[i].ore = 0.0; }
                            sim::Terrain::Volcano => { sim.grid[i].food = 0.0; sim.grid[i].ore = 60.0; }
                            sim::Terrain::CoralReef => { sim.grid[i].food = 6.0; sim.grid[i].ore = 0.0; }
                        }
                        self.effects.push(render::Fx {
                            x: x as f64 * CELL + CELL / 2.0,
                            y: y as f64 * CELL + CELL / 2.0,
                            life: 16.0,
                        });
                    }
                }
            }
            render::draw_terrain(&self.terrain_ctx, &sim);
        }
        if !self.pending_road.is_empty() {
            let pts: Vec<(usize, usize)> = self.pending_road.drain(..).collect();
            let mut sim = self.sim.borrow_mut();
            for (x, y) in pts {
                if x < sim::W && y < sim::H {
                    sim.toggle_road(x as i32, y as i32);
                }
            }
            render::draw_terrain(&self.terrain_ctx, &sim);
        }

        let dt = ((now - self.last) / 1000.0).max(0.0).min(0.1);
        self.last = now;
        if !self.paused {
            self.acc += dt * self.speed;
            while self.acc >= sim::TICK_DT {
                self.sim.borrow_mut().tick();
                self.acc -= sim::TICK_DT;
            }
        }
        self.fps_frames += 1;
        self.fps_time += dt;
        if self.fps_time >= 0.5 {
            self.fps = self.fps_frames as f64 / self.fps_time;
            self.fps_frames = 0;
            self.fps_time = 0.0;
        }
        if let Some((_, t)) = self.build_flash.as_mut() {
            *t -= dt;
            if *t <= 0.0 {
                self.build_flash = None;
            }
        }
        for e in self.effects.iter_mut() {
            e.life -= 1.0;
        }
        self.effects.retain(|e| e.life > 0.0);
        let sim = self.sim.borrow();
        render::draw(
            &self.ctx,
            &self.terrain,
            &sim,
            sim.tick_count,
            self.paused,
            self.speed,
            self.fps,
            &self.effects,
            self.zoom,
            self.cam_x,
            self.cam_y,
            self.build_flash,
            self.selected_town,
            self.hud,
        );
    }

    fn sync_ui(&self) {
        self.btn_pause
            .set_inner_html(if self.paused { "▶" } else { "⏸" });
        self.btn_bless
            .set_inner_html(if self.bless_mode { "🌱ON" } else { "🌱" });
        self.btn_inspire
            .set_inner_html(if self.inspire_mode { "💡ON" } else { "💡" });
        if let Some(doc) = self.btn_bless.parent_element().and_then(|p| p.owner_document()) {
            if let Some(el) = doc.get_element_by_id("btnBuild") {
                let _ = el.set_inner_html(if self.build_mode { "🏗ON" } else { "🏗" });
            }
            if let Some(el) = doc.get_element_by_id("btnRoad") {
                let _ = el.set_inner_html(if self.road_mode { "🛤ON" } else { "🛤" });
            }
        }
        self.speed_lbl.set_inner_html(&format!("x{:.1}", self.speed));
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.sync_ui();
    }

    fn toggle_bless(&mut self) {
        self.bless_mode = !self.bless_mode;
        if self.bless_mode {
            self.inspire_mode = false;
            self.build_mode = false;
            self.road_mode = false;
        }
        self.sync_ui();
    }

    fn toggle_inspire(&mut self) {
        self.inspire_mode = !self.inspire_mode;
        if self.inspire_mode {
            self.bless_mode = false;
            self.build_mode = false;
            self.road_mode = false;
        }
        self.sync_ui();
    }

    fn toggle_build(&mut self) {
        self.build_mode = !self.build_mode;
        if self.build_mode {
            self.bless_mode = false;
            self.inspire_mode = false;
            self.road_mode = false;
        }
        self.sync_ui();
    }

    fn toggle_road_mode(&mut self) {
        self.road_mode = !self.road_mode;
        if self.road_mode {
            self.bless_mode = false;
            self.inspire_mode = false;
            self.build_mode = false;
        }
        self.sync_ui();
    }

    fn cycle_build_terrain(&mut self) {
        self.build_terrain = match self.build_terrain {
            sim::Terrain::Forest => sim::Terrain::Hills,
            sim::Terrain::Hills => sim::Terrain::Water,
            sim::Terrain::Water => sim::Terrain::Desert,
            sim::Terrain::Desert => sim::Terrain::Tundra,
            sim::Terrain::Tundra => sim::Terrain::Jungle,
            sim::Terrain::Jungle => sim::Terrain::Swamp,
            sim::Terrain::Swamp => sim::Terrain::Grass,
            sim::Terrain::Grass => sim::Terrain::Forest,
            sim::Terrain::Farm => sim::Terrain::Forest,
            sim::Terrain::Volcano => sim::Terrain::Grass,
            sim::Terrain::CoralReef => sim::Terrain::Water,
        };
        self.sync_ui();
    }

    fn inspire_at(&mut self, client_x: i32, client_y: i32) {
        if !self.inspire_mode {
            return;
        }
        if let Some((gx, gy)) = self.to_grid(client_x as f64, client_y as f64) {
            let mut best = None;
            let mut bd = 4;
            for (ti, t) in self.sim.borrow().towns.iter().enumerate() {
                let d = (t.x as i32 - gx as i32).abs().max(t.y as i32 - gy as i32);
                if d < bd {
                    bd = d;
                    best = Some(ti);
                }
            }
            if let Some(ti) = best {
                self.sim.borrow_mut().inspire(ti);
                self.build_flash = Some((ti, 1.0));
            }
        }
    }

    fn cycle_weather(&mut self) {
        self.sim.borrow_mut().cycle_weather();
    }

    fn change_speed(&mut self, d: f64) {
        self.speed = (self.speed + d).clamp(0.25, 200.0);
        self.sync_ui();
    }

    fn new_world(&mut self) {
        self.seed += 1;
        self.effects.clear();
        let s = sim::Sim::new(self.seed);
        render::draw_terrain(&self.terrain_ctx, &s);
        self.sim = Rc::new(RefCell::new(s));
        self.acc = 0.0;
    }

    fn save_to_local(&self) {
        let json = self.sim.borrow().save_json();
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let ls = match window.local_storage() {
            Ok(Some(ls)) => ls,
            _ => return,
        };
        let _ = ls.set_item("sandbox_save", &json);
    }

    fn load_from_local(&mut self) -> bool {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return false,
        };
        let ls = match window.local_storage() {
            Ok(Some(ls)) => ls,
            _ => return false,
        };
        let json = match ls.get_item("sandbox_save") {
            Ok(Some(j)) => j,
            _ => return false,
        };
        if let Some(s) = sim::Sim::load_json(&json) {
            render::draw_terrain(&self.terrain_ctx, &s);
            self.sim = Rc::new(RefCell::new(s));
            self.effects.clear();
            self.acc = 0.0;
            true
        } else {
            false
        }
    }

    fn download_save(&self) {
        let json = self.sim.borrow().save_json();
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let arr = Uint8Array::new_with_byte_offset_and_length(
            &JsValue::from_str(&json),
            0,
            json.len() as u32,
        );
        let bag = BlobPropertyBag::new();
        bag.set_type("application/json");
        let blob = match Blob::new_with_str_sequence_and_options(&arr, &bag) {
            Ok(b) => b,
            Err(_) => return,
        };
        let url = match Url::create_object_url_with_blob(&blob) {
            Ok(u) => u,
            Err(_) => return,
        };
        let a: HtmlAnchorElement = match window.document() {
            Some(doc) => match doc.create_element("a") {
                Ok(el) => match el.dyn_into() {
                    Ok(a) => a,
                    Err(_) => return,
                },
                Err(_) => return,
            },
            None => return,
        };
        a.set_href(&url);
        a.set_download("sandbox_save.json");
        a.click();
        let _ = Url::revoke_object_url(&url);
    }

    fn upload_save(&mut self) {
        let window = web_sys::window().expect("no window");
        let document = window.document().expect("doc");
        let input: HtmlInputElement = document
            .create_element("input")
            .expect("input")
            .dyn_into()
            .expect("dyn");
        input.set_type("file");
        input.set_accept(".json");
        let sim_clone = Rc::clone(&self.sim);
        let terrain_clone = self.terrain_ctx.clone();
        let input_clone = input.clone();
        let cb = Closure::wrap(Box::new(move || {
            let files = match input_clone.files() {
                Some(f) => f,
                None => return,
            };
            let file = match files.get(0) {
                Some(f) => f,
                None => return,
            };
            let reader = web_sys::FileReader::new().expect("reader");
            let reader2 = reader.clone();
            let sim_inner = Rc::clone(&sim_clone);
            let terrain_inner = terrain_clone.clone();
            let onload = Closure::wrap(Box::new(move || {
                let text = reader2.result().expect("result").as_string().expect("str");
                if let Some(s) = sim::Sim::load_json(&text) {
                    render::draw_terrain(&terrain_inner, &s);
                    *sim_inner.borrow_mut() = s;
                }
            }) as Box<dyn FnMut()>);
            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            let _ = reader.read_as_text(&file.into());
            std::mem::forget(onload);
        }) as Box<dyn FnMut()>);
        let _ = input.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
        std::mem::forget(cb);
        input.click();
    }

    fn canvas_px(&self, client_x: f64, client_y: f64) -> Option<(f64, f64)> {
        let canvas = self.ctx.canvas()?;
        let r = canvas.get_bounding_client_rect();
        let w = r.width();
        let h = r.height();
        if w <= 0.0 || h <= 0.0 {
            return None;
        }
        let scale = w / world_size().0;
        Some(((client_x - r.left()) / scale, (client_y - r.top()) / scale))
    }

    fn to_grid(&self, client_x: f64, client_y: f64) -> Option<(usize, usize)> {
        let (px, py) = self.canvas_px(client_x, client_y)?;
        let (cw, ch) = world_size();
        let tx = cw / 2.0 - self.cam_x * self.zoom;
        let ty = ch / 2.0 - self.cam_y * self.zoom;
        let wx = (px - tx) / self.zoom;
        let wy = (py - ty) / self.zoom;
        let gx = (wx / CELL) as i32;
        let gy = (wy / CELL) as i32;
        if gx >= 0 && gy >= 0 && (gx as usize) < sim::W && (gy as usize) < sim::H {
            Some((gx as usize, gy as usize))
        } else {
            None
        }
    }

    fn clamp_cam(&mut self) {
        let (cw, ch) = world_size();
        let (hw, hh) = (cw / 2.0, ch / 2.0);
        if self.zoom > 1.0 {
            self.cam_x = self.cam_x.clamp(hw / self.zoom, cw - hw / self.zoom);
            self.cam_y = self.cam_y.clamp(hh / self.zoom, ch - hh / self.zoom);
        } else {
            self.cam_x = hw;
            self.cam_y = hh;
        }
    }

    fn zoom_at(&mut self, mx: f64, my: f64, factor: f64) {
        let (cw, ch) = world_size();
        let (hw, hh) = (cw / 2.0, ch / 2.0);
        let old = self.zoom;
        let wx = self.cam_x + (mx - hw) / old;
        let wy = self.cam_y + (my - hh) / old;
        let z = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
        self.zoom = z;
        self.cam_x = wx - (mx - hw) / z;
        self.cam_y = wy - (my - hh) / z;
        self.clamp_cam();
    }

    fn pan_by(&mut self, dx: f64, dy: f64) {
        self.cam_x -= dx / self.zoom;
        self.cam_y -= dy / self.zoom;
        self.clamp_cam();
    }

    fn update_pointer(&mut self, id: i32, px: f64, py: f64) -> (f64, f64) {
        let mut prev = (px, py);
        for p in &mut self.pointers {
            if p.0 == id {
                prev = (p.1 as f64, p.2 as f64);
                p.1 = px as i32;
                p.2 = py as i32;
                break;
            }
        }
        prev
    }

    fn handle_pinch(&mut self) {
        if self.pointers.len() != 2 {
            return;
        }
        let (x1, y1) = (self.pointers[0].1 as f64, self.pointers[0].2 as f64);
        let (x2, y2) = (self.pointers[1].1 as f64, self.pointers[1].2 as f64);
        let mx = (x1 + x2) / 2.0;
        let my = (y1 + y2) / 2.0;
        let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        if self.pinch_prev_dist > 0.0 {
            self.zoom_at(mx, my, d / self.pinch_prev_dist);
            self.cam_x -= (mx - self.pinch_prev_mx) / self.zoom;
            self.cam_y -= (my - self.pinch_prev_my) / self.zoom;
            self.clamp_cam();
        }
        self.pinch_prev_dist = d;
        self.pinch_prev_mx = mx;
        self.pinch_prev_my = my;
    }

    fn bless_at(&mut self, client_x: i32, client_y: i32) {
        if self.bless_mode {
            if let Some((x, y)) = self.to_grid(client_x as f64, client_y as f64) {
                self.pending_bless.push((x, y));
            }
        }
    }

    fn build_terrain_at(&mut self, client_x: i32, client_y: i32) {
        if self.build_mode {
            if let Some((x, y)) = self.to_grid(client_x as f64, client_y as f64) {
                self.pending_build.push((x, y));
            }
        }
    }

    fn road_at(&mut self, client_x: i32, client_y: i32) {
        if self.road_mode {
            if let Some((x, y)) = self.to_grid(client_x as f64, client_y as f64) {
                self.pending_road.push((x, y));
            }
        }
    }

    fn select_town_at(&mut self, client_x: i32, client_y: i32) {
        if self.bless_mode || self.inspire_mode || self.build_mode || self.road_mode {
            return;
        }
        if let Some((gx, gy)) = self.to_grid(client_x as f64, client_y as f64) {
            let mut best = None;
            let mut bd = 6;
            for (ti, t) in self.sim.borrow().towns.iter().enumerate() {
                let d = ((t.x as i32 - gx as i32).abs()).max((t.y as i32 - gy as i32).abs());
                if d < bd {
                    bd = d;
                    best = Some(ti);
                }
            }
            self.selected_town = if self.selected_town == best { None } else { best };
        } else {
            self.selected_town = None;
        }
    }

    fn build(&mut self, kind: sim::BuildingKind) {
        let n = self.sim.borrow().towns.len();
        if n == 0 {
            return;
        }
        let ti = self.selected_town.unwrap_or_else(|| self.build_cursor % n);
        self.sim.borrow_mut().build_request(ti, kind);
        if self.selected_town.is_none() {
            self.build_cursor += 1;
        }
        self.build_flash = Some((ti, 1.0));
    }

    fn breed_cows(&mut self) {
        let n = self.sim.borrow().towns.len();
        if n == 0 {
            return;
        }
        let ti = self.selected_town.unwrap_or_else(|| self.build_cursor % n);
        let (x, y) = {
            let s = self.sim.borrow();
            (s.towns[ti].x, s.towns[ti].y)
        };
        if self.sim.borrow_mut().breed_domestic(ti) {
            self.build_flash = Some((ti, 1.0));
            self.effects.push(render::Fx {
                x: x as f64 * CELL + CELL / 2.0,
                y: y as f64 * CELL + CELL / 2.0,
                life: 26.0,
            });
        }
        if self.selected_town.is_none() {
            self.build_cursor += 1;
        }
    }

    fn on_key(&mut self, e: KeyboardEvent) {
        match e.key().as_str() {
            "Escape" => {
                self.bless_mode = false;
                self.inspire_mode = false;
                self.build_mode = false;
                self.road_mode = false;
                self.selected_town = None;
            }
            " " => self.toggle_pause(),
            "+" | "=" => self.change_speed(1.0),
            "-" | "_" | "—" => self.change_speed(-1.0),
            "b" | "B" | "и" | "И" => self.toggle_bless(),
            "i" | "I" | "ш" | "Ш" => self.toggle_inspire(),
            "n" | "N" | "т" | "Т" => self.toggle_build(),
            "m" | "M" | "ь" | "Ь" => self.cycle_build_terrain(),
            "d" | "D" | "в" | "В" => self.toggle_road_mode(),
            "w" | "W" | "ц" | "Ц" => self.cycle_weather(),
            "1" => self.build(sim::BuildingKind::House),
            "2" => self.build(sim::BuildingKind::Well),
            "3" => self.build(sim::BuildingKind::TradePost),
            "4" => self.build(sim::BuildingKind::Farm),
            "5" => self.build(sim::BuildingKind::Sanctuary),
            "6" => self.build(sim::BuildingKind::Clinic),
            "7" => self.build(sim::BuildingKind::Wall),
            "8" => self.build(sim::BuildingKind::Barracks),
            "t" | "T" | "е" | "Е" => self.build(sim::BuildingKind::Temple),
            "9" => self.build(sim::BuildingKind::University),
            "0" => self.build(sim::BuildingKind::Smithy),
            "q" | "Q" | "й" | "Й" => self.build(sim::BuildingKind::Library),
            "e" | "E" | "у" | "У" => self.build(sim::BuildingKind::Warehouse),
            "f" | "F" | "а" | "А" => self.build(sim::BuildingKind::Sawmill),
            "c" | "C" | "с" | "С" => self.breed_cows(),
            "r" | "R" | "к" | "К" => self.new_world(),
            "F5" => self.save_to_local(),
            "F9" => {
                if !self.load_from_local() {
                    web_sys::console::log_1(&"No saved world found".into());
                }
            }
            "F1" => self.hud.show_weather = !self.hud.show_weather,
            "F2" => self.hud.show_buildings = !self.hud.show_buildings,
            "F3" => self.hud.show_resources = !self.hud.show_resources,
            "F4" => self.hud.show_diplomacy = !self.hud.show_diplomacy,
            "F6" => self.hud.show_animals = !self.hud.show_animals,
            "F7" => self.hud.show_caravans = !self.hud.show_caravans,
            "Tab" => self.hud.show_tech_tree = !self.hud.show_tech_tree,
            "[" => { self.hud.hud_font_size = (self.hud.hud_font_size - 1.0).max(7.0); }
            "]" => { self.hud.hud_font_size = (self.hud.hud_font_size + 1.0).min(20.0); }
            _ => {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;

    let canvas: HtmlCanvasElement = document
        .query_selector("#game")?
        .ok_or("no canvas element")?
        .dyn_into()
        .map_err(|_| "canvas element mismatch")?;
    canvas.set_width((sim::W as f64 * CELL) as u32);
    canvas.set_height((sim::H as f64 * CELL) as u32);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .ok_or("no 2d context")?
        .dyn_into()
        .map_err(|_| "context mismatch")?;

    let terrain: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into()
        .map_err(|_| "terrain canvas mismatch")?;
    terrain.set_width((sim::W as f64 * CELL) as u32);
    terrain.set_height((sim::H as f64 * CELL) as u32);
    let terrain_ctx: CanvasRenderingContext2d = terrain
        .get_context("2d")?
        .ok_or("no terrain context")?
        .dyn_into()
        .map_err(|_| "terrain context mismatch")?;

    let btn_pause = document.get_element_by_id("btnPause").ok_or("no btnPause")?;
    let btn_bless = document.get_element_by_id("btnBless").ok_or("no btnBless")?;
    let btn_inspire = document.get_element_by_id("btnInspire").ok_or("no btnInspire")?;
    let btn_build = document.get_element_by_id("btnBuild").ok_or("no btnBuild")?;
    let btn_road = document.get_element_by_id("btnRoad").ok_or("no btnRoad")?;
    let speed_lbl = document.get_element_by_id("speedlbl").ok_or("no speedlbl")?;

    let seed = Date::now() as u64;
    let sim = Rc::new(RefCell::new(sim::Sim::new(seed)));
    render::draw_terrain(&terrain_ctx, &sim.borrow());

    let app = Rc::new(RefCell::new(App {
        sim: Rc::clone(&sim),
        ctx,
        terrain,
        terrain_ctx,
        paused: false,
        bless_mode: false,
        inspire_mode: false,
        build_mode: false,
        build_terrain: sim::Terrain::Forest,
        road_mode: false,
        pending_bless: Vec::new(),
        pending_build: Vec::new(),
        pending_road: Vec::new(),
        effects: Vec::new(),
        btn_pause: btn_pause.clone(),
        btn_bless: btn_bless.clone(),
        btn_inspire: btn_inspire.clone(),
        _btn_build: btn_build.clone(),
        _btn_road: btn_road.clone(),
        speed_lbl,
        speed: 2.0,
        acc: 0.0,
        last: Date::now(),
        fps_frames: 0,
        fps_time: 0.0,
        fps: 0.0,
        seed,
        zoom: 1.0,
        cam_x: sim::W as f64 * CELL / 2.0,
        cam_y: sim::H as f64 * CELL / 2.0,
        pointers: Vec::new(),
        drag_ptr: None,
        pinch_prev_dist: 0.0,
        pinch_prev_mx: 0.0,
        pinch_prev_my: 0.0,
        build_cursor: 0,
        build_flash: None,
        selected_town: None,
        hud: render::HudConfig::default(),
    }));
    app.borrow().sync_ui();

    let app_key = Rc::clone(&app);
    let key: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new(
        move |e: KeyboardEvent| {
            app_key.borrow_mut().on_key(e);
        },
    ) as Box<dyn FnMut(KeyboardEvent)>);
    window
        .add_event_listener_with_callback("keydown", key.as_ref().unchecked_ref::<Function>())?;
    std::mem::forget(key);

    {
        let app_pt = Rc::clone(&app);
        let canvas_c = canvas.clone();
        let pd = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            let id = e.pointer_id();
            let (px, py) = app_pt.borrow().canvas_px(e.client_x() as f64, e.client_y() as f64).unwrap_or((0.0, 0.0));
            let mut app = app_pt.borrow_mut();
            app.pointers.push((id, px as i32, py as i32));
            if app.pointers.len() == 1 {
                app.drag_ptr = Some(id);
                if app.bless_mode {
                    app.bless_at(e.client_x(), e.client_y());
                } else if app.inspire_mode {
                    app.inspire_at(e.client_x(), e.client_y());
                } else if app.build_mode {
                    app.build_terrain_at(e.client_x(), e.client_y());
                } else if app.road_mode {
                    app.road_at(e.client_x(), e.client_y());
                } else {
                    app.select_town_at(e.client_x(), e.client_y());
                }
            } else {
                app.drag_ptr = None;
                if app.pointers.len() == 2 {
                    app.pinch_prev_dist = 0.0;
                    app.handle_pinch();
                }
            }
            let _ = canvas_c.set_pointer_capture(id);
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointerdown", pd.as_ref().unchecked_ref())?;
        std::mem::forget(pd);
    }
    {
        let app_pt = Rc::clone(&app);
        let pm = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            let id = e.pointer_id();
            let (px, py) = app_pt.borrow().canvas_px(e.client_x() as f64, e.client_y() as f64).unwrap_or((0.0, 0.0));
            let mut app = app_pt.borrow_mut();
            let (prev_x, prev_y) = app.update_pointer(id, px, py);
            if app.pointers.len() == 2 {
                app.handle_pinch();
            } else if app.pointers.len() == 1 && app.drag_ptr == Some(id) {
                if app.bless_mode {
                    app.bless_at(e.client_x(), e.client_y());
                } else if app.inspire_mode {
                    // остаёмся на месте: тап применяет идею, панорама отключена
                } else if app.build_mode {
                    app.build_terrain_at(e.client_x(), e.client_y());
                } else if app.road_mode {
                    app.road_at(e.client_x(), e.client_y());
                } else {
                    app.pan_by(px - prev_x, py - prev_y);
                }
            }
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointermove", pm.as_ref().unchecked_ref())?;
        std::mem::forget(pm);
    }
    {
        let app_pt = Rc::clone(&app);
        let up = Closure::wrap(Box::new(move |e: PointerEvent| {
            let id = e.pointer_id();
            let mut app = app_pt.borrow_mut();
            app.pointers.retain(|p| p.0 != id);
            if app.pointers.len() < 2 {
                app.pinch_prev_dist = 0.0;
            }
            if app.pointers.is_empty() {
                app.drag_ptr = None;
            }
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointerup", up.as_ref().unchecked_ref())?;
        canvas.add_event_listener_with_callback("pointercancel", up.as_ref().unchecked_ref())?;
        std::mem::forget(up);
    }
    {
        let app_w = Rc::clone(&app);
        let wheel = Closure::wrap(Box::new(move |e: WheelEvent| {
            e.prevent_default();
            let mut app = app_w.borrow_mut();
            let f = if e.delta_y() < 0.0 { 1.12 } else { 1.0 / 1.12 };
            if let Some((mx, my)) = app.canvas_px(e.client_x() as f64, e.client_y() as f64) {
                app.zoom_at(mx, my, f);
            }
        }) as Box<dyn FnMut(WheelEvent)>);
        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_passive(false);
        let canvas_w = canvas.clone();
        let et: &EventTarget = canvas_w.as_ref();
        et.add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            wheel.as_ref().unchecked_ref(),
            &opts,
        )?;
        std::mem::forget(wheel);
    }

    fn bind_click(elem: &Element, app: &Rc<RefCell<App>>, f: fn(&mut App)) -> Result<(), JsValue> {
        let app_cb = Rc::clone(app);
        let cb = Closure::wrap(Box::new(move || {
            f(&mut app_cb.borrow_mut());
        }) as Box<dyn FnMut()>);
        elem.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref::<Function>())?;
        std::mem::forget(cb);
        Ok(())
    }

    bind_click(&btn_pause, &app, |a| a.toggle_pause())?;
    bind_click(&btn_bless, &app, |a| a.toggle_bless())?;
    bind_click(&btn_inspire, &app, |a| a.toggle_inspire())?;
    bind_click(&btn_build, &app, |a| a.toggle_build())?;
    bind_click(&btn_road, &app, |a| a.toggle_road_mode())?;
    bind_click(&document.get_element_by_id("btnWeather").ok_or("no btnWeather")?, &app, |a| {
        a.cycle_weather()
    })?;
    bind_click(&document.get_element_by_id("btnNew").ok_or("no btnNew")?, &app, |a| {
        a.new_world()
    })?;
    bind_click(&document.get_element_by_id("btnSlower").ok_or("no btnSlower")?, &app, |a| {
        a.change_speed(-1.0)
    })?;
    bind_click(&document.get_element_by_id("btnFaster").ok_or("no btnFaster")?, &app, |a| {
        a.change_speed(1.0)
    })?;
    bind_click(&document.get_element_by_id("btnHouse").ok_or("no btnHouse")?, &app, |a| {
        a.build(sim::BuildingKind::House)
    })?;
    bind_click(&document.get_element_by_id("btnWell").ok_or("no btnWell")?, &app, |a| {
        a.build(sim::BuildingKind::Well)
    })?;
    bind_click(&document.get_element_by_id("btnPost").ok_or("no btnPost")?, &app, |a| {
        a.build(sim::BuildingKind::TradePost)
    })?;
    bind_click(&document.get_element_by_id("btnFarm").ok_or("no btnFarm")?, &app, |a| {
        a.build(sim::BuildingKind::Farm)
    })?;
    bind_click(&document.get_element_by_id("btnChurch").ok_or("no btnChurch")?, &app, |a| {
        a.build(sim::BuildingKind::Sanctuary)
    })?;
    bind_click(&document.get_element_by_id("btnClinic").ok_or("no btnClinic")?, &app, |a| {
        a.build(sim::BuildingKind::Clinic)
    })?;
    bind_click(&document.get_element_by_id("btnWall").ok_or("no btnWall")?, &app, |a| {
        a.build(sim::BuildingKind::Wall)
    })?;
    bind_click(&document.get_element_by_id("btnBarracks").ok_or("no btnBarracks")?, &app, |a| {
        a.build(sim::BuildingKind::Barracks)
    })?;
    bind_click(&document.get_element_by_id("btnTemple").ok_or("no btnTemple")?, &app, |a| {
        a.build(sim::BuildingKind::Temple)
    })?;
    bind_click(&document.get_element_by_id("btnWarehouse").ok_or("no btnWarehouse")?, &app, |a| {
        a.build(sim::BuildingKind::Warehouse)
    })?;
    bind_click(&document.get_element_by_id("btnSawmill").ok_or("no btnSawmill")?, &app, |a| {
        a.build(sim::BuildingKind::Sawmill)
    })?;
    bind_click(&document.get_element_by_id("btnCow").ok_or("no btnCow")?, &app, |a| {
        a.breed_cows()
    })?;
    bind_click(&document.get_element_by_id("btnHud").ok_or("no btnHud")?, &app, |a| {
        a.hud.cycle();
    })?;
    bind_click(&document.get_element_by_id("btnSave").ok_or("no btnSave")?, &app, |a| {
        a.save_to_local();
        a.download_save();
    })?;

    let app_loop = Rc::clone(&app);
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    {
        let mut g = f.borrow_mut();
        let f2 = Rc::clone(&f);
        *g = Some(Closure::wrap(Box::new(move || {
            let now = Date::now();
            app_loop.borrow_mut().step(now);
            if let Some(w) = web_sys::window() {
                if let Some(cb) = f2.borrow().as_ref() {
                    let _ = w.request_animation_frame(cb.as_ref().unchecked_ref::<Function>());
                }
            }
        }) as Box<dyn FnMut()>));
    }
    if let Some(w) = web_sys::window() {
        let g = f.borrow();
        if let Some(cb) = g.as_ref() {
            let _ = w.request_animation_frame(cb.as_ref().unchecked_ref::<Function>())?;
        }
    }

    Ok(())
}