DEVICE_HOST := "remarkable"

build:
        cargo build --release --target=armv7-unknown-linux-gnueabihf

deploy:
	ssh {{DEVICE_HOST}} 'killall -q -9 remtiski || true; systemctl stop xochitl || true'
	scp ./target/armv7-unknown-linux-gnueabihf/release/remtiski {{DEVICE_HOST}}:
	ssh {{DEVICE_HOST}} 'RUST_BACKTRACE=1 RUST_LOG=debug ./remtiski'
