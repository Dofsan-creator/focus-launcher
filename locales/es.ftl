app-name = Focus Launcher
lang-title = Elige tu idioma
welcome-title = Bienvenido
welcome-body = Todo funciona con mando, ratón o teclado. Nada cambia en tu PC sin tu permiso.
welcome-autostart = Abrir al iniciar sesión
welcome-sim = Las sesiones empiezan en simulación: ves lo que cambiaría y no se toca nada.
welcome-continue = Continuar

filter-all = Todo
filter-games = Juegos
filter-software = Programas
filter-social = Social

top-add = Añadir
top-stores = Tiendas
top-settings = Ajustes
top-power = Energía

home-resume = Continuar
home-empty-title = Todavía no hay nada
home-empty-body = Los juegos instalados aparecen solos. También puedes añadir un programa.
home-played = { $hours ->
    [one] { $hours } hora de uso
   *[other] { $hours } horas de uso
}
home-running = En curso

detail-play = Jugar
detail-open = Abrir
detail-favorite = Marcar como favorito
detail-unfavorite = Quitar de favoritos
detail-remove = Quitar de la biblioteca
detail-launches = { $count ->
    [one] { $count } inicio
   *[other] { $count } inicios
}
detail-source = Origen: { $store }

prep-title = Preparando
prep-simulation = Simulación: no se cambiará nada.
prep-real = Estos cambios se desharán al terminar.
prep-nothing = No hay nada que cambiar en esta sesión.
prep-start = Empezar
prep-cancel = Cancelar
prep-throttle = Ralentizar en segundo plano: { $name }
prep-suspend = Pausar: { $name }
prep-close = Cerrar: { $name }
prep-service = Detener por ahora: { $name }
prep-awake = Mantener el PC despierto
prep-power = Plan de energía de alto rendimiento
prep-ask = Tú decides: { $name }
prep-no-guardian = Se omiten los cambios de servicios: el servicio auxiliar no está instalado.

end-title = Sesión terminada
end-time = { $minutes ->
    [one] { $minutes } minuto
   *[other] { $minutes } minutos
}
end-restored = Todo se ha restaurado.
end-problem = Algunos elementos no se pudieron restaurar. Los detalles están en el registro.
end-back = Volver a la biblioteca

panel-title = Panel rápido
panel-return = Volver a él
panel-close-entry = Cerrarlo
panel-library = Volver a la biblioteca
panel-none = No hay nada en curso

close-title = ¿Cerrar { $title }?
close-warning = Se perderá el progreso no guardado.
close-social = Una llamada o una emisión puede detenerse.
close-no = No
close-yes = Sí, cerrar
close-waiting = Esperando a que se cierre. Puede que te pida guardar.
close-force = Forzar cierre
close-force-sure = Forzar el cierre puede perder trabajo. ¿Seguro?

set-title = Ajustes
set-language = Idioma
set-lang-status = Estado de la traducción: { $status }
status-reviewed = revisada
status-draft = por revisar
set-autostart = Abrir al iniciar sesión
set-real = Optimización real
set-real-body = Mientras Focus está abierto, lo que el juego no necesita se aparta: programas ralentizados, tareas en segundo plano de Windows en pausa, potencia máxima. Todo vuelve cuando sales de Focus.
set-real-confirm = ¿Permitir cambios reales en este PC durante las sesiones?
set-keep-social = No tocar las apps sociales
set-awake = Mantener el PC despierto durante una sesión
set-high-perf = Plan de energía de alto rendimiento
set-big-picture = Bloquear el otro modo de pantalla completa de Steam
set-sounds = Sonidos
set-motion = Reducir animaciones
set-theme = Tema según la hora
set-glyphs = Iconos de botones
glyph-auto = Automático
glyph-playStation = Formas
glyph-xbox = Letras
set-detect = Detectar mando
set-guardian = Servicio auxiliar
set-guardian-body = Solo hace falta para pausar servicios de Windows. Windows pide permiso una vez, y esa ventana necesita ratón o teclado.
set-guardian-on = Instalado
set-guardian-off = No instalado
set-guardian-install = Instalar
set-analysis = Análisis del sistema
state-on = Activado
state-off = Desactivado
yes = Sí
no = No

ctl-family = Mando: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = No detectado
ctl-mode = Modo: { $mode }
mode-auto = Auto
mode-native = Nativo
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Ninguno
ctl-connected = Mando conectado: { $family }

an-title = Análisis del sistema
an-processes = Procesos: { $count }
an-services = Servicios: { $count }
an-memory = Memoria: { $gb } GB
an-cpu = Procesador: { $name }
an-top = Los que más memoria usan
an-loading = Leyendo tu sistema. No se cambia nada.

add-title = Añadir un programa
add-body = Elige un archivo. Se lee, nunca se ejecuta.
add-up = Carpeta superior
add-suggest = Sugerir desde esta carpeta
add-suggested = Archivos más probables
add-name = Nombre
add-kind = Tipo
add-save = Añadir a la biblioteca
add-script = Es un script. Revisa su ruta completa antes de añadirlo.
kind-game = Juego
kind-software = Programa
kind-social = Social
kind-other = Otro

store-title = Tiendas y aplicaciones
store-install = Instalar
store-page = Página oficial
store-confirm = ¿Instalar { $name } de { $publisher }?
store-source = Origen: gestor de paquetes de Windows
store-uac = Windows puede pedir permiso. Esa ventana necesita ratón o teclado.
store-progress = Instalando { $name }
store-done = Instalación terminada.
store-failed = La instalación falló. Puedes usar la página oficial.

power-title = Energía
power-quit = Salir al escritorio
power-restart = Reiniciar
power-shutdown = Apagar
power-sure = ¿Seguro?

kbd-done = Hecho
kbd-space = Espacio
kbd-delete = Borrar

hint-select = Elegir
hint-back = Atrás
hint-panel = Panel rápido

err-generic = Algo salió mal. Los detalles están en el registro.
autostart-disabled = El lanzador falló al iniciar dos veces, así que se desactivó la apertura al iniciar sesión.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Buscar
hint-options = Opciones
home-searching = Resultados para «{ $text }»
detail-remove-confirm = ¿Quitar { $title } de la biblioteca?
set-section-console = Modo consola
set-console = Pasar Windows al modo consola mientras Focus está abierto
set-console-body = El escritorio, la barra de tareas y las notificaciones desaparecen. Sal de Focus para recuperar tu Windows de siempre.
set-hide-taskbar = Ocultar la barra de tareas
set-silence = Silenciar las notificaciones
set-start-minimized = Iniciar minimizado al abrir la sesión
set-section-opt = Rendimiento
set-section-sound = Sonido y pantalla
set-music = Música de fondo
set-vibration = Vibración del mando
opt-off = Desactivado
opt-off-body = Simulación: ves lo que cambiaría, no se toca nada.
opt-balanced = Equilibrado
opt-balanced-body = Las aplicaciones en segundo plano se ralentizan durante una sesión y luego se restauran.
opt-max = Máximo
opt-max-body = Todas las palancas permitidas: aplicaciones ralentizadas, actualizaciones en pausa, alto rendimiento. Después todo se restaura.
power-quit-body = Windows vuelve a la normalidad.
power-restart-body = Reinicia el PC.
power-shutdown-body = Apaga el PC.
con-title = Conexiones
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Buscar de nuevo
con-searching = Buscando…
con-connected = Conectado
con-connected-to = Conectado a { $name }
con-connect = Conectar
con-disconnected = Desconectado.
con-password = Contraseña
con-none = No se encontró ninguna red.
con-ethernet = Conexión por cable
con-offline = Sin conexión
con-result-success = Conectado.
con-result-badPassword = Contraseña incorrecta.
con-result-timeout = La red no respondió.
con-result-failed = No se pudo conectar.
con-result-notFound = Red no encontrada. Vuelve a buscar.
con-result-noAdapter = No se encontró ningún adaptador Wi-Fi.
bt-paired = Tus dispositivos
bt-search = Buscar dispositivos
bt-pair = Vincular
bt-pair-hint = Pon el dispositivo en modo de vinculación y luego busca.
bt-forget-confirm = ¿Olvidar { $name }?
bt-forgotten = Dispositivo olvidado.
bt-result-success = Dispositivo vinculado.
bt-result-failed = No se pudo vincular.
setup-step = Paso { $current }
setup-next = Siguiente
setup-back = Atrás
setup-start = Empezar
setup-console-title = Tu PC se convierte en una consola
setup-console-body = Mientras Focus está abierto, Windows se aparta: sin escritorio, sin barra de tareas ni notificaciones. Sal de Focus y todo vuelve a la normalidad.
setup-pad-title = Tu mando
setup-pad-body = Elige cómo se muestran los botones. Focus también funciona con ratón y teclado.
setup-net-title = Sigue conectado
setup-net-body = El Wi-Fi y el Bluetooth están siempre a un botón, incluso durante una partida.
setup-lib-title = Tu biblioteca
setup-lib-body = { $count ->
    [one] { $count } juego o aplicación encontrado en este PC.
   *[other] { $count } juegos y aplicaciones encontrados en este PC.
}
setup-opt-title = Rendimiento durante las sesiones
setup-opt-body = Focus puede liberar tu PC mientras juegas y luego dejarlo todo como estaba. Tú eliges, y podrás cambiarlo más tarde.
setup-comfort-title = Sonido y comodidad
setup-comfort-body = Ajusta el ambiente. Abrir Focus al iniciar la sesión hace que tu PC arranque como una consola.
setup-done-title = Todo listo
setup-done-body = Tu biblioteca te espera. Pulsa Empezar para entrar.
setup-open-wifi = Configurar el Wi-Fi
setup-open-bt = Vincular un mando
store-installed = Instalado
filter-stores = Tiendas
launch-starting = Iniciando…
store-open = Abrir
panel-screenshot = Captura de pantalla
shot-saved = Captura guardada en Imágenes\Focus.
detail-update = Actualización disponible
detail-update-open = Actualizar
power-sleep = Suspender
power-sleep-body = El PC entra en suspensión, como el reposo de una consola.
end-auto = Volviendo a la biblioteca…
con-ethernet-on = Conectado por cable (Ethernet)
ctl-native-note = Fuera de Steam, Focus Input sustituye el mando PlayStation por un mando Xbox que todos los juegos reconocen. Para evitar órdenes duplicadas necesita HidHide (en la tienda de Focus, Controladores): Focus oculta el mando real al juego durante la partida y luego lo devuelve. Sin HidHide, «Auto» deja el mando tal cual.
steam-guide-on = En Steam, el botón central del mando aún abre Steam sobre tus juegos. En Steam: Parámetros, Control, Parámetros avanzados, desactiva la opción del botón Guía que pone Steam en primer plano y la configuración de combinaciones del botón Guía.
steam-guide-off = Steam deja el botón central a Focus.
steam-guide-open = Abrir los ajustes de mando de Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Preparando el PC…
profile-on = Modo consola
profile-off = Modo normal
unit-gb = GB
profile-flash = Tu PC está en modo consola
profile-shell = Escritorio de Windows oculto: vuelve cuando sales de Focus
profile-freed = { $gb } GB de memoria liberados para tus juegos
profile-power = Potencia máxima activada
profile-slowed = { $count ->
    [one] { $count } programa innecesario ralentizado
   *[other] { $count } programas innecesarios ralentizados
}
profile-closed = { $count ->
    [one] { $count } programa cerrado
   *[other] { $count } programas cerrados
}
profile-paused = { $count ->
    [one] { $count } tarea en segundo plano de Windows en pausa
   *[other] { $count } tareas en segundo plano de Windows en pausa
}
profile-title = Lo que Focus hace por ti
profile-lead = Mientras Focus está abierto, tu PC se comporta como una consola: todo lo que el juego no necesita se aparta. Todo vuelve cuando sales de Focus.
profile-lead-off = El escritorio está oculto, pero tu PC aún no está aligerado para jugar. Activa el rendimiento en los ajustes.
profile-none = Tu PC está en su estado normal. Activa el modo consola en los ajustes para que se comporte como una consola.
profile-memory = Memoria libre para tus juegos
profile-power-label = Potencia
power-plan-balanced = Equilibrada
power-plan-high = Máxima
power-plan-saver = Ahorro de energía
power-plan-other = Personalizada
apps-title = { $count ->
    [one] { $count } aplicación sigue abierta
   *[other] { $count } aplicaciones siguen abiertas
}
apps-body = Cerrarlas libera tu PC para el juego. Si alguna te pide guardar, se queda abierta: no se pierde nada.
apps-closing = Cerrando…
apps-close = Cerrarlas
apps-keep = Dejarlas
set-stop-shell = Ocultar por completo el escritorio de Windows

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = Descargas
dl-empty = No se está descargando nada ahora.
dl-steam = Juegos de Steam
dl-waiting = En espera
dl-focus = Instalaciones iniciadas desde Focus
dl-state-running = Instalando…
dl-state-done = Instalado
dl-state-failed = La instalación ha fallado
dl-open-steam = Ver en Steam
dl-open-epic = Ver en Epic Games
dl-other = Las demás tiendas muestran sus descargas en su propia ventana.
help-title = Ayuda y actualización
help-version = Focus { $version }
upd-check = Buscar una actualización
upd-checking = Buscando…
upd-none = Focus está actualizado.
upd-available = Focus { $version } está disponible
upd-install = Instalar la actualización
upd-downloading = Descargando la actualización: { $percent } %
upd-restart = Focus se cerrará y volverá a abrirse actualizado.
upd-failed = No se ha podido actualizar. Inténtalo más tarde.
rep-title = Informar de un problema
rep-body = Describe lo que ha pasado. Después tu navegador se abre en una ficha ya rellenada: necesitas una cuenta gratuita de GitHub para enviarla. Tu nombre y tu carpeta personal se eliminan del informe.
rep-describe = Describir el problema
rep-send = Abrir la ficha
rep-opened = La ficha está abierta en tu navegador.

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = Perfil de rendimiento
guide-title = El botón central de tu mando sigue abriendo Steam
guide-step-1 = Abre los ajustes de Steam con el botón de abajo y elige «Mando».
guide-step-2 = Desactiva «El botón Guía enfoca Steam».
guide-step-3 = Desactiva «Activar la configuración de combinaciones del botón Guía para mandos» y vuelve aquí.
guide-check = Hecho, comprobar
guide-later = Más tarde
guide-show = Ver cómo arreglarlo
