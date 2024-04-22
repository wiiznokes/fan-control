none = Aucun
delete = Supprimer
settings = Paramètre
name = Nom
value = Valeur
theme = Thème
update_delay = Délai de mise à jour
update_delay_value = { $value } ms
temp_selection = Temp selection
min_temp = min temp
min_speed = min speed
max_temp = max temp
max_speed = max speed
idle_temp = idle temp
idle_speed = idle speed
load_temp = load temp
load_speed = load speed
launch_graph_window = Ajout de coordonnées

# Add item description
add_item = Ajouter un item
add_fan = Monitore un ventilateur
add_temp = Monitore un capteur de temperature
add_custom_temp = Defini une logique entre des valeurs (Max, Moyenne, ...)
add_control = Applique un certain comportement a un ventilateur
add_flat = Retourne une valeur fixe
add_linear = Prend 5 variables :
    - une température minimale et maximale
    - une vitesse minimale et maximale
    - une valeur de capteur
    Si le capteur < température minimale -> vitesse minimale
    Si le capteur > température maximale -> vitesse maximale
    Sinon, une moyenne est calculée (voir icône)
add_target = Prend 5 variables :
    - une température idéale et une température de déclenchement
    - une vitesse idéale et une vitesse de déclenchement
    - une valeur de capteur
    Si le capteur > température de déclenchement, la vitesse de déclenchement est définie
    jusqu'à ce que ce capteur < température idéale
add_graph = Graphe

# Config
config_name = Nom de la configuration
save_config = Enregistrer/renommer cette configuration
delete_config = Supprimer la configuration
create_config = Créer une configuration

# Error
already_used_error = Ce nom est déjà utilisé
invalid_value_error = Cette valeur est invalide