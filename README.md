# Docker-Rust-Redis-PG
A sample CURD app showcasing Docker use case with Rust, Redis &amp; PG

## Development Information
This application is developed as my side hobby project to promote clean rust architectures and docker env.

## Release v1.0
This release introduces cache less CURD app with clean actix-web architecture (Source mentioned in this file).

## Future Release v1.1
In release v1.1 I will introduce,
1. Redis cache with in same infrastructure.
2. Few Benchmarks.

## Build Process

This document outlines the steps required to build and run the project locally.

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Clone the Repository

Clone the project repository to your local machine using Git

### Use docker

### Buid
```bash
docker-compose build
```

### Run
```bash
docker-compose up
```

### Stop
```bash
docker-compose down
```


### Inspiration
1. Application Inspiration : [Dreams Of Codes](https://github.com/dreamsofcode-io/spellbook)
2. Architecture Inspiration : [cookiecutter-rust-actix-clean-architecture by MDUYN](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/tree/main) 

