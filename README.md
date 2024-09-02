# Тестовое задание

## Постановка задачи

Необходимо разработать демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе. [Модель данных](https://github.com/VadimYarovoy/WB-L0/wiki/Data-format) в формате JSON прилагается к заданию.

## Требования

- Заказы должны быть иммутабельны (не меняются после создания, только добавляются). Исходя из этого, подумайте насчет модели хранения в кэше и в PostgreSQL. Модель в файле model.json
- Подумайте как избежать проблем, связанных с тем, что в ручку (http-endpoint) могут закинуть что-угодно
- Для тестирования сделайте себе отдельный скрипт для публикации данных через API
- Подумайте, как не терять данные в случае ошибок или проблем с сервисом

## Конфигурация

Все параметры задаются в `docker-compose.yaml`при развертывании в качестве контейнеров. Если запускать локально только бинарный крейт, то параметры задаются в `config.toml`.

## Локальное развертывание

Шаг 1: Клонирование репозитория

```bash
git clone https://github.com/VadimYarovoy/WB-L0.git
```

Шаг 2: Сборка

```bash
docker-compose build
```

Шаг 3: Запуск

```bash
docker-compose up
```

## API

GET: Получение записи с указанным id

```bash
curl -X GET "http://localhost:3000/api/orders/get/1"
```

POST: Создание новой записи

```bash
curl -X POST http://localhost:3000/api/orders/create \
     -H "Content-Type: application/json" \
     -d '{"keys": "values"}'
```

## Скрипт для запуска запросов

Шаг 1: Перейти в директорию `backend`

```bash
cd backend
```

Шаг 2: Запуск скритпа для запросов

- GET: один входной параметр - `id`

```bash
cargo run --bin query_script get {id}
```

- GET: один входной параметр - `file`, имя файла в папке `models` без указания расширения. Расширение файла должно быть `Json`

```bash
cargo run --bin query_script post {model}
```

## Нагрузочное тестирование

[Результаты](https://github.com/VadimYarovoy/WB-L0/wiki/Performance-tests) тестирования с использование WRK
