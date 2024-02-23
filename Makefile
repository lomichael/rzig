build:
	cargo build	

test: build 
	./test.sh

clean:
	rm -f rzig *~ tmp*

.PHONY: test clean
