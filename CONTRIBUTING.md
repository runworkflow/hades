# Contributing

Thanks for helping. For anything bigger than a small fix, please open an issue first so we can agree on the approach.

## Set up

You need macOS 13 or newer, the Xcode Command Line Tools, Rust, Node.js 22 or newer and pnpm.

```sh
pnpm install
pnpm tauri dev
```

## Before you open a pull request

Run all of these and make sure they pass:

```sh
pnpm typecheck
pnpm lint
pnpm build
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
```

## Style

- Plain, full-word names. Booleans read as yes/no questions, functions as verb phrases.
- Keep files under about 250 lines. Files and folders are kebab-case, React components PascalCase, variables and functions camelCase.
- The code has no comments. Rename or restructure instead.
- Format only the files you edited with Prettier.
- Add a new dependency only with a clear reason in the pull request.
- Commit messages are one line, lowercase, in conventional style, for example `fix: keep focus after unlock`.

## Security changes

Anything that touches encryption, key handling, the clipboard or the sync folder needs extra care. Describe the risk and how you checked it in the pull request. Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).
