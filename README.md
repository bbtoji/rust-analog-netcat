# rust-analog-netcat 
Простой аналог Netcat на Rust (Linux only)
## Возможности

-  Режим сервера (прослушивание порта)
-  Режим клиента (подключение к удалённому хосту)
-  Выполнение базовых Linux комманд.

```sh
$ cargo build --release
```
Прослушивание(сервер):
```sh
$ ./target/release/rustcat listen --port 1337
```
Подключение(клиент):
```sh
$ ./target/release/rustcat connect --host 127.0.0.1 --port 1337
```
