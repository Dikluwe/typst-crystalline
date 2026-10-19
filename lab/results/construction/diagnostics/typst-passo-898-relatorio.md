# Relatório — Passo 898: `Content::Align`/`Content::Place` no eixo vertical, sob `height: auto`

**Data:** 2026-07-24
**Commit de partida:** `7ee509a197db6b13a6872d26d0637e1cb25be69c` (P897, 2026-07-24 15:17:30 -0300)

---

## Protocolo de dois agentes — como foi executado

Este passo pediu explicitamente um protocolo de TDD em dois agentes separados (secção "Protocolo de
TDD em dois agentes" da materialização), em vez da disciplina de reversão-temporária usada em
P890-897. Executado assim:

- **Agente A** (subagente `general-purpose`, sessão isolada, sem contexto desta conversa além do
  briefing dado): recebeu verbatim a secção "O bug" e "Fase A" da materialização, mais instruções
  explícitas para **não ler** o corpo das funções de correcção horizontal já existentes
  (`apply_pending_equation_fixups`, `apply_pending_align_fixups`) — só sabia que "um mecanismo de
  correcção diferida existe". Investigou (Fase A) e escreveu 2 testes em `tests.rs`, confirmou
  vermelho, e parou sem implementar nada de produção.
- **Agente B** (subagente `general-purpose` separado, também sem contexto desta conversa): recebeu
  os testes já escritos pelo Agente A (sem poder editá-los) mais os factos da Fase A relayados
  (formula, infinito-vs-NaN, consumidores classificados, achado extra da corrupção de `cursor_y`) e
  a localização do mecanismo horizontal já existente para reaproveitar. Implementou até os 2 testes
  passarem, sem tocar em `tests.rs`.
- **Eu (orquestrador)**: revi o trabalho de ambos antes de fechar o passo — encontrei 2 problemas
  não triviais (detalhados abaixo) que nenhum dos dois agentes tinha detectado, corrigi-os, e
  completei o resto do fluxo (L0, lint, benchmark, relatório).

**Nota de transparência do próprio Agente A** (incluída no relatório dele, preservada aqui): ao ler
`resolve_alignment` (permitido explicitamente), o bloco de leitura acabou por incluir também, por
estarem adjacentes no ficheiro, os corpos completos de `apply_pending_equation_fixups`/
`apply_pending_align_fixups` — um erro de âmbito da leitura, não intencional. O Agente A relatou isto
espontaneamente e mitigou desenhando os testes só a partir de invariantes de comportamento observável
(posições medidas via `FixedMetrics`), não replicando nenhum detalhe do mecanismo. Registado aqui
para quem avaliar a independência dos testes saber disto.

---

## Fase A — factos confirmados pelo Agente A

1. **Fórmula e sintoma**: `resolve_alignment` (`mod.rs`, ~linha 820-824) —
   `VAlign::Horizon => origin_y + (available_h - content_h) / 2.0`,
   `VAlign::Bottom => origin_y + (available_h - content_h)`. Com `available_h = f64::INFINITY` e
   `content_h` finito, o resultado é sempre `f64::INFINITY`, **nunca `NaN`** (confirmado por leitura
   e empiricamente).
2. **`Content::Equation` NÃO é vulnerável** no eixo vertical — `equation.rs` nunca chama
   `available_height()`/`page_bottom_limit()` (grep confirma 0 ocorrências); posicionamento vertical
   de equação é puramente sequencial via `cursor_y`.
3. **Consumidores classificados** (grep exaustivo em `01_core/src/engine/layout/`):
   - Seguros (só decidem quebra de página, infinito = "nunca quebrar", fallback correcto):
     `cursor.rs:385`, `grid.rs:448`, `grid.rs:574`, `sequence.rs:69-70`, `block.rs:109-111`,
     `placement.rs:80`.
   - Vulneráveis, corrigidos neste passo: `layout_align` (`remaining_h` passado a
     `resolve_alignment`), `layout_align` (`cursor_y = Pt(page_bottom_limit())` no ramo
     Horizon/Bottom — corrompe também `page.height` final via `compute_page_height()`, não só a
     posição de um item), `layout_place` (`avail_h_page` nos ramos sem célula activa).
   - Vulnerável, **fora de âmbito**: `grid.rs:460` (distribuição de `fr` entre linhas de grid sob
     `height: auto` — diverge do vanilla, onde `fr` numa página `auto` degenera a 0). Não coberto
     pelos testes, não corrigido.

---

## Fase B — implementação do Agente B (desenho original)

Mecanismo simétrico a P897: campo `pending_align_v_centering`, método
`apply_pending_align_v_fixups` (reaproveita `resolve_alignment`), `helpers::shift_frame_item_y`
novo. Guarda adicional em `layout_align` para a corrupção de `cursor_y`: o ramo
`(sem célula, Horizon|Bottom)` só usa `page_bottom_limit()` quando finito, caindo no ramo `_`
(`target_y + sub_h`, já finito) caso contrário.

Testes do Agente A (`p898_align_bottom_sob_height_auto_nao_produz_infinito`,
`p898_align_horizon_apos_bloco_alto_alinha_contra_altura_final_da_pagina` — renomeados de `p873_`
para `p898_` na revisão, o Agente A rotulou-os incorrectamente com o número de um passo antigo e não
relacionado) confirmados verdes pelo Agente B. Suíte completa reportada verde por ele: 4726+734+41,
0 falhas.

## Revisão — 2 problemas encontrados e corrigidos antes de fechar o passo

**Trust but verify**: o relatório do Agente B descrevia a implementação como completa e correcta.
Antes de aceitar isso, revi o diff directamente (não só o relatório) e testei um caminho que os 2
testes do Agente A não cobriam (`Content::Place`, que tem `dx`/`dy` — os testes de A só usam
`Content::Align`, que não tem).

### Achado 1 (falso positivo, não era bug real)

O diagnóstico automático assinalava `shift_frame_item_y` como `dead_code`. Investigação directa do
diff mostrou que `apply_pending_align_v_fixups` chama `helpers::shift_frame_item_y` correctamente —
o warning era de um estado intermédio (stale), não reproduzido num `cargo build` limpo. Sem acção
necessária.

### Achado 2 (bug real, confirmado e corrigido)

A implementação original do Agente B gravava `origin_y + dy` no campo `origin_y` do tuplo pendente
(mesmo truque de linearidade que P897 usa para `origin_x + dx` no eixo X). **Isto é seguro no eixo
X mas não no eixo Y**: `apply_pending_align_fixups` (X) usa `final_avail_w = page_width - 2*margin`,
uma **constante** da página inteira, independente de `origin_x` — por isso somar `dx` a `origin_x`
antes de gravar não contamina nada. `apply_pending_align_v_fixups` (Y), por design do próprio Agente
B (necessário, já que a altura restante depende de onde no fluxo o item estava), usa
`final_avail_h = page_height - margin - origin_y`, que **depende** de `origin_y`. Gravar
`origin_y + dy` faz `dy` entrar nesse cálculo, produzindo:
- `Bottom`: `dy` completamente descartado (o termo `+dy` cancela algebricamente com o `-dy` implícito
  em `final_avail_h - content_h`).
- `Horizon`: `dy` aplicado a metade do valor correcto.

**Confirmado por teste exploratório antes de corrigir** (não só por álgebra): `Content::Place(bottom,
dy: 30pt)` sob `height: auto`, com um bloco alto antes para dar corpo à página — posição Y observada
saiu **idêntica** à posição sem `dy` nenhum (erro de exactamente 30pt, o valor de `dy` inteiro
perdido).

**Correcção**: separar `origin_y` puro (usado para `final_avail_h`) de `dy` (só somado ao resultado
**depois** de `resolve_alignment` já ter usado o `origin_y` puro) em campos distintos do tuplo —
`pending_align_v_centering` passou de 6 para 7 campos:
`(start_idx, count, align, content_h, origin_y puro, dy, applied_y)`. `layout_align` (que não tem
`dy`) grava `0.0` nesse campo.

Teste de regressão permanente adicionado (não fazia parte dos 2 do Agente A):
`p898_place_bottom_com_dy_sob_height_auto_aplica_dy_correctamente`
(`01_core/src/engine/layout/tests.rs`). Vermelho confirmado antes da correcção (reintroduzi
temporariamente o bug combinando `origin_y+dy` de novo, corri o teste, falhou com o mesmo sintoma;
restaurei a correcção, teste passou).

### Achado 3 (comentário desactualizado, cosmético)

Uma nota de código no `layout_place` original do Agente B ainda descrevia o truque `origin_y + dy`
como se fosse a abordagem final, imediatamente antes da minha correcção — teria ficado contraditório
com o comentário novo logo a seguir. Removido o comentário obsoleto.

---

## Suíte completa (após as correcções da revisão)

```
typst-core:    4727 passed; 0 failed; 2 ignored   (+3 vs baseline P897: 2 do Agente A, 1 meu)
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

## `crystalline-lint`

```
crystalline-lint --fix-hashes .
Fixed 13 files (mesmo L0 prompts/engine/layout.md) → hash 9c17da8a
Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
warning: Prompt órfão: 'prompts/infra/package_version_resolution.md' [V7]  ← pré-existente, não relacionado
```

## Recompilação do `.typ` de 30 secções

Hash confirmado igual a P896/P897: `9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`.
O ficheiro usa `height: auto` (linha 6) mas **não** contém `#align(horizon)`/`#align(bottom)`/
`#place(`. Recompilado mesmo assim para confirmar ausência de regressão no caminho geral:

```
exit=0, MediaBox: [0 0 478.23 5624.71] — idêntico a P896/897
Sequência de numeração (1)...(44) — completa, idêntica
```

**Confirmação visual dedicada** (o ficheiro de 30 secções não exercita o caminho deste passo):
ficheiro `.typ` mínimo com `height: auto` + bloco alto + `#align(horizon)` + `#place(bottom, dy:
0.3cm)`. Renderizado sem crash, `MediaBox` finito, posições Y dentro dos limites da página
(`pdftotext -bbox`: `rodape` yMax=152.94pt, dentro do limite de conteúdo 156.1pt = altura da página
184.46pt menos margem). Confirmado visualmente por PNG: texto legível, sem sobreposição com
infinito/off-page.

**Achado incidental durante o teste visual, não relacionado com este passo**: um teste inicial com
`#place(bottom + right, dy: 1cm)[canto inferior direito, deslocado 1cm]` mostrou o texto quase todo
fora da página horizontalmente. Investigado e confirmado **pré-existente, já documentado em P772j**:
`measure_content` (`helpers.rs`) devolve `content_w = 0.0` para `Content::Place` com corpo de texto
simples (só tem braços para `Content::Shape`/`Content::Sequence`; P772j corrigiu isto só para
`Content::Align`, deixando `Content::Place`/`Content::Transform` explicitamente fora de âmbito nesse
passo). Confirmado não relacionado com P898: a linha `measure_content(body, avail_w_page)` em
`layout_place` não foi tocada nem por mim nem pelo Agente B. Registado aqui, não corrigido (fora de
âmbito deste passo).

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`:

| Cenário | P897 (baseline) | P898 |
|---|---|---|
| 01-hello | 94.9 ms | 95.6 ms |
| 02-lorem | 118.2 ms | 118.6 ms |
| 03-images | 102.4 ms | 103.9 ms |
| 04-math | 158.4 ms | 156.7 ms |
| 05-tables | 116.6 ms | 115.7 ms |
| 06-long | 374.0 ms | 378.0 ms |
| 07-context | 136.7 ms | 137.0 ms |

Todas as leituras dentro de ruído normal (~1-2%), sem a inflação da 1ª leitura observada em P896/P897
— não foi preciso remedição isolada desta vez.

## Fora de âmbito (registado, não corrigido/investigado)

1. `grid.rs:460` — unidades `fr` em linhas de grid sob `height: auto` (achado do Agente A).
2. `Content::Place` aninhado em sub-frame (`in_sub_frame`) usa `origin_y = 0.0` local; a correcção
   diferida resolve contra a altura final da página **raiz** — limitação pré-existente idêntica já
   presente no eixo X de P897 para o mesmo caso, não expandida aqui (Agente B).
3. `measure_content` devolve `content_w = 0.0` para `Content::Place`/texto simples — pré-existente,
   P772j, confirmado não relacionado com este passo (achado incidental da confirmação visual).
4. P893 (`FontMetrics::math_constants`) continua parado em Fase A, sem conflito com este passo.

## Resultado

- `Content::Align`/`Content::Place` no eixo vertical já não produzem posição infinita nem corrompem
  `page.height` sob `height: auto`; `VAlign::Horizon`/`VAlign::Bottom` alinham correctamente contra a
  altura final real da página.
- `Content::Equation` confirmado não vulnerável no eixo vertical — nada a corrigir aí.
- Protocolo de dois agentes seguido e documentado; revisão do orquestrador encontrou e corrigiu um
  bug de implementação real (`dy` descartado/meio-aplicado em `Content::Place`) não coberto pelos
  testes originais do Agente A — teste de regressão dedicado adicionado.
- Suíte completa verde (4727+734+41, 0 falhas). `crystalline-lint`: 0 drift.
- `.typ` de 30 secções recompila sem regressão (hash igual a P896/897); confirmação visual dedicada
  para o caminho vertical (o ficheiro de 30 secções não o exercita).
- Benchmark: sem regressão, leituras estáveis desta vez (sem necessidade de remedição isolada).
