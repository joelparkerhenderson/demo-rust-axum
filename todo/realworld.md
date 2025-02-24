# Realworld AXUM SQLx

Setup with macOS and brew and docker:

```sh
brew install homebrew/cask/docker
```

Get realworld-axum-sqlx:

```sh
git clone https://github.com/launchbadge/realworld-axum-sqlx
cd realworld-axum-sqlx
```

Get sqlx command line inteface with postgres database:

```sh
cargo install sqlx-cli --features postgres
```

Run postgres 14 database via docker container:

```sh
docker run -d --name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD={password} postgres:14
```

