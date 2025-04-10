none = Geen
delete = Verwijderen
settings = Instellingen
about = Over
name = Naam
value = Waarde
theme = Thema
update_delay = Updatevertraging
update_delay_value = { $value } ms
temp_selection = Temperatuur selecteren
min_temp = Minimum temperatuur
min_speed = Minimum snelheid
max_temp = Maximum temperatuur
max_speed = Maximum snelheid
idle_temp = Temperatuur bij inactiviteit
idle_speed = Snelheid bij inactiviteit
load_temp = Temperatuur bij systeembelasting
load_speed = Snelheid bij systeembelasting
launch_graph_window = Coördinaten toevoegen
config_saved = Configuratie succesvol opgeslagen
repository = Repository
donate = Doneren
issues_tracker = Meld een fout

# Add item description
add_item = Item toevoegen
add_fan = Ventilatorsensor toevoegen
add_temp = Temperatuursensor toevoegen
add_custom_temp = Defineer het logische verband tussen waarden (max, gemiddelde, ...)
add_control = 
Assigns a certain behavior to a certain hardware component
add_flat = Returns a fixed value
add_linear = Take 5 variables:
    - a min and a max temp
    - a min and a max speed
    - a sensor value
    if sensor < min temp -> min speed
    if sensor > max temp-> max speed
    otherwise, an average is calculated (see icon)
add_target = Take 5 variables:
    - a ideal and a trigger temp
    - a ideal and a trigger speed
    - a sensor value
    If the sensor > trigger temperature, trigger speed is set
    until this sensor is < ideal temperature
add_graph = Graph

# Config
config_name = Configuration name
save_config = Save/rename this configuration
delete_config = Delete configuration
create_config = Create configuration

# Error
already_used_error = This name is already being use
invalid_value_error = this value is invalid

# Warning
config_not_saved = Configuration not saved

# Dialogs
udev_rules_dialog_ok = I understand
udev_rules_dialog_remind_later = Remind me Later
udev_rules_dialog_copy_to_clipboard = Copy Commands to Clipboard
