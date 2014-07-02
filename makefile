q_test: q.rs
	rustc -o $@ --test $<

test: q_test
	./q_test

.DEFAULT: test
