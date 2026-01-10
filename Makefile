BASE=docker exec -it balance-rust-app

run:
	$(BASE) cargo run

watch:
	$(BASE) cargo watch -q -c -x "run --features local"

tests:
	$(BASE) cargo test

clean-tests:
	$(BASE) cargo clean && $(BASE) cargo build && $(BASE) cargo test

migrate-tool-install:
	$(BASE) cargo install sqlx-cli

migrate:
	$(BASE) sqlx migrate run