# Prompt L0 — `infra/export/tests/mod` — Testes agregadores E2E
Hash do Código: 9348b39d

**Camada**: L3 (apoio — testes)
**Ficheiro alvo**: `03_infra/src/export/tests/mod.rs`
**Criado em**: 2026-05-19 (P307c, como `tests.rs`)
**Decomposto em**: 2026-09-22 (P1479)
**ADRs**: ADR-0037 Regra 5 Ajuste C (testes E2E cross-cutting), ADR-0100 (coesão L3)

---

## Contexto

Sucessor de `infra/export/tests.md` (`03_infra/src/export/tests.rs`,
hash `0c915794`, removido neste passo). Ficheiro agregador de **230
testes** unitários e end-to-end do exporter PDF que cruzam múltiplos
domínios (gradients+images+multifont+layout) na mesma invocação
`export_pdf` — o caso genuíno de ADR-0037 Regra 5 Ajuste C, que não é
candidato a decomposição (perda de cobertura cross-feature).

Distribuição por feature (counts P307a, inalterada por este passo):

| Prefixo | Tests | Cluster |
|---|---:|---|
| `p270_*` | 69 | CMYK gradient |
| `p273_*` | 56 | Gradient relative + clip |
| `p272_*` | 15 | Conic Coons |
| `p274_*` | 14 | Adaptive N |
| `p269_*` | 13 | Gradient focal |
| `p281_*` | 10 | PageContext unificado |
| `p263_*` | 8 | Gradient linear |
| Outros | 45 | distribuídos |
| **Total** | **230** | |

## P1479 — decomposição do ficheiro único em `tests/`

Ver `typst-passo-1479.md` para a medição completa.

### Medição anterior à decisão

`crystalline-lint .` reportava `error: Erro de sintaxe: Syntax error
detected in AST [PARSE] --> ./03_infra/src/export/tests.rs:11146`
desde a integração de P1477 — bloqueando o gate `0 erros` de P1477 e
P1478 apesar de ambos estarem funcionalmente GREEN (`rustc` sempre
aceitou o ficheiro sem qualquer erro). Investigação por bissecção
isolou a causa-raiz a um identificador local `raw` numa função
específica (`assert_cidset_present_and_dense`,
`tests/p1477_pdfa_icc_version_and_cidset_tests.md`) — não a tamanho de
ficheiro (hipótese inicial, refutada quando o próprio ficheiro isolado
de 494 linhas ainda falhava). Corrigido lá, não aqui — este ficheiro
(`mod.rs`) só perdeu conteúdo, não ganhou código novo.

### Decisão

`tests.rs` → `tests/mod.rs`. Os quatro módulos por-passo
(`p1475`/`p1476`/`p1477`, extraídos daqui; `p1478`, extraído de
`builder.rs` onde tinha sido colocado temporariamente por decisão de
P1478) mudam para ficheiros próprios em `tests/`, cada um com o seu
próprio L0 (ADR-0129, mesmo padrão de `gradients/*.md`). Os 230 testes
históricos cross-cutting **ficam** aqui — não são "cluster" no sentido
de ADR-0100 (não testam um único passo isoladamente).

`export/mod.rs` não muda — `#[cfg(test)] mod tests;` resolve
`tests/mod.rs` automaticamente pela convenção padrão de módulos Rust
(ficheiro vs. diretório com `mod.rs` são intercambiáveis).

### Scope-out explícito

Os 230 testes históricos não são candidatos a decomposição adicional
por este passo — critério de decisão fica inalterado
("decomposição futura só se ratio LOC/coverage justificar", herdado do
L0 anterior). Se `tests/mod.rs` voltar a crescer o suficiente para
reativar algum limite de ferramenta, a decisão é nova, não implícita.

## Restrições estruturais

- Categoria Regra 6 (ADR-0037): "infraestrutura de testes E2E". Excede
  limite 800 LOC por design — testes cruzam features (gradients dentro
  de Groups, multifont com images, etc.).
- `use super::*` traz toda a API interna de `export/mod.rs`
  (re-exports dos submódulos).
- Módulos por-passo declarados via `#[cfg(test)] mod pXXXX_..._tests;`
  no fim do ficheiro — resolvidos para os ficheiros irmãos em
  `tests/`.

## P1480 — registo do módulo de identificação PDF/UA

O agregador declara `p1480_pdfua_identification_tests`, owner test-only
individualizado por `infra/export/tests/p1480_pdfua_identification_tests.md`.
O módulo novo cobre exclusivamente `pdfuaid` e
`/ViewerPreferences /DisplayDocTitle true`; a validação de título ausente
permanece nos testes internos de `03_infra/src/pipeline.rs`, owner
`infra/pipeline.md`. Esta declaração não altera a API nem o comportamento
produtivo do exporter.

## P1481 — registro do módulo de estrutura PDF/UA-1

O agregador declara `p1481_pdfua_structure_tests`, com owner
`infra/export/tests/p1481_pdfua_structure_tests.md`. O módulo verifica
idioma no catálogo/XMP, nó+MCID de texto real e controle não-UA.

## Interface

Nenhuma — é módulo `#[cfg(test)]` com `#[test]` functions e
declarações `mod` para os submódulos por-passo. Não consumido por
código de produção.

## Critérios de verificação

`cargo test -p typst-infra --lib export::tests` deve passar o mesmo
total de testes de antes da decomposição (230 aqui + os módulos por-passo,
incluindo os 5 testes de P1480). Falhas indicam regressão em algum cluster L3.
`crystalline-lint .` deve reportar zero erros, incluindo zero achados
`[PARSE]` (critério de sucesso primário de P1479).

## Não-objectivos

- **Não** testa paridade vanilla (vive em `lab/parity/`).
- **Não** testa binário cristalino vs cristalino (vive em `p307b_snapshot_tests.rs`).
- **Não** valida performance.

## P1483 — módulo de testes de separação spot

Registrar `p1483_pdf_spot_tests`, com owner test-only próprio
`infra/export/tests/p1483_pdf_spot_tests.md`. Ele exercita
fill/stroke/texto, colorants `None`/`All`/nomeado, tint
0/50/100, reuso de recurso, gradientes e controle processual
em PDF; verifica fallback dos backends sem spot sem depender
de bytes PDF exatos.

## P1486 — módulo de testes de rótulos de páginas

Registrar `p1486_pdf_page_labels_tests`, owner test-only próprio
`infra/export/tests/p1486_pdf_page_labels_tests.md`. O módulo compara os
rótulos de cada página retida, incluindo contadores lógicos deslocados,
seleção descontínua e fallback físico; o teste não se satisfaz com a
presença de `/PageLabels` no catálogo.
