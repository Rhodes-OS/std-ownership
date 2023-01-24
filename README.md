# std-ownership
An ownership model that is used to replace the Ring in Linux.
## Benchmark

### 1w Concurrency

It's surprising! The ownership model is 10x faster than Ring in Syscall.

#### Ownership Borrow:
[![ownership](benches/typical_ownership.svg)](benches/typical_ownership.svg)

#### Geteuid(Syscall):
[![geteuid](benches/typical_geteuid.svg)](benches/typical_geteuid.svg)
