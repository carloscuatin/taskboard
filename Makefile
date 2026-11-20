API = docker compose run --rm --no-deps api
WEB = docker compose run --rm --no-deps web sh -c

.PHONY: build up down api-fmt api-fmt-check api-lint api-test api-build api-deny api-shell \
        web-install web-dev web-format-check web-lint web-typecheck web-test web-build web-shell \
        check hooks clean

build: ; docker compose build api web

up:    ; docker compose up -d db
down:  ; docker compose down

api-fmt:        ; $(API) cargo fmt --all
api-fmt-check:  ; $(API) cargo fmt --all --check
api-lint:       ; $(API) cargo clippy --workspace --all-targets -- -D warnings
api-test:       ; docker compose run --rm api cargo test --workspace
api-build:      ; $(API) cargo build --workspace
api-deny:       ; $(API) cargo deny check
api-shell:      ; docker compose run --rm api bash

web-install:       ; $(WEB) 'pnpm install --frozen-lockfile'
web-dev:           ; docker compose run --rm --service-ports web sh -c 'pnpm install && pnpm dev --host'
web-format-check:  ; $(WEB) 'pnpm format:check'
web-lint:          ; $(WEB) 'pnpm lint'
web-typecheck:     ; $(WEB) 'pnpm typecheck'
web-test:          ; $(WEB) 'pnpm test'
web-build:         ; $(WEB) 'pnpm build'
web-shell:         ; docker compose run --rm web sh

# The same gates CI runs, in the same order.
check: api-fmt-check api-lint api-test api-build api-deny \
       web-install web-format-check web-lint web-typecheck web-test web-build

hooks: ; git config core.hooksPath .githooks

clean: ; docker compose down -v
