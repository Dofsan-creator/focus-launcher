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
