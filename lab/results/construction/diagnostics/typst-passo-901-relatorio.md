# Relatório — Passo 901: barra do radical (`sqrt`/`root`) mal posicionada

**Data:** 2026-07-25
**Commit de partida:** `5cc9b681f` (P900)

---

## Protocolo de dois agentes — como foi executado, incluindo desvio

Este passo pediu o mesmo protocolo de dois agentes de P898 (cálculo geométrico). Executado assim:

- **Agente A** (subagente `general-purpose`, sessão isolada): investigou o bug de forma
  independente, confirmou a convenção de coordenadas por leitura de código
  (`hconcat_spaced`/`layout_equation`/`layout_text_node`), mediu directamente via `mutool
  trace`/`pdftotext -bbox` num PDF real, e escreveu 2 testes com uma verificação **relacional**
  (`overline_y <= radicand_ink_top_y`, não um valor exacto hardcoded) — confirmou vermelho, não
  tocou em nenhum ficheiro de produção.
- **Agente B**: lançado com o diagnóstico completo do Agente A relayado, **falhou a meio da
  investigação inicial** (erro de limite de sessão da API, "session limit · resets 12am
  America/Sao_Paulo") — confirmado por `git diff --stat` que não tocou em nenhum ficheiro de
  produção antes de falhar (só `tests.rs`, do Agente A, estava alterado).
- **Desvio do protocolo, registado explicitamente**: em vez de tentar relançar o Agente B (o
  orquestrador já tinha, nesse ponto, uma análise própria e independente do bug — feita antes de
  despachar o Agente A, para poder escrever o briefing dele — que **coincidia** com o diagnóstico
  do Agente A em toda a causa raiz, divergindo apenas num detalhe de fórmula menor, ver secção
  seguinte), o orquestrador implementou directamente a correcção, actuando como reviewer rigoroso
  do próprio trabalho (mesmo padrão de escrutínio que P898 usou para rever o Agente B) em vez de
  aceitar cegamente. Isto reduz a independência formal do "Agente B" desta vez, mas a correcção
  ainda foi desenhada a partir de uma investigação (a do Agente A) genuinamente separada da
  implementação, e a implementação foi verificada por testes que o implementador não escreveu.

## Diagnóstico (convergente entre a análise do orquestrador e o Agente A)

### Convenção de coordenadas confirmada

Dentro de `MathBox.items` (`01_core/src/engine/math/layout/`), `y=0` é a **baseline** dessa
`MathBox`, `y` cresce para baixo — já documentado desde P800 (`_comum.md`, linha 26-28: "`layout_
equation` devolve items com posições relativas à baseline da fórmula"), mas nunca declarado
explicitamente dentro de `root.rs`. Confirmado por leitura de `hconcat_spaced` (`mod.rs:904-942`,
só desloca `x` ao juntar boxes irmãs — só correcto se todas partilharem a mesma baseline `y=0`) e
`layout_equation` (`mod.rs:339-354`, chama `place()` uma única vez com `baseline_y = box.ascent`,
o que anula o termo `-ascent` da fórmula de `place()` e faz os `y` locais sobreviverem inalterados
até à `Vec<FrameItem>` final).

### Causa exacta

`layout_root` (`root.rs`, antes da correcção) calculava `overline_y`/`rad_offset_y`/`sym_dy`
assumindo (implicitamente, nunca declarado no código, contradizendo a convenção real) "`y=0` =
topo da caixa" — com offsets **positivos** (para baixo) tanto para a overline como para o
radicando. Sob a convenção real, isto colocava a barra muito perto da baseline do radicando
(`overline_y ≈ 1.1pt`) em vez de acima do topo da tinta (`radicand_ink_top_y ≈ -8pt`, medido pelo
Agente A com `FixedMetrics`) — a barra atravessava o glifo perto da base, exactamente o sintoma
"strikethrough" catalogado em P894.

### Não depende de P893

Confirmado por leitura de código antes de implementar: o bug é de aritmética/sinal na fórmula de
posicionamento (`overline_y`/`rad_offset_y`/`sym_dy`), completamente independente de `gap`/
`line_thickness` virem de valores de fallback ou da tabela MATH real da fonte (P893). A correcção
aplica-se identicamente em qualquer dos dois casos.

## Correcção implementada

```rust
// Radicando — SEM deslocamento vertical (já está na sua própria baseline,
// que é a baseline do composto).
for item in rad_box.items {
    items.push(offset_item(item, Pt(radical_width), Pt(0.0)));
}

// Overline — acima do topo da tinta do radicando por `gap`, barra
// centrada na sua própria espessura.
let overline_y = -(rad_box.ascent + gap + line_thickness / 2.0);

// Símbolo √ — topo do glifo esticado coincide com o topo da barra.
let sym_dy = radical_box.ascent - total_ascent;

// Índice de root(n,x) — ajustado de 0.0 para -total_ascent (achado da
// revisão do orquestrador, ver abaixo).
let idx_dy = -total_ascent;
```

`total_ascent`/`total_descent`/`rad_height_pt` (magnitudes, não posições com sinal) ficaram
inalterados — já estavam correctos.

### Achado extra da revisão do orquestrador (mesmo padrão de P898)

O índice opcional de `root(n, x)` usava `idx_dy = 0.0` (comentário original: "y=0 (topo)") — sob a
convenção ANTIGA (errada), `y=0` realmente correspondia ao topo, então isto estava correcto NESSE
contexto. Ao corrigir a convenção geral para `y=0=baseline`, deixar `idx_dy=0.0` inalterado teria
introduzido uma **regressão nova**: o índice passaria a ficar posicionado na baseline do composto
em vez de no topo — visualmente errado, mas não coberto por nenhum dos 2 testes do Agente A (que só
verificam a relação overline-vs-radicando, não a posição do índice). Corrigido para `idx_dy =
-total_ascent`, preservando a intenção original ("topo") sob a convenção correcta. Confirmado
visualmente: `root(3, x)` mostra o "3" correctamente posicionado acima-esquerda do símbolo.

## Testes

`layout_root_overline_fica_acima_do_topo_do_radicando` e
`layout_root_com_indice_overline_fica_acima_do_topo_do_radicando` (Agente A) — verificação
**relacional** (`overline_y <= radicand_y - rad_box.ascent`), não valor hardcoded, robusta a
mudanças de constantes/fonte. Vermelho confirmado pelo Agente A antes da implementação
(`overline_y=1.116 > radicand_ink_top_y=-8.088`); verde confirmado depois pelo orquestrador.

Os 4 testes pré-existentes de `layout_root` (`layout_root_contem_radical_e_radicando`,
`layout_root_tem_overline`, `layout_root_overline_horizontal`, `layout_root_com_indice_contem_
indice`) — todos verificações de presença/horizontalidade, não de posição com sinal — continuam a
passar sem alteração.

## Suíte completa

```
typst-core:    4735 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

## `crystalline-lint`

`--fix-hashes .`: 1 ficheiro (`root.rs`, L0 `math/layout/root.md`, actualizado com secção `## P901`
documentando a correcção). `crystalline-lint .`: 0 drift, só o warning pré-existente V7 (não
relacionado).

## Confirmação visual

`$ sqrt(x) $`, `$ sqrt(a^2+b^2) $` (caso exacto de P894), `$ root(3, x) $` — comparados lado a lado
com o binário vanilla real. Antes: barra atravessava o conteúdo. Depois: barra claramente acima do
conteúdo em todos os casos, incluindo `a^2+b^2` (confirma que o sobrescrito elevado é correctamente
considerado no cálculo de `rad_box.ascent`). Estrutura muito próxima do vanilla — diferenças
residuais são de forma do glifo esticado (qualidade da montagem do símbolo `√`) e ajuste fino da
posição do índice, não do bug catalogado.

Também confirmado com radical aninhado (`sqrt(a + sqrt(b + sqrt(c)))`, caso mencionado em P894) —
cada barra correctamente empilhada acima do seu próprio conteúdo, sem atravessamentos.

`.typ` de 30 secções: hash confirmado igual aos passos anteriores
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila sem regressão
(`exit=0`, `(1)`...`(44)` completo). Secções com `sqrt` (1, 13, 14, 17, 18 e outras — 7 ocorrências
de `sqrt(` no ficheiro) beneficiam directamente da correcção.

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Todas as leituras dentro da baseline estabelecida
(`04-math` 154.9ms, consistente com leituras anteriores ~153-161ms). Sem regressão.

## Fora de âmbito

Diferenças residuais de forma do glifo `√` esticado (qualidade da montagem, não posição) e ajuste
fino da posição horizontal do índice em `root(n,x)` face ao vanilla — cosméticas, não o bug
catalogado neste passo, não investigadas.
