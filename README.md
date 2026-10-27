# DeepSeek-Web

## Fase 0

Janela Tauri 2 que abre diretamente o chat oficial em `https://chat.deepseek.com/`. Ainda não há painel, scripts injetados ou ferramentas.

### Pré-requisitos no Fedora

Instale Rust/Cargo e as dependências de sistema indicadas nos [pré-requisitos oficiais do Tauri para Fedora](https://v2.tauri.app/start/prerequisites/#linux). Confirme os componentes principais:

```bash
rustc --version
cargo --version
pkg-config --modversion gtk+-3.0 webkit2gtk-4.1
```

### Compilar e executar

Na raiz do repositório:

```bash
cargo build --manifest-path src-tauri/Cargo.toml
cargo run --manifest-path src-tauri/Cargo.toml
```

Se funcionar, uma janela intitulada **DeepSeek Harness** abrirá o site oficial, que poderá solicitar login. O carregamento depende de conexão com a internet e da compatibilidade do site com WebKitGTK. Se a compilação ou o carregamento falhar, registre a mensagem de erro e o comportamento observado; não inclua credenciais ou cookies ao compartilhá-los.
