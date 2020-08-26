set title "Comparing different sort algorithms (Length of Vect vs. Micro Sec)"
set datafile separator ","
set key left top
plot "../sort" u 1:2 w lp t "Merge sort (parallel)", "" u 1:3 w lp t "Merge sort (sequential)", "" u 1:4 w lp t "Built in"
