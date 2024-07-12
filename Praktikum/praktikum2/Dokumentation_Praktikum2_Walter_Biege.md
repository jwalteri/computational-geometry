# Praktikum 2

## Aufgabenstellung
Lesen Sie die SVG-Datei 'DeutschlandMitStaedten.svg' und ermitteln Sie die Flächen der einzelnen Bundesländer (bezüglich der in der Datei verwendeten Skala). Am Ende der Datei befinden sich Koordinaten von Städten, Versuchen Sie herauszufinden (bzw. lassen Sie das Ihren Rechner machen), in welchem Bundesland diese jeweils liegen.

## Einlesen der Daten
Die Bundesländer liegen als Path in einer SVG-File vor. Ein Path besteht aus mehreren absoluten und relativen Punktangaben, welche mit 'M', 'l', 'z' und 'H'. Um den Aufwand, die Daten aus der SVG-File auszulesen, zu veringern, wurde der Inhalt jedes Paths pro Bundesland in eine eigene File geladen (siehe Ordner 'states'). Durch den Wegfall der komplexen XML-Struktur wird die Interpretation der Daten sehr einfach, da nur noch der führende Character beachtet werden muss. Alle Punktabgaben wurden zu absoluten Punkten umgerechnet. Einige Bundesländer besitzen 'Inseln', Bereiche außerhalb der Hauptfläche, und 'Löcher', Bereiche in der Hauptfläche, welche nicht zum Bundesland gehören.

Als Datenstruktur wird 'struct state' verwendet, welche den Namen, die Punkte des Bundeslandes, dessen Löcher und Inseln beinhaltet. Es werden Funktionen zum Berechnen der Fläche und zum Prüfen von Besitztümer angeboten.

## Berechnung der Fläche
Die erste Aufgabe hat das Ziel, die Fläche eines jeden Bundeslandes zu berechnen. Dabei ist zu beachten, dass manche Bundesländer Inseln und Löcher haben. Die Flächen von Inseln werden der Hauptfläche des Bundeslandes hinzu addiert, während die Fläche von Löchern subtrahiert wird.

Zur Berechnung der Fläche eines Polygons wird der ([Shoelace-Algorithmus](https://www.101computing.net/the-shoelace-algorithm/)) verwendet. 

## Test auf Besitz einer Stadt
Die zweite Aufgabe umfasst eine Liste von 16 deutschen Städten und die Frage, welche Stadt (Punkt) in welchem Bundesland (Polygon) liegt. Dabei ist wieder zu beachten, dass Bundesländer Löcher besitzen. Dadurch kann es vorkommen, dass eine Stadt in mehreren Bundesländern liegt: Berlin liegt im Polygon Brandenburg und im Polygon Berlin. Achtung: Berlin ist ein Loch von Brandenburg. Umgesetzt kann das u.a. mit [Ray-Castings](http://www.philliplemons.com/posts/ray-casting-algorithm).


## Implementierung
- Programmiersprache: Rust (rustc 1.73.0 (cc66ad468 2023-10-03))
- Die SVG wurde eingelesen und in einzelne Files pro State aufgeteilt

<div style="page-break-after: always;"></div>

## main() - Pseudo Code
1. states = Liest die einzelnen Files ein
2. Für jeden state
   1. state_points = relative_file_to_absolute_vector
   2. state.set_holes_and_islands(state_points);
   3. state_vector.push(state);
   4. state.get_area()

### exampleState.get_area() - Pseudo Code
1. area += Fläche des Hauptbereichs
2. area += Fläche aller Inseln
3. area -= Flächer aller Löcher

### shoelace zur Berechnung der Fläche - Pseudo Code
1. Für alle Punkte eines States
   1. area += aktuellerPunkt.x * nächsterPunkt.y
   2. area -= nächsterPunkt.x * aktuellerPunkt.y
2. absoluter Betrag der area / 2

### point_inside_polygon() - Pseudo Code
1. Für alle Punkte des Polygons
   1. aktuellerPunkt
   2. punktVorAktuellemPunkt (n-1)
   3. inside = false
   4. Überprüfe, ob der Strahl vom Punkt aus die Kante des Polygons schneidet.Prüft, ob Y des aktuellen Punktes (oder des vorherigen Punktes) unterhalb des betrachteten Punktes liegt und ob die X-Koor der Kante links vom betrachteten Punkt liegt.
   5. Überprüfe, ob der Strahl die Kante schneidet und der Schnittpunkt rechts vom betrachteten Punkt liegt. --> inside = !inside
   6. aktuellerPunkt = punktVorAktuellemPunkt
2. Nach allen Punkten: return inside

<div style="page-break-after: always;"></div>

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
| Niedersachsen            | 40633.313  |
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


### Vergleich Berechnung - Statista
Quelle für Größe der Bundesländer laut Statista: [ Fläche der deutschen Bundesländer zum 31. Dezember 2022 ](https://de.statista.com/statistik/daten/studie/154868/umfrage/flaeche-der-deutschen-bundeslaender/)    

| Bundesland                | Fläche (berechnet) | Fläche (Statista) | Verhältnis |
|---------------------------|---------------|---------------|------------|
| Baden-Württemberg         | 30522,156     | 35748         | 0,85       |
| Bayern                    | 60026,28      | 70542         | 0,85       |
| Berlin                    | 766,1914      | 891           | 0,86       |
| Brandenburg               | 25275,941     | 29654         | 0,85       |
| Bremen                    | 340,93457     | 420           | 0,81       |
| Hamburg                   | 633,3203      | 755           | 0,84       |
| Hessen                    | 17977,523     | 21116         | 0,85       |
| Mecklenburg-Vorpommern    | 19658,783     | 23295         | 0,84       |
| Niedersachsen             | 40633,313     | 47710         | 0,85       |
| Nordrhein-Westfalen       | 28966,52      | 34113         | 0,85       |
| Rheinland-Pfalz           | 16913,566     | 19858         | 0,85       |
| Saarland                  | 2179,7573     | 2572          | 0,85       |
| Sachsen                   | 15667,891     | 18450         | 0,95       |
| Sachsen-Anhalt            | 17450,543     | 20467         | 0,77       |
| Schleswig-Holstein        | 13456,4375    | 15804         | 0,85       |
| Thueringen                | 13724,586     | 16202         | 0,85       |

Bei der Betrachtung der Tabelle fällt auf, dass fast überall ein Verhältnis von ca. 0,85 auftritt (bzw. ein Fehler von ca. 15%). Dadurch kann behauptet werden, dass die Messungen als korrekt angenommen werden können, wenn das Verhältnis übereinstimmt. Jedoch gibt es ein paar Ausreißer: Bremen, Sachsen und Sachsen-Anhalt

Relativer Fehler:
- Sachsen-Anhalt : 9,41 %
- Sachsen: 11,76 %
- Bremen: 4,71 %