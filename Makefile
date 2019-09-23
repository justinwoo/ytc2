build:
	cargo build --release
	cp target/release/ytc2 output/
	cp pick.xsl output/
