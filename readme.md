A `mdbook` processor that handles wiki-style links, e.g. `[[Some Title]]`, which will link to a chapter named `Some Title.md` if it exists, regardless of its subdirectory.

Notes:

- Ignores matching syntax in code blocks.
- Assumes that filenames are globally unique, e.g. there isn't both `./Some Title.md` and `subdir/Some Title.md`.
