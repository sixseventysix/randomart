# randomart

Generates images from strings using a randomly grown expression tree.

## Crates

### `engine`
The pipeline:
- `Op`: one node of an expression (`X`, `Y`, `Const`, `Sin`, `Add`, ...)
- `Grammar`: a probabilistic context-free grammar. It is an iterator: seeded with a
  random stream, it walks the grammar depth-first and yields `Op`s in prefix order.
  Collecting it gives one tree as a flat `Vec<Op>`.
- `seed`: turns a string into three seeds, one per colour channel, and so into three trees
- `Backend`: the trait every backend implements
- `PixelBuffer`: flat RGB image buffer

### Backends
Execution backends. Each one does exactly one thing: take three `Vec<Op>` trees
(red, green, blue) and return a `PixelBuffer`. A backend reads each tree back to
front with a stack, so every op's inputs are ready before the op itself.

- **`closure-tree`**: builds each tree into a tree of Rust closures

`cranelift-backend` (JIT to native code) and `metal` (GPU) are still in the repo but
not part of the build until they are ported to `Vec<Op>`.

> The closure backend uses the CORE-MATH project for its math implementations of functions not guaranteed by IEEE 754 to be correctly rounded.

### `cli`
Owns all I/O. Parses CLI arguments, invokes a backend, and saves the resulting `PixelBuffer` as a PNG. Optionally writes the formula as JSON.

## Usage

The CLI builds a single `randomart` binary, using the closure backend.

```sh
cargo build --release
```

Generate an image from a string seed:

```sh
./randomart generate "hello world" 10
```

`depth` controls how deep the expression tree is allowed to grow. Higher depth means more complex images. Each step of the grammar (for example `C → sin C`, or `C → A`) uses one level.

Save the formula as JSON alongside the image:

```sh
./randomart generate "hello world" 10 --save-json
```

This writes a `.json` file next to the PNG: three lists of ops, one per colour
channel, in prefix order (for example `["Sin", "Add", "X", {"Const": 0.3}]`).
Files saved by older versions, which stored a nested tree, can't be read. You can re-render from it later:

```sh
./randomart read formula.json
```

Other options for `generate`:

```
--width <WIDTH>    Image width in pixels  [default: 512]
--height <HEIGHT>  Image height in pixels [default: 512]
--out <OUT>        Output filename stem   [default: the input string]
```

Output is always written to the current working directory. Pass `--help` to any binary or subcommand for full usage.
