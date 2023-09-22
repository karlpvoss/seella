COMPOSE := docker compose

.PHONY: watch
watch:
	cargo watch -x check -x clippy

.PHONY: up
up:
	$(COMPOSE) up -d --wait

.PHONY: down
down:
	$(COMPOSE) down --remove-orphans

.PHONY: logs
logs:
	$(COMPOSE) logs -f

.PHONY: cqlsh
cqlsh:
	$(COMPOSE) exec scylla1 cqlsh -u cassandra -p cassandra
