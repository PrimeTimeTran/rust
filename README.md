https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html

## Rust
(time cargo run --bin main -- 45) 2>> rust.log

## Go
(time go run main.go 45) 2>> go.log

(time cargo run --bin main -- 45) 2>> rust.log
(time cargo run --bin main -- 50) 2>> rust.log
(time cargo run --bin main -- 75) 2>> rust.log

(time go run main.go 45) 2>> go.log
(time go run main.go 50) 2>> go.log
(time go run main.go 75) 2>> go.log
