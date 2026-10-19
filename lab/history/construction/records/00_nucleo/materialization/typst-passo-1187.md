# P1187 — individualizar CLI/build e nuclear a identidade do build

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** P1186 GREEN; ADR-0129; ADR Tekt-0004 pinada
**Classe ADR-0127:** linhagem/norma interna; nenhuma mudança na CLI pública

## Objetivo e classificação

Resolver:

```text
shell/cli.md
├── 02_shell/src/cli.rs
└── 02_shell/build.rs
```

`cli.rs` possui parsing/intents e exibe versão+commit (hash de código
`00d53433`); `build.rs` captura o commit em compile-time e configura rerun
(hash `8cbf4ebb`). Os owners são distintos, mas compartilham uma obrigação
real: o valor produzido pelo build script é o consumido pela apresentação da
identidade do CLI, com fallback explícito. Portanto criar Núcleo, não duplicar
a claim. Baseline condicionado: V15=20, V26=0, V5=413.

## Núcleo e owners — L0 primeiro

1. Criar `00_nucleo/prompts/_nuclei/shell/build-identity.toml` com schema
   Tekt 1/kind nucleus e claims atômicas:
   - `must`: identidade do build flui por `TYPST_COMMIT_SHA` em compile-time;
   - `must`: o commit exibido deriva desse valor e pode ser abreviado apenas
     na apresentação humana;
   - `must`: ausência de git/env possui fallback explícito e não faz I/O em
     runtime;
   - `must-not`: confundir hash cristalino, hash vanilla e versão do crate.
2. Atualizar `shell/cli.md` como owner exclusivo de `cli.rs`. Conservar todo o
   contrato público vigente/pending e a apresentação `--version`; remover o
   corpo operacional de `build.rs`. Adicionar pin completo do Núcleo.
3. Criar `shell/build.md` como owner exclusivo de `02_shell/build.rs`, com
   `Command git rev-parse`, precedência do env e gatilhos `.git/HEAD`/`.git/refs`.
   Adicionar o mesmo pin do Núcleo.
4. O Núcleo não contém owner, consumer nem `Hash do Código`; os prompts
   permanecem completos em seu escopo específico.

Calcular primeiro SHA-256 completo do TOML; inserir pins idênticos nos dois
prompts; então gravar `Hash do Código`, calcular hashes integrais efetivos dos
prompts e ressellar sources. Validar V26 antes de V5. Se o schema/pin produzir
V26, parar sem improvisar formato e registrar a saída exata.

## GREEN e verificação

Exigir V15 20→19, V26=0, V5 esperado 413→411, ausência focal e exatamente
dois prompts consumidores do Núcleo. Dry-run continua bloqueado por 19 V15 e
não escreve. Sources idênticos fora da linhagem; `cli.rs` conserva path,
`build.rs` reponta para `shell/build.md`.

Executar testes focais de versão/CLI existentes, `cargo test -p typst-shell`,
`cargo build`, `git diff --check` e índice vazio. Não alterar constantes,
parsing, output, build script ou contrato público.

Fechar em `00_nucleo/diagnosticos/typst-p1187-saneamento-cli-build.md`, incluindo
digest/pins do primeiro Núcleo e prova V26.

## Próximo passo

P1188 individualiza formatter/snapshot de `info` e nucleariza a projeção segura.
