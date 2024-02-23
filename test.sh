#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/rzig "test/$input" > tmp.s
    cc tmp.s -o tmp
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
	echo "$input => $actual"
    else
	echo "$input => $expected expected, but got $actual"
	exit 1
    fi
}

assert 42 "test.zig" 
echo OK
