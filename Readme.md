# financial-parser

`financial-parser` — CLI-утилита для чтения, конвертации и записи финансовых сообщений форматов **MT940** и **CAMT053**. Также поддерживаются чтение и запись форматов `xml` и `csv`.

**Warning:** Часть данных теряется при конвертации из-за частичной совместимости форматов!

**Warning:** Циклическая конвертация **MT940** → **CAMT053** → **MT940** может работать некорректно!

## Установка

Сборка из исходников с помощью Cargo:

```bash
git clone <repo_url>
cd financial-parser
cargo build --release
```

Исполняемый файл будет доступен в `target/release/financial-parser`.

## Использование

```bash
financial-parser --in-format <mt940|camt053|xml|csv> [--out-format <mt940|camt053|xml|csv>] \
           [-i <input_file>] [-o <output_file>] [-v]
```

### Параметры

* `-i, --input` — входной файл (по умолчанию `-` — stdin)
* `-o, --output` — выходной файл (по умолчанию `-` — stdout)
* `--in-format` — формат входного файла (`mt940`, `camt053`, `xml`, `csv`)
* `--out-format` — формат выходного файла (по умолчанию такой же, как `in-format`)
* `-v, --verbose` — включает подробный вывод

### Примеры

Конвертация файла MT940 в CAMT053 и вывод результата в stdout:

```bash
financial-parser --in-format mt940 -i statement.mt940 --out-format camt053
```

Запуск готовых примеров:

```bash
cargo run --example read_and_write_xml
```

```bash
cargo run --example convert_mt940_to_camt053
```

```bash
cargo run --example convert_camt053_to_mt940
```
