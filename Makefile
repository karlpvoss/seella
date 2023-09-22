.PHONY: readme
readme:
	./README.sh > README.md

.PHONY: watch
watch:
	cargo watch -x check -x clippy

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
