functions/chan2chat:
	rustup target add x86_64-unknown-linux-musl
	RUSTFLAGS='-C target-feature=+crt-static' cargo build -r --target x86_64-unknown-linux-musl

deploy: functions/chan2chat
	mkdir functions/
	cp target/x86_64-unknown-linux-musl/release/chan_2_chat functions/chan2chat
