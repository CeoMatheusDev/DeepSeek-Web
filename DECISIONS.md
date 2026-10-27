# Decisões arquiteturais

## Escopo definido

- Aplicativo pessoal para Fedora 44, usando Tauri 2, Rust e WebKitGTK.
- A interface do DeepSeek é a do site oficial; o harness será acrescentado por scripts JS/TS injetados, sem reconstruí-la.
- A stack futura inclui `reqwest`, `serde`, `tokio` e `tauri-plugin-webproxy`, conforme definido pelo desenvolvedor. Esses componentes ainda não foram integrados.
- As fases são sequenciais. Terminal e arquivos deverão operar no workspace `~/deepseek-harness/workspace/`; execuções destrutivas exigirão confirmação explícita do usuário.

## Fase 0 — janela mínima

- `src-tauri/` contém o executável Rust. A configuração da janela está em `src-tauri/tauri.conf.json`.
- A janela principal carrega diretamente `https://chat.deepseek.com/` como URL externa, sem iframe e sem interface web local. Não é necessário servidor de desenvolvimento para esta fase.
- `tauri` fornece o runtime Tauri 2; `tauri-build`, confirmado para uso, é a dependência oficial de compilação usada pelo `build.rs` do template Tauri.
- As versões das dependências serão registradas no `src-tauri/Cargo.lock` gerado pelo primeiro build; ele ainda não foi gerado porque o ambiente de trabalho não tem Cargo.
- O ícone PNG simples em `src-tauri/icons/icon.png` existe para atender à geração do contexto Tauri no Linux; não é uma decisão de design da interface.
- O empacotamento está desativado (`bundle.active: false`): o objetivo atual é executar localmente, não distribuir o aplicativo.
- Não há comandos IPC, permissões remotas ou scripts injetados nesta fase. O formato das mensagens IPC ainda não foi decidido.
- `tauri-plugin-webproxy` permanece na stack planejada, mas não é integrado nesta fase. Antes da Fase 2, avaliar com o desenvolvedor a limitação registrada no [README do plugin](https://github.com/mantou132/tauri-plugin-webproxy): a página proxificada não chama o IPC Tauri diretamente.
