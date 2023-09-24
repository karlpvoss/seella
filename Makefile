.PHONY: readme
readme:
	./README.sh > README.md

.PHONY: watch
watch:
	cargo watch -x check -x clippy

.PHONY: pre-tag
pre-tag:
	cargo check; cargo clippy; cargo build --release; cargo test; cargo test --release; cargo package

.PHONY: up
up:
	docker compose up -d --wait

.PHONY: down
down:
	docker compose down --remove-orphans

.PHONY: logs
logs:
	docker compose logs -f

.PHONY: cqlsh
cqlsh:
	docker compose exec scylla1 cqlsh -u cassandra -p cassandra

.PHONY: init-db
init-db:
	docker compose exec scylla1 cqlsh -u cassandra -p cassandra -f /data/load-db.cql
