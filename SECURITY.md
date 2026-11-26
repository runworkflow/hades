# Security

## Reporting a vulnerability

Please report security problems privately, not in a public issue.

Use GitHub's private reporting: open the **Security** tab of this repository and choose **Report a vulnerability**.

Include what you found, the steps to reproduce it, and the version of Hades you used. Please do not share real secrets or real vault files.

We will acknowledge your report as soon as we can, work on a fix, and credit you in the release notes if you want.

## Supported versions

Only the latest release gets security fixes.

## Good to know

- Hades has not had an independent security audit yet.
- Release builds are not notarized by Apple yet.
- The vault lives on your Mac. Anyone with your master password or recovery key can open it.
- If you use folder sync, anyone who can read the sync folder holds your encrypted vault and could try to guess your master password offline. Use a long, unique password.
- A forgotten master password and recovery key cannot be reset.
