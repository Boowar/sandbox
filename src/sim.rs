pub const W: usize = 150;
pub const H: usize = 96;
pub const TICK_DT: f64 = 0.08;

const FOOD_MAX: f32 = 10.0;
const SEEK_RADIUS: i32 = 26;
const HOME_BOUND: f32 = 14.0;
const CARRY_LIMIT: f32 = 20.0;
const HUNGRY_AT: f32 = 60.0;
const STARVE: f32 = 100.0;
const BIRTH_EVERY: u64 = 220;
const REGROW_EVERY: u64 = 22;
const MAX_AGENTS: usize = 300;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Terrain {
    Grass,
    Forest,
    Water,
}

impl Terrain {
    fn walkable(self) -> bool {
        !matches!(self, Terrain::Water)
    }
}

#[derive(Clone)]
pub struct Cell {
    pub terrain: Terrain,
    pub food: f32,
}

pub struct Agent {
    pub home: usize,
    pub x: i32,
    pub y: i32,
    pub hunger: f32,
    pub energy: f32,
    pub carried: f32,
}

pub struct Settlement {
    pub x: i32,
    pub y: i32,
    pub stockpile: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

enum Action {
    Move(i32, i32),
    Stay,
    Eat,
    Deposit,
    Die,
}

pub struct Sim {
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
    pub towns: Vec<Settlement>,
    pub tick_count: u64,
    rng: u64,
}

fn idx(x: i32, y: i32) -> usize {
    y as usize * W + x as usize
}

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < W as i32 && y >= 0 && y < H as i32
}

fn rnd(rng: &mut u64) -> u32 {
    *rng ^= *rng << 13;
    *rng ^= *rng >> 7;
    *rng ^= *rng << 17;
    (*rng >> 32) as u32
}

fn rfrac(rng: &mut u64) -> f32 {
    rnd(rng) as f32 / u32::MAX as f32
}

impl Sim {
    pub fn new(seed: u64) -> Self {
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let grid = Self::make_terrain(&mut rng);
        let mut sim = Sim {
            grid,
            agents: Vec::new(),
            towns: Vec::new(),
            tick_count: 0,
            rng,
        };
        sim.spawn_world();
        sim
    }

    fn make_terrain(rng: &mut u64) -> Vec<Cell> {
        let mut grid = vec![
            Cell { terrain: Terrain::Grass, food: FOOD_MAX };
            W * H
        ];
        for cell in grid.iter_mut() {
            let p = rfrac(rng);
            cell.terrain = if p < 0.22 {
                Terrain::Water
            } else if p < 0.46 {
                Terrain::Forest
            } else {
                Terrain::Grass
            };
            cell.food = FOOD_MAX;
        }
        for _ in 0..4 {
            let mut next = grid.clone();
            for y in 0..H {
                for x in 0..W {
                    let i = idx(x as i32, y as i32);
                    let t = grid[i].terrain;
                    let mut water = 0;
                    let mut tree = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if !in_bounds(nx, ny) {
                                continue;
                            }
                            match grid[idx(nx, ny)].terrain {
                                Terrain::Water => water += 1,
                                Terrain::Forest => tree += 1,
                                Terrain::Grass => {}
                            }
                        }
                    }
                    next[i].terrain = match t {
                        Terrain::Water => {
                            if water >= 5 { Terrain::Water } else { Terrain::Grass }
                        }
                        Terrain::Forest => {
                            if tree >= 4 && water < 4 { Terrain::Forest } else { Terrain::Grass }
                        }
                        Terrain::Grass => {
                            if water >= 5 { Terrain::Water } else { Terrain::Grass }
                        }
                    };
                }
            }
            grid = next;
        }
        grid
    }

    fn spawn_world(&mut self) {
        const PALETTE: [(u8, u8, u8); 3] = [(255, 209, 102), (6, 214, 160), (239, 71, 111)];
        let per = W as i32 / (PALETTE.len() + 1) as i32;
        for (i, &(r, g, b)) in PALETTE.iter().enumerate() {
            let cx = per * (i as i32 + 1);
            let cy = H as i32 / 2;
            for dy in -4..=4 {
                for dx in -4..=4 {
                    let x = cx + dx;
                    let y = cy + dy;
                    if in_bounds(x, y) {
                        self.grid[idx(x, y)].terrain = Terrain::Grass;
                    }
                }
            }
            self.towns.push(Settlement {
                x: cx,
                y: cy,
                stockpile: 80.0,
                r,
                g,
                b,
            });
            let n = (rnd(&mut self.rng) % 14 + 12) as usize;
            for _ in 0..n {
                self.spawn_agent(i, cx, cy);
            }
        }
    }

    fn spawn_agent(&mut self, home: usize, cx: i32, cy: i32) {
        for _ in 0..64 {
            let ang = rfrac(&mut self.rng) * 6.2832;
            let r = rfrac(&mut self.rng) * 6.0 + 2.0;
            let x = cx + (ang.cos() * r) as i32;
            let y = cy + (ang.sin() * r) as i32;
            if in_bounds(x, y) && self.grid[idx(x, y)].terrain.walkable() {
                self.agents.push(Agent {
                    home,
                    x,
                    y,
                    hunger: rfrac(&mut self.rng) * 20.0,
                    energy: 80.0 + rfrac(&mut self.rng) * 20.0,
                    carried: 0.0,
                });
                return;
            }
        }
        self.agents.push(Agent {
            home,
            x: cx,
            y: cy,
            hunger: 10.0,
            energy: 90.0,
            carried: 0.0,
        });
    }

    pub fn pop(&self, ti: usize) -> usize {
        self.agents.iter().filter(|a| a.home == ti).count()
    }

    pub fn bless(&mut self, gx: i32, gy: i32, r: i32) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy > r * r {
                    continue;
                }
                let nx = gx + dx;
                let ny = gy + dy;
                if !in_bounds(nx, ny) {
                    continue;
                }
                let c = &mut self.grid[idx(nx, ny)];
                match c.terrain {
                    Terrain::Grass => {
                        c.terrain = Terrain::Forest;
                        c.food = FOOD_MAX;
                    }
                    Terrain::Forest => c.food = FOOD_MAX,
                    Terrain::Water => {}
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        for a in self.agents.iter_mut() {
            a.hunger = (a.hunger + 1.1).min(140.0);
        }

        if self.tick_count % REGROW_EVERY == 0 {
            for cell in self.grid.iter_mut() {
                if cell.terrain == Terrain::Forest {
                    cell.food = (cell.food + 1.0).min(FOOD_MAX);
                }
            }
        }

        let actions: Vec<Action> = self.agents.iter().map(|a| self.decide(a)).collect();
        let mut dead = Vec::new();
        for (i, act) in actions.into_iter().enumerate() {
            self.apply(i, act, &mut dead);
        }
        for &i in dead.iter().rev() {
            self.agents.remove(i);
        }

        if self.tick_count % BIRTH_EVERY == 0 {
            self.births();
        }
    }

    fn decide(&self, a: &Agent) -> Action {
        if a.hunger >= STARVE {
            return Action::Die;
        }
        if a.energy <= 6.0 {
            return Action::Stay;
        }
        let t = &self.towns[a.home];
        let (hx, hy) = (t.x, t.y);
        let at_home = (a.x - hx).abs() <= 1 && (a.y - hy).abs() <= 1;

        if a.carried >= CARRY_LIMIT {
            if at_home {
                Action::Deposit
            } else {
                let (nx, ny) = self.steer(a, hx, hy);
                Action::Move(nx, ny)
            }
        } else if a.hunger >= HUNGRY_AT {
            if at_home {
                if self.towns[a.home].stockpile > 0.0 {
                    Action::Eat
                } else {
                    Action::Stay
                }
            } else if a.carried > 0.0 {
                let (nx, ny) = self.steer(a, hx, hy);
                Action::Move(nx, ny)
            } else if let Some((fx, fy)) = self.food_target(a.x, a.y) {
                let (nx, ny) = self.steer(a, fx, fy);
                Action::Move(nx, ny)
            } else {
                self.wander(a)
            }
        } else if let Some((fx, fy)) = self.food_target(a.x, a.y) {
            let (nx, ny) = self.steer(a, fx, fy);
            Action::Move(nx, ny)
        } else {
            self.wander(a)
        }
    }

    fn steer(&self, a: &Agent, tx: i32, ty: i32) -> (i32, i32) {
        let mut best = (a.x, a.y);
        let mut bs = i32::MIN;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = a.x + dx;
                let ny = a.y + dy;
                if !in_bounds(nx, ny) || !self.grid[idx(nx, ny)].terrain.walkable() {
                    continue;
                }
                let d = (nx - tx).pow(2) + (ny - ty).pow(2);
                let sc = -d + Self::jitter(a, self.tick_count);
                if sc > bs {
                    bs = sc;
                    best = (nx, ny);
                }
            }
        }
        best
    }

    fn wander(&self, a: &Agent) -> Action {
        let t = &self.towns[a.home];
        let d = ((a.x - t.x).pow(2) as f32 + (a.y - t.y).pow(2) as f32).sqrt();
        if d > HOME_BOUND {
            let (nx, ny) = self.steer(a, t.x, t.y);
            Action::Move(nx, ny)
        } else {
            for k in 0..8u32 {
                let h = (a.x as u32).wrapping_mul(0x21f0_aaad)
                    ^ (a.y as u32 & 1).wrapping_mul(k + 1)
                    ^ (self.tick_count as u32).wrapping_mul(0x0100_0101)
                    ^ a.home as u32;
                let dx = (h % 3) as i32 - 1;
                let dy = ((h >> 2) % 3) as i32 - 1;
                let nx = a.x + dx;
                let ny = a.y + dy;
                if in_bounds(nx, ny) && self.grid[idx(nx, ny)].terrain.walkable() {
                    return Action::Move(nx, ny);
                }
            }
            Action::Stay
        }
    }

    fn jitter(a: &Agent, tick: u64) -> i32 {
        let h = (a.x as u32).wrapping_mul(0x45d9_f3b)
            ^ (a.y as u32).wrapping_mul(0x119d_e1f3)
            ^ (tick as u32).wrapping_mul(0xabcd_ef01)
            ^ a.home as u32;
        (h % 7) as i32 - 3
    }

    fn food_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        for r in 1..=SEEK_RADIUS {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs() != r && dy.abs() != r {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if in_bounds(nx, ny) {
                        let c = &self.grid[idx(nx, ny)];
                        if c.terrain == Terrain::Forest && c.food > 0.5 {
                            return Some((nx, ny));
                        }
                    }
                }
            }
        }
        None
    }

    fn apply(&mut self, i: usize, act: Action, dead: &mut Vec<usize>) {
        match act {
            Action::Move(nx, ny) => {
                let a = &mut self.agents[i];
                if a.energy <= 0.0 {
                    return;
                }
                a.x = nx;
                a.y = ny;
                a.energy -= 0.6;
                let c = &self.grid[idx(nx, ny)];
                if a.carried < CARRY_LIMIT && c.terrain == Terrain::Forest && c.food > 0.5 {
                    a.carried = (a.carried + 2.0).min(CARRY_LIMIT);
                    self.grid[idx(nx, ny)].food -= 1.0;
                }
            }
            Action::Stay => {
                let a = &mut self.agents[i];
                a.energy = (a.energy + 6.0).min(100.0);
            }
            Action::Eat => {
                let ti = self.agents[i].home;
                if self.towns[ti].stockpile > 0.0 {
                    self.towns[ti].stockpile -= 2.0;
                    let a = &mut self.agents[i];
                    a.hunger = (a.hunger - 30.0).max(0.0);
                    a.energy = (a.energy + 4.0).min(100.0);
                }
            }
            Action::Deposit => {
                let ti = self.agents[i].home;
                let a = &mut self.agents[i];
                self.towns[ti].stockpile += a.carried;
                a.carried = 0.0;
                a.hunger = (a.hunger - 5.0).max(0.0);
            }
            Action::Die => dead.push(i),
        }
    }

    fn births(&mut self) {
        if self.agents.len() >= MAX_AGENTS {
            return;
        }
        for ti in 0..self.towns.len() {
            let (mut stock, tx, ty) = {
                let t = &self.towns[ti];
                (t.stockpile, t.x, t.y)
            };
            if self.pop(ti) >= 12 || stock < 30.0 {
                continue;
            }
            stock -= 15.0;
            self.towns[ti].stockpile = stock;
            self.spawn_agent(ti, tx, ty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn total_food(s: &Sim) -> f32 {
        s.towns.iter().map(|t| t.stockpile).sum()
    }

    #[test]
    fn same_seed_same_world() {
        let a = Sim::new(42);
        let b = Sim::new(42);
        assert_eq!(a.grid.len(), b.grid.len());
        for i in 0..a.grid.len() {
            assert_eq!(a.grid[i].terrain, b.grid[i].terrain);
            assert_eq!(a.grid[i].food.to_bits(), b.grid[i].food.to_bits());
        }
        assert_eq!(a.agents.len(), b.agents.len());
        for i in 0..a.agents.len() {
            assert_eq!(a.agents[i].x, b.agents[i].x);
            assert_eq!(a.agents[i].y, b.agents[i].y);
            assert_eq!(a.agents[i].home, b.agents[i].home);
        }
    }

    #[test]
    fn simulation_is_deterministic() {
        let mut a = Sim::new(7);
        let mut b = Sim::new(7);
        for _ in 0..500 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.agents.len(), b.agents.len());
        for (i, j) in a.agents.iter().zip(b.agents.iter()) {
            assert_eq!(i.x, j.x);
            assert_eq!(i.y, j.y);
            assert_eq!(i.hunger.to_bits(), j.hunger.to_bits());
            assert_eq!(i.energy.to_bits(), j.energy.to_bits());
            assert_eq!(i.carried.to_bits(), j.carried.to_bits());
        }
        let fa: Vec<u32> = a.towns.iter().map(|t| t.stockpile.to_bits()).collect();
        let fb: Vec<u32> = b.towns.iter().map(|t| t.stockpile.to_bits()).collect();
        assert_eq!(fa, fb);
    }

    #[test]
    fn world_has_towns_and_agents() {
        let s = Sim::new(5);
        assert!(s.towns.len() >= 3);
        assert!(s.agents.len() > 20);
    }

    #[test]
    fn agents_stay_walkable_in_bounds_and_home_valid() {
        let mut s = Sim::new(99);
        for _ in 0..1000 {
            s.tick();
            for a in &s.agents {
                assert!(a.x >= 0 && a.x < W as i32);
                assert!(a.y >= 0 && a.y < H as i32);
                assert!(a.home < s.towns.len());
                assert!(s.grid[idx(a.x, a.y)].terrain.walkable());
            }
        }
    }

    #[test]
    fn agents_never_start_or_end_on_water() {
        let mut s = Sim::new(11);
        for a in &s.agents {
            assert!(s.grid[idx(a.x, a.y)].terrain.walkable());
        }
        for _ in 0..500 {
            s.tick();
        }
        for a in &s.agents {
            assert!(s.grid[idx(a.x, a.y)].terrain.walkable());
        }
    }

    #[test]
    fn bless_creates_forest_with_food() {
        let mut s = Sim::new(3);
        let (x, y) = (s.towns[0].x, s.towns[0].y);
        s.bless(x, y, 2);
        for dy in -2..=2 {
            for dx in -2..=2 {
                if dx * dx + dy * dy > 4 {
                    continue;
                }
                let cx = x + dx;
                let cy = y + dy;
                assert!(in_bounds(cx, cy));
                assert_eq!(s.grid[idx(cx, cy)].terrain, Terrain::Forest);
                assert_eq!(s.grid[idx(cx, cy)].food, FOOD_MAX);
            }
        }
    }

    #[test]
    fn gathering_increases_stockpile() {
        let mut s = Sim::new(1);
        let stock0 = s.towns[0].stockpile;
        s.bless(s.towns[0].x, s.towns[0].y, 12);
        let mut deposited = false;
        for _ in 0..3000 {
            s.tick();
            if s.towns[0].stockpile > stock0 + 10.0 {
                deposited = true;
                break;
            }
        }
        assert!(deposited, "stockpile should grow from gathering");
    }

    #[test]
    fn starvation_keeps_population_sane() {
        let mut s = Sim::new(6);
        let before = s.agents.len();
        for _ in 0..2000 {
            s.tick();
        }
        assert!(s.agents.len() <= before * 4 + 10);
        assert!(total_food(&s) >= 0.0);
    }
}