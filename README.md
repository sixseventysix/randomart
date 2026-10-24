# RandomArt
This repository has become so much more than just generating RandomArt images. I hope to use this as a workspace for experimenting with different compiler/PL techniques. The current implementation uses a VM to compute the image buffers and dispatches instructions for all pixel positions one at a time. The rendering code layout allows `rayon` to shard work by pixels across threads and to use SIMD inside a thread, although only for functions that can actually use SIMD. [Transcendental functions](https://en.wikipedia.org/wiki/Transcendental_function) utilise [CORE-MATH](https://core-math.gitlabpages.inria.fr) algorithms to achieve correctly-rounded floating point calculations (which I think will become the standard in the near future across compiler and hardware implementations). Some things I want to experiment with:
- [Copy-and-patch kernels](https://en.wikipedia.org/wiki/Copy-and-patch)
- Building fast, correctly-rounded kernels for Metal GPU (since that's the only machine I own)
- Faster, vectorizable implementations of the CORE-MATH algorithms
- [Catamorphic](https://en.wikipedia.org/wiki/Catamorphism) implementations of the passes done on the core RandomArt object. Mathematically speaking, the PCFG is a stream and expansions of the grammar can be defined as iterator functions.

> Catamorphism might be a subset of the things I am looking for. There is a lot to learn from functional programming paradigms here, and I lean towards implementing the RandomArt stream in that philosophy. A compiler that is just fold/unfold operations and their composition.
