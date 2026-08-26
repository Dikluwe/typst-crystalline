# Prompt L0 — `shell/build` — captura de identidade compile-time
Hash do Código: 24e7f743


**Camada:** tooling de build da L2
**Ficheiro proprietário:** `02_shell/build.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/build-identity.toml sha256:a97f32705be8700feaa6a5c89440ffa49d15c11145ea83744307aefc65a50297

## Contrato

Declarar rerun para `TYPST_COMMIT_SHA`, `.git/HEAD` e `.git/refs`. Quando a
variável não foi fornecida, executar `git rev-parse HEAD` e publicar stdout
válido como `cargo:rustc-env=TYPST_COMMIT_SHA`. Falhas são silenciosas e deixam
o fallback para o consumer compile-time. Nenhum I/O ocorre no binário runtime.
