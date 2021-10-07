<h1 align="center">Chat in Rust</h1>

[![Licensed under either of Apache License, Version 2.0 or MIT license at your option.](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/minikin/chat-rs/blob/main/LICENSE-MIT)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/minikin/chat-rs/blob/main/CODE_OF_CONDUCT.md)
[![CI](https://github.com/minikin/chat-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/minikin/chat-rs/actions/workflows/ci.yml)

# Content

- [Features](#features)
- [Demo](#demo)
- [Todo](#todo)
- [Support](#support)
- [License](#license)

## Features

_WIP_

## Demo

1. To run the server

```sh
cargo run --release --bin server -- localhost:8088
```

2. To run the client:

```sh
cargo run --release --bin client -- localhost:8088
```

## Commands

1. **join**

To create a chat group and to join it, type:

```sh
join CHAT_GROUP_NAME
```

_Chat group name must not contain any spaces._

2. **post**

To post messages to a chat group, type `chat group name` and your `message`

```sh
     [group]    [message]
post berlin   Hi from Berlin!
```

## TODO

- [ ] Add user
- [ ] Leave chat group

## Support

Post issues and feature requests on the GitHub [issue tracker](https://github.com/minikin/chat-rs/issues).

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
