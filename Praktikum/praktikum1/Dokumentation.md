# Computational Geometry

## Praktikum 1

**Titel:** Bestimmung der Anzahl der sich schneidenden Strecken inklusive Zeitmessung.  
**Gruppe:** Johannes Walter, Luca Biege  
**Datum:** 05.04.2024  


# Einleitung

Im folgenden Praktikumsversuch geht es darum, einen Code zu entwickeln, welcher die Anzahl von Schnittstellen vieler Strecken berechnet und diese ausgibt. Die Strecken sind durch ein Punktpaar mit den Koordinaten [x1, y1, x2, x2] definiert und in einer *.dat* Datei gespeichert. Hierbei entpricht jede Zeile einer Strecke mit dem Startpunkt *P1* und dem Endpunkt *P2*. Die drei Dateien beinhalten 1000, 10.000 und 100.000 Strecken.  
Dabei soll die Dauer für die Ausführung des Programms gemessen und verglichen werden.  
Als Programmiersprache wird *Rust* verwendet.

# Grundlagen

**Strecken** unterscheiden sich von Geraden darin, dass sie einen klaren Start- und Endpunkt besitzen und damit eine endliche Länge haben. Eine Form zur Darstellung von Strecken mit dem Startpunkt P1 und dem Endpunkt P2 ist:
$$ \vec{x} = \overrightarrow{P1} + t \cdot \vec{d} \ \ \ \ \ | \ t \in \mathbb{R} \text{ mit } 0 \leq t \leq 1 \ | $$
$$ \text{mit} \ \ \ \ \ \vec{d} = \overrightarrow{P1P2} = \overrightarrow{P2} - \overrightarrow{P1} $$

Zwei Strecken haben einen gemeinsamen **Schnittpunkt**, wenn sie mindestens einen gemeinsamen Punkt beinhalten. Das gilt auch, wenn exakt der Endpunkt einer Strecke auf der anderen Strecke liegt (wie ein "T"), oder wenn die beiden Strecken auf derselben Geraden liegen und sich dabei berühren (teilweise oder komplett die eine Strecke innerhalb der anderen liegt). Die Abbildung zeigt exemplarisch Möglichkeiten für Streckenpaare mit (mindestens) einem Schnittpunkt:

![Beispiele für Streckenpaare mit einem Schnittpunkt.](bilder/Strecken_Schnittpunkte.png)

Die **Determinante** einer 2x2 Matrix gibt eine Aussage über die Abhängigkeit der Vektoren einer Matrix. In diesem Fall wird sie verwendet um zu prüfen, wie zwei (Richtungs-) Vektoren zueinander stehen. Ist das Ergebnis der Berechnung $ det(A) = 0 $, so bedeutet dies, dass die Vektoren in dieselbe Richtung zeigen.

$$ det(A) = \begin{bmatrix}
a & b \\
c & d \\ 
\end{bmatrix}
 = a \cdot d - c \cdot b
$$

Das **Kreuzprodukt** zweier Vektoren ist ein Normalenvektor, also ein Vektor, welcher senkrecht auf diesen beiden Vektoren steht. Die Länge des Vektors gibt eine Aussage über die durch die beiden Vektoren aufgespannte Fläche. Sind die Vektoren parallel zueinander, spannen sie keine Fläche auf, d.h. das Kreuzprodukt $ \vec{a} \times \vec{b} = 0 $. Das Kreuzprodukt zweier 2-dimensionaler Vektoren lässt sich folgendermaßen berechnen:

$$ \vec{a} \times \vec{b} = \begin{bmatrix} a1 \\ a2 \\ \end{bmatrix} \times \begin{bmatrix} b1 \\ b2 \\ \end{bmatrix} = a1 \cdot b2 - a2 \cdot b1
$$
 

# Ausführung

Zur besseren Übersicht sind die Koordinaten zeilenweise aus der Datei ausgelesen und einem (n x 4)-Vektor `points` als float-Werte gespeichert. Fehlerhafte Zeilen werden dabei übersprungen und vermerkt.  
Anschließend wird in einer verschachtelten for-Schleife jede Zeile von `points` ausgelesen und die Koordianten als `Point` P1 und P2, bzw. für die zweite Strecke als P3 und P4 gespeichert. Aus ihnen lassen sich die Richtungsvektoren D12 und D34 berechnen.
```rust
struct Point {
    x: f64,
    y: f64,
}
```
Im folgenden Abschnitt werden zwei gefundene Lösungsansätze für die weitere Berechnung erläutert und analysiert.

## Ansatz geometrisch über Dreisatz
Ein elementarer Test, um herauszufinden, ob sich zwei Strecken schneiden, ist das Gleichsetzen der Parametergleichungen. Punkte entlang der Strecke werden dabei durch die Verwendung eines Parameters beschrieben. Beide Strecken werden durch eine Parametergleichung dargestellt.
Gegeben sind zwei Strecken:
$$ p := [p_1 p_2] \text{ und } q := [q_1 q_2] \text{ im } ℝ² $$


![Beispiele für Streckenpaare mit einem Schnittpunkt.](bilder/Strecken_Schnitt.png) (Nach Prof. M. Fischer, HS München)

Berechnen des Schnittpunkts durch Gleichsetzen:
$$ p_1 + \lambda(p_2 - p_1) = q_1 + \mu(q_2 - q_1) $$

Die Parameter $\lambda$ und $\mu$ repräsentieren Punkte entlang der Strecke. Sie nehmen immer Werte zwischen 0 und 1 an. So erhält man den Ausgangspunkt der Strecke für den Wert 0 und den Endpunkt der Strecke für den Wert 1.
Löst man das obige Gleichungssystem und erhält für $\lambda$ oder $\mu$ einen Wert außerhalb des Bereichs von 0 bis 1, bedeutet das, dass ein Punkt außerhalb der Strecken liegt und somit kein Schnittpunkt ist.

Dieser Ansatz lässt sich mit Stift und Papier leicht umsetzen. Für die Berechnung mit Code bietet sich jedoch eine Lösung mithilfe von Determinaten an, da dies in Code sehr viel lesbarer und anwendbarer umsetzen lässt.

Quelle: Vorlesungsunterlagen, 02Grundlagen, Prof. M. Fischer, HS München

## Ansatz über Determinante

[Luca Ansatz]


# Test mit GeoGebra

[]


# Auswertung
Im vorliegenden Praktikum wurden drei Files mit Strecken ausgewertet.

## s_1000_1.dat

| Datei | Anzahl Schnittpunkte | Berechnungsdauer |
|----------|----------|----------|
| s_1000_1.dat     |  15  |  12 ms  |
| s_10000_1.dat    | 737   | 1.173 ms   |
| s_100000_1.dat   | 77.139   | 11.7203 ms   |

