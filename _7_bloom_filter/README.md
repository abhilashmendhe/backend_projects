# 🌸 Bloom Filters

Bloom filter is a space-efficient, probabilistic data structure based on hashing. The filter uses a bit array that stores the hashed *item* on multiple bit positions. It guarantees, 

* **No false negatives**: If the filter says an item is absent, then it is definitely *absent*.
* **Possible false positives**: If the filter says an item is present, the item may or may not actually be present. The probability is tunable and typically is set below 1%.

---
## Bloom filter formulas

To build a bloom filter, 

* A bit array of *`m`* bits, all initialized to `0`.
* *`k`* independent hash functions, each mapping any element to a position in `[0, m−1]`.

To compute *`m`* and *`k`*,


##### 1. Bit Array Size (`m`)


```text
m = -(n × ln(p)) / (ln(2))²
```

##### 2. Number of Hash Functions (`k`)

```text
k = (m / n) × ln(2)
```

Where,

| Symbol | Description |
|--------|-------------|
| `m` | Size of the bit array |
| `n` | Number of expected elements |
| `p` | Desired false positive probability |
| `k` | Number of hash functions |

The size of the bit array `(m)` determines the memory usage, while the number of hash functions `(k)` determines the trade-off between insertion cost and false positive probability. Choosing these values using the formulas below minimizes the false positive rate for a given number of expected elements.

---

## Algorithm

#### Inserting an item

1. Compute `h₁(x), h₂(x), … hₖ(x)`.
2. Set `bit_array[hᵢ(x)] = 1` for each i.

#### Querying an item

1. Again compute the same k positions.
2. If all bits are set to `1` → item is probably present in the set.
3. If any bit is `0` → item is definitely absent in the set.

#### Example
```
Insert:
apple
banana

Bit Array
0010101010010100

Query:
apple   -> Probably Present
orange  -> Definitely Absent
```

---

## Limitations

Bloom filters cannot:

- Remove elements (standard Bloom filter)
- Recover stored elements
- Eliminate false positives
- Grow automatically without rebuilding

---

## Implementation Variants

This repository contains multiple implementations demonstrating different design approaches.

| Version | Language | Focus |
|---------|----------|-------|
| v1 | Python | **[Basic implementation](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v1)** |
| v2 | Rust | **[Idiomatic Rust](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v2)** |
| v3 | Rust | **[Thread-safe shared ownership](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v3)** |
| v4 | Rust | **[Actor model using Tokio channels](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v4)** |

---

## References

1. <a id="ref-1"></a> Burton H. Bloom, *[Space/Time Trade-offs in Hash Coding with Allowable Errors](https://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf)* (1970).

1. <a id="ref-2"></a> Brilliant.org - *[Bloom Filters](https://brilliant.org/wiki/bloom-filter)*.

1. <a id="ref-3"></a> Medium.com - *[Bloom Filters Explained: A Guide to Probabilistic Data Structures](https://medium.com/@bishal.khanal.2057/bloom-filters-explained-a-guide-to-probabilistic-data-structures-b736476135cf)*.
