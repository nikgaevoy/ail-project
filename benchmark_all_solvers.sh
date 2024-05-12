for s in "first-uip-basic" "first-uip" "second-uip" "third-uip" "all-uip" "saturating-all-uip" "rel-sat"
do
echo "Running: $s"
./benchmark_solver.sh "$s"
done

find benchmark/unsatisfiable/ -size 0 -print -delete
