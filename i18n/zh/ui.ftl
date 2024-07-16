none = 空
delete = 删除
settings = 设置
name = 名称
value = 值
theme = 主题
update_delay = 更新延迟
update_delay_value = { $value } ms
temp_selection = 温度选择
min_temp = 最低温度
min_speed = 最小速度
max_temp = 最高温度
max_speed = 最大速度
idle_temp = 怠速温度
idle_speed = 怠速速度
load_temp = 负载温度
load_speed = 负载速度
launch_graph_window = 添加坐标

# Add item description
add_item = 添加项目
add_fan = 监控风扇传感器
add_temp = 监控温度传感器
add_custom_temp = 定义值之间的逻辑（最大值、平均值、 ...）
add_control = 将特定行为分配给特定硬件组件
add_flat = 返回一个固定值
add_linear = 取决于5个变量:
    - 最低和最高温度
    - 最小和最大速度
    - 一个传感器值
    如果传感器 < 最低温度 -> 最小速度
    如果传感器 > 最高温度 -> 最大速度
    否则，计算平均值（见图标）
add_target = 取决于5个变量:
    - 理想和触发温度
    - 理想和触发速度
    - 一个传感器值
    如果传感器 > 触发温度，会设置触发速度
    直到这个传感器 < 理想温度
add_graph = 图表

# Config
config_name = 配置名称
save_config = 保存/重命名此配置
delete_config = 删除配置
create_config = 创建配置

# Error
already_used_error = 此名称已被使用
invalid_value_error = 此值无效
