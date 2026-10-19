# Relatório de Verificação — Passo 811: `frak()` sem argumento corrompe o PDF gerado

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `c98ffc8ac` (HEAD) + working tree P808–P811
- **Hora da Medição:** 2026-07-21 ~20:30 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #13 de P810 (o mais grave da fila): `$ frak() $` — o cristalino compilava (exit 0) e produzia um **PDF inválido**; o vanilla rejeita com `error: missing argument: body`. Corrupção confirmada com `pdfinfo`: `Kid object (page 1) is wrong type (null)` — o `/Pages` declarava `/Kids [3 0 R]` sem o objecto 3 existir.

## 2. Diagnóstico e Medição

Duas causas independentes medidas:
1. **Export (a corrupção, geral):** caminhos de build usavam `doc.pages.len().max(1)` para o `/Kids` mas emitiam 0 objectos de página quando o documento não tem conteúdo. A corrupção **não era específica de `frak()`** — ficheiro vazio, `$ $` e ` ` produziam o mesmo PDF inválido (medido); o vanilla emite 1 página em branco A4 nos três casos (medido).
2. **Eval (a validação em falta):** `wrap_math_style` aceitava body ausente como `Content::Empty` silencioso (sancionado pelo L0, actualizado neste passo).

## 3. A Solução Implementada

Ambas (o prompt previa os dois cenários; a medição mostrou que corrigir só a validação deixaria ` ` a corromper):
- `PdfBuilder::build()` (ponto único dos exports) sintetiza 1 página em branco A4 quando `doc.pages.is_empty()` — sem clone no caminho quente.
- `wrap_math_style` (12 funções math style) rejeita body ausente com `missing argument: body` (literal vanilla medido).
- L0s actualizados: `infra/export/builder.md` §P811, `stdlib/math_style.md` (revoga o `Content::Empty` sancionado).

## 4. Testes Automatizados Persistidos (com nomeação explícita)

Novos (os 3 falharam antes): `p811_frak_sem_argumento_erro_missing_body` (+ controlo `frak(A)`), `p811_bb_sem_argumento_erro_missing_body`, `p811_export_documento_sem_paginas_emite_pagina_em_branco` (infra — `/Kids [3 0 R]` + `/Type /Page /` + MediaBox A4). Existente alterado: `p311b_empty_args_produces_empty_body` marcado `#[ignore]` (consagrava o comportamento revogado; padrão P800).

Validação E2E: `$ frak() $` → erro literal do vanilla ✓; vazio/`$ $`/` ` → `Pages: 1` válidos ✓; `frak(A)` → `𝔄` idêntico ✓.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4348 passed; 1 ignored → DEPOIS 4349 passed; 0 failed; 2 ignored (+2 testes, +1 ignore ✓)
Suite 'typst-infra':       ANTES 658 passed; 5 ignored → DEPOIS 659 passed; 5 ignored (+1 ✓)
Workspace: typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas
crystalline-lint . → exit 0
```

## 6. Âmbito

Os restantes pontos do achado #13 de P810 (`display`/`inline`/`script`/`sscript` sem efeito geométrico, itálico em wrappers de tamanho, `scr` + variation selectors, `NN`/`RR`/`ZZ`/`QQ`/`CC`, `#math.display`) continuam pendentes para passo separado.
