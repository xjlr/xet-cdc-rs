# xet-cdc-rs

Rust implementation of Xet content-defined chunking, focused on exact CDC compatibility and systems programming practice.

This repository is a focused Rust learning and portfolio project based on the existing C++ implementation in `xjlr/xet-cdc-lab`.

The goal is not to port the entire C++ project. The intended scope is:

- GearHash
- ChunkBoundary
- streaming Chunker with feed/finish semantics
- exact reproduction of Xet CDC boundaries
- keyed BLAKE3 chunk hashing
- Xet hash representation
- reference manifest parsing
- validation against the official reference data

Final validation target: 796/796 matching chunks.

Out of scope: deduplication experiments, benchmarks, CLI polish, and unrelated utilities.
