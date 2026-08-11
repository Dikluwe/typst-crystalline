# L0 de desenho — Passo 994 — Opção β: catch-all do `MathLayouter` delega ao `Layouter` normal

**Tipo**: Desenho pré-implementação (ADR-0084 B2, per decisão do diagnóstico P993).
**Decisão do dono**: β confirmada — preserva a separação de responsabilidades entre os
dois layouters em vez de o `MathLayouter` acumular conhecimento sobre tipos que não são
seus.
**Gate ADR-0127**: aplica-se — este L0 é o desenho a apresentar para confirmação antes de
qualquer código. Nenhum código é escrito neste passo.
**Diagnóstico pai**: `diagnostico-math-aninhado-layout-fase-a-passo-993.md`.

---

## Achado favorável, descoberto ao preparar este L0

O sistema já tem duas peças que fazem quase exactamente o que β precisa — não é preciso
desenhar do zero:

1. **`measure_content_real`** (`01_core/src/engine/layout/mod.rs`, P712) — já constrói um
   `Layouter` isolado e chama `layout_sub_frame` sobre um `Content` qualquer (incluindo
   tipos não-matemáticos: `Styled`, `Box`, `Align`, `Pad`, `Block`), devolvendo os itens
   realmente desenhados. É o mecanismo por trás de `measure()`.
2. **`FrameItem::Group`** (`entities/layout_types.rs`) — já existe e já é usado (P242,
   P248, P273.10) para embrulhar um conjunto de `FrameItem`s desenhados noutro sítio,
   com posição própria e, opcionalmente, `clip_mask`.

β, tal como desenhado abaixo, é a combinação destes dois mecanismos num ponto novo (o
catch-all de `layout_node`), não um mecanismo arquitectural novo.

---

## 1. Diferença importante em relação a `measure_content_real`

`measure_content_real` usa `FixedMetrics` porque `measure()` corre em contexto L1 puro,
sem acesso a métricas de fonte reais (`FallbackFontMetrics` é L3). Isso é uma divergência
mecânica documentada (ADR-0107), aceite porque `measure()` só precisa de proporções, não
de bytes exactos.

**O `MathLayouter` não tem essa limitação**: já é genérico sobre `M: FontMetrics` e já
recebe `self.metrics: &'a M` reais (é assim que `attach.rs`/`frac.rs`/etc. produzem
geometria real hoje, fora de testes). Se o `Layouter` normal também for genérico sobre
`FontMetrics` da mesma forma — **confirmar por leitura directa antes de assumir** — o
`MathLayouter` pode construir o `Layouter` temporário com `self.metrics` (o mesmo trait
object/generic real), não com `FixedMetrics`. Isso produziria geometria real para o
conteúdo embutido, não uma aproximação — melhor do que o precedente de `measure()`, não
apenas igual.

**Pergunta a responder por leitura de código antes de implementar** (Fase A da
implementação, não decidir aqui): `Layouter::new` aceita `M: FontMetrics` genérico da
mesma forma que `MathLayouter::new`? Se sim, o tipo do `Layouter` temporário no catch-all
é `Layouter<M>` com `self.metrics` passado directamente. Se não (por exemplo, se
`Layouter` só aceita um tipo concreto ou `&dyn FontMetrics`), documentar a diferença e
escolher a forma que preserva métricas reais sem duplicar código.

---

## 2. Onde entra a mudança

`01_core/src/engine/math/layout/mod.rs`, braço final de `layout_node` (linha 638 no
diagnóstico P993):

**Hoje**:
```rust
other => {
    let text: EcoString = other.plain_text().into();
    self.layout_text_node(&text, style)
}
```

**Proposto** (esqueleto, não código final — a assinatura exacta depende da resposta à
pergunta da secção 1):
```rust
other => {
    self.layout_external(other, style)
}
```

Com `layout_external` (nome provisório) fazendo, na ordem:

1. Construir um `Layouter` temporário isolado, com as mesmas métricas reais que o
   `MathLayouter` já tem (`self.metrics`), e a `StyleChain`/`TextStyle` corrente (`style`,
   já recebido pelo `layout_node`).
2. Chamar `layout_sub_frame` (mesmo mecanismo de `measure_content_real`, `Content::Place`,
   `Content::Transform`, `grid.rs`) com uma região sem limites de largura (paridade com o
   uso já existente para medição) — **decidir na Fase A da implementação se a largura deve
   ser ilimitada ou restrita ao espaço disponível na linha da equação**; medir o
   comportamento do vanilla neste ponto antes de decidir (ADR-0123), não presumir.
3. Obter `(height, items)` (ou `Frame` — a forma exacta devolvida por `layout_sub_frame`,
   confirmar por leitura) do resultado.
4. Calcular o deslocamento vertical (baseline) do bloco embutido. O vanilla usa
   `height/2 + axis_height` quando a caixa não declara baseline própria (diagnóstico P993
   §2.3, `resolve.rs:228-230`). O `MathLayouter` já tem `self.constants.axis_height`
   disponível (usado por `apply_axis_offset`) — reaproveitar o mesmo campo, não recalcular.
5. Embrulhar `items` num `FrameItem::Group` com a posição/baseline calculada, e devolver
   isso como o resultado do `MathBox` deste nó (o tipo intermédio que `layout_node`
   devolve — confirmar a forma exacta de encaixe de um `FrameItem::Group` dentro de
   `MathBox`, que hoje só agrega `ascent`/`descent`/`width`/`items` de itens
   `Text`/`Glyph`/etc.; pode ser preciso que `MathBox` aceite `Group` como mais um tipo de
   item, ou que o `ascent`/`descent`/`width` do `MathBox` sejam derivados directamente das
   dimensões do `Group` — decidir na Fase A com base no que `MathBox` já suporta).

---

## 3. Isto toca contrato público? (ADR-0127)

Análise preliminar, a confirmar na Fase A:

- **`Content`** (enum fechado, ADR-0026): **não** é tocado. Nenhuma variante nova — os
  tipos `Styled`/`Box`/`Align`/`Pad`/`Block` já existem; a mudança é só em como
  `layout_node` os trata.
- **`FrameItem::Group`**: já existe, não precisa de campo novo (a confirmar: os campos
  actuais — `pos`, `clip_mask`, `inner_width`, `inner_height`, `items` — chegam para este
  uso, ou falta um campo de baseline/offset próprio? Verificar por leitura antes de
  assumir que serve tal como está).
- **`MathLayouter`/`layout_node`**: a assinatura pública (`pub fn layout_equation`) não
  muda. O que muda é interno ao módulo (`pub(super)`) — mas o `MathBox` pode precisar de
  um variant/campo novo para representar um item `Group` embutido, o que **seria** mudança
  de contrato interno relevante o suficiente para justificar registo explícito no L0 final
  (mesmo não sendo API pública do crate).
- **Assinatura de `layout_node`**: not expected to change (continua a receber
  `&Content`/`style`, devolver `MathBox`).

**Conclusão preliminar**: mudança contida, não deve tocar o enum `Content` fechado nem a
API pública do `MathLayouter`. O ponto que precisa de confirmação/decisão explícita é se
`MathBox` precisa de um novo tipo de item interno para acomodar um `Group`. Se sim, isso é
o único ponto que pode justificar uma nova paragem de gate — mas por ser um tipo interno
(`pub(super)`), provavelmente cai em "fluxo contínuo" per ADR-0127 (correcção/extensão sem
mudar contrato *público*), não em paragem obrigatória. Confirmar esta leitura antes de
prosseguir.

---

## 4. Causa secundária (`align`) — fix pequeno, independente de β

Não depende de nada da secção 2/3. `eval_math_arg_value`
(`01_core/src/engine/eval/math.rs:193-210`) trata todos os argumentos posicionais dentro
de modo math como `Value::Content`, incluindo o ident `center` que deveria resolver como
`Value::Alignment` (ou equivalente cristalino). Fix: no caminho de avaliação de chamadas a
funções de layout dentro de `$...$`, resolver idents que a função espera como não-Content
(ex.: alinhamento) para o seu tipo real antes de os empacotar como `Value::Content`.
Pode ser feito no mesmo passo de implementação que β (não precisa de passo separado), mas
é conceptualmente independente — testar e poder reverter separadamente.

---

## 5. Plano de implementação (Fase B, protocolo de dois agentes — risco alto, toca
fronteira entre dois layouters)

1. **Fase A** (leitura, sem código): confirmar as três perguntas em aberto — genérico de
   `Layouter` sobre `FontMetrics` (secção 1), forma exacta de `layout_sub_frame`/`MathBox`
   (secção 2 item 5), e se `FrameItem::Group` precisa de campo novo (secção 3). Registar
   respostas no L0 final antes de qualquer código.
2. **Fase B — dois agentes** (per P898): Agente A escreve testes cobrindo os 6 casos do
   diagnóstico P993 (A1-A4c) mais um caso de não-regressão (conteúdo matemático puro, sem
   função de layout, deve continuar idêntico). Agente B implementa a partir do L0 final
   (pós Fase A).
3. **Revisão do orquestrador**: testar um caso composto não coberto — por exemplo, função
   de layout dentro de função de layout dentro de math (`text(size: 10pt)[$box(...)[$x$]$]`),
   para confirmar que a recursão não é superficial.
4. **Fase C — Revalidação**: recompilar o `.typ` de 30 secções + o estendido, `compare.py`,
   confirmar os 6 casos do diagnóstico corrigidos. Benchmark completo, zero regressão.
   Aplicar também o fix da secção 4 (`align`) e confirmar A4a separadamente.

---

## Resultado esperado

- `$...$` aninhado em `text()`/`box()`/`align()`/`pad()`/`block()` (e, por construção do
  mecanismo, qualquer função de layout futura não testada explicitamente) preserva
  itálico automático, `^`/`_`, tamanhos e efeitos visuais da função externa.
- `Content` (enum fechado) inalterado. API pública do `MathLayouter` inalterada.
- Fix pontual de `align()` aplicado e verificado em separado.
- Zero regressão nos ~5814 testes existentes.
