:: Меняем кодировку консоли на UTF-8
chcp 65001 >nul

set "this_dir=%~dp0"
:: Удаляем обратный слэш в конце
set "this_dir=%this_dir:~0,-1%"

:: -C dir требует +nightly -Z unstable-options
cargo +nightly -Z unstable-options -C "%this_dir%" build --all-targets 

:: Ждём нажатие Enter перед закрытием консоли
pause
