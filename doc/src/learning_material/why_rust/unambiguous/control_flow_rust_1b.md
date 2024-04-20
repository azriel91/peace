# Control Flow - Rust 1

<div style="display: flex; justify-content: center;">

<object
    type="image/svg+xml"
    data="control_flow.svg"
    width="300"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEWBmkAbaAfQBMB7AdwDsDz5T0sBvAX2y1MgGcBrY7gAd4SEpGrcArrnxIY1AC5M2HJOWrVoSBZHXFY0btCUt2OUhOJ1yvSYOK54hZWc5Va9UsSQjw0RqYcFDR0DMQ2LqCg1OSkJNTwALaGmDj4RGTuoQE4uXlYAETBHgwAFACUBUFZnuG8TPmNhTYAhE1V5jVhPij%2BDe09fqR5HZyW1rb2js5NOAWS1LwxNLkAPABGsAD0AHzg5NwKo2oaWjp6BkYmA%2Bqa2rrUG7BYu3SHo1x8AsKixOJSMkgcmgiiYBXIkgUWHIuCeLx2n14WCEImgVWisRIcW4SG4qTwhCxXQCBQ%2BxLqYLJIVqgz6hQ%2B43INjsDicBEpqluZwe%2BkMxg55h4-BRv3%2B0lk8hMpKiMTixGgiXIACsePj0kTqQwmIBeDcApzvVTVeCJYQCg5AaSl5aQFtYBdnaCjOZUzZTEAPBuABH3Oad7hc%2BSZADLkQSF31Rfwk4qBkp1gD6dqL%2BADmKQ46syhuIKYtFKwAG1k8VsgAaNypmwAXSThPT2TTeZpvjpOYrNYYhablrrpDLaUb5LIDsmrJmDYyrZbfZZ0wInYJw57Jzu52ovKuTCHGothbn3J9VynuZ7iODorDgOBoOze8NLaDIrEx4lIIUZdACj8yXxh1QRDxHBwsqx0EcSQCAUb9GmgAAPBQECYAByJBJFgbhyFgYBBHIcQFDgGCfyLDNjQAMjjAwQRwnBuHAeBBBINQ6FgJgiOgEi8lbbx2yYAAqBimNyCxuCsJl%2BwnJh8IMUhSORCiqO8chaKYUScM3b1F0ufksHY%2BTmOvH5bwBe8z3U-xQCAA)</small>


</div>

```rust
# use std::{
#     fs::File,
#     io::{self, BufRead, BufReader, BufWriter, Write},
#     net::{SocketAddr, TcpStream},
#     path::{Path, PathBuf},
# };
#
pub enum Outcome {
    Downloaded,
    Cached,
    UnknownHost(io::Error),
    ConnectionLost(io::Error),
    OutOfDiskSpace {
        path: PathBuf,
        error: io::Error,
    },
}

// Define `download()`
pub fn download(path: &str) -> Outcome {
    // ..
#     let path = Path::new(path);
#     if path.exists() {
#         // Assume content is unchanged.
#         return Outcome::Cached;
#     }
#
#     let socket_addr = SocketAddr::from(([127, 0, 0, 1], 12345));
#     let mut line = String::with_capacity(4096);
#
#     let tcp_stream = match TcpStream::connect(socket_addr) {
#         Ok(tcp_stream) => tcp_stream,
#         Err(error) => return Outcome::UnknownHost(error),
#     };
#     let mut input = BufReader::new(tcp_stream);
#     let file = File::create(path).unwrap();
#     let mut out = BufWriter::new(file);
#
#     loop {
#         let n = match input.read_line(&mut line) {
#             Ok(n) => n,
#             Err(error) => return Outcome::ConnectionLost(error),
#         };
#
#         if n == 0 {
#             break;
#         }
#
#         if let Err(error) = out.write_all(line.as_bytes()) {
#             return Outcome::OutOfDiskSpace {
#                 path: path.to_path_buf(),
#                 error,
#             };
#         };
#
#         line.clear();
#     }
#
#     Outcome::Downloaded
}

// Use `download()`
pub fn use_download() {
    let outcome_1 = download("/tmp/a_file.txt");

    match outcome_1 {
        Outcome::Downloaded => {}
        // Outcome::Cached => {}
        Outcome::UnknownHost(_) => {}
        Outcome::ConnectionLost(_) => {}
        // Outcome::OutOfDiskSpace { .. } => {}
    }

    let outcome_2 = download("/tmp/b_file.txt");
}
#
# fn main() {}
```
