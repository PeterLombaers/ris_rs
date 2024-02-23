# Benchmark Results

## Rust Only
| Date | Change | AH | Single Ref |
|------|--------|----|------------|
| 2024-02-08 | Handwritten Init |  24.491 ms  | 3.5846 µs |
| 2024-02-08 | Hashmap with capacity 30 | 23.670 ms | 3.1250 µs |
| 2024-02-08 | Also shrink_to_fit | 20.154 ms | 3.3157 µs | 
| 2024-02-08 | Vector with capacity 5000 | 23.987 ms | 3.6744 µs |
| 2024-02-08 | Hashmap with capacity 20 | 18.208 ms | 2.9724 µs |
| 2024-02-23 | Switch to iterator and bytearray | 32.977 ms | 4.6335 µs |
| 2024-02-23 | Use Rayon for parallelism | 21.466 ms | 475.59 µs |

## Python Bindings
| Date | Change | Rust | Python |
|------|--------|--------|------|
| 2024-02-08 | Initial Bindings (String Input) | 74.5673 | 463.6952 |
| 2024-02-23 | Switch to iterator and bytearray | 53.4637 | 413.2013 |
| 2024-02-23 | Use Rayon for parallelism | 38.9063 | 411.1623 |

## Remarks
### Capacity tests (2024-02-08)
- Giving the hashmap enough capacity speeds up parsing.
- Removing unused hashmap memory with `shrink_to_fit` makes single reference slower, but a full file faster. Probably because less memory needs to be moved around in the full file version.
- Also giving the vector of references enough capacity seems to make the parsing slower. I guess because more memory is necessary at the start.
- This depends on the capacity we give the hashmap. If it's too much, there will be unnecessary memory allocated, if it's too little there will be extra allocations. There seems to be a perfect middle ground.

It seems useful to make sure the hashmap has sufficient capacity but is then shrunk back to actual size. Some trial and error leads to 20 being the right capacity, at least for the Appenzeller-Herzog file.

### Switch to byte arrays and iterator
After the switch, parsing from rust is clearly slower, but parsing from Python is faster.
I'm not sure this is because I forgot to add the --release tag when developing the python
package with Maturin, or that it's really faster. One reason it could be faster is because
I can just read bytes in Python instead of a string. In any case, the switch will be worth it
because now we can use Rayon for parallelism, which is definitely benificial once we want
to make the parsing of the individual references more elaborate.

### Using Rayon for parellilism
This was amazingly easy and there is a nice speedup. Only in the case of an individual reference
did it get much slower. But that's to be expected because of the overhead of making a threadpool.