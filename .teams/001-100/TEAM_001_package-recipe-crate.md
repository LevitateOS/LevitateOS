# TEAM_001: Package Recipe Crate

## Goal
Create a Rust crate for parsing and executing S-expression package recipes as defined in `docs/package-recipe-format.md`.

## Design

### Crate Structure
```
crates/recipe/
├── Cargo.toml
├── src/
│   ├── lib.rs        # Public API
│   ├── parser.rs     # S-expression parser (~30 lines)
│   ├── ast.rs        # Expr enum (Atom, List)
│   ├── recipe.rs     # Recipe struct and actions
│   └── executor.rs   # Execute recipe actions
```

### Core Types
```rust
// AST (minimal)
enum Expr {
    Atom(String),
    List(Vec<Expr>),
}

// Recipe actions
enum Action {
    Description(String),
    License(Vec<String>),
    Deps(Vec<String>),
    BuildDeps(Vec<String>),
    Acquire(AcquireSpec),
    Build(BuildSpec),
    Install(InstallSpec),
    Configure(ConfigureSpec),
    Start(StartSpec),
    Stop(StopSpec),
    Update(UpdateSpec),
    Remove(RemoveSpec),
    Hooks(HooksSpec),
}
```

## Status
- [x] Create crate structure
- [x] Implement S-expression parser
- [x] Define AST types
- [x] Implement recipe parsing
- [ ] Implement executor (basic actions)
- [x] Add tests (9 unit tests + 1 doctest)

## Decisions
- Parser is minimal - all semantics in interpreter
- No external dependencies for parser (pure Rust)
- Executor will shell out for commands initially
