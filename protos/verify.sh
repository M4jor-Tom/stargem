#!/usr/bin/env bash
set -euo pipefail

echo "Verifying proto files compile with protoc..."
for f in protos/grpc/*.proto protos/quic/*.proto; do
  echo "  Checking $f"
  protoc --proto_path=protos --include_imports -o /dev/null "$f"
done
echo "All proto files compile successfully."
