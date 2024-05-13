PROJECT_ROOT := $(shell git rev-parse --show-toplevel)
SOV_CLI_REL_PATH := $(PROJECT_ROOT)/target/debug/starter-cli-wallet

CELESTIA_CONFIG := $(PROJECT_ROOT)/celestia_rollup_config.toml

DOCKER_COMPOSE_DIR := ../docker
DOCKER_COMPOSE_CFG := $(DOCKER_COMPOSE_DIR)/docker-compose.yml
CREDENTIALS_DIR := $(DOCKER_COMPOSE_DIR)/credentials

# at height 3 the credits will already belong to the keys
START_HEIGHT := 3
KEY_NAME := bridge-0
RPC_PORT := 26658

# get the jwt token
get_auth = $(shell cat $(CREDENTIALS_DIR)/$(KEY_NAME).jwt)

# Docker compose commands
docker_compose := docker compose -f $(DOCKER_COMPOSE_CFG)
# start the service
compose_up := $(docker_compose) up --build --force-recreate -d
# stop the service
compose_down := $(docker_compose) down
# follow the logs
compose_logs := $(docker_compose) logs -f
# check if service is running
# we need a bridge and validator so naive check is to have at least 2 services
compose_is_running = test 2 -le "$(shell $(docker_compose) ps --filter status=running --format json | jq 'length')"

# start the celestia network and generate a new config
start: check-docker start-network update-celestia-config
	@$(compose_logs)

# start the celestia network or attach to existing
start-network: check-docker
	@echo "Starting local celestia network setup"
	@echo $(DOCKER_COMPOSE_CFG)
	@if $(compose_is_running); then $(compose_logs); else $(compose_up); fi

# stop the celestia network
stop: check-docker
	@echo "Stopping local celestia network setup"
	@$(compose_down)

# clean up rollup data and stop the celestia network
clean: stop clean-db

# listen to celestia network logs
compose-logs: check-docker
	@echo "Showing celestia logs"
	@$(compose_logs)

# check if docker is installed
check-docker:
	@command -v docker > /dev/null 2>&1 || { echo "Docker is not installed"; exit 1; }
	@# TODO: check docker compose too

# check if celestia network is running
check-compose-running: check-docker
	@$(compose_is_running)

# wait for the celestia network to perform setup and coins transfers
wait-compose-ready:
	@echo "Waiting for services to finish setup"
	@$(compose_logs) | awk '/Provisioning finished./ {print;exit}' # exit when encounter this log entry

# update the rollup configuration with local network params
update-celestia-config: wait-compose-ready
ifeq ($(shell uname -s),Darwin)
	@sed -i '' 's/^\(celestia_rpc_auth_token = \)"[^"]*"/\1"$(get_auth)"/' celestia_rollup_config.toml
	@sed -i '' 's#^\(celestia_rpc_address = \)"[^"]*"#\1"http://127.0.0.1:$(RPC_PORT)"#' celestia_rollup_config.toml
	@sed -i '' 's#^\(start_height = \)[0-9]*#\1$(START_HEIGHT)#' celestia_rollup_config.toml
else
	@sed -i 's/^\(celestia_rpc_auth_token = \)"[^"]*"/\1"$(get_auth)"/' celestia_rollup_config.toml
	@sed -i 's#^\(celestia_rpc_address = \)"[^"]*"#\1"http://127.0.0.1:$(RPC_PORT)"#' celestia_rollup_config.toml
	@sed -i 's#^\(start_height = \)[0-9]*#\1$(START_HEIGHT)#' celestia_rollup_config.toml
endif