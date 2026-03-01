# build123d CAD Code Generator — Justfile
# Run with: just <recipe>

# Default recipe: run verification
default: verify

# ─── Build & Verify ──────────────────────────────────────────────────

# Full verification (mimics CI)
verify: fmt clippy test build
	@echo "✅ Verification Passed!"

# Check formatting
fmt:
	echo "🎨 Checking Formatting..."
	cargo fmt -- --check

# Check lints
clippy:
	echo "📎 Checking Lints..."
	cargo clippy -- -D warnings

# Run tests
test:
	echo "🧪 Running Tests..."
	cargo test

# Build release
build:
	echo "🏗️  Building Project..."
	cargo build --release

# Quick check (no binary)
check:
	cargo check

# ─── Examples ────────────────────────────────────────────────────────

# Generate a simple box with hole (quick test)
box:
	cargo run -- "Create a simple box 50x50x30mm with a centered circular hole of 10mm diameter"

# Generate a parametric flange
flange:
	cargo run -- "Create a parametric flange with 150mm outer diameter, 80mm bore, 20mm thickness, and 8 equally spaced M10 bolt holes on a 120mm PCD"

# Generate a bearing housing
bearing:
	cargo run -- "Create a parametric bearing housing with 4 bolt mounting pattern, 50mm bore diameter, and integrated cooling fins"

# Generate a hexagonal nut
nut:
	cargo run -- "Create a standard M12 hexagonal nut with correct thread dimensions"

# Generate a rectangular plate with slots
plate:
	cargo run -- "Create a rectangular plate 200x100x10mm with two parallel slots 60x8mm centered along the length"

# Generate a pipe coupling
coupling:
	cargo run -- "Create a pipe coupling joint for 2 inch NPS pipes with flanged ends"

# Run with a custom prompt
run prompt:
	cargo run -- "{{prompt}}"

# Run the agentic workflow example
example:
	echo "🚀 Running Agentic Workflow..."
	cargo run --example agentic_workflow

# ─── Viewer ──────────────────────────────────────────────────────────

# Open the 3D viewer for the latest output
view:
	#!/usr/bin/env bash
	latest=$(ls -td .output/*/ 2>/dev/null | head -1)
	if [ -z "$latest" ]; then
	    echo "❌ No output directory found. Run an example first."
	    exit 1
	fi
	viewer="$latest/viewer.html"
	if [ -f "$viewer" ]; then
	    echo "🌐 Opening $viewer"
	    open "$viewer"
	else
	    echo "❌ No viewer.html found in $latest"
	    echo "   Available files:"
	    ls "$latest"
	fi

# ─── Utilities ───────────────────────────────────────────────────────

# Clean all output directories
clean:
	rm -rf .output/

# Show project stats
stats:
	@echo "📊 Project Statistics"
	@echo "─────────────────────"
	@echo "Rust source files: $(find src -name '*.rs' | wc -l | tr -d ' ')"
	@echo "Lines of Rust:     $(find src -name '*.rs' | xargs wc -l | tail -1 | awk '{print $$1}')"
	@echo "Knowledge files:   $(find docs/knowledge -name '*.md' | wc -l | tr -d ' ')"
	@echo "Output runs:       $(ls -d .output/*/ 2>/dev/null | wc -l | tr -d ' ')"

# Run GitHub Actions locally via Docker
act:
	echo "🐳 Running GitHub Actions locally..."
	act --container-architecture linux/amd64 -P ubuntu-latest=node:16-bullseye -v
