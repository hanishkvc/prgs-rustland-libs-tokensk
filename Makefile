

cbuild:
	cargo build
	cargo build --release

ctest:
	cargo test -- --show-output

pdf: README.rst
	rst2pdf README.rst

clean:
	rm README.pdf || /bin/true
	cargo clean

