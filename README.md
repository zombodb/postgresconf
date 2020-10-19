## postgresconf webinar example code

https://postgresconf.org/conferences/postgres-webinar-series/program/proposals/pgx-build-postgres-extensions-with-rust

This is the complete set of examples I intend to demonstrate during the above webinar.

### System Dependencies

First off, you need the following software to build this extension:

- A toolchain capable of building Postgres
  > On Ubuntu, this is the full set of dependencies
  ```
  sudo apt install -y clang llvm make curl bison flex zlib1g zlib1g-dev pkg-config libssl-dev libreadline-dev`
  ```
- Rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- The `cargo-pgx` cargo subcommand (`cargo install cargo-pgx`)

Once these tools are installed, you need to run [`cargo pgx init`](https://github.com/zombodb/pgx/tree/master/cargo-pgx#first-time-initialization)
in order to locally install the three versions of Postgres `pgx` requires.  Depending on your internet connection and
computer, this step may take 5-10 minutes.

### Building this Extension

Finally, clone this repo and build it:

```shell script
$ git clone https://github.com/zombodb/postgresconf.git
$ cd postgresconf
$ cargo pgx run pg12
``` 

This will drop you into a `psql` shell to the `cargo-pgx`-managed Postgres 12, in a database named `postgresconf`.

Simply create the extension:

```sql
CREATE EXTENSION postgresconf;
```
