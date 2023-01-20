# std-ownership
An ownership model that is used to replace the system call in Linux.
## Benchmark

### 1w Concurrency

Ownership Borrow:
[![ownership](benches/typical_ownership.svg)](benches/typical_ownership.svg)

Geteuid(Syscall):
[![geteuid](benches/typical_geteuid.svg)](benches/typical_geteuid.svg)
