# Development Guide

## Adding New Snippet

Visit the previous version of [30-seconds][30-seconds-previous] repository.
It is more readable and easier to find things. Then find missing function to implement.

Frequently used documentations:

- [Iterator in std::iter][iter]
- [slice - Rust][slice]
- [Itertools - Rust][itertools]

## Release Checklist

- Ensure local `master` is up to date to `origin/master`.
- Run `just up` to check outdated dependencies. Run `just up --write` and review dependency updates.
  Commit updated `Cargo` files.
- Run `just check`. To make sure that everything is ok.
- Run the release task `just release v<major.minor.path>`. Such `just release v0.1.7`.
- **Push the release commit to GitHub**, NOT including the tag. (But do not publish a new version of dryip to crates.io yet.)
- Once CI for `master` finishes successfully, **push the version tag**.
  (Trying to do this in one step seems to result in GitHub Actions not seeing the tag
  push and thus not run the release workflow.)
- Wait for CI to finish creating the release. If the release build fails, then
  delete the tag from GitHub, make fixes, re-tag, delete the release, and push.
- Copy the relevant section of the CHANGELOG.md to the tagged release notes.
- Run `cargo publish`.

## Commit Message Format

This repository is using [Agular's commit message format][commit-message]

<!-- dprint-ignore-start -->

[30-seconds-previous]: https://github.com/30-seconds/30-seconds-of-python/tree/e6064b1236bbe64f0a7f4e1127223b75848b92e8
[iter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[slice]: https://doc.rust-lang.org/std/primitive.slice.html
[itertools]: https://docs.rs/itertools/latest/itertools/trait.Itertools.html
[commit-message]: https://github.com/angular/angular/blob/2095a08781167e91a60a4cec65c694688b319cd0/CONTRIBUTING.md#-commit-message-format

<!-- dprint-ignore-end -->
