# randomart
![image from randomart](./data/images/My_brain_on_drugs.png)

image generated using the string: `My brain on drugs` with depth = 40\
(sadly, this cannot be made anymore because i chose multi-threaded tree gen sadface)

using Metal GPU for creating the image. paying some overhead because of creating the .metal file, the binaries, and then executing them, but the actual execution is much faster than pulling any tricks on the cpu.
## generate mode
creates image from string
```bash
usage: cargo run --release -- generate <string> <depth>
```
takes string and maximum depth as inputs, outputs the image and its respective formula in a txt file

## read mode
reads the randomart spec lang and generates its corresponding image
```bash
usage: cargo run --release -- read <input>
```
takes file name as input. no need to mention ".txt" in the name
> this reads from .txt files strictly

## references:
https://netsec.ethz.ch/publications/papers/validation.pdf

https://www.youtube.com/watch?v=3D_h2RE0o0E

## gallery(built during testing phase(not reproducible))
![1](./data/images/141120240040.png)
![2](./data/images/141120240053.png)
![3](./data/images/141120240010.png)
![4](./data/images/131120242325.png)
![5](./data/images/141120240017.png)
![6](./data/images/141120242253.png)
![8](./data/images/spiderman.png)
![9](./data/images/spiderman_1.png)
![10](./data/images/spiderman_2.png)
![11](./data/images/spiderman3.png)