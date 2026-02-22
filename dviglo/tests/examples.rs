// Запускает примеры как тесты

use std::process::Command;


fn test_example(name: &str) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(&["run", "--quiet", "--example", name])
        .status()
        .expect("Failed to execute cargo run");

    if status.success() {
        Ok(())
    } else {
        Err(format!("Example \"{}\" failed to run!", name))
    }
}


// Генерирует функции для запуска примеров
macro_rules! test_examples {
    ($($name: ident), *) => {
        $(
            #[test]
            fn $name() -> Result<(), String> {
                test_example(stringify!($name))
            }
        )*
    }
}


test_examples!(
    hello
);
