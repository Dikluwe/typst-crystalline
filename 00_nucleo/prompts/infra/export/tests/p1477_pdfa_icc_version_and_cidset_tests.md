# Prompt L0 — `infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests`
Hash do Código: 7d69ea74

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs`
**Criado em**: 2026-09-22 (P1477, então dentro de `tests.rs`)
**Extraído em**: 2026-09-22 (P1479)
**ADRs**: ADR-0037 Regra 5 (extraído "por cluster", ADR-0100 §Positivas), ADR-0129 (1:1)

---

## Contexto

11 testes do contrato P1477 (segundo perfil ICC sRGB por versão do
PDF; `/CIDSet` universal em CIDFonts embutidas). Ver
`00_nucleo/materialization/typst-passo-1477.md` e `infra/export/
builder.md`/`infra/export/images.md` §P1477.

## P1479 — extração de `tests.rs` **e** a causa-raiz do achado `[PARSE]`

Movido de dentro do antigo `03_infra/src/export/tests.rs` para
ficheiro próprio, per `typst-passo-1479.md`. **Este é o ficheiro onde
o achado `[PARSE]` do `crystalline-lint` (bloqueava P1477/P1478 desde
a integração) foi isolado e corrigido** — não era tamanho de ficheiro
(a hipótese original de P1477/P1478): mesmo isolado em 494 linhas, o
erro persistia.

### Medição — bissecção até à causa-raiz

Bissecção controlada (truncamento com restauro entre tentativas,
depois trivialização preservando estrutura de chavetas) isolou a causa
a um identificador local: a variável `raw` em `assert_cidset_present_
and_dense` (`let raw = object_stream_raw_bytes(...); let decompressed
= inflate(&raw);`). Renomear `raw` → `raw_bytes` faz o achado `[PARSE]`
desaparecer, sem qualquer outra mudança — confirmado isoladamente
(só o rename) e na íntegra (`crystalline-lint .` no projeto inteiro
→ zero achados `[PARSE]`). Não foi possível determinar a causa exata
dentro do parser do `crystalline-lint` (binário fora do alcance desta
sessão para auditar) — `rustc` sempre aceitou ambas as formas (`raw`
e `raw_bytes`) sem qualquer erro ou aviso, confirmando que não é uma
regra da linguagem Rust, é um limite/bug específico da ferramenta.

### Achado adicional — deduplicação (papel C)

As duas funções que exercitam `/CIDSet` (`build_cidfont`/
`build_multifont`) tinham corpos quase idênticos linha-a-linha.
Extraídas para um helper único, `assert_cidset_present_and_dense(pdf,
context)`, reduzindo duplicação real — decisão tomada durante a mesma
investigação (uma hipótese intermediária, refutada depois pelo achado
do `raw`, mas mantida por ser uma melhoria genuína e por já estar
verificada GREEN). Consequência observável mínima: o teste do caminho
`build_multifont` ganhou a mesma verificação de `/Filter /FlateDecode`
que só o de `build_cidfont` tinha antes — reforço de cobertura, não
mudança de contrato (`build_multifont` usa o mesmo `build_cidset_
stream` que `build_cidfont`, a asserção já era verdadeira, só não
testada nesse caminho).

## Restrições estruturais

- `use super::*` traz a API de `tests/mod.rs`.
- `use std::io::Read;` necessário para `flate2::read::ZlibDecoder::
  read_to_end`.

## Interface

Nenhuma — `#[cfg(test)]` com `#[test]` functions.

## Critérios de verificação

`cargo test -p typst-infra --lib p1477_pdfa_icc_version_and_cidset_tests`
→ 11/11. `crystalline-lint .` → zero achados `[PARSE]` em todo o
projeto (critério de sucesso primário de P1479, verificado aqui
especificamente porque a causa-raiz vivia neste ficheiro).

## Não-objectivos

Ver `infra/export/tests/mod.md`.
