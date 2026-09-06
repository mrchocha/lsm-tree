# LSM Tree (WIP)

A **Log-Structured Merge (LSM) Tree implementation in Rust**, built while studying LSM Trees and their underlying storage concepts.

The goal of this project is to understand how modern storage engines use **WALs, MemTables, Bloom Filters, SSTables, and compaction** to efficiently handle writes and persistent storage.

## Features

* [x] WAL persistence
* [ ] WAL file chunking / rotation
* [x] MemTable implementation
  * [X] Btree based
  * [ ] Skip list based
* [x] Bloom filter
* [ ] SSTable persistence
* [ ] SSTable merge / compaction
* [ ] Delete / tombstone handling

## Reference

**The Log-Structured Merge-Tree (LSM-Tree)**
Patrick O'Neil, Edward Cheng, Dieter Gawlick, Elizabeth O'Neil — *Acta Informatica, 1996*

[Read the original LSM-Tree paper](https://dsf.berkeley.edu/cs286/papers/lsm-acta1996.pdf)

Built with ❤️ by Rahul
