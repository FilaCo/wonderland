# Roadmap

## Phase 1 — Engine Core

- [ ] SDL3 window + event loop
- [x] ECS component storage (`SparseSet` becomes `ComponentPool<T>`)
- [ ] Event bus (deferred queues + immediate signals)
- [ ] Input system (platform events → named actions via events)

## Phase 2 — Vulkan Renderer

- [ ] Vulkan bootstrap (instance, device, swapchain, command buffers)
- [ ] 2D sprite rendering + batch rendering
- [ ] Shader pipeline, texture loading
- [ ] Camera/viewport system

## Phase 3 — Asset & Scene

- [ ] Asset manager (load/cache/hot-reload textures, shaders, audio)
- [ ] Scene definition (what is a "level", how is it serialized)
- [ ] Audio

## Phase 4 — UI System

- [ ] Retained-mode UI tree (C++ API)
- [ ] Layout engine, event propagation (clicks, focus)
- [ ] Hatter language implementation → compiles/interprets to the C++ tree

## Phase 5 — AI System

- [ ] Behavior tree framework (C++ node API)
- [ ] Reuse Hatter parser → AI node registry + evaluator

## Phase 6 — ECS Scripting

- [ ] Alice language
- [ ] Hot-reload integration

## Phase 7 — Polish

- [ ] Editor tooling
- [ ] Documentation, examples, a real small game
