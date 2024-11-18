CARGO = cargo
CARGO_FORMAT = cargo fmt
CARGO_CLIPPY = cargo clippy
CARGO_TEST = cargo test
CARGO_BUILD = cargo build
CARGO_RUN = cargo run

# Команди

# Запуск програми
run:
	$(CARGO_RUN)

# Запуск тестів
test:
	$(CARGO_TEST)

# Форматування коду
fmt:
	$(CARGO_FORMAT)

# Лінтинг з Clippy
clippy:
	$(CARGO_CLIPPY)

# Запуск формату та лінтингу перед коммітом
check: fmt clippy

# Збірка проєкту
build:
	$(CARGO_BUILD)

# Очистка проєкту (видалення збудованих файлів)
clean:
	$(CARGO) clean

# Запуск усіх тестів і перевірка
full_check: test check

# Генерація документації
doc:
	$(CARGO) doc --no-deps

# Всі основні команди разом
all: fmt clippy test build

.PHONY: run test fmt clippy clean build all check full_check doc