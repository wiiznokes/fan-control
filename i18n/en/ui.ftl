none = None
delete = Delete
settings = Settings
about = About
name = Name
value = Value
theme = Theme
update_delay = Update delay
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
launch_graph_window = Add coordinates
config_saved = Configuration successfully saved
repository = Repository
donate = Donate
issues_tracker = Report an Issue

# Add item description
add_item = Add an item
add_fan = Monitor a fan sensor
add_temp = Monitor a temp sensor
add_custom_temp = Define logic between values (Max, Averrage, ...)
add_control = Assigns a certain behavior to a certain hardware component
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