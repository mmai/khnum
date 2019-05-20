services:
	docker-compose up -d
initdb: services
	diesel setup
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
run:
	cargo watch -x run
