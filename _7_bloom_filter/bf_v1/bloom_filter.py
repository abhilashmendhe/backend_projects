import math 
import mmh3
import hashlib

class BloomFilter:

    def __init__(self, n: int, p: float):
        self.m = math.ceil(-1 * n * math.log(p) / (math.log(2)**2))
        self.barr = bytearray(self.m) 
        self.k = max(1, round((self.m / n ) * math.log(2)))

    def locations(self, item: str) -> list[int]:
        positions = []
        h1 = int(hashlib.sha1(item.encode()).hexdigest(), 16)
        h2 = mmh3.hash128(item)
        for i in range(self.k):
            positions.append((h1 + i * h2) % self.m)
        return positions
    
    def insert(self, item: str):
        for i in self.locations(item):
            self.barr[i] = 1

    def query(self, item: str):
        return all(self.barr[i] for i in self.locations(item))
    
bf = BloomFilter(10, 0.1)
bf.insert("apple")
bf.insert("bana")
print(bf.query("banana"))
print(bf.query("ba"))
print(bf.query("apple"))
print(bf.query("ap"))
print(bf.query("bana"))
print(bf.query("baa"))