# randomart

Generates images from strings using a randomly grown expression tree.

## Crates

### `randomart-core`
Common types and algorithms shared across the whole project:
- `Node`: the AST
- `Grammar`: probabilistic tree generation
- `PixelBuffer`: flat RGB image buffer
- `Statistics`: tree analysis
- `Rng`: seeded random number generation

### `randomart-{backend}`
Execution backends. Each one does exactly one thing: take an AST and return a `PixelBuffer`.

- **metal**: compiles the AST to Metal Shading Language and runs it on the GPU
- **cranelift-jit**: JIT-compiles the AST to native code via Cranelift
- **closure-tree**: interprets the AST as a tree of Rust closures
- **llvm-aot**: Uses the Rust build system to generate the AST as Rust native code, compiles, and runs it.

> The CPU backends use the CORE-MATH project for their math implementations of functions not guaranteed by IEEE 754 to be correctly rounded. The Metal backend currently does not support this because it doesn't have native support of `f64`. The Metal output may not be bit-identical to the CPU output.

### `randomart-cli`
Owns all I/O. Parses CLI arguments, invokes a backend, and saves the resulting `PixelBuffer` as a PNG. Optionally writes the formula as JSON.

## Usage

The CLI builds a single `randomart` binary. The backend is chosen at build time
via a Cargo feature, so only that backend's dependencies are compiled in.
`closure` is the default; `metal` is macOS-only.

```sh
cargo build --release                                          # closure-tree (default)
cargo build --release --no-default-features --features cranelift
cargo build --release --no-default-features --features metal    # macOS only
```

Generate an image from a string seed:

```sh
./randomart generate "hello world" 10
```

`depth` controls how deep the expression tree is allowed to grow. Higher depth means more complex images.

Save the formula as JSON alongside the image:

```sh
./randomart generate "hello world" 10 --save-json
```

This writes a `.json` file next to the PNG. You can then re-render from it later:

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
