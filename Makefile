test:
	rm -f screenshot.png
	cargo test --test test-trunk-hello-world
	file screenshot.png


