# Development Guide

## Adding New Snippet

Visit the previous version of [30-seconds][30-seconds-previous] repository.
It is more readable and easier to find things. Then find missing function to implement.

Frequently used documentations:

- [Iterator in std::iter][iter]

## Release Checklist

- Run `just check`.
- Run the release task: `just release v<major.minor.path>`. Such `just release v0.1.7`.
- Check if [Continuous Integration][ci] workflow is completed successfully.
- Push the tags: `git push --tags`

<!-- dprint-ignore-start -->

[30-seconds-previous]: https://github.com/30-seconds/30-seconds-of-python/tree/e6064b1236bbe64f0a7f4e1127223b75848b92e8
[iter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

[ci]: https://github.com/azzamsa/dryip-of-rust/actions/workflows/ci.yml
[cd]: https://github.com/azzamsa/dryip-of-rust/actions/workflows/cd.yml

<!-- dprint-ignore-end -->
