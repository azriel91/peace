# Control Flow - Rust

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%;">

```rust ,ignore
pub fn download(path: &str)
-> Result<Success, Fail> {
    // ..
}

pub enum Success {
    Downloaded,
    Cached,
}

pub enum Fail {
    UnknownHost(io::Error),
    ConnectionLost(io::Error),
    OutOfDiskSpace {
        path: PathBuf,
        error: io::Error,
    },
}
```

<!--

#
# use std::{
#     fs::File,
#     io::{self, BufRead, BufReader, BufWriter, Write},
#     net::{SocketAddr, TcpStream},
#     path::{Path, PathBuf},
# };
#
# mod _std {
# // provided by Rust standard library
# pub enum Result<T, E> {
#     Ok(T),
#     Err(E),
# }
# }
#
# pub fn download2(path: &str) -> Result<Success, Fail> {
#     let path = Path::new(path);
#     if path.exists() {
#         // Assume content is unchanged.
#         return Ok(Success::Cached);
#     }
#
#     let socket_addr = SocketAddr::from(([127, 0, 0, 1], 12345));
#     let mut line = String::with_capacity(4096);
#
#     let tcp_stream = TcpStream::connect(socket_addr)
#         .map_err(Fail::UnknownHost)?;
#     let mut input = BufReader::new(tcp_stream);
#     let file = File::create(path).unwrap();
#     let mut out = BufWriter::new(file);
#
#     loop {
#         let n = input
#             .read_line(&mut line)
#             .map_err(Fail::ConnectionLost)?;
#
#         if n == 0 {
#             break;
#         }
#
#         out.write_all(line.as_bytes())
#             .map_err(|error| Fail::OutOfDiskSpace {
#                 path: path.to_path_buf(),
#                 error,
#             })?;
#
#         line.clear();
#     }
#
#     Ok(Success::Downloaded)
# }
#
# // Use `download()`
# pub fn use_download() {
#     match download2("/tmp/a_file.txt") {
#         Ok(Success::Downloaded) => {}
#         Ok(Success::Cached) => {}
#         Err(Fail::UnknownHost(_)) => {}
#         Err(Fail::ConnectionLost(_)) => {}
#         Err(Fail::OutOfDiskSpace { .. }) => {}
#     }
#
#     match download2("/tmp/b_file.txt") {
#         Ok(_) => {}
#         Err(_) => {}
#     }
# }
# fn main() {}

-->

</div>

<div style="flex-basis: 50%;">

<object
    type="image/svg+xml"
    data="control_flow.svg"
    width="250"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEWBmkAbaAfQBMB7AdwDsDz5T0sBvAX2y1MgGcBrY7gAd4SEpGrcArrnxIY1AC5M2HJOWrVoSBZHXFY0btCUt2OUhOJ1yvSYOK54hZWc5Va9UsSQjw0RqYcFDR0DMQ2LqCg1OSkJNTwALaGmDj4RGTuoQE4uXlYAETBHgwAFACUBUFZnuG8TPmNhTYAhE1V5jVhPij%2BDe09fqR5HZyW1rb2js5NOAWS1LwxNLkAPABGsAD0AHzg5NwKo2oaWjp6BkYmA%2Bqa2rrUG7BYu3SHo1x8AsKixOJSMkgcmgiiYBXIkgUWHIuCeLx2n14WCEImgVWisRIcW4SG4qTwhCxXQCBQ%2BxLqYLJIVqgz6hQ%2B43INjsDicBEpqluZwe%2BkMxg55h4-BRv3%2B0lk8hMpKiMTixGgiXIACsePj0kTqQwmIBeDcApzvVTVeCJYQCg5AaSl5aQFtYBdnaCjOZUzZTEAPBuABH3Oad7hc%2BSZADLkQSF31Rfwk4qBkp1gD6dqL%2BADmKQ46syhuIKYtFKwAG1k8VsgAaNypmwAXSThPT2TTeZpvjpOYrNYYhablrrpDLaUb5LIDsmrJmDYyrZbfZZ0wInYJw57Jzu52ovKuTCHGothbn3J9VynuZ7iODorDgOBoOze8NLaDIrEx4lIIUZdACj8yXxh1QRDxHBwsqx0EcSQCAUb9GmgAAPBQECYAByJBJFgbhyFgYBBHIcQFDgGCfyLDNjQAMjjAwQRwnBuHAeBBBINQ6FgJgiOgEi8lbbx2yYAAqBimNyCxuCsJl%2BwnJh8IMUhSORCiqO8chaKYUScM3b1F0ufksHY%2BTmOvH5bwBe8z3U-xQCAA)</small>

</div>

</div>


### Usage

```rust
# #![deny(unused_must_use)]
# fn main() {
// Use `download()`
let result = download("/tmp/a_file.txt");
match result {
    Ok(Success::Downloaded) => {}
    Ok(Success::Cached) => {}
    Err(Fail::UnknownHost(_)) => {}
    Err(Fail::ConnectionLost(_)) => {}
    Err(Fail::OutOfDiskSpace { .. }) => {}
}

match download("/tmp/b_file.txt") {
    Ok(_) => {}
    Err(Fail::UnknownHost(_)) => {}
    Err(Fail::ConnectionLost(_)) => {}
    Err(Fail::OutOfDiskSpace { .. }) => {}
}

upload("/tmp/a_file.txt");
// match upload_outcome {
//     Ok(_) => {}
//     Err(FailUpload::UnknownHost(_)) => {}
//     Err(FailUpload::ConnectionLost(_)) => {}
// }
# }
#
# pub fn download(_s: &str) -> Result<Success, Fail> {
#     Ok(Success::Downloaded)
# }
# pub fn upload(_s: &str) -> Result<Success, FailUpload> {
#     Ok(Success::Downloaded)
# }
#
# pub enum Success {
#     Downloaded,
#     Cached,
# }
#
# pub enum Fail {
#     UnknownHost(std::io::Error),
#     ConnectionLost(std::io::Error),
#     OutOfDiskSpace {
#         path: std::path::PathBuf,
#         error: std::io::Error,
#     },
# }
#
# pub enum FailUpload {
#     UnknownHost(std::io::Error),
#     ConnectionLost(std::io::Error),
# }
```

<!--

1. In Rust, when our function can fail, instead of throwing exceptions, we return a `Result`.
2. And our `Success` and `Fail` enums have variants for both the successful and erroneous cases.
3. When we look at the usage of the `download()` function, we don't handle exceptions with catch blocks, we do a `match` on the return value.
4. Because every return value is linked to one function call, any error that we see, is unambiguously to that function call.
5. The `ConnectionLost` in line 8 must come form the `download()` in line 3.
6. The `ConnectionLost` in line 15 must come form the `download()` in line 12.
7. Rust guides us to write unambiguous control flow.
8. (excited tone) Do you know what this means?
9. It means, when an error happens, you don't need to live debug, or write logs, to trace how the code execution got there.
10. You know that this error, definitely came from here.
11. The cost of investigation decreases, because there is clarity.

-->
