# Praktikum 2

## Aufgabenstellung
Lesen Sie die SVG-Datei 'DeutschlandMitStaedten.svg' und ermitteln Sie die Flächen der einzelnen Bundesländer (bezüglich der in der Datei verwendeten Skala). Am Ende der Datei befinden sich Koordinaten von Städten, Versuchen Sie herauszufinden (bzw. lassen Sie das Ihren Rechner machen ;-), in welchem Bundesland diese jeweils liegen.

## Einlesen der Daten
Die Bundesländer liegen als Path in einer SVG-File vor. Ein Path besteht aus mehreren absoluten und relativen Punktangaben, welche mit 'M', 'l', 'z' und 'H'. Um den Aufwand, die Daten aus der SVG-File auszulesen, zu veringern, wurde der Inhalt jedes Paths pro Bundesland in eine eigene File geladen (siehe Ordner 'states'). Durch den Wegfall der komplexen XML-Struktur wird die Interpretation der Daten sehr einfach, da nur noch der führende Character beachtet werden muss. Alle Punktabgaben wurden zu absoluten Punkten umgerechnet. Einige Bundesländer besitzen 'Inseln', Bereiche außerhalb der Hauptfläche, und 'Löcher', Bereiche in der Hauptfläche, welche nicht zum Bundesland gehören.

Als Datenstruktur wird 'struct state' verwendet, welche den Namen, die Punkte des Bundeslandes, dessen Löcher und Inseln beinhaltet. Es werden Funktionen zum Berechnen der Fläche und zum Prüfen von Besitztümer angeboten.

## Berechnung der Fläche
Die erste Aufgabe hat das Ziel, die Fläche eines jeden Bundeslandes zu berechnen. Dabei ist zu beachten, dass manche Bundesländer Inseln und Löcher haben. Die Flächen von Inseln werden der Hauptfläche des Bundeslandes hinzu addiert, während die Fläche von Löchern subtrahiert wird.

Zur Berechnung der Fläche eines Polygons wird der Shoelace-Algorithmus verwendet.

QUELLE EINFÜGEN: https://www.101computing.net/the-shoelace-algorithm/

DREIECKS-SHOELACE AUS VORLESUNG


## Test auf Besitz einer Stadt
Die zweite Aufgabe umfasst eine Liste von 16 deutschen Städten und die Frage, welche Stadt (Punkt) in welchem Bundesland (Polygon) liegt. Dabei ist wieder zu beachten, dass Bundesländer Löcher besitzen. Dadurch kann es vorkommen, dass eine Stadt in mehreren Bundesländern liegt: Berlin liegt im Polygon Brandenburg und im Polygon Berlin. Achtung: Berlin ist ein Loch von Brandenburg.


QUELLE EINFÜGEN: http://www.philliplemons.com/posts/ray-casting-algorithm

# BEWEIS ÜBER VERGLEICH MIT NORMIERTEN DATEN AUS REALITÄT
VERHÄLTNIS unser Bayern zu realität bayern -> Faktor 
Faktor auf alle anderen bundesländer anwenden -> 
Fehler berechnen

## Ergebnisse

### Fläche der Bundesländer

| Bundesland               | Fläche     |
|--------------------------|------------|
| Baden-Württemberg        | 30522.156  |
| Bayern                   | 60026.28   |
| Berlin                   | 766.1914   |
| Brandenburg              | 25275.941  |
| Bremen                   | 340.93457  |
| Hamburg                  | 633.3203   |
| Hessen                   | 17977.523  |
| Mecklenburg-Vorpommern   | 19658.783  |
| Niedersachsen            | 40647.758  |
| Nordrhein-Westfalen      | 28966.52   |
| Rheinland-Pfalz          | 16913.566  |
| Saarland                 | 2179.7573  |
| Sachsen-Anhalt           | 17450.543  |
| Sachsen                  | 15667.891  |
| Schleswig-Holstein       | 13456.4375 |
| Thueringen               | 13724.586  |

### Zuordnung Stadt - Bundesland
| Stadt         | Bundesland           |
|---------------|----------------------|
| München       | Bayern               |
| Berlin        | Berlin               |
| Stuttgart     | Baden-Württemberg    |
| Saarbrücken   | Saarland             |
| Wiesbaden     | Hessen               |
| Mainz         | Rheinland-Pfalz      |
| Düsseldorf    | Nordrhein-Westfalen  |
| Bremen        | Bremen               |
| Erfurt        | Thüringen            |
| Dresden       | Sachsen              |
| Magdeburg     | Sachsen-Anhalt       |
| Hannover      | Niedersachsen        |
| Hamburg       | Hamburg              |
| Kiel          | Schleswig-Holstein   |
| Schwerin      | Mecklenburg-Vorpommern |
| Potsdam       | Brandenburg          |
