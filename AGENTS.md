# Eternal Dominion — Development Rules

## Project Goal
Build a fully playable cosmic sandbox strategy game using Rust and Bevy. The game contains independently evolving universes, rival deities, civilizations, a soul economy, divine powers, and persistent world history.

## Architecture
- Keep the simulation core independent of Bevy.
- Use a modular Rust workspace with clear crate responsibilities.
- Keep presentation, simulation, content definitions, and persistence separate.
- Avoid circular dependencies and unnecessary abstractions.
- Do not introduce networking or microservices without an explicit requirement.

## Simulation
- Use a fixed simulation timestep independent of rendering FPS.
- Make randomness seedable and controllable in tests.
- Define and test a stable system update order.
- Unobserved universes must continue to evolve.
- Track the source, expenditure, transfer, and removal of resources.
- Preserve enough state to resume deterministic simulation where supported.

## Rust Quality
- Avoid `unsafe` unless explicitly justified and reviewed.
- Handle production errors explicitly; do not use `unwrap()` as routine error handling.
- Keep dependencies minimal and justified.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` for relevant changes.
- Keep simulation tests independent of the graphical runtime.

## Performance
- Measure before optimizing.
- Avoid unnecessary cloning of large state.
- Use lower-frequency background simulation where appropriate.
- Preserve gameplay consistency when optimizing distant universes.
- Establish performance budgets from benchmark results.

## Persistence
- Version save schemas.
- Handle invalid saves safely.
- Test save/load round trips.
- Protect valid saves from partial writes.
- Plan migrations when persisted data structures change.

## AI-Assisted Development
- Inspect relevant code before modifying it.
- Keep changes focused on the requested task.
- Do not claim code builds or tests pass unless verified.
- Explain architectural and dependency changes.
- Do not silently change gameplay rules or save formats.

## Definition of Done
A change is complete when:
1. Its behavior is implemented.
2. Relevant tests pass.
3. Formatting and lint checks pass, where applicable.
4. New gameplay rules are documented.
5. Existing save compatibility and simulation determinism are considered.
