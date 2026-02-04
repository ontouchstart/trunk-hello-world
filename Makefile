build:	.cargo/bin/trunk
	./.cargo/bin/trunk build --release --dist docs --public-url /trunk-hello-world

dev:	.cargo/bin/trunk
	./.cargo/bin/trunk serve --dist docs 

.cargo/bin/trunk:
	cargo install trunk --force --root .cargo
