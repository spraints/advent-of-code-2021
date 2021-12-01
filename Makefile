all.txt: src/*.rs
	cargo run all > all.txt

.PHONY: diff
diff:
	cargo run all | diff all.txt -
