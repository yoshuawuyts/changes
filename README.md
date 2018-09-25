# changes
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Compute the differences between two states.

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Usage
```rs,ignore
use changes::diff;

let current = vec![ 1, 2, 3 ];
let desired = vec![ 1, 2, 3, 4 ];
let diff = diff(current, desired);

println!("{:?}", diff);
// => [
//      Ops::Noop(0),            // Don't do anything at index 0
//      Ops::Noop(1),            // Don't do anything at index 0
//      Ops::Noop(2),            // Don't do anything at index 0
//      Ops::Move(3, 3),         // Move value from index 3 to current index 3
//      Ops::Done,               // End
//    ]
```

## Installation
```sh
$ cargo add changes
```

## References
- [Myers diff in linear space: implementation](https://blog.jcoglan.com/2017/04/25/myers-diff-in-linear-space-implementation)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/changes.svg?style=flat-square
[2]: https://crates.io/crates/changes
[3]: https://img.shields.io/travis/yoshuawuyts/changes.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/changes
[5]: https://img.shields.io/crates/d/changes.svg?style=flat-square
[6]: https://crates.io/crates/changes
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/changes
[9]: https://github.com/yoshuawuyts/changes/releases
