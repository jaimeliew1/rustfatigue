#!/bin/bash
set -euo pipefail

echo "⚙️ Installing Rust toolchain via rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Change to parent directory of script
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
  python3.14
)

echo "🔧 Building and testing wheels for Python versions: ${PY_VERSIONS[*]}"
for PY in "${PY_VERSIONS[@]}"; do
  echo "▶ Building for $PY..."
  uv build --python "$PY"

  echo "🧪 Testing wheel for $PY..."
  WHEEL=$(ls -t dist/*.whl | head -n1)
  TEST_VENV=$(mktemp -d)
  uv venv --python "$PY" "$TEST_VENV"
  uv pip install --python "$TEST_VENV/bin/python" "$WHEEL" pytest

  # copy the test file to a clean dir so the local rustfatigue/ source
  # doesn't shadow the installed wheel when imported
  TEST_DIR=$(mktemp -d)
  cp rustfatigue/tests/rustfatigue_test.py "$TEST_DIR/"
  (cd "$TEST_DIR" && "$TEST_VENV/bin/python" -m pytest . -q)

  rm -rf "$TEST_VENV" "$TEST_DIR"
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