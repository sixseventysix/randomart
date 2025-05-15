# randomart spec lang
nested functions representation in this specific format:
```text
<function>  ( <arg1> <arg2> ... )
```
> note that tokenization is done by splitting on whitespace, so tokens `<function>`, `<arg>`, `(`, `)` need to be separated by at least one whitespace. the type of whitespace does not matter.

## node types
| Syntax                | Meaning / Operation                                      | Arity |
|-----------------------|----------------------------------------------------------|-------|
| `x`                   | Returns the x-coordinate                                 | 0     |
| `y`                   | Returns the y-coordinate                                 | 0     |
| `const_ v`            | Constant float value `v`                                 | 1     |
| `add  a b`            | Computes the average: `(a + b) / 2`                      | 2     |
| `mult  a b`           | Computes the product: `a * b`                            | 2     |
| `div  a b`            | Computes division: `a / b` with safe fallback to 0       | 2     |
| `sin  a`              | Computes sine of `a`                                     | 1     |
| `cos  a`              | Computes cosine of `a`                                   | 1     |
| `exp  a`              | Computes exponential: `e^a`                              | 1     |
| `sqrt  a`             | Computes square root of `a`, clamps to 0 if negative     | 1     |
| `mixu  a b c d`       | Unbounded version: `(a*c + b*d) / (a + b + epsilon)`     | 4     |
| `triple  a b c` *     | Groups three channels R, G,                              | 3     |
| `rule  n` *           | Refers to grammar rule number `n`                        | 1     |
| `random` *            | Placeholder, replaced by a constant in `[-1, 1]`         | 0     |

> those marked with * are only used in tree generation, not in evaluation

## examples
### a full tree
```text
triple (
    add ( 
        x 
        y 
    )
    mult ( 
        sin ( 
            x
        ) 
        cos ( 
            y 
        ) 
    )
    const_ ( 
        0.5 
    )
)
```
this represents an rgb function where:\
r = average of x and y\
g = product of sin(x) and cos(y)\
b = constant 0.5

### sliced from the middle of the tree
```text
mult ( 
    sin ( 
        sqrt ( 
            x
        ) 
    ) 
    sin ( 
        sin ( 
            x
        ) 
    ) 
)
``` 

this does the following operation:
`mult(sin(sqrt(x)), sin(sin(x)))`