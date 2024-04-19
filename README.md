# x86-instruction-set-analyzer

Analyze x86 instruction sets for a binary.

## Example

```shell
$ cargo run -- ../rust-rocksdb/target/release/rocksdb

Format: Elf Little-endian 64-bit
Kind: Dynamic
Architecture: X86_64

Instruction set usage:
X64: 1004745 (71.87%)
INTEL8086: 154738 (11.07%)
INTEL386: 149737 (10.71%)
AVX: 48527 (3.47%)
MULTIBYTENOP: 12733 (0.91%)
CMOV: 11046 (0.79%)
BMI2: 4477 (0.32%)
AVX512F: 3121 (0.22%)
SSE: 2853 (0.20%)
AVX2: 2254 (0.16%)
AVX512VL: 2084 (0.15%)
BMI1: 1100 (0.08%)
AVX512BW: 634 (0.05%)
LZCNT: 555 (0.04%)
POPCNT: 494 (0.04%)
SSE2: 375 (0.03%)
AVX512DQ: 140 (0.01%)
INTEL186: 123 (0.01%)
INTEL486: 89 (0.01%)
INTEL286: 87 (0.01%)
FPU: 80 (0.01%)
FMA: 40 (0.00%)
PAUSE: 14 (0.00%)
MOVBE: 13 (0.00%)
CPUID: 10 (0.00%)
TSC: 6 (0.00%)
AVX512CD: 4 (0.00%)
CLFSH: 4 (0.00%)
RDRAND: 1 (0.00%)
RDSEED: 1 (0.00%)
XSAVE: 1 (0.00%)
```
