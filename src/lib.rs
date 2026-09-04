mod render;
mod sim;

use js_sys::{Date, Function};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Element, HtmlCanvasElement, KeyboardEvent, PointerEvent,
};

const CELL: f64 = 8.0;
const BLESS_R: i32 = 3;

fn to_grid(canvas: &HtmlCanvasElement, client_x: f64, client_y: f64) -> Option<(usize, usize)> {
    let r = canvas.get_bounding_client_rect();
    let w = r.width();
    let h = r.height();
    if w <= 0.0 || h <= 0.0 {
        return None;
    }
    let scale = w / (sim::W as f64 * CELL);
    let gx = ((client_x - r.left()) / scale / CELL) as i32;
    let gy = ((client_y - r.top()) / scale / CELL) as i32;
    if gx >= 0 && gy >= 0 && gx < sim::W as i32 && gy < sim::H as i32 {
        Some((gx as usize, gy as usize))
    } else {
        None
    }
}

struct App {
    sim: Rc<RefCell<sim::Sim>>,
    ctx: CanvasRenderingContext2d,
    terrain: HtmlCanvasElement,
    terrain_ctx: CanvasRenderingContext2d,
    paused: bool,
    bless_mode: bool,
    pending_bless: Vec<(usize, usize)>,
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
}

impl App {
    fn step(&mut self, now: f64) {
        if !self.pending_bless.is_empty() {
            let pts: Vec<(usize, usize)> = self.pending_bless.drain(..).collect();
            let mut sim = self.sim.borrow_mut();
            for (x, y) in pts {
                sim.bless(x as i32, y as i32, BLESS_R);
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
        render::draw(
            &self.ctx,
            &self.terrain,
            &self.sim.borrow(),
            self.paused,
            self.speed,
            self.fps,
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
        let s = sim::Sim::new(self.seed);
        render::draw_terrain(&self.terrain_ctx, &s);
        self.sim = Rc::new(RefCell::new(s));
        self.acc = 0.0;
    }

    fn bless_at(&mut self, client_x: i32, client_y: i32) {
        if self.bless_mode {
            if let Some((x, y)) = to_grid(&self.terrain, client_x as f64, client_y as f64) {
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

    let drag = Rc::new(RefCell::new(false));
    {
        let app_pt = Rc::clone(&app);
        let drag_pt = Rc::clone(&drag);
        let pd = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            app_pt.borrow_mut().bless_at(e.client_x(), e.client_y());
            *drag_pt.borrow_mut() = true;
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointerdown", pd.as_ref().unchecked_ref())?;
        std::mem::forget(pd);
    }
    {
        let app_pt = Rc::clone(&app);
        let drag_pt = Rc::clone(&drag);
        let pm = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            if *drag_pt.borrow() {
                app_pt.borrow_mut().bless_at(e.client_x(), e.client_y());
            }
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointermove", pm.as_ref().unchecked_ref())?;
        std::mem::forget(pm);
    }
    {
        let drag_pt = Rc::clone(&drag);
        let up = Closure::wrap(Box::new(move |_e: PointerEvent| {
            *drag_pt.borrow_mut() = false;
        }) as Box<dyn FnMut(PointerEvent)>);
        canvas.add_event_listener_with_callback("pointerup", up.as_ref().unchecked_ref())?;
        canvas.add_event_listener_with_callback("pointercancel", up.as_ref().unchecked_ref())?;
        std::mem::forget(up);
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