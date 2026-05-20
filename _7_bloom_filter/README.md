# 🌸 Bloom Filters

Bloom filter is a sapce-efficient, probabilistic data structure based on hashing. The filter uses a bit array that stores the hashed *item* on multiple bit positions. It gaurantees, 

* **No false negatives**: If the filter says an item is absent, then it is definitely *absent*.
* **Prabable false positives**: If the filter says an item is present, chances are item may or may not be present. The probability is tunable and typically is set below 1%.

---
## Bloom filter formulas

To build a bloom filter, 

* A bit array of *`m`* bits, all initialised to `0`.
* *`k`* independent hash functions, each mapping any element to a position in `[0, m−1]`.

To compute *`m`* and *`k`*,


###### 1. Bit Array Size (`m`)

\[
m = -\frac{n \ln(p)}{(\ln 2)^2}
\]

###### 2. Number of Hash Functions (`k`)

\[
k = \frac{m}{n} \ln(2)
\]

Where,

| Symbol | Description |
|--------|-------------|
| `m` | Size of the bit array |
| `n` | Number of expected elements |
| `p` | Desired false positive probability |
| `k` | Number of hash functions |

---

## Algorithm

#### Inserting an element

1. Compute `h₁(x), h₂(x), … hₖ(x)`.
2. Set `bit_array[hᵢ(x)] = 1` for each i.

#### Querying an element

1. Again compute the same k positions.
2. If all bits are set to `1` → element is probably present in the set.
3. If any bit is `0` → element is definitely absent.

---

## Implementation

1. **[Bloom Filter v1](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v1)** - A simple implementation in Python.

2. **[Bloom Filter v2](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v2)** - Basic implementation in Rust.

3. **[Bloom Filter v3](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v3)** - Implementation in Rust with sharing ownership and mutation across threads.

4. **[Bloom Filter v4](https://github.com/abhilashmendhe/backend_projects/tree/main/_7_bloom_filter/bf_v4)** - Implementation in Rust using tokio-channels using Actor-style pattern.


---

## References
1. <a id="ref-1"></a> *Bloom Filter*. URL: https://brilliant.org/wiki/bloom-filter

1. <a id="ref-2"></a>Bloom Filters Explained: A Guide to Probabilistic Data Structures*. URL: https://medium.com/@bishal.khanal.2057/bloom-filters-explained-a-guide-to-probabilistic-data-structures-b736476135cf
