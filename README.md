# Rusty Notes
Mostly following the diesel getting start guide, but for SqLite.

## Setup deps
```bash
cargo install diesel_cli
```

## Set up database
```bash
diesel migration redo
```
## Insert Into Database
```bash
cargo run --bin write_item hello
```
## Print from database
```bash
cargo run --bin show_lists -v 
```