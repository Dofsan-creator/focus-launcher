app-name = Focus Launcher
lang-title = Escolha seu idioma
welcome-title = Boas-vindas
welcome-body = Tudo aqui funciona com controle, mouse ou teclado. Nada muda no seu PC sem sua permissão.
welcome-autostart = Abrir ao iniciar a sessão
welcome-sim = As sessões começam em simulação: você vê o que mudaria e nada é alterado.
welcome-continue = Continuar

filter-all = Tudo
filter-games = Jogos
filter-software = Programas
filter-social = Social

top-add = Adicionar
top-stores = Lojas
top-settings = Configurações
top-power = Energia

home-resume = Continuar
home-empty-title = Nada aqui ainda
home-empty-body = Os jogos instalados aparecem sozinhos. Você também pode adicionar um programa.
home-played = { $hours ->
    [one] { $hours } hora de uso
   *[other] { $hours } horas de uso
}
home-running = Em execução

detail-play = Jogar
detail-open = Abrir
detail-favorite = Favoritar
detail-unfavorite = Remover dos favoritos
detail-remove = Remover da biblioteca
detail-launches = { $count ->
    [one] { $count } inicialização
   *[other] { $count } inicializações
}
detail-source = Origem: { $store }

prep-title = Preparando
prep-simulation = Simulação: nada será alterado.
prep-real = Essas mudanças serão desfeitas quando você terminar.
prep-nothing = Nada a mudar nesta sessão.
prep-start = Iniciar
prep-cancel = Cancelar
prep-throttle = Desacelerar em segundo plano: { $name }
prep-suspend = Pausar: { $name }
prep-close = Fechar: { $name }
prep-service = Parar por enquanto: { $name }
prep-awake = Manter o PC acordado
prep-power = Plano de energia de alto desempenho
prep-ask = Você decide: { $name }
prep-no-guardian = Mudanças de serviços foram ignoradas: o serviço auxiliar não está instalado.

end-title = Sessão encerrada
end-time = { $minutes ->
    [one] { $minutes } minuto
   *[other] { $minutes } minutos
}
end-restored = Tudo foi restaurado.
end-problem = Alguns itens não puderam ser restaurados. Os detalhes estão no registro.
end-back = Voltar à biblioteca

panel-title = Painel rápido
panel-return = Voltar para ele
panel-close-entry = Fechar
panel-library = Voltar à biblioteca
panel-none = Nada em execução

close-title = Fechar { $title }?
close-warning = O progresso não salvo será perdido.
close-social = Uma chamada ou transmissão pode parar.
close-no = Não
close-yes = Sim, fechar
close-waiting = Aguardando o fechamento. Talvez ele peça para salvar.
close-force = Forçar fechamento
close-force-sure = Forçar o fechamento pode perder trabalho. Tem certeza?

set-title = Configurações
set-language = Idioma
set-lang-status = Status da tradução: { $status }
status-reviewed = revisada
status-draft = a revisar
set-autostart = Abrir ao iniciar a sessão
set-real = Otimização real
set-real-body = Desligada: só simulação. Ligada: apps em segundo plano são desacelerados durante a sessão e depois restaurados.
set-real-confirm = Permitir mudanças reais neste PC durante as sessões?
set-keep-social = Não mexer nos apps sociais
set-awake = Manter o PC acordado durante a sessão
set-high-perf = Plano de energia de alto desempenho
set-big-picture = Bloquear o outro modo de tela cheia do Steam
set-sounds = Sons
set-motion = Reduzir animações
set-theme = Tema conforme o horário
set-glyphs = Ícones dos botões
glyph-auto = Automático
glyph-playStation = Formas
glyph-xbox = Letras
set-detect = Detectar controle
set-guardian = Serviço auxiliar
set-guardian-body = Necessário só para pausar serviços do Windows. O Windows pede permissão uma vez, e essa janela precisa de mouse ou teclado.
set-guardian-on = Instalado
set-guardian-off = Não instalado
set-guardian-install = Instalar
set-analysis = Análise do sistema
state-on = Ligado
state-off = Desligado
yes = Sim
no = Não

ctl-family = Controle: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Não detectado
ctl-mode = Modo: { $mode }
mode-auto = Auto
mode-native = Nativo
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Nenhum
ctl-connected = Controle conectado: { $family }

an-title = Análise do sistema
an-processes = Processos: { $count }
an-services = Serviços: { $count }
an-memory = Memória: { $gb } GB
an-cpu = Processador: { $name }
an-top = Os que mais usam memória
an-loading = Lendo seu sistema. Nada é alterado.

add-title = Adicionar um programa
add-body = Escolha um arquivo. Ele é lido, nunca executado.
add-up = Pasta acima
add-suggest = Sugerir desta pasta
add-suggested = Arquivos mais prováveis
add-name = Nome
add-kind = Tipo
add-save = Adicionar à biblioteca
add-script = Isto é um script. Confira o caminho completo antes de adicionar.
kind-game = Jogo
kind-software = Programa
kind-social = Social
kind-other = Outro

store-title = Lojas e apps
store-install = Instalar
store-page = Página oficial
store-confirm = Instalar { $name } de { $publisher }?
store-source = Origem: gerenciador de pacotes do Windows
store-uac = O Windows pode pedir permissão. Essa janela precisa de mouse ou teclado.
store-progress = Instalando { $name }
store-done = Instalação concluída.
store-failed = A instalação falhou. Você pode usar a página oficial.

power-title = Energia
power-quit = Sair para a área de trabalho
power-restart = Reiniciar
power-shutdown = Desligar
power-sure = Tem certeza?

kbd-done = Pronto
kbd-space = Espaço
kbd-delete = Apagar

hint-select = Escolher
hint-back = Voltar
hint-panel = Painel rápido

err-generic = Algo deu errado. Os detalhes estão no registro.
autostart-disabled = O launcher falhou ao iniciar duas vezes, então a abertura ao iniciar a sessão foi desligada.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Pesquisar
hint-options = Opções
home-searching = Resultados para “{ $text }”
detail-remove-confirm = Remover { $title } da biblioteca?
set-section-console = Modo console
set-console = Colocar o Windows no modo console enquanto o Focus estiver aberto
set-console-body = A barra de tarefas some e as notificações ficam em silêncio. Saia do Focus para voltar ao Windows de sempre.
set-hide-taskbar = Ocultar a barra de tarefas
set-silence = Silenciar notificações
set-start-minimized = Iniciar minimizado ao abrir a sessão
set-section-opt = Desempenho
set-section-sound = Som e tela
set-music = Música ambiente
set-vibration = Vibração do controle
opt-off = Desativado
opt-off-body = Simulação: você vê o que mudaria, nada é alterado.
opt-balanced = Equilibrado
opt-balanced-body = Os apps em segundo plano ficam mais lentos durante uma sessão e depois são restaurados.
opt-max = Máximo
opt-max-body = Todas as alavancas permitidas: apps mais lentos, atualizações pausadas, alto desempenho. Depois tudo é restaurado.
power-quit-body = O Windows volta ao normal.
power-restart-body = Reinicia o PC.
power-shutdown-body = Desliga o PC.
con-title = Conexões
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Procurar de novo
con-searching = Procurando…
con-connected = Conectado
con-connected-to = Conectado a { $name }
con-connect = Conectar
con-disconnected = Desconectado.
con-password = Senha
con-none = Nenhuma rede encontrada.
con-ethernet = Conexão com fio
con-offline = Sem conexão
con-result-success = Conectado.
con-result-badPassword = Senha incorreta.
con-result-timeout = A rede não respondeu.
con-result-failed = Falha na conexão.
con-result-notFound = Rede não encontrada. Procure de novo.
con-result-noAdapter = Nenhum adaptador Wi-Fi encontrado.
bt-paired = Seus dispositivos
bt-search = Procurar dispositivos
bt-pair = Parear
bt-pair-hint = Coloque o dispositivo no modo de pareamento e depois procure.
bt-forget-confirm = Esquecer { $name }?
bt-forgotten = Dispositivo esquecido.
bt-result-success = Dispositivo pareado.
bt-result-failed = Falha no pareamento.
setup-step = Etapa { $current }
setup-next = Próximo
setup-back = Voltar
setup-start = Começar
setup-console-title = Seu PC vira um console
setup-console-body = Enquanto o Focus estiver aberto, o Windows sai de cena: sem barra de tarefas, sem notificações. Saia do Focus e tudo volta ao normal.
setup-pad-title = Seu controle
setup-pad-body = Escolha como os botões aparecem. O Focus também funciona com mouse e teclado.
setup-net-title = Fique conectado
setup-net-body = Wi-Fi e Bluetooth estão sempre a um botão de distância, mesmo durante um jogo.
setup-lib-title = Sua biblioteca
setup-lib-body = { $count ->
    [one] { $count } jogo ou app encontrado neste PC.
   *[other] { $count } jogos e apps encontrados neste PC.
}
setup-opt-title = Desempenho durante as sessões
setup-opt-body = O Focus pode liberar o PC enquanto você joga e depois deixar tudo como estava. Você escolhe e pode mudar depois.
setup-comfort-title = Som e conforto
setup-comfort-body = Ajuste o clima. Abrir o Focus ao iniciar a sessão faz o PC ligar como um console.
setup-done-title = Tudo pronto
setup-done-body = Sua biblioteca está esperando. Aperte Começar para entrar.
setup-open-wifi = Configurar o Wi-Fi
setup-open-bt = Parear um controle
store-installed = Instalado
filter-stores = Lojas
launch-starting = Iniciando…
store-open = Abrir
panel-screenshot = Captura de tela
shot-saved = Captura salva em Imagens\Focus.
detail-update = Atualização disponível
detail-update-open = Atualizar
power-sleep = Suspender
power-sleep-body = O PC entra em suspensão, como o repouso de um console.
end-auto = Voltando à biblioteca…
con-ethernet-on = Conectado por cabo (Ethernet)
ctl-native-note = Fora do Steam, o Focus Input substitui o controle PlayStation por um controle Xbox que todo jogo reconhece. Para evitar comandos duplicados é preciso o HidHide (na loja do Focus, Drivers): o Focus esconde o controle real do jogo durante a partida e o devolve depois. Sem HidHide, “Auto” deixa o controle como está.
steam-guide-on = No Steam, o botão central do controle ainda traz o Steam por cima dos seus jogos. No Steam: Configurações, Controle, Configurações avançadas, desative a opção do botão Guia que traz o Steam para frente e a configuração de combinações do botão Guia.
steam-guide-off = O Steam deixa o botão central para o Focus.
steam-guide-open = Abrir as configurações de controle do Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Preparando o PC…
profile-on = Modo console
profile-off = Modo normal
unit-gb = GB
profile-flash = Seu PC está no modo console
profile-shell = Área de trabalho do Windows oculta: ela volta quando você sai do Focus
profile-freed = { $gb } GB de memória liberados para os seus jogos
profile-power = Potência máxima ativada
profile-slowed = { $count ->
    [one] { $count } programa desnecessário desacelerado
   *[other] { $count } programas desnecessários desacelerados
}
profile-closed = { $count ->
    [one] { $count } programa fechado
   *[other] { $count } programas fechados
}
profile-paused = { $count ->
    [one] { $count } tarefa em segundo plano do Windows pausada
   *[other] { $count } tarefas em segundo plano do Windows pausadas
}
profile-title = O que o Focus faz por você
profile-lead = Enquanto o Focus estiver aberto, seu PC se comporta como um console: tudo o que o jogo não precisa fica de lado. Tudo volta quando você sai do Focus.
profile-lead-off = A área de trabalho está oculta, mas seu PC ainda não está aliviado para jogar. Ative o desempenho nas configurações.
profile-none = Seu PC está no estado normal. Ative o modo console nas configurações para que ele se comporte como um console.
profile-memory = Memória livre para os seus jogos
profile-power-label = Potência
power-plan-balanced = Equilibrada
power-plan-high = Máxima
power-plan-saver = Economia de energia
power-plan-other = Personalizada
apps-title = { $count ->
    [one] { $count } aplicativo ainda está aberto
   *[other] { $count } aplicativos ainda estão abertos
}
apps-body = Fechá-los libera o PC para o jogo. Se algum pedir para salvar, ele continua aberto: nada se perde.
apps-closing = Fechando…
apps-close = Fechar
apps-keep = Manter abertos
set-stop-shell = Ocultar completamente a área de trabalho do Windows

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = Downloads
dl-empty = Nada está sendo baixado no momento.
dl-steam = Jogos da Steam
dl-waiting = Aguardando
dl-focus = Instalações iniciadas pelo Focus
dl-state-running = Instalando…
dl-state-done = Instalado
dl-state-failed = A instalação falhou
dl-open-steam = Ver na Steam
dl-open-epic = Ver na Epic Games
dl-other = As outras lojas mostram os downloads na própria janela.
help-title = Ajuda e atualização
help-version = Focus { $version }
upd-check = Procurar atualização
upd-checking = Procurando…
upd-none = O Focus está atualizado.
upd-available = O Focus { $version } está disponível
upd-install = Instalar a atualização
upd-downloading = Baixando a atualização: { $percent } %
upd-restart = O Focus vai fechar e reabrir atualizado.
upd-failed = Não foi possível atualizar. Tente de novo mais tarde.
rep-title = Relatar um problema
rep-body = Descreva o que aconteceu. Depois, seu navegador abre um formulário já preenchido: é preciso ter uma conta gratuita no GitHub para enviar. Seu nome e sua pasta pessoal são removidos do relatório.
rep-describe = Descrever o problema
rep-send = Abrir o formulário
rep-opened = O formulário está aberto no navegador.
