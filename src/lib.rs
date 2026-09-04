mod render;
mod sim;

use js_sys::{Date, Function};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Element, EventTarget, HtmlCanvasElement, KeyboardEvent, PointerEvent,
    WheelEvent,
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
    pending_bless: Vec<(usize, usize)>,
    effects: Vec<render::Fx>,
    btn_pause: Element,
    btn_bless: Element,
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
        );
    }

    fn sync_ui(&self) {
        self.btn_pause
            .set_inner_html(if self.paused { "▶" } else { "⏸" });
        self.btn_bless
            .set_inner_html(if self.bless_mode { "🌱ON" } else { "🌱" });
        self.speed_lbl.set_inner_html(&format!("x{:.1}", self.speed));
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.sync_ui();
    }

    fn toggle_bless(&mut self) {
        self.bless_mode = !self.bless_mode;
        self.sync_ui();
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

    fn on_key(&mut self, e: KeyboardEvent) {
        match e.key().as_str() {
            " " => self.toggle_pause(),
            "+" | "=" => self.change_speed(1.0),
            "-" | "_" | "—" => self.change_speed(-1.0),
            "b" | "B" | "и" | "И" => self.toggle_bless(),
            "r" | "R" | "к" | "К" => self.new_world(),
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
        pending_bless: Vec::new(),
        effects: Vec::new(),
        btn_pause: btn_pause.clone(),
        btn_bless: btn_bless.clone(),
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
    bind_click(&document.get_element_by_id("btnNew").ok_or("no btnNew")?, &app, |a| {
        a.new_world()
    })?;
    bind_click(&document.get_element_by_id("btnSlower").ok_or("no btnSlower")?, &app, |a| {
        a.change_speed(-1.0)
    })?;
    bind_click(&document.get_element_by_id("btnFaster").ok_or("no btnFaster")?, &app, |a| {
        a.change_speed(1.0)
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