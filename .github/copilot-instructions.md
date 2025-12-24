# Autoclicker Project - AI Coding Instructions

## Project Architecture

This is a **Tauri desktop application** with a Vue 3 + TypeScript frontend and Rust backend.

- **Frontend**: `src/` (Vue 3 SFCs with `<script setup>` syntax)
- **Backend**: `src-tauri/src/` (Rust code)
- **Build config**: [vite.config.ts](../../vite.config.ts), [src-tauri/Cargo.toml](../../src-tauri/Cargo.toml), [tauri.conf.json](../../src-tauri/tauri.conf.json)

### Frontend-Backend Communication

Frontend calls Rust functions via `invoke()` from `@tauri-apps/api/core`:
```typescript
// src/App.vue
const result = await invoke("greet", { name: name.value });
```

Rust functions are exposed with `#[tauri::command]` macro in [src-tauri/src/lib.rs](../../src-tauri/src/lib.rs):
```rust
#[tauri::command]
fn greet(name: &str) -> String { ... }
```

Commands must be registered in the `invoke_handler!` macro in [src-tauri/src/lib.rs](../../src-tauri/src/lib.rs#L11).

## Development Workflow

### Running the App
- **Full dev mode**: `npm run tauri dev` (starts Vite dev server on port 1420 + Tauri window)
- **Web dev only**: `npm run dev` (Vite dev server for frontend testing)
- **Preview**: `npm run preview` (production build preview)

### Building
- **Frontend only**: `npm run build` (runs `vue-tsc --noEmit` for type checking + Vite bundle to `dist/`)
- **Full release**: `npm run tauri build` (compiles Rust + bundles frontend)

### Key Development Details
- Vite watches `src/` only; `src-tauri/` changes require restart
- Tauri dev server runs on fixed port 1420 (fails if unavailable)
- TypeScript strict mode enabled for both frontend and Rust (via [tsconfig.json](../../tsconfig.json))

## Project Structure Patterns

- **Single-file components**: Vue files follow `<script setup>` with reactive refs (`ref()`)
- **Styling**: Tailwind CSS utility-first framework with custom animations in [tailwind.config.js](../../tailwind.config.js)
- **Global styles**: [src/index.css](../../src/index.css) imports Tailwind and defines base styles
- **Animations**: Custom keyframes for `pulse-glow`, `slide-in-top`, `fade-in`, `scale-in` defined in Tailwind config
- **Responsive design**: Mobile-first approach using Tailwind breakpoints (sm:, md:, lg:)
- **Dark mode**: CSS class-based dark mode with `:dark:` variants throughout
- **Tauri plugins**: Currently using `tauri-plugin-opener` for opening URLs

## Important Files & Their Roles

| File | Purpose |
|------|---------|
| [src-tauri/src/lib.rs](../../src-tauri/src/lib.rs) | Tauri app entry point; autoclicker commands with `rdev` crate |
| [src-tauri/src/main.rs](../../src-tauri/src/main.rs) | Minimal Windows subsystem wrapper |
| [src/App.vue](../../src/App.vue) | Root component; UI with Tailwind + animations |
| [src/index.css](../../src/index.css) | Tailwind directives and base styles |
| [src/main.ts](../../src/main.ts) | Vue app initialization; imports Tailwind CSS |
| [tailwind.config.js](../../tailwind.config.js) | Custom animations, colors, shadows |
| [postcss.config.js](../../postcss.config.js) | PostCSS plugin config for Tailwind |
| [tauri.conf.json](../../src-tauri/tauri.conf.json) | Window config, bundle settings, security |
| [vite.config.ts](../../vite.config.ts) | Vite + Tauri dev server config |

## Styling & Animation Patterns

### Tailwind CSS Setup
- Configured in [tailwind.config.js](../../tailwind.config.js) with extended theme
- Uses `@tailwind` directives in [src/index.css](../../src/index.css)
- PostCSS processes Tailwind before Vite bundling

### Custom Animations
```javascript
// From tailwind.config.js
animation: {
  'pulse-glow': 'pulse-glow 2s cubic-bezier(...) infinite',
  'slide-in-top': 'slide-in-top 0.5s ease-out',
  'scale-in': 'scale-in 0.3s ease-out',
  // ...
}
```

### Responsive & Dark Mode
- Use `sm:`, `md:`, `lg:` prefixes for responsive classes
- Use `dark:` prefix for dark mode variants
- Automatically detects system preference via `prefers-color-scheme`

## Common Tasks

### Adding a Rust Command
1. Define function with `#[tauri::command]` in [src-tauri/src/lib.rs](../../src-tauri/src/lib.rs)
2. Add to `generate_handler!` macro
3. Call via `invoke("command_name", {args})` in Vue

### Adding Frontend Dependencies
- **UI/Styling**: Install via npm, then import in Vue components or [src/index.css](../../src/index.css)
- **Regular packages**: `npm install <pkg>` (updates [package.json](../../package.json))

### Modifying Tailwind Theme
- Edit [tailwind.config.js](../../tailwind.config.js) to add animations, colors, shadows
- Changes hot-reload in dev mode (no restart needed)
- Custom animations must include keyframes definition in `theme.extend.keyframes`

## Architecture Rationale

- **Tauri over Electron**: Smaller bundle, native OS APIs, better performance
- **Vue 3 + TypeScript**: Type safety for frontend logic
- **Tailwind CSS**: Utility-first styling for rapid, responsive UI development
- **Custom animations**: Handcrafted keyframes for smooth, polished UX without animation libraries
