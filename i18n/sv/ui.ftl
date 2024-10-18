none = Ingen
delete = Ta bort
settings = Inställningar
name = Namn
value = Värde
theme = Tema
update_delay = Uppdateringsfördröjning
update_delay_value = { $value } ms
temp_selection = Val av temperatur
min_temp = min temperatur
min_speed = min hastighet
max_temp = max temperatur
max_speed = max hastighet
idle_temp = overksam temperatur
idle_speed = overksam hastighet
load_temp = last temperatur
load_speed = last hastighet
launch_graph_window = Lägg till koordinater
config_saved = Konfigurationen har sparats

# Lägg till objektbeskrivning
add_item = Lägg till ett objekt
add_fan = Övervaka en fläktsensor
add_temp = Övervaka en temperatursensor
add_custom_temp = Definiera logik mellan värden (Max, Medel, ...)
add_control = Tilldelar ett visst beteende till en viss hårdvarukomponent
add_flat = Returnerar ett fast värde
add_linear = Tar 5 variabler:
    - en min och en maxtemperatur
    - en min och en maxhastighet
    - ett sensor värde
    om sensor < min temp -> min hastighet
    om sensor > max temp-> max hastighetd
    annars beräknas ett medelvärde (se ikon)
add_target = Tar 5 variabler:
    - en ideal och en triggertemp
    - en ideal och en triggerhastighet
    - ett sensor värde
    Om sensorn > triggertemperatur är triggerhastigheten inställd
    tills denna sensor är < ideal temperatur
add_graph = Graf

# Konfiguration
config_name = Konfigurationens namn
save_config = Spara/byt namn på denna konfiguration
delete_config = Ta bort konfiguration
create_config = Skapa konfiguration

# Fel
already_used_error = Det här namnet används redan
invalid_value_error = detta värde är ogiltigt
