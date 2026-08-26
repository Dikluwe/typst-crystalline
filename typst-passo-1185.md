# P1185 — individualizar o owner do hub `testing`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** ADR-0129; P1184 GREEN
**Classe ADR-0127:** linhagem interna, fluxo contínuo

## Objetivo e medição

Resolver a colisão:

```text
testing/math_oracle.md
├── 01_core/src/testing/math_oracle.rs
└── 01_core/src/testing/mod.rs
```

P1182 classificou o grupo como A sem claim compartilhada. Medição atual:
`math_oracle.rs` contém as fórmulas puras e testes do oráculo (hash de código
`65c64f45`); `testing/mod.rs` possui somente o hub test-only que declara
`math_oracle` (hash `b9cb23ff`). O L0 vigente mistura essas responsabilidades.
Não criar Núcleo.

Baseline condicionado a P1184: V15=22, V26=0, V5=417. Reproduzir duas vezes o
lint V15/V26 e congelar HEAD, árvore, hora e SHA do linter antes de editar.

## L0 primeiro

1. Manter `00_nucleo/prompts/testing/math_oracle.md` como owner exclusivo de
   `01_core/src/testing/math_oracle.rs`. Preservar fórmulas, fontes vanilla,
   pureza/independência do código produtivo e critérios P969+ posteriores.
   Remover somente a alegação de ownership do hub.
2. Criar `00_nucleo/prompts/testing/mod.md` como owner exclusivo de
   `01_core/src/testing/mod.rs`: módulo test-only, declaração
   `pub(crate) mod math_oracle`, zero fórmula e zero comportamento produtivo.
3. Cada L0 recebe exatamente um `Hash do Código`; nenhum duplica o contrato do
   outro.

## Resselo e GREEN

Calcular `code_hash` removendo apenas `@prompt-hash`; escrever a metadata;
depois calcular o SHA-256 do L0 completo e atualizar somente os headers. O
source do oráculo mantém seu path; o hub reponta para `testing/mod.md`.

Exigir:

- V15 22→21; V26=0; V5 esperado 417→415 e ausência dos dois sources;
- dry-run global bloqueado pelas 21 colisões restantes e zero writes;
- sources byte-idênticos fora de `@prompt`/`@prompt-hash`;
- `cargo test -p typst-core testing::math_oracle` seleciona testes e passa;
- `cargo build`, `git diff --check` e índice vazio.

Fechar em `00_nucleo/diagnosticos/typst-p1185-saneamento-testing-owners.md`,
com proveniência, hashes, contagens e confirmação de zero Núcleos/alteração
funcional.

## Próximo passo

P1186 individualiza `compiler/eval/repr.rs` do hub `foundations`.
