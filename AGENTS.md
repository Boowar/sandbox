# AGENTS.md

## Current state

Rust → WebAssembly (wasm-bindgen / web-sys) проект — god-game «имитация жизни».
Сборка и dev-сервер: Trunk 0.21.x. Рендер — Canvas 2D. Ветка по умолчанию: `main`.

## Commands

- `cargo check` — быстрая проверка типов (host target)
- `cargo test` — юнит-тесты симуляции (детерминированность, границы, еда, bless)
- `trunk build --release` — production-сборка в `dist/`
- `trunk serve` — dev-сервер с live-reload на http://localhost:8080

## Conventions

- Стек: Rust + wasm32-unknown-unknown, без других языков.
- Симуляция детерминирована (Xorshift RNG). Визуальные фичи — в `render.rs`,
  логика — в `sim.rs`.
- Путь фичи: бриф в `PROJECT.md` → реализация → `cargo check` + `trunk build` → коммит.