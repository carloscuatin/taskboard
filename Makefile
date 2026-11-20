API = docker compose run --rm --no-deps api
WEB = docker compose run --rm --no-deps web sh -c

.DEFAULT_GOAL := build

.PHONY: env build up down api-fmt api-fmt-check api-lint api-test api-build api-deny api-dev api-shell \
        web-install web-dev web-format-check web-lint web-typecheck web-test web-build web-shell \
        check hooks clean

# Docker Compose refuses to read compose.yaml without POSTGRES_PASSWORD, so every target that
# calls it depends on `env`, which makes sure .env exists and carries a password. An existing .env
# is never rewritten, even an incomplete one (say, a copy of .env.example): it is the user's file.
env: .env
	@grep -Eq '^POSTGRES_PASSWORD=.+' .env || { \
		echo "make: .env has no POSTGRES_PASSWORD value: delete .env so make env regenerates it, or set the value" >&2; \
		exit 1; }

# Written to a uniquely named temp file and moved with mv -n, so two concurrent makes cannot
# clobber each other: the first one wins and both end up using the same .env. A failed openssl
# leaves no .env behind. The loser's temp file stays (git-ignored by .env*).
.env:
	@umask 077; \
	tmp=$$(mktemp .env.XXXXXX) && \
	pw=$$(openssl rand -hex 16) && \
	printf 'POSTGRES_PASSWORD=%s\nDATABASE_URL=postgres://taskboard:%s@localhost:5432/taskboard\n' "$$pw" "$$pw" > "$$tmp" && \
	{ mv -n "$$tmp" .env || test -s .env; }

build: env ; docker compose build api web

up:    env ; docker compose up -d db
down:  env ; docker compose down

api-fmt:        env ; $(API) cargo fmt --all
api-fmt-check:  env ; $(API) cargo fmt --all --check
api-lint:       env ; $(API) cargo clippy --workspace --all-targets -- -D warnings
api-test:       env ; docker compose run --rm api cargo test --workspace
api-build:      env ; $(API) cargo build --workspace
api-deny:       env ; $(API) cargo deny check
# Rebuilds and restarts the API on every save under crates/ or in a Cargo.toml.
api-dev:        env ; docker compose run --rm --service-ports api \
                      watchexec --restart --watch crates --watch Cargo.toml --exts rs,toml -- cargo run -p taskboard-api
api-shell:      env ; docker compose run --rm api bash

web-install:       env ; $(WEB) 'pnpm install --frozen-lockfile'
web-dev:           env ; docker compose run --rm --service-ports web sh -c 'pnpm install && pnpm dev --host'
web-format-check:  env ; $(WEB) 'pnpm format:check'
web-lint:          env ; $(WEB) 'pnpm lint'
web-typecheck:     env ; $(WEB) 'pnpm typecheck'
web-test:          env ; $(WEB) 'pnpm test'
web-build:         env ; $(WEB) 'pnpm build'
web-shell:         env ; docker compose run --rm web sh

# The same gates CI runs, in the same order.
check: api-fmt-check api-lint api-test api-build api-deny \
       web-install web-format-check web-lint web-typecheck web-test web-build

hooks: ; git config core.hooksPath .githooks

clean: env ; docker compose down -v
