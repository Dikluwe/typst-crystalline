# Prompt L0 — P1486 testes de rótulos de páginas PDF
Hash do Código: c8423432

**Camada:** L3 (testes)
**Ficheiro alvo:** `03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs`
**Criado em:** 2026-09-24
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O diagnóstico `00_nucleo/diagnosticos/typst-p1486-page-labels.md` registra
as fixtures, hashes e comandos de extração. No vanilla ratificado,
seleção física 1+3 rotula arábico `1, 3`, romano `i, iii`, fallback sem
padrão `1, 3` e padrão após `counter(page).update(10)` `10, 12`.
O cristalino anterior a P1486 apresenta `1, 2` e não emite
`/PageLabels` (`infra/export/builder.rs:1134-1140`). O rótulo do leitor
é observável; object IDs e bytes totais não são (ADR-0107).

## Contrato de testes

Testes RED→GREEN constroem `PagedDocument` com quatro páginas, exercitam
as três variantes de exportação e extraem os rótulos efetivos a partir
da number tree e dos seus estilos, não apenas a presença da chave.
Cobrir sem seleção, seleção 1–4, 2–4, 1+3, só 4, e seleção vazia;
numeração arábica, romana, literal com prefixo/sufixo, sem padrão,
e `Page.logical_number` deslocado. Verificar que seleção retendo todas
as páginas não cria fallback desnecessário. Conferir que catálogo e
number tree são internamente coerentes, sem amarrar IDs, offsets ou
serialização. Regressões de tagged/PDF/A/PDF/UA e de conteúdo
desenhado pertencem ao mesmo módulo ou ao gate externo com `qpdf` e
`pypdf`; nunca a uma comparação de bytes de PDF completo.
