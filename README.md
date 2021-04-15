# SALT UTXO Rust

Rust implementation of the SALT Engineering Backend Technical Challenge.

### Features:

- Solution is implemented as an HTTP server using [Tide](https://github.com/http-rs/tide)
  , [SQLx](https://github.com/launchbadge/sqlx), and [async-std](https://github.com/async-rs/async-std).
- PostgreSQL is used to host the BTC UTXO data for efficient query capabilities.
- Multiple REST routes to satisfy both balance retrieval and additional data insights.

### Install & Execute

#### Prerequisites
- Git
- Make
- Docker
- Docker Compose

#### Run
- Clone the repository.
```shell
git clone https://github.com/lfordyce/salt_utxo_rust && cd salt_utxo_rust
```
- Run using the Makefile
```shell
make compose
```
- For further make commands:
```shell
make help
✓ usage: make [target]

build                          - Build the rust based utxo docker image
compose                        - Run docker compose up
docker-pull                    - docker pull latest images
help                           - Show help message
lint                           - Run Rust clippy for linting
scan                           - Scan for known vulnerabilities
style-check                    - Run Rust formatter on project
```
### API:
- Get the balance by providing the BTC address and balance type (spent/unspent).
```shell
curl 'http://0.0.0.0:8000/api/v1/addrs/{address}?unspentOnly={true/false}'
# example
curl 'http://0.0.0.0:8000/api/v1/addrs/1CL5TbB2MaR4mrFjtYQ5GyA3cP2bSmPxAn?unspentOnly=true'
```
- Get all unique BTC addresses within data set.
```shell
curl 'http://0.0.0.0:8000/api/v1/addrs'
```
- Get pagination result set of the BTC UTXO data.
```shell
curl 'http://0.0.0.0:8000/api/v1/records/offset/{offset}/limit/{limit}'
# example
curl 'http://0.0.0.0:8000/api/v1/records/offset/0/limit/20'
```

### Additional notes:
- Build and run standalone docker PostgreSQL:
```shell
# Build
cd psql && docker build -t psql_utxo .
# Run
docker run --rm -e POSTGRES_PASSWORD=changeme -e POSTGRES_DB=postgres -p 5432:5432 psql_utxo:latest
```