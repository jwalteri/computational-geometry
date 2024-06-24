import matplotlib.pyplot as plt

def read_lines_from_file(filename):
    lines = []
    with open(filename, 'r') as file:
        for line in file:
            # Entferne führende und nachfolgende Leerzeichen und splitte die Zeile nach Leerzeichen
            parts = line.strip().split()
            if len(parts) == 4:
                start_x, start_y, end_x, end_y = map(float, parts)
                lines.append((start_x, start_y, end_x, end_y))
    return lines

def generate_plot(filename, output_filename):
    lines = read_lines_from_file(filename)

    # Erstelle eine neue Figur
    plt.figure()

    # Füge jede Linie zum Plot hinzu
    for line in lines:
        start_x, start_y, end_x, end_y = line
        plt.plot([start_x, end_x], [start_y, end_y], color='blue', linewidth=0.5)

    plt.xlabel('X')
    plt.ylabel('Y')
    plt.title('Liniensegmente')
    plt.gca().set_aspect('equal', adjustable='box')
    plt.grid(True)
    plt.savefig(output_filename, format='png')
    plt.close()


# Definiere die Liniensegmente
# lines = [
#     (11.661, 99.158, 12.303, 98.2142),
#     (13.826, 39.723, 14.4292, 40.3226),
#     (64.2391, 60.3998, 64.766, 60.068),
#     (22.0666, 80.7662, 22.092, 80.782),
#     (40.679, 29.133, 40.9639, 29.4171),
#     (72.9043, 42.6164, 73.457, 42.752),
#     (33.098, 83.288, 33.7057, 83.3654),
#     (90.5472, 22.7331, 91.157, 22.422),
#     (84.4049, 31.5979, 84.638, 31.534),
#     (50.46, 72.164, 50.7028, 72.0683),
#     (59.1112, 34.2894, 59.349, 33.979),
#     (85.4035, 50.8396, 86.132, 51.16),
#     (42.196, 86.926, 42.4875, 86.4614),
#     (37.546, 63.217, 37.7671, 63.0458),
#     (9.061, 21.568, 9.68597, 22.0677),
#     (92.9745, 89.0119, 93.219, 88.95),
# ]
filename1 = 'G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_1.dat'  # Der Name der Datei, die die Liniensegmente enthält
filename2 = 'G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_10000_1.dat'
filename3 = 'G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_100000_1.dat'

generate_plot(filename1, 'output1.png')
generate_plot(filename2, 'output2.png')
generate_plot(filename3, 'output3.png')