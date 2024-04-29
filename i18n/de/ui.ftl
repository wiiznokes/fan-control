none = Keine
delete = Löschen
settings = Einstellungen
name = Name
value = Wert
theme = Thema
update_delay = Update-Verzögerung
update_delay_value = { $value } ms
temp_selection = Temperaturauswahl
min_temp = Mindesttemperatur
min_speed = Mindestgeschwindigkeit
max_temp = Maximaltemperatur
max_speed = Maximale Geschwindigkeit
idle_temp = Leerlauftemperatur
idle_speed = Leerlaufdrehzahl
load_temp = Lasttemperatur
load_speed = Lastgeschwindigkeit
launch_graph_window = Koordinaten hinzufügen

# Add item description
add_item = Ein Element hinzufügen
add_fan = Überwachung eines Lüftersensors
add_temp = Überwachung eines Temperatursensors
add_custom_temp = Definiere die Logik zwischen Werten (Max, Durchschnitt, ...)
add_control = Weist einer bestimmten Hardwarekomponente ein bestimmtes Verhalten zu
add_flat = Gibt einen festen Wert zurück
add_linear = Nimm 5 Variablen:
    - eine Mindest- und eine Höchsttemperatur
    - eine Mindest- und eine Höchstgeschwindigkeit
    - einen Sensorwert
    wenn Sensor < Mindesttemperatur -> Mindestgeschwindigkeit
    wenn Sensor > maximale Temperatur-> maximale Geschwindigkeit
    Andernfalls wird ein Durchschnitt berechnet (siehe Symbol).
add_target = Nimm 5 Variablen:
    - eine ideale und eine Auslösetemperatur
    - eine ideale und eine Auslösegeschwindigkeit
    - einen Sensorwert
    Wenn der Sensor > Auslösetemperatur ist, wird die Auslösegeschwindigkeit
    so lange gesetzt, bis dieser Sensor < Idealtemperatur ist.
add_graph = Diagramm

# Config
config_name = Konfiguration Name
save_config = Konfiguration speichern/umbenennen
delete_config = Konfiguration löschen
create_config = Konfiguration erstellen

# Error
already_used_error = Dieser Name wird bereits verwendet
invalid_value_error = dieser Wert ist ungültig
