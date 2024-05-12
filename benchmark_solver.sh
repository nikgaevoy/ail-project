for f in benchmark/unsatisfiable/*.dimacs
do
    echo "Processing $f file..."

    timeout 10s ./target/release/ail-project -s "$1" "$f" > "$f.$1.answer"
done
