# ⬆ Shift

> One command to upgrade anything.

Shift is a universal upgrade pilot that automates dependency upgrades — including the code changes needed to stay compatible. It detects your stack, applies AST-based codemods, and lets you review every change in an interactive merge editor before anything is applied.

```bash
shift upgrade react@19
```

## Status

Early development. Built with Rust.

- [x] Workspace structure
- [x] `shift status` — detects your stack
- [ ] Snapshot & rollback
- [ ] Codemod engine
- [ ] Merge editor
- [ ] Public launch

## Quick Start

```bash
shift init             # scan project, create .shift/ config
shift status           # show stack and available upgrades
shift upgrade react@19 # upgrade with interactive review
shift rollback 1       # instant rollback
```

## License

MIT