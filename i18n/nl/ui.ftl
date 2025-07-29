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
min_temp = Minimumtemperatuur
min_speed = Minimumsnelheid
max_temp = Maximumtemperatuur
max_speed = Maximumsnelheid
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
add_control = Een specifieke actie aan een specifieke hardwarecomponent toewijzen
add_flat = Geeft een vaste waarde als resultaat
add_linear = Gebruik 5 variabelen:
    - een minimum- en een maximumtemperatuur
    - een minimum- en een maximumsnelheid
    - een sensorwaarde
    als sensorwaarde < minimumtemperatuur; dan minimumsnelheid
    als sensorwaarde > maximumtemperatuur; dan maximumsnelheid
    anders wordt er een gemiddelde genomen (zie icoontje)

add_target = Gebruik 5 variabelen:
    - een optimum- en een triggertemperatuur
    - een optimum- en een triggersnelheid
    - een sensorwaarde
    als sensorwaarde > triggertemperatuur; dan wordt de triggersnelheid ingesteld
    totdat sensorwaarde < optimumtemperatuur

add_graph = Grafiek

# Config
config_name = Configuratienaam
save_config = Deze configuratie opslaan/hernoemen
delete_config = Configuratie verwijderen
create_config = Configuratie aanmaken

# Error
already_used_error = Deze naam wordt al gebruikt
invalid_value_error = Deze waarde is ongeldig

# Warning
config_not_saved = De configuratie is niet opgeslagen

# Dialogs
udev_rules =
    .ok = Ik begrijp het
    .copy_to_clipboard = Commando's naar het klembord kopiëren
    .remind_later = Herinner me er later aan
