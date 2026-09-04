pub const W: usize = 150;
pub const H: usize = 96;
pub const TICK_DT: f64 = 0.08;

const FOOD_MAX: f32 = 10.0;
const ORE_MAX: f32 = 60.0;
const SEEK_RADIUS: i32 = 26;
const HOME_BOUND: f32 = 14.0;
const HUNGRY_AT: f32 = 60.0;
const THIRSTY_AT: f32 = 60.0;
const STARVE: f32 = 100.0;
const BIRTH_EVERY: u64 = 220;
const REGROW_EVERY: u64 = 22;
const MAX_AGENTS: usize = 300;
const BIRTH_MIN_FOOD: f32 = 30.0;
const BIRTH_MIN_WATER: f32 = 20.0;
const BIRTH_FOOD: f32 = 15.0;
const BIRTH_WATER: f32 = 10.0;
const BUILD_MIN_FOOD: f32 = 20.0;
const BUILD_MIN_WATER: f32 = 10.0;
const HOUSE_COST: f32 = 30.0;
const WELL_COST: f32 = 20.0;
const HOUSE_CAP_BONUS: usize = 4;
const WAR_START_FOOD: f32 = 55.0;
const WAR_START_WATER: f32 = 40.0;
const WAR_START_POP: usize = 8;
const WAR_START_TOWN_RANGE: f32 = 60.0;
const ARMY_TARGETS_POP: usize = 8;
const RAISE_FOOD: f32 = 20.0;
const RAID_CHANCE_PER_TICK: f32 = 0.06;
const RAID_TARGET_POP: usize = 6;
const WEATHER_PLAYER_TIME: f64 = 640.0;
const IDEA_TIME: f64 = 1200.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Weather {
    Clear,
    Rain,
    Heat,
    Frost,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TownIdea {
    None,
    War,
    Prosperity,
    Toil,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Role {
    Worker,
    Farmer,
    Miner,
}
const PEACE_CHANCE_PER_TICK: f32 = 0.02;
const PEACE_FOOD_WATER_MIN: f32 = 70.0;
const WELL_WATER_PER_TICK: f32 = 0.3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Terrain {
    Grass,
    Forest,
    Hills,
    Water,
}

impl Terrain {
    fn walkable(self) -> bool {
        !matches!(self, Terrain::Water)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ResourceKind {
    Food,
    Water,
    Ore,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BuildingKind {
    House,
    Well,
}

impl BuildingKind {
    pub fn cost(self) -> f32 {
        match self {
            BuildingKind::House => HOUSE_COST,
            BuildingKind::Well => WELL_COST,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Stock {
    pub food: f32,
    pub water: f32,
    pub ore: f32,
}

#[derive(Clone)]
pub struct Cell {
    pub terrain: Terrain,
    pub food: f32,
    pub ore: f32,
}

pub struct Agent {
    pub home: usize,
    pub x: i32,
    pub y: i32,
    pub dir_x: i32,
    pub dir_y: i32,
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub want: ResourceKind,
    pub carry: Option<(ResourceKind, f32)>,
    pub family: usize,
    pub founder: bool,
    pub raider: bool,
    pub target_town: Option<usize>,
    pub role: Role,
}

pub struct Settlement {
    pub x: i32,
    pub y: i32,
    pub stocks: Stock,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub cap: usize,
    pub queue: Vec<(BuildingKind, f32)>,
    pub built: Vec<BuildingKind>,
    pub at_war: bool,
    pub raiders: u32,
    pub enemy: Option<usize>,
    pub idea: TownIdea,
    pub idea_left: f64,
}

pub struct Family {
    pub id: usize,
    pub town: usize,
    pub members: u32,
    pub children: u32,
    pub name: String,
    pub extinct: bool,
    pub accent: (u8, u8, u8),
    pub role: Role,
}

enum Action {
    Move(i32, i32),
    Stay,
    Eat,
    Drink,
    Deposit,
    Die,
}

pub struct Sim {
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
    pub towns: Vec<Settlement>,
    pub families: Vec<Family>,
    pub tick_count: u64,
    pub weather: Weather,
    pub weather_left: f64,
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
            families: Vec::new(),
            tick_count: 0,
            weather: Weather::Clear,
            weather_left: 0.0,
            rng,
        };
        sim.ensure_hills();
        sim.spawn_world();
        sim
    }

    fn make_terrain(rng: &mut u64) -> Vec<Cell> {
        let mut grid = vec![
            Cell { terrain: Terrain::Grass, food: FOOD_MAX, ore: 0.0 };
            W * H
        ];
        for cell in grid.iter_mut() {
            let p = rfrac(rng);
            cell.terrain = if p < 0.38 {
                Terrain::Forest
            } else if p < 0.55 {
                Terrain::Hills
            } else {
                Terrain::Grass
            };
            cell.food = FOOD_MAX;
            cell.ore = if cell.terrain == Terrain::Hills { ORE_MAX } else { 0.0 };
        }
        Self::carve_lakes(rng, &mut grid);
        for _ in 0..4 {
            let mut next = grid.clone();
            for y in 0..H {
                for x in 0..W {
                    let i = idx(x as i32, y as i32);
                    let t = grid[i].terrain;
                    let mut water = 0;
                    let mut tree = 0;
                    let mut hill = 0;
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
                                Terrain::Hills => hill += 1,
                                Terrain::Grass => {}
                            }
                        }
                    }
                    next[i].terrain = match t {
                        Terrain::Water => {
                            if water >= 2 && tree < 3 { Terrain::Water } else { Terrain::Grass }
                        }
                        Terrain::Forest => {
                            if tree >= 3 && water < 3 { Terrain::Forest } else { Terrain::Grass }
                        }
                        Terrain::Hills => {
                            if hill >= 2 && water < 4 { Terrain::Hills } else { Terrain::Grass }
                        }
                        Terrain::Grass => {
                            if water >= 4 {
                                Terrain::Water
                            } else if tree >= 5 && water < 3 && hill < 4 {
                                Terrain::Forest
                            } else if hill >= 4 && water < 3 && tree < 4 {
                                Terrain::Hills
                            } else {
                                Terrain::Grass
                            }
                        }
                    };
                    if next[i].terrain == Terrain::Hills && next[i].ore <= 0.0 {
                        next[i].ore = ORE_MAX;
                    }
                }
            }
            grid = next;
        }
        grid
    }

    fn carve_lakes(rng: &mut u64, grid: &mut Vec<Cell>) {
        let lakes = 4 + (rfrac(rng) * 3.0) as i32;
        for _ in 0..lakes {
            let radius: f64 = (5.0 + rfrac(rng) * 6.0) as f64;
            let target = (std::f64::consts::PI * radius * radius * 0.85) as i32;
            let margin = radius as i32 + 4;
            let cx = margin + (rfrac(rng) as f64 * (W as i32 - 2 * margin) as f64) as i32;
            let cy = margin + (rfrac(rng) as f64 * (H as i32 - 2 * margin) as f64) as i32;
            let mut queue = vec![(cx, cy)];
            queue.reverse();
            let mut done = 0;
            while let Some((x, y)) = queue.pop() {
                if done >= target {
                    break;
                }
                if !in_bounds(x, y) {
                    continue;
                }
                let i = idx(x, y);
                if grid[i].terrain == Terrain::Water {
                    continue;
                }
                let dx = x - cx;
                let dy = y - cy;
                if ((dx * dx + dy * dy) as f64).sqrt() > radius {
                    continue;
                }
                grid[i].terrain = Terrain::Water;
                grid[i].food = FOOD_MAX;
                grid[i].ore = 0.0;
                done += 1;
                queue.push((x + 1, y));
                queue.push((x - 1, y));
                queue.push((x, y + 1));
                queue.push((x, y - 1));
            }
        }
    }

    fn ensure_hills(&mut self) {
        let has_hill = self.grid.iter().any(|c| c.terrain == Terrain::Hills);
        if has_hill {
            return;
        }
        for (fx, fy) in [(W as i32 / 3, H as i32 / 3), (2 * W as i32 / 3, 2 * H as i32 / 3)] {
            for dy in -4..=4 {
                for dx in -4..=4 {
                    let x = fx + dx;
                    let y = fy + dy;
                    if in_bounds(x, y) {
                        let c = &mut self.grid[idx(x, y)];
                        c.terrain = Terrain::Hills;
                        c.ore = ORE_MAX;
                    }
                }
            }
        }
    }

    fn family_name(rng: &mut u64) -> String {
        const SYL_A: [&str; 16] = [
            "Ару", "Бер", "Вил", "Гар", "Дон", "Же", "Ир", "Йор", "Кел", "Лаш", "Мор", "Нюас",
            "Орм", "Пел", "Рай", "Сел",
        ];
        const SYL_B: [&str; 16] = [
            "дар", "рин", "валь", "тор", "гун", "жей", "мил", "холь", "зен", "фель", "кон", "сар",
            "мир", "таль", "нуар", "вик",
        ];
        let a = SYL_A[rnd(rng) as usize % SYL_A.len()];
        let b = SYL_B[rnd(rng) as usize % SYL_B.len()];
        format!("{}{}", a, b)
    }

    fn family_accent(found_count: usize, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        match found_count % 3 {
            0 => (r, g.min(120), 230),
            1 => (230, g, b.min(120)),
            _ => (r, g, b),
        }
    }

    fn spawn_world(&mut self) {
        const PALETTE: [(u8, u8, u8); 3] = [(255, 209, 102), (6, 214, 160), (239, 71, 111)];
        const FAMILIES_PER_TOWN: usize = 2;
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
                stocks: Stock { food: 80.0, water: 40.0, ore: 40.0 },
                r,
                g,
                b,
                cap: 12,
                queue: Vec::new(),
                built: Vec::new(),
                at_war: false,
                raiders: 0,
                enemy: None,
                idea: TownIdea::None,
                idea_left: 0.0,
            });
            let base = self.families.len();
            for k in 0..FAMILIES_PER_TOWN {
                self.families.push(Family {
                    id: base + k,
                    town: i,
                    members: 0,
                    children: 0,
                    name: Self::family_name(&mut self.rng),
                    extinct: false,
                    accent: Self::family_accent(k, r, g, b),
                    role: match (base + k) % 3 {
                        1 => Role::Farmer,
                        2 => Role::Miner,
                        _ => Role::Worker,
                    },
                });
            }
            let n = (rnd(&mut self.rng) % 14 + 12) as usize;
            for j in 0..n {
                let fam = base + j % FAMILIES_PER_TOWN;
                let founder = j < FAMILIES_PER_TOWN;
                self.spawn_agent(i, cx, cy, fam, founder);
            }
        }
    }

    fn spawn_agent(&mut self, home: usize, cx: i32, cy: i32, family: usize, founder: bool) {
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
                    dir_x: 0,
                    dir_y: 0,
                    hunger: rfrac(&mut self.rng) * 20.0,
                    thirst: rfrac(&mut self.rng) * 20.0,
                    energy: 80.0 + rfrac(&mut self.rng) * 20.0,
                    want: ResourceKind::Food,
                    carry: None,
                    family,
                    founder,
                    raider: false,
                    target_town: None,
                    role: self.families[family].role,
                });
                self.families[family].members += 1;
                return;
            }
        }
        self.agents.push(Agent {
            home,
            x: cx,
            y: cy,
            dir_x: 0,
            dir_y: 0,
            hunger: 10.0,
            thirst: 10.0,
            energy: 90.0,
            want: ResourceKind::Food,
            carry: None,
            family,
            founder,
            raider: false,
            target_town: None,
            role: self.families[family].role,
        });
        self.families[family].members += 1;
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
                    Terrain::Hills | Terrain::Water => {}
                }
            }
        }
    }

    pub fn build_request(&mut self, ti: usize, kind: BuildingKind) {
        if ti < self.towns.len() {
            self.towns[ti].queue.push((kind, 0.0));
        }
    }

    fn construction(&mut self) {
        for ti in 0..self.towns.len() {
            let apply = {
                let t = &mut self.towns[ti];
                if t.queue.is_empty() {
                    continue;
                }
                if t.stocks.food < BUILD_MIN_FOOD || t.stocks.water < BUILD_MIN_WATER {
                    continue;
                }
                if t.stocks.ore < 1.0 {
                    continue;
                }
                t.stocks.ore -= 1.0;
                let (kind, progress) = &mut t.queue[0];
                *progress += if t.idea == TownIdea::Toil { 2.0 } else { 1.0 };
                if *progress >= kind.cost() {
                    Some(t.queue.remove(0).0)
                } else {
                    None
                }
            };
            if let Some(k) = apply {
                let t = &mut self.towns[ti];
                t.built.push(k);
                if k == BuildingKind::House {
                    t.cap += HOUSE_CAP_BONUS;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.weather_breath();

        let hunger_rate = match self.weather {
            Weather::Frost => 1.3,
            _ => 1.1,
        };
        let thirst_rate = match self.weather {
            Weather::Rain => 0.5,
            Weather::Heat => 1.1,
            _ => 0.8,
        };
        for a in self.agents.iter_mut() {
            a.hunger = (a.hunger + hunger_rate).min(140.0);
            a.thirst = (a.thirst + thirst_rate).min(140.0);
        }

        if self.tick_count % REGROW_EVERY == 0 {
            let berry = match self.weather {
                Weather::Rain => 2.0,
                Weather::Frost => 0.5,
                _ => 1.0,
            };
            for cell in self.grid.iter_mut() {
                if cell.terrain == Terrain::Forest {
                    cell.food = (cell.food + berry).min(FOOD_MAX);
                } else if cell.terrain == Terrain::Hills {
                    cell.ore = (cell.ore + 0.5).min(ORE_MAX);
                }
            }
        }

        for t in self.towns.iter_mut() {
            let wells = t.built.iter().filter(|b| **b == BuildingKind::Well).count();
            if wells > 0 {
                t.stocks.water = (t.stocks.water + WELL_WATER_PER_TICK * wells as f32).min(200.0);
            }
            if self.weather == Weather::Heat {
                t.stocks.water = (t.stocks.water - 0.08).max(0.0);
            }
            if t.idea_left > 0.0 {
                t.idea_left -= 1.0;
                if t.idea_left <= 0.0 {
                    t.idea = TownIdea::None;
                }
            }
        }

        self.construction();

        let actions: Vec<(Action, ResourceKind)> = self.agents.iter().map(|a| self.decide(a)).collect();
        let mut dead = Vec::new();
        for (i, (act, want)) in actions.into_iter().enumerate() {
            self.agents[i].want = want;
            self.apply(i, act, &mut dead);
        }
        for &i in dead.iter().rev() {
            self.agents.remove(i);
        }
        self.release_dead_raiders(&dead);

        if self.tick_count % BIRTH_EVERY == 0 {
            self.reproduction();
        }
        self.sync_families();
        self.war_step();
    }

    fn weather_breath(&mut self) {
        if self.weather_left > 0.0 {
            self.weather_left -= 1.0;
            return;
        }
        let p = rfrac(&mut self.rng);
        self.weather = if p < 0.55 {
            Weather::Clear
        } else if p < 0.7 {
            Weather::Rain
        } else if p < 0.85 {
            Weather::Heat
        } else {
            Weather::Frost
        };
        self.weather_left = 300.0 + rfrac(&mut self.rng) as f64 * 400.0;
    }

    pub fn cycle_weather(&mut self) {
        self.weather = match self.weather {
            Weather::Clear => Weather::Rain,
            Weather::Rain => Weather::Heat,
            Weather::Heat => Weather::Frost,
            Weather::Frost => Weather::Clear,
        };
        self.weather_left = WEATHER_PLAYER_TIME;
    }

    pub fn inspire(&mut self, ti: usize) {
        if ti >= self.towns.len() {
            return;
        }
        let t = &mut self.towns[ti];
        t.idea = match t.idea {
            TownIdea::None => TownIdea::War,
            TownIdea::War => TownIdea::Prosperity,
            TownIdea::Prosperity => TownIdea::Toil,
            TownIdea::Toil => TownIdea::None,
        };
        t.idea_left = IDEA_TIME;
    }

    fn decide(&self, a: &Agent) -> (Action, ResourceKind) {
        if a.hunger >= STARVE || a.thirst >= STARVE {
            return (Action::Die, a.want);
        }
        if a.raider {
            return self.army_action(a);
        }
        let t = &self.towns[a.home];
        let (hx, hy) = (t.x, t.y);
        let at_home = (a.x - hx).abs() <= 1 && (a.y - hy).abs() <= 1;

        if let Some((kind, _)) = a.carry {
            if at_home {
                (Action::Deposit, kind)
            } else {
                let (nx, ny) = self.steer(a, hx, hy);
                (Action::Move(nx, ny), kind)
            }
        } else if at_home {
            if a.hunger >= HUNGRY_AT {
                if self.towns[a.home].stocks.food > 0.0 {
                    (Action::Eat, a.want)
                } else {
                    self.gather_action(a, Some(ResourceKind::Food))
                }
            } else if a.thirst >= THIRSTY_AT {
                if self.towns[a.home].stocks.water > 0.0 {
                    (Action::Drink, a.want)
                } else {
                    self.gather_action(a, Some(ResourceKind::Water))
                }
            } else {
                self.gather_action(a, None)
            }
        } else {
            self.gather_action(a, None)
        }
    }

    fn gather_action(&self, a: &Agent, force: Option<ResourceKind>) -> (Action, ResourceKind) {
        let kind = force.unwrap_or_else(|| self.role_kind(a));
        let d = match kind {
            ResourceKind::Food => self.food_target(a.x, a.y),
            ResourceKind::Water => self.water_target(a.x, a.y),
            ResourceKind::Ore => self.ore_target(a.x, a.y),
        };
        if let Some((fx, fy)) = d {
            let (nx, ny) = self.steer(a, fx, fy);
            (Action::Move(nx, ny), kind)
        } else if force.is_none() {
            let mn = self.most_needed(a.home);
            if mn != kind {
                self.gather_action(a, Some(mn))
            } else {
                let others: [ResourceKind; 2] = match kind {
                    ResourceKind::Food => [ResourceKind::Water, ResourceKind::Ore],
                    ResourceKind::Water => [ResourceKind::Food, ResourceKind::Ore],
                    ResourceKind::Ore => [ResourceKind::Food, ResourceKind::Water],
                };
                for k in others {
                    let d = match k {
                        ResourceKind::Food => self.food_target(a.x, a.y),
                        ResourceKind::Water => self.water_target(a.x, a.y),
                        ResourceKind::Ore => self.ore_target(a.x, a.y),
                    };
                    if let Some((fx, fy)) = d {
                        let (nx, ny) = self.steer(a, fx, fy);
                        return (Action::Move(nx, ny), k);
                    }
                }
                (self.wander(a), kind)
            }
        } else {
            (self.wander(a), kind)
        }
    }

    fn role_kind(&self, a: &Agent) -> ResourceKind {
        let need = self.most_needed(a.home);
        let st = &self.towns[a.home].stocks;
        let (kind, ok) = match a.role {
            Role::Worker => (need, true),
            Role::Farmer => (
                ResourceKind::Food,
                st.food > 25.0 && st.water > 8.0 && st.ore > 8.0,
            ),
            Role::Miner => (
                ResourceKind::Ore,
                st.ore < 120.0 && st.water > 8.0 && st.food > 8.0,
            ),
        };
        if ok && kind != need {
            kind
        } else {
            need
        }
    }

    fn most_needed(&self, ti: usize) -> ResourceKind {
        let s = &self.towns[ti].stocks;
        let f = s.food / BIRTH_MIN_FOOD;
        let w = s.water / BIRTH_MIN_WATER;
        let o = s.ore / 15.0;
        if w < f && w <= o {
            ResourceKind::Water
        } else if o < f {
            ResourceKind::Ore
        } else {
            ResourceKind::Food
        }
    }

    fn seek<F>(&self, x: i32, y: i32, pred: F) -> Option<(i32, i32)>
    where
        F: Fn(&Sim, i32, i32) -> bool,
    {
        for r in 1..=SEEK_RADIUS {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs() != r && dy.abs() != r {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if in_bounds(nx, ny) && pred(self, nx, ny) {
                        return Some((nx, ny));
                    }
                }
            }
        }
        None
    }

    fn food_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            let c = &s.grid[idx(nx, ny)];
            c.terrain == Terrain::Forest && c.food > 0.5
        })
    }

    fn water_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            s.grid[idx(nx, ny)].terrain.walkable() && s.is_water_adj(nx, ny)
        })
    }

    fn ore_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            let c = &s.grid[idx(nx, ny)];
            c.terrain == Terrain::Hills && c.ore > 0.5
        })
    }

    fn is_water_adj(&self, x: i32, y: i32) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if in_bounds(nx, ny) && self.grid[idx(nx, ny)].terrain == Terrain::Water {
                    return true;
                }
            }
        }
        false
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

    fn apply(&mut self, i: usize, act: Action, dead: &mut Vec<usize>) {
        match act {
            Action::Move(nx, ny) => {
                let mut ok = true;
                let (mut ox, mut oy) = (0, 0);
                {
                    let a = &self.agents[i];
                    if a.energy <= 0.0 {
                        ok = false;
                    } else {
                        ox = a.x;
                        oy = a.y;
                    }
                }
                if !ok {
                    return;
                }
                let wadj = self.is_water_adj(nx, ny);
                {
                    let a = &mut self.agents[i];
                    a.x = nx;
                    a.y = ny;
                    a.dir_x = (nx - ox).clamp(-1, 1);
                    a.dir_y = (ny - oy).clamp(-1, 1);
                    a.energy -= 0.6;
                    if a.carry.is_none() {
                        match a.want {
                            ResourceKind::Food => {
                                let c = &self.grid[idx(nx, ny)];
                                if c.terrain == Terrain::Forest && c.food > 0.5 {
                                    self.grid[idx(nx, ny)].food -= 1.0;
                                    a.carry = Some((ResourceKind::Food, 2.0));
                                }
                            }
                            ResourceKind::Water => {
                                if wadj && self.grid[idx(nx, ny)].terrain.walkable() {
                                    a.carry = Some((ResourceKind::Water, 2.0));
                                }
                            }
                            ResourceKind::Ore => {
                                let c = &self.grid[idx(nx, ny)];
                                if c.terrain == Terrain::Hills && c.ore > 0.5 {
                                    self.grid[idx(nx, ny)].ore -= 1.0;
                                    a.carry = Some((ResourceKind::Ore, 1.0));
                                }
                            }
                        }
                    }
                }
                if self.agents[i].raider {
                    self.combat_check(i);
                }
            }
            Action::Stay => {
                let a = &mut self.agents[i];
                a.energy = (a.energy + 6.0).min(100.0);
            }
            Action::Eat => {
                let ti = self.agents[i].home;
                if self.towns[ti].stocks.food > 0.0 {
                    self.towns[ti].stocks.food -= 2.0;
                    let a = &mut self.agents[i];
                    a.hunger = (a.hunger - 30.0).max(0.0);
                    a.energy = (a.energy + 4.0).min(100.0);
                }
            }
            Action::Drink => {
                let ti = self.agents[i].home;
                if self.towns[ti].stocks.water > 0.0 {
                    self.towns[ti].stocks.water -= 2.0;
                    let a = &mut self.agents[i];
                    a.thirst = (a.thirst - 30.0).max(0.0);
                    a.energy = (a.energy + 2.0).min(100.0);
                }
            }
            Action::Deposit => {
                let ti = self.agents[i].home;
                let a = &mut self.agents[i];
                if let Some((kind, qty)) = a.carry.take() {
                    let st = &mut self.towns[ti].stocks;
                    match kind {
                        ResourceKind::Food => st.food += qty,
                        ResourceKind::Water => st.water += qty,
                        ResourceKind::Ore => st.ore += qty,
                    }
                }
                a.hunger = (a.hunger - 5.0).max(0.0);
            }
            Action::Die => dead.push(i),
        }
    }

    fn reproduction(&mut self) {
        if self.agents.len() >= MAX_AGENTS {
            return;
        }
        let mut used_town = vec![false; self.towns.len()];
        for fid in 0..self.families.len() {
            let fam_town = self.families[fid].town;
            if self.families[fid].extinct || self.families[fid].members < 2 {
                continue;
            }
            if used_town[fam_town] {
                continue;
            }
            if self.pop(fam_town) >= self.towns[fam_town].cap {
                continue;
            }
            let st = &self.towns[fam_town].stocks;
            if st.food < BIRTH_MIN_FOOD || st.water < BIRTH_MIN_WATER {
                continue;
            }
            let (tx, ty) = (self.towns[fam_town].x, self.towns[fam_town].y);
            let (cf, cw) = if self.towns[fam_town].idea == TownIdea::Prosperity {
                (BIRTH_FOOD * 0.5, BIRTH_WATER * 0.5)
            } else {
                (BIRTH_FOOD, BIRTH_WATER)
            };
            self.towns[fam_town].stocks.food -= cf;
            self.towns[fam_town].stocks.water -= cw;
            self.spawn_agent(fam_town, tx, ty, fid, false);
            self.families[fid].children += 1;
            used_town[fam_town] = true;
        }
    }

    fn neighbors(&self, i: usize, j: usize) -> bool {
        let (x1, y1) = (self.towns[i].x, self.towns[i].y);
        let (x2, y2) = (self.towns[j].x, self.towns[j].y);
        let d = ((x1 - x2).pow(2) as f32 + (y1 - y2).pow(2) as f32).sqrt();
        d <= WAR_START_TOWN_RANGE
    }

    fn war_step(&mut self) {
        for i in 0..self.towns.len() {
            if self.towns[i].at_war {
                if self.towns[i].raiders <= 0 {
                    if self.pop(i) >= ARMY_TARGETS_POP {
                        let count = ((self.pop(i) as f32 * 0.3).min(6.0)) as u32;
                        self.muster_army(i, count);
                    } else if self.towns[i].stocks.food < 12.0 && self.towns[i].stocks.water < 8.0 {
                        self.end_war(i);
                    } else if self.towns[i].idea != TownIdea::War
                        && self.raiders_ok(&self.towns[i])
                        && rfrac(&mut self.rng) < PEACE_CHANCE_PER_TICK
                    {
                        self.end_war(i);
                    }
                }
                continue;
            }
            let chance = RAID_CHANCE_PER_TICK
                * if self.towns[i].idea == TownIdea::War { 3.0 } else { 1.0 };
            if rfrac(&mut self.rng) < chance {
                self.try_raid(i);
            }
        }
    }

    fn try_raid(&mut self, ti: usize) {
        if self.pop(ti) < WAR_START_POP {
            return;
        }
        {
            let me = &self.towns[ti];
            if me.stocks.food < WAR_START_FOOD || me.stocks.water < WAR_START_WATER {
                return;
            }
        }
        for j in 0..self.towns.len() {
            if j == ti || self.towns[j].at_war || !self.neighbors(ti, j) || self.pop(j) < RAID_TARGET_POP {
                continue;
            }
            let mut r = 1.5f32;
            for (k, _) in self.towns.iter().enumerate() {
                if k != ti && k != j && self.neighbors(j, k) {
                    r = (r * 1.4).min(8.0);
                }
            }
            if rfrac(&mut self.rng) < 1.0 / r {
                let raiders = ((self.towns[ti].stocks.food / RAISE_FOOD).min(4.0) as u32).max(1);
                self.declare_war(ti, j, raiders);
                if rfrac(&mut self.rng) < 0.6 {
                    self.declare_war(j, ti, 1);
                }
                return;
            }
        }
    }

    fn declare_war(&mut self, ti: usize, enemy: usize, raiders: u32) {
        self.towns[ti].at_war = true;
        self.towns[ti].enemy = Some(enemy);
        self.muster_army(ti, raiders);
    }

    fn muster_army(&mut self, ti: usize, count: u32) {
        let enemy = self.towns[ti].enemy;
        let mut left = count;
        for i in 0..self.agents.len() {
            if left == 0 {
                break;
            }
            if self.agents[i].home == ti && !self.agents[i].raider {
                self.agents[i].raider = true;
                self.agents[i].target_town = enemy;
                left -= 1;
            }
        }
        self.towns[ti].raiders = count;
    }

    fn release_dead_raiders(&mut self, dead: &[usize]) {
        let mut homes = Vec::new();
        for &i in dead {
            if i < self.agents.len() && self.agents[i].raider {
                homes.push(self.agents[i].home);
            }
        }
        for ti in homes {
            if ti < self.towns.len() {
                self.towns[ti].raiders = self.towns[ti].raiders.saturating_sub(1);
            }
        }
    }

    fn raiders_ok(&self, t: &Settlement) -> bool {
        t.stocks.food >= PEACE_FOOD_WATER_MIN && t.stocks.water >= PEACE_FOOD_WATER_MIN
    }

    fn end_war(&mut self, ti: usize) {
        self.towns[ti].at_war = false;
        self.towns[ti].raiders = 0;
        self.towns[ti].enemy = None;
        for a in self.agents.iter_mut() {
            if a.home == ti {
                a.raider = false;
                a.target_town = None;
            }
        }
    }

    fn army_action(&self, a: &Agent) -> (Action, ResourceKind) {
        let t = &self.towns[a.home];
        let (tx, ty) = (t.x, t.y);
        let at_home = (a.x - tx).abs() <= 1 && (a.y - ty).abs() <= 1;
        let dn = a.target_town;
        if a.carry.is_some() {
            if at_home {
                (Action::Deposit, ResourceKind::Food)
            } else {
                let (nx, ny) = self.steer(a, tx, ty);
                (Action::Move(nx, ny), ResourceKind::Food)
            }
        } else if let Some(j) = dn {
            if j >= self.towns.len() {
                (Action::Stay, ResourceKind::Food)
            } else {
                let (ex, ey) = (self.towns[j].x, self.towns[j].y);
                let (nx, ny) = self.steer(a, ex, ey);
                (Action::Move(nx, ny), ResourceKind::Food)
            }
        } else if at_home {
            if self.towns[a.home].stocks.food > 0.0 {
                (Action::Stay, ResourceKind::Food)
            } else {
                self.gather_action(a, Some(ResourceKind::Food))
            }
        } else {
            let (nx, ny) = self.steer(a, tx, ty);
            (Action::Move(nx, ny), ResourceKind::Food)
        }
    }

    fn combat_check(&mut self, i: usize) {
        let (my_x, my_y) = {
            let a = &self.agents[i];
            (a.x, a.y)
        };
        let Some(j) = self.agents[i].target_town else {
            return;
        };
        if j >= self.towns.len() || !self.towns[j].at_war {
            return;
        }
        let (ex, ey) = (self.towns[j].x, self.towns[j].y);
        let d = (my_x - ex).abs().max(my_y - ey);
        if d <= 4 {
            let t = &mut self.towns[j];
            let take_f = t.stocks.food.min(3.0);
            t.stocks.food -= take_f;
            let take_w = t.stocks.water.min(2.0);
            t.stocks.water -= take_w;
        }
        for k in 0..self.agents.len() {
            if k == i {
                continue;
            }
            let foe = {
                let b = &self.agents[k];
                (b.home, b.raider, b.x, b.y)
            };
            if foe.1 || foe.0 == self.agents[i].home {
                continue;
            }
            if (foe.2 - my_x).abs() <= 1 && (foe.3 - my_y).abs() <= 1 {
                if rfrac(&mut self.rng) < 0.3 {
                    self.agents[k].hunger = STARVE;
                }
                return;
            }
        }
    }

    fn sync_families(&mut self) {
        for fam in self.families.iter_mut() {
            let members = self
                .agents
                .iter()
                .filter(|a| a.family == fam.id)
                .count() as u32;
            fam.members = members;
            fam.extinct = members == 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn total_food(s: &Sim) -> f32 {
        s.towns.iter().map(|t| t.stocks.food).sum()
    }

    fn stock_bits(s: &Sim) -> Vec<(u32, u32, u32)> {
        s.towns
            .iter()
            .map(|t| (t.stocks.food.to_bits(), t.stocks.water.to_bits(), t.stocks.ore.to_bits()))
            .collect()
    }

    #[test]
    fn same_seed_same_world() {
        let a = Sim::new(42);
        let b = Sim::new(42);
        assert_eq!(a.grid.len(), b.grid.len());
        for i in 0..a.grid.len() {
            assert_eq!(a.grid[i].terrain, b.grid[i].terrain);
            assert_eq!(a.grid[i].food.to_bits(), b.grid[i].food.to_bits());
            assert_eq!(a.grid[i].ore.to_bits(), b.grid[i].ore.to_bits());
        }
        assert_eq!(a.agents.len(), b.agents.len());
        for i in 0..a.agents.len() {
            assert_eq!(a.agents[i].x, b.agents[i].x);
            assert_eq!(a.agents[i].y, b.agents[i].y);
            assert_eq!(a.agents[i].home, b.agents[i].home);
            assert_eq!(a.agents[i].thirst.to_bits(), b.agents[i].thirst.to_bits());
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
            assert_eq!(i.thirst.to_bits(), j.thirst.to_bits());
            assert_eq!(i.energy.to_bits(), j.energy.to_bits());
            assert_eq!(i.want, j.want);
            assert_eq!(
                i.carry.map(|(k, q)| (k, q.to_bits())),
                j.carry.map(|(k, q)| (k, q.to_bits()))
            );
        }
        assert_eq!(stock_bits(&a), stock_bits(&b));
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
        s.towns[0].stocks.food = 10.0;
        s.towns[0].stocks.water = 150.0;
        s.towns[0].stocks.ore = 100.0;
        let s0 = s.towns[0].stocks.food;
        s.bless(s.towns[0].x, s.towns[0].y, 12);
        let mut done = false;
        for _ in 0..3000 {
            s.tick();
            if s.towns[0].stocks.food > s0 + 10.0 {
                done = true;
                break;
            }
        }
        assert!(done, "food stock should grow from gathering");
    }

    #[test]
    fn ore_mining_increases_ore_stock() {
        let mut s = Sim::new(12);
        let (x, y) = (s.towns[0].x, s.towns[0].y);
        for dy in -5..=5 {
            for dx in -5..=5 {
                let nx = x + dx;
                let ny = y + dy;
                if in_bounds(nx, ny) {
                    let c = &mut s.grid[idx(nx, ny)];
                    c.terrain = Terrain::Hills;
                    c.ore = ORE_MAX;
                }
            }
        }
        for _ in 0..3000 {
            s.tick();
            if s.towns[0].stocks.ore > 5.0 {
                return;
            }
        }
        panic!("ore stock should grow from mining");
    }

    #[test]
    fn water_collection_increases_water_stock() {
        let mut s = Sim::new(13);
        let (x, y) = (s.towns[0].x, s.towns[0].y);
        s.towns[0].stocks.food = 100.0;
        s.towns[0].stocks.water = 0.0;
        s.towns[0].stocks.ore = 100.0;
        for dy in 2..=5 {
            for dx in -5..=5 {
                let nx = x + dx;
                let ny = y + dy;
                if in_bounds(nx, ny) {
                    s.grid[idx(nx, ny)].terrain = Terrain::Water;
                }
            }
        }
        for _ in 0..3000 {
            s.tick();
            if s.towns[0].stocks.water > 5.0 {
                return;
            }
        }
        panic!("water stock should grow from shore collection");
    }

    #[test]
    fn births_require_food_and_water() {
        let mut s = Sim::new(14);
        s.agents.retain(|a| a.home != 0);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        for _ in 0..2 {
            s.spawn_agent(0, tx, ty, 0, false);
        }
        while s.tick_count % BIRTH_EVERY != BIRTH_EVERY - 1 {
            s.tick();
        }
        s.towns[0].stocks.food = 60.0;
        s.towns[0].stocks.water = 40.0;
        s.towns[0].stocks.ore = 100.0;
        let before = s.pop(0);
        let (f0, w0) = (s.towns[0].stocks.food, s.towns[0].stocks.water);
        s.tick();
        assert_eq!(s.pop(0), before + 1, "population should grow with food+water");
        assert!(s.towns[0].stocks.food < f0, "births consume food");
        assert!(s.towns[0].stocks.water < w0, "births consume water");
    }

    #[test]
    fn construction_completes_house_and_raises_cap() {
        let mut s = Sim::new(15);
        s.towns[0].stocks.food = 100.0;
        s.towns[0].stocks.water = 100.0;
        s.towns[0].stocks.ore = 400.0;
        s.build_request(0, BuildingKind::House);
        for _ in 0..HOUSE_COST as usize + 5 {
            s.tick();
        }
        assert_eq!(s.towns[0].built, vec![BuildingKind::House]);
        assert_eq!(s.towns[0].cap, 12 + HOUSE_CAP_BONUS);
    }

    #[test]
    fn construction_stops_without_ore() {
        let mut s = Sim::new(16);
        s.agents.clear();
        s.towns[0].stocks.food = 100.0;
        s.towns[0].stocks.water = 100.0;
        s.towns[0].stocks.ore = 5.0;
        s.build_request(0, BuildingKind::Well);
        for _ in 0..200 {
            s.tick();
        }
        assert!(s.towns[0].built.is_empty(), "cannot build without ore");
    }

    #[test]
    fn well_regenerates_water() {
        let mut s = Sim::new(17);
        s.towns[0].stocks.food = 100.0;
        s.towns[0].stocks.water = 100.0;
        s.towns[0].stocks.ore = 400.0;
        s.build_request(0, BuildingKind::Well);
        let w0 = s.towns[0].stocks.water;
        for _ in 0..WELL_COST as usize + 50 {
            s.tick();
        }
        assert!(
            s.towns[0].stocks.water > w0,
            "well should regenerate water over time"
        );
    }

    #[test]
    fn terrain_has_forests_and_water() {
        for seed in 1..=6u64 {
            let s = Sim::new(seed);
            let mut forest = 0;
            let mut water = 0;
            let mut hills = 0;
            for c in s.grid.iter() {
                match c.terrain {
                    Terrain::Forest => forest += 1,
                    Terrain::Water => water += 1,
                    Terrain::Hills => hills += 1,
                    Terrain::Grass => {}
                }
            }
            let n = (W * H) as f64;
            let (wf, ff, hf) = (water as f64 / n, forest as f64 / n, hills as f64 / n);
            assert!(wf >= 0.045, "seed {}: too little water {:.1}%", seed, wf * 100.0);
            assert!(ff >= 0.18, "seed {}: too little forest {:.1}%", seed, ff * 100.0);
            assert!(hf >= 0.06, "seed {}: too little hills {:.1}%", seed, hf * 100.0);
        }
    }

    #[test]
    fn families_cover_agents_and_have_founders() {
        let s = Sim::new(21);
        assert_eq!(s.families.len(), s.towns.len() * 2);
        assert!(s.agents.iter().all(|a| a.family < s.families.len()));
        let fam_members: u32 = s.families.iter().map(|f| f.members).sum();
        assert_eq!(fam_members as usize, s.agents.len());
        assert!(s.agents.iter().any(|a| a.founder));
    }

    #[test]
    fn reproduction_grows_families_with_children() {
        let mut s = Sim::new(22);
        for t in s.towns.iter_mut() {
            t.cap = 200;
        }
        let mut children = 0u32;
        for _ in 0..12000 {
            s.tick();
            children = s.families.iter().map(|f| f.children).sum();
            if children > 0 {
                break;
            }
        }
        assert!(children > 0, "families should produce children");
        assert!(s.agents.iter().all(|a| a.family < s.families.len()));
    }

    #[test]
    fn single_member_family_cannot_reproduce() {
        let mut s = Sim::new(23);
        s.towns[0].cap = 200;
        s.agents.retain(|a| a.home != 0);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.spawn_agent(0, tx, ty, 0, false);
        s.towns[0].stocks.food = 500.0;
        s.towns[0].stocks.water = 500.0;
        s.towns[0].stocks.ore = 100.0;
        for _ in 0..BIRTH_EVERY * 2 + 5 {
            s.tick();
        }
        assert_eq!(s.families[0].children, 0, "single member cannot have children");
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

    fn prep_two_towns(s: &mut Sim) {
        let keep = |home: usize| home < 2;
        s.agents.retain(|a| keep(a.home));
        for t in 0..2 {
            s.towns[t].stocks.food = 200.0;
            s.towns[t].stocks.water = 150.0;
            s.towns[t].stocks.ore = 100.0;
            while s.pop(t) < 9 {
                let (tx, ty) = (s.towns[t].x, s.towns[t].y);
                s.spawn_agent(t, tx, ty, 0, false);
            }
        }
    }

    #[test]
    fn war_declares_and_musters_raiders() {
        let mut s = Sim::new(33);
        prep_two_towns(&mut s);
        let mut found = false;
        for _ in 0..40 {
            s.try_raid(0);
            if s.towns[1].at_war {
                found = true;
                break;
            }
        }
        assert!(found, "neighbor town should eventually be raided");
        assert_eq!(s.towns[1].enemy, Some(0));
        assert!(s.agents.iter().any(|a| a.home == 1 && a.raider));
    }

    #[test]
    fn combat_kills_defenders_and_loots() {
        let mut s = Sim::new(34);
        prep_two_towns(&mut s);
        let (ex, ey) = (s.towns[1].x, s.towns[1].y);
        s.agents.retain(|a| !(a.home == 0));
        let ridx = s.agents.len();
        s.spawn_agent(0, ex + 1, ey, 0, false);
        let r = &mut s.agents[ridx];
        r.raider = true;
        r.target_town = Some(1);
        s.towns[1].at_war = true;
        s.towns[1].enemy = Some(0);
        let food_before = s.towns[1].stocks.food;
        let mut killed = false;
        for _ in 0..60 {
            s.combat_check(ridx);
            if s.agents.iter().any(|a| a.home == 1 && a.hunger >= STARVE) {
                killed = true;
                break;
            }
        }
        assert!(killed, "raider should kill nearby defenders");
        assert!(s.towns[1].stocks.food < food_before, "raider should loot food");
    }

    #[test]
    fn end_war_demobilizes_force() {
        let mut s = Sim::new(35);
        prep_two_towns(&mut s);
        s.declare_war(1, 0, 3);
        assert!(s.towns[1].at_war);
        assert!(s.agents.iter().any(|a| a.home == 1 && a.raider));
        s.end_war(1);
        assert!(!s.towns[1].at_war);
        assert_eq!(s.towns[1].enemy, None);
        assert_eq!(s.towns[1].raiders, 0);
        assert!(s.agents.iter().all(|a| a.home != 1 || (!a.raider && a.target_town.is_none())));
    }

    #[test]
    fn manual_weather_cycles() {
        let mut s = Sim::new(36);
        assert_eq!(s.weather, Weather::Clear);
        s.cycle_weather();
        assert_eq!(s.weather, Weather::Rain);
        assert!(s.weather_left > 0.0);
        s.cycle_weather();
        assert_eq!(s.weather, Weather::Heat);
        s.cycle_weather();
        assert_eq!(s.weather, Weather::Frost);
        s.cycle_weather();
        assert_eq!(s.weather, Weather::Clear);
    }

    #[test]
    fn weather_heat_dries_town_wells() {
        let mut s = Sim::new(37);
        prep_two_towns(&mut s);
        s.towns[0].stocks.water = 50.0;
        s.weather = Weather::Heat;
        s.weather_left = 100.0;
        s.tick();
        assert!(s.towns[0].stocks.water < 50.0, "heat should evaporate town water");
        assert_eq!(s.weather, Weather::Heat, "weather persists until timer expires");
    }

    #[test]
    fn ideas_cycle_and_expire() {
        let mut s = Sim::new(38);
        prep_two_towns(&mut s);
        assert_eq!(s.towns[0].idea, TownIdea::None);
        s.inspire(0);
        assert_eq!(s.towns[0].idea, TownIdea::War);
        assert_eq!(s.towns[0].idea_left, IDEA_TIME);
        s.inspire(0);
        assert_eq!(s.towns[0].idea, TownIdea::Prosperity);
        s.inspire(0);
        assert_eq!(s.towns[0].idea, TownIdea::Toil);
        s.inspire(0);
        assert_eq!(s.towns[0].idea, TownIdea::None);
    }

    #[test]
    fn prosperity_halves_birth_cost() {
        let mut s = Sim::new(39);
        prep_two_towns(&mut s);
        let (tx0, ty0) = (s.towns[0].x, s.towns[0].y);
        let (tx1, ty1) = (s.towns[1].x, s.towns[1].y);
        let mut fid = 0usize;
        for _ in 0..2 {
            s.spawn_agent(0, tx0, ty0, fid, false);
            s.spawn_agent(1, tx1, ty1, fid + 1, false);
            fid += 2;
            s.families[fid - 2].members = 2;
            s.families[fid - 1].members = 2;
        }
        s.towns[0].idea = TownIdea::None;
        s.towns[1].idea = TownIdea::Prosperity;
        s.towns[0].cap = 60;
        s.towns[1].cap = 60;
        s.towns[0].stocks = Stock { food: 100.0, water: 100.0, ore: 100.0 };
        s.towns[1].stocks = Stock { food: 100.0, water: 100.0, ore: 100.0 };
        s.reproduction();
        let cost_plain = 100.0 - s.towns[0].stocks.food;
        let cost_blessed = 100.0 - s.towns[1].stocks.food;
        assert!(cost_plain > 0.0 && cost_blessed > 0.0 && cost_blessed < cost_plain,
            "prosperity should cheapen birth (blessed {} < plain {})", cost_blessed, cost_plain);
    }

    #[test]
    fn every_agent_inherits_family_role() {
        let s = Sim::new(40);
        assert!(!s.families.is_empty());
        for a in s.agents.iter() {
            assert_eq!(a.role, s.families[a.family].role, "agent role must match its family");
        }
        let roles: std::collections::HashSet<Role> = s.families.iter().map(|f| f.role).collect();
        assert!(roles.len() >= 2, "founding families should cover multiple roles");
    }

    #[test]
    fn farmer_prefers_food_and_miner_ore() {
        let mut s = Sim::new(41);
        prep_two_towns(&mut s);
        let fi = s.agents.iter().position(|a| a.role == Role::Farmer).unwrap();
        let mi = s.agents.iter().position(|a| a.role == Role::Miner).unwrap();
        let (_, fw) = s.decide(&s.agents[fi]);
        let (_, mw) = s.decide(&s.agents[mi]);
        assert_eq!(fw, ResourceKind::Food, "farmer should want food");
        assert_eq!(mw, ResourceKind::Ore, "miner should want ore");
    }

    #[test]
    fn role_falls_back_when_source_empty() {
        let mut s = Sim::new(42);
        prep_two_towns(&mut s);
        let mi = s.agents.iter().position(|a| a.role == Role::Miner).unwrap();
        for c in s.grid.iter_mut() {
            if c.terrain == Terrain::Hills {
                c.ore = 0.0;
            }
        }
        for _ in 0..20 {
            let (_, w) = s.decide(&s.agents[mi]);
            if w != ResourceKind::Ore {
                return;
            }
        }
        panic!("miner with no ore should eventually fall back to another need");
    }
}