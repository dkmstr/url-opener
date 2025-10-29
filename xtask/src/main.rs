use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: cargo xtask bundle");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "bundle" => bundle(),
        _ => {
            eprintln!("Comando desconocido");
            std::process::exit(1);
        }
    }
}

fn bundle() {
    // 1. Compilar en release
    let status = Command::new("cargo")
        .args(&["build", "--release", "-p", "app"])
        .status()
        .expect("falló cargo build");
    assert!(status.success());

    // 2. Rutas
    let target_dir = PathBuf::from("target/release");
    let bin = target_dir.join("app"); // nombre del binario
    let app_bundle = PathBuf::from("UrlOpener.app");
    let contents = app_bundle.join("Contents");
    let macos_dir = contents.join("MacOS");

    // 3. Crear estructura
    fs::create_dir_all(&macos_dir).unwrap();
    fs::create_dir_all(contents.join("Resources")).unwrap();

    // 4. Copiar binario
    fs::copy(&bin, macos_dir.join("url_opener")).unwrap();

    // 5. Copiar Info.plist
    fs::copy("app/Info.plist", contents.join("Info.plist")).unwrap();

    println!("Bundle creado en {:?}", app_bundle);
}
