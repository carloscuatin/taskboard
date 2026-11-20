# Taskboard

A multi-user API for projects and tasks, built to learn Rust on the backend and Vue on the
frontend in a domain where auth earns its place: sessions with refresh rotation, per-project roles
and a global admin.

Everything runs in Docker. The host needs only Docker and `make`.

## Layout

| Path | What |
|---|---|
| `api/` | Rust workspace: `domain`, `application`, `adapters`, `api` (axum) |
| `web/` | Vue 3 + Vite SPA, managed with pnpm |
| `docs/` | Architecture decisions and diagrams |

## Commands

    make build      # build the dev images
    make up         # start Postgres
    make web-dev    # Vite dev server on http://localhost:5173
    make check      # every gate CI runs, in the same order
    make hooks      # enable the pre-commit hook (formatting)
    make clean      # remove containers and volumes

Single gates: `api-fmt-check`, `api-lint`, `api-test`, `api-build`, `api-deny`, and
`web-format-check`, `web-lint`, `web-typecheck`, `web-test`, `web-build`. `make api-shell` and
`make web-shell` open a shell in each container.

## Commits

Conventional Commits, validated in CI with the scopes in `commitlint.config.mjs`.
