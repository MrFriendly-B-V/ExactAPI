all:
.PHONY: up build

up:
	export DOCKER_BUILDKIT=1
	export COMPOSE_DOCKER_CLI_BUILD=1
	docker compose up -d --build

build:
	DOCKER_BUILDKIT=1 docker build --ssh=default -t registry.mrfriendly.uk/exactapi .