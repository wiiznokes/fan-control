none = Nenhum
delete = Excluir
settings = Configurações
about = Sobre
name = Nome
value = Valor
theme = Tema
update_delay = Atualizar atraso
update_delay_value = { $value } ms
temp_selection = Seletor de temperatura
min_temp = min temp
min_speed = min speed
max_temp = max temp
max_speed = max speed
idle_temp = idle temp
idle_speed = idle speed
load_temp = load temp
load_speed = load speed
launch_graph_window = Adicionar coordenadas
config_saved = Configuração salva com sucesso
repository = Repositório
donate = Fazer uma doação
issues_tracker = Relatar um problema

# Add item description
add_item = Adicionar um item
add_fan = Monitorar um sensor de ventilador
add_temp = Monitorar um sensor de temperatura
add_custom_temp = Definir uma lógica entre valores (Max, Média, ...)
add_control = Atribui um determinado comportamento a um determinado componente de hardware
add_flat = Retorna um valor fixo
add_linear = Pegue 5 variáveis:
    - uma temperatura mínima e máxima
    - uma velocidade mínima e máxima
    - um sensor de valor
    se sensor < temp. mínima -> velocidade mínima
    se sensor > temp. máxima -> velocidade máxima
    caso contrário, uma média é calculada (veja o ícone)
add_target = Pegue 5 variáveis:
    - uma temperatura ideal e uma de gatilho
    - uma velocidade ideal e uma de gatilho
    - um sensor de valor
    Se o sensor > temp. de gatilho, a velocidade de gatilho é definida
    até que este sensor seja < temperatura ideal
add_graph = Gráfico

# Config
config_name = Nome da configuração
save_config = Salvar/renomear esta configuração
delete_config = Excluir configuração
create_config = Criar configuração

# Error
already_used_error = Este nome já está sendo usado
invalid_value_error = este valor é inválido

# Warning
config_not_saved = Configuração não salva

# Dialogs
udev_rules_dialog_ok = Eu compreendo
udev_rules_dialog_remind_later = Lembre-me depois
udev_rules_dialog_copy_to_clipboard = Copiar comandos para a área de transferência
