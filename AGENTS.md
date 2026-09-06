# AGENTS.md

## Current state

Rust → WebAssembly (wasm-bindgen / web-sys) проект — god-game «имитация жизни».
Сборка и dev-сервер: Trunk 0.21.x. Рендер — Canvas 2D. Ветка по умолчанию: `main`.

Все 34 пункта дорожной карты реализованы + 10 итераций улучшений. 94 юнит-теста. Баланс оттюнен.

## Commands

- `cargo check` — быстрая проверка типов (host target)
- `cargo test` — 94 юнит-тестов
- `trunk build --release` — production-сборка в `dist/`
- `trunk serve` — dev-сервер с live-reload на http://localhost:8080

## Conventions

- Стек: Rust + wasm32-unknown-unknown, без других языков.
- Симуляция детерминирована (Xorshift RNG). Визуальные фичи — в `render.rs`,
  логика — в `sim.rs`, UI/контролы — в `lib.rs`.
- Путь фичи: бриф в `PROJECT.md` → реализация → `cargo test` + `cargo check` + `trunk build` → коммит.

## Architecture

- `src/sim.rs` (~6460 строк) — вся логика: клетки, агенты, роли, строительство, торговля, война, религия, болезни, погода, события, тесты
- `src/render.rs` (~1460 строк) — Canvas 2D пиксель-арт, панель города, HUD, tech tree panel
- `src/lib.rs` (~890 строк) — bootstrap, rAF-цикл, ввод (клавиатура/тач/колесо), кнопки, управление

## Key mechanics

- **Экономика**: еда/вода/руда/мясо/рыба/золото; колодцы дают воду (1.0/тик, +50% при дожде)
- **Склады** (📦): увеличивают лимиты ресурсов (+40 еда/вода, +25 руда, +20 мясо/рыба, +100 золото)
- **Автострой**: Well→House→Farm→Warehouse(60% food cap)→TradePost→Clinic→Sanctuary→Wall→Barracks→University→Smithy→Library; only when pop > 0
- **Технологии**: 12 tech across 4 тиров; бонусы: Agriculture ×1.25 food, Construction +20% build, Warfare +0.20 def, Theology ×1.5 faith, Engineering −0.10 energy, Cartography caravan 2-step, Medicine ×0.5 plague, Metallurgy ×1.5 ore, Commerce ×0.8 prices, Philosophy +25% dev, Mastery −10% consumption
- **Погода**: дождь +50% колодцы, жара −30% фермы, мороз −30% торговля + −30% фермы
- **Ресурсы**: лес регенерирует медленно (REGROW_EVERY=44); руда/вода конечны; дожди/метеориты
- **Торговля**: караваны между городами; торговые посты генерируют золото
- **Миграция**: жители уходят из голодающих городов; не едут в мёртвые (pop=0)
- **Дороги**: строятся/разбираются вручную (D); нельзя на воде/горах; лес на дороге не регенерирует; ускоряют караваны/армии
- **Цветные индикаторы**: 🟢 >60%, 🟡 30-60%, 🔴 <30% заполненности ресурса