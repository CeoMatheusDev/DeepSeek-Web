fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("falha ao iniciar a janela do DeepSeek");
}
