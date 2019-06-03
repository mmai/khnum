services:
	docker-compose up -d
initdb: services
	diesel setup --migration-dir migrations/postgres/
migrate:
	diesel migration run --migration-dir migrations/postgres/
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
test:
	cargo test
coverage:
	# launch tests & coverage, for tests only: "cargo test"
	echo "currently fails to #190 tarpaulin bug"
	cargo tarpaulin -v
run:
	cargo watch -x run
doc:
	cargo doc --open
