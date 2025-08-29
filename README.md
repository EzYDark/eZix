# eZix

Declarative system setup for Windows 11 inspired by Nix/NixOS. Linux support planned via Nix plus native tooling (e.g. apt). For now: Windows-first, focused on repeatable configuration.

> Current package managers in scope: `winget`, `scoop`, `npm`, `choco` (integration will land incrementally).

---

## Why this exists
Treat OS tweaks like code. Declare modules, run once, get the same machine every time. If a module isn’t declared, it gets disabled to keep drift low.


---

## Modules
- [ ] Zen Browser