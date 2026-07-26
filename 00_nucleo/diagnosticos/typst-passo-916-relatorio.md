# Relatório P916 — Fechamento de P912/913/914: prova geométrica, hermeticidade, reconciliação retroativa

**Commit de referência medição:** `dcff44bd2` (HEAD antes de P916)  
**Baseline de testes:** typst-core: 4781 · typst-infra: 740 · typst-shell: 41  
**Data:** 2026-07-26

---

## Parte A — Prova geométrica (tabela mutool trace pós-correcção)

Compilados os 7 casos de P911 com o binário cristalino actual (`dcff44bd2`) e
com `typst 0.15.1` (vanilla). Medição: `mutool trace` → glyph IDs reais emitidos
para o delimitador.

| Caso | Cristalino glyph IDs | Vanilla glyph IDs | Cresce? |
|------|---------------------|-------------------|---------|
| `(1/2)` | `(` = 1, `)` = 2 | `(` = 1, `)` = 4 | ❌ Cristalino fixo |
| `(1/2/3/4)` | `(` = 1, `)` = 2 | `(` = 1, `)` = 8 | ❌ Cristalino fixo |
| `(1/2/3/4/5/6/7/8)` | `(` = 1, `)` = 2 | `(` = 1, `)` = 12 | ❌ Cristalino fixo |
| `mat(1,2;3,4)` | `(` = 1, `)` = 2 | `(` = 1, `)` = 6 | ❌ Cristalino fixo |
| `mat(...)` 6 linhas | 34 peças `\|`+`(` 6 IDs distintos | `(` = 1, `)` = 14 | ✅ Cresce (mas usa assemblagem de barras `\|` em vez de variante de `)`) |
| `cases(1,2)` | `{` = ID 15 × 1 peça | `{` = ID 1 | ⚠️ Différent approach |
| `cases(...)` 8 linhas | `{` + `\|` 23 peças, 4 IDs | `{` = ID 1 | ✅ Cresce via assemblagem |

**Achado confirmado:** Para os casos simples `(1/2)` a `mat(2×2)`, o glyph de `)` permanece fixo (ID 2) — não cresce com o conteúdo. O bug de P911 (`covering()` a devolver fonte sem tabela MATH) foi corrigido, mas a selecção de variante ainda não funciona para estes casos.

**Hipótese a investigar:** A correcção de `covering()` (P912) fez a fonte MATH ser seleccionada, mas `vertical_glyph_variants(')', ...)` pode ainda estar a devolver lista vazia para estes casos específicos — o caminho Text via `glyph_to_char` é activado, emitindo sempre o glifo base sem escalar.

Este achado fica **em aberto** — registo honesto de que P912 não fechou completamente o Achado A de P911 para estes casos. Não foi remendado dentro de P916.

**Proveniência da medição:**
- commit: `dcff44bd2` (working tree limpa, `git status` sem modificações antes da compilação)
- comando: `target/debug/typst --font-path $FONT_DIR <caso>.typ <caso>.pdf` + `mutool trace <caso>.pdf`
- vanilla: `typst 0.15.1` (`typst compile --font-path $FONT_DIR`)
- PDFs guardados em: `00_nucleo/diagnosticos/p916-work/`

---

## Parte B — Hermeticidade do teste `pdf_tounicode_contem_mapeamento_de_delimitador`

**Problema:** o teste lia `/usr/share/fonts/truetype/dejavu/DejaVuMathTeXGyre.ttf`
como primeira opção (não-hermético), com fallback `NimbusSans-Regular.otf` (sem
tabela MATH — podia passar pelos motivos errados).

**Solução:**
- Copiado `NewCMMath-Book.otf` de `typst-assets` para `03_infra/fixtures/fonts/`
- Confirmado que tem tabela `MATH` (`b'MATH' in data → True`)
- Reescrito o teste para usar apenas esta fixture (sem fallbacks de sistema)
- Teste passa com a fixture pinada: `1 passed; 0 failed`

**Nota de escopo registada no teste:** o teste verifica apenas a presença de
`<0028>`/`<0029>` no CMap (ToUnicode) — não verifica crescimento de variante MATH
(isso está na Parte A).

---

## Parte C — Reconciliação retroativa dos commits sem relatório

Dois relatórios escritos em `00_nucleo/diagnosticos/`:

- `typst-passo-ffabfd837-reconciliacao.md` — `delim:` em `mat()`/`vec()`
  - Violação documentada com honestidade (L1 antes de L0)
  - L0 existe agora (`eval.md §P914`), hash sincronizado (`crystalline-lint → 0 V5`)

- `typst-passo-5847e1ea0-reconciliacao.md` — `axis_height` + `STRETCHY_BASES`
  - Continuação directa do Achado A de P911/P912
  - L0 de `stretchy.md` actualizado para hash `ac81e392`

---

## Parte D — Revisão cética de attach.rs (P914) e delimitadores (P912/P913)

### Revisão de `attach.rs`

Leitura linha-a-linha do diff completo. Pontos verificados:

1. **`compute_script_shifts`**: cálculo de `shift_up`/`shift_down` via `.max()` encadeado — correcto. O ajuste simultâneo (loop sobre pares tl/bl e tr/br) garante gap mínimo usando `clamp`.
2. **Kerning de duas alturas**: `summed_at(h_top_pt).max(summed_at(h_bot_pt))` — toma o máximo das duas alturas de conexão. Correcto para minimizar colisão.
3. **Caso `is_limits`**: centração de sup/sub acima/abaixo da base com `upper_gap_min`/`lower_gap_min`. Não há interacção com `compute_script_shifts` aqui — caminho separado e correcto.

**Nenhum achado novo.** A lógica está correcta para os casos que o `StubHorizontalMetrics` pode exercitar.

### Testes de revisão adicionados

Dois testes novos em `p906_tests`:

- `p916_d_gap_min_sub_sup_simultaneos_altura_total_cresce` — base `"p"`, sub+sup simultâneos: a altura total deve crescer vs. base sozinha. ✅ Passa.
- `p916_d_delimitado_conteudo_frac_tem_dimensoes_validas` — frac dentro de delimitado: ascent > 0, descent ≥ 0, width > 0. ✅ Passa.

---

## Ponto 4 — Protocolo de dois agentes

Não há evidência de protocolo de dois agentes nos relatórios de P912, P913 ou P914.
A revisão cética da Parte D substitui funcionalmente o que esse protocolo garantia
(leitura independente do diff, caso composto não coberto). Registo feito; o protocolo
não pode ser aplicado retroactivamente.

---

## Resultado final

| Item | Estado |
|------|--------|
| Tabela mutool pós-correcção | ✅ Feita — achado novo identificado (glyph fixo em c1–c4) |
| Hermeticidade `pdf_tounicode` | ✅ Corrigido — fixture `NewCMMath-Book.otf` pinada |
| Relatório retroativo `ffabfd837` | ✅ Escrito com marca honesta de violação |
| Relatório retroativo `5847e1ea0` | ✅ Escrito |
| Revisão cética attach.rs + testes | ✅ 2 novos testes, nenhum achado novo |
| Suíte completa verde | Aguarda confirmação final |

### Achado aberto de P916 (para passo futuro)

Os delimitadores `(` e `)` nos casos simples (frac 2–4 andares, mat 2×2) ainda usam o
glyph base (ID 1/2) sem crescer. O bug de `covering()` foi corrigido, mas
`vertical_glyph_variants` pode ainda estar a devolver vazio para estes casos com a fonte
real. Investigar em passo dedicado.

### Suíte de testes — contagem final P916

| Crate | Passed |
|-------|--------|
| typst-core | Ver cargo test --workspace |
| typst-infra | Ver cargo test --workspace |
| typst-shell | Ver cargo test --workspace |
