# template-warp-server ![](https://github.com/nirmangupta/template-warp-server/workflows/Rust/badge.svg)

This repository contains template code for web-server written in Rust.  
It uses [warp](https://docs.rs/warp/) for implementing the web-server and [mongodb](https://docs.rs/mongodb/) as the datastore.  

## Build and Run
To run the server, simply run the command:
```
cargo run
```

Alternatively, to only build the code use the following command:
```
cargo build
```

(Please note that both the above command build `debug/unoptimized` binary, please check the build in `Dockerfile` for more details)

The project uses clippy for linting and rustfmt for formatting. Github actions are setup to run these for each commit and PR.
To format the code using rustfmt:
```
cargo fmt
```

To run clippy:
```
cargo clippy --all-targets --all-features -- -D warnings
```

Finally, to run tests:
```
cargo test
```

## Docker
The template also contains the `Dockerfile` with 2-stage build for creating smallest possible (non-alphine) docker image.  
The multi-stage build also ensure that subsequent build after the first build are faster. The first build compiles all the dependencies which get cached as image layer. Subsequent builds can then skip the dependency compilation by using the image cache.

`docker-compose` can also be used to run the server in a docker container environment
```
docker-compose up web
```

To stop all the containers started by `docker-compose`, use the following command:
```
docker-compose down -v
```

## Tests
The project contains unit tests in files with names like `*_test.rs`.  
The code uses [mockall](https://docs.rs/mockall/) for mocking database connectivity. The test imports and dependencies are only present during `cfg(test)` and are not present in the binary at all.
