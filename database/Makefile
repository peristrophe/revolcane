
APP_NAME := revolcane

# for development (inside docker network)
export DB_HOST       := 172.56.57.100
export DB_PORT       := 5432
export DB_USER       := test
export DB_PASSWORD   := test
export DB_NAME       := revolcane

.PHONY: prepare postgres-run postgres-init

# out of dev-container
prepare:
	-docker network create --subnet=172.56.57.0/24 $(APP_NAME)_network

# out of dev-container
postgres-run:
	$(MAKE) prepare
	docker run --rm \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_INITDB_ARGS="--encoding=UTF-8 --locale=C" \
		--net $(APP_NAME)_network \
		--ip ${DB_HOST} \
		-p ${DB_PORT}:${DB_PORT} \
		postgres:15.3-bullseye

postgres-init:
	-test -f ./init.dcl && \
		psql service=init-dev < ./init.dcl && \
		psql service=dev < ./init.ddl && \
		psql service=dev < ./init.dml
	-test -f ./database/init.dcl && \
		psql service=init-dev < ./database/init.dcl && \
		psql service=dev < ./database/init.ddl && \
		psql service=dev < ./database/init.dml
