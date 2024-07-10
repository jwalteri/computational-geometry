# Praktikum 1

## Aufgabenstellung
In dem Tar-File 'strecken.tgz' (s.u.) befinden sich Dateien mit jeweils 4 Koordinaten pro Zeile. Diese stellen jeweils die x- und y-Koordinaten eines Start- bzw. Endpunkts einer Strecke dar. Lesen Sie jeweils eine Datei ein und ermitteln Sie die Anzahl der sich schneidenden (d.h. mindestens ein gemeinsamer Punkt) Strecken, indem Sie jedes Paar von Strecken gegeneinander testen. Messen Sie die pro Datei aufgewendete Zeit. Begründen Sie nachvollziehbar, warum die Anzahl der von Ihrem Programm jeweils gefundenen Schnittpunkte korrekt ist.

## Implementierung
- Programmiersprache: Rust (rustc 1.73.0 (cc66ad468 2023-10-03))

### main() - Pseudo Code
Inputdaten einlesen, umwandeln und auf Schnittpunkte testen.

1. points = extract_points(file_path);
2. segments = convert_points_to_segments(&points);
3. intersections = get_intersections(segments);

### get_intersections() - Pseudo Code
1. Vergleiche jedes Segment mit jedem anderen: p1p2.intersects(q1q2)
   1. Test1
      1. Orientierung von q1 zu Linie p1p2
      2. Orientierung von q2 zu Linie p1p2
      3. Liegen q1 und q2 auf der gleichen Seite von p1p2? Ja -> false
   2. Test2
      1. Orientierung von p1 zu Linie q1q2
      2. Orientierung von p2 zu Linie q1q2
      3. Liegen p1 und p2 auf der gleichen Seite von q1q2? Ja -> false
   3. Test3
      1. Alle Punkte kollinear? Ja -> Schnitt?
         1. Überlappen sich die x-Koordinaten der Segmente? Ja -> true
         2. Überlappen sich die y-Koordinaten der Segmente? Ja -> true
         3. Sonst false
   4. Alle Tests bestanden? Ja -> true

### Test auf Korrektheit

### Geo
Wir nutzen die im Praktikum angesprochene Bibliothek "geo".
Diese beinhaltet eine Implementierung zum Test auf Intersections.
Wir erhalten in allen Fällen die gleiche Anzahl an Intersections.

| File | Implementation | Geo-lib | Intersections |
|------|----------------|---------|---------|
| s_1000_1 | 20 ms | 97 ms | 11 |
| s_10000_1 | 1366 ms | 7337 ms | 732 |
| s_100000_1 | 140605 ms | 736555 ms | 77138 |

### Unit-Tests
Wir testen verschiedene Fälle mit Unit-Tests. Dadurch können wir sicher gehen, dass der Standardfall, sowie Grenzfälle abgedeckt sind.