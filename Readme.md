````markdown
# financial-parser

`financial-parser` — CLI-утилита для чтения, конвертации и записи финансовых сообщений форматов **MT940** и **CAMT053**. Также поддерживаются чтение и запись форматов `xml` и `csv`.

## Установка

Сборка из исходников с помощью Cargo:

```bash
git clone <repo_url>
cd financial-parser
cargo build --release
````

Исполняемый файл будет доступен в `target/release/financial-parser`.

## Использование

```bash
financial-parser --in-format <mt940|camt053|xml|csv> [--out-format <mt940|camt053|xml|csv>] \
           [-i <input_file>] [-o <output_file>] [-v]
```

### Параметры

* `-i, --input` — Входной файл (по умолчанию `-` — stdin)
* `-o, --output` — Выходной файл (по умолчанию `-` — stdout)
* `--in-format` — Формат входного файла (`mt940`, `camt053`, `xml`, `csv`)
* `--out-format` — Формат выходного файла (по умолчанию такой же, как `in-format`)
* `-v, --verbose` — Включает подробный вывод

### Примеры

Конвертация файла MT940 в CAMT053 и вывод результата в stdout:

```bash
financial-parser --in-format mt940 -i statement.mt940 --out-format camt053
```
