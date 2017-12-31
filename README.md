# mentoring.acm.umn.edu

## Prerequisites

Install [`just`](https://github.com/casey/just), if you don't have it already.
You can usually do this with `cargo install just` if you already have Rust and Cargo installed.

Install [Docker](https://www.docker.com/).
It's almost definitely in your distro's package manager.

## Building

Run `just build` to build in a Docker container, outputting artifacts to `mentoring.tgz`.

Alternatively, run `just build-local` to build locally.
This assumes all dependencies are already installed, which may be a dangerous assumption!

## Running

### In a Docker container

Run `just build-docker` to build a Docker container for the project.
Run `just run-docker` to run this Docker container.

### Locally

Run `just run-local` to run locally.
