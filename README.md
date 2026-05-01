# merkle-etag

A Merkle-tree-based content hasher that produces a stable, content-based hash for data that may arrive in arbitrary parts.

## Build

```bash
cargo build --release
cp target/release/merkle-etag ./merkle-etag
```

## Usage

The binary accepts a subcommand and reads JSON from stdin.

### compute

Compute the Merkle root hash of data split into canonical blocks.

```bash
echo '{"block_size": 1024, "data_hex": "48656c6c6f"}' | ./merkle-etag compute
```

### stream-init / stream-feed / stream-finalize

Incremental streaming API for computing the Merkle root from parts that arrive out of order.

## Known Issues

- `compute` produces incorrect hashes for data spanning 3 or more canonical blocks. See `test_compute.sh` for a failing test case.
