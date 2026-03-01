# Changelog

## [1.4.0] (2026-02-28)

### Features

* **observability:** Add OpenTelemetry bootstrap module with OTLP export and graceful shutdown
* **observability:** Add mission root spans, orchestration spans, and tool-level spans
* **observability:** Add `otel_smoke` example for collector connectivity checks

### Documentation

* **README.md:** Add SigNoz/local OTLP runbook and smoke command
* **docs/src:** Add observability guide and refresh intro/architecture/api/tutorial pages
* **ARCHITECTURE.md:** Add dedicated OpenTelemetry architecture and operations section

## [1.3.0] (2026-02-28)

### Features

* **pipeline:** Streamline to 7 agents — remove SupplyReviewer and OilGasReviewer
* **tools:** Add CodeSnippetSearch for pattern matching in generated code
* **tools:** Add CodeSnippetReplace for atomic search-and-replace on generated code
* **tools:** Add LinterTool (Python syntax validation) to all 4 reviewer agents
* **viewer:** Interactive 3D STL viewer (Three.js) — auto-generates HTML and opens in browser
* **exec:** Auto-execute generated scripts via `uv run --with build123d python3`
* **exec:** Code sanitization — strip trailing LLM commentary before saving scripts

### Bug Fixes

* **compliance:** Fix ComplianceReviewer infinite loop — robust APPROVED detection handles markdown/emoji formats
* **consensus:** Update consensus tracking from 6/6 to 4/4 reviewers
* **exec:** Fix double-nested output directory path in auto-execution

### Documentation

* **README.md:** Complete rewrite reflecting 7-agent pipeline, new tools, and 3D viewer
* **ARCHITECTURE.md:** Updated agent roster, consensus diagrams, tool tables, and auto-execution pipeline

## [1.2.0](https://github.com/hghalebi/-blender-rs-lib/compare/v1.1.0...v1.2.0) (2026-01-19)


### Features

* Migrate to Anthropic Claude via Azure MaaS ([09c574d](https://github.com/hghalebi/-blender-rs-lib/commit/09c574d6f4d6a15337c34919d54a750f93e0f776))

## [1.1.0](https://github.com/hghalebi/-blender-rs-lib/compare/v1.0.0...v1.1.0) (2026-01-19)


### Features

* Add CLI tutorial + migrate to Claude via Azure MaaS ([296cf2d](https://github.com/hghalebi/-blender-rs-lib/commit/296cf2df5f9235a667c3a2b676ee85dd154f66d4))

## [1.0.0](https://github.com/hghalebi/-blender-rs-lib/compare/v0.2.0...v1.0.0) (2026-01-16)


### ⚠ BREAKING CHANGES

* Requires CompletionClient and ProviderClient traits in scope.

### Code Refactoring

* upgrade rig-core to 0.28.0 ([73aa540](https://github.com/hghalebi/-blender-rs-lib/commit/73aa540fa759ee214b7caa87600c9b8b387d6ba2))

## [0.2.0](https://github.com/hghalebi/-blender-rs-lib/compare/v0.1.0...v0.2.0) (2026-01-16)


### Features

* **arch:** migrate from line-based surgery to search-and-replace ([820fa2f](https://github.com/hghalebi/-blender-rs-lib/commit/820fa2fc17b272c66dbc57b29ec7633d5dd8ed69))
* **arch:** migrate from line-based surgery to search-and-replace ([4c76c48](https://github.com/hghalebi/-blender-rs-lib/commit/4c76c4829ad0bdb87774c0cab4515962e35f387d))
* **cli:** enhance help output with examples and architecture details ([7bc1460](https://github.com/hghalebi/-blender-rs-lib/commit/7bc146041012bef5b3c9c3837a7cf72edcb1ca48))
* **infra:** implement pre-flight python syntax check ([cb68273](https://github.com/hghalebi/-blender-rs-lib/commit/cb682735cfc09a0de07cd169966b7ee8afc465a4))
* **infra:** implement pre-flight python syntax check ([ce8c727](https://github.com/hghalebi/-blender-rs-lib/commit/ce8c7276b8d3212abead836ae6d7e69310f65337))


### Bug Fixes

* **ci:** fix clippy errors and conditional imports ([1a86858](https://github.com/hghalebi/-blender-rs-lib/commit/1a868585e25a6170310788556fa3ce2f685133a9))
* **ci:** fix clippy errors and conditional imports ([#12](https://github.com/hghalebi/-blender-rs-lib/issues/12)) ([cba9242](https://github.com/hghalebi/-blender-rs-lib/commit/cba9242a2b2946479b56a214c51af568938729f6))
* **ci:** remove invalid reviewer step ([03db07e](https://github.com/hghalebi/-blender-rs-lib/commit/03db07e7e6fc407572c01fae0071e4798ab565fe))
* **ci:** remove invalid reviewer step ([a716d90](https://github.com/hghalebi/-blender-rs-lib/commit/a716d90f4579b78c7e3ffb10d63c0595e27b3b63))
* **ci:** remove invalid reviewer step ([#13](https://github.com/hghalebi/-blender-rs-lib/issues/13)) ([2dceedc](https://github.com/hghalebi/-blender-rs-lib/commit/2dceedc23d5fc26d1c7c05527c35ad3591db8465))
* **ci:** remove invalid reviewer step ([#14](https://github.com/hghalebi/-blender-rs-lib/issues/14)) ([03db07e](https://github.com/hghalebi/-blender-rs-lib/commit/03db07e7e6fc407572c01fae0071e4798ab565fe))

## 0.1.0 (2026-01-16)


### Features

* **arch:** migrate from line-based surgery to search-and-replace ([820fa2f](https://github.com/hghalebi/-blender-rs-lib/commit/820fa2fc17b272c66dbc57b29ec7633d5dd8ed69))
* **arch:** migrate from line-based surgery to search-and-replace ([4c76c48](https://github.com/hghalebi/-blender-rs-lib/commit/4c76c4829ad0bdb87774c0cab4515962e35f387d))
* **cli:** enhance help output with examples and architecture details ([7bc1460](https://github.com/hghalebi/-blender-rs-lib/commit/7bc146041012bef5b3c9c3837a7cf72edcb1ca48))
* **infra:** implement pre-flight python syntax check ([cb68273](https://github.com/hghalebi/-blender-rs-lib/commit/cb682735cfc09a0de07cd169966b7ee8afc465a4))
* **infra:** implement pre-flight python syntax check ([ce8c727](https://github.com/hghalebi/-blender-rs-lib/commit/ce8c7276b8d3212abead836ae6d7e69310f65337))


### Bug Fixes

* **ci:** fix clippy errors and conditional imports ([1a86858](https://github.com/hghalebi/-blender-rs-lib/commit/1a868585e25a6170310788556fa3ce2f685133a9))
* **ci:** fix clippy errors and conditional imports ([#12](https://github.com/hghalebi/-blender-rs-lib/issues/12)) ([cba9242](https://github.com/hghalebi/-blender-rs-lib/commit/cba9242a2b2946479b56a214c51af568938729f6))
* **ci:** remove invalid reviewer step ([03db07e](https://github.com/hghalebi/-blender-rs-lib/commit/03db07e7e6fc407572c01fae0071e4798ab565fe))
* **ci:** remove invalid reviewer step ([a716d90](https://github.com/hghalebi/-blender-rs-lib/commit/a716d90f4579b78c7e3ffb10d63c0595e27b3b63))
* **ci:** remove invalid reviewer step ([#13](https://github.com/hghalebi/-blender-rs-lib/issues/13)) ([2dceedc](https://github.com/hghalebi/-blender-rs-lib/commit/2dceedc23d5fc26d1c7c05527c35ad3591db8465))
* **ci:** remove invalid reviewer step ([#14](https://github.com/hghalebi/-blender-rs-lib/issues/14)) ([03db07e](https://github.com/hghalebi/-blender-rs-lib/commit/03db07e7e6fc407572c01fae0071e4798ab565fe))

## [0.2.0](https://github.com/hghalebi/blender-rs/compare/v0.1.0...v0.2.0) (2026-01-16)


### Features

* **arch:** migrate from line-based surgery to search-and-replace ([820fa2f](https://github.com/hghalebi/blender-rs/commit/820fa2fc17b272c66dbc57b29ec7633d5dd8ed69))
* **arch:** migrate from line-based surgery to search-and-replace ([4c76c48](https://github.com/hghalebi/blender-rs/commit/4c76c4829ad0bdb87774c0cab4515962e35f387d))
* **infra:** implement pre-flight python syntax check ([cb68273](https://github.com/hghalebi/blender-rs/commit/cb682735cfc09a0de07cd169966b7ee8afc465a4))
* **infra:** implement pre-flight python syntax check ([ce8c727](https://github.com/hghalebi/blender-rs/commit/ce8c7276b8d3212abead836ae6d7e69310f65337))

## 0.2.0 (2026-01-16)

### Features

* **arch:** migrate from line-based surgery to search-and-replace blocks ([#24](https://github.com/hghalebi/blender-rs/issues/24))
* **docs:** extensive updates to README and ARCHITECTURE ([#25](https://github.com/hghalebi/blender-rs/issues/25))

## 0.1.0 (2026-01-15)


### Bug Fixes

* **ci:** fix clippy errors and conditional imports ([1a86858](https://github.com/hghalebi/blender-rs/commit/1a868585e25a6170310788556fa3ce2f685133a9))
* **ci:** fix clippy errors and conditional imports ([#12](https://github.com/hghalebi/blender-rs/issues/12)) ([cba9242](https://github.com/hghalebi/blender-rs/commit/cba9242a2b2946479b56a214c51af568938729f6))
* **ci:** remove invalid reviewer step ([a716d90](https://github.com/hghalebi/blender-rs/commit/a716d90f4579b78c7e3ffb10d63c0595e27b3b63))
* **ci:** remove invalid reviewer step ([#13](https://github.com/hghalebi/blender-rs/issues/13)) ([2dceedc](https://github.com/hghalebi/blender-rs/commit/2dceedc23d5fc26d1c7c05527c35ad3591db8465))
* **ci:** remove invalid reviewer step ([#14](https://github.com/hghalebi/blender-rs/issues/14)) ([03db07e](https://github.com/hghalebi/blender-rs/commit/03db07e7e6fc407572c01fae0071e4798ab565fe))
