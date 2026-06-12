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

     > **Passo mecânico obrigatório — transformador × posições de padrão**
     > (achados P321 + P323; o erro reincidiu apesar do lembrete, por isso
     > vira regra, não nota). Antes de rodar o transformador de construções
     > sobre os sites do LOTE:
     > 1. `grep -rn 'matches!' --include='*.rs'` filtrado pelas variantes do
     >    LOTE, e grep equivalente para destruturações em `if let`/`match`
     >    com struct-literal (`Content::Nome {`) — **registrar a lista** (vai
     >    para o relatório).
     > 2. **Excluir esses sites da passada automática** (ver item 5 —
     >    skip-list por `ficheiro:linha`); tratá-los à mão.
     > 3. O transformador só serve **construções** com `Box` externo único;
     >    **qualquer posição de padrão é manual por regra**, não por lembrete.
     >    `Box::new` aninhado (ex.: `StateUpdate::Set(Box::new(v))`) também é
     >    manual — o transformador sobre-remove.
     > 4. **Verificação pós-passada (C1-bis, achado P324 — 3ª reincidência).**
     >    O elo que falhou no P324 foi o **(ii)**: o grep do passo 1 *capturou*
     >    os padrões aninhados (`Value::Content(Content::Nome {…})`), mas a
     >    exclusão do passo 2 confiou na heurística interna do transformador,
     >    que os converteu na mesma (E0164). A exclusão deixa de ser
     >    promessa e vira **verificável**: depois da passada, intersetar a
     >    lista de sites que o transformador tocou com a lista do grep (passo
     >    1). **Interseção não-vazia ⇒ parar e reverter antes de compilar** —
     >    o transformador tocou num padrão. A regra existe para tornar o erro
     >    *impossível*, não documentado.
     > 5. **Skip-list por linha-do-grep (C1-ter, achado P325 — 4ª reincidência).**
     >    A C1-bis *apanha* o erro mas não o torna impossível: a heurística
     >    interna do transformador (decidir construção-vs-padrão) continua o
     >    elo fraco. **A exclusão deixa de depender da heurística**: a lista
     >    do passo 1 vira **skip-list explícita** — o transformador recebe o
     >    conjunto de `ficheiro:linha` e **pula** qualquer `Content::Nome {`
     >    cuja linha esteja na lista (não decide; obedece). A verificação
     >    pós-passada (item 4) **permanece**, rebaixada a rede de segurança:
     >    a partir de P326, interseção não-vazia = a skip-list falhou
     >    (reportar como falha da C1-ter). O transformador é **efémero**
     >    (script `/tmp` reescrito por lote) — por isso o mecanismo mora
     >    **aqui**, não no script: cada passo que use transformador gera a
     >    skip-list do grep e passa-a ao script antes da passada.
     > 6. **A skip-list cobre AMBOS os transformadores (C1-quater, achado P326).**
     >    Há dois transformadores efémeros: o de **construções** (`Content::X {…}`
     >    → `Content::x(…)`) e o de **padrões** (`if let`/`match` → `(e)` +
     >    prefixo `e.`). No P326 a skip-list cobriu só o de construções; o de
     >    padrões, sem ela, quebrou patterns **aninhados** (`Shape { kind:
     >    ShapeKind::Path(items), .. }` — perdeu o binding `items`). Regra: a
     >    **mesma** skip-list por `ficheiro:linha` é entregue **aos dois**;
     >    nenhum decide por heurística. E **padrões aninhados são excluídos por
     >    classe**: qualquer padrão com destruturação interna além do
     >    `Content::X` de topo (enum/struct interno) é **sempre** manual
     >    (receita validada P326: `let <Pat> = … else { panic!… }`), nunca pela
     >    passada automática — mesmo que não esteja na skip-list. A verificação
     >    pós-passada (item 4) é a rede de segurança dos dois.
     > 7. **Construtor vs Arc-wrap nas construções (C2, achado P327).** O
     >    transformador de construções emite `Content::x(…)` **só** quando o
     >    construtor ergonómico cobre **todos** os campos da variante. Para
     >    variantes **densas** (ex.: `Grid`/`GridCell` 10 campos; o `table_cell`
     >    só toma 5) o construtor perde os cosméticos → a emissão correta é
     >    `Content::X(Arc::new(Elem{…}))` (caminho qualificado, sem novos
     >    imports). Em `materialize_time` (recursa um campo, preserva o resto):
     >    `Content::X(Arc::new(Elem { campo: novo, ..(**e).clone() }))`. A regra:
     >    transformador conhece a aridade do construtor por variante; se < nº de
     >    campos, Arc-wrap.
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

`Content` tem **77 variantes** (baseline P313). Estado em **P327** (Lote 12 incluído):

### Migradas para o modelo D — 61

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
- **Lote 8 P323 — largura (4)**: `Ref`, `Outline` (locatável), `Columns`,
  `Quote`. `Outline` é **unit struct** (precedente `Divider`/`Linebreak`;
  campos do vanilla pendentes de cobertura) — opção (a) confirmada: migrar
  uniforme, absorvendo `element_kind`/`to_payload` no trait. Derive Hash em
  `Ref`/`Outline`/`Quote`; manual em `Columns` (`Length`/f64). Construtor
  `Content::reference` (não `r#ref`: evita raw identifier nos call-sites).
  `Ref`/`Outline` ficam no arm `|`-terminal combinado de `map_*` (sem split —
  são leaves sem binding).
- **Lote 9 P324 — largura (5)**: `SmartQuote`, `Stack`, `Cite` (locatável M1),
  `Transform`, `Place`. Derive Hash em `SmartQuote`/`Cite`; manual em `Stack`
  (`Length`), `Transform` (`TransformMatrix`/6×f64), `Place` (f64/`Align2D`).
  **Dependência**: `CitationForm` ganhou `Hash` por derive (Copy+Eq sem floats;
  precedente `Parity` P320; L0 `citation_form.md` atualizado por pinar derives).
  `Transform`/`Place` `is_empty` no default `false` (não delegam — paridade hub
  `_ => false`, precedente Align L7). C1 aplicada: sites de padrão tratados à
  mão; o transformador errou de novo (3 padrões → construtor) — corrigidos.
- **Lote 10 P325 — largura (3)**: `Pad`, `Bibliography` (locatável P181C),
  `Equation` (locatável P186B). **2 de 3 locatáveis** — absorvem
  `element_kind`/`to_payload`. Derive Hash em `Bibliography` (`BibEntry: Hash`)
  e `Equation` (`Content`+bool); manual em `Pad` (`Sides<Length>`/f64).
  **`Equation` é assimétrico** (inédito): `map_content` recursa no body,
  `map_text` é **terminal** (math structural; arm `|`-combinado, sem split).
  `Equation` `is_empty` no default `false` (não delega). **C1-bis estreou e
  apanhou a 4ª reincidência**: o transformador converteu 2 padrões aninhados
  (`Value::Content(Content::{pad,bibliography}(…))`) → a verificação pós-passada
  (interseção sites-tocados ∩ grep ≠ ∅) parou antes de compilar; revertidos à
  mão. A heurística do transformador continua o elo fraco (candidato a C1-ter:
  excluir por linha-do-grep, não por heurística).
- **Lote 11 P326 — largura (2)**: `Footnote`, `Shape`. **Ambas
  não-locatáveis** (achado: contraria a hipótese — `Footnote` é P295 Fase-1
  marker-only, scope-out; *não* locatável hoje). `Footnote` contentor (body,
  derive Hash); `Shape` **leaf** geometria (manual Hash por `Value`/`Color`/
  `Stroke`; arm `|`-combinado, sem split). `Footnote` `is_empty` no default
  `false`. **C1-ter estreou e funcionou**: skip-list por `ficheiro:linha`
  preveniu o transformador de tocar nos 21 padrões aninhados (1ª passada limpa
  em 5 lotes; verificação pós-passada confirmou interseção vazia). Achado
  separado: o transformador de **padrões** (distinto do de construções) não
  honra a skip-list e quebrou patterns `Shape { kind: Path(items), .. }`
  aninhados — corrigidos à mão com `let-else`; **a skip-list deve cobrir ambos
  os transformadores** (nota para futuros lotes). `content.rs` cresceu +10
  (hub encolheu, mas 2 construtores novos — `shape` 5-param e `footnote`,
  inexistentes antes — compensaram).
- **Lote 12 P327 — bloco grid/table cell (4)**: `TableCell`, `Table`,
  `GridCell`, `Grid` (~193 sites, o maior lote). Todas **não-locatáveis**,
  **Hash manual** (`TrackSizing`/`Length`/`Align2D`/`Sides`/`Stroke`/`Color`,
  f64). Contentores: `Table`/`Grid` recursam em `children`/`cells` (Vec) +
  `Grid` em header/footer; `TableCell`/`GridCell` (gémeas, 10 campos) no body.
  **Válvula**: o `|`-combinado **com binding** entre as 4 NÃO existe no hub
  (arms separados); o único combinado estava em `layout/grid.rs`
  (`GridCell | TableCell`) — **dividido** (tipos `Elem` distintos não partilham
  or-pattern). Custo ≈ largura; bloco rodado inteiro (decisão do dono) com
  validação intermediária após cada variante. **C1-quater estreou**: skip-list
  entregue aos dois transformadores; padrões aninhados (tuple if-let
  `(Grid, Table)`, `Shape Path(items)`) à mão. Achado de tooling: o
  transformador de **construções** assume construtor-cobre-campos — falso para
  densos (constrsolver só toma 5 de 10); usado `Arc::new(Elem{…})` direto com
  caminho qualificado (sem novos imports). `content.rs` **−238** (encolhimento
  grande esperado — arms verbosos das 4).

### Fora de lote — decisão própria

- **`Set*` (4)** → decisão **F / DEBT 99.E** (superfície da StyleChain; NÃO
  migrar por lote): `SetHeadingNumbering` (62), `SetEquationNumbering` (16),
  `SetPage` (8), `SetFigureNumbering` (5). Medido em `medicao-pre-f-passo-318.md`.
- **Primitivos de AST → DEBT-58** (não element-shaped): `MathSequence` (21),
  `MathText` (50), `MathIdent` (108), `Sequence` (208), `Empty` (116),
  `Block` (101); **em triagem**: `Space` (13, cola de texto), e os wrappers
  `Styled` (58), `Boxed` (58), `Labelled` (55); **observação** (leaf, candidato
  à triagem): `Text` (40).

### Element-shaped restantes — 1 (o Lote 13, por largura)

(O Lote 12 P327 — bloco grid/table cell TableCell/Table/GridCell/Grid, 4
variantes, ~193 sites — saiu daqui.)

`Figure`89.

> **`Figure` (L13, o último element-shaped)**: ao fechar, **dispara o gatilho
> do DEBT-58** (triagem dos primitivos de AST — conversa de desenho, não lote).

### Estimativa

**1 element-shaped restante** (`Figure`) → **Lote 13**, o último. Conta: 61
migradas + 4 `Set*` + 11 (DEBT-58: 6 + Space + 3 wrappers + Text) + 1 restante
= 77. ✓
