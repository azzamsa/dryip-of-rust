# Development Guide

## Adding New Snippet

Use the [previous][30-seconds-previous] version of the repository.
It is more readable and easier to find things.

Many of the snippets are one-liners.
Use [Iterator in std::iter][iter] documentation to find suitable function.

# Release Checklist

- Run the lint check: `make is_verified`.
- Run the release task: `make release version=v<mayor.minor.path>`. Such `make release version=v0.1.7`.
- Check if [Continuous Integration][ci] workflow is completed successfully.
- Push the tags: `git push --tags`
- Wait for [Continuous Deployment][cd] workflow to finish.
- Create a new GitHub release with the created tag above, and copy the release news from the CHANGELOG.md.

<!-- dprint-ignore-start -->

[30-seconds-previous]: https://github.com/30-seconds/30-seconds-of-python/tree/e6064b1236bbe64f0a7f4e1127223b75848b92e8
[iter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

[ci]: https://github.com/azzamsa/dryip-of-rust/actions/workflows/ci.yml
[cd]: https://github.com/azzamsa/dryip-of-rust/actions/workflows/cd.yml

<!-- dprint-ignore-end -->
