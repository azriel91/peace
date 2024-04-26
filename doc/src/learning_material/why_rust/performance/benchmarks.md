# Benchmarks

<div style="font-size: 1.5em;">

> Rust is blazingly fast ...
>
> -- <https://www.rust-lang.org/>


### Charts

* [lightningcss](https://github.com/parcel-bundler/lightningcss?tab=readme-ov-file#benchmarks)
* [TechEmpower](https://www.techempower.com/benchmarks/)
* [Benchmarks game](https://goodmanwen.github.io/Programming-Language-Benchmarks-Visualization/)

</div>

<div class="hidden">

1. "Rust is blazingly fast", says the rust-lang website.
2. and *many* projects also claim to be "blazingly fast".
3. But let's look at evidence, not claims.
4. Here are some benchmarks, and the first one is from LightningCSS.
5. CSS nano took 544 ms to minify 10,000 lines of CSS.
6. LightningCSS took 4 ms.
7. That's 100 times faster.
8. Okay we get it. It's fast.
9. If you're interested in benchmarks for web applications,
10. the TechEmpower benchmarks measure the number of requests that are completed per second by different web frameworks.
11. and these benchmarks are designed to simulate realistic workloads.
12. The entire flow of receiving a request, deserialization, communication with a database, and responding to the client, is measured.
13. Rust does pretty well.
14. In the CPU benchmarks game, which primarily measures CPU efficiency and Memory access,
15. C, C++, and Rust are close together, we'll see why in the upcoming slides.
16. Julia, Go, Java, Swift, and JS feature here as well.

</div>
