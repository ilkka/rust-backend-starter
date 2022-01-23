# Rust backend starter

Rust backend starter with

- OpenAPI spec & swagger with rocket_okapi
- Postgres with diesel

Why? Because I happened to work through this and all the bits weren't immediately obvious.

## How to start developing

1. Install diesel CLI with

   ```shell
   cargo install diesel_cli --no-default-features --features postgres
   ```

2. Run the dev database with

   ```shell
   docker-compose up -d
   ```

3. Run migrations with

   ```shell
   diesel migration run
   ```

4. Install watchexec with

   ```shell
   cargo install watchexec-cli
   ```

5. Start in dev mode with

   ```shell
   watchexec --restart cargo run
   ```
