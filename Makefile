.PHONY: deploy
deploy:
	cargo build --release
	cp target/release/sqlit /home/max/bin/sqlit