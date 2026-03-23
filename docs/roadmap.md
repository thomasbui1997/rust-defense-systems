# Roadmap

## Phase 1 — TACLANE Log Ingestion CLI
- Rust fundamentals
- file I/O
- parsing
- basic testing

## Phase 2 — Configuration Validator CLI
- validate synthetic mission, network, or device config files
- practice schema design, enums, and structured diagnostics
- emphasize operator-friendly error messages and safe defaults
- portfolio signal: prevents bad deployments before runtime

## Phase 3 — Telemetry Replay and Anomaly Detection Tool
- replay synthetic telemetry streams from files
- detect stale timestamps, dropouts, and out-of-range values
- use threads, channels, and time-aware processing
- portfolio signal: monitoring and reliability tooling

## Phase 4 — Network Protocol Emulator
- implement a small custom protocol over TCP or UDP
- practice framing, parsing, retries, timeouts, and checksums
- compare sync vs async design tradeoffs
- portfolio signal: systems and networking depth

## Phase 5 — Fault Injection and Robustness Harness
- feed malformed logs, corrupt packets, and delayed inputs into earlier tools
- verify graceful failures, clear diagnostics, and test coverage
- focus on resilience, not just happy-path correctness
- portfolio signal: engineering discipline under failure conditions

## Phase 6 — Embedded-Oriented Data Recorder Simulation
- simulate bounded buffers, log rotation, and constrained memory behavior
- practice state machines and explicit resource limits
- introduce embedded-style design without requiring hardware
- portfolio signal: edge-device and low-level systems thinking

## Phase 7 — Situational Awareness Terminal Dashboard
- tail synthetic events and display health counters in real time
- integrate earlier parsers and telemetry components into one operator view
- focus on concurrency, streaming I/O, and CLI UX
- portfolio signal: demo-ready end-to-end tooling

## Roadmap Principles
- keep scenarios realistic but clearly synthetic
- prefer fewer polished projects over many unfinished ones
- document design tradeoffs, failure modes, and testing strategy for each phase
- make every phase demoable with sample input and output
