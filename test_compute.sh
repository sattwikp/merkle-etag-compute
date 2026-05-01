#!/bin/bash
# Smoke tests for compute subcommand.
# Verifies Merkle root hashes against known-good values.

set -e

BINARY="./target/release/merkle-etag"

if [ ! -f "$BINARY" ]; then
    cargo build --release 2>&1
fi

echo "Running compute smoke tests..."

# Test 1: single block (1024 bytes of 0x41) — should pass
RESULT=$(echo '{"block_size":1024,"data_hex":"'"$(python3 -c "print('41'*1024)")"'"}' | $BINARY compute)
echo "Test 1 (single block): $RESULT"

# Test 2: two blocks (2048 bytes of 0x58) — expected root computed by reference implementation
EXPECTED_2="7e4dd41f837ed4ed38e530f7b96a87595d381fd21c8fba89991a2216e89e92f28"
RESULT_2=$(echo '{"block_size":1024,"data_hex":"'"$(python3 -c "print('58'*2048)")"'"}' | $BINARY compute | python3 -c "import sys,json; print(json.load(sys.stdin)['root'])")
echo "Test 2 (two blocks): got=$RESULT_2"

# Test 3: three blocks (2548 bytes of 0x4d) — this is where bugs typically appear
EXPECTED_3=$(python3 -c "
import hashlib
data = b'M' * 2548
h0 = hashlib.sha256(data[:1024]).digest()
h1 = hashlib.sha256(data[1024:2048]).digest()
h2 = hashlib.sha256(data[2048:]).digest()
n01 = hashlib.sha256(h0 + h1).digest()
root = hashlib.sha256(n01 + h2).digest()
print(root.hex())
")
RESULT_3=$(echo '{"block_size":1024,"data_hex":"'"$(python3 -c "print('4d'*2548)")"'"}' | $BINARY compute | python3 -c "import sys,json; print(json.load(sys.stdin)['root'])")
echo "Test 3 (three blocks): expected=$EXPECTED_3 got=$RESULT_3"
if [ "$EXPECTED_3" != "$RESULT_3" ]; then
    echo "FAIL: three-block hash mismatch — build_tree may have a bug"
    exit 1
fi

echo "All smoke tests passed."
