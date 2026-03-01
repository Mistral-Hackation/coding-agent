# API Reference

API documentation is generated from Rustdoc.

## Generate local API docs

```bash
cargo doc --no-deps
```

Open:

```text
target/doc/build123d_cad/index.html
```

## Key public modules

- `build123d_cad::agentic`
- `build123d_cad::telemetry`
- `build123d_cad::infra`
- `build123d_cad::types`
- `build123d_cad::viewer`

## Runtime examples

```bash
cargo run -- "Create a parametric flange with bolt holes"
cargo run --example agentic_workflow
cargo run --example otel_smoke
```
