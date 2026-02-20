# Spec: `build.rs` (Build Script do Cargo)

## 1. Objetivo Central
O script `build.rs` é um cidadão nativo do modelo de compilação do Cargo. Sua responsabilidade é puramente a de preparar variáveis macro do sistema ou do processo de building antes da compilação do crate alvo (`typst-cli` original, agora `wiring`). Ele não faz parte da biblioteca runtime.

Ações do Script:
1. Exportar a variável de ambiente alvo e repassá-la via `cargo:rustc-env=TARGET={...}`.
2. Ler a flag de geração de artefatos `GEN_ARTIFACTS`.
3. Se ativada, utilizar o módulo de parsing CLI nativo (antigo `args.rs`, novo `args_cli.rs`) para instanciar a factory do Clap sem iniciar a aplicação.
4. Renderizar `.1` manpages manuais iterativamente por subcomandos (`typst.1`, `typst-watch.1`, etc) usando `clap_mangen`.
5. Gerar scripts auto completions para diferentes terminais do SO via `clap_complete`.

## 2. Posição Arquitetural Tekt/Crystal
Sendo um Build Script acoplado na abstração do Cargo, não existe divisão razoável de L1/L3 dentro das fronteiras da própria aplicação. Qualquer abstração seria apenas _overengineering_ que lutaria contra a especificação rígida do Cargo.

Decisão Arquitetural:
- Assentar o `build.rs` nativo do Cargo diretamente no crate "Application Root", nosso `04_wiring/build.rs`.
- Alterar o acoplamento do `#[path = "src/args.rs"]` original para referenciar o módulo `args_cli.rs` corretamente desacoplado da antiga source tree.
