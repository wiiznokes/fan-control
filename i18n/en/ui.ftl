fan = Fan
none = None
config_name = Configuration name
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