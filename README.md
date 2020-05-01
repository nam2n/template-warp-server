# template-warp-server ![](https://github.com/nirmangupta/template-warp-server/workflows/Rust/badge.svg)

This repository contains template code for web-server written in Rust.  
It uses [warp](https://docs.rs/warp/) for implementing the web-server and [mongodb](https://docs.rs/mongodb/) as the datastore.  

## Build and Run
To run the server, simply execute the command:
```
cargo run
```

Alternatively, to build the code use the following command:
```
cargo build
```

(Please note that both the above commands build a `debug/unoptimized` binary, please check the build in `Dockerfile` for more `release` binary)

The project uses `clippy` for linting and `rustfmt` for formatting. Github actions are setup to run these for each commit and PR.
To format the code using `rustfmt`:
```
cargo fmt
```

To run `clippy`:
```
cargo clippy --all-targets --all-features -- -D warnings
```

Finally, to run tests:
```
cargo test
```

## Docker
The template also contains the `Dockerfile` with 2-stage build for creating smallest possible (non-alphine) docker image.  
The multi-stage build also ensures that subsequent builds (after the first one) are faster. The first build compiles all the dependencies which get cached as image layer. Subsequent builds can then skip the dependency compilation by using the image cache. A full build will only run if new dependencies are added or if image cache is pruned.

`docker-compose` can also be used to run the server in a container environment
```
docker-compose up web
```
This will build the project, run mongo database, and run then web-server.

To stop all the containers started by `docker-compose`, use the following command:
```
docker-compose down -v
```

## Tests
The project contains unit tests in files with names like `*_test.rs`.  
The code uses [mockall](https://docs.rs/mockall/) for mocking database connectivity. The test imports are guarded by `cfg_attr(test)`, this ensures that these are not present in runnable binary. All dependencies required for only executing tests are added as `dev-dependencies`.

## Continous Integration
The projects contains the CI pipeline defined as github workflow.
The pipeline is defined at `/github/workflows/rust.yml`.

## Notes
1. This repository does NOT show the code organisation for large rust projects.
2. As number of endpoints in the server increase, the compilation will become slower. This is because `warp` does a lot of code-generation at compile time for faster runtime. `BoxedFilter` can be used if faster compile times are necessary.
3. The docker image build is optimized for performance while trading for higher binary size. This can be changed by adjusting `.cargo` configuration.
4. Docker image contains release binary with `debuginfo` for deeper error/crash reporting. If this is not required, then remove 'debug = true` from `Cargo.toml`. This will reduce the binary size significantly.   
