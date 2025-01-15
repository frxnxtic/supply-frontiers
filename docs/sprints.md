## Sprint 1: Инициализация репозитория и структуры проекта

- [x]  **1.1: Создать новый Cargo-проект**
    - Запустить `cargo new supply_frontier --bin` (или `-lib`)
    - Проверить структуру (Cargo.toml, src/main.rs)
    - **DoD**: Проект собирается, лежит в репо
- [x]  **1.2: Добавить .gitignore и [README.md](http://readme.md/)**
    - Создать/дополнить `.gitignore` (Rust, IDE файлы)
    - Написать краткий README (описание проекта)
    - **DoD**: Нет лишних артефактов, README виден на GitHub
- [x]  **1.3: Настроить CI (GitHub Actions)**
    - Добавить `.github/workflows/ci.yml`
    - Настроить сборку (`cargo build --release`) и тесты (`cargo test`)
    - **DoD**: Автосборка при пуше (зелёный билд)
- [x]  **1.4: Организовать базовую структуру директорий**
    - Создать папки `src/core`, `src/gui`, `src/data`
    - Создать папку `assets/` для спрайтов/иконок
    - **DoD**: Код не ломается, сборка ок
- [x]  **1.5: Проверить сборку на локальной машине**
    - Запустить `cargo build` в Debug и Release
    - **DoD**: Проект без варнингов, готов к разработке

## Sprint 2: Выбор и тестирование GUI-фреймворка

- [x]  **2.1: Исследовать фреймворки (Bevy, ggez, Macroquad)**
    - Прочитать документацию, сравнить плюсы/минусы
    - Сформировать сводку
    - **DoD**: Есть md-файл с обзором
- [x]  **2.2: Сформировать требования к движку**
    - Составить список Must Have (2D, камеры, UI)
    - Решить, нужен ли 3D, мультиплеер
    - **DoD**: Чёткий перечень критериев выбора
- [x]  **2.3: Создать тестовый проект с Bevy**
    - Написать минимальный код: окно, фон, простой спрайт
    - Проверить, что компилируется, окно видно
    - **DoD**: При `cargo run` появляется окно с квадратиком
- [x]  **2.4: Принять окончательное решение по GUI**
    - Сопоставить результаты исследований
    - Зафиксировать выбор (например, Bevy)
    - **DoD**: Документирован выбор и основные причины

## Sprint 3: Детальное проектирование ядра

- [x]  **3.1: Нарисовать UML-диаграмму сущностей**
    - Определить структуры `World`, `City`, `Factory`, `Transport`
    - Указать основные поля (название, координаты, вместимость)
    - **DoD**: Диаграмма сохранена в `docs/`
- [x]  **3.2: Решить, как хранить карту (граф vs тайлы)**
    - Анализировать плюсы/минусы графа и тайлов
    - **DoD**: Закреплён выбор, записан в документации
- [x]  **3.3: Описать план системы обновления (update)**
    - Определить последовательность обновлений (фабрики → транспорт → экономика)
    - **DoD**: Псевдокод в `docs/`
- [x]  **3.4: Спланировать взаимодействие Core <-> GUI**
    - Решить, будет ли ядро отдельным crate или интегрировано с Bevy ECS
    - **DoD**: Документирован подход
- [x]  **3.5: Итоговый документ «Архитектура ядра»**
    - Собрать все результаты в `architecture.md`
    - **DoD**: Документ добавлен в репо, понятно структурирован

## Sprint 4: Разработка базовых структур (часть 1)

- [x]  **4.1: Создать модули `city.rs`, `factory.rs`, `transport.rs`**
    - Определить структуры и базовые методы
    - **DoD**: Компилируется, модули подключены
- [x]  **4.2: Определить поля для структур**
    - `City`: название, координаты, вместимость
    - `Factory`: тип продукции, вместимость
    - `Transport`: тип транспорта, скорость, вместимость
    - **DoD**: Поля согласованы и определены
- [x]  **4.3: Написать конструкторы и базовые методы**
    - Реализовать `new` методы для структур
    - **DoD**: Методы работают корректно, тесты проходят
- [x]  **4.4: Добавить юнит-тесты**
    - Написать базовые тесты для каждой структуры
    - **DoD**: Все тесты зелёные

## Sprint 5: Система карты и сетки/графа

- [ ]  **5.1: Создать структуру `Map`**
    - Определить хранение узлов и рёбер или тайлов
    - **DoD**: Структура `Map` реализована, код компилируется
- [ ]  **5.2: Реализовать методы получения соседей и проверки проходимости**
    - `get_neighbors(node_id)`
    - `is_passable(node_id)`
    - **DoD**: Методы работают корректно, тесты проходят
- [ ]  **5.3: Подключить рельеф (опционально)**
    - Добавить высоту/сложность для узлов
    - **DoD**: Рельеф влияет на стоимость путей
- [ ]  **5.4: Тест на маленьком примере карты**
    - Создать небольшой граф и проверить методы
    - **DoD**: Методы корректно работают на тестовом графе

## Sprint 6: Базовая симуляция тика

- [ ]  **6.1: Ввести `GameTime`**
    - Создать структуру для отслеживания текущего тика
    - **DoD**: В `world.update(dt)` время обновляется
- [ ]  **6.2: Реализовать потребление городами и производство фабриками**
    - Города потребляют ресурсы, фабрики производят
    - **DoD**: Количество ресурсов изменяется корректно
- [ ]  **6.3: Движение транспорта**
    - Обновлять позиции транспорта в каждом тике
    - **DoD**: Транспорт перемещается по маршруту
- [ ]  **6.4: Тест на 10 тиков**
    - Проверить корректность изменения состояния мира
    - **DoD**: Всё работает согласно ожиданиям

## Sprint 7: Простая экономика (доходы/расходы)

- [ ]  **7.1: Ввести `player_balance`**
    - Добавить баланс игрока
    - **DoD**: Баланс отображается и обновляется
- [ ]  **7.2: Доход от поставок**
    - Увеличивать баланс при успешных поставках
    - **DoD**: Баланс увеличивается корректно
- [ ]  **7.3: Расход на содержание транспорта**
    - Уменьшать баланс за обслуживание транспорта
    - **DoD**: Баланс уменьшается корректно
- [ ]  **7.4: Тестирование баланса**
    - Проверить правильность вычислений доходов и расходов
    - **DoD**: Баланс обновляется правильно в тестах

## Sprint 8: Система событий (мини-ивенты)

- [ ]  **8.1: Создать структуру `GameEvent`**
    - Определить типы событий и их параметры
    - **DoD**: Структура `GameEvent` реализована
- [ ]  **8.2: Генерация случайных событий**
    - Реализовать генерацию аварий, поломок, задержек
    - **DoD**: События генерируются с заданной вероятностью
- [ ]  **8.3: Реакция на события**
    - Обновлять состояние транспорта при событиях
    - **DoD**: Транспорт реагирует на события корректно
- [ ]  **8.4: Тестирование системы событий**
    - Проверить корректность генерации и реакции на события
    - **DoD**: Система событий работает без ошибок

## Sprint 9: Интеграция Bevy и главное окно

- [ ]  **9.1: Подключить Bevy в проект**
    - Добавить зависимость Bevy в `Cargo.toml`
    - **DoD**: Проект собирается с Bevy
- [ ]  **9.2: Создать стартовое окно и сцену**
    - Реализовать начальную сцену с UI
    - **DoD**: Окно запускается и отображает сцену
- [ ]  **9.3: Привязать `world.update(dt)` к игровому циклу Bevy**
    - Вызов функции обновления мира в каждом кадре
    - **DoD**: Обновление мира синхронизировано с игровым циклом
- [ ]  **9.4: Тестирование интеграции**
    - Проверить работу обновлений и отображение в Bevy
    - **DoD**: Всё работает корректно, без сбоев

## Sprint 10: Простейшая визуализация карты

- [ ]  **10.1: Нарисовать 2D-тайлы или узлы**
    - Реализовать отрисовку карты в 2D
    - **DoD**: Карта отображается на экране
- [ ]  **10.2: Добавить камеру (WASD, колесо мыши для зума)**
    - Реализовать управление камерой
    - **DoD**: Камера может двигаться и масштабироваться
- [ ]  **10.3: Минимальный UI для координат отладки**
    - Отображать координаты курсора или текущего узла
    - **DoD**: Координаты отображаются корректно
- [ ]  **10.4: Тест на маленькой карте**
    - Проверить отображение и навигацию по небольшой карте
    - **DoD**: Карта корректно отображается, нет вылетов

## Sprint 11: Отрисовка городов, фабрик, транспорта

- [ ]  **11.1: Добавить спрайты для `City` и `Factory`**
    - Реализовать визуальное представление городов и фабрик
    - **DoD**: Объекты отображаются на карте
- [ ]  **11.2: Добавить иконки транспорта**
    - Реализовать спрайты для различных типов транспорта
    - **DoD**: Транспорт отображается и различается по типу
- [ ]  **11.3: Обновление позиций транспорта каждый кадр**
    - Реализовать плавное перемещение транспорта
    - **DoD**: Транспорт перемещается по маршруту без лагов
- [ ]  **11.4: Простой тест движения**
    - Проверить движение одного транспортного средства
    - **DoD**: Транспорт корректно перемещается туда-сюда

## Sprint 12: GUI-элементы интерфейса

- [ ]  **12.1: Создать HUD с балансом**
    - Отображать текущий баланс игрока на экране
    - **DoD**: Баланс виден и обновляется в реальном времени
- [ ]  **12.2: Добавить панель инструментов**
    - Реализовать кнопки "Построить дорогу", "Купить транспорт"
    - **DoD**: Кнопки отображаются и реагируют на клики
- [ ]  **12.3: Реализовать меню паузы (ESC)**
    - Позволить игроку ставить игру на паузу
    - **DoD**: При нажатии ESC игра останавливается, отображается меню
- [ ]  **12.4: Базовая стилизация UI**
    - Добавить фоны, цвета и стилизацию элементов интерфейса
    - **DoD**: UI выглядит аккуратно и профессионально

## Sprint 13: Управление транспортом (маршруты, приказы)

- [ ]  **13.1: Реализовать режим выбора транспорта (клик)**
    - Позволить игроку выбирать транспортное средство по клику
    - **DoD**: Выбранное транспортное средство выделяется
- [ ]  **13.2: Добавить возможность назначать маршрут**
    - Реализовать функционал задания маршрута от точки A до точки B
    - **DoD**: Транспорт движется по заданному маршруту
- [ ]  **13.3: Добавить маркер маршрута на карте**
    - Визуально отображать маршрут транспортного средства
    - **DoD**: Игрок видит путь, по которому движется транспорт
- [ ]  **13.4: Проверка завершения маршрута**
    - Реализовать логику окончания маршрута и возврата в состояние ожидания
    - **DoD**: Транспорт корректно завершает маршрут и готов к новым приказам

## Sprint 14: Строительство дорог/объектов

- [ ]  **14.1: Реализовать кнопку "Построить дорогу"**
    - Добавить кнопку для начала строительства дороги
    - **DoD**: Кнопка доступна и реагирует на нажатие
- [ ]  **14.2: Реализовать списывание денег и учёт времени постройки**
    - Списывать средства при начале строительства и учитывать время выполнения
    - **DoD**: Деньги списываются, время строительства отслеживается
- [ ]  **14.3: Отрисовка строящейся дороги**
    - Визуально отображать процесс строительства дороги
    - **DoD**: Дорога постепенно строится и становится доступной
- [ ]  **14.4: Тестирование постройки дороги**
    - Проверить корректность процесса строительства и отображения дороги
    - **DoD**: Дорога строится правильно, баланс обновляется

## Sprint 15: Расширенная экономика (спрос/предложение)

- [ ]  **15.1: Ввести динамические цены в городах**
    - Реализовать изменение цен в зависимости от спроса и предложения
    - **DoD**: Цены корректно меняются в зависимости от рыночных условий
- [ ]  **15.2: Ввести лимиты производства на фабриках**
    - Ограничить объём производства фабрик в зависимости от ресурсов
    - **DoD**: Фабрики не могут производить больше установленных лимитов
- [ ]  **15.3: Реализовать падение цены при перенасыщении города**
    - Уменьшать цену товара, если спрос в городе превышает предложение
    - **DoD**: Цена падает при перенасыщении и восстанавливается при сбалансировании
- [ ]  **15.4: Тестирование экономической системы**
    - Провести тестовые прогонки для проверки корректности работы экономики
    - **DoD**: Экономика работает стабильно и предсказуемо

## Sprint 16: Сохранение/загрузка игры

- [ ]  **16.1: Реализовать функции сохранения и загрузки**
    - Создать функции для сериализации и десериализации состояния игры
    - **DoD**: Возможность сохранить и загрузить игру корректно
- [ ]  **16.2: Добавить интерфейс для сохранения/загрузки**
    - Реализовать кнопки "Сохранить" и "Загрузить" в интерфейсе
    - **DoD**: Игрок может сохранять и загружать игру через UI
- [ ]  **16.3: Проверить целостность данных после загрузки**
    - Убедиться, что состояние игры сохраняется и восстанавливается без ошибок
    - **DoD**: После загрузки игра работает корректно, все данные на месте
- [ ]  **16.4: Тестирование системы сохранения/загрузки**
    - Провести тесты на сохранение и загрузку в различных сценариях
    - **DoD**: Система работает без сбоев во всех тестах

## Sprint 17: Поиск пути (Dijkstra/A*)

- [ ]  **17.1: Реализовать алгоритм Dijkstra**
    - Создать модуль для поиска кратчайшего пути с использованием Dijkstra
    - **DoD**: Алгоритм корректно находит кратчайшие пути в тестовых графах
- [ ]  **17.2: Реализовать алгоритм A**
    - Создать модуль для поиска пути с использованием A*
    - **DoD**: Алгоритм корректно находит оптимальные пути в тестовых условиях
- [ ]  **17.3: Интегрировать алгоритмы с транспортом**
    - Связать выбор маршрута транспорта с результатами алгоритмов поиска пути
    - **DoD**: Транспорт использует найденные маршруты для движения
- [ ]  **17.4: Тестирование алгоритмов поиска пути**
    - Провести тесты на различных картах и сценариях
    - **DoD**: Алгоритмы работают корректно во всех тестах

## Sprint 18: Автоматическое распределение грузов (упрощённый VRP)

- [ ]  **18.1: Создать `LogisticsManager`**
    - Реализовать менеджер логистики для автоматического распределения грузов
    - **DoD**: `LogisticsManager` функционирует и интегрирован с миром игры
- [ ]  **18.2: Реализовать эвристику для распределения**
    - Создать простую эвристику для распределения грузов между городами и транспортом
    - **DoD**: Эвристика корректно распределяет грузы в тестовых условиях
- [ ]  **18.3: Интегрировать распределение с транспортом**
    - Связать `LogisticsManager` с транспортными средствами для назначения маршрутов
    - **DoD**: Транспорт получает автоматические маршруты от менеджера
- [ ]  **18.4: Тестирование системы автоматической логистики**
    - Провести тесты на распределение грузов в различных сценариях
    - **DoD**: Логистика работает корректно и эффективно в тестах

## Sprint 19: Генерация мира (увеличенная карта)

- [ ]  **19.1: Реализовать генератор ландшафта**
    - Создать процедурную генерацию ландшафта (горы, реки и т.д.)
    - **DoD**: Ландшафт генерируется корректно и разнообразно
- [ ]  **19.2: Расставить города и фабрики**
    - Автоматически размещать города и фабрики на карте
    - **DoD**: Города и фабрики размещаются равномерно и логично
- [ ]  **19.3: Добавить начальные дороги (опционально)**
    - Реализовать автоматическое соединение ближайших городов
    - **DoD**: Начальные дороги строятся корректно или игрок может их строить вручную
- [ ]  **19.4: Тестирование генерации мира**
    - Провести тесты на различных картах для проверки корректности генерации
    - **DoD**: Генерация мира работает стабильно и без ошибок

## Sprint 20: Балансировка экономики

- [ ]  **20.1: Провести тестовые прогонки**
    - Провести серию тестов для проверки баланса экономики
    - **DoD**: Есть собранные данные о тестах
- [ ]  **20.2: Подкрутить цены и затраты**
    - Настроить цены на товары и затраты на транспорт
    - **DoD**: Экономика выглядит сбалансированной и реалистичной
- [ ]  **20.3: Реализовать инструмент для "live" настройки**
    - Создать интерфейс для динамической настройки параметров экономики
    - **DoD**: Игрок может изменять параметры экономики в реальном времени
- [ ]  **20.4: Документировать подход к балансировке**
    - Описать в документации методы и принципы балансировки
    - **DoD**: Документация доступна и понятна

## Sprint 21: Система технологий и апгрейдов

- [ ]  **21.1: Создать технологическое дерево (Tech Tree)**
    - Реализовать структуру для управления технологиями и апгрейдами
    - **DoD**: Tech Tree функционирует и интегрирован с игрой
- [ ]  **21.2: Ввести очки исследований**
    - Добавить систему накопления очков для исследований
    - **DoD**: Очки начисляются и могут быть потрачены на апгрейды
- [ ]  **21.3: Реализовать эффекты апгрейдов**
    - Ввести бонусы от апгрейдов (ускорение транспорта, увеличение производства)
    - **DoD**: Апгрейды дают ожидаемые бонусы
- [ ]  **21.4: Добавить UI для управления технологиями**
    - Создать интерфейс для просмотра и покупки технологий
    - **DoD**: Игрок может видеть и приобретать доступные технологии

## Sprint 22: Система контрактов и квестов

- [ ]  **22.1: Создать структуру `Contract`**
    - Определить структуру контрактов (цель, срок, награда)
    - **DoD**: Структура `Contract` реализована
- [ ]  **22.2: Реализовать генерацию контрактов**
    - Создать систему случайного генерирования контрактов
    - **DoD**: Контракты генерируются с заданной частотой и разнообразием
- [ ]  **22.3: Реализовать проверку выполнения контрактов**
    - Проверять выполнение условий контрактов и начислять награды или штрафы
    - **DoD**: Контракты завершаются корректно
- [ ]  **22.4: Добавить UI для отображения контрактов**
    - Создать интерфейс для просмотра активных контрактов
    - **DoD**: Игрок видит и может взаимодействовать с контрактами

## Sprint 23: AI-конкуренты (базовый)

- [ ]  **23.1: Создать `AIManager`**
    - Реализовать менеджер для управления действиями AI
    - **DoD**: `AIManager` функционирует и интегрирован с игрой
- [ ]  **23.2: Реализовать логику строительства дорог AI**
    - AI строит дороги в выгодных направлениях
    - **DoD**: AI строит дороги на основе своей логики
- [ ]  **23.3: Реализовать закупку транспорта AI**
    - AI покупает транспортные средства для расширения сети
    - **DoD**: AI приобретает транспорт по мере необходимости
- [ ]  **23.4: Тестирование поведения AI**
    - Проверить корректность действий AI в различных сценариях
    - **DoD**: AI действует согласно заданной логике без ошибок

## Sprint 24: Экран победы/проигрыша

- [ ]  **24.1: Реализовать условие банкротства**
    - Определить критерии для проигрыша (банкротство)
    - **DoD**: При банкротстве появляется экран "Game Over"
- [ ]  **24.2: Реализовать условие победы**
    - Определить критерии для победы (достижение цели)
    - **DoD**: При достижении цели появляется экран "Победа"
- [ ]  **24.3: Создать итоговый экран со статистикой**
    - Отображать статистику игры после победы/поражения
    - **DoD**: Игрок видит подробную статистику после завершения игры
- [ ]  **24.4: Добавить кнопку "Начать заново"**
    - Позволить игроку начать игру с начала
    - **DoD**: Кнопка работает корректно и перезапускает игру

## Sprint 25: Система сценариев (кампания)

- [ ]  **25.1: Создать структуру `Scenario`**
    - Определить структуру для сценариев кампании (правила, цели)
    - **DoD**: Структура `Scenario` реализована
- [ ]  **25.2: Реализовать загрузку сценариев**
    - Загрузка сценариев из файлов (JSON/YAML)
    - **DoD**: Сценарии загружаются корректно
- [ ]  **25.3: Реализовать выбор сценария в меню**
    - Добавить интерфейс для выбора сценария перед началом игры
    - **DoD**: Игрок может выбрать сценарий из списка
- [ ]  **25.4: Тестирование сценариев кампании**
    - Проверить корректность работы различных сценариев
    - **DoD**: Сценарии работают как задумано без ошибок

## Sprint 26: Крупные сюжетные события (в кампании)

- [ ]  **26.1: Реализовать систему скриптовых ивентов**
    - Создать систему для запуска крупных событий (ураганы, землетрясения)
    - **DoD**: Ивенты запускаются по заданным условиям
- [ ]  **26.2: Реализовать эффекты ивентов на карту**
    - Внедрить изменения в карту в результате ивентов (разрушение дорог)
    - **DoD**: Карта изменяется корректно в ответ на ивенты
- [ ]  **26.3: Добавить уведомления для игрока**
    - Уведомлять игрока о наступлении ивентов
    - **DoD**: Игрок получает уведомления при наступлении ивентов
- [ ]  **26.4: Тестирование ивентов в сценариях**
    - Проверить корректность запуска ивентов и их эффектов
    - **DoD**: Ивенты работают как задумано в тестовых сценариях

## Sprint 27: Балансировка и полировка сценариев

- [ ]  **27.1: Провести игровое тестирование сценариев**
    - Пройтись по всем сценариям и собрать обратную связь
    - **DoD**: Собрана обратная связь и результаты тестов
- [ ]  **27.2: Подкрутить параметры сценариев**
    - Настроить сложности, сроки и цели для улучшения баланса
    - **DoD**: Параметры сценариев обновлены и задокументированы
- [ ]  **27.3: Ввести систему оценок (Rank S, A, B)**
    - Оценивать выполнение сценариев по рейтингу
    - **DoD**: Рейтинг отображается на итоговом экране
- [ ]  **27.4: Внедрить обратную связь от тестеров**
    - Учесть комментарии и внести необходимые изменения
    - **DoD**: Внесены коррективы на основе обратной связи

## Sprint 28: Система достижений (Achievements)

- [ ]  **28.1: Реализовать `AchievementManager`**
    - Создать менеджер для отслеживания достижений
    - **DoD**: `AchievementManager` функционирует корректно
- [ ]  **28.2: Определить условия для достижений**
    - Задать условия, такие как "Перевёз 10k тонн"
    - **DoD**: Условия достижений реализованы
- [ ]  **28.3: Реализовать всплывающие уведомления**
    - Отображать уведомления при достижении ачивок
    - **DoD**: Уведомления появляются корректно
- [ ]  **28.4: Добавить список достижений в UI**
    - Создать интерфейс для просмотра всех достижений
    - **DoD**: Игрок видит список ачивок и их статусы

## Sprint 29: Визуальные улучшения (анимации, эффекты)

- [ ]  **29.1: Реализовать анимацию транспорта**
    - Добавить анимации движения транспорта
    - **DoD**: Транспорт выглядит живо и анимированно
- [ ]  **29.2: Добавить эффекты погоды**
    - Реализовать визуальные эффекты дождя, снега и т.д.
    - **DoD**: Эффекты погоды отображаются корректно
- [ ]  **29.3: Реализовать плавные переходы**
    - Добавить плавные переходы между сценами и состояниями
    - **DoD**: Переходы выглядят плавно и профессионально
- [ ]  **29.4: Проверить FPS на больших картах**
    - Оптимизировать рендеринг для больших карт
    - **DoD**: FPS остаётся приемлемым на больших картах

## Sprint 30: Звук и музыка

- [ ]  **30.1: Подключить аудиоплагин (Bevy Audio)**
    - Добавить поддержку аудио в проект
    - **DoD**: Аудиоплагин работает корректно
- [ ]  **30.2: Добавить фоновую музыку**
    - Внедрить фоновую музыку, которая играет в игре
    - **DoD**: Музыка играет плавно и не прерывается
- [ ]  **30.3: Добавить звуки кликов и уведомлений**
    - Реализовать звуковые эффекты для интерфейса и ивентов
    - **DoD**: Звуки воспроизводятся корректно при действиях
- [ ]  **30.4: Реализовать настройки громкости**
    - Добавить возможность регулировать громкость музыки и звуков
    - **DoD**: Игрок может изменять громкость через настройки

## Sprint 31: Улучшение UI/UX

- [ ]  **31.1: Создать окно настроек**
    - Добавить интерфейс для изменения настроек графики, звука и управления
    - **DoD**: Игрок может изменять настройки через UI
- [ ]  **31.2: Добавить подсказки при наведении**
    - Реализовать всплывающие подсказки для кнопок и элементов интерфейса
    - **DoD**: Подсказки появляются корректно при наведении
- [ ]  **31.3: Реализовать Drag & Drop для панелей**
    - Позволить игроку перетаскивать панели интерфейса по экрану
    - **DoD**: Панели можно свободно перемещать и фиксировать позиции
- [ ]  **31.4: Провести тестирование UX**
    - Проверить удобство использования интерфейса и внести улучшения
    - **DoD**: Интерфейс удобен и интуитивно понятен

## Sprint 32: Оптимизация производительности

- [ ]  **32.1: Провести профилирование проекта**
    - Использовать инструменты профилирования (например, `cargo profiler`)
    - **DoD**: Определены узкие места производительности
- [ ]  **32.2: Распараллелить тяжёлые вычисления**
    - Внедрить многопоточность для ресурсоёмких процессов
    - **DoD**: Производительность улучшена, нет лагов
- [ ]  **32.3: Разделить обновление карты на зоны**
    - Оптимизировать обновление карты, разделив её на регионы
    - **DoD**: Обновление карты происходит быстрее и эффективнее
- [ ]  **32.4: Провести тестирование на больших картах**
    - Проверить стабильность и производительность на масштабных сценариях
    - **DoD**: Игра стабильно работает на больших картах

## Sprint 33: Мультиплатформенные сборки

- [ ]  **33.1: Настроить GitHub Actions для мультиплатформенной сборки**
    - Добавить CI-конфигурации для Windows, Linux и macOS
    - **DoD**: Сборки проходят на всех платформах
- [ ]  **33.2: Проверить сборки на реальных системах**
    - Запустить и протестировать сборки на разных ОС
    - **DoD**: Сборки работают корректно на Windows, Linux и macOS
- [ ]  **33.3: Убедиться в корректности путей к ресурсам**
    - Проверить относительные пути к файлам и ресурсам
    - **DoD**: Путь к ресурсам корректен на всех платформах
- [ ]  **33.4: Создать итоговые сборки и артефакты**
    - Подготовить финальные билд-артефакты для релиза
    - **DoD**: Финальные сборки доступны для загрузки и тестирования

## Sprint 34: Полировка интерфейса и текста

- [ ]  **34.1: Привести все надписи к единому стилю**
    - Стандартизировать шрифты, цвета и оформление текстовых элементов
    - **DoD**: Интерфейс выглядит единообразно и профессионально
- [ ]  **34.2: Проверить орфографию и грамматику**
    - Вылечить все ошибки в тексте и подсказках
    - **DoD**: Нет опечаток и грамматических ошибок
- [ ]  **34.3: Реализовать локализацию (EN, RU и др.)**
    - Добавить поддержку нескольких языков
    - **DoD**: Игрок может переключать язык интерфейса
- [ ]  **34.4: Проверить отображение шрифтов**
    - Убедиться, что шрифты корректно отображаются на всех языках
    - **DoD**: Текст читаем и отображается без искажений

## Sprint 35: Демоверсия и маркетинг

- [ ]  **35.1: Ограничить функционал для демо**
    - Сделать ограниченную карту или один сценарий для демо-версии
    - **DoD**: Демо-сборка доступна и работает корректно
- [ ]  **35.2: Создать скриншоты и трейлер**
    - Сделать качественные скриншоты и записать короткий трейлер игры
    - **DoD**: Promo материалы готовы для публикации
- [ ]  **35.3: Подготовить страницу проекта ([itch.io/Steam](http://itch.io/Steam))**
    - Оформить страницу игры с описанием, скриншотами и трейлером
    - **DoD**: Страница доступна для просмотра и загрузки демо
- [ ]  **35.4: Собрать ранний фидбэк**
    - Запустить демо, собрать отзывы от первых игроков
    - **DoD**: Получены отзывы и предложения по улучшению

## Sprint 36: Релиз и пост-релизная поддержка

- [ ]  **36.1: Выпустить версию 1.0**
    - Подготовить финальные сборки и объявить релиз
    - **DoD**: Версия 1.0 доступна для загрузки
- [ ]  **36.2: Организовать каналы общения (Discord/Forum)**
    - Создать сервер Discord или форум для общения с игроками
    - **DoD**: Каналы связи доступны и функционируют
- [ ]  **36.3: Планировать DLC и патчи**
    - Разработать план дальнейшего развития игры через DLC и обновления
    - **DoD**: Идеи и планы задокументированы
- [ ]  **36.4: Собирать и анализировать статистику/аналитику (опционально)**
    - Внедрить системы сбора данных о поведении игроков (с согласия)
    - **DoD**: Данные собираются и анализируются для улучшения игры