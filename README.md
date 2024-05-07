# Introduction

Implementation of the project from the book -
[Zero To Production In Rust](https://zero2prod.com), which is a great resource to learn
[Rust](https://www.rust-lang.org/) and the [Actix-web](https://actix.rs/)
framework by implementing a real-world production ready email newsletter app.

The final version in the "final" branch.
master branch is using axum framwork instead of actix-web.

# Key features
* actix-web framework
* Postgres database
* unit and integration tests
* setup Continuous Integration and Deployment pipeline with Docker
* a basic but robust implementation of user authentication
* host and deploy your application to cloud server like Digital ocean.
* a comprehensive overview of how to make an API endpoint secure. 
* Idempotency - How do you make your API retry-safe
* Error handling (which Rust is good at)
* Telemetry (testing is for "known unknown" issues while telemetry is for "unknown unknown" issue)
* Exploring many Rust crates

# Book review
* The project doesn't simply provide instructions on how to accomplish tasks but takes a step-by-step approach of "problem identification - problem solution - identifying shortcomings - refactoring" to arrive at an optimal solution, explaining the reasoning behind each step.
* made me realize the importance and assistance of testing code.
* It provides a detailed demonstration of the code development process.
* helpful for learning Rust and backend development. After completing the [Rust book](https://doc.rust-lang.org/book/), you can go reading this book.

# Basic Usage
## Launch a (migrated) Postgres database and Redis via Docker:
```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```

## How to run
```bash
cargo run
```
