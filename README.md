# Advent of Code 2024

My advent of code completion in 2024.

## Objectives

- 🎯 Complete all 25 days of the advent of code.
- 🧮 Use a Rusty approach with extensive use of iterators and functional programming unless it affects readability in some instances.
- 🛠️ Use only the standard library and [itertools](https://crates.io/crates/itertools) (Edit: and other few other essential ones, see notes below).

**Notes**:
- On day 8, I realized linear algebra operations are quite frequently used, so I added [glam](https://crates.io/crates/glam) to the list of dependencies.
- On day 13, I added [nom](https://crates.io/crates/nom) to the list of dependencies to parse the input easier (and to learn something new).

## Rought Works

There are some rough works in the [rough_works](./rough_works) directory. These are some handwritten notes to help me think through the problems. The file names are in the format `<day>_<name>.png`.

The drawings are drawn in MS Paint with my mouse, so they are not the most beautiful things in the world, but that's how I like to jot down my thoughts :P.

## Environment Setup

Copy the `.env.example` file to `.env` and fill in your `SESSION`cookie from the advent of code website.

## Usage

To run different days, use the following command:

```bash
cargo run --bin <day>
```

For example, for day 1:

```bash
cargo run --bin 1
```