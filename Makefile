CFLAGS=-std=c11 -g -static

guacc: guacc.c

test: guacc
	./test.sh

clean:
	rm -f guacc *.o *~ tmp*

.PHONY: test clean
