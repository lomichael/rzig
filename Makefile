build:
	cargo build	

debug:
	RUST_LOG=debug cargo build 

test: debug
	./test.sh

clean:
	rm -rf rzig *~ tmp*

.PHONY: test clean
