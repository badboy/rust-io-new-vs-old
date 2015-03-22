default: old-io new-io
old: old-io
new: new-io

FLAGS=-O

LEN=10
FILE=40mb.rdb

old-io: old-io.rs
	rustc $(FLAGS) old-io.rs

new-io: new-io.rs
	rustc $(FLAGS) new-io.rs

perf: new-io old-io
	perf stat ./new-io $(FILE) $(LEN)
	perf stat ./old-io $(FILE) $(LEN)
