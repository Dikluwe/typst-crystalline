# Relatório — Passo 908: correcção estrutural completa do achado de P904 (Item 2)

**Data:** 2026-07-25
**Commit de partida:** `0de933aaf` (P907)

---

## Resumo executivo

P904 diagnosticou mas não corrigiu: `Content::Align`/`Content::Place` aninhado dentro de
`layout_sub_frame` (sob `width`/`height: auto`) gravava uma correcção diferida (`pending_align_
centering`/`pending_align_v_centering`) indexada à lista **local** do sub-frame — inválida assim
que os items fossem copiados para o pai. P904 mitigou (descartar a entrada, nunca corromper um
item não relacionado) sem corrigir a posição. P908 fecha essa lacuna com um mecanismo geral
baseado em `path: Vec<usize>` (substitui `start_idx: usize`), aplicado aos 9 contratos de
composição confirmados por leitura directa do código, incluindo um achado adicional descoberto
durante a implementação: um bug de sentinel pré-existente, mais grave que o âmbito original de
P908, que afectava páginas de tamanho **fixo**, não só `auto`.

## Fase A — mapeamento completo (documentado no L0 antes da Fase B, protocolo de nucleação)

9 contratos de composição de `layout_sub_frame` mapeados por leitura directa (não inferidos),
agrupados em 4 famílias — ver `00_nucleo/prompts/engine/layout.md` §P908 para a tabela completa e
a ordem de flush confirmada (`flush_pending_floats` → `flush_pending_footnote_bodies` →
`apply_pending_align_(v_)fixups`, sempre nesta ordem, em `new_page()`/`finish()`; `flush_pending_
cell_tails` corre já no contexto da página seguinte, antes do fixup **dessa** página).

Dois achados de escopo, ambos levados ao dono antes de prosseguir (`AskUserQuestion`):

1. **`start_idx: usize` não chega** — dois contratos (`transform.rs`, `grid.rs` overflow em linha
   `Fixed`) envolvem os items num `FrameItem::Group` novo antes de os inserir no pai — o alvo
   deixa de ser indexável directamente. Generalização para `path: Vec<usize>` (descida por
   `Group.items`/`Link.items`, último elemento = índice efectivo).
2. **`grid.rs` tem um terceiro mecanismo de diferimento** (`pending_cell_tails`, cross-página, além
   de floats e footnotes) — eleva o número real de contratos de 7 para 9. Decisão: cobrir os 9,
   incluindo o caso cross-página.

## Achado adicional (descoberto durante a Fase B, não planeado) — bug de sentinel pré-existente

Ao escrever o primeiro teste black-box (`Place` aninhado em `Align`, controlo com página de altura
**fixa**), a corrida de controlo devolveu `y≈1_000_000_195`, não uma posição razoável. Diagnóstico:
`layout_place` chamava `self.available_height()`, que lê `self.regions.current.height` — e
`layout_sub_frame` substitui esse campo por um sentinel interno (`1_000_000_000.0`, **finito**)
sempre que a região é `height: None` (Align/Place/Transform/grid-cell, todos `unconstrained_
height: true`). Sendo finito, o desvio de P897/898 ("se infinito, diferir") nunca disparava —
`Place` `Bottom`/`Horizon` aninhado dentro de **qualquer** sub-frame não-restringido resolvia de
imediato contra ~1 mil milhões de pt, **mesmo em página de tamanho fixo** (não só `width`/`height:
auto`) — 13 call sites de `available_width()`/`available_height()` expostos. `layout_align` já
tinha uma guarda (`else if self.is_height_unconstrained`, antes de chamar `page_bottom_limit()`);
`layout_place` não tinha.

Confirmado com o dono (`AskUserQuestion`) que este achado é um **pré-requisito** de P908 (o
mecanismo `path`-based fica sobre uma base correcta) — corrigido primeiro, dentro do mesmo passo:

```rust
let avail_h_page = if self.is_height_unconstrained {
    f64::INFINITY
} else {
    self.available_height()
};
```

`available_width()`/`available_height()` em si **não foram alteradas** — usadas legitimamente por
outros call sites (`block.rs`, `curve.rs`, `image.rs`, etc.) para decisões de overflow interno,
onde o sentinel representa correctamente "sem limite prático" — só `layout_place` precisava da
guarda adicional.

## Fase B — implementação

### Mecanismo central

- `pending_align_centering`/`pending_align_v_centering`: `start_idx: usize` → `path: Vec<usize>`
  (aliases `PendingAlignXEntry`/`PendingAlignYEntry`, `mod.rs`).
- `layout_sub_frame`: `truncate` → `split_off` — devolve as entradas órfãs (2 vectores novos no
  retorno) em vez de as descartar.
- `helpers::resolve_path_slice`: descida recursiva por `path` através de `Group.items`/
  `Link.items`, generaliza `apply_pending_align_fixups`/`_v_fixups`.
- `DeferredFloat`/`DeferredCellTail`: 2 campos novos cada (`orphaned_align_x`/`_y`), threading
  através dos dois mecanismos de diferimento existentes.

### Regra de rebase — duas famílias, tratamento assimétrico (achado empírico)

- **Merge imediato** (`layout_align`, `layout_place`, `emit_deferred_float`, `grid.rs` sem
  overflow, `flush_pending_cell_tails`, `flush_pending_footnote_bodies`): `origin`/`applied`
  recebem o **mesmo** delta aditivo já aplicado aos `FrameItem`s reais — mas nunca a variante que
  subtrai um ascender (`- sub_origin_y`/`- y_offset`/`- ascender.0`). Confirmado por teste que usar
  o delta dos items (que subtrai ascender) produz um resíduo de exactamente 1 ascender nas
  quantidades lógicas.
- **Envolvimento em `Group`** (`transform.rs`, `grid.rs` overflow-`Fixed`): dois sub-casos
  distintos, descobertos só durante a implementação:
  - `grid.rs` (clip-`Group`, `matrix: identity`): items já vêm **absolutos** antes de entrarem no
    `Group` — só `path.insert(0, group_idx)`, sem delta adicional.
  - `transform.rs` (items **verdadeiramente locais**, nunca compostos): `origin_y`/`applied_y`
    precisam de tratamento **assimétrico** — `origin_y` só leva `+ pos.y` (somar o ascender aqui
    empurra-o para além de `page_height - margin`, disparando o clamp `max(0.0, …)` de
    `apply_pending_align_v_fixups` e quebrando o cancelamento algébrico do termo `origin_y` que as
    fórmulas Bottom/Horizon dependem); `applied_y` leva `+ pos.y + ascender_local` (tem de bater
    com a posição física real do item, que nunca teve o ascender cancelado por uma composição
    `iy - sub_origin_y`, ao contrário de `layout_align`/`layout_place`). Eixo X sem esta assimetria
    em nenhum caso.

### `grid.rs` — família "diferido-cruzado" (`pending_cell_tails`)

Entradas cujo intervalo `[path[0], path[0]+count)` cai inteiramente em `head` ou em `tail`
(decisão replicada de `slice_frame_items_at_height`, calculada por índice original antes de
consumir a lista) são rebaseadas e roteadas para o destino correcto; entradas cujo intervalo
atravessa a fronteira são descartadas — degradação graciosa, caso raro (exige overflow de linha
`Auto`/`Fraction` **e** um `Align`/`Place` aninhado cujos items fiquem divididos pelo próprio
corte), documentado no campo `DeferredCellTail::orphaned_align_x/_y`, não silencioso.

## Testes

4 testes novos, TDD (vermelho confirmado antes de cada correcção):

- `p908_place_aninhado_em_align_sob_height_auto_posicao_bate_com_formula` — aninhamento simples,
  família merge-imediato.
- `p908_place_aninhado_em_align_duplo_sob_height_auto_posicao_bate_com_formula` — aninhamento
  duplo (`path` com comprimento > 1), pedido explícito da materialização.
- `p908_place_aninhado_em_transform_sob_height_auto_posicao_bate_com_formula` — família
  envolvimento-em-`Group`, posição absoluta (`Group.pos` + local).
- `p908_layout_sub_frame_devolve_entradas_orfas_em_vez_de_descartar` — teste de mecanismo directo,
  complementar ao teste de mitigação de P904 (mantido intacto, continua válido — agora vacuously
  true por uma razão diferente: `layout_sub_frame` já não descarta, devolve).

**Nota metodológica registada** (evita reproduzir o erro): a primeira versão dos 3 primeiros
testes comparava a corrida `height: auto` contra uma corrida de controlo com `height` explícito —
mas ambas atravessam o mesmo caminho `is_height_unconstrained` (reflecte estar dentro de um
sub-frame não-restringido, independente da página em si ser `auto` ou fixa), pelo que um mecanismo
incompleto produzia a MESMA resposta errada nas duas corridas — falso positivo. Reescritos para
usar a fórmula já validada por `p898_place_bottom_com_dy_sob_height_auto_aplica_dy_correctamente`
(`page.height - margin - content_h + dy`) como verdade-terreno independente.

Casos representativos de `Deferred*` (diferido-plano/float, diferido-cruzado/grid-tail) **não**
têm teste dedicado — cobertos pela mesma regra de rebase already exercised nos outros contratos e
pela suíte de regressão completa (P245/P251/P304 inalterados), registados como scope-out de
cobertura directa neste passo, candidatos a passo dedicado se algum caso real os expuser.

## Suíte completa

```
typst_core:  4774 passed; 0 failed; 3 ignored (+4 novos: 3 P908 + 1 mecanismo)
typst_shell:  734 passed; 0 failed; 5 ignored
typst_infra:   41 passed; 0 failed; 0 ignored
```

Zero regressões — P896/P897/P898/P904 (casos não aninhados) continuam a passar sem alteração.

## `crystalline-lint`

`--fix-hashes .`: 13 ficheiros (mesmo L0 `layout.md`). 0 drift, só o warning V7 pré-existente e não
relacionado (`infra/package_version_resolution.md`).

## `.typ` de 30 secções

Hash confirmado igual aos passos anteriores
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila sem regressão
(`exit=0`, `(1)`...`(44)` completo) — não exercita directamente os caminhos aninhados de P908, mas
confirma ausência de regressão no caminho comum.

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Todas as leituras dentro da baseline estabelecida em
P907 (mesma sessão): `01-hello` 89.8ms, `02-lorem` 113.1ms, `03-images` 98.7ms, `04-math` 150.1ms,
`05-tables` 110.8ms, `06-long` 364.8ms, `07-context` 130.4ms — sem regressão atribuível a P908 (o
mecanismo só acrescenta trabalho quando Align/Place/Transform/grid-cell/float/footnote é aninhado
sob `width`/`height: auto`, caminho não exercitado pelos 7 cenários de benchmark).

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| Mapeamento dos 9 contratos de composição | Confirmado por leitura directa, documentado no L0 antes da Fase B | ✅ |
| `path: Vec<usize>` (substitui `start_idx`) | Implementado, `resolve_path_slice` generaliza os fixups | ✅ |
| `layout_sub_frame` devolve em vez de descartar | Implementado, testado directamente | ✅ |
| Bug de sentinel pré-existente em `layout_place` (`available_height()`) | Achado não planeado, confirmado com o dono, corrigido como pré-requisito | ✅ |
| Família merge-imediato (6 contratos) | Implementado, testado (2 casos + regressão completa) | ✅ |
| Família envolvimento-em-`Group` (2 contratos, tratamento assimétrico origin/applied) | Implementado, testado (1 caso + regressão completa) | ✅ |
| Família diferido-cruzado (`pending_cell_tails`, `grid.rs`) | Implementado, coberto pela regressão completa, sem teste directo dedicado | ⚠️ Registado |
| Teste de mitigação de P904 | Mantido intacto, continua válido (vacuously, razão diferente) | ✅ |
