# Passo 286 — Decorações textuais cientes de wrap (multi-line)

**Frente**: `P-text-deco-multiline` (P284.1 candidato).
**Origem**: P284 §5.3 — consumer Layouter de
`Underline`/`Strike`/`Overline` assume single-line; se o body
quebra para linha nova entre `start_x` e `end_x`, a decoração
desenha uma única linha que vai de `start_x` da linha inicial até
`end_x` da linha final — visualmente incorrecta.
**Pré-requisitos**: nenhum bloqueador. Infraestrutura `flush_line`
existente desde P216A/B; `Regions` struct em
`01_core/src/entities/region.rs`.

---

## §1 — Objectivo

Corrigir o consumer Layouter de
`Content::Underline`/`Strike`/`Overline` (P284 §2.3) para que cada
linha de texto coberta pela decoração produza um `FrameItem::Line`
próprio, em vez de um único Line que ignora as quebras de linha
intermédias.

Comportamento desejado:

- Body single-line (cabe numa linha): **1** `FrameItem::Line`
  emitida (paridade P284 bit-exact).
- Body N-line (faz wrap N-1 vezes): **N** `FrameItem::Line`
  emitidas, cada uma cobrindo a região horizontal do body nessa
  linha; offset Y e thickness consistentes por kind (paridade
  vanilla `text/deco.rs`).

Razão de ser:

- Fecha o último defeito conhecido remanescente do cluster
  decorações P284. Pós-P285 (stroke/RG activado), P286 fecha
  P284.1 — cluster completo.
- Padrão P285 §8.3 ("activação posterior de feature
  parseada-mas-inerte") aplicado simetricamente — desta vez para
  o caso `wrap` em vez do caso `stroke`.

Não-objectivo arquitectural: zero variants novos no `Content` enum.
Zero campos novos em `FrameItem::Line`. Modificação confinada ao
consumer Layouter + helpers L1 internos.

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Três ambiguidades factuais antes de materializar:

### A.1 — Inventário do mecanismo `flush_line` actual

Listar literalmente em
`00_nucleo/diagnosticos/diagnostico-deco-multiline-passo-286.md`:

1. **API actual de `flush_line`** — assinatura, sítios que invocam,
   ordem em que se desencadeia.
2. **Que estado regista o Layouter quando uma linha é fechada** —
   `Regions.current.width`, `cursor_y` pós-flush, `current_line`
   esvaziado? Onde fica o histórico se algum?
3. **Como `cursor_x` é reinicializado** após flush — vai para
   `Region.margin_left`? Existe `line_start_x` ou similar?

Output: tabela com a sequência de operações observada
empíricamente em `layout/mod.rs` (não a partir de documentação ou
comentários — leitura literal do código pós-P216A/B/C).

### A.2 — Estratégia de captura de linhas cobertas

Decisão arquitectural (per ADR-0065 critério #2 — atravessa
mecanismos pré-existentes):

| Opção | Mecanismo | Prós | Contras |
|---|---|---|---|
| **(a)** Snapshot pré-layout + pós-layout do contador de flushes; reconstruir as N linhas a partir do histórico de `Region.current_y` | Não toca em `flush_line`; lógica fica isolada no consumer da decoração | Depende de o Layouter expor histórico — A.1 confirma se existe |
| **(b)** Hook/callback registado no Layouter antes de `layout_content(body)` — `flush_line` invoca callback com `(start_x, end_x, y)` da linha que está a fechar; consumer da decoração colecciona e emite Lines no fim | Captura natural sem reconstrução | Adiciona campo opcional `pending_line_callbacks` ao Layouter — toca infraestrutura |
| **(c)** Reescrever consumer da decoração para iterar manualmente sobre o body, emitindo Line por cada palavra/run de glifos numa linha; detectar wrap via comparação `cursor_y` antes/depois de cada item | Granularidade máxima; sem alteração de infra | Replica lógica de wrap; alto risco de divergir do flush_line oficial |

Default sugerido: **(a)** se A.1 confirmar que `Region.history` ou
equivalente existe (provável dado P216A/B/C field-agregation).
**(b)** se A.1 mostrar que não há histórico e seria mais limpo
adicionar callback do que reconstruir.
**(c)** rejeitada salvo se A.1 mostrar que (a) e (b) são
estructuralmente bloqueados — improvável.

### A.3 — Tratamento do estado `extent` em decorações multi-line

`extent: Option<Length>` em P284 estende horizontalmente a linha
para além de `start_x`/`end_x`. Em multi-line, a decisão é:

| Opção | Comportamento |
|---|---|
| **(α)** Aplicar `extent` a **todas** as N linhas — cada Line emitida estende-se por `extent` em ambos os lados | Visualmente consistente; cada linha "respira" igual |
| **(β)** Aplicar `extent` apenas à **primeira** e **última** linhas — linhas do meio usam `(line_start, line_end)` sem padding | Paridade vanilla? — Fase A.3 deve verificar empíricamente em `lab/typst-original` |
| **(γ)** Aplicar `extent` apenas à **última** linha (extensão simétrica a `evade` que vanilla aplica desde início) | Comportamento ambíguo; pouco intuitivo |

Default sugerido: **(α)** — comportamento mais previsível e simétrico
ao P284 (extent é cosmético uniforme, não positional). **(β)** apenas
se inspecção vanilla mostrar paridade explícita.
**(γ)** rejeitada salvo descoberta vanilla específica.

---

## §3 — Materialização

Após Fase A produzir inventário + estratégia captura + política
extent:

1. Modificar consumer Layouter
   `01_core/src/engine/layout/mod.rs:1987-2014` (referência P285 §2.2)
   conforme decisão A.2:
   - Se A.2 → (a): consultar `Region.history` (ou equivalente)
     antes e depois de `layout_content(body)`; iterar sobre
     entradas novas; emitir 1 `FrameItem::Line` por entrada.
   - Se A.2 → (b): registar callback antes de
     `layout_content(body)`; remover callback após; emitir Lines
     a partir do estado coleccionado.
2. Calcular `line_y` por linha — herda do `cursor_y` no momento
   do flush dessa linha + `offset_pt` (constante per-kind, já
   estabelecido em P284).
3. Aplicar herança de cor per-linha — `color: stroke.or(style.fill)`
   exactamente como P285 fixou. Cada Line emitida tem mesma cor
   (não há razão para variar per-line; se houver, registar em
   diagnóstico).
4. Aplicar `extent_pt` conforme A.3.
5. Caso especial single-line — se durante `layout_content(body)`
   nenhum flush ocorrer, fallback para algoritmo P284 actual
   (1 Line, bit-exact preserved).
6. Testes:
   - L1 unitário: body single-line → 1 Line (regression P284).
   - L1 unitário: body forçado a wrap (largura suficientemente
     pequena, body suficientemente longo) → 2 Lines.
   - L1 unitário: body wrap 3-line → 3 Lines.
   - L1 unitário: `extent` aplicado per A.3 (verifica X em cada
     Line emitida).
   - L1 unitário: `stroke` herda em todas as linhas (cor
     consistente).
   - L3 integração PDF: `#underline[texto longo que faz wrap em
     largura 60mm]` produz 2 operadores `q ... S Q` separados no
     PDF; ambos com `RG` correcto.
7. Actualizar L0
   `00_nucleo/prompts/engine/layout.md` (ou caminho equivalente)
   com nota multi-line + propagar hash via
   `crystalline-lint --fix-hashes`.
8. Actualizar Tabela A.3 linha 103 — nota:
   "stroke funcional (P285); multi-line wrap-aware (P286)".
9. Marcar restrição graded P284 §5.3 como **RESOLVIDA** com
   referência cruzada P286.
10. Se A.2 → (b) requer alteração em `flush_line`: actualizar L0
    `region.md` + propagar hash; documentar callback em ADR-0061
    secção §"Aplicações cumulativas" (sem promover a ADR nova).

**Sem caps** (per P282 §7). Estimativa de testes: ~6-12 (delta
modesto; passo focado).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P285: 2 725 testes.
  Esperado: ~2 731 a ~2 737.
- `crystalline-lint` zero violations.
- Hash L0 `layout.md` muda (consumer alterado).
- Hash L0 `region.md` muda **se** A.2 → (b) (callback adicionado).
- Hash L0 `export.rs` **preserved** (`66cb8ac3` desde P285) — o
  emit não muda; apenas mais ou menos Lines passam pelo mesmo
  caminho.
- **Regressão bit-exact validada** para body single-line — caso
  P284 sem wrap produz mesmo PDF byte-a-byte vs pré-P286.
- Tabela A.3 linha 103 actualizada com referência P286.
- P284 §5.3 marcada RESOLVIDA.
- Cluster decorações P284-P285-P286 marcado como **COMPLETO**
  (zero defeitos graded remanescentes).
- Diagnóstico A.1+A.2+A.3 produzido.

---

## §5 — Não-objectivos

- **Não** alterar `FrameItem::Line` — N Lines emitidas usam o
  variant pós-P285 sem modificação adicional.
- **Não** implementar `evade` (descender skipping, scope-out P284
  §3.1). Continua ADR-0054 graded.
- **Não** alterar `flush_line` semanticamente — se A.2 → (b),
  apenas estender com callback opcional; ordem e timing actuais
  preservados.
- **Não** unificar `Underline`/`Strike`/`Overline` num variant
  tagged (decisão P284 A.3 → opção α; preservar).
- **Não** estender o mecanismo a outras decorações inline além
  das 3 P284. Se Tabela A revelar candidatos futuros (smallcaps,
  super/sub script standalone), passos próprios.
- **Não** materializar `text-deco`-em-Math (decorar uma equação
  inteira com `#underline[$x+y$]`). Hipoteticamente já funciona
  via recursão `layout_content`; se Fase A.2 revelar quebra de
  layout em math, registar em diagnóstico mas não corrigir neste
  passo.

---

## §6 — Pendências relacionadas

Resolve:
- **P284 §5.3** — restrição graded multi-line wrap.

Não resolve (continua aberto):
- Objecto `Stroke` rico (Tabela A.7 linha 201) — distinct passo.
- `evade` em Underline/Overline (P284 §3.1 scope-out) — passo
  futuro condicional.
- `text.script` super/sub standalone (Tabela A.3 linha 98) —
  feature distinta.

---

## §7 — Risco residual

Risco principal: A.2 → (a) pode revelar que `Region.history` não
existe em forma consultável, forçando A.2 → (b). Callback no
Layouter é mudança estrutural pequena mas não cosmética —
introduz acoplamento entre consumer da decoração e infraestrutura
`flush_line`.

Mitigação: A.1 inclui inspecção literal pós-P216A/B/C. Se nenhuma
forma de observação directa existir, A.2 → (b) é o caminho;
documentar o porquê em diagnóstico.

Risco secundário: granularidade do flush. Vanilla typst aplica
decoração em "lines de inline content" — mas o que o Layouter
crystalline chama "linha" pode ser semanticamente distinto
(line-boxes, runs, frames). Se Fase A revelar que `flush_line`
não corresponde 1:1 a uma linha visual de texto, ajustar
estratégia A.2 conforme o que existe.

Mitigação: A.1 deve registar explicitamente a definição operacional
de "linha" no Layouter actual — não deduzir do nome.

Risco terciário: regressão bit-exact em body single-line. Mitigação:
caso especial §3 ponto 5 — se nenhum flush ocorre durante
`layout_content(body)`, executa exactamente o algoritmo P284
original. Validado por teste regression dedicado.

---

## §8 — Ponteiros

- Consumer actual: `01_core/src/engine/layout/mod.rs:1987-2014`
  (referência P285 §2.2).
- Algoritmo P284 a estender: P284 §2.3 do relatório.
- Infraestrutura `flush_line` + `Regions`: `01_core/src/entities/region.rs`
  + `layout/mod.rs` (P216A/B/C, P243 extensão).
- Precedente "activação posterior de feature graded": P285 §8.3
  (stroke parseado-mas-inerte → activo) — P286 é a aplicação
  paralela para o caso `wrap`.
- Vanilla: `lab/typst-original/crates/typst-library/src/text/deco.rs`
  — verificar comportamento multi-line de referência (em
  particular tratamento de `extent` para Fase A.3).
- ADR processual: ADR-0065 (inventariar-primeiro; 3 critérios
  cobertos por A.1/A.2/A.3).
- ADR scope: ADR-0054 graded (justifica não-objectivos §5).
- ADR layout: ADR-0061 (padrão Layout Fase 2; este passo não
  promove mas pode anotar em §"Aplicações cumulativas" se A.2 → (b)
  estender o Layouter).

---

*Spec P286 produzida 2026-05-19 pós-P285 (FrameItem::Line.color
activado). Frente `P-text-deco-multiline` — fecho do cluster
decorações P284-P285-P286 ao resolver o último defeito graded
remanescente (P284 §5.3 wrap). Modificação confinada ao consumer
Layouter; zero variants novos; hash `export.rs 66cb8ac3` preserved.
Fase A obrigatória (inventário `flush_line`, estratégia captura,
política `extent` multi-line). Critério de fecho inclui validação
bit-exact para body single-line (regression P284). Sem caps LOC
ou magnitude (P282 §7).*
