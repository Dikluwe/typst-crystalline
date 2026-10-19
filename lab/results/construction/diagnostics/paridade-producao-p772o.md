# P772o — Investigação instrumentada: colapso de espaço em fontes variáveis de peso alto

> **Passo:** 772o
> **Data:** 2026-07-16/17
> **Commit-base:** `61b7edee78fdae9b020e458f5989f638cbf04096` (HEAD, mesmo de
> P772k/l/m/n).
> **Medido/implementado em:** 2026-07-16T22:38Z–2026-07-17T00:02Z.
> **Dependência:** P772m §3 (achado original, hipótese não confirmada).

---

## 1. Reprodução (Passo 0)

Ambiente reconstituído de P772m (venv persistiu em `/tmp/p772m/venv`,
`fonttools` 4.63.0):

```
#set text(font: "Ubuntu Sans", weight: 800)
Weight test here
```

Confirmado: `Weighttesthere` (todas as palavras coladas), exit 0, sem erro
de compilação.

---

## 2. Instrumentação (Passo 1) — a hipótese de P772m estava incompleta

Instrumentação temporária (`eprintln!`, revertida no fim do passo) em
`FallbackFontMetrics::advance` (`03_infra/src/font_metrics.rs:704-746`,
chamada por `space_width()` e por `word_width()`/`text_width()` — **não
só pelo espaço**, ao contrário do que P772m assumiu).

### 2.1 — Medição do espaço: confirma metade da hipótese

`adv_units=229` (glifo espaço) **idêntico** para pesos pedidos 100, 400,
700, 800 — o valor nunca muda. Confirma que `advance()` ignora o peso
pedido.

### 2.2 — Comparação com `fontTools.varLib.instancer` (chão de verdade): a magnitude não bate

```python
# UbuntuSans-Variable.ttf, glifo "space"
default (wght=400): 229
wght=800:            240   # diferença de 11 unidades / 1000 upem ≈ 0.12pt a 11pt
```

Uma diferença de ~0.12pt **não explica** palavras coladas. A hipótese de
P772m (só o espaço estava errado) estava **incompleta** — media-se o
sintoma mais visível, não a causa suficiente.

### 2.3 — Achado real: `advance()` também mede as PALAVRAS, não só o espaço

Instrumentação em todos os caracteres (não só `' '`) revelou:
`adv_units` para `W`,`e`,`i`,`g`,`h`,`t` a peso pedido 800 = `924, 552,
243, 575, 571, 393` — **idênticos** aos valores da instância por omissão
(wght=400) de `UbuntuSans-Variable.ttf`, confirmados via `fontTools`:

```python
default (wght=400): [924, 552, 243, 575, 571, 393]  sum=3258
wght=800 real:       [948, 584, 289, 594, 589, 444]  sum=3448   # +190 unidades = ~2.09pt em "Weight" só
```

**Causa real, confirmada por medição, não suposição:** `advance()` —
usada tanto para `space_width()` como para `word_width()`/`text_width()`
(a função "fonte única de verdade do nível palavra", P593) — nunca aplica
as coordenadas de eixo (`wght`/`ital`) à `Face` (`ttf_parser`) antes de
ler `glyph_hor_advance`. Isto afecta **todas** as larguras medidas pelo
*layout* para fontes variáveis, não só o espaço. Entretanto, o *shaper*
(`03_infra/src/shaper.rs`, que converte `FrameItem::Text` →
`FrameItem::TextShaped` numa passagem posterior) **já** aplica
`rb_face.set_variations(&axis_vars)` correctamente — por isso os glifos
desenhados ficam correctamente mais largos (bold visível na inspecção
visual de P772m), mas o *layout* já tinha posicionado cada palavra
usando a largura estreita (por omissão). O erro acumula carácter a
carácter; para "Weight test here" a peso 800 excede largamente a largura
do espaço.

### Cache não é a causa (Passo 2, "testar sem cache")

`cached_face(slot_idx)` cacheia por `slot_idx` (identidade do ficheiro de
fonte), não por variante — mas isto não é o bug: mesmo sem cache nenhuma,
uma face recém-parseada por `ttf_parser::Face::parse` nunca aplica
variação sem uma chamada explícita a `set_variation`. Confirmado por
leitura de `CachedFace::new` (`Face::parse(slice, 0)`, sem
`set_variation`) — não foi necessário desactivar o cache experimentalmente
para refutar esta hipótese alternativa.

**Achado colateral relevante:** a chave de cache (`advance_width_key`,
P659/P677) **já** incluía um `axis_hash` calculado a partir das mesmas
coordenadas de eixo — a infra-estrutura para diferenciar por variante já
existia; só a aplicação real à face usada para medir estava em falta.

---

## 3. Âmbito — testado em 3 fontes variáveis (Passo 2)

| Fonte | Eixo | Resultado antes da correcção |
|---|---|---|
| `Ubuntu Sans` (`glyf`/`gvar`, `.ttf`) | wght 100–800 | Cola a partir de ~770 |
| `Noto Sans` (`glyf`/`gvar`, `.ttf`) | wght 100–900 | **Nunca cola**, nenhum peso testado |
| `Cantarell` (CFF2/`HVAR`, `.otf`) | wght 100–800 | Cola no peso 800 |

2 de 3 fontes exibem o bug — **mecanismo geral** (afecta qualquer fonte
variável), **magnitude dependente do desenho da fonte** (o quanto os
avanços dos glifos variam ao longo do eixo). `Noto Sans` aparentemente
mantém avanços quase constantes entre pesos (desenho que preserva reflow
de texto); `Ubuntu Sans`/`Cantarell` não.

---

## 4. Correcção aplicada

`03_infra/src/font_metrics.rs`, `FallbackFontMetrics::advance`: dentro do
loop carácter-a-carácter, clona-se a `Face` cacheada (`ttf_parser::Face`
implementa `Clone` — cópia ~2KB da estrutura já parseada, **não** um
re-parse dos bytes) e aplica-se `face.set_variation(tag, value)` para
cada entrada de `axis_variations_for_font_variant(&text_style_to_font_variant(style))`
— exactamente as mesmas coordenadas já usadas por `shaper.rs` e já usadas
para diferenciar a chave de cache (P659). `set_variation` devolve `None`
silenciosamente para fontes não-variáveis ou eixos ausentes — sem
alteração de comportamento para esse caso.

L0 actualizado antes de considerar o passo fechado:
`00_nucleo/prompts/infra/font_metrics.md` — nova secção "Variação de eixo
aplicada à medição (P772o)", invariante nova, histórico de revisões.
Hashes re-sincronizados via `crystalline-lint --fix-hashes .`
(`03_infra/src/fallback_fonts.rs` partilha este L0 — corrigido à mão
depois de o `--fix-hashes` automático escrever o hash novo na entrada
errada das suas duas linhas `@prompt`/`@prompt-hash`; achado registado
para quem usar `--fix-hashes` num ficheiro com múltiplos `@prompt`).

Teste de regressão novo, co-localizado:
`font_metrics::tests::p772o_advance_aplica_variacao_wght_em_fonte_variavel`
— carrega `UbuntuSans-Variable.ttf` real via `SystemWorld`, mede
`advance("Weight", ...)` a peso 400 vs 800, exige diferença > 1.5pt
(medido: ~2.09pt). Falha se a regressão voltar.

---

## 5. Resultado — Ubuntu Sans corrigido, Cantarell continua por explicar

### 5.1 Ubuntu Sans — corrigido e verificado em toda a gama

```
pesos 100, 200, 300, 400, 500, 600, 700, 760, 770, 780, 790, 799, 800
→ "Weight test here" em todos, sem excepção (antes: colava a partir de ~770)
```

### 5.2 Noto Sans — sem regressão

`weight: 900` (o seu próprio máximo) → `"Weight test here"`, inalterado.

### 5.3 Cantarell-VF — **continua a colar no peso 800, apesar da correcção**

Instrumentação directa confirma que, com a correcção aplicada, `advance()`
para Cantarell **já mede correctamente**: espaço a wght=800 = 200 unidades,
que bate exactamente com `fontTools.varLib.instancer` (220→200 unidades,
o espaço desta fonte **estreita** com o peso, ao contrário do de Ubuntu
Sans). As larguras de "Weight"/"test"/"here" também batem com o chão de
verdade. Ainda assim, a inspecção visual (`mutool draw` + crop, não só
`pdftotext`) confirma sobreposição total dos glifos — **zero espaço
visível**, não um espaço apertado.

**Isto não é a mesma causa já corrigida.** Com as métricas de `advance()`
agora correctas, a causa residual está noutro lado — candidato mais
provável (não confirmado): `Cantarell-VF.otf` é `CFF2`+`HVAR` (contorno
CFF2 com deltas de variação via `HVAR`), ao contrário do `glyf`/`gvar` de
`Ubuntu Sans`; é possível que o *shaper* (`rustybuzz`, via
`rb_face.set_variations`) não aplique os deltas de `HVAR`/`CFF2` da
mesma forma que `fontTools`/`ttf_parser` para este tipo de fonte,
desenhando os glifos mais largos do que o layout (agora correcto) prevê
— o inverso do bug original. **Não verificado nesta ronda** — precisaria
de instrumentar `shaper.rs` (posições de glifo devolvidas por
`rustybuzz::shape`) e comparar directamente com os avanços que o motor
CFF2/HVAR de `fontTools` calcula para a mesma fonte/peso, o que fica fora
do orçamento deste passo.

**Descartado nesta ronda** (para o próximo passo não repetir):
- Não é o cache de `cached_face` (§2, confirmado por leitura de código).
- Não é `advance()`/`font_metrics.rs` para Cantarell especificamente — já
  mede certo, confirmado por instrumentação directa.
- Não é `units_per_em` (1000 para ambas as fontes, sem mismatch).

---

## 6. Decisão (Passo 3)

Causa confirmada e corrigida para o mecanismo geral (medida em
`advance()`, afecta qualquer fonte variável cujo layout dependa dela) —
correcção pequena e isolada, dentro deste passo, conforme haviam
previsto os critérios de fecho. Cantarell-VF expõe uma **segunda causa**,
não isolada com confiança suficiente para corrigir às cegas — registada
como item de investigação em aberto, não como falha deste passo (mesma
disciplina de P763h Parte B, citada no prompt do passo).

---

## 7. Validação

```
cargo test --workspace
  4176 (typst-core) + 645 (typst-infra, +1 novo) + 33 + 2 + 29 + 2, 0 falhas
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente, não relacionado)
```

Sem regressão nos testes de fonte existentes (P666-669, P525, P530) —
suite completa verde, incluindo os testes de instanciação de fonte
variável já presentes.

## Critério de fecho do passo (`typst-passo-772o.md`)

- [x] Reprodução confirmada com o mesmo ambiente de P772m.
- [x] `space_width()` e a face cacheada instrumentados e comparados com o
      caminho de shaping normal — revelou que o problema não era só o
      espaço (achado que corrige a hipótese original).
- [x] Causa confirmada por medição (não suposição): `advance()` nunca
      aplica variação de eixo; comparado com `fontTools.varLib.instancer`
      como chão de verdade.
- [x] Testado em 3 fontes variáveis (Ubuntu Sans, Noto Sans, Cantarell) —
      2/3 afectadas; mecanismo geral, magnitude dependente da fonte.
- [x] Corrigido para o mecanismo confirmado (Ubuntu Sans): validação
      visual e numérica em todos os pesos testados (100–800), sem
      regressão nos testes de fonte existentes.
- [x] Não corrigido para Cantarell-VF: registado o que foi descartado
      (cache, `advance()`/font_metrics.rs para esta fonte, units_per_em)
      e a hipótese mais provável não verificada (CFF2/HVAR no shaper),
      para o próximo passo não repetir o mesmo caminho.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772o.md`.

---

## Próximo passo

Item fechado (Ubuntu Sans, e por extensão qualquer fonte variável
`glyf`/`gvar` com o mesmo padrão) — sem dependência conhecida, não
destrava outras partes da migração.

Item em aberto: colapso de espaço em `Cantarell-VF.otf` (CFF2/HVAR) a
peso alto, com métricas de `advance()` já correctas — aponta para o
*shaper* (`03_infra/src/shaper.rs`/`rustybuzz`) como próximo local a
instrumentar, não para `font_metrics.rs`. Candidato a passo dedicado
futuro, não bloqueante.
