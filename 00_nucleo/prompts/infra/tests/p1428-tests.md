# L0 — Public fraction representation and provenance tests
Hash do Código: 22bca0fd

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/p1428_math_frac.rs`

## Current evidence

- `03_infra/tests/p1428_math_frac.rs:141` — one-pass HTML text extraction.
- `03_infra/tests/p1428_math_frac.rs:240`, `:271` — evaluated carriers and unique fraction bits.
- `03_infra/tests/p1428_math_frac.rs:303`, `:358` — public repr and route/diagnostic/MATH/binom controls.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Extract HTML text in one pass from text nodes only. Decode known named and valid numeric entities once; unknown/malformed entities, absent or unbalanced markup fail observably. Do not recursively decode or compare escaped markup as the language repr.

Evaluate through eval_to_module_with_sink and traverse the resulting Content. Require exactly one MathFracElem to observe num_deparenthesized and denom_deparenthesized, without inferring state from fixture spelling. Preserve coverage of the three public frac forms, explicit/omitted style and cascade, signature diagnostics, OpenType constants and binom independence.

Acceptance concerns public behavior, domain carriers and layout invariants. Do not execute mutants/external runners, implementation hashes, artificial orders or administrative artifacts. The complementary public controls belong to `00_nucleo/prompts/infra/tests/p1427-tests.md`.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T361: 03_infra/tests/p1428_math_frac.rs:100. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

No decoder de teste, reconhecer prefixo hexadecimal por get(..2).is_some_and(eq_ignore_ascii_case("#x")), que aceita somente #x/#X ASCII e não corta UTF-8. Manter radix 16, decimal, char::from_u32 e falha Unknown::HtmlEntity; decode uma vez. Adicionar teste de X maiúsculo, Unicode escalar, surrogate, fora do máximo, truncamento e prefixos UTF-8 inválidos antes da troca.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.
