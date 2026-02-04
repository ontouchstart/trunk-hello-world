build:	.cargo/bin/trunk
	./.cargo/bin/trunk build --release --dist docs --public-url /trunk-hello-world

.cargo/bin/trunk:
	cargo install trunk --force --root .cargo
