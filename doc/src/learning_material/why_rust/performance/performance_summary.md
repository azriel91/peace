# Performance Summary

<div style="font-size: 1.5em;">

1. Runtime performance depends on different factors, such as CPU usage and memory.

2. LLVM languages compile to CPU specific instructions, which run very quickly on that CPU.

3. Memory is split into the stack and the heap.

4. Rust makes use of the stack, reducing the time taken to fetch information.

5. Rust has no garbage collection, so applications perform consistently.
