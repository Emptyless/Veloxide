# Folder Explanations

This document explains the purpose of each folder in the repository.

- [src](https://github.com/liamwh/veloxide/tree/main/crates/veloxide-server/src):  This folder contains the source code for the application.
  - [src/application](https://github.com/liamwh/veloxide/tree/main/crates/veloxide-server/src/application): This folder contains the application layer.
  - [src/domain](https://github.com/liamwh/veloxide/tree/main/crates/veloxide-server/src/domain): This folder contains the domain layer.
  - [src/presentation](https://github.com/liamwh/veloxide/tree/main/crates/veloxide-server/src/presentation): This folder contains the presentation layer, containing things like handlers, view models, the GraphQL server and the Axum web server.
- [tests](https://github.com/liamwh/veloxide/tree/main/crates/veloxide-server/tests): This folder contains the [integration tests](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html) for the application, such as the BDD tests. [Unit tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html) are found in the tests module found in the same file as the things they're testing.
- [docker](https://github.com/liamwh/veloxide/tree/main/docker): This folder contains the configurations for the containers spun up by `docker-compose`.
