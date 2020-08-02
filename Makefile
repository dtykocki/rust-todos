cnf ?= config.env
include $(cnf)

APP_NAME=rust-todos

build:
	docker build -t $(APP_NAME) .

run:
	docker run -d -i -t --rm --env-file=./config.env -p=$(ROCKET_PORT):$(ROCKET_PORT) --name="$(APP_NAME)" $(APP_NAME)

up: build run

stop:
	# Rocket doesn't seem to support SIGTERM for graceful shutdowns:
	# https://github.com/SergioBenitez/Rocket/issues/180#issuecomment-600814975
	# https://github.com/SergioBenitez/Rocket/issues/180#issuecomment-648224092
	docker stop -t 30 $(APP_NAME); docker rm $(APP_NAME)

logs:
	docker logs rust-todos --follow
