for f in `ls -v benchmark/unsatisfiable/$2*.dimacs`;
do
    echo "Processing $f"

	if ! timeout 10m ./target/release/ail-project -s "$1" "$f" > "$f.$1.answer";
	then
		echo "Timeout"
		exit;
	fi
done
