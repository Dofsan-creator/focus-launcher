app-name = Focus Launcher
lang-title = Выберите язык
welcome-title = Добро пожаловать
welcome-body = Здесь всё работает с геймпадом, мышью или клавиатурой. Без вашего согласия на ПК ничего не меняется.
welcome-autostart = Открывать при входе в систему
welcome-sim = Сеансы начинаются в режиме симуляции: вы видите, что изменилось бы, и ничего не трогается.
welcome-continue = Продолжить

filter-all = Все
filter-games = Игры
filter-software = Программы
filter-social = Общение

top-add = Добавить
top-stores = Магазины
top-settings = Настройки
top-power = Питание

home-resume = Продолжить
home-empty-title = Здесь пока пусто
home-empty-body = Установленные игры появятся сами. Можно также добавить программу.
home-played = { $hours ->
    [one] { $hours } час использования
    [few] { $hours } часа использования
   *[other] { $hours } часов использования
}
home-running = Запущено

detail-play = Играть
detail-open = Открыть
detail-favorite = В избранное
detail-unfavorite = Убрать из избранного
detail-remove = Убрать из библиотеки
detail-launches = { $count ->
    [one] { $count } запуск
    [few] { $count } запуска
   *[other] { $count } запусков
}
detail-source = Источник: { $store }

prep-title = Подготовка
prep-simulation = Симуляция: ничего не будет изменено.
prep-real = Эти изменения будут отменены, когда вы закончите.
prep-nothing = Для этого сеанса менять нечего.
prep-start = Запустить
prep-cancel = Отмена
prep-throttle = Замедлить в фоне: { $name }
prep-suspend = Приостановить: { $name }
prep-close = Закрыть: { $name }
prep-service = Временно остановить: { $name }
prep-awake = Не давать ПК засыпать
prep-power = Схема питания «Высокая производительность»
prep-ask = Решать вам: { $name }
prep-no-guardian = Изменения служб пропущены: вспомогательная служба не установлена.

end-title = Сеанс завершён
end-time = { $minutes ->
    [one] { $minutes } минута
    [few] { $minutes } минуты
   *[other] { $minutes } минут
}
end-restored = Всё восстановлено.
end-problem = Некоторые элементы восстановить не удалось. Подробности в журнале.
end-back = Назад в библиотеку

panel-title = Быстрая панель
panel-return = Вернуться
panel-close-entry = Закрыть
panel-library = Назад в библиотеку
panel-none = Ничего не запущено

close-title = Закрыть { $title }?
close-warning = Несохранённый прогресс будет потерян.
close-social = Звонок или трансляция могут прерваться.
close-no = Нет
close-yes = Да, закрыть
close-waiting = Ожидание закрытия. Возможно, программа просит сохранить данные.
close-force = Закрыть принудительно
close-force-sure = Принудительное закрытие может привести к потере работы. Вы уверены?

set-title = Настройки
set-language = Язык
set-lang-status = Состояние перевода: { $status }
status-reviewed = проверен
status-draft = требует проверки
set-autostart = Открывать при входе в систему
set-real = Реальная оптимизация
set-real-body = Выкл.: только симуляция. Вкл.: фоновые приложения замедляются на время сеанса, затем восстанавливаются.
set-real-confirm = Разрешить реальные изменения на этом ПК во время сеансов?
set-keep-social = Не трогать приложения для общения
set-awake = Не давать ПК засыпать во время сеанса
set-high-perf = Схема питания «Высокая производительность»
set-big-picture = Блокировать другой полноэкранный режим Steam
set-sounds = Звуки
set-motion = Уменьшить анимацию
set-theme = Тема по времени суток
set-glyphs = Значки кнопок
glyph-auto = Автоматически
glyph-playStation = Фигуры
glyph-xbox = Буквы
set-detect = Определить геймпад
set-guardian = Вспомогательная служба
set-guardian-body = Нужна только для приостановки служб Windows. Windows один раз запросит разрешение, и это окно требует мыши или клавиатуры.
set-guardian-on = Установлена
set-guardian-off = Не установлена
set-guardian-install = Установить
set-analysis = Анализ системы
state-on = Вкл.
state-off = Выкл.
yes = Да
no = Нет

ctl-family = Геймпад: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Не найден
ctl-mode = Режим: { $mode }
mode-auto = Авто
mode-native = Встроенный
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Нет
ctl-connected = Геймпад подключён: { $family }

an-title = Анализ системы
an-processes = Процессы: { $count }
an-services = Службы: { $count }
an-memory = Память: { $gb } ГБ
an-cpu = Процессор: { $name }
an-top = Больше всего памяти
an-loading = Читаем вашу систему. Ничего не меняется.

add-title = Добавить программу
add-body = Выберите файл. Он только читается, не запускается.
add-up = На уровень выше
add-suggest = Предложить из этой папки
add-suggested = Наиболее вероятные файлы
add-name = Название
add-kind = Тип
add-save = Добавить в библиотеку
add-script = Это скрипт. Проверьте полный путь перед добавлением.
kind-game = Игра
kind-software = Программа
kind-social = Общение
kind-other = Другое

store-title = Магазины и приложения
store-install = Установить
store-page = Официальная страница
store-confirm = Установить { $name } от { $publisher }?
store-source = Источник: диспетчер пакетов Windows
store-uac = Windows может запросить разрешение. Это окно требует мыши или клавиатуры.
store-progress = Установка { $name }
store-done = Установка завершена.
store-failed = Установка не удалась. Можно воспользоваться официальной страницей.

power-title = Питание
power-quit = Выйти на рабочий стол
power-restart = Перезагрузить
power-shutdown = Выключить
power-sure = Вы уверены?

kbd-done = Готово
kbd-space = Пробел
kbd-delete = Стереть

hint-select = Выбрать
hint-back = Назад
hint-panel = Быстрая панель

err-generic = Что-то пошло не так. Подробности в журнале.
autostart-disabled = Лаунчер дважды не смог запуститься, поэтому открытие при входе в систему отключено.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Поиск
hint-options = Параметры
home-searching = Результаты по запросу «{ $text }»
detail-remove-confirm = Убрать { $title } из библиотеки?
set-section-console = Режим консоли
set-console = Переводить Windows в режим консоли, пока открыт Focus
set-console-body = Панель задач скрывается, уведомления замолкают. Закройте Focus, чтобы вернуть привычный Windows.
set-hide-taskbar = Скрывать панель задач
set-silence = Отключать уведомления
set-start-minimized = Запускать свёрнутым при входе в систему
set-section-opt = Производительность
set-section-sound = Звук и экран
set-music = Фоновая музыка
set-vibration = Вибрация геймпада
opt-off = Выключено
opt-off-body = Симуляция: видно, что изменилось бы, ничего не трогается.
opt-balanced = Сбалансировано
opt-balanced-body = Фоновые приложения замедляются на время сеанса, затем всё восстанавливается.
opt-max = Максимум
opt-max-body = Все разрешённые меры: приложения замедлены, обновления на паузе, высокая производительность. Затем всё восстанавливается.
power-quit-body = Windows возвращается в обычный режим.
power-restart-body = Перезагружает компьютер.
power-shutdown-body = Выключает компьютер.
con-title = Подключения
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Искать снова
con-searching = Поиск…
con-connected = Подключено
con-connected-to = Подключено к { $name }
con-connect = Подключиться
con-disconnected = Отключено.
con-password = Пароль
con-none = Сети не найдены.
con-ethernet = Проводное подключение
con-offline = Нет подключения
con-result-success = Подключено.
con-result-badPassword = Неверный пароль.
con-result-timeout = Сеть не ответила.
con-result-failed = Не удалось подключиться.
con-result-notFound = Сеть не найдена. Повторите поиск.
con-result-noAdapter = Адаптер Wi-Fi не найден.
bt-paired = Ваши устройства
bt-search = Найти устройства
bt-pair = Связать
bt-pair-hint = Переведите устройство в режим сопряжения и запустите поиск.
bt-forget-confirm = Забыть { $name }?
bt-forgotten = Устройство забыто.
bt-result-success = Устройство связано.
bt-result-failed = Не удалось связать устройство.
setup-step = Шаг { $current }
setup-next = Далее
setup-back = Назад
setup-start = Начать
setup-console-title = Ваш ПК становится консолью
setup-console-body = Пока Focus открыт, Windows уходит в тень: ни панели задач, ни уведомлений. Закройте Focus, и всё вернётся как было.
setup-pad-title = Ваш геймпад
setup-pad-body = Выберите, как показывать кнопки. Focus работает и с мышью, и с клавиатурой.
setup-net-title = Оставайтесь на связи
setup-net-body = Wi-Fi и Bluetooth всегда в одном нажатии, даже во время игры.
setup-lib-title = Ваша библиотека
setup-lib-body = { $count ->
    [one] Найдена { $count } игра или программа на этом ПК.
    [few] Найдено { $count } игры и программы на этом ПК.
    [many] Найдено { $count } игр и программ на этом ПК.
   *[other] Найдено игр и программ на этом ПК: { $count }.
}
setup-opt-title = Производительность во время сеансов
setup-opt-body = Focus может разгрузить ПК, пока вы играете, а затем всё вернуть. Выбор за вами, его можно изменить позже.
setup-comfort-title = Звук и удобство
setup-comfort-body = Настройте атмосферу. Если Focus открывается при входе в систему, ПК запускается как консоль.
setup-done-title = Всё готово
setup-done-body = Библиотека ждёт. Нажмите «Начать», чтобы войти.
setup-open-wifi = Настроить Wi-Fi
setup-open-bt = Подключить геймпад
store-installed = Установлено
filter-stores = Магазины
launch-starting = Запуск…
store-open = Открыть
panel-screenshot = Снимок экрана
shot-saved = Снимок сохранён в «Изображения\Focus».
detail-update = Доступно обновление
detail-update-open = Обновить
power-sleep = Спящий режим
power-sleep-body = ПК переходит в сон, как консоль в режиме покоя.
end-auto = Возврат в библиотеку…
con-ethernet-on = Подключено по кабелю (Ethernet)
ctl-native-note = Вне Steam Focus Input заменяет геймпад PlayStation геймпадом Xbox, который понимают все игры. Чтобы не было двойных нажатий, нужен HidHide (в магазине Focus, раздел драйверов): Focus скрывает настоящий геймпад от игры на время партии и потом возвращает. Без HidHide режим «Авто» оставляет геймпад как есть.
steam-guide-on = В Steam центральная кнопка геймпада всё ещё выводит Steam поверх игр. В Steam: Настройки, Контроллер, Расширенные настройки — отключите вывод Steam на передний план кнопкой Guide и настройку сочетаний кнопки Guide.
steam-guide-off = Steam оставляет центральную кнопку для Focus.
steam-guide-open = Открыть настройки контроллера Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Подготовка ПК…
profile-on = Режим консоли
profile-off = Обычный режим
unit-gb = ГБ
profile-flash = Ваш ПК в режиме консоли
profile-shell = Рабочий стол Windows скрыт: он вернётся, когда вы закроете Focus
profile-freed = { $gb } ГБ памяти освобождено для игр
profile-power = Включена максимальная мощность
profile-slowed = { $count ->
    [one] Замедлена { $count } ненужная программа
    [few] Замедлены { $count } ненужные программы
   *[other] Замедлено { $count } ненужных программ
}
profile-closed = { $count ->
    [one] Закрыта { $count } программа
    [few] Закрыты { $count } программы
   *[other] Закрыто { $count } программ
}
profile-paused = { $count ->
    [one] Приостановлена { $count } фоновая задача Windows
    [few] Приостановлены { $count } фоновые задачи Windows
   *[other] Приостановлено { $count } фоновых задач Windows
}
profile-title = Что Focus делает для вас
profile-lead = Пока Focus открыт, ваш ПК ведёт себя как консоль: всё, что не нужно игре, отложено в сторону. Всё вернётся, когда вы закроете Focus.
profile-lead-off = Рабочий стол скрыт, но ПК ещё не разгружен для игр. Включите производительность в настройках.
profile-none = Ваш ПК в обычном состоянии. Включите режим консоли в настройках, чтобы он вёл себя как консоль.
profile-memory = Свободная память для игр
profile-power-label = Мощность
power-plan-balanced = Сбалансированная
power-plan-high = Максимальная
power-plan-saver = Экономия энергии
power-plan-other = Своя
apps-title = { $count ->
    [one] Ещё открыто { $count } приложение
    [few] Ещё открыты { $count } приложения
   *[other] Ещё открыто { $count } приложений
}
apps-body = Закрыв их, вы освободите ПК для игры. Если какое-то попросит сохранить данные, оно останется открытым: ничего не потеряется.
apps-closing = Закрываем…
apps-close = Закрыть
apps-keep = Оставить
set-stop-shell = Полностью скрывать рабочий стол Windows
