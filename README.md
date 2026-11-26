# Hades

An open-source, local-first secret manager for macOS. Keep passwords, API keys, SSH keys, notes and files in an encrypted vault on your Mac.

> Early release (0.1). Not independently audited and not yet notarized by Apple. Read the install steps below.

## Features

- One encrypted vault, opened with a master password, a recovery key, or Touch ID.
- Passwords, API keys, SSH keys, notes and secure files, organized in folders.
- Locks itself when idle, on sleep, on screen lock and when you switch users.
- Copied secrets are cleared from the clipboard after 30 seconds.
- Optional sync between Macs through any folder (iCloud Drive, Dropbox, Syncthing). The folder only ever holds encrypted files, and edits made on two Macs at once are kept as conflict copies.
- No account and no tracking. Nothing leaves your Mac unless you turn on folder sync.

## Install

Hades is not notarized yet, so macOS shows a warning the first time you open it. This is expected for apps without an Apple Developer ID.

1. Download the latest `.dmg` from the [Releases page](https://github.com/runworkflow/hades/releases).
2. Open it and drag **Hades** into **Applications**.
3. Open Hades. macOS says it cannot verify the app. Click **Done**.
4. Open **System Settings > Privacy & Security**, scroll down, click **Open Anyway** next to Hades, and confirm.

On macOS 13 and 14, steps 3 and 4 are simpler: right-click Hades, choose **Open**, then **Open** again.

Prefer the Terminal? This does the same as steps 3 and 4:

```sh
xattr -dr com.apple.quarantine /Applications/Hades.app
```

Needs macOS 13 or newer. The download is built for Apple Silicon. On an Intel Mac, build from source.

## Build from source

You need macOS 13 or newer, the Xcode Command Line Tools (`xcode-select --install`), Rust, Node.js 22 or newer and pnpm.

```sh
pnpm install
pnpm tauri dev
pnpm release
```

`pnpm tauri dev` runs the app for development. `pnpm release` builds the app and the `.dmg` into `target/release/bundle`.

## How your secrets are protected

- Your master password goes through Argon2id (256 MiB) to unlock a random vault key.
- Every item has its own key, and everything is encrypted with XChaCha20-Poly1305.
- Touch ID uses a key in the Secure Enclave. Your recovery key is a second way in, and it is shown only once.
- If you forget both the password and the recovery key, your data cannot be recovered. Nobody can reset it.

Hades has not had an independent security audit yet. Please read [SECURITY.md](SECURITY.md).

## Contributing

Bug reports and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## Credits

Icons are the Solar Icons Set by [480 Design](https://www.figma.com/community/file/1166831539721848736), used under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). See [NOTICE](NOTICE).

## License

Apache License 2.0. See [LICENSE](LICENSE).
