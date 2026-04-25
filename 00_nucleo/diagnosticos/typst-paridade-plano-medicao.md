# Plano de medição de paridade com o vanilla Typst

**Status**: `PROPOSTO`
**Data**: 2026-04-24

> **Revisto no Passo 147 (2026-04-24)**: o documento original
> foi escrito durante a fase precoce do projecto (referências
> a "Passos 19–21" eram contemporâneas). O projecto está
> agora em Passo 146; pipeline end-to-end estável; DEBT-1
> fechado por ADR-0054 (perfil observacional graded).
> Decisões conceptuais do documento (4 níveis P1–P4, DTOs
> propostos, modos de comparação) **permanecem válidas** e
> são alvo de materialização em Passo 148. Discrepâncias de
> contexto temporal foram corrigidas neste passo. Vocabulário
> de status migrado de `**Estado**: PROPOSTO` para o canónico
> `**Status**: \`PROPOSTO\`` (P84.8g + P145).

**Contexto**: o projecto tem **146 passos executados**.
Pipeline end-to-end (parse → eval → layout → export PDF) está
**estável** desde o Passo 22+; multi-font, hyphenation e
consumer integral de `StyleDelta` (DEBT-52: 6/8 gaps
materializados; gap 7 fechado em 144) existem. **DEBT-1
fechado** no Passo 142 com cumprimento de ADR-0054 (perfil
observacional graded). 57 ADRs vigentes; 10 DEBTs abertos.
**ADR-0033** (paridade funcional) e **ADR-0054** (perfil
observacional graded) governam o que "paridade" significa.

A pergunta "em que percentual de paridade estamos?" continua
sem **resposta numérica** porque o projecto continua a medir
testes acumulados, não paridade comparativa: a infra que
permite responder ainda não foi materializada. A estimativa
inicial era de ~10 passos no ADR-0001; o número actual cresceu
para 146 porque a análise revelou que `typst-library` tem de
ser estratificada antes de `eval()` poder migrar (ADR-0016,
ADR-0026).

Este documento define como passar a medir. Materialização da
infra é o **Passo 148**.

---

## 1 — O que "paridade" significa neste projeto

Há dois sentidos de paridade. Distingui-los é a primeira decisão.

### Paridade de implementação

Replicar a estrutura interna do original: mesma vtable, mesmas proc macros, mesmos tipos. **Este projeto rejeita esta paridade** — está documentado em ADR-0026 (`Content` como enum vs vtable do original).

### Paridade funcional

Para o mesmo input Typst, o cristalino produz o mesmo output observável que o original. É o que importa medir. Subdivide-se em quatro níveis, alinhados com o pipeline do compilador:

| Nível | Compara | Granularidade |
|-------|---------|---------------|
| P1 — Parse | árvore sintáctica | `CompactNode` (já existe no Passo 9) |
| P2 — Eval | `Value`/`Module` | DTO comparável (não existe ainda) |
| P3 — Layout | `Frame`/`FrameItem` posicional | DTO geométrico tolerante a métricas |
| P4 — Export | bytes do PDF / pixels do PNG | tolerância configurável |

Cada nível só pode ser medido depois do anterior estar verde para o mesmo input.

---

## 2 — O que já existe e o que falta

### Existe

- `lab/parity/` com `Cargo.toml` separado do workspace
  cristalino (verificado em 147.1: `lab/parity/src/main.rs` +
  `compact.rs`; `tests/parse_parity.rs`; corpus em
  `markup/`/`math/`/`code/` totalizando 11 ficheiros).
- `CompactNode` DTO para comparação estrutural (Passo 9 v2/v3).
- `assert_paridade()`, `assert_paridade_math()`,
  `assert_paridade_code()` para parsing.
- Corpus mínimo em `lab/parity/corpus/` (11 ficheiros `.typ`).
- Runner interactivo `cargo run --manifest-path
  lab/parity/Cargo.toml -- <input>`.
- **Pipeline end-to-end em L1 + L3** (Passos 19+): `parse →
  eval → layout` produz `PagedDocument` em L1
  (`01_core/src/rules/`); `compile_to_pdf_bytes` em L3
  (`03_infra/src/pipeline.rs`) acrescenta export PDF e devolve
  `Vec<u8>`.
- **Export PDF estável** com Helvetica fallback + CIDFont
  embedding (ADR-0027); **multi-font** per document (Passo
  146, ADR-0055 decisão 5); **hyphenation** integrada no
  algoritmo greedy de quebra de linha (Passo 144, ADR-0057;
  crate `hypher` em L1).
- **Consumer integral de `StyleDelta` para 9/10 campos**
  (DEBT-52 6/8 gaps fechados; `lang` parcialmente consumido
  via hyphenation, sem shaping). Faux-bold via stroke
  (Passo 139); tracking + leading materializados (137/138).
- **57 ADRs vigentes**; **DEBT-1 + DEBT-52 encerrados** no
  Passo 142.

### Falta

- DTO comparável para `Value`/`Module` (P2) — `value_dto.rs`
  em `lab/parity/src/`.
- DTO comparável para `Frame`/`FrameItem` com tolerância a
  diferenças de métricas de fonte (P3) — `frame_dto.rs`.
- Mecanismo de comparação PDF/PNG com tolerância (P4) —
  `pdf_compare.rs`.
- Corpus expandido — actualmente cobre **parse** (markup +
  math + code), não cobre `semantic/` (P2) nem `visual/`
  (P3/P4).
- Tests P3 (`tests/layout_parity.rs`) e P4
  (`tests/export_parity.rs`).
- Métrica agregada: "X de Y inputs passam em P1, P2, P3, P4".
- Painel/relatório que mostre evolução da paridade ao longo
  dos passos (`reports/latest.md` + `reports/history/`).
- **Materialização concreta destes itens é o Passo 148**.

---

## 3 — Estrutura proposta

```
lab/parity/
  src/
    main.rs           ← runner interactivo (já existe)
    compact.rs        ← CompactNode para P1 (já existe)
    value_dto.rs      ← NOVO — DTO de Value/Module para P2
    frame_dto.rs      ← NOVO — DTO geométrico para P3
    pdf_compare.rs    ← NOVO — comparação de PDF e PNG para P4
    report.rs         ← NOVO — agregação e geração de relatório
  tests/
    parse_parity.rs   ← P1 (já existe)
    eval_parity.rs    ← NOVO — P2
    layout_parity.rs  ← NOVO — P3
    export_parity.rs  ← NOVO — P4
  corpus/
    markup/, math/, code/   ← já existem para P1
    semantic/               ← NOVO — corpus para P2 (input → valor esperado)
    visual/                 ← NOVO — corpus para P3/P4 (input + PDF de referência)
  reports/
    latest.md         ← NOVO — relatório actual
    history/          ← NOVO — relatórios passados, um por passo
```

A crate `lab/parity` continua fora do workspace cristalino — não pode importar nem o workspace cristalino tem de a conhecer.

---

## 4 — Os quatro níveis em detalhe

### P1 — Parse (já implementado)

Entrada: string ou ficheiro `.typ`.
Comparação: `CompactNode` cristalino vs `CompactNode` original (spans removidos estruturalmente).
Critério de igualdade: igualdade exacta de árvores.

Métrica actual: `corpus_completo` test passa = todos os ficheiros do corpus produzem árvores idênticas.

Acção: nenhuma — está pronto. Apenas expandir o corpus (ver secção 5).

### P2 — Eval

Entrada: ficheiro `.typ` + nome de variável a inspeccionar (`#let r = ...`).
Comparação: `Value` cristalino vs `Value` original, ambos convertidos para `ValueDTO`.

`ValueDTO` necessário porque os tipos de `Value` em cada lado são tipos Rust distintos:

```rust
#[derive(Debug, PartialEq)]
pub enum ValueDTO {
    None,
    Auto,
    Bool(bool),
    Int(i64),
    Float(OrderedFloat),  // wrapper que aceita comparação por bits
    Str(String),
    Array(Vec<ValueDTO>),
    Dict(Vec<(String, ValueDTO)>),  // ordenado, não HashMap
    Func(String),         // nome ou hash — funções não comparam por igualdade
    Content(ContentDTO),  // ver P3
    Type(String),
    Other(String),        // catch-all com nome do tipo
}
```

Float merece atenção: o Typst trata NaN especificamente (ver Passo 14 diagnóstico). `OrderedFloat` deve preservar essa semântica. Decisão a tomar quando P2 for implementado.

Métrica: `% de inputs do corpus semantic/ onde ValueDTO(cristalino) == ValueDTO(original)`.

### P3 — Layout

Entrada: ficheiro `.typ`.
Comparação: `Frame`/`FrameItem` em cada página.

Problema: o cristalino usa `FixedMetrics` (monoespaçado) enquanto o original usa `FontBookMetrics` (proporcional, ttf-parser). As posições absolutas vão divergir.

Solução: tolerância configurável por dimensão.

```rust
pub struct LayoutTolerance {
    pub absolute_pt: f64,   // ex: 5.0pt — diferença máxima de coordenadas
    pub item_count: usize,  // ex: 0 — número de itens deve bater exactamente
    pub page_count: usize,  // ex: 0 — número de páginas deve bater
    pub text_content: bool, // true — texto extraído deve bater exactamente
}
```

`text_content == true` é o critério mais importante: ignora geometria, exige que o texto extraído de cada `Frame` seja igual em ordem e conteúdo. Isto valida que o pipeline de eval está a produzir o conteúdo certo, mesmo que o posicionamento ainda não esteja afinado.

Métrica: três percentagens
- % com paridade de conteúdo (texto bate)
- % com paridade estrutural (mesmo número de páginas e itens)
- % com paridade geométrica (posições dentro da tolerância)

### P4 — Export

Entrada: ficheiro `.typ`.
Comparação: PDF gerado pelos dois pipelines.

PDFs raramente são byte-idênticos (timestamps, ordem de objectos, IDs gerados). Comparação por bytes é inútil. Há duas opções:

**Opção A — Renderizar e comparar PNG**

Cada PDF é renderizado para PNG via uma ferramenta externa (`pdftoppm` do poppler ou `mupdf`). Os PNGs são comparados pixel a pixel com tolerância.

```rust
pub struct VisualTolerance {
    pub max_pixel_diff: u8,        // 0–255, tolerância por canal
    pub max_diff_ratio: f64,       // 0.0–1.0, fracção máxima de pixels divergentes
}
```

Vantagem: mede o que o utilizador efectivamente vê.
Desvantagem: depende de ferramentas externas; CI tem de as ter instaladas.

**Opção B — Comparação textual normalizada**

Extrair texto e posições de cada PDF, ignorar metadados, comparar como em P3 mas a partir do PDF.

Vantagem: sem dependências externas.
Desvantagem: não detecta diferenças de rendering (cores, fontes, alinhamento real).

**Recomendação**: começar com B (mais simples, sem deps externas), adicionar A quando o pipeline visual estabilizar.

Métrica: % de inputs do corpus visual/ onde o output passa na tolerância configurada.

---

## 5 — Corpus

O corpus actual tem 11 ficheiros, suficientes para P1 mas demasiado pequenos para P2/P3/P4. Duas fontes possíveis:

### Corpus oficial do Typst

`lab/typst-original/tests/` contém testes de integração do compilador original — centenas de ficheiros `.typ` com PNG/PDF de referência. Reutilizar este corpus.

Vantagem: cobertura ampla, mantida pelos autores do Typst.
Desvantagem: muitos casos vão falhar inicialmente — o cristalino só implementa Empty/Text/Space/Sequence em Content (ADR-0026). Filtrar o corpus por features suportadas.

### Corpus próprio cristalino

Crescer o corpus à medida que cada feature é implementada. Cada passo adiciona casos correspondentes.

**Recomendação**: ambos. O corpus oficial dá medida bruta de "quanto do Typst funciona". O corpus próprio dá medida refinada de "as features que dizemos suportar funcionam".

### Categorização do corpus

Cada ficheiro do corpus tem metadados (em ficheiro `.toml` adjacente ou cabeçalho de comentário no `.typ`):

```toml
# corpus/visual/heading-simple.typ.toml
features = ["text", "heading"]
levels   = ["P1", "P2", "P3"]    # P4 ainda não — sem fonte real
notes    = "Content::Heading não implementado — espera-se falha em P3"
```

Permite filtrar: `cargo run --bin parity-runner -- --features=text --level=P3`.

---

## 6 — Métrica agregada

Um único número de "% de paridade" é enganador. O relatório usa uma matriz:

```
                  P1 Parse    P2 Eval    P3 Layout   P4 Export
markup/            100%        100%        100%        N/A
math/              100%        N/A         N/A         N/A
code/              100%        85%         60%         N/A
semantic/          N/A         92%         N/A         N/A
visual/            100%        90%         50%         30%
─────────────────
TOTAL              100%        90%         55%         30%
                   (88/88)     (45/50)     (22/40)     (6/20)
```

`N/A` significa que esse nível não se aplica àquele subcorpus (ex: ficheiros de `math/` não têm output visual relevante a este ponto).

O TOTAL não é uma média ponderada — é o percentual de inputs que **passam todos os níveis aplicáveis**. É a métrica mais honesta.

---

## 7 — Ligação com a sequência de passos

A pergunta do utilizador era "em que paridade estamos?". A
resposta hoje continua a ser "não medimos comparativamente".
Para essa resposta passar a existir, a sequência futura de
passos precisa de incluir materialização da infra:

| Quando | Estado / Acção |
|--------|----------------|
| **Estado em 2026-04-24 (Passo 146)** | Pipeline end-to-end estável; eval, layout, export PDF (multi-font + hyphenation) materializados. **Apenas P1 está medido** (parse parity via `CompactNode`). |
| **Passo 148** (próximo) | Materializar `frame_dto.rs` + `tests/layout_parity.rs` (`text_content` mode). Gerar primeiro relatório agregado em `lab/parity/reports/latest.md`. |
| **Passo 149+** (futuro) | Materializar `value_dto.rs` + `tests/eval_parity.rs` (P2); expandir `corpus/semantic/`. |
| **Passo 150+** (futuro) | Materializar `pdf_compare.rs` + `tests/export_parity.rs` (P4). Opção B (textual) primeiro; Opção A (visual) quando dependências CI estiverem decididas. |
| A cada N passos | Gerar relatório histórico em `lab/parity/reports/history/`. |

Numeração 148/149/150 é **indicativa**, não compromisso. Se
materialização cresce em complexidade, ramifica em sub-passos
(ex: 148A/148B). A primeira acção concreta é **P3** — o
pipeline já chega lá; teste end-to-end existe via
`compile_to_pdf_bytes` desde Passo 113.

---

## 8 — O que este plano não decide

- Tolerância numérica concreta para P3 e P4 — depende de medições empíricas que ainda não foram feitas
- Se o corpus oficial do Typst entra no repositório ou é referenciado por path em `lab/typst-original/tests/`
- Como integrar o relatório no CI — pode ser GitHub Actions, pode ser apenas relatório manual; decidir quando o pipeline P3 estiver implementado
- Se a métrica entra no `crystalline-lint` como nova violation (ex: V15 — ParityRegression). Provavelmente não — paridade não é regra arquitectural.

---

## 9 — Próximas acções concretas

Pós-Passo 147 (este — actualização documental). A ordem
permanece a do documento original; apenas as condições
temporais foram actualizadas para o estado real de
2026-04-24.

1. **Passo 148** — Implementar `frame_dto.rs` com
   `LayoutTolerance` e modo `text_content=true`. Adicionar
   `tests/layout_parity.rs` com o corpus actual (11
   ficheiros; todos devem passar em conteúdo dado que
   `eval` + `layout` cristalinos estão estáveis). Gerar
   primeiro relatório agregado em
   `lab/parity/reports/latest.md` — **número-base** que o
   utilizador pediu.
2. **Passo 149+** — Implementar P2 (`value_dto.rs` +
   `tests/eval_parity.rs`) quando expansão do corpus
   `semantic/` for priorizada. `eval()` já suporta as
   features que faltavam à data do documento original
   (Passo 17 já foi; `#set`/`#show` activos).
3. **Passo 150+** — Implementar P4 Opção B (textual) com
   `pdf_compare.rs` + `tests/export_parity.rs`. `export_pdf()`
   está estável desde Passo 22+; multi-font (Passo 146).
   Opção A (visual) quando dependências CI
   (`pdftoppm`/`mupdf`) forem decididas.
4. **Decisão sobre corpus**: oficial vs próprio vs ambos.
   Documentar em ADR separada se a decisão for não-trivial.
   Recomendação preliminar: mix (ambos), com filtro por
   features. Alvo: durante Passo 148 ou imediatamente antes.

A partir do **Passo 148**, o utilizador passa a ter o número
que pediu — em formato de **matriz**, não de percentual único.
