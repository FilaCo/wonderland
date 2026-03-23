# Wonderland — Concept

Wonderland is a focused, fast, and specialized 2D game engine written in C++. Rather than exposing a single general-purpose API, Wonderland provides dedicated domain-specific languages (DSLs) tailored to each major concern of a game — matching the right tool to the task at hand.

## Core DSLs

### Alice — Game Logic (ECS)
Alice is an ECS-based (Entity–Component–System) DSL for authoring game logic. It gives developers a structured, data-oriented way to define entities, attach components, and compose systems. Alice is designed for performance: systems run in parallel where dependencies allow, keeping game logic fast even as the world grows complex.

### Hatter — UI (Declarative / QML-like)
Hatter is a declarative, QML-inspired DSL for building in-game user interfaces — menus, HUDs, dialogue boxes, and beyond. UI is described as a hierarchy of reactive elements, where state changes automatically propagate to the view. Hatter keeps UI authoring separate from game logic, making both easier to reason about and iterate on.

### Planned DSLs
Wonderland's specialization principle extends to other domains as the engine matures — AI behavior, shader authoring, audio scripting — each with a language shaped around its own idioms.

## Integration

DSLs in Wonderland communicate through a shared **event system**. Alice systems emit events, Hatter elements react to them, and future DSLs plug into the same bus. This keeps each DSL self-contained while allowing them to compose freely — an Alice health component fires a `damage` event, and a Hatter health bar updates in response, with neither knowing about the other's internals.

## Design Philosophy

Every DSL in Wonderland exists to reduce the gap between intent and implementation. A game developer should be able to express *what* they want — a health bar that reacts to damage, an enemy that patrols a path — without fighting a generic API to get there.

This specialization is what makes Wonderland *Wonderland*: a world where every tool fits its purpose perfectly.
