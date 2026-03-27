# Blaze Checker / DETI00YKH CHECKER - слив сурсов

Короче вот полные сурсы Blaze Checker, он же DETI00YKH CHECKER, тот самый "чекер" с сайта cs2checker.ru за 300р/мес.

Реверснул оба бинарника, вытащил весь Rust бэкенд + фронтенд (Svelte), собрал обратно, всё компилится. (По дизайну могут быть не точности, ибо там идёт загрузка кастм дизайна по Api в виде customization.dat)

## Кто автор

cr1stal12444, зовут Данил. Использует GitHub Pro что имеется у его отца xD, ну и там уже все эти Claude хуяудэ, через неё и пишет весь код. Сам программировать не умеет вообще, просто кидает промты и ошибки в нейронку пока не заработает, топ xd

Откуда это известно:
- В бинарнике остались пути `C:\Users\daniel\.cargo\registry\...` — не знает про strip.
- Ключ шифрования `CS2_CHECKER_SECURE_KEY_2024_V1` прям текстом в коде.
- "Антиреверс" это `IsDebuggerPresent()` и массив фейковых урлов типа `https://fake-api.example.com/auth`.
- Один API ключ на всех юзеров, без разделения.
- База читов это 14 MD5 хешей в открытом JSON на GitHub.

## За что берут 300р/мес

| Что обещают | Что по факту |
|---|---|
| Сканирование памяти CS2 | ReadProcessMemory + поиск строк "aimbot", "wallhack" в тексте |
| Анализ файлов | Ищет папки с именами nixware, onetap, skeet |
| Проверка браузера | SQLite запрос + 19 доменов |
| Детекция читов по хешу | Аж 3 хеша (Memesense, Xone, Midnight) |
| Prefetch анализ | Перебор имён в C:\Windows\Prefetch |
| System Informer | Просто запуск бесплатного SystemInformer.exe |
| Инструменты | Бесплатные NirSoft утилиты которые и так можно скачать |
| Защита | IsDebuggerPresent + tasklist |

Всё это пишется за вечер. А тут можно просто скачать.

## Структура

```
├── launcher/                          # Лаунчер (deti00checker_v2.exe)
│   ├── src-tauri/src/
│   │   ├── main.rs
│   │   ├── core/
│   │   │   ├── commands.rs
│   │   │   └── launcher.rs
│   │   ├── services/
│   │   │   ├── api_service.rs
│   │   │   └── download_service.rs
│   │   ├── security/
│   │   │   ├── anti_disasm.rs
│   │   │   └── polymorphic.rs
│   │   └── models/
│   │       └── state.rs
│   └── build/                         # Оригинальный фронт (из бинарника)
│
├── checker/                           # Сам чекер (cs2checker.exe)
│   ├── src-tauri/src/
│   │   ├── main.rs
│   │   ├── discord_rpc.rs
│   │   ├── modules/
│   │   │   └── game_check.rs
│   │   ├── security/
│   │   │   ├── protection.rs
│   │   │   └── rate_limiter.rs
│   │   ├── services/
│   │   │   ├── browser_history.rs
│   │   │   ├── memory_scanner.rs
│   │   │   ├── steam_service.rs
│   │   │   ├── telegram_reporter.rs
│   │   │   ├── system_info.rs
│   │   │   ├── prefetch_service.rs
│   │   │   ├── quick_folder_scan.rs
│   │   │   ├── customization_service.rs
│   │   │   ├── game_analyzer.rs
│   │   │   └── scanner/
│   │   │       ├── scanner_engine.rs
│   │   │       ├── file_scanner.rs
│   │   │       ├── utils/fast_filters.rs
│   │   │       └── detectors/ (exloader, memesense, midnight, xone)
│   │   └── frontend/commands/         # Все Tauri IPC хендлеры
│   └── build/                         # Оригинальный фронт (из бинарника)
│
├── build.bat                          # Собирает оба проекта
├── README.md
└── SECRETS_ANALYSIS.md
```

## Что внутри нашлось

### Telegram бот (дампнут из памяти процесса)
```
Токен:   8645841623:AAEiCU9Vflyp06b21bdYWAPFWNGJ081uNto
Chat ID: 1317562927
```
Шлёт отчёты через `POST /sendDocument`, формат multipart, файл `cs2_scan_report_<имя_пк>.txt`.

Токен и chat_id зашиты в customization.dat (зашифрован), но после запуска лежат в памяти открытым текстом. Дампнул через ReadProcessMemory.

### API ключ бэкенда
```
d2b5ae9308d3278d541b37ff0cca11cf2c9296b101c12c156742a38b87813a4b
```
Один на всех, шлётся как `X-API-Key` к cs2checker.ru/api.

### AES ключ
```
CS2_CHECKER_SECURE_KEY_2024_V1 + два нулевых байта (итого 32)
```
Шифрует customization.dat, режим AES-256-CTR, первые 16 байт файла = IV.

### Ещё один ключ в hex
```
9D93EA929B9AE9929C99E898EF9C92EE9D9CE8989A9C9BEEEEEE9A929BEA9A92
```

### GitHub
- Аккаунт: cr1stal12444
- Репо: Cs2-checker (https://github.com/cr1stal12444/Cs2-checker)
- Релизы качаются оттуда же, это вся "серверная инфраструктура".

### API url
```
cs2checker.ru/api/public              - статус
cs2checker.ru/api/download/cs2checker - скачать чекер
cs2checker.ru/api/download/tools      - скачать тулзы
```

### Discord
```
https://discord.gg/85YQR8sKFb
```

### Хеши читов которые он "детектит" (Чел пади сидит, и каждый день скачивает каждый из читов для обновления, ибо у них билды чуть ли не каждый день разные xDD)
```
Memesense: 2e737dfb6b404d6fb5760359d33ab98c2c7462bedc70b81e1fafd8fa27ea45e5
Xone:      31227a46942215f6c45113f9edad4d81f51628faa355a91a0b9ec00a81606ab3
Midnight:  0345b8892bc6e55c85e4683d8fa68c512b2ae2c79eb95a5826bc15032bdf14aa
```
Три хеша. Три. За 300 рублей в месяц.

### Юзернейм разработчика
```
daniel (из путей C:\Users\daniel\.cargo\ в бинарнике)
```

## Как собрать

Нужно:
- Windows 10/11
- Rust (`winget install Rustlang.Rustup`)
- Node.js (`winget install OpenJS.NodeJS.LTS`)

```
build.bat
```

Или руками:
```
cd launcher/src-tauri && cargo build --release
cd ../../checker/src-tauri && cargo build --release
```

Результат:
```
launcher/src-tauri/target/release/deti00checker-v2.exe
checker/src-tauri/target/release/cs2-checker.exe
```

## Как это реверсилось

Никаких дизассемблеров, IDA, Ghidra не использовал. Rust в релиз сборке оставляет кучу инфы:
- Пути ко всем .rs файлам (из паник сообщений).
- Имена всех Tauri команд.
- Все строки, урлы, ключи, домены.
- Имена полей структур (serde).
- Версии всех crate зависимостей.

Фронтенд вытащен прямо из exe - Tauri 2 пакует Svelte ассеты как brotli блобы в PE секции.

Telegram токен дампнут из памяти запущенного процесса через ReadProcessMemory. Он там лежит открытым текстом после расшифровки customization.dat.

Эх эти нейронщики.... 
