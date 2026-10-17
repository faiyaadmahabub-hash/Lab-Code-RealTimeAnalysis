# Verification

Date: 17 October 2026. Windows x86_64 MSVC, Rust/Cargo 1.94.0; Intel Core i7-8565U @ 1.80 GHz. Python 3.10.9 drives bounded subprocesses. Pillow 12.0.0 renders a captured text excerpt and is not needed to run the labs.

Run from the repository root:

```powershell
python -B scripts/verify.py
```

The helper contains the exact commands and captures their stdout/stderr. It rebuilds the three standard-library examples with `rustc --edition=2024 -O`, stores executables in a disposable directory and isolates Cargo output under `../../build/rust-labs`. No committed binary is executed.

| Check | Result |
|---|---|
| Standalone `lab2_safety` compile/run | Compile passed; run printed the main-thread vector then reached the 5-second timeout. Child process killed. Reproduces holding the mutex guard while joining a worker that needs the same mutex. |
| Standalone `lab2_jitter` compile/run | Passed: 100 samples while four busy threads ran. Process exited normally within 20 seconds. Metric accumulates schedule drift and subtracts another sleep interval; do not interpret its maximum as per-sleep jitter. |
| Standalone `lab4_metrics` compile/run | Passed: 5,000 stack and 5,000 heap timing samples. See captured excerpt. This is a single optimized run amid background load, without warmup or statistical isolation. |
| `cargo check --locked --manifest-path lab-4/lab4_metrics/Cargo.toml ...` | Expected exit 101: missing declared `my_benchmark` target. |
| `cargo run --locked --manifest-path lab-1/testProject/Cargo.toml ...` | Expected exit 101: `let` outside a function near source line 191 after an extra closing brace. |
| `cargo run --locked --manifest-path Lab-2/Cargo.toml ...` | Passed: `Hello, world!`. |
| `cargo bench --profile dev --locked --manifest-path lab5_costs/Cargo.toml ... --bench cost_benchmark -- --test` | Passed: all five benchmark functions smoke-tested. Caught-panic hook prints an expected panic diagnostic; process exits 0. |
| `cargo bench --profile dev --locked --manifest-path lab5_efficiency/Cargo.toml ... --bench efficiency_bench -- --test` | Passed: all four benchmark functions smoke-tested. Random data is not seeded. |

Here `...` denotes the helper's `--target-dir` argument, not an additional input or omitted check. The transcript lists the expanded command. Cargo installed the manifest/lockfile dependencies into the normal local Cargo cache; no global application packages changed.

PowerShell capture: `python -B scripts/verify.py | Set-Content -Encoding utf8 docs/assets/verification-transcript.txt`. Compiler output is shortened by the helper when verbose, and repository/build paths are normalized. The first pass exposed the known Lab 1 failure while the helper still expected success; its expected result was corrected and the bounded checks rerun.

`docs/assets/metrics-run.png` renders only the actual Lab 4 stdout excerpt from that capture, with environment labels. It is a transcript rendering, not a native terminal screenshot. The program prints `Observed WCET`, but that value is merely the largest sample observed in this run, not a worst-case execution bound. Stack zero-fill and heap capacity reservation perform different work.

No full statistical Criterion suite or automated correctness tests ran. Historical figures in `PROJECT_GUIDE.md` were transcribed before this update and were not remeasured here. Generated `target/` files and local permission settings were removed from Git's index only, with local files retained. No history was rewritten.
