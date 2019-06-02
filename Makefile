services:
	docker-compose up -d
initdb: services
	diesel setup --migration-dir migrations/postgres/
migrate:
	diesel migration run --migration-dir migrations/postgres/
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
test:
	# launch tests & coverage, for tests only: "cargo test"
	cargo tarpaulin -v --features sqlite
tmptest:
	rm -f /tmp/activue.sqlite3 && export $(cat .env | xargs) && cargo test
run:
	cargo watch -x run
doc:
	cargo doc --open
