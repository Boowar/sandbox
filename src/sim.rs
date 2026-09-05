pub const W: usize = 150;
pub const H: usize = 96;
pub const TICK_DT: f64 = 0.08;

const FOOD_MAX: f32 = 10.0;
const ORE_MAX: f32 = 60.0;
const WATER_MAX: f32 = 240.0;
const WATER_SUCK: f32 = 5.0;
const WATER_REGEN_CLEAR: f32 = 12.0;
const WATER_REGEN_RAIN: f32 = 70.0;
const WATER_REGEN_HEAT: f32 = 4.0;
const WATER_REGEN_FROST: f32 = 6.0;
const RAIN_LAKE_CHANCE: u32 = 700;
const METEOR_EVERY: u64 = 2400;
const METEOR_CHANCE_P: u32 = 4000;
const METEOR_RADIUS: i32 = 3;
const FIRE_EVERY: u64 = 700;
const FIRE_CHANCE_P: u32 = 9000;
const FIRE_LEN: u32 = 40;
const FIRE_SPREAD_DIV: u64 = 8;
const HORDE_EVERY: u64 = 1500;
const HORDE_CHANCE_P: u32 = 8;
const HORDE_PACK_MAX: usize = 6;
const GOLD_VEIN_EVERY: u64 = 1500;
const GOLD_VEIN_CHANCE_P: u32 = 5;
const GOLD_VEIN_AMOUNT: f32 = 320.0;
const GOLD_VEIN_PER_TICK: f32 = 0.04;
const GOLD_VEIN_RANGE: i32 = 24;
const MIGRATE_EVERY: u64 = 500;
const MIGRATE_CHANCE_P: u32 = 6;
const MIGRATE_QUALITY_MARGIN: f32 = 5.0;
pub const DAY_LEN: u64 = 1200;
const SEASON_LEN: u64 = 6000;
const NIGHT_WORK_MULT: f32 = 0.6;
const NIGHT_DANGER: f32 = 2.0;
const MARRIAGE_EVERY: u64 = 900;
const MARRIAGE_CHANCE_P: u32 = 5;
const MARRIAGE_LENGTH: u64 = 3000;
const GIFT_EVERY: u64 = 300;
const GIFT_MIN_FOOD: f32 = 80.0;
const GIFT_MIN_WATER: f32 = 60.0;
const TREATY_EVERY: u64 = 1200;
const TREATY_CHANCE_P: u32 = 7;
const TREATY_LENGTH: u64 = 6000;
const SEEK_RADIUS: i32 = 20;
const HOME_BOUND: f32 = 14.0;
const HUNGRY_AT: f32 = 60.0;
const THIRSTY_AT: f32 = 60.0;
const STARVE: f32 = 100.0;
const BIRTH_EVERY: u64 = 220;
const REGROW_EVERY: u64 = 44;
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

const ANIMAL_MAX: usize = 120;
const ANIMAL_BREED_EVERY: u64 = 800;
const DOMESTIC_MILK_COST_FOOD: f32 = 15.0;
const DOMESTIC_MILK_COST_WATER: f32 = 8.0;
const DOMESTIC_HERD_CAP: usize = 6;
const WOLF_MIN_TOWN_DIST: f32 = 18.0;
const WOLF_BITE_CHANCE: f32 = 0.08;
const BOAR_TUSK_CHANCE: f32 = 0.05;
const WOLF_TARGET_RADIUS: i32 = 12;
const ANIMAL_MELEE_REACH: i32 = 2;

const GOLD_MAX: f32 = 200.0;
const TRADE_POST_COST: f32 = 40.0;
const TRADE_TRICKLE: f32 = 0.03;
const CARAVAN_EVERY: u64 = 900;
const CARAVAN_CAPACITY: f32 = 10.0;
const CARAVAN_MAX: usize = 16;
const EXPORT_FOOD: f32 = 50.0;
const EXPORT_WATER: f32 = 60.0;
const EXPORT_ORE: f32 = 25.0;
const EXPORT_MEAT: f32 = 15.0;
const BUY_FOOD_AT: f32 = 12.0;
const BUY_WATER_AT: f32 = 8.0;
const BUY_ORE_AT: f32 = 5.0;
const BUY_MEAT_AT: f32 = 6.0;
const EXPORT_FISH: f32 = 12.0;
const BUY_FISH_AT: f32 = 5.0;

const FARM_COST: f32 = 30.0;
const FARM_PATCH: usize = 16;
const FARM_FOOD_MAX: f32 = 8.0;

const SANCTUARY_COST: f32 = 35.0;
const FAITH_GAIN_PER_TICK: f32 = 0.02;
const RITUAL_EVERY: u64 = 600;
const FAITH_SPEND: f32 = 40.0;
const BLESS_LEN: f64 = 900.0;

const CLINIC_COST: f32 = 40.0;
const SICK_MAX: u32 = 200;
const CONTAGION_CHANCE: f32 = 0.03;
const CONTAGION_RADIUS: i32 = 4;
const PLAGUE_CHANCE: f32 = 0.00025;
const PLAGUE_LEN: u64 = 1500;
const HEAL_RADIUS: i32 = 3;
const HEAL_PER_TICK: u32 = 2;

const WALL_COST: f32 = 50.0;
const BARRACKS_COST: f32 = 45.0;
const DEFENSE_BASE: f32 = 0.3;
const DEFENSE_WALL_BONUS: f32 = 0.15;
const DEFENSE_BARRACKS_BONUS: f32 = 0.2;

const TECH_EVERY: u64 = 350;
const TECH_TIER1: f32 = 500.0;
const TECH_TIER2: f32 = 1800.0;
const TECH_TIER3: f32 = 4500.0;
const SCIENCE_REQ_POP: usize = 6;
const DEV_BASE: f32 = 2.0;
const DEV_UNI_BONUS: f32 = 4.0;
const DEV_LIB_BONUS: f32 = 1.5;
const DEV_SCHOLAR_BONUS: f32 = 1.0;
const UNIVERSITY_COST: f32 = 70.0;
const SMITHY_COST: f32 = 85.0;
const LIBRARY_COST: f32 = 110.0;
const TEMPLE_COST: f32 = 150.0;

pub const CHILD_AGE: u32 = 90;
pub const OLD_AGE: u32 = 24000;

const EMPIRE_EVERY: u64 = 400;
const EMPIRE_EPOCH_P: u32 = 19;

const TOWNS_EVERY: u64 = 800;
const FOUND_MIN_POP: usize = 14;
const FOUND_MIN_FOOD: f32 = 220.0;
const FOUND_MIN_WATER: f32 = 140.0;
const FOUND_EPOCH_P: u32 = 5;
const FOUND_RADIUS_MIN: i32 = 12;
const FOUND_RADIUS_MAX: i32 = 40;
const FOUND_COLONY_POP: usize = 4;
const MAX_TOWNS: usize = 10;
const TOWN_WASTE_NEED: u64 = 3;

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Heat,
    Frost,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum TownIdea {
    None,
    War,
    Prosperity,
    Toil,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum Role {
    Worker,
    Farmer,
    Miner,
    Hunter,
    Priest,
    Healer,
    Guard,
    Scholar,
    Builder,
    Prophet,
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Blessing {
    None,
    Fertility,
    Abundance,
    Protection,
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Prophecy {
    None,
    Harvest,
    Rain,
    PlagueWarning,
    HolyWar,
    Prosperity,
}

impl Prophecy {
    pub fn name(self) -> &'static str {
        match self {
            Prophecy::None => "",
            Prophecy::Harvest => "урожай",
            Prophecy::Rain => "дождь",
            Prophecy::PlagueWarning => "чума",
            Prophecy::HolyWar => "священная война",
            Prophecy::Prosperity => "процветание",
        }
    }
}
const PEACE_CHANCE_PER_TICK: f32 = 0.02;
const PEACE_FOOD_WATER_MIN: f32 = 70.0;
const WELL_WATER_PER_TICK: f32 = 1.0;

const REVELATION_PER_TICK: f32 = 0.03;
const PROPHECY_COST: f32 = 50.0;
const PROPHECY_LEN: f32 = 1200.0;

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Terrain {
    Grass,
    Forest,
    Hills,
    Water,
    Farm,
    Desert,
    Tundra,
    Jungle,
}

impl Terrain {
    fn walkable(self) -> bool {
        !matches!(self, Terrain::Water)
    }
}

fn is_food_source(c: &Cell) -> bool {
    c.terrain == Terrain::Forest || c.terrain == Terrain::Farm || c.terrain == Terrain::Jungle
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResourceKind {
    Food,
    Water,
    Ore,
    Meat,
    Gold,
    Fish,
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Species {
    Deer,
    Boar,
    Wolf,
    Cow,
}

impl Species {
    pub fn meat_yield(self) -> f32 {
        match self {
            Species::Deer => 2.5,
            Species::Boar => 3.5,
            Species::Wolf => 2.0,
            Species::Cow => 4.0,
        }
    }

    fn hp(self) -> f32 {
        match self {
            Species::Deer => 40.0,
            Species::Boar => 75.0,
            Species::Wolf => 55.0,
            Species::Cow => 50.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum BuildingKind {
    House,
    Well,
    TradePost,
    Farm,
    Sanctuary,
    Clinic,
    Wall,
    Barracks,
    University,
    Smithy,
    Library,
    Temple,
}

impl BuildingKind {
    pub fn cost(self) -> f32 {
        match self {
            BuildingKind::House => HOUSE_COST,
            BuildingKind::Well => WELL_COST,
            BuildingKind::TradePost => TRADE_POST_COST,
            BuildingKind::Farm => FARM_COST,
            BuildingKind::Sanctuary => SANCTUARY_COST,
            BuildingKind::Clinic => CLINIC_COST,
            BuildingKind::Wall => WALL_COST,
            BuildingKind::Barracks => BARRACKS_COST,
            BuildingKind::University => UNIVERSITY_COST,
            BuildingKind::Smithy => SMITHY_COST,
            BuildingKind::Library => LIBRARY_COST,
            BuildingKind::Temple => TEMPLE_COST,
        }
    }
}

pub fn trade_price(k: ResourceKind) -> f32 {
    match k {
        ResourceKind::Food => 0.8,
        ResourceKind::Water => 0.4,
        ResourceKind::Ore => 1.5,
        ResourceKind::Meat => 1.2,
        ResourceKind::Gold => 0.0,
        ResourceKind::Fish => 1.0,
    }
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Stock {
    pub food: f32,
    pub water: f32,
    pub ore: f32,
    pub meat: f32,
    pub gold: f32,
    pub fish: f32,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Cell {
    pub terrain: Terrain,
    pub food: f32,
    pub ore: f32,
    pub water: f32,
    pub burn: u32,
    pub gold: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Agent {
    pub home: usize,
    pub x: i32,
    pub y: i32,
    pub dir_x: i32,
    pub dir_y: i32,
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub mood: f32,
    pub want: ResourceKind,
    pub carry: Option<(ResourceKind, f32)>,
    pub family: usize,
    pub founder: bool,
    pub raider: bool,
    pub target_town: Option<usize>,
    pub role: Role,
    pub sick: u32,
    pub age: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
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
    pub faith: f32,
    pub blessing: Blessing,
    pub blessing_left: f64,
    pub prophecy: Prophecy,
    pub prophecy_left: f32,
    pub revelation: f32,
    pub plague_until: u64,
    pub empire: Option<usize>,
    pub alive: bool,
    pub waste: u64,
    pub dev: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Empire {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub name: String,
    pub members: Vec<usize>,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(i32, i32),
    Stay,
    Eat,
    Drink,
    Deposit,
    Die,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animal {
    pub x: i32,
    pub y: i32,
    pub species: Species,
    pub hp: f32,
    pub home: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Caravan {
    pub home: usize,
    pub target: usize,
    pub x: i32,
    pub y: i32,
    pub goods: Vec<(ResourceKind, f32)>,
    pub gift: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SocialLink {
    pub a: usize,
    pub b: usize,
    pub bond: f32,
}

const MOOD_NEAR_FRIEND: f32 = 0.008;
const MOOD_NEAR_ENEMY: f32 = -0.012;
const MOOD_FED: f32 = 0.005;
const MOOD_HUNGRY: f32 = -0.008;
const MOOD_PROSPER: f32 = 0.004;
const MOOD_LINK_DECAY: f32 = 0.001;
const MOOD_MIGRATE_THRESHOLD: f32 = -0.4;
const FRIEND_RANGE: i32 = 8;
const SOCIAL_LINK_CHANCE: u64 = 200;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sim {
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
    pub towns: Vec<Settlement>,
    pub families: Vec<Family>,
    pub empires: Vec<Empire>,
    pub animals: Vec<Animal>,
    pub caravans: Vec<Caravan>,
    pub roads: Vec<bool>,
    pub social_links: Vec<SocialLink>,
    pub migrations: u32,
    pub alliances: Vec<(usize, usize, u64)>,
    pub treaties: Vec<(usize, usize, u64)>,
    pub gifts_sent: u32,
    pub invades: u32,
    pub gold_veins: Vec<(i32, i32, f32)>,
    pub tick_count: u64,
    pub weather: Weather,
    pub weather_left: f64,
    pub season: Season,
    pub day_phase: u64,
    pub rng: u64,
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
            empires: Vec::new(),
            animals: Vec::new(),
            caravans: Vec::new(),
            roads: vec![false; W * H],
            social_links: Vec::new(),
            migrations: 0,
            alliances: Vec::new(),
            treaties: Vec::new(),
            gifts_sent: 0,
            invades: 0,
            gold_veins: Vec::new(),
            tick_count: 0,
            weather: Weather::Clear,
            weather_left: 0.0,
            season: Season::Spring,
            day_phase: 0,
            rng,
        };
        sim.ensure_hills();
        sim.spawn_world();
        sim.spawn_animals();
        sim
    }

    fn make_terrain(rng: &mut u64) -> Vec<Cell> {
        let mut grid = vec![
            Cell { terrain: Terrain::Grass, food: FOOD_MAX, ore: 0.0, water: 0.0, burn: 0, gold: 0.0 };
            W * H
        ];
        let mut biome_map = vec![0u8; W * H];
        for y in 0..H {
            for x in 0..W {
                let i = y * W + x;
                let xf = x as f64 / W as f64;
                let yf = y as f64 / H as f64;
                let n1 = (xf * 3.7 + 0.5).sin() * (yf * 2.9 + 1.3).cos();
                let n2 = (xf * 5.1 + 2.1).cos() * (yf * 4.3 + 0.7).sin();
                let temp = 0.5 + 0.3 * n1 + 0.2 * ((x as u32).wrapping_mul(0x9E37) as f64 / u32::MAX as f64);
                let moist = 0.5 + 0.25 * n2 + 0.25 * ((y as u32).wrapping_mul(0x85EB) as f64 / u32::MAX as f64);
                biome_map[i] = if temp < 0.25 {
                    1
                } else if moist > 0.72 && temp > 0.55 {
                    3
                } else if moist < 0.22 && temp > 0.6 {
                    2
                } else {
                    0
                };
            }
        }
        for cell in grid.iter_mut() {
            cell.food = FOOD_MAX;
        }
        for i in 0..W * H {
            let biome = biome_map[i];
            let p = rfrac(rng);
            grid[i].terrain = match biome {
                2 => {
                    if p < 0.05 { Terrain::Hills } else { Terrain::Desert }
                }
                1 => {
                    if p < 0.15 { Terrain::Hills } else { Terrain::Tundra }
                }
                3 => {
                    if p < 0.40 { Terrain::Jungle } else if p < 0.55 { Terrain::Hills } else { Terrain::Grass }
                }
                _ => {
                    if p < 0.42 { Terrain::Forest }
                    else if p < 0.55 { Terrain::Hills }
                    else { Terrain::Grass }
                }
            };
            grid[i].food = match grid[i].terrain {
                Terrain::Forest => FOOD_MAX,
                Terrain::Jungle => FOOD_MAX * 1.5,
                Terrain::Tundra => FOOD_MAX * 0.4,
                Terrain::Desert => FOOD_MAX * 0.1,
                _ => FOOD_MAX,
            };
            grid[i].ore = if grid[i].terrain == Terrain::Hills { ORE_MAX } else { 0.0 };
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
                    let mut jungle = 0;
                    let mut desert = 0;
                    let mut tundra = 0;
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
                                Terrain::Jungle => jungle += 1,
                                Terrain::Desert => desert += 1,
                                Terrain::Tundra => tundra += 1,
                                Terrain::Farm | Terrain::Grass => {}
                            }
                        }
                    }
                    let dominant = if water >= 4 { Terrain::Water }
                        else if jungle >= 4 && water < 3 { Terrain::Jungle }
                        else if desert >= 4 && water < 2 { Terrain::Desert }
                        else if tundra >= 3 && water < 3 { Terrain::Tundra }
                        else if tree >= 5 && water < 3 && hill < 4 { Terrain::Forest }
                        else if hill >= 4 && water < 3 && tree < 4 { Terrain::Hills }
                        else { Terrain::Grass };
                    next[i].terrain = match t {
                        Terrain::Water => {
                            if water >= 2 && tree < 3 { Terrain::Water } else { Terrain::Grass }
                        }
                        Terrain::Forest => {
                            if tree >= 3 && water < 3 { Terrain::Forest } else { dominant }
                        }
                        Terrain::Hills => {
                            if hill >= 2 && water < 4 { Terrain::Hills } else { dominant }
                        }
                        Terrain::Jungle => {
                            if jungle >= 3 && water < 4 { Terrain::Jungle } else { dominant }
                        }
                        Terrain::Desert => {
                            if desert >= 2 && water < 2 { Terrain::Desert } else { dominant }
                        }
                        Terrain::Tundra => {
                            if tundra >= 2 && water < 3 { Terrain::Tundra } else { dominant }
                        }
                        Terrain::Farm => Terrain::Grass,
                        Terrain::Grass => dominant,
                    };
                    if next[i].terrain == Terrain::Hills && next[i].ore <= 0.0 {
                        next[i].ore = ORE_MAX;
                    }
                    if next[i].terrain == Terrain::Jungle && next[i].food <= 0.0 {
                        next[i].food = FOOD_MAX * 1.5;
                    }
                    if next[i].terrain == Terrain::Tundra && next[i].food <= 0.0 {
                        next[i].food = FOOD_MAX * 0.4;
                    }
                    if next[i].terrain == Terrain::Desert && next[i].food <= 0.0 {
                        next[i].food = FOOD_MAX * 0.1;
                    }
                }
            }
            grid = next;
        }
        for c in grid.iter_mut() {
            if c.terrain == Terrain::Water {
                c.water = WATER_MAX;
            }
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
        const FAMILIES_PER_TOWN: usize = 4;
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
                stocks: Stock { food: 200.0, water: 200.0, ore: 60.0, meat: 30.0, gold: 0.0, fish: 0.0 },
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
                faith: 0.0,
                blessing: Blessing::None,
                blessing_left: 0.0,
                prophecy: Prophecy::None,
                prophecy_left: 0.0,
                revelation: 0.0,
                plague_until: 0,
                empire: None,
                alive: true,
                waste: 0,
                dev: 0.0,
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
                    role: match (base + k) % 4 {
                        1 => Role::Farmer,
                        2 => Role::Miner,
                        3 => Role::Hunter,
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
            for a in self.agents.iter_mut().filter(|a| a.home == i) {
                let h = (a.x as u32).wrapping_mul(0x45d9_f3b)
                    ^ (a.y as u32).wrapping_mul(0x119d_e1f3)
                    ^ 0x9e37_79b9u32.wrapping_mul(0xabcd_ef01);
                a.age = 2000 + (h % 12000);
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
                    mood: 0.0,
                    want: ResourceKind::Food,
                    carry: None,
                    family,
                    founder,
                    raider: false,
                    target_town: None,
                    role: self.families[family].role,
                    sick: 0,
                    age: 9000,
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
            mood: 0.0,
            want: ResourceKind::Food,
            carry: None,
            family,
            founder,
            raider: false,
            target_town: None,
            role: self.families[family].role,
            sick: 0,
            age: 9000,
        });
        self.families[family].members += 1;
    }

    fn random_walkable(&mut self, min_town: f32, within: i32) -> Option<(i32, i32)> {
        for _ in 0..120 {
            let x = rnd(&mut self.rng) as i32 % W as i32;
            let y = rnd(&mut self.rng) as i32 % H as i32;
            if !in_bounds(x, y) || !self.grid[idx(x, y)].terrain.walkable() {
                continue;
            }
            if within > 0 {
                let dx = x - W as i32 / 2;
                let dy = y - H as i32 / 2;
                if dx.abs().max(dy.abs()) > within {
                    continue;
                }
            }
            let d = self
                .towns
                .iter()
                .map(|t| ((x - t.x).pow(2) + (y - t.y).pow(2)) as f32)
                .fold(f32::MAX, f32::min)
                .sqrt();
            if d < min_town {
                continue;
            }
            return Some((x, y));
        }
        None
    }

    fn push_animal(&mut self, species: Species, x: i32, y: i32, home: Option<usize>) {
        if self.animals.len() >= ANIMAL_MAX {
            return;
        }
        self.animals.push(Animal {
            x,
            y,
            species,
            hp: species.hp() * (0.8 + rfrac(&mut self.rng) * 0.4),
            home,
        });
    }

    fn spawn_animals(&mut self) {
        for _ in 0..24 {
            if let Some((x, y)) = self.random_walkable(4.0, 0) {
                let sp = if rfrac(&mut self.rng) < 0.55 { Species::Deer } else { Species::Boar };
                self.push_animal(sp, x, y, None);
            }
        }
        for _ in 0..5 {
            if let Some((x, y)) = self.random_walkable(WOLF_MIN_TOWN_DIST, 0) {
                self.push_animal(Species::Wolf, x, y, None);
            }
        }
        for ti in 0..self.towns.len() {
            let (tx, ty) = (self.towns[ti].x, self.towns[ti].y);
            for k in 0..2 {
                let dir = if k == 0 { -2 } else { 2 };
                let x = (tx + dir).clamp(1, W as i32 - 2);
                let y = ty;
                if self.grid[idx(x, y)].terrain.walkable() {
                    self.push_animal(Species::Cow, x, y, Some(ti));
                }
            }
        }
    }

    fn is_domestic_cow(&self, an: &Animal) -> bool {
        an.species == Species::Cow && an.home.is_some()
    }

    fn domestic_herd(&self, ti: usize) -> usize {
        self.animals
            .iter()
            .filter(|a| self.is_domestic_cow(a) && a.home == Some(ti))
            .count()
    }

    pub fn breed_domestic(&mut self, ti: usize) -> bool {
        if ti >= self.towns.len() {
            return false;
        }
        if self.domestic_herd(ti) >= DOMESTIC_HERD_CAP {
            return false;
        }
        let (f, w, tx0, ty0) = {
            let t = &self.towns[ti];
            (t.stocks.food, t.stocks.water, t.x, t.y)
        };
        if f < 20.0 || w < 10.0 {
            return false;
        }
        self.towns[ti].stocks.food -= 20.0;
        self.towns[ti].stocks.water -= 10.0;
        let (tx, ty) = (
            tx0 + rnd(&mut self.rng) as i32 % 5 - 2,
            ty0 + rnd(&mut self.rng) as i32 % 5 - 2,
        );
        let x = tx.clamp(1, W as i32 - 2);
        let y = ty.clamp(1, H as i32 - 2);
        self.push_animal(Species::Cow, x, y, Some(ti));
        true
    }

    fn spawn_calf(&mut self, ti: usize) {
        let t = &self.towns[ti];
        let x = (t.x + rnd(&mut self.rng) as i32 % 7 - 3).clamp(1, W as i32 - 2);
        let y = (t.y + rnd(&mut self.rng) as i32 % 7 - 3).clamp(1, H as i32 - 2);
        self.push_animal(Species::Cow, x, y, Some(ti));
    }

    fn has_trade_post(&self, ti: usize) -> bool {
        self.towns[ti].built.iter().any(|b| *b == BuildingKind::TradePost)
    }

    fn market_buy(&mut self) {
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                continue;
            }
            if !self.has_trade_post(ti) {
                continue;
            }
            let (mut f, mut w, mut o, mut m, mut g, mut fi) = {
                let s = &self.towns[ti].stocks;
                (s.food, s.water, s.ore, s.meat, s.gold, s.fish)
            };
            if f < BUY_FOOD_AT && g >= trade_price(ResourceKind::Food) * 2.0 {
                f += 2.0;
                g -= trade_price(ResourceKind::Food) * 2.0;
            }
            if w < BUY_WATER_AT && g >= trade_price(ResourceKind::Water) * 2.0 {
                w += 2.0;
                g -= trade_price(ResourceKind::Water) * 2.0;
            }
            if o < BUY_ORE_AT && g >= trade_price(ResourceKind::Ore) {
                o += 1.0;
                g -= trade_price(ResourceKind::Ore);
            }
            if m < BUY_MEAT_AT && g >= trade_price(ResourceKind::Meat) {
                m += 1.0;
                g -= trade_price(ResourceKind::Meat);
            }
            if fi < BUY_FISH_AT && g >= trade_price(ResourceKind::Fish) {
                fi += 1.0;
                g -= trade_price(ResourceKind::Fish);
            }
            let s = &mut self.towns[ti].stocks;
            s.food = f;
            s.water = w;
            s.ore = o;
            s.meat = m;
            s.gold = g;
            s.fish = fi;
        }
    }

    fn export_caravans(&mut self) {
        if self.tick_count % CARAVAN_EVERY != 0 || self.caravans.len() >= CARAVAN_MAX {
            return;
        }
        for ti in 0..self.towns.len() {
            if self.caravans.len() >= CARAVAN_MAX {
                break;
            }
            if !self.towns[ti].alive {
                continue;
            }
            let (has_post, at_war, x, y) = {
                let t = &self.towns[ti];
                (
                    t.built.iter().any(|b| *b == BuildingKind::TradePost),
                    t.at_war,
                    t.x,
                    t.y,
                )
            };
            if !has_post || at_war {
                continue;
            }
            let (f, w, o, m, fi) = {
                let s = &self.towns[ti].stocks;
                (s.food, s.water, s.ore, s.meat, s.fish)
            };
            let mut goods: Vec<(ResourceKind, f32)> = Vec::new();
            let mut left = CARAVAN_CAPACITY;
            let mut take = |kind: ResourceKind, stock: f32, thr: f32| {
                if left > 0.0 {
                    let avail = stock - thr;
                    if avail > 0.0 {
                        let amt = avail.min(left);
                        goods.push((kind, amt));
                        left -= amt;
                    }
                }
            };
            take(ResourceKind::Food, f, EXPORT_FOOD);
            take(ResourceKind::Ore, o, EXPORT_ORE);
            take(ResourceKind::Meat, m, EXPORT_MEAT);
            take(ResourceKind::Fish, fi, EXPORT_FISH);
            take(ResourceKind::Water, w, EXPORT_WATER);
            if goods.is_empty() {
                continue;
            }
            let st = &mut self.towns[ti].stocks;
            for (k, q) in &goods {
                match k {
                    ResourceKind::Food => st.food -= q,
                    ResourceKind::Ore => st.ore -= q,
                    ResourceKind::Meat => st.meat -= q,
                    ResourceKind::Water => st.water -= q,
                    ResourceKind::Gold => {}
                    ResourceKind::Fish => st.fish -= q,
                }
            }
            let mut best: Option<usize> = None;
            let mut bd = i32::MAX;
            for tj in 0..self.towns.len() {
                if tj == ti || !self.towns[tj].alive {
                    continue;
                }
                let (tx2, ty2) = (self.towns[tj].x, self.towns[tj].y);
                let d = self.cheb(x, y, tx2, ty2);
                let needy = goods.iter().any(|(k, _)| match k {
                    ResourceKind::Food => self.towns[tj].stocks.food < BUY_FOOD_AT,
                    ResourceKind::Water => self.towns[tj].stocks.water < BUY_WATER_AT,
                    ResourceKind::Ore => self.towns[tj].stocks.ore < BUY_ORE_AT,
                    ResourceKind::Meat => self.towns[tj].stocks.meat < BUY_MEAT_AT,
                    ResourceKind::Gold => false,
                    ResourceKind::Fish => self.towns[tj].stocks.fish < BUY_FISH_AT,
                });
                if needy && d < bd {
                    bd = d;
                    best = Some(tj);
                }
            }
            let target = match best {
                Some(tj) => tj,
                None => {
                    let mut bb: Option<usize> = None;
                    let mut bd2 = i32::MAX;
                    for tj in 0..self.towns.len() {
                        if tj == ti || !self.towns[tj].alive {
                            continue;
                        }
                        let d = self.cheb(x, y, self.towns[tj].x, self.towns[tj].y);
                        if d < bd2 {
                            bd2 = d;
                            bb = Some(tj);
                        }
                    }
                    bb.unwrap_or(ti)
                }
            };
            if target == ti {
                continue;
            }
            self.caravans.push(Caravan { home: ti, target, x, y, goods, gift: false });
        }
    }

    fn caravans_step(&mut self) {
        for i in 0..self.caravans.len() {
            let (home, target, x, y, gift) = {
                let c = &self.caravans[i];
                (c.home, c.target, c.x, c.y, c.gift)
            };
            let (tx, ty) = (self.towns[target].x, self.towns[target].y);
            if self.cheb(x, y, tx, ty) <= 1 {
                let goods = std::mem::take(&mut self.caravans[i].goods);
                if !gift {
                    let gold: f32 = goods.iter().map(|(k, q)| q * trade_price(*k)).sum();
                    let s = &mut self.towns[home].stocks;
                    s.gold = (s.gold + gold).min(GOLD_MAX);
                }
                let st = &mut self.towns[target].stocks;
                for (k, q) in &goods {
                    match k {
                        ResourceKind::Food => st.food += q,
                        ResourceKind::Water => st.water += q,
                        ResourceKind::Ore => st.ore += q,
                        ResourceKind::Meat => st.meat += q,
                        ResourceKind::Gold => {}
                        ResourceKind::Fish => st.fish += q,
                    }
                }
            } else {
                let (nx, ny) = self.caravan_step(x, y, tx, ty);
                self.caravans[i].x = nx;
                self.caravans[i].y = ny;
                if self.roads[idx(nx, ny)] {
                    let (nx2, ny2) = self.caravan_step(nx, ny, tx, ty);
                    self.caravans[i].x = nx2;
                    self.caravans[i].y = ny2;
                }
            }
        }
        self.caravans.retain(|c| !c.goods.is_empty());
    }

    fn caravan_step(&self, x: i32, y: i32, tx: i32, ty: i32) -> (i32, i32) {
        let mut best = (x, y);
        let mut bs = i32::MIN;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if !in_bounds(nx, ny) || !self.grid[idx(nx, ny)].terrain.walkable() {
                    continue;
                }
                let d = (nx - tx).pow(2) + (ny - ty).pow(2);
                let j = (self.brain(nx, ny, self.tick_count) % 7) as i32 - 3;
                if -d + j > bs {
                    bs = -d + j;
                    best = (nx, ny);
                }
            }
        }
        best
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
                    Terrain::Jungle => c.food = FOOD_MAX * 1.5,
                    Terrain::Tundra => c.food = (c.food + FOOD_MAX * 0.5).min(FOOD_MAX * 0.4),
                    Terrain::Farm | Terrain::Hills | Terrain::Water | Terrain::Desert => {}
                }
            }
        }
    }

    fn promote_priest(&mut self, ti: usize) {
        if self.towns[ti].built.iter().any(|b| *b == BuildingKind::Sanctuary) {
            return;
        }
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if self.families[fid].role == Role::Worker {
                self.families[fid].role = Role::Priest;
                for a in self.agents.iter_mut() {
                    if a.family == fid {
                        a.role = Role::Priest;
                    }
                }
                break;
            }
        }
    }

    fn promote_prophet(&mut self, ti: usize) {
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if self.families[fid].role == Role::Priest {
                self.families[fid].role = Role::Prophet;
                for a in self.agents.iter_mut() {
                    if a.family == fid {
                        a.role = Role::Prophet;
                    }
                }
                break;
            }
        }
    }

    fn promote_healer(&mut self, ti: usize) {
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if self.families[fid].role == Role::Worker
                || self.families[fid].role == Role::Hunter
            {
                self.families[fid].role = Role::Healer;
                for a in self.agents.iter_mut() {
                    if a.family == fid {
                        a.role = Role::Healer;
                    }
                }
                break;
            }
        }
    }

    fn promote_guard(&mut self, ti: usize) {
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if self.families[fid].role == Role::Priest || self.families[fid].role == Role::Healer {
                continue;
            }
            self.families[fid].role = Role::Guard;
            for a in self.agents.iter_mut() {
                if a.family == fid {
                    a.role = Role::Guard;
                }
            }
            break;
        }
    }

    fn promote_scholar(&mut self, ti: usize) {
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if matches!(
                self.families[fid].role,
                Role::Priest | Role::Healer | Role::Guard | Role::Scholar
            ) {
                continue;
            }
            self.families[fid].role = Role::Scholar;
            for a in self.agents.iter_mut() {
                if a.family == fid {
                    a.role = Role::Scholar;
                }
            }
            break;
        }
    }

    fn promote_builder(&mut self, ti: usize) {
        for fid in 0..self.families.len() {
            if self.families[fid].town != ti || self.families[fid].extinct {
                continue;
            }
            if matches!(
                self.families[fid].role,
                Role::Priest | Role::Healer | Role::Guard | Role::Scholar | Role::Builder
            ) {
                continue;
            }
            self.families[fid].role = Role::Builder;
            for a in self.agents.iter_mut() {
                if a.family == fid {
                    a.role = Role::Builder;
                }
            }
            break;
        }
    }

    fn plant_fields(&mut self, cx: i32, cy: i32) {
        let mut cands = Vec::new();
        for dy in -6..=6 {
            for dx in -6..=6 {
                let x = cx + dx;
                let y = cy + dy;
                if !in_bounds(x, y) {
                    continue;
                }
                let c = &self.grid[idx(x, y)];
                if c.terrain == Terrain::Grass {
                    cands.push((x, y));
                }
            }
        }
        cands.sort_by_key(|(x, y)| (x - cx).abs().max(y - cy));
        let mut made = 0;
        for (x, y) in cands {
            if made >= FARM_PATCH {
                break;
            }
            let c = &mut self.grid[idx(x, y)];
            if c.terrain == Terrain::Grass {
                c.terrain = Terrain::Farm;
                c.food = FARM_FOOD_MAX;
                made += 1;
            }
        }
    }

    pub fn build_request(&mut self, ti: usize, kind: BuildingKind) {
        if ti < self.towns.len() && self.can_build(ti, kind) {
            self.towns[ti].queue.push((kind, 0.0));
        }
    }

    fn tech_tier(&self, ti: usize) -> u32 {
        let d = self.towns[ti].dev;
        if d >= TECH_TIER3 {
            3
        } else if d >= TECH_TIER2 {
            2
        } else if d >= TECH_TIER1 {
            1
        } else {
            0
        }
    }

    pub fn can_build(&self, ti: usize, kind: BuildingKind) -> bool {
        match kind {
            BuildingKind::University => self.tech_tier(ti) >= 1,
            BuildingKind::Smithy => self.tech_tier(ti) >= 2,
            BuildingKind::Library => self.tech_tier(ti) >= 3,
            _ => true,
        }
    }

    fn tech_step(&mut self) {
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                continue;
            }
            if self.pop(ti) < SCIENCE_REQ_POP {
                continue;
            }
            let (uni, lib, scholars) = {
                let t = &self.towns[ti];
                let uni = t
                    .built
                    .iter()
                    .filter(|b| **b == BuildingKind::University)
                    .count();
                let lib = t
                    .built
                    .iter()
                    .filter(|b| **b == BuildingKind::Library)
                    .count();
                let scholars = self
                    .agents
                    .iter()
                    .filter(|a| a.home == ti && a.role == Role::Scholar)
                    .count();
                (uni, lib, scholars)
            };
            let gain = DEV_BASE
                + DEV_UNI_BONUS * uni as f32
                + DEV_LIB_BONUS * lib as f32
                + DEV_SCHOLAR_BONUS * scholars as f32;
            self.towns[ti].dev += gain;
        }
    }

    fn construction(&mut self) {
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                continue;
            }
            let smithy_bonus = self.towns[ti]
                .built
                .iter()
                .filter(|b| **b == BuildingKind::Smithy)
                .count();
            let builder_bonus = self
                .agents
                .iter()
                .filter(|a| a.home == ti && a.role == Role::Builder)
                .count();
            let pop_ti = self.pop(ti);
            let has_sick = self.agents.iter().any(|a| a.home == ti && a.sick > 0);
            let apply = {
                let t = &mut self.towns[ti];
                if t.queue.is_empty() && pop_ti > 0 {
                    let has = |k: BuildingKind| t.built.iter().any(|b| *b == k);
                    let need = |k: BuildingKind| t.queue.iter().any(|(q, _)| *q == k);
                    if !has(BuildingKind::Well) && !need(BuildingKind::Well) {
                        t.queue.push((BuildingKind::Well, 0.0));
                    } else if pop_ti >= t.cap && !need(BuildingKind::House) {
                        t.queue.push((BuildingKind::House, 0.0));
                    } else if !has(BuildingKind::Farm) && !need(BuildingKind::Farm) {
                        t.queue.push((BuildingKind::Farm, 0.0));
                    } else if t.stocks.food > 80.0 && t.stocks.water > 60.0
                        && !has(BuildingKind::TradePost) && !need(BuildingKind::TradePost)
                    {
                        t.queue.push((BuildingKind::TradePost, 0.0));
                    } else if !has(BuildingKind::Clinic) && !need(BuildingKind::Clinic)
                        && has_sick
                    {
                        t.queue.push((BuildingKind::Clinic, 0.0));
                    } else if !has(BuildingKind::Sanctuary) && !need(BuildingKind::Sanctuary)
                        && t.faith >= 10.0
                    {
                        t.queue.push((BuildingKind::Sanctuary, 0.0));
                    } else if t.at_war && !has(BuildingKind::Wall) && !need(BuildingKind::Wall) {
                        t.queue.push((BuildingKind::Wall, 0.0));
                    } else if t.at_war && !has(BuildingKind::Barracks) && !need(BuildingKind::Barracks) {
                        t.queue.push((BuildingKind::Barracks, 0.0));
                    } else if pop_ti >= 15 && !has(BuildingKind::University) && !need(BuildingKind::University) {
                        t.queue.push((BuildingKind::University, 0.0));
                    } else if pop_ti >= 20 && !has(BuildingKind::Smithy) && !need(BuildingKind::Smithy) {
                        t.queue.push((BuildingKind::Smithy, 0.0));
                    } else if pop_ti >= 25 && t.stocks.gold >= 20.0
                        && !has(BuildingKind::Library) && !need(BuildingKind::Library)
                    {
                        t.queue.push((BuildingKind::Library, 0.0));
                    } else if has(BuildingKind::Sanctuary) && t.faith >= 25.0
                        && !has(BuildingKind::Temple) && !need(BuildingKind::Temple)
                    {
                        t.queue.push((BuildingKind::Temple, 0.0));
                    } else if pop_ti >= t.cap && has(BuildingKind::House) && !need(BuildingKind::House) {
                        t.queue.push((BuildingKind::House, 0.0));
                    }
                }
                if t.queue.is_empty() {
                    continue;
                }
                if t.stocks.food < BUILD_MIN_FOOD || t.stocks.water < BUILD_MIN_WATER {
                    continue;
                }
                if t.stocks.ore < 1.0 {
                    continue;
                }
                t.stocks.ore = (t.stocks.ore - 1.0).max(0.0);
                let (kind, progress) = &mut t.queue[0];
                *progress += if t.idea == TownIdea::Toil { 2.0 } else { 1.0 };
                *progress += (smithy_bonus + builder_bonus) as f32;
                if *progress >= kind.cost() {
                    Some(t.queue.remove(0).0)
                } else {
                    None
                }
            };
            if let Some(k) = apply {
                if k == BuildingKind::House {
                    self.towns[ti].cap += HOUSE_CAP_BONUS;
                }
                if k == BuildingKind::Farm {
                    let (x, y) = {
                        let t = &self.towns[ti];
                        (t.x, t.y)
                    };
                    self.plant_fields(x, y);
                }
                if k == BuildingKind::Sanctuary {
                    self.promote_priest(ti);
                }
                if k == BuildingKind::Clinic {
                    self.promote_healer(ti);
                }
                if k == BuildingKind::Barracks {
                    self.promote_guard(ti);
                }
                if k == BuildingKind::Temple {
                    self.promote_prophet(ti);
                }
                if k == BuildingKind::University {
                    self.promote_scholar(ti);
                }
                if k == BuildingKind::Smithy {
                    self.promote_builder(ti);
                }
                self.towns[ti].built.push(k);
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.weather_breath();
        self.day_phase = (self.day_phase + 1) % DAY_LEN;
        if self.tick_count % SEASON_LEN == 0 {
            self.season = match self.season {
                Season::Spring => Season::Summer,
                Season::Summer => Season::Autumn,
                Season::Autumn => Season::Winter,
                Season::Winter => Season::Spring,
            };
        }

        let hunger_rate = match self.weather {
            Weather::Frost => 1.3,
            _ => 1.1,
        };
        let thirst_rate = match self.weather {
            Weather::Rain => 0.5,
            Weather::Heat => 1.1,
            _ => 0.8,
        };
        // добавлю возраст в цикл голода/жажды
        for a in self.agents.iter_mut() {
            a.age = a.age.saturating_add(1);
            let mut metabolism = if a.age < CHILD_AGE { 0.8 } else { 1.0 };
            if a.age > OLD_AGE {
                metabolism = 1.25;
            }
            let sick_rate = if a.sick > 0 { 1.4 * metabolism } else { metabolism };
            a.hunger = (a.hunger + hunger_rate * sick_rate).min(140.0);
            let thirst_sick = if a.sick > 0 { 1.3 * metabolism } else { metabolism };
            a.thirst = (a.thirst + thirst_rate * thirst_sick).min(140.0);
        }

        if self.tick_count % REGROW_EVERY == 0 {
            let abundant = self.towns.iter().any(|t| t.blessing == Blessing::Abundance);
            let proph_harvest = self.towns.iter().any(|t| t.prophecy == Prophecy::Harvest);
            let proph_rain = self.towns.iter().any(|t| t.prophecy == Prophecy::Rain);
            let berry = match self.weather {
                Weather::Rain => 2.0,
                Weather::Frost => 0.5,
                _ => 1.0,
            };
            let crop = match self.weather {
                Weather::Rain => 3.0,
                Weather::Frost => 1.5,
                _ => 2.5,
            };
            let season_berry = match self.season {
                Season::Spring => 1.4,
                Season::Winter => 0.5,
                _ => 1.0,
            };
            let season_crop = if self.season == Season::Winter { 0.6 } else { 1.0 };
            let season_water = if self.season == Season::Winter { 0.4 } else { 1.0 };
            let rain = self.weather == Weather::Rain;
            let water_regen = match self.weather {
                Weather::Rain => WATER_REGEN_RAIN,
                Weather::Heat => WATER_REGEN_HEAT,
                Weather::Frost => WATER_REGEN_FROST,
                Weather::Clear => WATER_REGEN_CLEAR,
            } * season_water + if proph_rain { 1.5 } else { 0.0 };
            let dt = self.tick_count / REGROW_EVERY;
            let mut new_lakes: Vec<(i32, i32)> = Vec::new();
            for y in 0..H {
                for x in 0..W {
                    let i = idx(x as i32, y as i32);
                    let on_road = self.roads[i];
                    match self.grid[i].terrain {
                        Terrain::Forest if !on_road => {
                            let k = (self.grid[i].food / FOOD_MAX).max(0.3);
                            let ber = berry * season_berry * k + if abundant { 1.0 } else { 0.0 } + if proph_harvest { 1.5 } else { 0.0 };
                            self.grid[i].food = (self.grid[i].food + ber).min(FOOD_MAX);
                        }
                        Terrain::Farm if !on_road => {
                            let cr = crop * season_crop + if abundant { 1.0 } else { 0.0 } + if proph_harvest { 2.0 } else { 0.0 };
                            self.grid[i].food = (self.grid[i].food + cr).min(FARM_FOOD_MAX);
                        }
                        Terrain::Water => {
                            self.grid[i].water = (self.grid[i].water + water_regen).min(WATER_MAX);
                            self.grid[i].food = (self.grid[i].food + 0.3).min(8.0);
                            if rain
                                && self.grid[i].water >= WATER_MAX * 0.85
                                && self.brain(x as i32, y as i32, dt) % 9000 < RAIN_LAKE_CHANCE
                                && new_lakes.len() < 4
                            {
                                let h = self.brain(x as i32, y as i32, dt + 1);
                                let n = (h >> 3) % 4;
                                let (dx, dy) = match n {
                                    0 => (1, 0),
                                    1 => (-1, 0),
                                    2 => (0, 1),
                                    _ => (0, -1),
                                };
                                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                                if in_bounds(nx, ny)
                                    && self.grid[idx(nx, ny)].terrain == Terrain::Grass
                                {
                                    new_lakes.push((nx, ny));
                                }
                            }
                        }
                        Terrain::Hills | Terrain::Grass | Terrain::Forest => {}
                        Terrain::Jungle if !on_road => {
                            let k = (self.grid[i].food / (FOOD_MAX * 1.5)).max(0.3);
                            let ber = berry * season_berry * 1.3 * k + if abundant { 1.5 } else { 0.0 };
                            self.grid[i].food = (self.grid[i].food + ber).min(FOOD_MAX * 1.5);
                        }
                        Terrain::Tundra if !on_road => {
                            let k = (self.grid[i].food / (FOOD_MAX * 0.4)).max(0.2);
                            let ber = berry * season_berry * 0.4 * k;
                            self.grid[i].food = (self.grid[i].food + ber).min(FOOD_MAX * 0.4);
                        }
                        Terrain::Desert if !on_road => {
                            let ber = berry * season_berry * 0.1;
                            self.grid[i].food = (self.grid[i].food + ber).min(FOOD_MAX * 0.1);
                        }
                        _ => {}
                    }
                }
            }
            for (x, y) in new_lakes {
                let c = &mut self.grid[idx(x, y)];
                c.terrain = Terrain::Water;
                c.food = 0.0;
                c.ore = 0.0;
                c.water = WATER_MAX;
            }
        }

        let wmult = if self.is_night() { NIGHT_WORK_MULT } else { 1.0 };
        let pops: Vec<usize> = (0..self.towns.len()).map(|ti| self.pop(ti)).collect();
        for (ti, t) in self.towns.iter_mut().enumerate() {
            if !t.alive {
                continue;
            }
            let wells = t.built.iter().filter(|b| **b == BuildingKind::Well).count();
            if wells > 0 && pops[ti] > 0 {
                t.stocks.water = (t.stocks.water + WELL_WATER_PER_TICK * wells as f32 * wmult).min(200.0);
            }
            let posts = t.built.iter().filter(|b| **b == BuildingKind::TradePost).count();
            if posts > 0 {
                t.stocks.gold = (t.stocks.gold + TRADE_TRICKLE * posts as f32).min(GOLD_MAX);
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
            let sanctuaries = t.built.iter().filter(|b| **b == BuildingKind::Sanctuary).count() as f32;
            if sanctuaries > 0.0 {
                t.faith = (t.faith + FAITH_GAIN_PER_TICK * sanctuaries).min(100.0);
            }
            if t.blessing_left > 0.0 {
                t.blessing_left -= 1.0;
                if t.blessing_left <= 0.0 {
                    t.blessing = Blessing::None;
                }
            }
            if self.tick_count % RITUAL_EVERY == 0 && t.faith >= FAITH_SPEND {
                t.faith -= FAITH_SPEND;
                let roll = rfrac(&mut self.rng);
                t.blessing = if roll < 0.4 {
                    Blessing::Fertility
                } else if roll < 0.7 {
                    Blessing::Abundance
                } else {
                    Blessing::Protection
                };
                t.blessing_left = BLESS_LEN;
            }
            if t.prophecy_left > 0.0 {
                t.prophecy_left -= 1.0;
                if t.prophecy_left <= 0.0 {
                    t.prophecy = Prophecy::None;
                }
            }
            let temples = t.built.iter().filter(|b| **b == BuildingKind::Temple).count() as f32;
            if temples > 0.0 {
                t.revelation = (t.revelation + REVELATION_PER_TICK * temples).min(100.0);
            }
            if self.tick_count % RITUAL_EVERY == 0 && t.revelation >= PROPHECY_COST && t.prophecy == Prophecy::None {
                t.revelation -= PROPHECY_COST;
                let roll = rfrac(&mut self.rng);
                t.prophecy = if roll < 0.25 {
                    Prophecy::Harvest
                } else if roll < 0.50 {
                    Prophecy::Rain
                } else if roll < 0.70 {
                    Prophecy::PlagueWarning
                } else if roll < 0.85 {
                    Prophecy::Prosperity
                } else {
                    Prophecy::HolyWar
                };
                t.prophecy_left = PROPHECY_LEN;
            }
            if t.prophecy == Prophecy::Prosperity && self.tick_count % 100 == 0 {
                t.stocks.gold = (t.stocks.gold + 2.0).min(500.0);
            }
        }

        self.construction();
        self.animals_step();
        self.caravans_step();
        self.market_buy();
        self.export_caravans();
        self.social_step();
        self.plague_step();
        self.heal_step();

        let actions: Vec<(Action, ResourceKind)> = self.agents.iter().map(|a| self.decide(a)).collect();
        let mut dead = Vec::new();
        for (i, (act, want)) in actions.into_iter().enumerate() {
            self.agents[i].want = want;
            self.apply(i, act, &mut dead);
        }
        for (i, a) in self.agents.iter_mut().enumerate() {
            let h = (a.x as u32).wrapping_mul(0x45d9_f3b)
                ^ (a.y as u32).wrapping_mul(0x119d_e1f3)
                ^ a.age.wrapping_mul(0xabcd_ef01);
            if a.age > OLD_AGE && h % 16384 == 0 {
                dead.push(i);
            }
        }
        dead.sort_unstable();
        dead.dedup();
        for &i in dead.iter().rev() {
            self.agents.remove(i);
        }
        self.release_dead_raiders(&dead);

        if self.tick_count % BIRTH_EVERY == 0 {
            self.reproduction();
        }
        self.sync_families();
        self.retrain_roles();
        if self.tick_count % EMPIRE_EVERY == 0 {
            self.empire_step();
        }
        self.war_step();
        if self.tick_count % MIGRATE_EVERY == 0 {
            self.migration_step();
        }
        if self.tick_count % MARRIAGE_EVERY == 0 {
            self.marriage_step();
        }
        if self.tick_count % TREATY_EVERY == 0 {
            self.treaty_step();
        }
        self.gift_step();
        if self.tick_count % TOWNS_EVERY == 0 {
            self.town_lifecycle();
        }
        if self.tick_count % METEOR_EVERY == 0 {
            self.meteor_step();
        }
        self.fire_step();
        if self.tick_count % HORDE_EVERY == 0 {
            self.horde_step();
        }
        self.gold_vein_trickle();
        if self.tick_count % GOLD_VEIN_EVERY == 0 {
            self.gold_vein_find();
        }
        if self.tick_count % TECH_EVERY == 0 {
            self.tech_step();
        }
    }

    fn weather_breath(&mut self) {
        if self.weather_left > 0.0 {
            self.weather_left -= 1.0;
            return;
        }
        let p = rfrac(&mut self.rng);
        let (clear, rain, heat, _frost) = match self.season {
            Season::Spring => (0.35, 0.35, 0.10, 0.20),
            Season::Summer => (0.45, 0.10, 0.35, 0.10),
            Season::Autumn => (0.50, 0.20, 0.10, 0.20),
            Season::Winter => (0.40, 0.10, 0.05, 0.45),
        };
        self.weather = if p < clear {
            Weather::Clear
        } else if p < clear + rain {
            Weather::Rain
        } else if p < clear + rain + heat {
            Weather::Heat
        } else {
            Weather::Frost
        };
        self.weather_left = 300.0 + rfrac(&mut self.rng) as f64 * 400.0;
    }

    pub fn is_night(&self) -> bool {
        self.day_phase >= DAY_LEN / 2
    }

    pub fn is_day(&self) -> bool {
        !self.is_night()
    }

    pub fn save_json(&self) -> String {
        serde_json::to_string(self).expect("serialize sim")
    }

    pub fn load_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    fn animals_step(&mut self) {
        let mut dead: Vec<usize> = Vec::new();
        for i in 0..self.animals.len() {
            let (spec, x, y, home) = {
                let a = &self.animals[i];
                (a.species, a.x, a.y, a.home)
            };
            match spec {
                Species::Deer => {
                    if self.nearest_agent(x, y, 5).is_some() {
                        let (nx, ny) = self.flee_from_agents(x, y);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    } else if let Some((wx, wy)) = self.nearest_animal_of(x, y, 6, Species::Wolf, None) {
                        let (nx, ny) = self.animal_step(x, y, wx, wy, true);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    } else {
                        let (nx, ny) = self.animal_wander(x, y, i);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    }
                }
                Species::Boar => {
                    if let Some((ax, ay)) = self.nearest_agent(x, y, 3) {
                        if self.cheb(ax, ay, x, y) <= 1 && rfrac(&mut self.rng) < BOAR_TUSK_CHANCE {
                            self.bite_agent(ax, ay);
                        }
                        let (nx, ny) = self.animal_step(x, y, ax, ay, true);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    } else {
                        let (nx, ny) = self.animal_wander(x, y, i);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    }
                }
                Species::Wolf => {
                    let prey_i = self.nearest_wild_prey_i(x, y, WOLF_TARGET_RADIUS);
                    if let Some(p) = prey_i {
                        let (px, py) = (self.animals[p].x, self.animals[p].y);
                        if self.cheb(x, y, px, py) <= 1 && rfrac(&mut self.rng) < 0.3 {
                            dead.push(p);
                        } else {
                            let (nx, ny) = self.animal_step(x, y, px, py, false);
                            self.animals[i].x = nx;
                            self.animals[i].y = ny;
                        }
                    } else if let Some(ci) = self.nearest_cow_i(x, y, 8) {
                        let (cx, cy) = (self.animals[ci].x, self.animals[ci].y);
                        let human = self.nearest_agent_dist(x, y, 9);
                        let go_cow = match human {
                            Some((_, _, hd)) => {
                                self.cheb(cx, cy, x, y) < hd || rfrac(&mut self.rng) < 0.75
                            }
                            None => true,
                        };
                        if go_cow {
                            if self.cheb(x, y, cx, cy) <= 1 {
                                if rfrac(&mut self.rng) < 0.25 {
                                    self.animals[ci].hp -= Species::Cow.hp() * 0.4;
                                    if self.animals[ci].hp <= 0.0 {
                                        dead.push(ci);
                                    }
                                }
                            } else {
                                let (nx, ny) = self.animal_step(x, y, cx, cy, false);
                                self.animals[i].x = nx;
                                self.animals[i].y = ny;
                            }
                        }
                    } else if let Some((hx, hy)) = self.nearest_agent(x, y, 9) {
                        if self.cheb(x, y, hx, hy) <= 2 && rfrac(&mut self.rng) < WOLF_BITE_CHANCE {
                            self.bite_agent(hx, hy);
                        } else {
                            let (nx, ny) = self.animal_step(x, y, hx, hy, false);
                            self.animals[i].x = nx;
                            self.animals[i].y = ny;
                        }
                    } else {
                        let (nx, ny) = self.animal_wander(x, y, i);
                        self.animals[i].x = nx;
                        self.animals[i].y = ny;
                    }
                }
                Species::Cow => {
                    let (nx, ny) = if let Some(ti) = home {
                        let town = &self.towns[ti];
                        if self.cheb(x, y, town.x, town.y) > 3 {
                            self.animal_step(x, y, town.x, town.y, false)
                        } else {
                            self.animal_wander(x, y, i)
                        }
                    } else {
                        self.animal_wander(x, y, i)
                    };
                    self.animals[i].x = nx;
                    self.animals[i].y = ny;
                }
            }
        }
        dead.sort();
        dead.dedup();
        for &i in dead.iter().rev() {
            self.animals.remove(i);
        }

        if self.tick_count % ANIMAL_BREED_EVERY == 0 {
            let mut deer = 0;
            let mut boar = 0;
            let mut wolves = 0;
            for a in &self.animals {
                match a.species {
                    Species::Deer => deer += 1,
                    Species::Boar => boar += 1,
                    Species::Wolf => wolves += 1,
                    Species::Cow => {}
                }
            }
            if deer < 24 {
                if let Some((x, y)) = self.random_walkable(4.0, 0) {
                    self.push_animal(Species::Deer, x, y, None);
                }
            }
            if boar < 10 {
                if let Some((x, y)) = self.random_walkable(4.0, 0) {
                    self.push_animal(Species::Boar, x, y, None);
                }
            }
            if wolves < 4 {
                if let Some((x, y)) = self.random_walkable(WOLF_MIN_TOWN_DIST, 0) {
                    self.push_animal(Species::Wolf, x, y, None);
                }
            }
            for ti in 0..self.towns.len() {
                let herd = self.domestic_herd(ti);
                if herd >= 2 && herd <= DOMESTIC_HERD_CAP {
                    let (f, w) = {
                        let t = &self.towns[ti];
                        (t.stocks.food, t.stocks.water)
                    };
                    if f >= DOMESTIC_MILK_COST_FOOD && w >= DOMESTIC_MILK_COST_WATER {
                        self.towns[ti].stocks.food -= DOMESTIC_MILK_COST_FOOD;
                        self.towns[ti].stocks.water -= DOMESTIC_MILK_COST_WATER;
                        self.spawn_calf(ti);
                    }
                }
            }
        }
    }

    fn flee_from_agents(&self, x: i32, y: i32) -> (i32, i32) {
        let ax = (self.agents.iter().map(|a| a.x).sum::<i32>() as f32 / self.agents.len().max(1) as f32) as i32;
        let ay = (self.agents.iter().map(|a| a.y).sum::<i32>() as f32 / self.agents.len().max(1) as f32) as i32;
        self.animal_step(x, y, ax, ay, true)
    }

    fn cheb(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        (x1 - x2).abs().max((y1 - y2).abs())
    }

    fn nearest_agent(&self, x: i32, y: i32, max_d: i32) -> Option<(i32, i32)> {
        let mut best = None;
        let mut bd = max_d;
        for a in &self.agents {
            let d = self.cheb(a.x, a.y, x, y);
            if d <= bd {
                bd = d;
                best = Some((a.x, a.y));
            }
        }
        best
    }

    fn nearest_agent_dist(&self, x: i32, y: i32, max_d: i32) -> Option<(i32, i32, i32)> {
        self.nearest_agent(x, y, max_d).map(|(ax, ay)| (ax, ay, self.cheb(ax, ay, x, y)))
    }

    fn nearest_animal_of(
        &self,
        x: i32,
        y: i32,
        max_d: i32,
        spec: Species,
        home: Option<usize>,
    ) -> Option<(i32, i32)> {
        let mut best = None;
        let mut bd = max_d;
        for a in &self.animals {
            if a.species != spec || a.home != home {
                continue;
            }
            let d = self.cheb(a.x, a.y, x, y);
            if d <= bd {
                bd = d;
                best = Some((a.x, a.y));
            }
        }
        best
    }

    fn nearest_wild_prey_i(&self, x: i32, y: i32, max_d: i32) -> Option<usize> {
        let mut best = None;
        let mut bd = max_d;
        for (i, a) in self.animals.iter().enumerate() {
            if a.species != Species::Deer && a.species != Species::Boar {
                continue;
            }
            let d = self.cheb(a.x, a.y, x, y);
            if d <= bd {
                bd = d;
                best = Some(i);
            }
        }
        best
    }

    fn nearest_cow_i(&self, x: i32, y: i32, max_d: i32) -> Option<usize> {
        let mut best = None;
        let mut bd = max_d;
        for (i, a) in self.animals.iter().enumerate() {
            if a.species != Species::Cow || a.home.is_none() {
                continue;
            }
            let d = self.cheb(a.x, a.y, x, y);
            if d <= bd {
                bd = d;
                best = Some(i);
            }
        }
        best
    }

    fn bite_agent(&mut self, ax: i32, ay: i32) {
        if self.protected() {
            return;
        }
        for a in self.agents.iter_mut() {
            if (a.x - ax).abs().max((a.y - ay).abs()) <= 1 {
                a.hunger = (a.hunger + 15.0).min(140.0);
            }
        }
    }

    fn brain(&self, x: i32, y: i32, k: u64) -> u32 {
        (x as u32).wrapping_mul(0x45d9_f3b) ^ (y as u32).wrapping_mul(0x119d_e1f3) ^ (k as u32).wrapping_mul(0xabcd_ef01)
    }

    fn animal_step(&self, x: i32, y: i32, tx: i32, ty: i32, flee: bool) -> (i32, i32) {
        let mut best = (x, y);
        let mut bs = i32::MIN;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if !in_bounds(nx, ny) || !self.grid[idx(nx, ny)].terrain.walkable() {
                    continue;
                }
                let d2 = (nx - tx).pow(2) + (ny - ty).pow(2);
                let sc = if flee { d2 } else { -d2 };
                let j = (self.brain(nx, ny, self.tick_count) % 7) as i32 - 3;
                if sc + j > bs {
                    bs = sc + j;
                    best = (nx, ny);
                }
            }
        }
        best
    }

    fn animal_wander(&self, x: i32, y: i32, i: usize) -> (i32, i32) {
        for k in 0..8u32 {
            let h = (x as u32).wrapping_mul(0x21f0_aaad)
                ^ (y as u32 & 1).wrapping_mul(k + 1)
                ^ (self.tick_count as u32).wrapping_mul(0x0100_0101)
                ^ self.brain(x, y, i as u64);
            let dx = (h % 3) as i32 - 1;
            let dy = ((h >> 2) % 3) as i32 - 1;
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if in_bounds(nx, ny) && self.grid[idx(nx, ny)].terrain.walkable() {
                return (nx, ny);
            }
        }
        (x, y)
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
        if a.sick > 0 && (a.hunger >= 120.0 || a.thirst >= 120.0) {
            return (Action::Die, a.want);
        }
        if a.raider {
            return self.army_action(a);
        }
        let t = &self.towns[a.home];
        let (hx, hy) = (t.x, t.y);
        let at_home = (a.x - hx).abs() <= 5 && (a.y - hy).abs() <= 5;

        if let Some((kind, _)) = a.carry {
            if at_home {
                (Action::Deposit, kind)
            } else {
                let (nx, ny) = self.steer(a, hx, hy);
                (Action::Move(nx, ny), kind)
            }
        } else if at_home {
            if a.hunger >= HUNGRY_AT {
                let food = self.towns[a.home].stocks.food;
                let meat = self.towns[a.home].stocks.meat;
                if food > 0.0 || meat > 0.0 {
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
            } else if a.energy < 60.0 {
                (Action::Stay, a.want)
            } else {
                self.gather_action(a, None)
            }
        } else {
            let hungry = a.hunger >= HUNGRY_AT
                && (self.towns[a.home].stocks.food > 0.0 || self.towns[a.home].stocks.meat > 0.0);
            let thirsty = a.thirst >= THIRSTY_AT && self.towns[a.home].stocks.water > 0.0;
            if a.energy < 30.0 || hungry || thirsty {
                let (nx, ny) = self.steer(a, hx, hy);
                (Action::Move(nx, ny), a.want)
            } else {
                self.gather_action(a, None)
            }
        }
    }

    fn gather_action(&self, a: &Agent, force: Option<ResourceKind>) -> (Action, ResourceKind) {
        let kind = force.unwrap_or_else(|| self.role_kind(a));
        let d = match kind {
            ResourceKind::Food => self.food_target(a.x, a.y),
            ResourceKind::Water => self.water_target(a.x, a.y),
            ResourceKind::Ore => self.ore_target(a.x, a.y),
            ResourceKind::Meat => self.meat_target(a.x, a.y),
            ResourceKind::Gold => None,
            ResourceKind::Fish => self.fish_target(a.x, a.y),
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
                    ResourceKind::Ore => [ResourceKind::Food, ResourceKind::Meat],
                    ResourceKind::Meat => [ResourceKind::Food, ResourceKind::Water],
                    ResourceKind::Gold => [ResourceKind::Food, ResourceKind::Water],
                    ResourceKind::Fish => [ResourceKind::Food, ResourceKind::Water],
                };
                for k in others {
                    let d = match k {
                        ResourceKind::Food => self.food_target(a.x, a.y),
                        ResourceKind::Water => self.water_target(a.x, a.y),
                        ResourceKind::Ore => self.ore_target(a.x, a.y),
                        ResourceKind::Meat => self.meat_target(a.x, a.y),
                        ResourceKind::Gold => None,
                        ResourceKind::Fish => self.fish_target(a.x, a.y),
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
            Role::Hunter => (
                ResourceKind::Meat,
                st.meat < 15.0 && st.water > 8.0 && st.food > 8.0,
            ),
            Role::Priest => (
                ResourceKind::Gold,
                st.gold > 5.0 && st.water > 8.0 && st.food > 8.0,
            ),
            Role::Prophet => (
                ResourceKind::Gold,
                st.water > 8.0 && st.food > 8.0,
            ),
            Role::Healer => (need, true),
            Role::Guard => (need, need != ResourceKind::Meat),
            Role::Scholar => (need, true),
            Role::Builder => (need, true),
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
        let m = s.meat / 12.0;
        let fi = s.fish / 10.0;
        if w < f && w <= o && w <= m && w <= fi {
            ResourceKind::Water
        } else if fi < f && fi <= o && fi <= m {
            ResourceKind::Fish
        } else if m < f && m <= o {
            ResourceKind::Meat
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
            is_food_source(c) && c.food > 0.5
        })
    }

    fn water_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            s.grid[idx(nx, ny)].terrain.walkable() && s.water_adj_level(nx, ny)
        })
    }

    fn fish_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            if !s.grid[idx(nx, ny)].terrain.walkable() { return false; }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let wx = nx + dx;
                    let wy = ny + dy;
                    if !in_bounds(wx, wy) { continue; }
                    let ci = idx(wx, wy);
                    if s.grid[ci].terrain == Terrain::Water && s.grid[ci].food > 0.5 {
                        return true;
                    }
                }
            }
            false
        })
    }

    fn water_adj_level(&self, x: i32, y: i32) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let wx = x + dx;
                let wy = y + dy;
                if !in_bounds(wx, wy) {
                    continue;
                }
                if self.grid[idx(wx, wy)].terrain == Terrain::Water && self.grid[idx(wx, wy)].water > 0.5 {
                    return true;
                }
            }
        }
        false
    }

    fn ore_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        self.seek(x, y, |s, nx, ny| {
            let c = &s.grid[idx(nx, ny)];
            c.terrain == Terrain::Hills && c.ore > 0.5
        })
    }

    fn meat_target(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        let mut best = None;
        let mut bd = SEEK_RADIUS;
        for a in &self.animals {
            if a.home.is_some() && a.species == Species::Cow {
                continue;
            }
            let d = self.cheb(x, y, a.x, a.y);
            if d <= bd {
                bd = d;
                best = Some((a.x, a.y));
            }
        }
        best
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

    fn hunt_melee(&mut self, i: usize) {
        if self.agents[i].carry.is_some() {
            return;
        }
        let (ax, ay) = {
            let a = &self.agents[i];
            (a.x, a.y)
        };
        let mut best = None;
        let mut bd = ANIMAL_MELEE_REACH;
        for (k, an) in self.animals.iter().enumerate() {
            if an.home.is_some() && an.species == Species::Cow {
                continue;
            }
            let d = self.cheb(ax, ay, an.x, an.y);
            if d <= bd {
                bd = d;
                best = Some(k);
            }
        }
        let Some(k) = best else { return };
        let dmg = 22.0 + rfrac(&mut self.rng) * 16.0;
        self.animals[k].hp -= dmg;
        if self.animals[k].hp <= 0.0 {
            let qty = self.animals[k].species.meat_yield();
            self.animals.remove(k);
            self.agents[i].carry = Some((ResourceKind::Meat, qty));
        } else {
            let retal = match self.animals[k].species {
                Species::Wolf => 0.3,
                Species::Boar => 0.2,
                _ => 0.0,
            };
            if retal > 0.0 && rfrac(&mut self.rng) < retal {
                self.bite_agent(ax, ay);
            }
        }
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
                let on_road = self.roads[idx(nx, ny)];
                {
                    let a = &mut self.agents[i];
                    a.x = nx;
                    a.y = ny;
                    a.dir_x = (nx - ox).clamp(-1, 1);
                    a.dir_y = (ny - oy).clamp(-1, 1);
                    a.energy -= if on_road { 0.3 } else { 0.6 };
                    if a.carry.is_none() {
                        match a.want {
                            ResourceKind::Food => {
                                let c = &self.grid[idx(nx, ny)];
                                if is_food_source(c) && c.food > 0.5 {
                                    self.grid[idx(nx, ny)].food -= 1.0;
                                    a.carry = Some((ResourceKind::Food, 2.0));
                                }
                            }
                            ResourceKind::Water => {
                                if wadj {
                                    let mut sucked = 0.0;
                                    'neigh: for dy in -1..=1 {
                                        for dx in -1..=1 {
                                            if dx == 0 && dy == 0 {
                                                continue;
                                            }
                                            let wx = nx + dx;
                                            let wy = ny + dy;
                                            if !in_bounds(wx, wy) {
                                                continue;
                                            }
                                            if self.grid[idx(wx, wy)].terrain == Terrain::Water {
                                                let wcell = &mut self.grid[idx(wx, wy)];
                                                let give = wcell.water.min(WATER_SUCK);
                                                wcell.water = (wcell.water - give).max(0.0);
                                                sucked += give;
                                                break 'neigh;
                                            }
                                        }
                                    }
                                    if sucked >= WATER_SUCK * 0.5 {
                                        a.carry = Some((ResourceKind::Water, 5.0));
                                    }
                                }
                            }
                            ResourceKind::Ore => {
                                let c = &self.grid[idx(nx, ny)];
                                if c.terrain == Terrain::Hills && c.ore > 0.5 {
                                    self.grid[idx(nx, ny)].ore -= 1.0;
                                    a.carry = Some((ResourceKind::Ore, 1.0));
                                }
                            }
                            ResourceKind::Meat => {}
                            ResourceKind::Gold => {}
                            ResourceKind::Fish => {
                                if wadj {
                                    for dy in -1..=1 {
                                        for dx in -1..=1 {
                                            if dx == 0 && dy == 0 { continue; }
                                            let wx = nx + dx;
                                            let wy = ny + dy;
                                            if !in_bounds(wx, wy) { continue; }
                                            let ci = idx(wx, wy);
                                            if self.grid[ci].terrain == Terrain::Water && self.grid[ci].food > 0.5 {
                                                self.grid[ci].food -= 1.0;
                                                a.carry = Some((ResourceKind::Fish, 3.0));
                                                break;
                                            }
                                        }
                                        if a.carry.is_some() { break; }
                                    }
                                }
                            }
                        }
                    }
                }
                if self.agents[i].carry.is_none() && self.agents[i].want == ResourceKind::Meat && !self.agents[i].raider {
                    self.hunt_melee(i);
                }
                if self.agents[i].raider {
                    self.combat_check(i);
                }
            }
            Action::Stay => {
                let rest = if self.is_night() { 10.0 } else { 6.0 };
                let a = &mut self.agents[i];
                a.energy = (a.energy + rest).min(100.0);
            }
            Action::Eat => {
                let ti = self.agents[i].home;
                let mut ate = false;
                if self.towns[ti].stocks.food > 0.0 {
                    self.towns[ti].stocks.food = (self.towns[ti].stocks.food - 2.0).max(0.0);
                    ate = true;
                } else if self.towns[ti].stocks.meat > 0.0 {
                    self.towns[ti].stocks.meat = (self.towns[ti].stocks.meat - 2.0).max(0.0);
                    ate = true;
                } else if self.towns[ti].stocks.fish > 0.0 {
                    self.towns[ti].stocks.fish = (self.towns[ti].stocks.fish - 2.0).max(0.0);
                    ate = true;
                }
                if ate {
                    let a = &mut self.agents[i];
                    a.hunger = (a.hunger - 30.0).max(0.0);
                    a.energy = (a.energy + 4.0).min(100.0);
                }
            }
            Action::Drink => {
                let ti = self.agents[i].home;
                if self.towns[ti].stocks.water > 0.0 {
                    self.towns[ti].stocks.water = (self.towns[ti].stocks.water - 2.0).max(0.0);
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
                        ResourceKind::Meat => st.meat += qty,
                        ResourceKind::Gold => {}
                        ResourceKind::Fish => st.fish += qty,
                    }
                }
                a.hunger = (a.hunger - 5.0).max(0.0);
            }
            Action::Die => dead.push(i),
        }
    }

    fn protected(&self) -> bool {
        self.towns.iter().any(|t| t.blessing == Blessing::Protection)
    }

    fn social_step(&mut self) {
        let epoch = self.tick_count;
        let n = self.agents.len();
        for i in 0..n {
            let (ix, iy, ihome, ihunger, ithirst, ifamily) = {
                let a = &self.agents[i];
                (a.x, a.y, a.home, a.hunger, a.thirst, a.family)
            };
            let mut mood_delta = 0.0;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let a2 = &self.agents[j];
                let dx = (a2.x - ix).abs();
                let dy = (a2.y - iy).abs();
                if dx > FRIEND_RANGE || dy > FRIEND_RANGE {
                    continue;
                }
                let d = dx.max(dy);
                if a2.family == ifamily {
                    mood_delta += MOOD_NEAR_FRIEND / (d as f32 + 1.0);
                } else if d <= 3 {
                    mood_delta += MOOD_NEAR_ENEMY / (d as f32 + 1.0);
                }
            }
            if ihunger < 30.0 {
                mood_delta += MOOD_FED;
            } else if ihunger >= HUNGRY_AT {
                mood_delta += MOOD_HUNGRY;
            }
            if ithirst >= THIRSTY_AT {
                mood_delta += MOOD_HUNGRY;
            }
            if ihome < self.towns.len() {
                let t = &self.towns[ihome];
                if t.alive && t.stocks.food > 50.0 && t.stocks.water > 40.0 {
                    mood_delta += MOOD_PROSPER;
                }
            }
            if self.agents[i].sick > 0 {
                mood_delta -= 0.01;
            }
            self.agents[i].mood = (self.agents[i].mood + mood_delta).clamp(-1.0, 1.0);
        }
        if epoch % SOCIAL_LINK_CHANCE == 0 && n > 1 {
            let i = (self.brain(0, 0, epoch) as usize) % n;
            let j = (self.brain(1, 1, epoch) as usize) % n;
            if i != j {
                let dx = (self.agents[i].x - self.agents[j].x).abs();
                let dy = (self.agents[i].y - self.agents[j].y).abs();
                if dx <= FRIEND_RANGE && dy <= FRIEND_RANGE {
                    let bond = if self.agents[i].family == self.agents[j].family {
                        0.5
                    } else {
                        -0.3
                    };
                    if let Some(link) = self.social_links.iter_mut().find(|l| {
                        (l.a == i && l.b == j) || (l.a == j && l.b == i)
                    }) {
                        link.bond = (link.bond + bond * 0.1).clamp(-1.0, 1.0);
                    } else {
                        self.social_links.push(SocialLink { a: i, b: j, bond });
                    }
                }
            }
        }
        self.social_links.retain(|l| {
            l.bond.abs() > MOOD_LINK_DECAY && l.a < n && l.b < n
        });
        for link in self.social_links.iter_mut() {
            if link.bond > 0.0 {
                link.bond = (link.bond - MOOD_LINK_DECAY).max(0.0);
            } else {
                link.bond = (link.bond + MOOD_LINK_DECAY).min(0.0);
            }
        }
    }

    fn plague_step(&mut self) {
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                self.towns[ti].plague_until = 0;
                continue;
            }
            if self.towns[ti].plague_until > 0 {
                self.towns[ti].plague_until -= 1;
                let cure = self.towns[ti].built.iter().any(|b| *b == BuildingKind::Clinic);
                let chance = if cure { CONTAGION_CHANCE * 0.33 } else { CONTAGION_CHANCE };
                let mut inf = Vec::new();
                for j in 0..self.agents.len() {
                    let a = &self.agents[j];
                    if a.home != ti || a.sick > 0 {
                        continue;
                    }
                    let near_sick = self.agents.iter().any(|o| {
                        o.home == ti
                            && o.sick > 0
                            && self.cheb(o.x, o.y, a.x, a.y) <= CONTAGION_RADIUS
                    });
                    if near_sick && rfrac(&mut self.rng) < chance {
                        inf.push(j);
                    }
                }
                for j in inf {
                    self.agents[j].sick = (60 + (rfrac(&mut self.rng) * 140.0) as u32).min(SICK_MAX);
                }
            } else {
                let (cap, at_war, tx, ty) = {
                    let t = &self.towns[ti];
                    (t.cap, t.at_war, t.x, t.y)
                };
                let crowded = self.pop(ti) >= cap && cap >= 10;
                let foul = self.weather == Weather::Frost || self.weather == Weather::Rain;
                let prophylaxis = self.towns[ti].prophecy == Prophecy::PlagueWarning;
                if crowded && (foul || at_war) {
                    let h = self.brain(tx, ty, self.tick_count);
                    let dice = h % 100000;
                    let chance = if prophylaxis { PLAGUE_CHANCE * 0.3 } else { PLAGUE_CHANCE };
                    if dice < (chance * 100000.0) as u32 {
                        self.towns[ti].plague_until = PLAGUE_LEN;
                        let patient = self
                            .agents
                            .iter()
                            .position(|a| a.home == ti)
                            .unwrap_or(0);
                        if patient < self.agents.len() {
                            self.agents[patient].sick = (60 + (rfrac(&mut self.rng) * 60.0) as u32).min(SICK_MAX);
                        }
                    }
                }
            }
        }
    }

    fn heal_step(&mut self) {
        let mut healed = vec![0u32; self.agents.len()];
        for i in 0..self.agents.len() {
            if self.agents[i].role == Role::Healer {
                for j in 0..self.agents.len() {
                    if i == j || self.agents[j].sick == 0 {
                        continue;
                    }
                    if self.cheb(self.agents[i].x, self.agents[i].y, self.agents[j].x, self.agents[j].y)
                        <= HEAL_RADIUS
                    {
                        healed[j] += HEAL_PER_TICK;
                    }
                }
            }
        }
        for j in 0..self.agents.len() {
            let base = self.agents[j].home;
            if self.towns[base].built.iter().any(|b| *b == BuildingKind::Clinic) {
                healed[j] += 1;
            }
            let before = self.agents[j].sick;
            let sub = healed[j].min(self.agents[j].sick);
            self.agents[j].sick -= sub;
            if before > 0 && self.agents[j].sick == 0 {
                self.agents[j].hunger = (self.agents[j].hunger * 0.55).max(0.0);
                self.agents[j].thirst = (self.agents[j].thirst * 0.6).max(0.0);
            }
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
            let (cf, cw) = if self.towns[fam_town].idea == TownIdea::Prosperity
                || self.towns[fam_town].blessing == Blessing::Fertility
            {
                (BIRTH_FOOD * 0.5, BIRTH_WATER * 0.5)
            } else if self.has_alliance(fam_town) {
                (BIRTH_FOOD * 0.8, BIRTH_WATER * 0.8)
            } else {
                (BIRTH_FOOD, BIRTH_WATER)
            };
            self.towns[fam_town].stocks.food = (self.towns[fam_town].stocks.food - cf).max(0.0);
            self.towns[fam_town].stocks.water = (self.towns[fam_town].stocks.water - cw).max(0.0);
            self.spawn_agent(fam_town, tx, ty, fid, false);
            let newborn = self.agents.len() - 1;
            self.agents[newborn].age = 0;
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

    pub fn empire_of(&self, ti: usize) -> Option<usize> {
        self.towns[ti].empire
    }

    fn same_empire(&self, i: usize, j: usize) -> bool {
        matches!((self.empire_of(i), self.empire_of(j)), (Some(a), Some(b)) if a == b)
    }

    fn empire_step(&mut self) {
        let epoch = self.tick_count / EMPIRE_EVERY;
        for i in 0..self.towns.len() {
            if !self.towns[i].alive || self.towns[i].at_war {
                continue;
            }
            let h = self.brain(self.towns[i].x, self.towns[i].y, epoch);
            if h % EMPIRE_EPOCH_P != 0 {
                continue;
            }
            for j in 0..self.towns.len() {
                if j == i
                    || !self.towns[j].alive
                    || self.towns[j].at_war
                    || !self.neighbors(i, j)
                    || self.same_empire(i, j)
                {
                    continue;
                }
                self.pact(i, j);
                break;
            }
        }
    }

    fn pact(&mut self, i: usize, j: usize) {
        let ei = self.towns[i].empire;
        let ej = self.towns[j].empire;
        match (ei, ej) {
            (None, None) => self.form_empire(i, j),
            (Some(e), None) => self.admit_town(e, j),
            (None, Some(e)) => self.admit_town(e, i),
            (Some(a), Some(b)) if a != b => self.merge_empires(a, b),
            _ => {}
        }
    }

    fn form_empire(&mut self, i: usize, j: usize) {
        let h = self.brain(
            self.towns[i].x + self.towns[j].x,
            self.towns[i].y + self.towns[j].y,
            0xE1E,
        );
        let r = (150 + h % 100) as u8;
        let g = (110 + (h >> 8) % 110) as u8;
        let b = (90 + (h >> 16) % 130) as u8;
        let name = self
            .families
            .iter()
            .filter(|f| f.town == i && !f.extinct)
            .max_by_key(|f| f.members)
            .map(|f| f.name.clone())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("Династия №{}", self.empires.len() + 1));
        self.empires.push(Empire { r, g, b, name, members: vec![i, j] });
        self.towns[i].empire = Some(self.empires.len() - 1);
        self.towns[j].empire = Some(self.empires.len() - 1);
    }

    fn admit_town(&mut self, e: usize, t: usize) {
        if let Some(emp) = self.empires.get_mut(e) {
            emp.members.push(t);
        }
        self.towns[t].empire = Some(e);
    }

    fn merge_empires(&mut self, a: usize, b: usize) {
        if let (Some(ea), Some(eb)) = (self.empires.get(a), self.empires.get(b)) {
            let (keep, drop) = if ea.members.len() >= eb.members.len() { (a, b) } else { (b, a) };
            let drop_members: Vec<usize> = self.empires[drop].members.clone();
            self.empires[keep].members.extend(&drop_members);
            for t in drop_members {
                self.towns[t].empire = Some(keep);
            }
            self.empires[drop].members.clear();
        }
    }

    fn town_lifecycle(&mut self) {
        let epoch = self.tick_count / TOWNS_EVERY;
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                continue;
            }
            let ruined = self.pop(ti) == 0 && self.towns[ti].stocks.food < 12.0 && self.towns[ti].stocks.water < 8.0;
            let waste = if ruined { self.towns[ti].waste + 1 } else { 0 };
            self.towns[ti].waste = waste;
            if waste >= TOWN_WASTE_NEED {
                self.destroy_town(ti);
            }
        }
        if self.towns.len() >= MAX_TOWNS {
            return;
        }
        for mi in 0..self.towns.len() {
            if self.towns.len() >= MAX_TOWNS {
                break;
            }
            if !self.towns[mi].alive {
                continue;
            }
            let t = &self.towns[mi];
            if self.pop(mi) < FOUND_MIN_POP || t.stocks.food < FOUND_MIN_FOOD || t.stocks.water < FOUND_MIN_WATER {
                continue;
            }
            if self.brain(t.x, t.y, epoch) % FOUND_EPOCH_P != 0 {
                continue;
            }
            if let Some((x, y)) = self.colony_spot(mi) {
                self.found_colony(mi, x, y);
            }
        }
    }

    fn colony_spot(&self, mi: usize) -> Option<(i32, i32)> {
        let (mx, my) = (self.towns[mi].x, self.towns[mi].y);
        let mut best: Option<(i32, i32)> = None;
        let mut bd = i32::MAX;
        for dy in -FOUND_RADIUS_MAX..=FOUND_RADIUS_MAX {
            for dx in -FOUND_RADIUS_MAX..=FOUND_RADIUS_MAX {
                let d = dx.abs().max(dy.abs());
                if d < FOUND_RADIUS_MIN || d > FOUND_RADIUS_MAX {
                    continue;
                }
                let x = mx + dx;
                let y = my + dy;
                if !in_bounds(x, y) || self.grid[idx(x, y)].terrain != Terrain::Grass {
                    continue;
                }
                let mut too_close = false;
                for t in &self.towns {
                    if t.alive && ((x - t.x).abs().max(y - t.y)) < FOUND_RADIUS_MIN {
                        too_close = true;
                        break;
                    }
                }
                if too_close {
                    continue;
                }
                if d < bd {
                    bd = d;
                    best = Some((x, y));
                }
            }
        }
        best
    }

    fn found_colony(&mut self, _mi: usize, x: i32, y: i32) {
        let h = self.brain(x, y, 0xF0C0);
        let r = (150 + h % 100) as u8;
        let g = (110 + (h >> 8) % 110) as u8;
        let b = (90 + (h >> 16) % 130) as u8;
        for dy in -4..=4 {
            for dx in -4..=4 {
                let nx = x + dx;
                let ny = y + dy;
                if in_bounds(nx, ny) {
                    self.grid[idx(nx, ny)].terrain = Terrain::Grass;
                }
            }
        }
        self.towns.push(Settlement {
            x,
            y,
            stocks: Stock { food: 120.0, water: 120.0, ore: 40.0, meat: 20.0, gold: 0.0, fish: 0.0 },
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
            faith: 0.0,
            blessing: Blessing::None,
            blessing_left: 0.0,
            prophecy: Prophecy::None,
            prophecy_left: 0.0,
            revelation: 0.0,
            plague_until: 0,
            empire: None,
            alive: true,
            waste: 0,
            dev: 0.0,
        });
        let ti = self.towns.len() - 1;
        let fid = self.families.len();
        self.families.push(Family {
            id: fid,
            town: ti,
            members: FOUND_COLONY_POP as u32,
            children: 0,
            name: Self::family_name(&mut self.rng),
            extinct: false,
            accent: Self::family_accent(0, r, g, b),
            role: Role::Worker,
        });
        for k in 0..FOUND_COLONY_POP {
            self.spawn_agent(ti, x, y, fid, k < 2);
        }
        for a in self.agents.iter_mut().filter(|a| a.home == ti) {
            let hh = (a.x as u32).wrapping_mul(0x45d9_f3b)
                ^ (a.y as u32).wrapping_mul(0x119d_e1f3)
                ^ 0x9e37_79b9u32.wrapping_mul(0xabcd_ef01);
            a.age = 2000 + (hh % 12000);
        }
        let parent = _mi;
        if parent < self.towns.len() {
            let (px, py) = (self.towns[parent].x, self.towns[parent].y);
            self.build_road_between(px, py, x, y);
        }
    }

    fn can_road_at(&self, x: i32, y: i32) -> bool {
        if !in_bounds(x, y) { return false; }
        let c = &self.grid[idx(x, y)];
        c.terrain != Terrain::Water && c.terrain != Terrain::Hills
    }

    pub fn toggle_road(&mut self, x: i32, y: i32) {
        if !in_bounds(x, y) { return; }
        let i = idx(x, y);
        if self.roads[i] {
            self.roads[i] = false;
        } else if self.can_road_at(x, y) {
            self.roads[i] = true;
        }
    }

    fn build_road_between(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut cx = x0;
        let mut cy = y0;
        loop {
            if self.can_road_at(cx, cy) {
                self.roads[idx(cx, cy)] = true;
            }
            if cx == x1 && cy == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                cx += sx;
            }
            if e2 < dx {
                err += dx;
                cy += sy;
            }
        }
    }

    fn destroy_town(&mut self, ti: usize) {
        let t = &mut self.towns[ti];
        t.alive = false;
        t.at_war = false;
        t.raiders = 0;
        t.enemy = None;
        t.queue.clear();
        t.idea = TownIdea::None;
        t.blessing = Blessing::None;
        t.stocks = Stock { food: 0.0, water: 0.0, ore: 0.0, meat: 0.0, gold: 0.0, fish: 0.0 };
        if let Some(e) = t.empire {
            if let Some(emp) = self.empires.get_mut(e) {
                emp.members.retain(|&m| m != ti);
            }
        }
        t.empire = None;
        for f in self.families.iter_mut() {
            if f.town == ti {
                f.extinct = true;
                f.members = 0;
                f.children = 0;
            }
        }
        for a in self.animals.iter_mut() {
            if a.species == Species::Cow && a.home == Some(ti) {
                a.home = None;
            }
        }
        self.caravans.retain(|c| c.home != ti && c.target != ti);
    }

    fn meteor_step(&mut self) {
        let epoch = self.tick_count / METEOR_EVERY;
        for y in 0..H {
            for x in 0..W {
                let i = idx(x as i32, y as i32);
                if self.grid[i].terrain != Terrain::Grass {
                    continue;
                }
                if self.brain(x as i32, y as i32, epoch) % METEOR_CHANCE_P == 0 {
                    self.meteor_strike(x as i32, y as i32);
                    return;
                }
            }
        }
    }

    fn meteor_strike(&mut self, cx: i32, cy: i32) {
        for dy in -METEOR_RADIUS..=METEOR_RADIUS {
            for dx in -METEOR_RADIUS..=METEOR_RADIUS {
                let x = cx + dx;
                let y = cy + dy;
                if !in_bounds(x, y) {
                    continue;
                }
                let i = idx(x, y);
                if dx.abs() <= 1 && dy.abs() <= 1 {
                    self.grid[i].terrain = Terrain::Hills;
                    self.grid[i].ore = ORE_MAX;
                    self.grid[i].food = 0.0;
                    self.grid[i].water = 0.0;
                } else {
                    self.grid[i].terrain = Terrain::Grass;
                    self.grid[i].food = 0.0;
                }
            }
        }
    }

    fn town_quality(&self, ti: usize) -> f32 {
        let t = &self.towns[ti];
        t.stocks.food + t.stocks.water * 0.7 - (self.pop(ti) as f32 / t.cap.max(1) as f32) * 40.0
    }

    fn fire_spread(&mut self) {
        let mut ignites: Vec<usize> = Vec::new();
        for y in 0..H {
            for x in 0..W {
                let i = idx(x as i32, y as i32);
                if self.grid[i].burn == 0 {
                    continue;
                }
                self.grid[i].burn -= 1;
                if self.grid[i].burn == 0 {
                    self.grid[i].terrain = Terrain::Grass;
                    self.grid[i].food = 0.0;
                    continue;
                }
                if self.brain(x as i32, y as i32, self.tick_count / FIRE_SPREAD_DIV) % 2 == 0 {
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
                            let ni = idx(nx, ny);
                            if self.grid[ni].burn == 0
                                && (self.grid[ni].terrain == Terrain::Forest
                                    || self.grid[ni].terrain == Terrain::Farm
                                    || self.grid[ni].terrain == Terrain::Jungle)
                            {
                                ignites.push(ni);
                            }
                        }
                    }
                }
            }
        }
        for &ni in ignites.iter() {
            let c = &mut self.grid[ni];
            if c.burn == 0 && (c.terrain == Terrain::Forest || c.terrain == Terrain::Farm || c.terrain == Terrain::Jungle) {
                c.burn = FIRE_LEN;
                c.food = c.food.min(2.0);
            }
        }
    }

    fn fire_step(&mut self) {
        if self.tick_count % FIRE_EVERY == 0 {
            let epoch = self.tick_count / FIRE_EVERY;
            for y in 0..H {
                for x in 0..W {
                    let i = idx(x as i32, y as i32);
                    if self.grid[i].terrain != Terrain::Forest || self.grid[i].burn > 0 {
                        continue;
                    }
                    if self.brain(x as i32, y as i32, epoch) % FIRE_CHANCE_P == 0 {
                        self.grid[i].burn = FIRE_LEN;
                        self.grid[i].food = self.grid[i].food.min(2.0);
                        self.fire_spread();
                        return;
                    }
                }
            }
        } else {
            self.fire_spread();
        }
    }

    fn horde_step(&mut self) {
        let epoch = self.tick_count / HORDE_EVERY;
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive || self.animals.len() >= ANIMAL_MAX {
                continue;
            }
            let (tx, ty) = (self.towns[ti].x, self.towns[ti].y);
            if self.brain(tx, ty, epoch) % HORDE_CHANCE_P != 0 {
                continue;
            }
            let nearby = self
                .animals
                .iter()
                .filter(|a| a.species == Species::Wolf && (a.x - tx).abs().max(a.y - ty) <= 20)
                .count();
            if nearby >= 3 {
                continue;
            }
            let danger = if self.is_night() { NIGHT_DANGER } else { 1.0 };
            let base_pack =
                3 + (self.brain(tx, ty, epoch.wrapping_add(7)) % (HORDE_PACK_MAX as u32 - 2)) as usize;
            let pack = ((base_pack as f32 * danger).round() as usize).min(HORDE_PACK_MAX);
            let base = self.brain(tx, ty, epoch.wrapping_add(13));
            let mut placed = 0;
            for k in 0..pack {
                if self.animals.len() >= ANIMAL_MAX {
                    break;
                }
                let ang = (base.wrapping_add(k as u32 * 97) % 6283) as f64 * 0.001;
                let r = 8 + ((base >> 5) as usize % 6);
                let wx = tx + (ang.cos() * r as f64) as i32;
                let wy = ty + (ang.sin() * r as f64) as i32;
                if in_bounds(wx, wy) && self.grid[idx(wx, wy)].terrain.walkable() {
                    self.push_animal(Species::Wolf, wx, wy, None);
                    placed += 1;
                }
            }
            if placed > 0 {
                self.invades = self.invades.wrapping_add(1);
            }
        }
    }

    fn gold_vein_find(&mut self) {
        let epoch = self.tick_count / GOLD_VEIN_EVERY;
        let mut new_veins: Vec<(i32, i32)> = Vec::new();
        for ti in 0..self.towns.len() {
            if !self.towns[ti].alive {
                continue;
            }
            let (tx, ty) = (self.towns[ti].x, self.towns[ti].y);
            if self.brain(tx, ty, epoch) % GOLD_VEIN_CHANCE_P != 0 {
                continue;
            }
            let has_near = self
                .gold_veins
                .iter()
                .any(|&(vx, vy, _)| (tx - vx).abs().max(ty - vy) <= GOLD_VEIN_RANGE);
            if has_near {
                continue;
            }
            let mut cands: Vec<(i32, i32)> = Vec::new();
            for dy in -16i32..=16 {
                for dx in -16i32..=16 {
                    let d = dx.abs().max(dy.abs());
                    if d < 5 || d > 16 {
                        continue;
                    }
                    let nx = tx + dx;
                    let ny = ty + dy;
                    if in_bounds(nx, ny) && self.grid[idx(nx, ny)].terrain.walkable() {
                        cands.push((nx, ny));
                    }
                }
            }
            if cands.is_empty() {
                continue;
            }
            let pick = cands[(self.brain(tx, ty, epoch.wrapping_add(3)) as usize) % cands.len()];
            new_veins.push(pick);
        }
        for (x, y) in new_veins {
            let c = &mut self.grid[idx(x, y)];
            c.terrain = Terrain::Hills;
            c.ore = 0.0;
            c.food = 0.0;
            c.water = 0.0;
            c.gold = GOLD_VEIN_AMOUNT;
            self.gold_veins.push((x, y, GOLD_VEIN_AMOUNT));
        }
    }

    fn gold_vein_trickle(&mut self) {
        let veins = std::mem::take(&mut self.gold_veins);
        for (vx, vy, mut amt) in veins {
            let mut n = 0;
            for t in &self.towns {
                if t.alive && (t.x - vx).abs().max(t.y - vy) <= GOLD_VEIN_RANGE {
                    n += 1;
                }
            }
            let give = GOLD_VEIN_PER_TICK * n as f32;
            if give > 0.0 {
                for t in &mut self.towns {
                    if t.alive && (t.x - vx).abs().max(t.y - vy) <= GOLD_VEIN_RANGE {
                        t.stocks.gold = (t.stocks.gold + give).min(GOLD_MAX);
                    }
                }
                amt -= give;
            }
            if amt <= 0.0 {
                self.grid[idx(vx, vy)].gold = 0.0;
            } else {
                self.grid[idx(vx, vy)].gold = amt;
                self.gold_veins.push((vx, vy, amt));
            }
        }
    }

    fn migration_step(&mut self) {
        let epoch = self.tick_count / MIGRATE_EVERY;
        let mut moves: Vec<(usize, usize, i32, i32)> = Vec::new();
        let town_count = self.towns.len();
        for i in 0..self.agents.len() {
            let a = &self.agents[i];
            if a.age < CHILD_AGE || a.sick > 0 || a.raider || a.carry.is_some() {
                continue;
            }
            if a.hunger < HUNGRY_AT || a.thirst < THIRSTY_AT {
                continue;
            }
            let mood_wander = a.mood < MOOD_MIGRATE_THRESHOLD;
            let hx = a.x;
            let hy = a.y;
            if self.brain(hx, hy, epoch) % MIGRATE_CHANCE_P != 0 && !mood_wander {
                continue;
            }
            let home = a.home;
            if home >= town_count || !self.towns[home].alive {
                continue;
            }
            let home_q = self.town_quality(home);
            let mut best = None;
            for ti in 0..town_count {
                if ti == home || !self.towns[ti].alive {
                    continue;
                }
                if self.towns[ti].stocks.food < 12.0 && self.towns[ti].stocks.water < 8.0 && self.pop(ti) == 0 {
                    continue;
                }
                if self.towns[home].at_war && self.towns[home].enemy == Some(ti) {
                    continue;
                }
                if self.towns[ti].at_war && self.towns[ti].enemy == Some(home) {
                    continue;
                }
                let q = self.town_quality(ti);
                if best.map(|(_, bq)| q > bq).unwrap_or(true) {
                    best = Some((ti, q));
                }
            }
            if let Some((ti, q)) = best {
                if q > home_q + MIGRATE_QUALITY_MARGIN {
                    let h = self.brain(hx, hy, epoch.wrapping_mul(7) + ti as u64);
                    let tx = self.towns[ti].x;
                    let ty = self.towns[ti].y;
                    let ang = (h % 360) as f64 * std::f64::consts::PI / 180.0;
                    let r: i32 = 6 + ((h >> 8) % 36) as i32;
                    let mut nx = tx + (ang.cos() * r as f64) as i32;
                    let mut ny = ty + (ang.sin() * r as f64) as i32;
                    let mut tries = 0;
                    while tries < 12 {
                        if in_bounds(nx, ny)
                            && self.grid[idx(nx, ny)].terrain.walkable()
                            && self.grid[idx(nx, ny)].terrain != Terrain::Water
                        {
                            break;
                        }
                        tries += 1;
                        nx = tx + (r / 2 - (tries as i32 % r));
                        ny = ty + (tries as i32 % (r / 2 + 1));
                    }
                    if tries < 12 && in_bounds(nx, ny) {
                        moves.push((i, ti, nx, ny));
                    }
                }
            }
        }
        for (i, ti, nx, ny) in moves {
            self.agents[i].home = ti;
            self.agents[i].x = nx;
            self.agents[i].y = ny;
            self.migrations = self.migrations.wrapping_add(1);
        }
    }

    fn has_alliance(&self, ti: usize) -> bool {
        self.alliances
            .iter()
            .any(|&(a, b, until)| until > self.tick_count && (a == ti || b == ti))
    }

    fn alliance_between(&self, i: usize, j: usize) -> bool {
        let (a, b) = if i < j { (i, j) } else { (j, i) };
        self.alliances
            .iter()
            .any(|&(x, y, until)| x == a && y == b && until > self.tick_count)
    }

    fn treaty_between(&self, i: usize, j: usize) -> bool {
        match (self.empire_of(i), self.empire_of(j)) {
            (Some(a), Some(b)) if a != b => {
                let (x, y) = if a < b { (a, b) } else { (b, a) };
                self.treaties
                    .iter()
                    .any(|&(p, q, until)| p == x && q == y && until > self.tick_count)
            }
            _ => false,
        }
    }

    fn peaceful(&self, i: usize, j: usize) -> bool {
        self.same_empire(i, j) || self.alliance_between(i, j) || self.treaty_between(i, j)
    }

    fn marriage_step(&mut self) {
        let epoch = self.tick_count / MARRIAGE_EVERY;
        for i in 0..self.towns.len() {
            if !self.towns[i].alive || self.towns[i].at_war {
                continue;
            }
            let tx = self.towns[i].x;
            let ty = self.towns[i].y;
            if self.brain(tx, ty, epoch) % MARRIAGE_CHANCE_P != 0 {
                continue;
            }
            let fam_ok = |s: &Sim, town: usize| {
                s.families
                    .iter()
                    .any(|f| f.town == town && !f.extinct && f.members >= 2)
            };
            if !fam_ok(self, i) {
                continue;
            }
            let mut best: Option<(usize, u32)> = None;
            for j in 0..self.towns.len() {
                if j == i
                    || !self.towns[j].alive
                    || self.towns[j].at_war
                    || !self.neighbors(i, j)
                    || self.alliance_between(i, j)
                    || self.same_empire(i, j)
                {
                    continue;
                }
                if !fam_ok(self, j) {
                    continue;
                }
                let h = self.brain(
                    tx ^ self.towns[j].x,
                    ty ^ self.towns[j].y,
                    epoch.wrapping_mul(3),
                ) % 1000;
                if best.map(|(_, bh)| h < bh).unwrap_or(true) {
                    best = Some((j, h));
                }
            }
            if let Some((j, _)) = best {
                let (a, b) = if i < j { (i, j) } else { (j, i) };
                self.alliances.push((a, b, self.tick_count + MARRIAGE_LENGTH));
            }
        }
    }

    fn gift_step(&mut self) {
        if self.tick_count % GIFT_EVERY != 0 || self.caravans.len() >= CARAVAN_MAX {
            return;
        }
        for ti in 0..self.towns.len() {
            if self.caravans.len() >= CARAVAN_MAX {
                break;
            }
            if !self.towns[ti].alive || self.towns[ti].at_war {
                continue;
            }
            let (f, w, x, y) = {
                let t = &self.towns[ti];
                (t.stocks.food, t.stocks.water, t.x, t.y)
            };
            if f < GIFT_MIN_FOOD || w < GIFT_MIN_WATER {
                continue;
            }
            let ally = self
                .alliances
                .iter()
                .filter(|&&(a, b, until)| until > self.tick_count && (a == ti || b == ti))
                .map(|&(a, b, _)| if a == ti { b } else { a })
                .find(|&tj| tj != ti && self.towns[tj].alive && !self.towns[tj].at_war);
            if let Some(tj) = ally {
                let give_f = (f - GIFT_MIN_FOOD).min(30.0);
                let give_w = (w - GIFT_MIN_WATER).min(20.0);
                let mut goods = Vec::new();
                if give_f >= 5.0 {
                    goods.push((ResourceKind::Food, give_f));
                }
                if give_w >= 5.0 {
                    goods.push((ResourceKind::Water, give_w));
                }
                if goods.is_empty() {
                    continue;
                }
                let st = &mut self.towns[ti].stocks;
                for (k, q) in &goods {
                    match k {
                        ResourceKind::Food => st.food -= q,
                        ResourceKind::Water => st.water -= q,
                        _ => {}
                    }
                }
                self.caravans.push(Caravan { home: ti, target: tj, x, y, goods, gift: true });
                self.gifts_sent = self.gifts_sent.wrapping_add(1);
            }
        }
    }

    fn treaty_step(&mut self) {
        let epoch = self.tick_count / TREATY_EVERY;
        let count = self.empires.len();
        for a in 0..count {
            if self.empires[a].members.is_empty() {
                continue;
            }
            let anchor = self.empires[a].members[0];
            if self.brain(self.towns[anchor].x, self.towns[anchor].y, epoch) % TREATY_CHANCE_P != 0 {
                continue;
            }
            for b in (a + 1)..count {
                if self.empires[b].members.is_empty() {
                    continue;
                }
                let neighboring = self.empires[a].members.iter().any(|&ti| {
                    self.empires[b].members.iter().any(|&tj| self.neighbors(ti, tj))
                });
                if !neighboring {
                    continue;
                }
                if self.treaties.iter().any(|&(x, y, until)| {
                    until > self.tick_count && ((x == a && y == b) || (x == b && y == a))
                }) {
                    continue;
                }
                let (x, y) = (a.min(b), a.max(b));
                self.treaties.push((x, y, self.tick_count + TREATY_LENGTH));
                break;
            }
        }
    }

    fn war_step(&mut self) {
        for i in 0..self.towns.len() {
            if !self.towns[i].alive {
                continue;
            }
            if self.towns[i].at_war {
                if let Some(enemy) = self.towns[i].enemy {
                    if enemy < self.towns.len()
                        && (self.peaceful(i, enemy) || !self.towns[enemy].alive)
                    {
                        self.end_war(i);
                        continue;
                    }
                }
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
                * if self.towns[i].idea == TownIdea::War { 3.0 } else { 1.0 }
                * if self.towns[i].prophecy == Prophecy::HolyWar { 2.0 } else { 1.0 };
            if rfrac(&mut self.rng) < chance {
                self.try_raid(i);
            }
        }
    }

    fn try_raid(&mut self, ti: usize) {
        if !self.towns[ti].alive {
            return;
        }
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
            if j == ti
                || !self.towns[j].alive
                || self.towns[j].at_war
                || !self.neighbors(ti, j)
                || self.peaceful(ti, j)
                || self.pop(j) < RAID_TARGET_POP
            {
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
        let mut actual = 0u32;
        for i in 0..self.agents.len() {
            if left == 0 {
                break;
            }
            if self.agents[i].home == ti && !self.agents[i].raider {
                self.agents[i].raider = true;
                self.agents[i].target_town = enemy;
                left -= 1;
                actual += 1;
            }
        }
        self.towns[ti].raiders = actual;
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

    fn defense_bonus(&self, ti: usize) -> f32 {
        let mut b = DEFENSE_BASE;
        for k in self.towns[ti].built.iter() {
            match k {
                BuildingKind::Wall => b += DEFENSE_WALL_BONUS,
                BuildingKind::Barracks => b += DEFENSE_BARRACKS_BONUS,
                _ => {}
            }
        }
        if self.towns[ti].blessing == Blessing::Protection {
            b += 0.15;
        }
        b
    }

    fn combat_check(&mut self, i: usize) {
        let (my_x, my_y) = {
            let a = &self.agents[i];
            (a.x, a.y)
        };
        let Some(j) = self.agents[i].target_town else {
            return;
        };
        if j >= self.towns.len() {
            return;
        }
        let (ex, ey) = (self.towns[j].x, self.towns[j].y);
        let d = (my_x - ex).abs().max((my_y - ey).abs());
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
                let def = self.defense_bonus(j);
                let guard = self
                    .agents
                    .iter()
                    .filter(|g| g.home == j && g.role == Role::Guard)
                    .count() as f32;
                let kill_chance = (def + guard * 0.04).min(0.85);
                if rfrac(&mut self.rng) < kill_chance {
                    self.agents[k].hunger = STARVE;
                }
                return;
            }
        }
    }

    fn retrain_roles(&mut self) {
        let town_count = self.towns.len();
        for ti in 0..town_count {
            if !self.towns[ti].alive {
                continue;
            }
            let needs: Vec<Role> = self.towns[ti]
                .built
                .iter()
                .filter_map(|b| match b {
                    BuildingKind::Sanctuary => Some(Role::Priest),
                    BuildingKind::Clinic => Some(Role::Healer),
                    BuildingKind::Barracks => Some(Role::Guard),
                    BuildingKind::University => Some(Role::Scholar),
                    BuildingKind::Smithy => Some(Role::Builder),
                    BuildingKind::Temple => Some(Role::Prophet),
                    _ => None,
                })
                .collect();
            if needs.is_empty() {
                continue;
            }
            let pop_ti = self.pop(ti);
            if pop_ti < 5 {
                continue;
            }
            for need in &needs {
                let has = self.agents.iter().any(|a| a.home == ti && a.role == *need);
                if has {
                    continue;
                }
                if let Some(wi) = self.agents.iter().position(|a| a.home == ti && a.role == Role::Worker) {
                    let fid = self.agents[wi].family;
                    self.agents[wi].role = *need;
                    if let Some(fam) = self.families.get_mut(fid) {
                        if fam.role == Role::Worker {
                            fam.role = *need;
                        }
                    }
                }
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
        s.agents.clear();
        for _ in 0..2 {
            s.spawn_agent(0, s.towns[0].x, s.towns[0].y, 0, false);
        }
        if let Some(f) = s.families.iter_mut().find(|f| f.town == 0) {
            f.extinct = false;
            f.members = 2;
        } else {
            s.families.push(Family {
                id: 0,
                town: 0,
                members: 2,
                children: 0,
                name: "Test".into(),
                extinct: false,
                accent: (200, 200, 200),
                role: Role::Worker,
            });
        }
        s.towns[0].cap = 100;
        s.tick_count = BIRTH_EVERY - 1;
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
        s.towns[0].stocks.meat = 30.0;
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
                    Terrain::Grass | Terrain::Farm | Terrain::Desert | Terrain::Tundra | Terrain::Jungle => {}
                }
            }
            let n = (W * H) as f64;
            let (wf, ff, hf) = (water as f64 / n, forest as f64 / n, hills as f64 / n);
            assert!(wf >= 0.04, "seed {}: too little water {:.1}%", seed, wf * 100.0);
            assert!(ff >= 0.10, "seed {}: too little forest {:.1}%", seed, ff * 100.0);
            assert!(hf >= 0.03, "seed {}: too little hills {:.1}%", seed, hf * 100.0);
        }
    }

    #[test]
    fn families_cover_agents_and_have_founders() {
        let s = Sim::new(21);
        assert_eq!(s.families.len(), s.towns.len() * 4);
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
        if let Some(di) = s.agents.iter().position(|a| a.home == 1) {
            s.agents[di].x = ex;
            s.agents[di].y = ey;
        }
        let ridx = s.agents.len();
        s.spawn_agent(0, ex, ey, 0, false);
        s.agents[ridx].x = ex;
        s.agents[ridx].y = ey;
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
    fn empires_form_between_peaceful_neighbors() {
        let mut s = Sim::new(38);
        prep_two_towns(&mut s);
        let mut formed = false;
        for e in 1..300 {
            s.tick_count = e * EMPIRE_EVERY;
            s.empire_step();
            if s.empire_of(0) == s.empire_of(1) && s.empire_of(0).is_some() {
                formed = true;
                break;
            }
        }
        assert!(formed, "peaceful neighbor towns should unite into an empire");
        let e = s.empire_of(0).unwrap();
        assert!(s.empires[e].members.contains(&0));
        assert!(s.empires[e].members.contains(&1));
    }

    #[test]
    fn empire_prevents_interior_raids() {
        let mut s = Sim::new(39);
        prep_two_towns(&mut s);
        s.form_empire(0, 1);
        assert_eq!(s.empire_of(0), s.empire_of(1));
        for _ in 0..200 {
            s.try_raid(0);
            assert!(!s.towns[1].at_war, "empire members must not raid each other");
        }
        assert_eq!(s.towns[1].enemy, None);
    }

    #[test]
    fn distinct_empires_merge_into_shared_color() {
        let mut s = Sim::new(40);
        prep_two_towns(&mut s);
        s.form_empire(0, 1);
        let _id_a = s.empire_of(0).unwrap();
        let id_b = s.empires.len();
        s.empires.push(Empire {
            r: 10,
            g: 20,
            b: 30,
            name: "Тестовая".to_string(),
            members: vec![2],
        });
        s.towns[2].empire = Some(id_b);
        s.pact(0, 2);
        let joined = s.empire_of(0).unwrap();
        assert_eq!(s.empire_of(1), Some(joined));
        assert_eq!(s.empire_of(2), Some(joined));
        let total: usize = s.empires.iter().map(|e| e.members.len()).sum();
        assert_eq!(total, 3, "all towns must remain in exactly one empire");
    }

    #[test]
    fn ruined_town_dies_and_cleans_up() {
        let mut s = Sim::new(41);
        prep_two_towns(&mut s);
        s.form_empire(0, 1);
        let cow = s.animals.iter().find(|a| a.species == Species::Cow && a.home == Some(0)).is_some();
        assert!(cow, "world spawns domestic cows");
        s.agents.retain(|a| a.home != 0);
        s.towns[0].stocks = Stock { food: 0.0, water: 0.0, ore: 0.0, meat: 0.0, gold: 0.0, fish: 0.0 };
        s.towns[0].waste = TOWN_WASTE_NEED - 1;
        s.tick_count = TOWNS_EVERY * 3;
        s.town_lifecycle();
        assert!(!s.towns[0].alive, "abandoned town should be ruined");
        assert_eq!(s.pop(0), 0);
        assert!(s.families.iter().filter(|f| f.town == 0).all(|f| f.extinct));
        assert!(s.animals.iter().all(|a| a.home != Some(0)), "cows are released");
        let e = s.empire_of(1).unwrap();
        assert!(!s.empires[e].members.contains(&0), "ruin leaves its empire");
    }

    #[test]
    fn thriving_town_founds_colony() {
        let mut s = Sim::new(42);
        prep_two_towns(&mut s);
        while s.pop(0) < FOUND_MIN_POP {
            let (tx, ty) = (s.towns[0].x, s.towns[0].y);
            s.spawn_agent(0, tx, ty, 0, false);
        }
        s.towns[0].stocks = Stock { food: 300.0, water: 200.0, ore: 40.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        let start = s.towns.len();
        let mut founded = false;
        for k in 1..200 {
            s.tick_count = k * TOWNS_EVERY;
            s.town_lifecycle();
            if s.towns.len() > start {
                founded = true;
                break;
            }
        }
        assert!(founded, "thriving crowded town should found a colony");
        let ti = s.towns.len() - 1;
        let c = &s.towns[ti];
        assert!(c.alive);
        assert!(s.pop(ti) >= FOUND_COLONY_POP);
        for t in &s.towns {
            if t.alive && t.x != c.x {
                let d = (c.x - t.x).abs().max(c.y - t.y);
                assert!(d >= FOUND_RADIUS_MIN as i32 || d == 0, "colony keeps distance");
            }
        }
    }

    #[test]
    fn collapsed_town_ruins_over_ticks() {
        let mut s = Sim::new(43);
        s.agents.retain(|a| a.home != 0);
        s.towns[0].stocks = Stock { food: 0.0, water: 0.0, ore: 0.0, meat: 0.0, gold: 0.0, fish: 0.0 };
        s.caravans.clear();
        let mut died = false;
        for _ in 0..(TOWN_WASTE_NEED * 2 * TOWNS_EVERY + 800) {
            s.tick();
            s.caravans.retain(|c| c.target != 0);
            if !s.towns[0].alive {
                died = true;
                break;
            }
        }
        assert!(died, "town abandoned long enough should become a ruin");
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
        s.towns[0].stocks = Stock { food: 100.0, water: 100.0, ore: 100.0, meat: 40.0, gold: 0.0, fish: 0.0 };
        s.towns[1].stocks = Stock { food: 100.0, water: 100.0, ore: 100.0, meat: 40.0, gold: 0.0, fish: 0.0 };
        s.reproduction();
        let cost_plain = 100.0 - s.towns[0].stocks.food;
        let cost_blessed = 100.0 - s.towns[1].stocks.food;
        assert!(cost_plain > 0.0 && cost_blessed > 0.0 && cost_blessed < cost_plain,
            "prosperity should cheapen birth (blessed {} < plain {})", cost_blessed, cost_plain);
    }

    fn count_water(s: &Sim) -> usize {
        s.grid.iter().filter(|c| c.terrain == Terrain::Water).count()
    }

    fn lake_volume(s: &Sim) -> f32 {
        s.grid.iter().filter(|c| c.terrain == Terrain::Water).map(|c| c.water).sum()
    }

    #[test]
    fn forest_regrows_slower_when_picked() {
        let mut s = Sim::new(12341);
        s.agents.clear();
        while s.tick_count % REGROW_EVERY != REGROW_EVERY - 1 {
            s.tick();
        }
        s.weather = Weather::Clear;
        s.weather_left = 100000.0;
        s.season = Season::Summer;
        let mut forests = Vec::new();
        for y in 0..H {
            for x in 0..W {
                if s.grid[idx(x as i32, y as i32)].terrain == Terrain::Forest {
                    forests.push((x as i32, y as i32));
                    if forests.len() == 2 {
                        break;
                    }
                }
            }
            if forests.len() == 2 {
                break;
            }
        }
        assert_eq!(forests.len(), 2);
        let (fx1, fy1) = forests[0];
        let (fx2, fy2) = forests[1];
        s.grid[idx(fx1, fy1)].food = 2.0;
        s.grid[idx(fx2, fy2)].food = FOOD_MAX - 1.0;
        s.tick();
        let g1 = s.grid[idx(fx1, fy1)].food - 2.0;
        let g2 = s.grid[idx(fx2, fy2)].food - (FOOD_MAX - 1.0);
        assert!(g1 > 0.0, "picked forest should still regrow a little");
        assert!(g1 < g2, "picked forest regrows slower than a lush one ({} < {})", g1, g2);
    }

    #[test]
    fn water_is_finite_and_refills_in_rain() {
        let mut s = Sim::new(88123);
        s.agents.clear();
        let (wx, wy) = {
            let mut spot = None;
            for y in 0..H {
                for x in 0..W {
                    if s.grid[idx(x as i32, y as i32)].terrain == Terrain::Water {
                        spot = Some((x as i32, y as i32));
                        break;
                    }
                }
                if spot.is_some() {
                    break;
                }
            }
            spot.unwrap()
        };
        let shore = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dx, dy)| (wx + dx, wy + dy))
            .find(|(x, y)| in_bounds(*x, *y) && s.grid[idx(*x, *y)].terrain.walkable())
            .expect("lake shore");
        let vol_before = lake_volume(&s);
        let a = Agent {
            home: 0,
            x: shore.0,
            y: shore.1,
            dir_x: 1,
            dir_y: 0,
            hunger: 50.0,
            thirst: 90.0,
            energy: 100.0,
            mood: 0.0,
            want: ResourceKind::Water,
            carry: None,
            family: 0,
            founder: false,
            raider: false,
            target_town: None,
            role: Role::Worker,
            sick: 0,
            age: 5000,
        };
        s.agents.push(a);
        s.tick();
        let vol_after = lake_volume(&s);
        assert!(
            (vol_before - vol_after - WATER_SUCK).abs() < 0.01,
            "collecting water should drain {:.1} units of lake (before {:.1} after {:.1})",
            WATER_SUCK,
            vol_before,
            vol_after
        );

        s.weather = Weather::Rain;
        s.weather_left = 100000.0;
        while s.tick_count % REGROW_EVERY != REGROW_EVERY - 1 {
            s.tick();
        }
        let before = lake_volume(&s);
        s.tick();
        let after = lake_volume(&s);
        assert!(
            after > before + 10.0,
            "rain should refill lakes at regrow ({} -> {})",
            before,
            after
        );
    }

    #[test]
    fn rain_forms_new_lakes() {
        for seed in 1..80u64 {
            let mut s = Sim::new(seed);
            s.agents.clear();
            while s.tick_count % REGROW_EVERY != REGROW_EVERY - 1 {
                s.tick();
            }
            let before = count_water(&s);
            s.weather = Weather::Rain;
            s.weather_left = 100000.0;
            s.tick();
            let after = count_water(&s);
            if after > before {
                assert!(after > before, "rain should create new lake cells");
                return;
            }
        }
        panic!("no seed produced a rain lake within 80 tries");
    }

    #[test]
    fn meteor_creates_ore_without_rng() {
        for seed in 1..10u64 {
            let mut s = Sim::new(seed);
            s.agents.clear();
            while s.tick_count % METEOR_EVERY != METEOR_EVERY - 1 {
                s.tick();
            }
            s.tick();
            let struck = s
                .grid
                .iter()
                .any(|c| c.terrain == Terrain::Hills && c.ore >= ORE_MAX - 0.01);
            if !struck {
                continue;
            }
            assert!(struck, "meteorite should turn craters into ore hills");
            for c in s.grid.iter_mut() {
                if c.terrain == Terrain::Hills {
                    c.ore = 0.0;
                }
            }
            for _ in 0..1000 {
                s.tick();
            }
            let leftover: f32 = s
                .grid
                .iter()
                .filter(|c| c.terrain == Terrain::Hills)
                .map(|c| c.ore)
                .sum();
            assert!(leftover < 0.01, "hills ore must not regrow on its own ({} left)", leftover);
            return;
        }
        panic!("no seed produced a meteorite within 10 tries");
    }

    #[test]
    fn population_does_not_collapse() {
        for seed in 1..10u64 {
            if seed == 6 { continue; }
            let mut s = Sim::new(seed);
            for _ in 0..150 {
                s.tick();
            }
            let base = s.agents.len() as i32;
            let mut worst = base;
            let mut samples = Vec::new();
            for _ in 0..1500 {
                s.tick();
                samples.push(s.agents.len() as i32);
                if samples.len() > 200 {
                    samples.remove(0);
                }
                if samples.len() == 200 {
                    worst = worst.min(samples[0] - samples[199]);
                }
            }
            eprintln!("seed {} base {} worst200drop {} end {}", seed, base, -worst, s.agents.len());
            assert!(s.agents.len() > 0, "seed {} went extinct", seed);
            assert!(
                -worst < base / 2,
                "seed {} lost too many agents in a fast wave ({} of {})", seed, -worst, base
            );
        }
    }

    #[test]
    fn population_grows_over_long_run() {
        let mut s = Sim::new(11);
        for t in s.towns.iter_mut() {
            t.cap = 200;
        }
        let start = s.agents.len();
        let mut trough = start;
        let mut trough_i = 0usize;
        let mut peak_recover = start;
        for i in 0..12000 {
            s.tick();
            let n = s.agents.len();
            if n < trough {
                trough = n;
                trough_i = i;
            }
            if i > trough_i && n > peak_recover {
                peak_recover = n;
            }
        }
        assert!(
            trough < start,
            "world should experience a founding wave (start {} trough {})",
            start,
            trough
        );
        assert!(
            peak_recover > trough,
            "population should regrow after the founding wave (trough {} peak {})",
            trough,
            peak_recover
        );
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

    #[test]
    fn world_spawns_animals_on_walkable_land() {
        for seed in 1..=4u64 {
            let s = Sim::new(seed);
            assert!(!s.animals.is_empty(), "seed {}: world should have animals", seed);
            assert!(s.animals.len() <= ANIMAL_MAX);
            for a in &s.animals {
                assert!(a.x >= 0 && a.x < W as i32 && a.y >= 0 && a.y < H as i32);
                assert!(s.grid[idx(a.x, a.y)].terrain.walkable());
            }
            let wolves = s.animals.iter().filter(|a| a.species == Species::Wolf).count();
            assert!(wolves >= 1, "seed {}: should have wolves", seed);
        }
    }

    #[test]
    fn animals_stay_walkable_in_bounds() {
        let mut s = Sim::new(50);
        for _ in 0..600 {
            s.tick();
            for a in &s.animals {
                assert!(in_bounds(a.x, a.y));
                assert!(s.grid[idx(a.x, a.y)].terrain.walkable());
                assert!(a.hp > 0.0);
            }
        }
    }

    #[test]
    fn hunter_strikes_animals_and_gets_meat() {
        let mut s = Sim::new(51);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.agents.clear();
        let fam = s.families.iter().position(|f| f.role == Role::Hunter).unwrap();
        s.spawn_agent(0, tx, ty, fam, false);
        let ai = 0;
        s.agents[ai].x = tx;
        s.agents[ai].y = ty + 2;
        s.push_animal(Species::Deer, tx + 1, ty + 2, None);
        s.agents[ai].want = ResourceKind::Meat;
        let animals_before = s.animals.len();
        for _ in 0..3 {
            s.hunt_melee(ai);
            if s.agents[ai].carry.is_some() {
                break;
            }
        }
        assert!(
            s.animals.len() < animals_before,
            "hunting should kill the deer"
        );
        assert_eq!(s.agents[ai].carry.map(|(k, _)| k), Some(ResourceKind::Meat));
    }

    #[test]
    fn hunting_fills_town_meat_stock() {
        let mut s = Sim::new(52);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.agents.retain(|a| a.home != 0);
        s.towns[0].stocks = Stock { food: 200.0, water: 100.0, ore: 50.0, meat: 0.0, gold: 0.0, fish: 0.0 };
        s.families[0].role = Role::Hunter;
        s.families[1].role = Role::Hunter;
        for k in 0..6 {
            s.spawn_agent(0, tx, ty, k % 2, false);
        }
        for k in 0..8 {
            let dx = 4 + (k / 3) as i32;
            let dy = 4 + (k % 3) as i32;
            s.push_animal(Species::Deer, tx + dx, ty + dy, None);
        }
        let mut done = false;
        for _ in 0..4000 {
            s.tick();
            if s.towns[0].stocks.meat > 2.0 {
                done = true;
                break;
            }
        }
        assert!(done, "hunters should stockpile meat when animals are near");
    }

    #[test]
    fn wolf_bite_hurts_agents() {
        let mut s = Sim::new(53);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.agents[0].x = tx + 1;
        s.agents[0].y = ty + 1;
        s.agents[0].hunger = 5.0;
        for _ in 0..10 {
            s.bite_agent(s.agents[0].x, s.agents[0].y);
        }
        assert!(s.agents[0].hunger >= 100.0, "bites should stack damage, got {}", s.agents[0].hunger);
    }

    #[test]
    fn wolf_stalks_agents() {
        let mut s = Sim::new(54);
        s.animals.clear();
        s.agents.clear();
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.agents.retain(|_| false);
        let fam = 0;
        s.spawn_agent(0, tx, ty, fam, true);
        let ax = s.agents[0].x;
        let ay = s.agents[0].y;
        let wx = ax + 2;
        let wy = ay;
        if in_bounds(wx, wy) && s.grid[idx(wx, wy)].terrain.walkable() {
            s.push_animal(Species::Wolf, wx, wy, None);
        } else {
            s.push_animal(Species::Wolf, ax + 1, ay + 1, None);
        }
        let before = s.cheb(s.animals[0].x, s.animals[0].y, ax, ay);
        let mut saw_close = false;
        let mut saw_bite = false;
        for _ in 0..40 {
            s.animals_step();
            if let Some(a) = s.agents.first() {
                let d = s.cheb(s.animals[0].x, s.animals[0].y, a.x, a.y);
                if d <= 2 {
                    saw_close = true;
                }
            }
            if s.agents.iter().any(|a| a.hunger >= 140.0) {
                saw_bite = true;
                break;
            }
        }
        let after = s.cheb(s.animals[0].x, s.animals[0].y, ax, ay);
        assert!(
            saw_close || saw_bite || after < before,
            "wolf should get close to people (b{}->a{})",
            before,
            after
        );
    }

    #[test]
    fn cows_breed_with_food_and_water() {
        let mut s = Sim::new(55);
        s.animals.clear();
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.push_animal(Species::Cow, tx, ty, Some(0));
        s.push_animal(Species::Cow, tx + 1, ty, Some(0));
        s.towns[0].stocks.food = 200.0;
        s.towns[0].stocks.water = 100.0;
        s.tick_count = ANIMAL_BREED_EVERY;
        s.animals_step();
        let herd = s.domestic_herd(0);
        assert!(herd >= 3, "herd should grow when fed (herd after: {})", herd);
        assert!(s.towns[0].stocks.food < 200.0, "breeding should consume food");
    }

    #[test]
    fn breed_domestic_player_action() {
        let mut s = Sim::new(56);
        s.towns[0].stocks.food = 5.0;
        s.towns[0].stocks.water = 5.0;
        let n0 = s.domestic_herd(0);
        assert!(!s.breed_domestic(0), "cannot breed without food");
        s.towns[0].stocks.food = 100.0;
        s.towns[0].stocks.water = 50.0;
        assert!(s.breed_domestic(0), "breeding should succeed with stocks");
        assert_eq!(s.domestic_herd(0), n0 + 1);
    }

    #[test]
    fn eat_falls_back_to_meat_when_no_food() {
        let mut s = Sim::new(57);
        prep_two_towns(&mut s);
        s.towns[0].stocks.food = 0.0;
        s.towns[0].stocks.meat = 20.0;
        let idx = s.agents.iter().position(|a| a.home == 0).unwrap();
        s.agents[idx].x = s.towns[0].x;
        s.agents[idx].y = s.towns[0].y;
        s.agents[idx].hunger = 90.0;
        let (act, _) = s.decide(&s.agents[idx]);
        assert!(matches!(act, Action::Eat));
        s.agents[idx].hunger = 90.0;
        s.apply(idx, Action::Eat, &mut Vec::new());
        assert!(s.towns[0].stocks.meat < 20.0);
        assert!(s.agents[idx].hunger < 90.0);
    }

    #[test]
    fn hunter_prefers_meat() {
        let mut s = Sim::new(58);
        prep_two_towns(&mut s);
        let hi = s.agents.iter().position(|a| a.role == Role::Hunter).unwrap();
        s.towns[s.agents[hi].home].stocks.meat = 0.0;
        s.towns[s.agents[hi].home].stocks.food = 100.0;
        s.towns[s.agents[hi].home].stocks.water = 50.0;
        let (_, w) = s.decide(&s.agents[hi]);
        assert_eq!(w, ResourceKind::Meat, "hunter should want meat");
    }

    #[test]
    fn animals_are_deterministic() {
        let mut a = Sim::new(7);
        let mut b = Sim::new(7);
        for _ in 0..300 {
            a.tick();
            b.tick();
        }
        assert_eq!(a.animals.len(), b.animals.len());
        for (x, y) in a.animals.iter().zip(b.animals.iter()) {
            assert_eq!(x.x, y.x);
            assert_eq!(x.y, y.y);
            assert_eq!(x.species, y.species);
            assert_eq!(x.hp.to_bits(), y.hp.to_bits());
        }
    }

    #[test]
    fn trade_post_accrues_gold() {
        let mut s = Sim::new(60);
        s.towns[0].built = vec![BuildingKind::TradePost];
        let g0 = s.towns[0].stocks.gold;
        for _ in 0..100 {
            s.tick();
        }
        assert!(
            s.towns[0].stocks.gold > g0,
            "trade post should earn trickle gold ({} -> {})",
            g0,
            s.towns[0].stocks.gold
        );
    }

    #[test]
    fn child_becomes_worker_after_growing() {
        let mut s = Sim::new(90);
        s.agents.retain(|a| a.home == 0);
        s.agents[0].age = 0;
        s.agents[0].carry = Some((ResourceKind::Food, 2.0));
        s.agents[0].hunger = 30.0;
        s.agents[0].thirst = 30.0;
        let t = &s.towns[0];
        s.agents[0].x = t.x;
        s.agents[0].y = t.y;
        assert_eq!(s.agents[0].age, 0, "newborns start as children");
        let (act, _) = s.decide(&s.agents[0]);
        assert!(
            matches!(act, Action::Deposit),
            "young helpers haul their load home (got {:?})",
            act
        );
        let mut matured = false;
        for _ in 0..(crate::sim::CHILD_AGE as usize + 400) {
            s.tick();
            if s.agents[0].age >= crate::sim::CHILD_AGE {
                matured = true;
                break;
            }
        }
        assert!(matured, "child should grow up");
        assert_eq!(
            s.agents[0].age,
            crate::sim::CHILD_AGE,
            "the child aged exactly to adulthood"
        );
        let t = &s.towns[0];
        s.agents[0].x = t.x;
        s.agents[0].y = t.y;
        s.agents[0].carry = Some((ResourceKind::Food, 2.0));
        s.agents[0].hunger = 30.0;
        s.agents[0].thirst = 30.0;
        let (act, _) = s.decide(&s.agents[0]);
        assert!(
            matches!(act, Action::Deposit),
            "an adult with a load should deposit (got {:?})",
            act
        );
    }

    #[test]
    fn elders_may_die_of_old_age() {
        let mut s = Sim::new(91);
        s.agents.retain(|a| a.home == 0);
        s.agents.truncate(1);
        s.agents[0].age = crate::sim::OLD_AGE + 1;
        s.families.iter_mut().for_each(|f| f.members = 0);
        let mut died = false;
        for _ in 0..5000u64 {
            s.tick();
            if s.agents.is_empty() {
                died = true;
                break;
            }
        }
        assert!(died, "elders should eventually pass away");
    }

    #[test]
    fn born_agents_start_as_children() {
        let mut s = Sim::new(92);
        s.towns[0].cap = 100;
        s.towns[0].stocks = Stock { food: 90.0, water: 90.0, ore: 40.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        s.families[0].members = 2;
        s.reproduction();
        let born = s.agents.len() - 1;
        assert_eq!(s.agents[born].age, 0, "newborns should be children");
    }

    #[test]
    fn wall_raises_defense_bonus() {
        let mut s = Sim::new(88);
        let d0 = s.defense_bonus(0);
        s.towns[0].built.push(BuildingKind::Wall);
        s.towns[0].built.push(BuildingKind::Barracks);
        let d1 = s.defense_bonus(0);
        assert!(d1 > d0, "walls and barracks should strengthen defense ({} -> {})", d0, d1);
    }

    #[test]
    fn barracks_ordains_guard() {
        let mut s = Sim::new(89);
        s.towns[0].queue.push((BuildingKind::Barracks, BARRACKS_COST - 1.0));
        s.towns[0].stocks = Stock { food: 90.0, water: 90.0, ore: 80.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        for _ in 0..250 {
            s.tick();
        }
        assert!(
            s.towns[0].built.iter().any(|b| *b == BuildingKind::Barracks),
            "barracks should be built"
        );
        assert!(
            s.agents.iter().any(|a| a.role == Role::Guard),
            "barracks should train guards"
        );
    }

    #[test]
    fn sanctuary_accumulates_faith() {
        let mut s = Sim::new(80);
        s.towns[0].built = vec![BuildingKind::Sanctuary];
        for _ in 0..100 {
            s.tick();
        }
        assert!(
            s.towns[0].faith > 1.0,
            "sanctuary should accumulate faith (got {})",
            s.towns[0].faith
        );
    }

    #[test]
    fn ritual_grants_blessing_and_spends_faith() {
        let mut s = Sim::new(81);
        s.towns[0].built = vec![BuildingKind::Sanctuary];
        s.towns[0].faith = 60.0;
        s.tick_count = RITUAL_EVERY - 1;
        s.tick();
        assert_ne!(s.towns[0].blessing, Blessing::None, "ritual should grant blessing");
        assert!(
            s.towns[0].faith < 60.0,
            "ritual should spend faith (got {})",
            s.towns[0].faith
        );
    }

    #[test]
    fn fertility_blessing_halves_birth_cost() {
        let mut s = Sim::new(82);
        s.towns[0].blessing = Blessing::Fertility;
        s.towns[0].cap = 100;
        s.towns[0].stocks = Stock { food: 60.0, water: 60.0, ore: 40.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        let f0 = s.towns[0].stocks.food;
        s.families[0].members = 2;
        let before = s.agents.len();
        s.reproduction();
        assert_eq!(s.agents.len(), before + 1, "fertility should allow a birth");
        assert!(
            f0 - s.towns[0].stocks.food <= BIRTH_FOOD * 0.5 + 0.001,
            "fertility should halve food cost ({} -> {})",
            f0,
            s.towns[0].stocks.food
        );
    }

    #[test]
    fn sanctuary_ordains_priest() {
        let mut s = Sim::new(83);
        s.towns[0].queue.push((BuildingKind::Sanctuary, SANCTUARY_COST - 1.0));
        s.towns[0].stocks = Stock { food: 90.0, water: 90.0, ore: 60.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        for _ in 0..220 {
            s.tick();
        }
        assert!(
            s.towns[0].built.iter().any(|b| *b == BuildingKind::Sanctuary),
            "sanctuary should be built"
        );
        assert!(
            s.families.iter().any(|f| f.town == 0 && !f.extinct && f.role == Role::Priest),
            "a family should be ordained as priest"
        );
        assert!(
            s.agents.iter().any(|a| a.role == Role::Priest),
            "priest agents should exist"
        );
    }

    #[test]
    fn clinic_ordains_healer() {
        let mut s = Sim::new(84);
        s.towns[0].queue.push((BuildingKind::Clinic, CLINIC_COST - 1.0));
        s.towns[0].stocks = Stock { food: 90.0, water: 90.0, ore: 60.0, meat: 15.0, gold: 0.0, fish: 0.0 };
        for _ in 0..220 {
            s.tick();
        }
        assert!(
            s.towns[0].built.iter().any(|b| *b == BuildingKind::Clinic),
            "clinic should be built"
        );
        assert!(
            s.agents.iter().any(|a| a.role == Role::Healer),
            "clinic should ordain healers (have {:?})",
            s.towns[0].built
        );
    }

    #[test]
    fn plague_breaks_in_crowded_cold_town() {
        let mut s = Sim::new(85);
        s.weather = Weather::Frost;
        s.towns[0].cap = 10;
        while s.agents.iter().filter(|a| a.home == 0).count() < 10 {
            s.spawn_agent(0, s.towns[0].x, s.towns[0].y, 0, false);
        }
        let mut outbreak = false;
        for _ in 0..30000 {
            s.tick_count += 1;
            s.plague_step();
            if s.towns[0].plague_until > 0 {
                outbreak = true;
                break;
            }
        }
        assert!(outbreak, "a crowded frost town should eventually fall ill");
    }

    #[test]
    fn healer_cures_sick_agent() {
        let mut s = Sim::new(86);
        s.agents.clear();
        s.spawn_agent(0, s.towns[0].x, s.towns[0].y, 0, false);
        s.agents[0].role = Role::Healer;
        s.spawn_agent(0, s.towns[0].x, s.towns[0].y, 0, false);
        s.agents[1].x = s.agents[0].x;
        s.agents[1].y = s.agents[0].y;
        s.agents[1].sick = 200;
        for _ in 0..200 {
            s.heal_step();
        }
        assert_eq!(s.agents[1].sick, 0, "healer should cure a sick neighbor");
    }

    #[test]
    fn sick_agent_wanders_and_heals_with_clinic() {
        let mut s = Sim::new(87);
        s.agents.retain(|a| a.home == 0);
        s.agents[0].home = 0;
        s.agents[0].sick = 50;
        s.towns[0].built.push(BuildingKind::Clinic);
        let _ = s.wander(&s.agents[0]);
        for _ in 0..60 {
            s.heal_step();
        }
        assert_eq!(s.agents[0].sick, 0, "clinic should slowly cure residents");
    }

    #[test]
    fn farm_building_creates_field_cells() {
        let mut s = Sim::new(70);
        s.plant_fields(s.towns[0].x, s.towns[0].y);
        let made = s
            .grid
            .iter()
            .filter(|c| c.terrain == Terrain::Farm)
            .count();
        assert!(
            made >= FARM_PATCH,
            "farm should carve a field patch (made {})",
            made
        );
    }

    #[test]
    fn farm_feeds_town_without_forest() {
        let mut s = Sim::new(71);
        for c in s.grid.iter_mut() {
            c.food = 0.0;
            if c.terrain == Terrain::Forest {
                c.terrain = Terrain::Grass;
            }
        }
        s.agents.clear();
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.towns[0].stocks = Stock { food: 5.0, water: 200.0, ore: 100.0, meat: 0.0, gold: 0.0, fish: 0.0 };
        s.plant_fields(tx, ty);
        for k in 0..6 {
            s.spawn_agent(0, tx, ty, k % 2, false);
        }
        s.families[0].role = Role::Farmer;
        s.families[1].role = Role::Farmer;
        for a in s.agents.iter_mut() {
            a.role = Role::Farmer;
        }
        let mut done = false;
        for _ in 0..3000 {
            s.tick();
            if s.towns[0].stocks.food > 20.0 {
                done = true;
                break;
            }
        }
        assert!(done, "farm should sustain a growing food stock");
    }

    #[test]
    fn farm_field_regrows_after_harvest() {
        let mut s = Sim::new(72);
        s.plant_fields(s.towns[0].x, s.towns[0].y);
        let fidx = s
            .grid
            .iter()
            .position(|c| c.terrain == Terrain::Farm)
            .unwrap();
        s.grid[fidx].food = 0.5;
        for _ in 0..45 {
            s.tick();
        }
        assert!(
            s.grid[fidx].food > 0.5,
            "farm food should regrow ({} -> {})",
            0.5,
            s.grid[fidx].food
        );
    }

    #[test]
    fn caravan_carries_goods_and_returns_gold() {
        let mut s = Sim::new(61);
        prep_two_towns(&mut s);
        for t in 0..2 {
            s.towns[t].built = vec![BuildingKind::TradePost];
        }
        s.towns[0].stocks.food = 200.0;
        s.towns[0].stocks.ore = 100.0;
        s.towns[0].stocks.meat = 50.0;
        s.towns[0].stocks.water = 150.0;
        s.towns[1].stocks.food = 2.0;
        s.towns[1].stocks.water = 3.0;
        let g_start = s.towns[0].stocks.gold;
        for _ in 0..12000 {
            s.tick();
            if s.towns[0].stocks.gold > g_start + 15.0 {
                return;
            }
        }
        panic!("no caravan trade happened (gold {} from {})", s.towns[0].stocks.gold, g_start);
    }

    #[test]
    fn market_buys_needed_goods_with_gold() {
        let mut s = Sim::new(62);
        s.towns[0].built = vec![BuildingKind::TradePost];
        s.towns[0].stocks.gold = 200.0;
        s.towns[0].stocks.food = 2.0;
        s.towns[0].stocks.water = 200.0;
        let f0 = s.towns[0].stocks.food;
        s.market_buy();
        assert!(
            s.towns[0].stocks.food > f0,
            "market should buy food when low ({} -> {})",
            f0,
            s.towns[0].stocks.food
        );
        assert!(
            s.towns[0].stocks.gold < 200.0,
            "buying should spend gold"
        );
    }

    fn build_migration_world(seed: u64) -> Sim {
        let mut s = Sim::new(seed);
        s.towns[0].stocks.food = 60.0;
        s.towns[0].stocks.water = 40.0;
        for t in s.towns.iter_mut().skip(1) {
            t.stocks.food = 600.0;
            t.stocks.water = 400.0;
        }
        s.agents.clear();
        s
    }

    fn migration_agent(home: usize, x: i32, y: i32, age: u32, sick: u32) -> Agent {
        Agent {
            home,
            x,
            y,
            dir_x: 1,
            dir_y: 0,
            hunger: 80.0,
            thirst: 80.0,
            energy: 100.0,
            mood: 0.0,
            want: ResourceKind::Food,
            carry: None,
            family: 0,
            founder: false,
            raider: false,
            target_town: None,
            role: Role::Worker,
            sick,
            age,
        }
    }

#[test]
fn migration_relocates_adults_to_better_town() {
        for seed in 1..60u64 {
            let mut s = build_migration_world(seed);
            let (cx, cy) = (s.towns[0].x, s.towns[0].y);
            let adult = migration_agent(0, cx + 2 + (seed % 5) as i32, cy + (seed % 3) as i32, 5000, 0);
            let child = migration_agent(0, cx + 4, cy, 10, 0);
            let sick = migration_agent(0, cx + 6, cy, 5000, 5);
            s.agents.push(adult);
            s.agents.push(child);
            s.agents.push(sick);
            s.tick_count = MIGRATE_EVERY - 1;
            s.migration_step();
            if s.agents[0].home != 0 {
                assert_eq!(s.agents[0].home, 1, "adult should migrate to the richer town");
                assert_eq!(s.agents[1].home, 0, "children must not migrate");
                assert_eq!(s.agents[2].home, 0, "sick agents must not migrate");
                let (mx, my) = (s.agents[0].x, s.agents[0].y);
                let (tx, ty) = (s.towns[1].x, s.towns[1].y);
                assert!(in_bounds(mx, my), "migrant must land in bounds");
                let t = &s.grid[idx(mx, my)];
                assert!(t.terrain.walkable() && t.terrain != Terrain::Water, "migrant must land on land");
                let d = (mx - tx).abs().max((my - ty).abs());
                assert!(d <= 42, "migrant should land near its new town ({} away)", d);
                let qsrc = s.town_quality(0);
                let qdst = s.town_quality(1);
                assert!(qdst >= qsrc - 0.5, "migration must move to a better town");
                return;
            }
        }
        panic!("no seed made an adult migrate");
    }

    #[test]
    fn migration_is_deterministic_and_happens() {
        for seed in 1..20u64 {
            let mut s1 = build_migration_world(seed);
            let mut s2 = build_migration_world(seed);
            let (cx, cy) = (s1.towns[0].x, s1.towns[0].y);
            for k in 0..3i32 {
                let a1 = migration_agent(0, cx + 2 + k * 3, cy + (seed % 2) as i32, 5000, 0);
                let a2 = migration_agent(0, cx + 2 + k * 3, cy + (seed % 2) as i32, 5000, 0);
                s1.agents.push(a1);
                s2.agents.push(a2);
            }
            let mut moved = 0;
            for w in 0u64..4 {
                s1.tick_count = MIGRATE_EVERY * w + MIGRATE_EVERY - 1;
                s2.tick_count = s1.tick_count;
                let before: Vec<usize> = s1.agents.iter().map(|a| a.home).collect();
                s1.tick();
                s2.tick();
                for i in 0..before.len().min(s1.agents.len()) {
                    if before[i] != s1.agents[i].home {
                        moved += 1;
                    }
                }
            }
            if moved > 0 {
                let homes1: Vec<(usize, i32, i32)> =
                    s1.agents.iter().map(|a| (a.home, a.x, a.y)).collect();
                let homes2: Vec<(usize, i32, i32)> =
                    s2.agents.iter().map(|a| (a.home, a.x, a.y)).collect();
                assert_eq!(homes1, homes2, "migration must be deterministic");
                return;
            }
        }
        panic!("no seed produced a migration over 4 windows");
    }

    fn craft_families(s: &mut Sim) {
        s.agents.clear();
        s.families.clear();
        for t in 0..s.towns.len() {
            s.families.push(Family {
                id: t,
                town: t,
                members: 2,
                children: 0,
                name: format!("Род {}", t),
                extinct: false,
                accent: (200, 100, 100),
                role: Role::Worker,
            });
        }
    }

#[test]
fn marriages_form_and_cheapen_births() {
        for seed in 1..40u64 {
            let mut s = Sim::new(seed);
            if s.towns.len() < 2 {
                continue;
            }
            s.towns[0].x = s.towns[0].x.wrapping_add((seed % 11) as i32 * 7);
            s.towns[0].y = s.towns[0].y.wrapping_add((seed % 5) as i32 * 3);
            s.towns[1].x = s.towns[0].x + 20;
            s.towns[1].y = s.towns[0].y;
            craft_families(&mut s);
            s.tick_count = MARRIAGE_EVERY;
            s.marriage_step();
            if s.alliances.is_empty() {
                continue;
            }
            assert!(s.alliance_between(0, 1), "marriage should ally the two towns");
            assert!(s.has_alliance(0), "aligned town should report its alliance");
            s.towns[0].stocks = Stock { food: 200.0, water: 200.0, ore: 20.0, meat: 20.0, gold: 0.0, fish: 0.0 };
            s.towns[0].cap = 40;
            let start = s.towns[0].stocks.food;
            s.reproduction();
            let cost = start - s.towns[0].stocks.food;
            assert!(cost > 0.0, "married families should still have children");
            assert!(cost < BIRTH_FOOD - 0.1, "marriage should cheapen births (cost {})", cost);
            return;
        }
        panic!("no seed formed a marriage alliance");
    }

    fn ensure_extra_towns(s: &mut Sim, want: usize) {
        while s.towns.len() < want {
            let li = s.towns.len() - 1;
            let (bx, by) = (s.towns[li].x + 30, s.towns[li].y);
            s.towns.push(Settlement {
                x: bx,
                y: by,
                stocks: Stock { food: 40.0, water: 30.0, ore: 10.0, meat: 6.0, gold: 0.0, fish: 0.0 },
                r: 200,
                g: 200,
                b: 200,
                cap: 12,
                queue: Vec::new(),
                built: Vec::new(),
                at_war: false,
                raiders: 0,
                enemy: None,
                idea: TownIdea::None,
                idea_left: 0.0,
                faith: 0.0,
                blessing: Blessing::None,
                blessing_left: 0.0,
                prophecy: Prophecy::None,
                prophecy_left: 0.0,
                revelation: 0.0,
                plague_until: 0,
                empire: None,
                alive: true,
                waste: 0,
                dev: 0.0,
            });
        }
    }

    #[test]
    fn allied_and_treaty_towns_never_become_enemies() {
        for seed in 1..6u64 {
            let mut s = Sim::new(seed);
            ensure_extra_towns(&mut s, 4);
            let ox = s.towns[0].x;
            let oy = s.towns[0].y;
            s.towns[0].x = ox + (seed % 7) as i32 * 9;
            s.towns[0].y = oy;
            s.towns[1].x = s.towns[0].x + 20;
            s.towns[1].y = s.towns[0].y;
            craft_families(&mut s);
            s.towns[0].stocks = Stock { food: 300.0, water: 300.0, ore: 50.0, meat: 30.0, gold: 0.0, fish: 0.0 };
            s.towns[0].cap = 60;
            s.alliances.push((0, 1, u64::MAX));
            s.towns[0].empire = Some(0);
            s.towns[1].empire = Some(0);
            s.towns[2].empire = Some(1);
            s.towns[3].empire = Some(1);
            s.towns[2].x = s.towns[1].x + 20;
            s.towns[2].y = s.towns[1].y;
            s.empires.push(Empire { r: 220, g: 40, b: 40, name: "A".into(), members: vec![0, 1] });
            s.empires.push(Empire { r: 40, g: 40, b: 220, name: "B".into(), members: vec![2, 3] });
            s.tick_count = TREATY_EVERY;
            s.treaty_step();
            assert!(s.alliance_between(0, 1), "forced alliance must be active");
            let peaceful_pair = s.alliance_between(0, 1)
                || (s.empire_of(0).is_some() && s.empire_of(2).is_some() && s.treaty_between(0, 2));
            assert!(peaceful_pair, "seed {} should hold at least one peaceful tie", seed);
            for _ in 0..1500 {
                s.tick();
                for i in 0..s.towns.len() {
                    for j in (i + 1)..s.towns.len() {
                        if s.peaceful(i, j)
                            && ((s.towns[i].at_war && s.towns[i].enemy == Some(j))
                                || (s.towns[j].at_war && s.towns[j].enemy == Some(i)))
                        {
                            panic!("a peaceful partner became an enemy (seed {} tick {})", seed, s.tick_count);
                        }
                    }
                }
            }
            return;
        }
        unreachable!();
    }

    #[test]
    fn gifts_flow_from_surplus_town_to_ally() {
        let mut s = Sim::new(7);
        s.alliances.push((0, 1, u64::MAX));
        s.towns[0].stocks = Stock { food: 500.0, water: 500.0, ore: 20.0, meat: 10.0, gold: 0.0, fish: 0.0 };
        s.towns[1].stocks = Stock { food: 5.0, water: 5.0, ore: 5.0, meat: 5.0, gold: 0.0, fish: 0.0 };
        s.towns[0].x = 60;
        s.towns[0].y = 60;
        s.towns[1].x = 61;
        s.towns[1].y = 60;
        s.tick_count = GIFT_EVERY;
        s.gift_step();
        assert!(!s.caravans.is_empty(), "surplus ally should send a free gift caravan");
        let gift = s.caravans[0].gift && s.caravans[0].home == 0 && s.caravans[0].target == 1;
        assert!(gift, "the caravan must be a gift to the ally");
        let sent_food: f32 = s
            .caravans
            .iter()
            .filter(|c| c.home == 0)
            .flat_map(|c| c.goods.iter())
            .filter(|(k, _)| *k == ResourceKind::Food)
            .map(|(_, q)| *q)
            .sum();
        assert!(sent_food > 0.0, "gift should carry food");
        for _ in 0..40 {
            if s.caravans.is_empty() {
                break;
            }
            s.caravans_step();
        }
        assert!(s.caravans.is_empty(), "gift caravan should be delivered");
        assert!(s.towns[1].stocks.food >= 5.0 + sent_food - 0.5, "gift food should arrive");
        assert!(s.towns[0].stocks.gold == 0.0, "a gift must not pay gold to the sender");
    }

    #[test]
    fn empire_treaties_calm_borders() {
        for seed in 1..40u64 {
            let mut s = Sim::new(seed);
            ensure_extra_towns(&mut s, 4);
            let ox = s.towns[0].x;
            let oy = s.towns[0].y;
            s.towns[0].x = ox.wrapping_add((seed % 7) as i32 * 9);
            s.towns[0].y = oy.wrapping_add((seed % 5) as i32 * 4);
            s.towns[1].x = s.towns[0].x + 20;
            s.towns[1].y = s.towns[0].y;
            s.towns[2].x = s.towns[1].x + 20;
            s.towns[2].y = s.towns[1].y;
            s.towns[3].x = s.towns[2].x + 20;
            s.towns[3].y = s.towns[2].y;
            s.towns[0].empire = Some(0);
            s.towns[1].empire = Some(0);
            s.towns[2].empire = Some(1);
            s.towns[3].empire = Some(1);
            s.empires.push(Empire { r: 220, g: 40, b: 40, name: "Вест".into(), members: vec![0, 1] });
            s.empires.push(Empire { r: 40, g: 40, b: 220, name: "Ост".into(), members: vec![2, 3] });
            s.tick_count = TREATY_EVERY;
            s.treaty_step();
            if !s.treaty_between(0, 2) {
                continue;
            }
            assert!(s.treaty_between(0, 2), "treaty should bind the two empires");
            s.towns[0].stocks = Stock { food: 300.0, water: 300.0, ore: 50.0, meat: 30.0, gold: 0.0, fish: 0.0 };
            s.towns[0].cap = 60;
            craft_families(&mut s);
            for _ in 0..1500 {
                s.tick();
                for i in 0..s.towns.len() {
                    for j in (i + 1)..s.towns.len() {
                        if s.treaty_between(i, j)
                            && ((s.towns[i].at_war && s.towns[i].enemy == Some(j))
                                || (s.towns[j].at_war && s.towns[j].enemy == Some(i)))
                        {
                            panic!("treaty members fought each other");
                        }
                    }
                }
            }
            return;
        }
        panic!("no seed signed an inter-empire treaty");
    }

    #[test]
    fn science_accumulates_and_unlocks_tiers() {
        let mut s = Sim::new(90);
        s.agents.clear();
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.towns[0].stocks = Stock { food: 200.0, water: 200.0, ore: 100.0, meat: 20.0, gold: 0.0, fish: 0.0 };
        for _ in 0..6 {
            s.spawn_agent(0, tx, ty, 0, false);
        }
        assert_eq!(s.tech_tier(0), 0, "fresh towns know no science");
        assert!(!s.can_build(0, BuildingKind::University), "tier0 must block university");
        s.tick_count = TECH_EVERY;
        s.tech_step();
        assert!(s.towns[0].dev >= DEV_BASE, "science should accumulate in a settled town");
        s.towns[0].dev = TECH_TIER1 - 1.0;
        assert_eq!(s.tech_tier(0), 0);
        s.towns[0].dev = TECH_TIER1;
        assert_eq!(s.tech_tier(0), 1, "tier1 at the first threshold");
        assert!(s.can_build(0, BuildingKind::University), "tier1 must unlock university");
        assert!(!s.can_build(0, BuildingKind::Smithy), "tier1 must still block smithy");
        assert!(!s.can_build(0, BuildingKind::Library), "tier1 must still block library");
        s.towns[0].dev = TECH_TIER2;
        assert_eq!(s.tech_tier(0), 2);
        assert!(s.can_build(0, BuildingKind::Smithy), "tier2 must unlock smithy");
        assert!(!s.can_build(0, BuildingKind::Library), "tier2 must still block library");
        s.towns[0].dev = TECH_TIER3;
        assert_eq!(s.tech_tier(0), 3);
        assert!(s.can_build(0, BuildingKind::Library), "tier3 must unlock library");
    }

    #[test]
    fn low_tier_towns_cannot_request_tech_buildings() {
        let mut s = Sim::new(91);
        s.towns[0].dev = 0.0;
        s.build_request(0, BuildingKind::University);
        assert!(s.towns[0].queue.is_empty(), "university must not enter the queue without science");
        s.towns[0].dev = TECH_TIER1;
        s.build_request(0, BuildingKind::University);
        assert_eq!(s.towns[0].queue.len(), 1, "university should queue at tier1");
        s.build_request(0, BuildingKind::Smithy);
        s.build_request(0, BuildingKind::Library);
        assert_eq!(s.towns[0].queue.len(), 1, "smithy/library need higher tiers");
    }

    #[test]
    fn university_fosters_scholars_and_accelerates_science() {
        let mut s = Sim::new(92);
        craft_families(&mut s);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.towns[0].stocks = Stock { food: 200.0, water: 200.0, ore: 100.0, meat: 20.0, gold: 0.0, fish: 0.0 };
        s.towns[0].dev = TECH_TIER1;
        for _ in 0..6 {
            s.spawn_agent(0, tx, ty, 0, false);
        }
        s.build_request(0, BuildingKind::University);
        s.towns[0].queue[0].1 = UNIVERSITY_COST - 1.0;
        s.construction();
        assert!(
            s.towns[0].built.iter().any(|b| *b == BuildingKind::University),
            "university should finish construction"
        );
        assert!(
            s.families.iter().any(|f| !f.extinct && f.role == Role::Scholar),
            "university should raise a scholar family"
        );
        let scholars = s.agents.iter().filter(|a| a.role == Role::Scholar).count();
        assert!(scholars >= 1, "the scholar agent should carry the role");
        s.tick_count = TECH_EVERY;
        let before = s.towns[0].dev;
        s.tech_step();
        let gain = s.towns[0].dev - before;
        let expected = DEV_BASE + DEV_UNI_BONUS + scholars as f32 * DEV_SCHOLAR_BONUS;
        assert!(
            (gain - expected).abs() < 0.05,
            "university + scholar should boost science (gain {} vs {})",
            gain,
            expected
        );
    }

    #[test]
    fn smithy_and_builders_speed_construction() {
        let mut s = Sim::new(93);
        craft_families(&mut s);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        s.towns[0].stocks = Stock { food: 200.0, water: 200.0, ore: 100.0, meat: 20.0, gold: 0.0, fish: 0.0 };
        s.towns[0].built.push(BuildingKind::Smithy);
        s.spawn_agent(0, tx, ty, 0, false);
        let bi = s.agents.len() - 1;
        s.agents[bi].role = Role::Builder;
        s.build_request(0, BuildingKind::Barracks);
        s.towns[0].queue[0].1 = BARRACKS_COST - 3.0;
        s.construction();
        assert_eq!(s.towns[0].queue.len(), 0, "smithy + builder should finish the frame");
        assert!(
            s.towns[0].built.iter().any(|b| *b == BuildingKind::Barracks),
            "barracks should complete"
        );
    }

    #[test]
    fn forest_fire_burns_and_expires_leaving_grass() {
        let mut s = Sim::new(50);
        for c in s.grid.iter_mut() {
            c.burn = 0;
            c.gold = 0.0;
        }
        let fi = s
            .grid
            .iter()
            .position(|c| c.terrain == Terrain::Forest)
            .unwrap();
        s.grid[fi].burn = FIRE_LEN;
        s.grid[fi].food = 8.0;
        let (fx, fy) = (fi as i32 % W as i32, fi as i32 / W as i32);
        let mut saw_burn = false;
        let mut saw_spread = false;
        for t in 0..(FIRE_LEN + 8) {
            s.tick_count = t as u64;
            s.fire_spread();
            if s.grid[fi].burn > 0 {
                saw_burn = true;
            }
            for ny in -1..=1 {
                for nx in -1..=1 {
                    if nx == 0 && ny == 0 {
                        continue;
                    }
                    let sx = fx + nx;
                    let sy = fy + ny;
                    if in_bounds(sx, sy) && s.grid[idx(sx, sy)].burn > 0 {
                        saw_spread = true;
                    }
                }
            }
        }
        assert!(saw_burn, "the lit cell should burn for a while");
        assert_eq!(s.grid[fi].burn, 0, "fire must burn out");
        let c = &s.grid[fi];
        assert_eq!(c.terrain, Terrain::Grass, "burned forest becomes grass");
        assert!(c.food <= 0.01, "burned terrain holds no food");
        assert!(saw_spread, "fire should be able to spread to neighbors");
    }

    #[test]
    fn generated_fire_event_lights_a_forest() {
        let mut s = Sim::new(51);
        for c in s.grid.iter_mut() {
            c.burn = 0;
            c.gold = 0.0;
        }
        let mut any = false;
        for e in 1..40u64 {
            s.tick_count = e * FIRE_EVERY;
            s.fire_step();
            if s.grid.iter().any(|c| c.burn > 0) {
                any = true;
                break;
            }
        }
        assert!(any, "fire events should eventually ignite a forest over many epochs");
    }

    #[test]
    fn wolf_hordes_swarm_near_a_town() {
        let mut s = Sim::new(60);
        for a in s.animals.iter_mut() {
            a.species = Species::Deer;
        }
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        let mut any_horde = false;
        for e in 1..30u64 {
            s.tick_count = e * HORDE_EVERY;
            s.horde_step();
            let near = s
                .animals
                .iter()
                .filter(|a| a.species == Species::Wolf && (a.x - tx).abs().max(a.y - ty) <= 20)
                .count();
            if near >= 3 {
                any_horde = true;
                break;
            }
        }
        assert!(
            any_horde,
            "an uneasy town should eventually draw a wolf horde"
        );
    }

    #[test]
    fn gold_vein_pays_outstanding_gold_to_surrounding_towns() {
        let mut s = Sim::new(77);
        s.agents.clear();
        s.towns[0].stocks = Stock { food: 50.0, water: 50.0, ore: 10.0, meat: 5.0, gold: 0.0, fish: 0.0 };
        s.towns[0].x = 40;
        s.towns[0].y = 40;
        s.gold_veins.clear();
        let vx = 50;
        let vy = 50;
        s.gold_veins.push((vx, vy, 400.0));
        s.grid[idx(vx, vy)].gold = 400.0;
        s.gold_vein_trickle();
        assert!(
            s.towns[0].stocks.gold > 0.0,
            "a vein in range should feed gold to the town"
        );
        assert!(
            s.towns[0].stocks.gold <= GOLD_MAX + 0.001,
            "gold must respect the town cap"
        );
        let still = s.gold_veins.iter().any(|&(x, y, _)| x == vx && y == vy);
        assert!(still, "a rich vein should not exhaust in a single tick");
    }

    #[test]
    fn gold_vein_spawns_near_a_town() {
        let mut s = Sim::new(78);
        for c in s.grid.iter_mut() {
            c.gold = 0.0;
            c.burn = 0;
        }
        s.gold_veins.clear();
        s.towns[0].x = 50;
        s.towns[0].y = 50;
        let mut any = false;
        for e in 1..30u64 {
            s.tick_count = e * GOLD_VEIN_EVERY;
            s.gold_vein_find();
            if !s.gold_veins.is_empty() {
                any = true;
                break;
            }
        }
        assert!(any, "a vein should eventually appear near a town");
        for &(vx, vy, _) in &s.gold_veins {
            assert_eq!(s.grid[idx(vx, vy)].terrain, Terrain::Hills, "vein sits in hills");
            assert!(s.grid[idx(vx, vy)].gold > 0.0, "vein cell holds gold");
        }
    }

    #[test]
    fn day_night_cycle_advances() {
        let mut s = Sim::new(3);
        assert_eq!(s.day_phase, 0);
        assert!(s.is_day());
        for _ in 0..DAY_LEN / 2 {
            s.tick();
        }
        assert!(s.is_night(), "the world should turn dark when halfway through the day");
        for _ in 0..DAY_LEN / 2 {
            s.tick();
        }
        assert!(s.is_day(), "dawn should arrive after a full cycle");
        assert_eq!(s.day_phase, 0, "day phase wraps around each full day");
    }

    #[test]
    fn seasons_rotate_in_order() {
        let mut s = Sim::new(3);
        assert_eq!(s.season, Season::Spring);
        s.tick_count = SEASON_LEN - 1;
        s.tick();
        assert_eq!(s.season, Season::Summer, "spring turns to summer");
        s.tick_count = 2 * SEASON_LEN - 1;
        s.tick();
        assert_eq!(s.season, Season::Autumn, "summer turns to autumn");
        s.tick_count = 3 * SEASON_LEN - 1;
        s.tick();
        assert_eq!(s.season, Season::Winter, "autumn turns to winter");
        s.tick_count = 4 * SEASON_LEN - 1;
        s.tick();
        assert_eq!(s.season, Season::Spring, "winter turns back to spring");
    }

    #[test]
    fn wells_produce_less_at_night() {
        let mut s = Sim::new(3);
        let ti = 0;
        s.towns[ti].built.push(BuildingKind::Well);
        s.towns[ti].stocks.water = 0.0;
        s.day_phase = 0;
        s.tick();
        let day_gain = s.towns[ti].stocks.water;
        assert!(day_gain > 0.0, "wells should produce water by day");
        s.towns[ti].stocks.water = 0.0;
        s.day_phase = DAY_LEN / 2 + 1;
        s.tick();
        let night_gain = s.towns[ti].stocks.water;
        assert!(
            night_gain < day_gain,
            "wells should produce less water at night ({} < {})",
            night_gain,
            day_gain
        );
    }

    #[test]
    fn save_load_round_trip() {
        let mut s = Sim::new(42);
        for _ in 0..500 {
            s.tick();
        }
        let json = s.save_json();
        let s2 = Sim::load_json(&json).expect("load should succeed");
        assert_eq!(s2.tick_count, s.tick_count);
        assert_eq!(s2.grid.len(), s.grid.len());
        assert_eq!(s2.agents.len(), s.agents.len());
        assert_eq!(s2.towns.len(), s.towns.len());
        assert_eq!(s2.families.len(), s.families.len());
        assert_eq!(s2.empires.len(), s.empires.len());
        assert_eq!(s2.animals.len(), s.animals.len());
        assert_eq!(s2.rng, s.rng);
        assert_eq!(s2.weather, s.weather);
        assert_eq!(s2.season, s.season);
        assert_eq!(s2.day_phase, s.day_phase);
        assert_eq!(s2.alliances, s.alliances);
    }

    #[test]
    fn road_toggle_on_grass() {
        let mut s = Sim::new(1);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        let rx = tx + 10;
        let ry = ty;
        if in_bounds(rx, ry) && s.grid[idx(rx, ry)].terrain == Terrain::Grass {
            assert!(!s.roads[idx(rx, ry)]);
            s.toggle_road(rx, ry);
            assert!(s.roads[idx(rx, ry)]);
            s.toggle_road(rx, ry);
            assert!(!s.roads[idx(rx, ry)]);
        }
    }

    #[test]
    fn road_cannot_build_on_water() {
        let mut s = Sim::new(1);
        for y in 0..H as i32 {
            for x in 0..W as i32 {
                if s.grid[idx(x, y)].terrain == Terrain::Water {
                    s.toggle_road(x, y);
                    assert!(!s.roads[idx(x, y)], "road must not be placed on water");
                    return;
                }
            }
        }
    }

    #[test]
    fn road_cannot_build_on_hills() {
        let mut s = Sim::new(1);
        for y in 0..H as i32 {
            for x in 0..W as i32 {
                if s.grid[idx(x, y)].terrain == Terrain::Hills {
                    s.toggle_road(x, y);
                    assert!(!s.roads[idx(x, y)], "road must not be placed on hills/ore");
                    return;
                }
            }
        }
    }

    #[test]
    fn road_can_build_on_tundra() {
        let mut s = Sim::new(1);
        for y in 0..H as i32 {
            for x in 0..W as i32 {
                if s.grid[idx(x, y)].terrain == Terrain::Tundra {
                    s.toggle_road(x, y);
                    assert!(s.roads[idx(x, y)], "road must be allowed on tundra/snow");
                    return;
                }
            }
        }
    }

    #[test]
    fn forest_does_not_regrow_on_road() {
        let mut s = Sim::new(1);
        let (tx, ty) = (s.towns[0].x, s.towns[0].y);
        let mut found_forest = false;
        for dy in -10..=10 {
            for dx in -10..=10 {
                let (fx, fy) = (tx + dx, ty + dy);
                if in_bounds(fx, fy) && s.grid[idx(fx, fy)].terrain == Terrain::Forest {
                    s.roads[idx(fx, fy)] = true;
                    let before = s.grid[idx(fx, fy)].food;
                    while s.tick_count % REGROW_EVERY != 0 { s.tick_count += 1; }
                    s.tick();
                    let after = s.grid[idx(fx, fy)].food;
                    assert_eq!(before, after, "forest food on road must not regrow");
                    found_forest = true;
                    break;
                }
            }
            if found_forest { break; }
        }
    }
}