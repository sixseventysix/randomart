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

### `randomart-metal` / `randomart-cranelift-jit` / `randomart-closure-tree`
Execution backends. Each one does exactly one thing: take an AST and return a `PixelBuffer`.

- **metal**: compiles the AST to Metal Shading Language and runs it on the GPU
- **cranelift-jit**: JIT-compiles the AST to native code via Cranelift
- **closure-tree**: interprets the AST as a tree of Rust closures

### `randomart-cli`
Owns all I/O. Parses CLI arguments, invokes a backend, and saves the resulting `PixelBuffer` as a PNG. Optionally writes the formula as JSON.

## Usage

Build a binary for the backend you want:

```sh
cargo build --release --bin randomart-metal
cargo build --release --bin randomart-cranelift
cargo build --release --bin randomart-closure-tree
```

Generate an image from a string seed:

```sh
./randomart-metal generate "hello world" 10
```

`depth` controls how deep the expression tree is allowed to grow. Higher depth means more complex images.

Save the formula as JSON alongside the image:

```sh
./randomart-metal generate "hello world" 10 --save-json
```

This writes a `.json` file next to the PNG. You can then re-render from it later:

```sh
./randomart-metal read formula.json
```

Other options for `generate`:

```
--width <WIDTH>    Image width in pixels  [default: 512]
--height <HEIGHT>  Image height in pixels [default: 512]
--out <OUT>        Output filename stem   [default: the input string]
```

Output is always written to the current working directory. Pass `--help` to any binary or subcommand for full usage.
