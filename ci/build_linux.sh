#!/bin/bash
set -euo pipefail
cd "$(dirname "$(realpath "$0")")/.."

# ----------------------------------------
# Build Python wheels using uv for multiple Python versions
# Assumes running inside a manylinux Docker container
# e.g. quay.io/pypa/manylinux_2_28_x86_64.
# ----------------------------------------

PY_VERSIONS=(
  python3.8
  python3.9
  python3.10
  python3.11
  python3.12
  python3.13
)

echo "🔧 Building wheels for Python versions: ${PY_VERSIONS[*]}"
for PY in "${PY_VERSIONS[@]}"; do
  echo "▶ Building for $PY..."
  uv build --python "$PY"
done

# ----------------------------------------
# Repair wheels with auditwheel to ensure manylinux compatibility
# ----------------------------------------

echo "🛠️ Repairing built wheels with auditwheel..."
for whl in dist/*.whl; do
  echo "▶ Repairing $whl"
  auditwheel repair "$whl" -w wheelhouse/
done

# ----------------------------------------
# Copy source distribution (.tar.gz) to wheelhouse/
# ----------------------------------------

echo "📦 Copying source distributions..."
cp dist/*.tar.gz wheelhouse/

echo "✅ Build complete. Files are in ./wheelhouse/"