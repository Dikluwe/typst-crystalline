# Passo 345 — relatório: o `==` de conteúdo virou semântico/morfológico (ADR-0107 exercida)

> **Resultado.** O `==` da **linguagem** sobre `Content`/`Value::Content` agora é
> **morfológico** (`Content::morph_canon` em `eval_binary_op`): compara texto, markup e
> estilo **semântico** (`*bold*`); ignora estilo de **render** (o `TextStyle` assado, o
> transporte β1, o `numbering_active` assado). Fecha o **Achado 2** na camada da
> linguagem (`it.body == [a]` casa). A tabela da Fase A saiu **inteiramente clara — sem
> AMBÍGUO** (a trava não parou). O `#[derive(PartialEq)]` do Rust ficou **intacto** (dois
> sistemas, ADR-0025). O `m1` (P341b) foi de **"a" → "b"**; o resíduo (`b`≠`c`) é o
> **guard**, fatia posterior. Suíte **2723** (lib) / **3242** (workspace) — 4 testes
> novos, **0 evoluídos**; lint **0/0**.

## C0 — base / pré-condição
HEAD pós-P344 (`6b61d27be`). **ADR-0107 confirmada NO `claude.md`** (secção L72 + linha
na tabela L117, não só no relatório). Suíte 2719/3238, lint 0/0, árvore limpa. Bate.

## C1 — gatilho β1 (§3a.8) resolvido pela via da linguagem
Anotado em `f_fronteira_e1.md §3a.8`: a igualdade-de-`Content` **virou requisito**
(`it.body == [a]`); o gatilho foi resolvido **pela linguagem, não pelo de-bake** — o `==`
morfológico trata o transporte β1 (`Styled` semanticamente vazio, só `custom`) como
**render/transparente**. O wrapper β1 **fica** (sem de-bake); o `PartialEq` do Rust segue
intacto.

---

## Fase A — tabela morfologia-por-variante (da fonte; vanilla como oráculo da semântica)

A ADR-0107 deu o exemplo; aqui está a **definição operacional**. Oráculo vanilla
(`typst 0.14.2`), medido: `[a]==[a]`→**true**; `[a]==[b]`→**false**; `strong[a]==[a]`→
**false**; `[*a*]==[a]`→**false**; `text(fill:red)[a]==[a]`→**false**;
`text(size:20pt)[a]==[a]`→**false**; numbering (ver abaixo) N1→**true**, N3→**false**.

| variante / campo | morfologia \| render | razão | `file:line` | vanilla |
|---|---|---|---|---|
| `Content::Text` — string | **morfologia** | o texto é o que o conteúdo é | `content.rs:122` | `[a]==[a]` true |
| `Content::Text` — `TextStyle` (assado) | **render → ignora** | estilo da chain assado no nó (heading bold, `#set text` ambiente); P343 #1; raiz do Achado 2. **Sempre** chain-baked: não há `text()` chamável que anexe estilo ao nó | bake `eval/mod.rs:310,320,368`, `markup.rs:118` | P342 `repr(it.body)=[a]` |
| `Content::Styled` — delta tipado (bold/italic) | **morfologia** | estilo **semântico** (`*bold*`/`_italic_`/`strong`/`emph`, ADR-0038); o `#show strong` o vê | `markup.rs:60,73`; `structural.rs:37,40` | `strong[a]!=[a]` false |
| `Content::Styled` — `delta.custom` (numbering β1) | **render → transparente** | transporte aditivo; desce no body | `mod.rs:402`; `style.rs` `push_custom` | N1 true |
| `Content::Styled` — outros delta (size/fill/weight/…) | **render** (não ocorre) | nenhum caminho de linguagem produz `Styled` com estes (`text()` não chamável; `#set` só muta a chain). Moot; classificado render para consistência com a chain | `eval_set_rule` `rules.rs:362,503` | (E/F: estilo **anexado** seria morfologia — mas não há caminho) |
| `HeadingElem.numbering_active` | **render → ignora** | assado da chain `#set heading(numbering:)`; **sem** caminho de arg explícito (`native_heading` **erra** — `= x` só) → corresponde só ao numbering-de-chain do vanilla | `heading.rs:33`; consumidor `layout:714`,`introspect:817` | **N1 true** (chain não entra no `==`) |
| `EquationElem.numbering_active` | **render → ignora** | idem (chain `#set math.equation`) | `equation.rs:29`; `layout:812`,`introspect:657` | paralelo N1 |
| `FigureElem.numbering` | **render → ignora** | assado da chain `#set figure(numbering:)` (`closures.rs:79`); sem arg explícito | `figure.rs:26`; `layout:860`,`introspect:412` | paralelo N1 |
| estrutural (level/body/sequence/listas/math/…) | **morfologia, recursa morfologicamente** | a forma de linguagem | (todas as variantes) | A/B |
| glyph de SmartQuote | **morfologia** | o glyph resolvido é o **conteúdo** (string de `Text`); lang-resolvido na criação mas o char é a morfologia | `mod.rs:317-338` | (borda; nota) |

**Sem AMBÍGUO.** As duas zonas de risco resolveram-se da fonte+oráculo:
1. **"render styles em `Styled"`** (size/fill): **não ocorre** — `text()` não é chamável
   em crist ("unknown variable: text") e `#set text` só muta a chain (P343). Logo o
   `TextStyle` do `Text` é **sempre** chain-baked (ignorável com segurança); o `Styled`
   só carrega bold/italic (semântico) ou custom (transporte).
2. **numbering**: vanilla **N1=true** (chain `#set` numbering **não** entra na igualdade)
   vs **N3=false** (arg explícito entra). Crist só tem o caminho de **chain** (sem
   `heading()` direto) → mapeia a N1 → **render, ignora**. Decidido pelo oráculo, não
   ambíguo.

A tabela **bate com a ADR-0107** → a trava não parou; o lote executou tudo.

---

## Fase B — o `==` morfológico (diff por estágio)

**Onde vive:** no caminho do `==` da **linguagem** (`eval_binary_op`), **não** no
`#[derive(PartialEq)]`.

- **C1 — `Content::morph_canon(&self) -> Content`** (`content.rs`, após `map_content`):
  forma canônica via `map_content` (transform total) que remove render — `Text` style →
  `TextStyle::default()`; `Styled` semanticamente vazio → transparente (desce no body);
  `Heading`/`Equation`/`Figure` → clona-e-zera `numbering_active`/`numbering` (preserva
  os demais campos). Morfologia (texto/markup/bold semântico) intacta.
- **`Styles::is_semantically_empty()`** (`style.rs`): typed-fields todos `None` (ignora
  `custom`) — distingue `Styled` semântico de transporte β1.
- **C2 — `eval_binary_op`** (`operators.rs`): braços `(Eq|Neq, Content(a), Content(b))`
  **antes** do wildcard → `a.morph_canon() == b.morph_canon()`. O wildcard
  `(Eq, a, b) => a == b` (PartialEq do Rust) fica para os demais tipos.
- **Prova de que o `PartialEq` do Rust ficou intacto:** nenhum `derive`/impl tocado; o
  teste `morfologia_eq_ignora_textstyle_assado` assevera **`assert_ne!(a_reg, a_bold)`**
  (Rust distingue o estilo) **e** `eval_binary_op(Eq, …) = true` (linguagem casa) no mesmo
  par — os dois sistemas coexistem. IndexMap/coleções/hash inalterados (suíte verde).

### Estágio E — evolução de testes
**0 testes evoluídos** (0 falhas na suíte): nenhum teste asseverava o `==` de conteúdo na
forma estrutural antiga (consistente com P341b — a divergência foi achada pelo binário,
não por unit test). **4 testes novos** (lock da semântica morfológica, `eval/tests.rs`):
`morfologia_eq_ignora_textstyle_assado` (Achado 2 + dois-sistemas), `…_distingue_texto`,
`…_estilo_semantico_e_morfologia` (bold é morfologia), `…_transporte_b1_transparente`.

### Estágio M — remedição do `m1` (P341b)
`#show heading: it => if it.body==[a] {[= b]} else if it.body==[b] {[= c]} else {it}` · `= a`:

| | vanilla | cristalino (P341b) | cristalino (P345) |
|---|---|---|---|
| `m1` | **c** | **a** | **b** |

O `==` morfológico fez o `m1` sair de "a" → "**b**" (a content-eq deixou de mascarar). O
resíduo **`b`≠`c`** é o **guard por-regra** (trunca no nível 1; o vanilla recursa até o
teto-64) — **mecanismo de render, fatia posterior** (P341b opção B). **Não** consertado
aqui; apontado.

### Estágio F — linhagem
`@updated 2026-06-17` nos 4 ficheiros editados; `--fix-hashes` (sincroniza os headers que
referenciam os 4 L0 tocados — `eval.md`/`content.md`/`style.md`/`f_fronteira_e1.md`;
demais ficheiros são só bump de `@prompt-hash`, 1 linha cada); **V7/V5 limpas**.

---

## Aceitação morfológica (ADR-0107 — não "o booleano bate")

Medido no binário crist (via `#show` MATCH/NOMATCH, pois `#(bool)` renderiza vazio):

1. **Mesma morfologia, render diferente → casa.** `it.body` de `= a` (bold assado) `== [a]`
   → **MATCH** (era NOMATCH). ✓
2. **Morfologia diferente → não casa.** `it.body(=a) == [b]` → **NOMATCH**; `[*x*] == [x]`
   → **NOMATCH** (estilo semântico É morfologia). ✓
3. **Estilo semântico permanece observável.** `it.body(=a) == strong[a]` → **NOMATCH**
   (bold é morfologia, não render); `#show strong` intacto (`rules.rs:113-127` inalterado). ✓

`it.body == [a]` casar é **consequência medida**, registrada como tal — não o critério.

---

## Verificação (gates)

```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): lib 2723 (2719 + 4 novos), workspace 3242 (3238 + 4).
  0 evoluídos (nenhum teste contradiz a morfologia). 4 novos listados.
lint: crystalline-lint . = 0 violations, 0 warnings (após --fix-hashes).
dois sistemas preservados: derive(PartialEq) do Rust sobre Content/Value inalterado
  (assert_ne! no teste + IndexMap/coleções/suíte verdes). Confirmado.
lente (tekt-cargo-dsm@98d8f9e): NÃO re-corrida — o diff prova delta-de-aresta ZERO
  (nenhum `use` novo em content.rs/style.rs/operators.rs; os `use` de eval/tests.rs são
  #[cfg(test)], fora do grafo de produção). Baseline 219|676|[90,4]|66|0 preservado por
  construção (content→elements 66, elem→elem 0, ciclos [90,4]).
perf: morph_canon é O(n) no tamanho da árvore (uma passagem map_content + clone) por
  operando, mesma ordem do `==` estrutural que alimenta; o `==` é caminho frio (não
  layout). Sem regressão observável na suíte (2723 em 0,40s, igual ao baseline). Não
  estimado em µs (não é hot path; medição micro seria ruído).
```

---

## Mapa de filtro (campo) — duas notas

1. **Lugar lógico desta fatia:** o `==` morfológico mora junto da definição de igualdade
   da linguagem (ADR-0025, `eval_binary_op`), como a aplicação ao **conteúdo** do que os
   números (P343) já tinham mostrado; é a **ADR-0107 exercida pela primeira vez**.
2. **Filtragem futura (registrar, NÃO executar nesta branch):** a ADR-0107 deveria ser uma
   das **primeiras** ADRs (fundadora, ao lado do P329). **Reorganizar/renumerar** as ADRs
   para as fundadoras virem primeiro é **melhoria do projeto**, mas **fora desta branch**:
   mexe em todas as referências cruzadas (`ver ADR-00NN`) e quebraria citações no meio do
   F. Item nomeado do mapa de filtro, para a versão destilada; **não** é ação agora.

## Item aberto carregado
`content→elements → 0` — fora da fila, sem dono (as três saídas: reconciliar baseline /
marco pós-F-6 / registrar lacuna).

---

## Arquivos tocados (substantivos)
- **Código (L1):** `entities/content.rs` (`morph_canon`), `entities/style.rs`
  (`is_semantically_empty`), `rules/eval/operators.rs` (braços Content Eq/Neq),
  `rules/eval/tests.rs` (4 testes).
- **L0:** `rules/eval.md`, `entities/content.md`, `entities/style.md`,
  `entities/f_fronteira_e1.md §3a.8` (resolução do gatilho β1).
- **Mecânico:** bump de `@prompt-hash` nos ficheiros que referenciam os 4 L0 (1 linha cada).
