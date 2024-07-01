import pandas as pd
import matplotlib.pyplot as plt

# Einlesen der CSV-Datei
df = pd.read_csv('rbox.csv')

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
    plt.plot(subset['Anzahl Punkte'], subset['Distance Tests'], marker='o', label=f'Dimension {dimension}')
plt.xlabel('Anzahl der Punkte')
plt.ylabel('Anzahl der Distance Tests')
plt.title('Anzahl der Distance Tests pro Dimension')
plt.legend()
plt.xscale('log')
plt.yscale('log')
plt.grid(True, which="both", ls="--")
plt.show()

# Graph 2: Pro Dimension eine Linie (Anzahl der Punkte vs. Anzahl der CPU_Seconds)
plt.figure(figsize=(10, 6))
for dimension in df['Dimension'].unique():
    subset = df[df['Dimension'] == dimension]
    plt.plot(subset['Anzahl Punkte'], subset['CPU Seconds'], marker='o', label=f'Dimension {dimension}')
plt.xlabel('Anzahl der Punkte')
plt.ylabel('CPU Seconds')
plt.title('CPU Seconds pro Dimension')
plt.legend()
plt.xscale('log')
plt.yscale('log')
plt.grid(True, which="both", ls="--")
plt.show()
