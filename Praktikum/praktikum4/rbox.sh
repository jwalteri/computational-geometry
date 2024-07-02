#!/bin/bash

# Funktion zur Generierung und Speicherung der Punkte
generate_points() {
  local num_points=$1
  local dimension=$2
  local filename=$3

  # Generiere Punkte und speichere sie in einer Variable
  #local points=$(rbox $num_points D$dimension)
  #local points=$(rbox l $num_points)
  local points=$(rbox $num_points r D$dimension)

  # Verwende die Variable als Input für qconvex und speichere das Ergebnis in der Zieldatei
  echo "$points" | qconvex TO "polygon/${filename}.txt"
}

# Funktion, die die Punkte für Dimensionen 1-10 mit verschiedenen Punktzahlen generiert
generate_all_points() {
  local points_array=(10 100 10000 100000 1000000 10000000)

# RANDOM POINTS IN SIMPLEX

#   for dimension in {2..8}; do
#     for points in "${points_array[@]}"; do
#       local filename="${dimension}_D_${points}"
#       generate_points $points $dimension $filename
#     done
#   done

#   local points_array=(1000000 10000000)
  
#   for dimension in {2..6}; do
#     for points in "${points_array[@]}"; do
#       local filename="${dimension}_D_${points}"
#       generate_points $points $dimension $filename
#     done
#   done
# }

# REGULAR SPIRALS 3D

#   for points in "${points_array[@]}"; do
#       local filename="spiral_${points}"
#       generate_points $points 0 $filename
#   done

# COSPHERICAL POINTS
    for dimension in {2..6}; do
        for points in "${points_array[@]}"; do
        local filename="polygon_${points}"
        generate_points $points $dimension $filename
        done
    done
}

# Aufruf der Funktion zum Generieren der Punkte
generate_all_points