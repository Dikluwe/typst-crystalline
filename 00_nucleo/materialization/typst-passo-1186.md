# P1186 — separar `eval/repr` do hub `foundations`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** P1185 GREEN
**Classe ADR-0127:** linhagem interna, fluxo contínuo

## Objetivo e medição

Resolver:

```text
compiler/stdlib/foundations.md
├── 01_core/src/compiler/stdlib/foundations/mod.rs
└── 01_core/src/compiler/eval/repr.rs
```

O hub apenas declara/reexporta nós (hash `c19e6bc9`). `eval/repr.rs` implementa
a representação exaustiva de valores/conteúdo/seletores (hash `333feb71`).
P1182 mediu ausência de claim comum: proximidade histórica não justifica
Núcleo. Baseline condicionado: V15=21, V26=0, V5=415.

## L0 primeiro

1. Atualizar `compiler/stdlib/foundations.md` como owner exclusivo de
   `foundations/mod.rs`. Conservar topologia/reexports, remoção de `len` e
   scope dos nós; remover as secções P1140.2, P1140.3-A e P1161 que pertencem
   ao algoritmo de repr.
2. Criar `compiler/eval/repr.md` como owner exclusivo de `eval/repr.rs`.
   Transferir integralmente as decisões de repr acima e medir no source as
   famílias atualmente cobertas antes de resumir seu contrato. Aceitação é
   morfologia textual pública, não igualdade da estrutura Rust.
3. Não alterar `compiler/stdlib/foundations/repr.md`, owner do constructor
   nativo `native_repr`; distinguir chamada da serialização efetiva em eval.
4. Não criar Núcleo.

## Resselo, testes e gates

Usar a ordem validada em P1183: `Hash do Código` primeiro; hash integral do
prompt depois; headers por `apply_patch`. Em `foundations/mod.rs`, manter o
path atual e mudar só `@prompt-hash`; em `eval/repr.rs`, mudar as duas linhas.

Exigir:

- RED reproduzido duas vezes sobre proveniência congelada;
- V15 21→20; V26=0; V5 esperado 415→413 e ausência focal;
- dry-run bloqueado pelas 20 colisões restantes, zero writes;
- sources idênticos fora da linhagem;
- testes focais descobertos por `cargo test -p typst-core compiler::eval::repr`;
  se o filtro for vazio, executar `cargo test -p typst-core`;
- `cargo build`, diff limpo e índice vazio.

Fechar em `00_nucleo/diagnosticos/typst-p1186-saneamento-foundations-repr.md`.

## Próximo passo

P1187 separa o build script da CLI e cria o primeiro Núcleo Tekt do corpus.
