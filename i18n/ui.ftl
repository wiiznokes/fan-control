none = Nessuno
delete = Elimina
settings = Impostazioni
name = Nome
value = Valore
theme = Tema
update_delay = Ritardo di aggiornamento
update_delay_value = { $value } ms
temp_selection = Selezione della temperatura
min_temp = temp min
min_speed = velocità min
max_temp = temp max
max_speed = velocità max
idle_temp = idle temp
idle_speed = idle speed
load_temp = load temp
load_speed = load speed
launch_graph_window = Aggiungi coordinate

# Add item description
add_item = Aggiungi un elemento
add_fan = Monitorare un sensore della ventola
add_temp = Monitorare un sensore di temperatura
add_custom_temp = Definire la logica tra i valori (Max, Media, ...)
add_control = Assegna un determinato comportamento a un determinato componente hardware
add_flat = Restituisce un valore fisso
add_linear = Prendi 5 variabili:
    - una temperatura minima e massima
    - una velocità minima e una massima
    - un valore del sensore
    se sensore < min temp -> velocità min
    se sensore > max temp-> velocità max
    altrimenti viene calcolata una media (vedi icona)
add_target = Prendi 5 variabili:
    - un ideale e una temperatura di attivazione
    - una velocità ideale e una velocità di attivazione
    - un valore del sensore
    Se il sensore > temperatura di attivazione, viene impostata la velocità di attivazione
     fino a quando questo sensore < temperatura ideale
add_graph = Grafico

# Config
config_name = Nome della configurazione
save_config = Salva/rinomina questa configurazione
delete_config = Elimina configurazione
create_config = Crea configurazione

# Error
already_used_error =  Questo nome è già in uso
invalid_value_error = questo valore non è valido
