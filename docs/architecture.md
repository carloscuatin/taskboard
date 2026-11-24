# Taskboard — architecture

A multi-user API for projects and tasks. Users register, log in and belong to projects with a role
(`owner`, `editor` or `viewer`); a global `admin` manages users. The project exists to learn Rust
(axum, sqlx) and Vue 3, and to practise authentication and authorization end to end.

## Current status

The four API crates and the Vue app build, and every gate passes. The domain has its first value
object, `Email`, which can only be obtained by parsing text.

## Decisions

### 1. Stack and layers

**Backend.** Rust (edition 2024) with axum on tokio, sqlx on Postgres, argon2 for passwords,
jsonwebtoken for access tokens and utoipa for the OpenAPI contract. **Frontend.** Vue 3 with Vite as
a static SPA, Pinia, Vue Router and TanStack Query, in a separate `web/` directory of the same
repository, so a contract change and its client land in one pull request.

**Layers are crates.** `domain` has no workspace dependencies; `application` (use cases and ports)
depends on `domain`; `adapters` (Postgres, hashing, tokens) depends on `application`; `api` (HTTP and
wiring) depends on both. The compiler rejects an import against the arrow. `cargo deny` closes the
other gap: `sqlx`, `argon2`, `jsonwebtoken`, `axum`, `utoipa` and `tokio` may be dependencies of the
outer crates only.

**Everything runs in Docker.** Rust, Node and Postgres never run on the host, because build scripts
and procedural macros of third-party crates would otherwise execute on it. Build output and
dependency caches live in named volumes.

Discarded: Actix and Rocket (axum is the tower-based standard and composes with its middleware);
Nuxt (a Node server layer would duplicate what the API does); micro frontends (one developer, one
product); Husky and commitlint on the host (they need Node outside Docker, so commit messages are
validated in CI and the local hook only checks formatting).

### 2. Authentication and authorization

- Passwords are hashed with argon2id and never stored or logged in clear text.
- A short-lived access token (JWT, 15 minutes) and an opaque refresh token, stored hashed and
  rotated on every use. Reusing a rotated token revokes the whole token family.
- Tokens travel in `httpOnly`, `Secure`, `SameSite=Lax` cookies. Requests that change state also
  need an anti-CSRF header.
- Login and registration are rate limited, and their errors never reveal whether an email exists.
- Authorization is decided by the user's role in the project of the resource (`owner`, `editor`,
  `viewer`); the global `admin` role only manages users.
- Input is validated at the edge, and domain errors are translated to HTTP without leaking
  internals.
- The identity of a user is their email address in its normalized form: trimmed and lowercased as
  ASCII. Internationalized domains are accepted in their `xn--` form only.
