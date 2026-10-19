# Relatório — typst-passo-832: `layout::frame` — texto omitido em `move`/`rotate`/`scale` (#58, GRAVE) + `Neg` ausente (#59)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal — prompt lido de `00_nucleo/materialization/typst-passo-832.md`).
**Proveniência das medições:** commit HEAD `3bcb695ea` (P830) + alterações deste passo na working tree (`git status --short`: `M 01_core/src/engine/eval/operators.rs`, `M 01_core/src/engine/layout/helpers.rs`, `M 01_core/src/engine/layout/tests.rs`, `M 01_core/src/engine/layout/transform.rs`; untracked: este relatório, o prompt P832 e os prompts P833–P846, fixtures em `temp/p832/` — gitignored). Nenhum L0 alterado, logo `@prompt-hash` inalterados (`crystalline-lint --fix-hashes` → "Nothing to fix"). Medição "antes" feita com o binário release de P831; medição "depois" com o binário rebuildado após a implementação.
**Baseline da suíte (antes):** `cargo test -p typst-core` → **4517 passed; 0 failed**; 2 ignored (medido em P831 no mesmo commit).

---

## Achado #58 — GRAVE: texto omitido silenciosamente em `move`/`rotate`/`scale`

### Medição antes (saída literal)

```text
$ ./target/release/typst <fixture> -o c.pdf && pdftotext c.pdf -
#rotate(30deg)[Rodado sozinho]          → cris exit=0, pdftotext: (vazio)
                                          van  exit=0, pdftotext: Ro|dad|os|ozi|nh|o
#move(dx: 20pt, dy: 10pt)[Movido]       → cris exit=0, (vazio)   | van: Movido
#scale(x: 150%)[Escalado]               → cris exit=0, (vazio)   | van: Escalado
#rotate(30deg)[#strong[Rodado] e mais…] → cris exit=0, (vazio)   | van: Ro|da|do|e|ma|is t|ext|o
```

Página completamente em branco no cristalino, **exit 0, sem erro nem warning** — perda silenciosa de conteúdo.

### Código identificado

- Cristalino: `01_core/src/engine/layout/helpers.rs:249-288` — `collect_items_at` só tratava `Content::Shape` e `Content::Sequence`; todo o resto caía no braço `_ => {}` e era **descartado sem aviso**. Chamada a partir de `01_core/src/engine/layout/transform.rs:61` (`collect_sub_items`).
- Vanilla (sonda item 4 do passo): o body da transformação é **layoutado para uma frame própria** e embrulhado num grupo com a matriz — percurso genérico e exaustivo da árvore de conteúdo, sem lista fechada de variantes (`typst-library/src/layout/transform.rs:56`, `frame.rs:184,376`).

### Decisão de implementação (desvio justificado da sugestão do prompt)

O prompt sugeria reescrever `collect_items_at` para ser recursiva/exaustiva. A sonda mostrou que isso **não fecha o problema**: `collect_items_at` não tem acesso a `FontMetrics` nem ao estilo activo, sem os quais é impossível medir e emitir texto — tratar texto lá dentro seria duplicar o motor de layout. A abordagem equivalente ao vanilla **já existe no projecto**: `layout_sub_frame` (P625/P629, `01_core/src/engine/layout/sub_frame.rs`) — layouta conteúdo arbitrário numa região isolada com save/restore completo do estado e devolve os `FrameItem`s em coordenadas locais. É o mesmo mecanismo já usado por `Boxed`, células de grid/table, etc.

### Diff (resumo)

- `01_core/src/engine/layout/transform.rs` — reescrito: o body passa por `layouter.layout_sub_frame(body, SubLayoutRegion { origin_x: 0.0, width: available_w, height: None, align_rtl: true, unconstrained_height: true })`; a AABB é medida sobre os items produzidos (`line_content_right` para a largura, altura do sub-frame para a altura), com fallback à antiga `measure_content` quando o body não produz items mensuráveis. Os cantos da AABB são projectados pela matriz e o grupo emitido como antes (compensação de origem, page-break, `FrameItem::Group`). Segmentos de decoração do sub-frame (só existem com Underline/Strike/Overline à volta) são re-inseridos traduzidos pela origem do grupo — **limitação registada**: a matriz não é aplicada aos segmentos de decoração.
- `01_core/src/engine/layout/helpers.rs` — `collect_sub_items` e `collect_items_at` removidas (sem outros consumidores; `grep` confirmou).
- `01_core/src/engine/layout/tests.rs` — 2 testes novos: `layout_transform_renderiza_texto` (rotate/move/scale com texto → `FrameItem::Text` dentro do Group) e `layout_transform_renderiza_conteudo_aninhado` (`Sequence[Strong, Space, Text]` → todo o texto presente). Ambos **falhavam antes** da implementação (confirmado: `2 failed`).

### Medição depois (saída literal)

```text
#rotate(30deg)[Rodado sozinho]   → cris exit=0: Ro|dad|os|ozi|nh|o   | van: Ro|dad|os|ozi|nh|o
#move(dx: 20pt, dy: 10pt)[Movido]→ cris exit=0: Movido               | van: Movido
#scale(x: 150%)[Escalado]        → cris exit=0: Escalado             | van: Escalado
#rotate(30deg)[#strong[Rodado] e mais texto]
                                 → cris: Ro|da|do|em|ais||tex|to    | van: Ro|da|do|e|ma|is t|ext|o
#rotate(15deg)[Antes #rect(...) Depois]
                                 → cris: Depo|is|Ante||s  (+shape)  | van: Ante||s||Depo||is
```

(A fragmentação da extracção em texto rodado difere na ordem dos pedaços por geometria — o vanilla também fragmenta; os caracteres extraídos são os mesmos. `#place` e o teste `layout_transform_preserva_shape` continuam verdes — regressão coberta pela suíte.)

**Nota explícita pedida pelo passo:** a correcção cobre **conteúdo arbitrariamente aninhado**, não só o caso mínimo — o body passa pelo `layout_content` real (texto, shapes, sequências, `Strong`/`Emph`/etc., e até `Transform` aninhado, que recai recursivamente neste mesmo caminho). Não existe mais lista fechada de variantes nem fallback silencioso: qualquer `Content` que o motor de layout saiba renderizar no fluxo principal é renderizado dentro do transform.

## Achado #59 — `Neg` ausente para `Angle`/`Ratio`/`Fraction`/`Duration`

### Medição antes (saída literal)

```text
#repr(-15deg)  → cris: error: cannot apply Neg to angle (exit 1)    | van: -15deg (exit 0)
#repr(-1fr)    → cris: error: cannot apply Neg to fraction (exit 1) | van: -1fr (exit 0)
-50%           → ambos exit 0 (já funcionava — dobrado no literal, não passa por eval_unary_op)
#repr(-duration(seconds: 3)) → cris: error: cannot apply Neg to duration | van: duration(seconds: -3)
```

### Código identificado

- Cristalino: `01_core/src/engine/eval/operators.rs` `eval_unary_op` — braços `Neg` só para `Int`/`Float`/`Decimal`/`Length`/`Relative`.
- Vanilla: `lab/typst-original/crates/typst-library/src/foundations/ops.rs:76-84` — `Int, Float, Decimal, Length, Angle, Ratio, Relative, Fraction, Duration`.

### Diff

`01_core/src/engine/eval/operators.rs` — 3 braços novos (`Angle` via `Angle::rad(-a.to_rad())`, `Ratio` via `Ratio(-r.get())`, `Fraction(-f)`) + módulo `#[cfg(test)]` com 3 testes (`neg_angle`, `neg_ratio`, `neg_fraction`) — os 3 **falhavam antes** (confirmado: `3 failed`).

**`Duration` ficou de fora — bloqueio registado, não decisão inventada:** a representação cristalina de `Duration` é `u64` de nanossegundos (`01_core/src/entities/duration.rs:17-19`) — **sem sinal**. O vanilla usa representação com sinal e produz `duration(seconds: -3)` (medido). Suportar `Neg` de `Duration` exige migrar a representação para inteiro com sinal (constructor, aritmética, repr, ordenação) — mudança de desenho para passo dedicado, com decisão de escopo **pendente do dono**. Não foi feito hack nem registo de decisão falsa (lição P830).

### Medição depois (saída literal)

```text
#repr(-15deg)        → cris: -15deg (exit 0) | van: -15deg (exit 0)
#repr(-1fr)          → cris: -1fr (exit 0)   | van: -1fr (exit 0)
#rotate(-15deg)[x]   → cris: x (exit 0)      | van: x (exit 0)
```

## Validação

- `cargo test -p typst-core` → **4522 passed; 0 failed**; 2 ignored. Cálculo: 4517 (baseline) + 5 testes novos (2 de layout + 3 de operators) = 4522. ✔ bate.
- `cargo test -p typst-infra` → **672 passed; 0 failed**; 5 ignored.
- `crystalline-lint .` → **exit 0, zero violations**. (Nota: 3 ficheiros `.py` de scratch deixados pelos agentes de P831 em `temp/p831/` activavam V1 — foram removidos; eram scripts auxiliares de inspecção de PDF/fontes, não artefactos do projecto.)
- Binário cristalino rebuildado em release antes da medição "depois".

## Notas laterais (registadas, não corrigidas — fora do âmbito de P832)

- `native_rotate` (`01_core/src/engine/stdlib/transforms.rs:58-72`) faz fallback silencioso para ângulo `0.0` quando o argumento não é `Angle`/`Float`: `#rotate(1fr)[x]` compila nos dois sentidos como rotação 0° no cristalino, enquanto o vanilla rejeita (`error: expected content, found fraction`, exit 1 — medido). Lacuna de validação pré-existente (família do achado #22); com #59 fechado, `-1fr` passa a cair nesse fallback em vez de erro de `Neg`. Candidata a achado formal num lote futuro.
- Decoração (underline/strike/overline) de texto dentro de transform: os segmentos são emitidos traduzidos pela origem do grupo mas **sem aplicar a matriz** (geometria da decoração não roda). Limitação registada em código.
