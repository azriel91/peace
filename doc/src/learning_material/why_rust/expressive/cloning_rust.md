# Cloning - Rust

### Copy

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```rust
#[derive(Clone, Copy)]
pub struct Data {
    value: u32,
}

let data_0 = Data { value: 123 };
let mut data_1 = data_0; // bitwise

data_1.value = 456;

println!("data_0: {}", data_0.value);
println!("data_1: {}", data_1.value);
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

<object
    type="image/svg+xml"
    data="cloning_rust_copy.svg"
    width="150"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEXzXgA6Y456ED6AzgC6yQB2A5ulgN4C%2B2WN8SA1sRI9%2B5AG6JyABhYcuOYXzESAjDM6h6AewAm0cvXgBbaJUEKWAIgA8AIwB8AZWq8%2BVgPT2LXMkRzX7ABL4BO6eXApKsFKWJMoATADMXvLOIuJRqlgWsYnJuMFUtAzMWQB05RVeoNDajCZmqYrpUuTkPoV0TCwA2hHNkgA0%2BRQ0nYwAuqCg1HjGDagANvVyWFq65LoAZvAArgvUpmxY0AAetPAsAORIO7CUmrDABJoM1HBYsJo79LrawAuMS5YTgkY61PRbXb7Q6sY5nBBXKDaXT0IEgoSNSLRLAAMgUR0o4EIeiQmgWDyulD4qDRKz6Khk3CJBBJZIpWEuBFuBCWtNB7VGxUZp3OLCRKOBKwWkEY4GoLBx0tl1BWOE2kAWCyoRPWWlghngCyuykkkkuqqw6s12vg63AmlEcCuAFYzRarVrCba9JtNDdDpcTW7QWqNZ6dXpeNRII7jabzSHuLRNHw9F7dQ8DUaOQAWeMW0YptMRtoOp0chL5xOF1M29ZRmPQF1VkM1tPURZNrDaTTUN7aC0EW3aIUVhMYlyCUFWKwsABUSrlBeZrPJsBYNgWO2gKx8U5IM-ni5V1ZX5FJa5YqGgms0AHdQEA)</small>

</div>
</div>

### Clone

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```rust
#[derive(Clone, Debug)]
pub struct DataOnHeap {
    num: u32,
    value: String,
}

let data_0 = DataOnHeap {
    num: 123,
    value: "hello".into()
};
let mut data_1 = data_0.clone();

data_1.value.make_ascii_uppercase();

println!("data_0: {data_0:#?}");
println!("data_1: {data_1:#?}");
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

<object
    type="image/svg+xml"
    data="cloning_rust_clone.svg"
    width="150"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEXzXgA6Y456ED6AzgC6yQB2A5uQAzpYDeAvtiWQVVoNmARnbdeNeEgDWxElNnkAbolbieJLIpkq1Yzj1D0A9gBNo5evAC20SvJ3sARAB4ARgD4AytWkzXAHovZ15%2Bdiw3LwAJfAIgkMk-JQAzegicZxt4BgAKAEpQnB09WHVIkhEAJgBmD1hAzxYADwB2Fg6i7WTdVTKDZ0ra%2BsaW9pYADi7%2BQTomcucAHWc8ABtVk2XpuNnhcgHl6IBRABkTgHkt0FBoM0Z7Rx7S1nJyGZo55jYsAG0SvtYABpcDsPnsWABdJL%2BZ4iV7vITzAx-J4AkTAhGffZQ0DUPB2R6oVYPXg4UwWcgWFLwACuq2oDk4WGgzVo8HYAHIkDTYJQTLBgAQTAxqHAsLATDT6BYzMBVowOVhNCRbvdKdBqXSGeJmayEJzubz%2BYLhfRRbBFcriqi1N8AGT8yCMBjwVZMyjgQiWJAmDawTmUGSoS2k7owtHsO1IDb0aDuz0Eb2%2B-mcgg8gjEkNaTHg9gAKkdzusq1DOaR%2BejJljodWTvA1EjtcY9dDOBSkHWVE9FNMsGyq05Ig6HNbWHbnY98Ap4BMyjgnIArCwR1o2x3Vl2p5YUiZuYyOUPl6PxxvJxTpNRIHPB8PRx8TDJLGfLL3%2B5yACy31fdCWPzfT2d5ywDkai-Vd7z-Z9yAvK9oEXMCtAgp9qCJOCsDMExqFFMxRwIKczGETkahXBQenkLRXFcfMmxbb9J0TcgfT9dh3FWGloFLOJyJISjqLrag7wTJNmKwVBoHWEwAHdri0VVLBohssDtOTyAUu9aAfJ9uxffk32A8YSK0E9-x0vtXXgo9EJtMoWHhUFES%2BbicF4rA8xUtS6KExjk39LBWPY0N-n0OyKDBctRxcty7nk-jBK9byRLTWAMw4oA)</small>


</div>
</div>
