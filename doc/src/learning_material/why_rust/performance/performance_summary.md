# Performance Summary

<div style="font-size: 1.5em;">

1. Runtime performance depends on different factors, such as CPU usage and memory.

2. LLVM languages compile to CPU specific instructions, reducing the amount of CPU work done at runtime.

3. Memory is split into the stack and the heap.

4. Rust makes use of the stack, reducing the work to fetch information.

5. Rust has no garbage collection, memory is freed immediately when no longer needed.
