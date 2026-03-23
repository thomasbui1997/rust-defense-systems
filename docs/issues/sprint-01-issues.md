# Sprint 01 — Issue Pack

Copy each section into a GitHub Issue.

---

## Issue 1 — Initialize Rust Project
**Labels**: build, size:S

**Why**
Start the project scaffold using Cargo.

**Task**
Run:
```
cargo new taclane_log_ingestion_cli
```
inside the phase folder.

**Definition of Done**
- Project builds
- `cargo run` prints hello world

**If stuck**
- Verify Rust installed: `rustc --version`

---

## Issue 2 — Read Rust Ownership Basics
**Labels**: learning, size:S

**Task**
Read Rust Book (ownership sections 1–3)

**Definition of Done**
- You can explain ownership in simple terms

---

## Issue 3 — Rustlings Ownership Exercises
**Labels**: learning, size:S

**Task**
Complete variables + ownership exercises

**Definition of Done**
- Exercises compile successfully

---

## Issue 4 — Read File Into String
**Labels**: build, size:S

**Task**
Use `std::fs::read_to_string` to read sample log file

**Definition of Done**
- Entire file printed

**If stuck**
AI prompt:
"Show minimal Rust example reading a file into a string"

---

## Issue 5 — Print Each Line
**Labels**: build, size:S

**Task**
Iterate through file lines

**Definition of Done**
- Each line printed individually

---

## Issue 6 — Define TaclaneLogEntry Struct
**Labels**: build, size:S

**Task**
Create struct with:
- timestamp
- level
- message

**Definition of Done**
- Struct compiles

---

## Issue 7 — Parse Single Line Into Struct
**Labels**: build, size:M

**Task**
Split one log line into fields

**Definition of Done**
- Extract timestamp and level at minimum

**If stuck**
- First parse only timestamp
AI prompt:
"How do I split a string by whitespace in Rust"

---

## Issue 8 — Add Basic Parsing Test
**Labels**: test, size:S

**Task**
Write one unit test for parsing

**Definition of Done**
- Test passes

---

## Issue 9 — Add CLI Argument for File Path
**Labels**: build, size:M

**Task**
Use `std::env::args`

**Definition of Done**
- Program accepts file path input

---

## Issue 10 — Fallback: Print First Line Only
**Labels**: fallback, size:S

**Task**
Only read and print first line

**Definition of Done**
- Minimal working path exists

---

## How to Use
- Create each issue in GitHub
- Add them to your Project board
- Move across columns as you work
