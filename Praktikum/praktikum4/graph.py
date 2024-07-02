import pandas as pd
import matplotlib.pyplot as plt

# Einlesen der CSV-Datei
df = pd.read_csv('csv/cospherical.csv')
output1 = "graphs/cospherical-pdt.png"
output2 = "graphs/cospherical-pcs.png"

# Extrahieren der Dimension und der Anzahl der Punkte aus dem Befehl
df['Dimension'] = df['Befehl'].str.extract(r'rbox \d+ D(\d+)')
df['Anzahl Punkte'] = df['Befehl'].str.extract(r'rbox (\d+) D\d+')

# Konvertiere die extrahierten Werte zu numerischen Datentypen
df['Dimension'] = pd.to_numeric(df['Dimension'])
df['Anzahl Punkte'] = pd.to_numeric(df['Anzahl Punkte'])
df['CPU Seconds'] = pd.to_numeric(df['CPU Seconds'])

# Graph 1: Pro Dimension eine Linie (Anzahl der Punkte vs. Anzahl der Distance Tests)
plt.figure(figsize=(10, 6))
for dimension in df['Dimension'].unique():
    subset = df[df['Dimension'] == dimension]
    # plt.plot(subset['Anzahl Punkte'], subset['Distance Tests'], marker='o', label=f'Dimension {dimension}')
    plt.plot(subset['Anzahl Punkte'], subset['Distance Tests'], marker='o', label=f'distance tests')
plt.xlabel('Anzahl der Punkte')
plt.ylabel('Anzahl der Distance Tests')
plt.title('Verhältnis Distance Tests und Anzahl der Punkte')
plt.legend()
plt.xscale('log')
plt.yscale('log')
plt.grid(True, which="both", ls="--")
plt.savefig(output1)  # Speichern des Graphen in eine Datei
plt.show()

# Graph 2: Pro Dimension eine Linie (Anzahl der Punkte vs. Anzahl der CPU_Seconds)
plt.figure(figsize=(10, 6))
for dimension in df['Dimension'].unique():
    subset = df[df['Dimension'] == dimension]
    # plt.plot(subset['Anzahl Punkte'], subset['CPU Seconds'], marker='o', label=f'Dimension {dimension}')
    plt.plot(subset['Anzahl Punkte'], subset['CPU Seconds'], marker='o', label=f'cpu seconds')
plt.xlabel('Anzahl der Punkte')
plt.ylabel('CPU Seconds')
plt.title('Verhältnis CPU Seconds und Anzahl der Punkte')
plt.legend()
plt.xscale('log')
plt.yscale('log')
plt.grid(True, which="both", ls="--")
plt.savefig(output2)  # Speichern des Graphen in eine Datei
plt.show()
