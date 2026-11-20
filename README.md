# Taskboard

A multi-user API for projects and tasks, built to learn Rust on the backend and Vue on the
frontend in a domain where auth earns its place: sessions with refresh rotation, per-project roles
and a global admin.

Everything runs in Docker. The host needs only Docker, `make` and `openssl`.

## Layout

| Path | What |
|---|---|
| `api/` | Rust workspace: `domain`, `application`, `adapters`, `api` (axum) |
| `web/` | Vue 3 + Vite SPA, managed with pnpm |
| `docs/` | Architecture decisions and diagrams |

## Commands

    make env        # generate .env with a random Postgres password (other targets do it for you)
    make build      # build the dev images
    make up         # start Postgres
    make api-dev    # run the API on http://localhost:8080, rebuilt and restarted on every save
    make web-dev    # Vite dev server on http://localhost:5173
    make check      # every gate CI runs, in the same order
    make hooks      # enable the pre-commit hook (formatting)
    make clean      # remove containers and volumes

Single gates: `api-fmt-check`, `api-lint`, `api-test`, `api-build`, `api-deny`, and
`web-format-check`, `web-lint`, `web-typecheck`, `web-test`, `web-build`. `make api-shell` and
`make web-shell` open a shell in each container.

## Postgres password

The Postgres password lives in `.env`, which is git-ignored. Every `make` target that calls Docker
Compose creates it if it is missing; `make env` does the same on its own and never overwrites an
existing file. Calling `docker compose` directly without `.env` fails with a message pointing here.

Postgres applies `POSTGRES_PASSWORD` only when it initializes an empty volume. If the `pgdata` volume
already exists with another password, the API and host clients fail with `password authentication
failed` while the healthcheck stays green, because it does not authenticate. Recreate the database:

    make clean
    make up

`make clean` also removes the build and package caches, so the next `make check` rebuilds them. To
reset only the database, remove its container and volume (the volume name carries the Compose
project prefix, the directory name; check it with `docker volume ls`):

    docker compose rm -sf db && docker volume rm taskboard_pgdata

## Commits

Conventional Commits, validated in CI with the scopes in `commitlint.config.mjs`.
