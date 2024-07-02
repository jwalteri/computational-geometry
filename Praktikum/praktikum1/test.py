import matplotlib.pyplot as plt

def read_points_from_file(file_path):
    points_list = []
    
    with open(file_path, 'r') as file:
        for line in file:
            # Entfernen von führenden und nachfolgenden Leerzeichen und Zeilenumbrüchen
            line = line.strip()
            if not line:
                continue
            
            # Teilen der Koordinaten in x1, y1, x2, y2
            coords = line.split()
            if len(coords) != 4:
                raise ValueError(f"Ungültiges Format in Zeile: {line}")
            
            # Umwandeln der Koordinaten in float und als Tupel speichern
            x1, y1, x2, y2 = map(float, coords)
            points = ((x1, y1), (x2, y2))
            points_list.append(points)
    
    return points_list

def plot_lines(points_list):
    plt.figure()
    
    for points in points_list:
        (x1, y1), (x2, y2) = points
        plt.plot([x1, x2], [y1, y2], marker='o')
    
    plt.xlabel('X')
    plt.ylabel('Y')
    plt.title('Plot der Liniensegmente')
    plt.grid(True)
    plt.show()

# Beispielaufruf
file_path = 'strecken/s_1000_1.dat'
points = read_points_from_file(file_path)

# Plotte die Linien
plot_lines(points)
