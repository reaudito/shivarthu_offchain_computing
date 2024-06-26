# shivarthu_offchain_computing


## Offchain server

```bash
cargo run -p offchain-server
```
Offchain computing for shivarthu

It uses [Iroh](https://github.com/n0-computer/iroh) and axum


## Surrealdb embedded database

```bash
cargo run -p embedded-database
```

### Add database folder to iroh

```bash
iroh start

iroh console

# Add database folder
blob add database

exit
```
