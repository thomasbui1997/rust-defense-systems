# Phase 1 — TACLANE Log Ingestion CLI

## Overview
This project simulates parsing logs from a secure network encryption device (TACLANE-style).

The goal is to:
- learn Rust fundamentals
- build a structured parser
- practice ownership and error handling

## Features (initial)
- read log file
- parse lines into structured data
- basic filtering
- simple summaries

## Example log format
```
2026-03-20T14:22:31Z INFO SESSION_ESTABLISHED peer=10.0.0.5 cipher=AES256
2026-03-20T14:22:35Z WARN PACKET_DROP reason=buffer_overflow
2026-03-20T14:22:40Z ERROR LINK_DOWN interface=eth0
```

## Notes
These logs are synthetic but modeled after real-world network device logging patterns.
