#! /bin/sh

if [ $# -eq 0 ]; then
	echo "Il faut y donner au moins un fichier de sudoku en entrée."
	exit 1
fi

for file in "$@"; do
	echo "Résolution de $file :"

	out_file_clauses="__tmp_clause_dimacs"
	./encode-sat "$file" "$out_file_clauses"

	out_file_sat_result="$out_file_sat_result.out"
	minisat "$out_file_clauses" "$out_file_sat_result" > /dev/null
	./decode-sat "$out_file_sat_result"

	rm "$out_file_clauses" "$out_file_sat_result"
done
