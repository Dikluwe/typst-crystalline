# Relatório — Passo 904: três achados pequenos registados em P897/898, agrupados

**Data:** 2026-07-25
**Commit de partida:** `05c57f8d5` (P903)

---

## Item 1 — unidades `fr` em linhas de grid sob `height: auto`

### Fase A

**Achado, mais grave do que a descrição original**: confirmado por leitura de código
(`grid.rs::layout_grid`, Fase 2 de resolução de `Fraction`) que `available_below =
page_bottom_limit() - grid_top_y` usava `page_bottom_limit()` directamente — infinito sob `height:
auto` (mesmo sentinel de P867). `remaining_v`/`row_heights[fr]` ficavam `f64::INFINITY`. **Não é só
"diverge do vanilla"** — confirmado por compilação directa (`./target/release/typst`) que isto
produz uma `MediaBox` malformada (`0 0 283.46 inf`), mesma classe do crash original de P894/895
("offset_x = infinito").

Confirmado contra o vanilla real (`lab/typst-original/target/release/typst`) com `rows: (1cm, 1fr,
1cm)` sob `height: auto`: `page.height = 113.385826pt`, **exactamente** `2×margin(28.35×2) +
1cm(28.35) + 1cm(28.35) = 113.4pt` — o `1fr` contribui **zero** para a altura final.

### Fase B — Implementação

```rust
let page_bottom = self.page_bottom_limit();
let available_below = if page_bottom.is_finite() {
    f64::max(0.0, page_bottom - grid_top_y)
} else {
    total_fixed_and_auto  // faz remaining_v = 0, sem replicar a mecânica interna do vanilla
};
```

Teste: `p904_grid_fr_row_sob_height_auto_nao_produz_infinito_degenera_a_zero` (`layout()` pipeline
completo). Vermelho confirmado (`page.height` = `inf`) antes da correcção.

**Confirmação visual**: recompilado o caso mínimo — `MediaBox` cristalino agora `[0 0 283.46
113.38]`, quase idêntico ao vanilla (`[0 0 283.46457 113.385826]`, diferença só de precisão de
impressão).

---

## Item 2 — `Content::Place` aninhado em sub-frame usa `origin_y = 0.0` local

### Fase A

**Caso mínimo reproduzido**: `#box[#place(bottom, dy: 0.3cm)[nested placed]]` depois de um bloco de
4cm, sob `height: auto`. Cristalino: `page.height=170.08pt`, item "nested placed" em `y≈126pt`.
Vanilla: `page.height=183.28pt`, item em `y≈153.6pt`. **Confirmado que não é só "posição
ligeiramente errada"** — a diferença de `page.height` (13pt) e de posição (27.6pt) é substancial.

**Causa exacta, mais séria do que a descrição original**: as entradas de `pending_align_centering`/
`pending_align_v_centering` gravadas por um `Align`/`Place` aninhado têm `start_idx` relativo à
lista `current_items` **local** do sub-frame (reiniciada vazia em `layout_sub_frame`) — não à lista
do frame pai/raiz, onde `apply_pending_align_(v_)fixups` (chamado só em `finish()`/`new_page()`)
aplica os deslocamentos. Como o caller de `layout_sub_frame` tipicamente copia os items devolvidos
para o pai a um índice **diferente** de `start_idx`, a correcção diferida arrisca aplicar-se ao(s)
item(ns) **errado(s)** do pai — corrupção silenciosa de posição de conteúdo não relacionado, não só
posição errada do próprio `Place` aninhado.

**Veredicto (Fase A ponto 2)**: **exige mudança estrutural**. Corrigir correctamente exigiria
propagar a posição do sub-frame pela hierarquia e re-basear `start_idx`/`origin_x`/`origin_y` — mas
`layout_sub_frame` é consumido por múltiplos callers (`layout_align`, `layout_place`, blocos), cada
um com a sua própria convenção de tradução de coordenadas ao compor os items do sub-frame no pai
(delta incremental em `layout_align`, alvo absoluto em `layout_place` — já divergentes desde
P897). Não há um único ponto barato para re-basear.

### Fase B — mitigação de segurança (não a correcção completa)

Por instrução explícita da materialização ("se exigir mudança estrutural maior, não implementar
aqui"), a correcção completa **não foi implementada** — fica registada como achado maior, candidato
a passo dedicado.

**Implementado em vez disso**: `layout_sub_frame` regista o comprimento de
`pending_align_centering`/`pending_align_v_centering` antes de `layout_content(content)` e
**descarta** (`truncate`) qualquer entrada gravada durante essa chamada — previne a corrupção
(reverte para o fallback já aceite, sem correcção, mesma limitação pré-existente de P897/898),
nunca aplica um deslocamento ao item errado.

Teste directo do mecanismo (não geometria completa, já que a geometria correcta continua fora de
âmbito): `p904_place_aninhado_em_sub_frame_nao_deixa_pending_orfao` — constrói um `Layouter`, força
`page_config.height = INFINITY`, chama `layout_sub_frame` com um `Content::Place` dentro, confirma
que os `Vec` pendentes voltam ao comprimento anterior à chamada (antes: `1` entrada órfã; depois:
`0`).

**Confirmação visual**: recompilado o caso mínimo do achado — output idêntico antes/depois da
mitigação (a corrupção não era visível NESTE caso específico por coincidência geométrica, confirmado
pela via directa do teste unitário, não pela visual). A posição de "nested placed" continua a
divergir do vanilla (limitação pré-existente, não corrigida, comportamento inalterado por este
passo — só a segurança contra corrupção de OUTROS items foi adicionada).

---

## Item 3 — `measure_content` devolve `content_w = 0.0` para `Content::Place`/texto simples

### Fase A

Relido `paridade-producao-p772j.md`: P772j **não estendeu** `measure_content` — corrigiu
`layout_align` directamente, medindo `content_w` a partir dos `sub_items` já layoutados via
`FontMetrics::line_content_right` (mesmo mecanismo de `measure_content_real`), deixando
`measure_content` inalterado. `Content::Place`/`Content::Transform` ficaram fora de âmbito
explicitamente nesse passo — não por serem mais complexos de medir, mas porque o passo tratava
especificamente do bug de `align()` em células de grid.

Confirmado (grep exaustivo) que há **dois call sites** de `Content::Place`, ambos já chamando
`layout_sub_frame` (têm `sub_items`/`body_items` disponíveis, tornando a extensão do mecanismo de
P772j directa, não uma reimplementação):
- `placement.rs::layout_place` (ramo `float: false`).
- `place.rs::layout` (ramo `float: true`, floats).

### Fase B — Implementação

Mesmo mecanismo de P772j nos dois call sites (`origin_x` do sub-frame de `Place` é sempre `0.0`):

```rust
let sub_item_refs: Vec<&FrameItem> = sub_items.iter().collect();
let content_w = self.metrics.line_content_right(&sub_item_refs).max(0.0);
```

`Content::Transform` (`transform.rs:58`) **não foi estendido** — fora do catálogo de sintomas
confirmados neste passo, pode envolver medição não-trivial com rotação/escala (não investigado,
registado).

Teste: `p904_place_right_mede_largura_real_do_texto_nao_zero` (`layout()` pipeline completo,
`place(right)[abcde]`). Vermelho confirmado (`placed_x=180.0` — posição de largura-zero) antes da
correcção; depois, `placed_x=147.0` (esperado≈144.0, tolerância larga ~5pt por a mesma "divergência
residual" entre métodos de medição já documentada e aceite em P772j — o que importa é a ordem de
grandeza, não bytes exactos, ADR-0107).

**Confirmação visual**: reproduzido o caso exacto de P898
(`#place(bottom + right, dy: 1cm)[canto inferior direito, deslocado 1cm]`, que antes mostrava só
"canto"/"i" truncados fora da página) — texto completo agora dentro da página em todas as 5
palavras, com posições x razoáveis (90.9 a 255.0, dentro dos 283.46pt de largura da página).

---

## Suíte completa (após os 3 itens)

```
typst-core:    4743 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

## `crystalline-lint`

`--fix-hashes .`: 13 ficheiros (mesmo L0 `engine/layout.md`, actualizado com secções para os 3
itens). 0 drift, só o warning pré-existente V7 (não relacionado).

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Todas as leituras na baseline estabelecida ou
ligeiramente abaixo (`04-math` 150.1ms, `01-hello` 91.7ms). Sem regressão.

## `.typ` de 30 secções

Hash confirmado igual aos passos anteriores
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila sem regressão
(`exit=0`, `(1)`...`(44)` completo).

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| 1 — `fr` em grid sob `height: auto` | Bug real, mais grave que descrito (PDF malformado) | ✅ Corrigido |
| 2 — `Place` aninhado em sub-frame | Exige mudança estrutural (múltiplos callers, sem ponto barato de rebase) | ⚠️ Mitigação de segurança implementada; correcção completa fora de âmbito, candidata a passo dedicado |
| 3 — `measure_content` para `Place` | Extensão directa do mecanismo já validado de P772j | ✅ Corrigido (2 call sites); `Transform` fora de âmbito, registado |
