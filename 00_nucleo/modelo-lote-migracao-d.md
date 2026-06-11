# Modelo de Lote — Migração D (ADR-0104/0105)

Artefato reutilizável. Instanciar um lote = preencher os **parâmetros** e
seguir as fases. Gravado no P317 como entregável; usado pelos lotes 3+ sem
retrabalho. Não é L0 (não materializa código): é a *receita* de como migrar
variantes do `Content` para o modelo D (`Nome(Arc<NomeElem>)` + `impl Element`).

Referências de forma: `entities/elements/_comum.md` (o trait `Element`),
`entities/elements/math_styled.md` (precedente de handler math),
`entities/elements/heading.md` (precedente de **locatável**:
`element_kind`/`to_payload`), `entities/content.md` (o hub).

---

## Parâmetros (preencher por lote)

- `N_LOTE` — número do lote (e do passo que o executa).
- `LOTE` — lista de variantes a migrar, **ordenada por largura de uso
  crescente** (medir com Pre-2; a mais barata primeiro).
- `NOTAS_FAMÍLIA` — particularidades da família (ex.: math é structural,
  terminal em `map_text`; locatável segue Heading; etc.).

A **largura de uso** de cada variante (nº de sites de construção/match fora
de `content.rs`) é o dimensionador e o preditor de custo:

```sh
grep -rnE "Content::Nome([^A-Za-z0-9]|$)" 01_core 02_shell 03_infra 04_wiring \
  --include='*.rs' | grep -v "entities/content.rs" | wc -l
```

> **Correção do preditor — arms `|`-combinados (achado P319):** uma variante que
> **partilha um braço de `match`** (`Pat_A | Pat_B => …` com binding de campos)
> com variantes **ainda não migradas** custa **mais** que a largura sugere: o
> `Arc<…Elem>` é um tipo distinto por variante, logo o `|` com binding deixa de
> compilar e o braço tem de ser **separado** (ou os campos extraídos num
> sub-`match`). Antes de estimar, **inspecionar os `|` que envolvem o LOTE**:
> ```sh
> grep -rnE "Content::(A|B|…)\b.*\|" 01_core 02_shell 03_infra 04_wiring --include='*.rs'
> grep -rnB1 -E "\| Content::(A|B|…)" 01_core 02_shell 03_infra 04_wiring --include='*.rs'
> ```
> Casos sem binding (`Content::A(_) | Content::B(_) => …`) **não** custam extra.
> Exemplo P319: `Underline|Strike|Overline` partilhavam um braço com binding em
> `layout/mod.rs` e `introspect.rs` → separados.

> **Critério de elegibilidade (achado P317, guideline):** o modelo D só
> compensa naturalmente para variantes **element-shaped** — com corpo(s)/campos
> próprios que justifiquem um struct + `impl Element`. Primitivos de AST (folhas
> como `Text`/`MathIdent`, contentores como `Sequence`/`MathSequence`,
> marcadores unit como `MathAlignPoint`) tendem a não compensar: embrulhá-los em
> `Arc<…Elem>` é overhead e a largura de uso costuma ser enorme (paralela a
> `Text`/`Sequence`, também não migrados). **Default: excluir e registar.**
> **A composição é decisão do dono por lote** e pode em princípio incluir
> primitivos — mas a guideline pesa contra. No P317 o dono **confirmou a
> exclusão**: o Lote 2 migrou as **11** `Math*` element-shaped e diferiu
> `{MathSequence, MathText, MathIdent}` como decisão futura própria (medir
> performance se migrar). Se um lote incluir primitivos, registar a ressalva no
> L0 da variante **e** no relatório, com a medição que a justifique.

---

## Fase A — L0 (redigir e PARAR no checkpoint)

1. **Um prompt fino por variante**: `entities/elements/<nome>.md`.
   Content-preserving do que já existir nos matches do hub; spec nova só
   do desenho do trait (que métodos do `Element` a variante implementa /
   usa default). Precedente: `math_styled.md`.
2. **Atualizar `entities/content.md`**: variante `Nome { … }` →
   `Nome(Arc<NomeElem>)`; braços dos 6 matches → dispatch (`m.metodo()`).
3. **`_comum.md` só muda se o trait mudar — e o trait NÃO muda em lote.**
   Mudança de trait = passo próprio, fora do modelo.
4. **CHECKPOINT humano (Trava Arquitetural).** Apresentar: decisões de
   composição, os L0s redigidos, e o **plano de toque** (a lista de sites do
   grep, por variante). Prosseguir para a Fase B **só com confirmação humana**
   (o humano guarda os L0 e calcula os hashes).

## Fase B — implementação (ordem: largura de uso crescente)

1. **Testes primeiro**: testes unitários do `impl Element` de cada variante,
   no módulo próprio (`#[cfg(test)]`). Confirmar que falham.
2. **Migrar variante a variante** (a mais barata primeiro):
   - `NomeElem` struct no módulo próprio `entities/elements/<nome>.rs`
     (`#[derive(Debug, Clone, PartialEq, Hash)]`), absorvendo os campos.
     **Campos com `f64`/`Length`/`Color`**: estes tipos **não** implementam
     `Hash` → derivar só `Debug, Clone, PartialEq` e implementar `Hash` **à mão
     via `Debug`** (`format!("{self:?}").hash(state)`), paridade `content_hash`.
     Comentar a ressalva `-0.0` vs `0.0` no código — ver L0 do **Lote 4 P319**
     (`overline.md`). Os lotes 5+ não precisam de redescobrir nem re-perguntar.
     **Regra (P320):** Debug-hash é só para quando o `Hash` canónico é **inseguro**
     (f64/`Length`); um tipo que pode derivar `Hash` com segurança (`Copy+Eq`,
     sem floats — ex.: `Parity` no Lote 5) **recebe o `derive`**, com a linha no
     L0 do tipo se ele especificar derives, e registado como **dependência do
     lote** (não conserto oportunista).
   - Variante do enum: `Nome(Arc<NomeElem>)`.
   - Construtor ergonómico em `content.rs` (`Content::nome(...)` → embrulha
     `Arc::new(NomeElem { … })`), preservando a assinatura de chamada onde
     der.
   - Os 6 matches do hub viram dispatch (`plain_text`, `is_empty`,
     `map_content`, `map_text`, `get_field`, `PartialEq`).
   - Sites de construção/match externos atualizados para o construtor/Arc.
3. **Locatável** (se a variante for): segue o precedente Heading —
   implementar `element_kind`/`to_payload`; o estado misto dos enums de
   introspecção permanece e esvazia lote a lote.
4. **Linhagem**: header `@prompt`/`@prompt-hash`/`@layer`/`@updated` em cada
   módulo novo; `crystalline-lint --fix-hashes .` após guardar os L0.

## Validação (critérios fixos)

- `cargo build` verde.
- `RUST_MIN_STACK=33554432 cargo test --workspace` verde. A contagem **cresce
  só pelos testes unitários novos**; **nenhuma asserção existente alterada**
  (a sintaxe de construção pode mudar; a asserção não).
- `crystalline-lint .` → **ZERO violations**. Qualquer violação = regressão
  do lote: parar e corrigir antes de seguir (desde o P317 "zero" vale sem
  asterisco).
- Ressalva conhecida da stack (`recursao_infinita_*` com stack default)
  pode reaparecer; **registrar, não é regressão** (correr com `RUST_MIN_STACK`).

## Medições (fecham o lote — métrica ADR-0104)

- **`content.rs`**: linhas antes/depois. Esperado: **encolhe** (o setup do
  trait foi pago no P316). Lote que não encolha o hub exige explicação no
  relatório.
- **Custo-por-variante**: ficheiros/linhas fora do módulo próprio,
  **comparado à previsão da tabela de largura** — o lote valida o preditor.
- **Parte atómica**: linhas dos módulos novos (`elements/<nome>.rs`).

## Relatório (`typst-passo-<N>-relatorio.md` + resumo no chat)

Decisões, medições vs previsão, `content.rs` antes/depois, contagem da suíte,
**proposta do lote seguinte derivada da tabela de largura** (decisão humana),
fora-de-escopo confirmado, e **atualização da "Contabilidade de variantes"**
(mover as variantes do lote de "restantes" para "migradas"; é item obrigatório
do relatório de **todo** lote — o roteiro mora no repo, não em conversa).

## Regras permanentes

- O trait `Element` **não muda em lote**; quebra de desenho → parar e voltar
  ao L0.
- **Conserto oportunista proibido** (nada fora do escopo do lote).
- Toda medição acompanhada do **comando** que a produz.
- Primitivos de AST não entram **por default**; inclusão só por decisão
  explícita do dono, com a ressalva no L0 da variante e a medição que a
  justifique (ver critério de elegibilidade acima).
- **L0 de variante deferida não fica em `00_nucleo/prompts/`** (geraria V7
  órfão permanente e quebraria o "zero violations"): é anexado à entrada de
  DEBT que regista o deferimento, e volta a `prompts/` no passo que o
  materializar.
- Se a Fase B precisar **editar** um prompt grosso (`rules/eval.md`,
  `rules/parse.md`, `rules/layout.md` ou outro com linhagem larga), o imposto
  morde: **fatiar primeiro** pela receita do P314 (partição content-preserving,
  `_comum.md` por área, `git rm` do prompt velho — a trilha fica no git).
- **Um commit isolável por lote** (pré-tarefas separadas do lote principal).
  Lição medida no diagnóstico P313: o P298 não era isolável no histórico e
  custou à medição.

---

## Contabilidade de variantes (o roteiro dos lotes — atualizar a CADA lote)

`Content` tem **77 variantes** (baseline P313). Estado em **P322** (Lote 7 incluído):

### Migradas para o modelo D — 43

- **P316 piloto (3)**: `Divider`, `Heading` (locatável), `MathStyled`.
- **Lote 2 P317 — math (11)**: `MathCases`, `MathMatrix`, `MathAlignPoint`,
  `MathAccent`, `MathCancel`, `MathDelimited`, `MathRoot`, `MathUnderover`,
  `MathFrac`, `MathAttach`, `MathOp`.
- **Lote 3 P318 — lista/termos (5)**: `EnumItem`, `Link`, `ListItem`,
  `TermItem`, `Terms`.
- **Lote 4 P319 — decorações (3)**: `Overline`, `Strike`, `Underline`.
- **Lote 5 P320 — quebras/espaços + grid/table header/footer (9; união 5+6)**:
  `GridFooter`, `GridHeader`, `TableFooter`, `TableHeader`, `Linebreak`,
  `Colbreak`, `VSpace`, `HSpace`, `Pagebreak`. (Dependência: `Parity` ganhou
  `Hash` por derive.)
- **Lote 6 P321 — state/counter + Metadata (7; 6 locatáveis)**:
  `CounterDisplay` (não-loc legacy), `Metadata`, `CounterDisplayCallback`,
  `State`, `StateDisplay`, `StateUpdate`, `CounterUpdate`. **1º lote locatável
  desde o piloto** — os 6 locatáveis absorvem `element_kind`/`to_payload`
  (`extract_payload` despacha). Quirk de `eq` preservado em
  `Metadata`/`State`/`StateUpdate` (caem em `_ => false`).
- **Lote 7 P322 — largura (5)**: `Raw`, `Align`, `Image`, `Hide`, `Repeat`.
  Lote não-locatável padrão. Hash manual em `Image` (`Value`/`PtrEqArc`),
  `Align` (`Align2D`), `Repeat` (`Length`/f64); derive em `Raw`, `Hide`.
  `.into()` redundante removido nos call-sites de teste (`raw`/`image` tomam
  `impl Into<…>`; `.into()` no argumento gerava E0283).

### Fora de lote — decisão própria

- **`Set*` (4)** → decisão **F / DEBT 99.E** (superfície da StyleChain; NÃO
  migrar por lote): `SetHeadingNumbering` (62), `SetEquationNumbering` (16),
  `SetPage` (8), `SetFigureNumbering` (5). Medido em `medicao-pre-f-passo-318.md`.
- **Primitivos de AST → DEBT-58** (não element-shaped): `MathSequence` (21),
  `MathText` (50), `MathIdent` (108), `Sequence` (208), `Empty` (116),
  `Block` (101); **em triagem**: `Space` (13, cola de texto), e os wrappers
  `Styled` (58), `Boxed` (58), `Labelled` (55); **observação** (leaf, candidato
  à triagem): `Text` (40).

### Element-shaped restantes — ~19 (os lotes 8+ saem daqui, por largura)

(O Lote 7 P322 — Raw/Align/Image/Hide/Repeat, 5 variantes — saiu daqui.)

`Quote`20 ·
`Columns`22 · `Ref`23 · `Outline`24 · `SmartQuote`28 · `Stack`30 · `Cite`32 ·
`TableCell`32 · `Transform`32 · `Place`34 · `Pad`39 · `Bibliography`40 ·
`Table`41 · `Equation`45 · `Footnote`46 · `GridCell`47 · `Shape`57 · `Grid`73 ·
`Figure`89.

> **Bloco grid/table cell** (lote próprio, ~193 sites; decisão do dono):
> `TableCell`(32) · `GridCell`(47) · `Table`(41) · `Grid`(73) — contentores
> pesados com muitos campos cosméticos.

### Estimativa

~19 element-shaped restantes ÷ 5–9 variantes/lote (ritmo validado P317–P322)
≈ **2–4 lotes** até esgotar os elegíveis — gatilho da triagem do DEBT-58 e da
decisão F. Conta: 43 migradas + 4 `Set*` + 11 (DEBT-58: 6 + Space + 3 wrappers +
Text) + 19 restantes = 77. ✓
