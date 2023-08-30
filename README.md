[![basebuild](https://github.com/c0d3-dump/mini-base/actions/workflows/rust.yml/badge.svg)](https://github.com/c0d3-dump/mini-base/actions/workflows/release.yml)

# mini-base

## minimal firebase alternative with tui, written in rust

***

![mini-base server image](./examples/todo/screenshots/server.png)


### how to run on linux

- downlaod linux binary from [release](https://github.com/c0d3-dump/mini-base/releases) section
- run binary using `./mini-base-linux-64` in terminal

### how to build for your system (make sure you have rust installed)

```bash
cargo run
```

## todos:

- [x] : initial tui
- [x] : web server using axum
- [x] : support for sqlite and mysql databases
- [x] : query parsing using nom parser
- [x] : role based authentication using jwt
- [x] : some examples to play with
- [x] : file upload support with role based access
- [ ] : screen to add cors access, auth/storage secrets
- [ ] : suggest new ideas

## Crates used for this project

* sqlx - database
* axum - web framework
* cursive - tui library
* jsonwebtoken - authentication
* sha2 - password hashing