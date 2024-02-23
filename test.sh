#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/rzig "test/$input"
    gcc tmp.s -o tmp # assembling using gcc
	# nasm -f bin tmp.s -o tmp # assembling using nasm
	chmod +x ./tmp
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
	echo "$input => $actual"
    else
	echo "$input => $expected expected, but got $actual"
	exit 1
    fi
}

assert 42 "simple.zig" 
echo OK
