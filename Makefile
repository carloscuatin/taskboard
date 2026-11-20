API = docker compose run --rm --no-deps api
WEB = docker compose run --rm --no-deps web sh -c

.PHONY: env build up down api-fmt api-fmt-check api-lint api-test api-build api-deny api-shell \
        web-install web-dev web-format-check web-lint web-typecheck web-test web-build web-shell \
        check hooks clean

# Docker Compose refuses to read compose.yaml without POSTGRES_PASSWORD, so every target that
# calls it depends on .env. The file has no prerequisites: make never regenerates an existing one.
# The password is generated before anything is written, so a failed openssl leaves no .env behind.
env: .env

.env:
	@umask 077; \
	pw=$$(openssl rand -hex 16) && \
	printf 'POSTGRES_PASSWORD=%s\nDATABASE_URL=postgres://taskboard:%s@localhost:5432/taskboard\n' "$$pw" "$$pw" > .env.tmp && \
	mv .env.tmp .env
	@echo "Generated .env"

build: .env ; docker compose build api web

up:    .env ; docker compose up -d db
down:  .env ; docker compose down

api-fmt:        .env ; $(API) cargo fmt --all
api-fmt-check:  .env ; $(API) cargo fmt --all --check
api-lint:       .env ; $(API) cargo clippy --workspace --all-targets -- -D warnings
api-test:       .env ; docker compose run --rm api cargo test --workspace
api-build:      .env ; $(API) cargo build --workspace
api-deny:       .env ; $(API) cargo deny check
api-shell:      .env ; docker compose run --rm api bash

web-install:       .env ; $(WEB) 'pnpm install --frozen-lockfile'
web-dev:           .env ; docker compose run --rm --service-ports web sh -c 'pnpm install && pnpm dev --host'
web-format-check:  .env ; $(WEB) 'pnpm format:check'
web-lint:          .env ; $(WEB) 'pnpm lint'
web-typecheck:     .env ; $(WEB) 'pnpm typecheck'
web-test:          .env ; $(WEB) 'pnpm test'
web-build:         .env ; $(WEB) 'pnpm build'
web-shell:         .env ; docker compose run --rm web sh

# The same gates CI runs, in the same order.
check: api-fmt-check api-lint api-test api-build api-deny \
       web-install web-format-check web-lint web-typecheck web-test web-build

hooks: ; git config core.hooksPath .githooks

clean: .env ; docker compose down -v
