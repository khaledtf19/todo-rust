# TODO-RUST
**a small todo CLI app with Rust**

## used tech
- **clap:** Command line argument parser for Rust
- **sqlx:** Rust SQL took kit
- **Tokio:** A runtime for writing asynchronous app with Rust
- **tabled:** to print the tables out with the struct
- **docker:** for the PostgreSQL DB

## HOW TO START 
1. run docker PostgreSQL container
   - `sudo docker run --name todo -e POSTGRES_PASSWORD=password -e POSTGRES_USER=dev -e POSTGRES_DB=todo -d postgres`
2. put the database URL in the `.env`

## COMMANDS
### CREATE GET DONE DELETE
- Create new todo task `cargo run create <task name> "<task discription>"`
