%% Computational Geometry - Praktikum 5
% Johannes Walter, Luca Biege
% MATLAB Version R2024a
% Date: 11.06.2024
%
%
% Code bis jetzt nur anhand Aufgabenstellung copy pasta aus ChatGPT !!!

%% Einlesen Polygon-Dateien

polygonFile = fopen('../polygons/polygon.txt', 'r');
polygon = fscanf(polygonFile, '%f %f', [2 Inf])';
fclose(polygonFile);

testpolygonFile = fopen('../polygons/testpolygon.txt', 'r');
testpolygon = fscanf(testpolygonFile, '%f %f', [2 Inf])';
fclose(testpolygonFile);

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
%n = size(polygon, 1);
n = size(testpolygon, 1);

% Aufstellen der linearen Ungleichungen
A = zeros(n, 3);
b = zeros(n, 1);

%% Kante zwischen Punkt i und i+1 (letzte Kante schließt zum ersten Punkt)
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
    
    % Ungleichung für die Seite
    A(i, :) = [normal, -1]; % normal * [x_m, y_m] - r >= 0
    b(i) = normal * [(x1 + x2)/2; (y1 + y2)/2];
end

% Zielfunktion: maximiere r
f = [0, 0, -1];

% Lineares Programm lösen
options = optimoptions('linprog', 'Display', 'off');
[x, fval, exitflag] = linprog(f, [], [], A, b, [], [], options);

% Ergebnisse
if exitflag == 1
    x_m = x(1);
    y_m = x(2);
    r = -fval;
    fprintf('Kreismittelpunkt: (%.2f, %.2f)\n', x_m, y_m);
    fprintf('Radius: %.2f\n', r);
else
    fprintf('Das Problem konnte nicht gelöst werden.\n');
end

%% Zeichnen

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