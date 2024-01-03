none = None
delete = Delete
settings = Settings
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

# Config
config_name = Configuration name
save_config = Save active configuration
delete_config = Delete configuration
create_config = Create configuration

# Error
already_used_error = This name is already being use
invalid_value_error = this value is invalid