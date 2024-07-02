#!/bin/bash

# Funktion zum Einlesen der Dateien und Extrahieren der gewünschten Daten
extract_data() {
  local input_folder=$1
  local output_file=$2

  # Erstelle den Header der Markdown-Tabelle
  echo "| Befehl | Points | Hyperplanes | Distance Tests | CPU Seconds |" > $output_file
  echo "|--------|--------|-------------|----------------|-------------|" >> $output_file

  # Durchlaufe alle Dateien im Ordner
  for file in "$input_folder"/*.txt; do
    # Lese die Datei und extrahiere die gewünschten Daten
    local befehl=$(grep -oP 'Statistics for: \K.*' "$file")

    local points=$(grep -oP 'Number of points processed: \K[0-9]+' "$file")
    local hyperplanes=$(grep -oP 'Number of hyperplanes created: \K[0-9]+' "$file")
    local distance_tests=$(grep -oP 'Number of distance tests for qhull: \K[0-9]+' "$file")
    local cpu_seconds=$(grep -oP 'CPU seconds to compute hull \(after input\): \K[0-9]+(\.[0-9]+)?' "$file")

    # Ersetze alle Vorkommen von "|" durch "I" in der Variable befehl
    befehl=$(echo "$befehl" | sed 's/|/I/g')

    # Füge die Daten zur Markdown-Tabelle hinzu
    echo "| $befehl | $points | $hyperplanes | $distance_tests | $cpu_seconds |" >> $output_file
  done
}

# Hauptfunktion zum Aufruf der Extraktionsfunktion
main() {
  local input_folder="cubic"
  local output_file="cubic.md"

  # Rufe die Funktion zum Extrahieren der Daten auf
  extract_data $input_folder $output_file
}

# Aufruf der Hauptfunktion
main
