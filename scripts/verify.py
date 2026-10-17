"""Bounded checks for the independent crates; all binaries are rebuilt locally."""
from pathlib import Path
import os
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT.parents[1] / 'build' / 'rust-labs'

def clean(value):
    return value.replace(str(ROOT), '.').replace(str(TARGET), '<build>')

def run(command, timeout=180, expected=0):
    print('$ ' + clean(' '.join(str(x) for x in command)), flush=True)
    try:
        p = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=timeout)
        output = (p.stdout + p.stderr).strip()
        lines = output.splitlines()
        # Keep the verification record short; compiler diagnostics remain visible.
        if len(lines) > 65:
            lines = lines[:45] + ['[middle compiler output omitted]'] + lines[-20:]
        print(clean('\n'.join(lines)))
        print('Exit:', p.returncode, '| expected:', expected, flush=True)
        return p.returncode == expected
    except subprocess.TimeoutExpired as error:
        print((error.stdout or b'').decode(errors='replace') if isinstance(error.stdout, bytes) else error.stdout or '')
        print('TIMEOUT after', timeout, 'seconds; child terminated by subprocess.run', flush=True)
        return expected == 'timeout'

print('Real-Time Analysis labs | Windows | Rust 1.94.0 | 2026-10-17')
print('Synthetic teaching inputs. Timing is one local sample, not a WCET guarantee.')
results=[]
with tempfile.TemporaryDirectory() as temp:
    for name,path in [('safety','Lab-2/lab2_safety/src/main.rs'),('jitter','Lab-2/lab2_jitter/src/main.rs'),('metrics','lab-4/lab4_metrics/src/main.rs')]:
        binary=Path(temp)/(name+'.exe')
        results.append(run(['rustc','--edition=2024','-O',str(ROOT/path),'-o',str(binary)]))
        results.append(run([str(binary)],timeout=5 if name=='safety' else 20,expected='timeout' if name=='safety' else 0))
    results.append(run(['cargo','check','--locked','--manifest-path','lab-4/lab4_metrics/Cargo.toml','--target-dir',str(TARGET)],expected=101))
    for path in ['lab-1/testProject/Cargo.toml','Lab-2/Cargo.toml']:
        results.append(run(['cargo','run','--locked','--manifest-path',path,'--target-dir',str(TARGET)], expected=101 if path.startswith('lab-1/') else 0))
    for path,bench in [('lab5_costs/Cargo.toml','cost_benchmark'),('lab5_efficiency/Cargo.toml','efficiency_bench')]:
        results.append(run(['cargo','bench','--profile','dev','--locked','--manifest-path',path,'--target-dir',str(TARGET),'--bench',bench,'--','--test'],timeout=300))
print('Expected outcomes matched:',sum(results),'/',len(results))
raise SystemExit(0 if all(results) else 1)
