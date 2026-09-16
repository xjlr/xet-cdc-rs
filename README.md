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

## Reference data

The boundary-compatibility test uses the published Hugging Face Xet reference pair. Download both files into `reference-data/`:

```bash
hf download xet-team/xet-spec-reference-files \
  Electric_Vehicle_Population_Data_20250917.csv \
  Electric_Vehicle_Population_Data_20250917.csv.chunks \
  --repo-type dataset \
  --local-dir reference-data
```

The `reference-data/` directory is intentionally ignored by Git. If the files are absent, the full 796-boundary integration test returns without failing a clean checkout; the small manifest-shape test still exercises the test-only size parser.
