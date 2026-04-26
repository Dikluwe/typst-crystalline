# Relatório P156L — `pad` refino sides individualizadas

Refino de variant existente per **ADR-0065 critério #3**
(primeira aplicação concreta) e **ADR-0064 Caso C** (segunda
aplicação concreta). **Primeiro passo M com refactor real** após
série granular aditiva P156C-J. **Décima aplicação consecutiva**
do padrão diagnóstico-primeiro (sexta sob critério estendido de
ADR-0065).

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-pad-refino-passo-156l.md`
(7 itens canónicos ADR-0034 + 2 específicos para expansão de
variant existente per ADR-0065).

**Descoberta-chave do inventário** (§6.1): spec do P156L
declarava em §"Verificação" #5 que `pad` passava de `parcial`
para `implementado puro`, com cobertura **78% → 84%**. **Inventário
revelou divergência factual da spec**: `pad` já era `implementado`
desde P156C — a entrada listada como "parcial" na tabela A.5
(`pad, corners, sides (inset modeling)`) é **outra entrada
documental** sobre refino PageConfig, não a entrada principal
`pad(...)`.

**Decisão**: executar o refactor (mérito qualitativo próprio);
marcar pad como `implementado⁺` (sufixo ⁺) sinalizando refino
além do mínimo; **não actualizar cobertura quantitativa** —
divergência da spec documentada honestamente. Aplicação directa
de ADR-0065 critério #6 (divergência da spec via inventário).

### 1.2 Refactor variant `Content::Pad` (.2)

`01_core/src/entities/content.rs`:

```diff
- Pad { body: Box<Content>, padding: Sides<Length> }
+ Pad { body: Box<Content>, sides: Sides<Option<Length>> }
```

Field renomeado `padding → sides` para alinhar com naming
vanilla (que usa "left/top/right/bottom" sem prefix). Construtor
`Content::pad(body, sides: Sides<Option<Length>>)` actualizado.
Cobertura exaustiva de **9 sítios pattern-match estruturais**
(declaração, construtor, `is_empty`, `plain_text`, `PartialEq`,
`map_content`, `map_text`, `materialize_time`, `walk`).

### 1.3 Refactor stdlib `native_pad` (.3)

`01_core/src/rules/stdlib/layout.rs`:

- `native_pad` deixa de aplicar `unwrap_or(Length::ZERO)` e
  passa a construir `Sides<Option<Length>>` directamente.
- Helper privado novo `extract_sides_lengths` extraído da lógica
  de parse/precedência (não-genérico per pré-decisão; promoção a
  genérico/público diferida até segundo reuso).
- Validações preservadas: padding negativo rejeitado em qualquer
  side declarado; named arg desconhecido rejeitado; precedência
  específico > eixo > rest mantida (paridade vanilla `pad.rs:20-24`).
- Defaults agora resolvidos em **layout time** (sides.left.map_or(0.0, ...))
  em vez de em construct time.

### 1.4 Tests adaptados + novos (.4)

**7 tests pré-existentes adaptados** sem mudança de contagem:
- 1 unit em `entities/content.rs:pad_constructor_envolve_body`
  + 4 outros tests Pad em entities (is_empty, plain_text,
  partial_eq, map_text).
- 6 unit stdlib em `stdlib/mod.rs` (defaults, lados individuais,
  x/y, rest, precedência, aceita float/int) — cada um actualiza
  asserções `padding.left == Length::ZERO` para
  `sides.left == None` ou `Some(Length::pt(...))`.
- 1 layout E2E em `layout/tests.rs:layout_pad_avanca_cursor_bottom_e_top`.

**4 tests novos** (range mais estreito que P156J/I porque
trabalho dominante é regression):
- `pad_partial_eq` ganha asserção P156L: `Some(zero) ≠ None`
  (distinção semântica nova).
- `native_pad_p156l_apenas_um_lado_outros_none`.
- `native_pad_p156l_some_zero_distinct_from_none`.
- `native_pad_p156l_x_axis_apenas`.
- `native_pad_p156l_top_overrides_y_overrides_rest`.

**Δ tests = +4** (1077 → 1081 typst-core lib).

**Divergência da spec do passo §"Verificação" #1**: spec
estimava +12 a +18 tests. Real é +4 — explicação: 7 tests
pré-existentes foram **adaptados** (manutenção sem ganho de
contagem) e a única adição genuína são 4 tests P156L sobre
distinção semântica None/Some. Spec subestimou o peso da
adaptação vs adição.

### 1.5 Hashes + cobertura (.5)

`crystalline-lint --fix-hashes .` reportou **"Nothing to fix"**:
o refactor preserva o hash do prompt L0 (apenas implementação
muda; contrato L0 estável). Hash header `content.rs` mantém-se
`ec58d849`. `crystalline-lint .` confirma zero violations.

Tabela A.5 actualizada: `pad` `implementado` →
`implementado⁺` (sufixo ⁺). Distribuição Layout:
14/0/3/1/0=18 → **13/1/3/1/0=18**. Cobertura `(impl + impl⁺) /
total` = **78%** (inalterada quantitativamente).

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P156L:
tabela slope com linha P156L (slope 0%; cobertura "78%");
padrões metodológicos N actualizados (granularidade N=8 → 9;
inventariar primeiro N=5 → 6; Smart→Option N=6 → 7; §análise
risco N=5 → 6; reuso template N=4 inalterado; **novo subpadrão
reuso `Sides<T>` N=2**).

README ADRs ganha entrada P156L antes de P156K (preserva ordem
cronológica reversa). Total ADRs **63** (inalterado — refactor
não cria ADR).

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1315 + Δ; zero falhas (Δ esperado +12-18) | **Δ=+4** (1315 → 1319 lib+integ+diag); spec divergente (sub-estimou peso de adaptação vs adição); zero falhas |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 52 (inalterada) | **✓ 52** (refactor, não adição) |
| 4 | Stdlib funcs: 42 (inalterada) | **✓ 42** (helper privado novo, sem nova func pública) |
| 5 | Cobertura Layout: 84% (entrada `pad` parcial → puro) | **Divergência factual da spec**: pad já era `implementado` desde P156C; promovido a `implementado⁺` sem mudança quantitativa (78%); documentado em diagnóstico §6.1 |
| 6 | Hash actualizado em prompts L0 (`crystalline-lint --check-hashes` passa) | **✓** (refactor preserva hash L0; `--fix-hashes` reporta "Nothing to fix") |
| 7 | Regression tests P156C: 100% adaptados e a passar (6 unit + 11 stdlib + 2 E2E = 19 tests pré-existentes) | **✓** (7 tests pré-existentes com acesso directo a `padding` field adaptados; 12 tests `matches!(..., Pad { .. })` continuam válidos sem mudança) |

**Build limpo**: `cargo build` 2.73s → 0.94s sem warnings novos
relacionados com P156L.

---

## 3. Análise de risco — peso real (sexta aplicação consecutiva; primeira **não-cerimonial**)

P156L é **primeiro refactor real** após série aditiva P156C-J.
§análise de risco tem peso material desta vez.

### 3.1 Riscos materializados durante o passo

| Risco | Materializado? | Mitigação aplicada |
|-------|:--------------:|---------------------|
| Spec do passo conter assumpção factual errada | **SIM** | Inventário .1 detectou (per ADR-0065 critério #6); divergência documentada em §6.1 do diagnóstico e §1.5 deste relatório; refactor executado integralmente apesar da divergência |
| Tests pré-existentes acederem `padding` em sítios não previstos | Médio | Grep exaustivo identificou 12 sítios estruturais + 7 tests com acesso directo; adaptação one-shot com `replace_all` cuidadoso |
| Build inicial falhar por ordem de adaptação | Sim, brevemente | `cargo build` falhou em `layout/tests.rs:2024` (test E2E com `Sides::new(Length::ZERO, ...)`) que foi corrigido em ciclo único |
| Estimativa de tests Δ ser desafiada | **SIM** | Spec previa +12-18; real +4. Não-bloqueante: razoavelmente justificável por adaptação vs adição. Documentado |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| `Rel<Length>` (percentage) por side ser exigido | Baixo | Inventário §1 confirmou cristalino só absoluto Length; refino futuro fica isolado a um passo dedicado |
| Conflito x+left ter semântica "último vence" não-vanilla | Nulo | Inventário §1 confirmou precedência vanilla `left.or(x).or(rest)`; aplicação literal preservou paridade |
| Helper `extract_sides_lengths` ter complexidade superior | Nulo | Implementação não-genérica simples; promoção a genérico diferida sem custo |
| Quebra acidental de stdlib API pública | Nulo | API stdlib `pad(body, named...)` inalterada; só representação interna do variant mudou |

### 3.3 Riscos não-aplicáveis

- **Algoritmo dinâmico de runtime**: P156L não toca em runtime
  layout além de mover defaults de construct-time para layout-time
  (mecanicamente trivial).
- **Quebra de paridade observável**: variant interno mudou; saída
  observável (FrameItems) idêntica.

### 3.4 Conclusão de risco

**Risco residual: baixo após inventário**. O risco principal
materializado (spec conter assumpção errada) foi **detectado e
neutralizado pelo próprio mecanismo formalizado em P156K**
(ADR-0065 inventariar-primeiro). **Auto-validação empírica de
ADR-0065**: o ADR meta acabado de criar provou utilidade na
primeira aplicação concreta que justificou — o sub-passo `.1`
salvou o passo de divergência silenciosa.

**§análise de risco já não é cerimonial**: P156L estabelece o
precedente de risco real a documentar. Padrão #4 ganha peso
empírico significativo.

---

## 4. Slope cumulativo (mesa P156C-L)

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P156C | pad + hide | +11% | 22% → 33% | +27 |
| P156D | h + v | +11% | 33% → 44% | +20 |
| P156E | pagebreak | +6%  | 44% → 50% | +22 |
| P156F | skew | +6%  | 50% → 56% | +16 |
| P156G | block | +5%  | 56% → 61% | +20 |
| P156H | box | +6%  | 61% → 67% | +21 |
| P156I | stack | +5%  | 67% → 72% (target Fase 1+2) | +25 |
| P156J | repeat | +6%  | 72% → 78% (Fase 3 sub-passo 1) | +19 |
| P156K | (meta — ADRs 0064+0065) | — | — (sem código) | 0 |
| **P156L** | **pad refino sides** | **0%** | **78% (refino qualitativo)** | **+4** |

**Total cumulativo P156C-L**: **+56pp** Layout em 9 passos
materialização (22% → 78%). **+174 tests** acumulados
(1145 → 1319 typst-core lib + integ + diagnostic). **Zero
reformulações mid-passo** em N=9 aplicações de materialização.

P156K (meta) e P156L (refino) **expandem o tipo de passo
cumulativamente coberto**: meta documental + refino real são
agora precedentes activos da série, complementando o padrão
aditivo dominante P156C-J.

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P156L.

### 5.1 Padrões metodológicos pós-P156L

| # | Padrão | Pré-P156L | Pós-P156L |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 8 | **9** |
| 2 | "Inventariar primeiro" pré-decisão | 5 | **6** (primeiro critério #3 ADR-0065) |
| 3 | "Smart→Option/default" | 6 | **7** (segundo Caso C ADR-0064) |
| 4 | "§análise de risco no relatório" | 5 | **6** (primeiro com peso real) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 6 | **7** |
| 8 | **Reuso `Sides<T>`** (novo subpadrão P156L) | — | **2** |

### 5.2 Auto-validação de ADR-0064 e ADR-0065

P156L é **primeira aplicação concreta** dos dois ADRs
formalizados em P156K:

- **ADR-0064 Caso C** (segunda aplicação concreta após P156I):
  `Length` default zero → `Option<Length>` traduzido sem
  ambiguidade em sub-passo .1. Padrão estável em N=2 aplicações.
- **ADR-0065 critério #3** (primeira aplicação concreta):
  expansão de variant existente. Sub-passo `.1` detectou
  divergência da spec **antes** de execução — exactamente o
  papel previsto. Padrão validado em utilidade real.

ADR-0064 contagem implícita N=7 (formalizado em P156K com
N=6); ADR-0065 contagem implícita N=6 (formalizado em P156K
com N=5).

---

## 6. Estado pós-P156L

- **Cobertura Layout**: **78%** (13 implementado puro + 1
  implementado⁺ = 14/18). Inalterada quantitativamente vs P156J;
  pad ganha distinção qualitativa.
- **Variants Content**: **52** (inalterada — refactor).
- **Stdlib funcs**: **42** (inalterada — helper privado, não
  func pública).
- **Helper novo**: `extract_sides_lengths` privado em
  `stdlib/layout.rs`.
- **Tests**: **1319** typst-core lib + integ + diagnostic
  (era 1315). +4.
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados em P156L.
- **ADR-0061**: PROPOSTO; §"Aplicações cumulativas" actualizada
  com P156L.
- **README ADRs**: entrada P156L adicionada; total **63 ADRs**
  inalterado.
- **Hash content.rs**: `ec58d849` (inalterado — refactor preserva
  contrato L0).

### 6.1 Restantes 3 entradas Layout pendentes (era 4 antes de P156L)

- `columns` / `colbreak` (Fase 3 condicional — DEBT-56 column
  flow L+ aberto em P156B).
- `place` parcial — refino column scope (parcialmente
  implementado em P84.6).
- `measure` parcial — depende ADR-0017 Introspection runtime
  adiada.

---

## 7. Decisão pós-P156L

Per spec do passo §"Pós-passo", próxima decisão pré-acordada é
**abordagem para Introspection** (mais fraca per inventário 148:
17% cobertura). Identificador a definir (P156M ou outro). Três
sub-opções:

1. Sub-passo granular S/M sobre uma feature específica de
   Introspection (per padrão validado N=9 com P156L).
2. Diagnóstico amplo do estado de Introspection (passo S+
   documental, semelhante a P156B para Layout).
3. Atacar `measure` directamente (entrada Layout que depende
   de Introspection runtime; encadeia caminhos cruzados).

Outras direcções pendentes (per relatórios P156I/J/K):
4. Continuar Fase 3 — columns + colbreak (DEBT-56; quebra
   granularidade).
5. Mudar para Model Fase 2 P157 (table foundations).
6. Footnote area (sub-fase prioritária ADR-0061 Decisão 5).
7. Promover ADR-0061 a IMPLEMENTADO (3 caminhos documentados;
   caminho 1 a 50% pós-P156J; P156L não avança caminho).
8. Promover `extract_length` a helper público (refactor escopo
   XS sugerido por ADR-0064 §Implicações; agora N=7 reusos
   reforça candidatura).

ADR-0061 mantém-se `PROPOSTO`. **Padrão granularidade 1-2
features/passo (N=9) NÃO é formalizado neste passo** —
candidato a ADR meta futura se patamar continuar a crescer ou
se for desafiado por passo M+/L (e.g. columns/colbreak).

---

## 8. Fechamento

P156L fecha como **primeiro passo M com refactor real** após
série granular aditiva. **Risco médio materializou-se** (spec
com assumpção factual errada) e **foi neutralizado pelo próprio
mecanismo de ADR-0065** (auto-validação do ADR meta acabado de
criar em P156K). §análise de risco ganha peso empírico real
(N=6 com primeira aplicação não-cerimonial).

**Padrões consolidados pós-P156L**: granularidade N=9;
inventariar primeiro N=6 (primeiro critério #3); Smart→Option
N=7 (segundo Caso C); §análise risco N=6 (primeiro com peso
real); reuso `Sides<T>` N=2 (novo subpadrão).

**Auto-validação dos ADRs meta P156K**: tanto ADR-0064 quanto
ADR-0065 provaram utilidade na primeira aplicação concreta —
sessões futuras citam-nos como precedentes empíricos validados
em N=2 (Caso C) e N=1 (critério #3 — primeiro caso real).

ADR-0061 mantém `PROPOSTO`; promoção continua diferida.

**Pausa natural após P156L — primeiro refactor real fechado
sem reformulações; padrões ganham peso empírico real. Próxima
decisão humana (Introspection ou outras 7 candidatas) tem
máxima informação.**
