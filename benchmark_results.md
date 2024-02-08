# Benchmark Results

## Rust Only
| Date | Change | AH | Single Ref |
|------|--------|----|------------|
| 2024-02-08 | Handwritten Init |  24.491 ms  | 3.5846 µs |
| 2024-02-08 | Hashmap with capacity 30 | 23.670 ms | 3.1250 µs |
| 2024-02-08 | Also shrink_to_fit | 20.154 ms | 3.3157 µs | 
| 2024-02-08 | Vector with capacity 5000 | 23.987 ms | 3.6744 µs |
| 2024-02-08 | Hashmap with capacity 20 | 18.208 ms | 2.9724 µs |



### Capacity tests (2024-02-08)
- Giving the hashmap enough capacity speeds up parsing.
- Removing unused hashmap memory with `shrink_to_fit` makes single reference slower, but a full file faster. Probably because less memory needs to be moved around in the full file version.
- Also giving the vector of references enough capacity seems to make the parsing slower. I guess because more memory is necessary at the start.
- This depends on the capacity we give the hashmap. If it's too much, there will be unnecessary memory allocated, if it's too little there will be extra allocations. There seems to be a perfect middle ground.

It seems useful to make sure the hashmap has sufficient capacity but is then shrunk back to actual size. Some trial and error leads to 20 being the right capacity, at least for the Appenzeller-Herzog file.