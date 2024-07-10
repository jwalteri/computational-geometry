%% Computational Geometry - Praktikum 5
% Johannes Walter, Luca Biege
% MATLAB Version R2024a
% ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
% 11.06.2024: Datenstrukturen angelegt, Rahmenprogramm geschrieben
% 25.06.2024: linprog gefixt, Hinweis: MatLab minimiert bei linear
% Programming die Funktion f., plot erstellt
% ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

%% Einlesen Polygon-Dateien

realpolygonFile = fopen('../polygons/polygon.txt', 'r');
realpolygon = fscanf(realpolygonFile, '%f %f', [2 Inf])';
fclose(realpolygonFile);

testpolygonFile = fopen('../polygons/testpolygon.txt', 'r');
testpolygon = fscanf(testpolygonFile, '%f %f', [2 Inf])';
fclose(testpolygonFile);

% CHOOSE POLYGON:
polygon = realpolygon;
%polygon = testpolygon;

% debug
debug = true;
%% Definition Variablen, Errorhandling
% Kreismittelpunkt: (x_m, y_m)
% Radius: r -> Ziel: Maximieren!
% Bedingung: Abstand M zu Polygonseite >= r

% Überprüfen, ob der erste und der letzte Eintrag gleich sind
if isequal(polygon(1, :), polygon(end, :))
    polygon(end, :) = [];
end

% Überprüfen, ob das Polygon konvex ist
if ~is_convex(polygon)
    error('Das Polygon ist nicht konvex. Der Algorithmus funktioniert nur für konvexe Polygone.');
end

% Anzahl der Eckpunkte des Polygons
n = size(polygon, 1);

% Aufstellen der linearen Ungleichungen
A = zeros(n, 3);
b = zeros(n, 1);

%% Kante zwischen Punkt i und i+1 (letzte Kante schließt zum ersten Punkt)

% debug:
if debug
    normals = zeros(n, 2);
    midpoints = zeros(n, 2);
    directions = zeros(n, 2);
    centroids = zeros(n, 2);
end

centroid = mean(polygon); % Schwerpunkt des Polygons

for i = 1:n
    if i < n
        x1 = polygon(i, 1);
        y1 = polygon(i, 2);
        x2 = polygon(i+1, 1);
        y2 = polygon(i+1, 2);
    else
        x1 = polygon(i, 1);
        y1 = polygon(i, 2);
        x2 = polygon(1, 1);
        y2 = polygon(1, 2);
    end
    
    % normierter Normalenvektor der Kante
    normal = [y2 - y1, -(x2 - x1)];
    normal = normal / norm(normal);

    % Überprüfen, ob der Normalenvektor nach Außen zeigt, sonst umdrehen
    midpoint = [(x1 + x2)/2, (y1 + y2)/2];
    direction = midpoint - centroid; % Richtung vom Schwerpunkt zum Mittelpunkt der Seite
    if dot(normal, direction) < 0
        normal = -normal; % Normalenvektor umdrehen, wenn er nach innen zeigt
    elseif debug
        fprintf("Skalar: %f, Midp: %f, %f   Dir: %f, %f\n", dot(normal, direction), midpoint(1), midpoint(2), direction(1), direction(2));
    end

    % debug:
    if debug
        midpoints(i, :) = midpoint;
        directions(i, :) = direction;
        normals(i, :) = normal;
        centroids(i, :) = centroid;
    end

    % Ungleichung für die Seite
    A(i, :) = [normal, 1]; % normal * [x_m, y_m] - r >= 0
    b(i) = normal * [(x1 + x2)/2; (y1 + y2)/2];
end

% debug:
if debug
    figure;
    hold on;
    quiver(midpoints(:, 1), midpoints(:, 2), normals(:, 1), normals(:, 2), 0.5, 'r');
    quiver(centroids(:, 1), centroids(:, 2), directions(:, 1), directions(:, 2), 0.5, 'g');
    plot(centroid(1), centroid(2), 'k.', 'MarkerSize', 10); % Mittelpunkt zeichnen
    hold off;
    title("debug: normals")
end

% Zielfunktion: maximiere r (also: -(minimiere r))
f = [0, 0, -1];

% Lineares Programm lösen
options = optimoptions('linprog', 'Display', 'off'); % Zwischenlösungen ausblenden
[x, fval, exitflag, output] = linprog(f, A, b, [], [], zeros(size(f)), [], options);

% Ergebnisse
if exitflag == 1
    x_m = x(1);
    y_m = x(2);
    r = abs(fval);
    fprintf('Kreismittelpunkt: (%.2f, %.2f)\n', x_m, y_m);
    fprintf('Radius: %.2f\n', r);
    draw_poly_and_circle(polygon, x_m, y_m, r)
else
    fprintf('Das Problem konnte nicht gelöst werden: exitflag = %d\n', exitflag);
    figure;
    hold on;
    draw_poly(polygon)
    hold off;
    
end

%% Zeichnen
function draw_poly(poly)
    fill(poly(:,1), poly(:,2), [0.9290 0.6940 0.1250]); % Polygon zeichnen
    axis equal;
    title('konvexes Polygon');
end

function draw_poly_and_circle(poly, x_m, y_m, r)
    figure;
    hold on;

    draw_poly(poly)
    viscircles([x_m, y_m], r, 'EdgeColor', [0 0.4470 0.7410]); % Kreis zeichnen
    plot(x_m, y_m, 'k.', 'MarkerSize', 10); % Mittelpunkt zeichnen

    hold off;    
    title('Größter einbeschreibbarer Kreis im konvexen Polygon');
end

%% Funktion für Test auf Konvexität
function convex = is_convex(polygon)
    % Überprüft, ob das Polygon konvex ist
    n = size(polygon, 1);
    if n < 3
        convex = false;
        return;
    end
    % Prüfe das Vorzeichen der Kreuzprodukte der aufeinanderfolgenden Kanten
    % Wenn die Vorzeichen alle gleich sind, ist das Polygon konvex
    signs = zeros(n, 1);
    for i = 1:n
        dx1 = polygon(mod(i, n)+1, 1) - polygon(i, 1);
        dy1 = polygon(mod(i, n)+1, 2) - polygon(i, 2);
        dx2 = polygon(mod(i+1, n)+1, 1) - polygon(mod(i, n)+1, 1);
        dy2 = polygon(mod(i+1, n)+1, 2) - polygon(mod(i, n)+1, 2);
        cross_product = dx1 * dy2 - dy1 * dx2;
        signs(i) = sign(cross_product);
    end
    convex = all(signs == signs(1));
end