all:
	cd sudoku_dimacs && cargo build --release
	cp sudoku_dimacs/target/release/encode-sat encode-sat
	cp sudoku_dimacs/target/release/decode-sat decode-sat

clean:
	rm encode-sat decode-sat
