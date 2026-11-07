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
set-real-body = Desactivada: solo simulación. Activada: las apps en segundo plano se ralentizan durante una sesión y luego se restauran.
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
