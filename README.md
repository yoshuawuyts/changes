# changes
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Compute the differences between two states.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
### Compute a diff between two vectors
```rust
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

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [James Coglan - The Myers diff algorithm: part 1](https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1)
- [James Coglan - The Myers diff algorithm: part 2](https://blog.jcoglan.com/2017/02/15/the-myers-diff-algorithm-part-2)
- [James Coglan - The Myers diff algorithm: part 3](https://blog.jcoglan.com/2017/02/17/the-myers-diff-algorithm-part-3)
- [James Coglan - Myers diff in linear space: theory](https://blog.jcoglan.com/2017/03/22/myers-diff-in-linear-space-theory)
- [James Coglan - Myers diff in linear space: implementation](https://blog.jcoglan.com/2017/04/25/myers-diff-in-linear-space-implementation)
- [James Coglan - The patience diff algorithm](https://blog.jcoglan.com/2017/09/19/the-patience-diff-algorithm/)
- [James Coglan - Implementing patience diff](https://blog.jcoglan.com/2017/09/28/implementing-patience-diff)

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

[releases]: https://github.com/yoshuawuyts/changes/releases
[contributing]: ./.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/changes/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/changes/labels/help%20wanted
