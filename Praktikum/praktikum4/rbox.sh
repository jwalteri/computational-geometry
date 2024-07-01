#!/bin/bash

# Funktion um rbox und qconvex auszuführen und die gewünschten Informationen zu extrahieren
run_rbox_qconvex() {
    local num_points=$1
    local dimension=$2
    
    # rbox Befehl
    local rbox_command="rbox $num_points D$dimension"
    # qconvex Befehl
    local qconvex_command="qconvex s"

    # Führe den rbox Befehl aus und leite die Ausgabe an qconvex weiter
    rbox_output=$($rbox_command)
    if [ $? -ne 0 ]; then
        echo "rbox error"
        exit 1
    fi
    
    # Führe den qconvex Befehl mit der Ausgabe von rbox als Eingabe aus
    qconvex_output=$(echo "$rbox_command" | $qconvex_command)
    if [ $? -ne 0 ]; then
        echo "qconvex error"
        exit 1
    fi

    echo $rbox_output

    # Extrahiere die gewünschten Informationen
    num_points_processed=$(echo "$qconvex_output" | grep "points processed" | awk '{print $1}')
    num_hyperplanes_created=$(echo "$qconvex_output" | grep "hyperplanes created" | awk '{print $1}')
    num_distance_tests=$(echo "$qconvex_output" | grep "distance tests for qhull" | awk '{print $1}')
    cpu_seconds=$(echo "$qconvex_output" | grep "CPU seconds" | awk '{print $3}')

    # Rückgabe der extrahierten Informationen
    echo "Number of points processed: $num_points_processed"
    echo "Number of hyperplanes created: $num_hyperplanes_created"
    echo "Number of distance tests for qhull: $num_distance_tests"
    echo "CPU seconds: $cpu_seconds"
}

# Beispielverwendung
if [ $# -ne 2 ]; then
    echo "Usage: $0 <num_points> <dimension>"
    exit 1
fi

num_points=$1
dimension=$2

run_rbox_qconvex $num_points $dimension
