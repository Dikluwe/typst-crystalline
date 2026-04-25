# Passo 150 — `FrameDTO` + matriz P3 (primeiro número agregado)

**Série**: 150 (passo **substantivo**; primeira materialização
da medição de paridade após inventário 148 + arqueologia 149).
**Precondição**: Passo 149 encerrado; ADR-0058 + ADR-0059
criadas; inventário 148 actualizado (cobertura: 54%
user-facing, 72% arquitectural); 1113 tests; zero violations;
59 ADRs; 10 DEBTs abertos.

**Numeração**: 150 ocupa a posição reformulada pela série
paridade (§9 dos documentos de paridade, pós-149).

**Natureza**: passo **substantivo**. Toca:
- `lab/parity/`: novos `frame_dto.rs`, `report.rs`, novos
  tests, expansão do corpus, primeiro relatório.
- `lab/parity/Cargo.toml`: possíveis dev-deps novas.
- `00_nucleo/diagnosticos/typst-paridade-*.md`: notas finais
  cruzando com este passo.
- **L1/L2/L3/L4 cristalino**: **intactos**. `lab/parity/`
  está fora do workspace cristalino (per ADR original do
  Passo 9).

**Decisões já confirmadas pelo utilizador**:
- **Modos**: 3 (text_content + structural + geometric).
- **Geometric**: marcado **experimental** na matriz; não
  conta para %; mostra números brutos para calibração futura.
- **Corpus**: próprio existente (11 ficheiros) + 5-10
  ficheiros novos cobrindo features `implementado` visuais
  (heading, set/show, math básico, fill, tracking).
- **Relatório**: markdown vivo em
  `lab/parity/reports/latest.md` + cópia histórica em
  `lab/parity/reports/history/2026-04-25-passo-150.md`.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — operacionalização da
  medição.
- **ADR-0054** (perfil observacional graded) — tolerância
  geometric `experimental` reflecte que cristalino diverge
  estruturalmente em métricas (FixedMetrics vs
  FontBookMetrics) — não-erro.
- **ADR-0034** (diagnóstico obrigatório) — espírito cumprido
  por inventário 148 + arqueologia 149.

---

## Contexto

A pergunta original que motivou a série paridade ("em que
paridade estamos?") teve duas reformulações pré-150:

- **Passo 148** estabeleceu que a pergunta exige inventário
  prévio do que cristalino afirma cobrir; produziu cobertura
  declarada (54% / 72%).
- **Passo 149** formalizou 2 divergências sem ADR
  detectadas no inventário (Value::Type → ADR-0058;
  Value::Args → ADR-0059); reclassificou ambas como
  `implementado⁺`; reforçou denominador honesto.

**Passo 150 é a materialização**: produz o primeiro número
agregado de paridade observacional contra vanilla.

A decisão sobre `geometric` é arquitecturalmente material:
cristalino usa `FixedMetrics` (cada char ~0.6×size,
monoespaçado); vanilla usa `FontBookMetrics` via
`ttf-parser` (proporcional). Divergência geométrica em
posições é **estrutural**, não defeito. Marcar `geometric`
como **experimental** na matriz exprime esta realidade:
mede-se o gap mas não conta para a "%" agregada.

Esta classificação `experimental` é nomenclatura nova
introduzida neste passo; documentada em
`typst-paridade-definicoes.md` na §P3 e implementada na
matriz como linha separada com nota explicativa.

---

## Objectivo

Ao fim do passo:

1. **`lab/parity/src/frame_dto.rs`** materializado:
   - Tipo `FrameDTO` com campos para representar
     `PagedDocument` de forma neutra (texto extraído,
     estrutura, posições).
   - Conversões `From<&typst_core::PagedDocument>` (via
     pipeline cristalino) e `From<&typst::PagedDocument>`
     (via vanilla).
   - Tipo `LayoutTolerance` com campos `text_content: bool`,
     `structural: bool`, `geometric_pt: f64`,
     `geometric_experimental: bool`.

2. **`lab/parity/src/report.rs`** materializado:
   - Estruturas para representar a matriz agregada (por
     categoria + total).
   - Função `render_markdown(matrix) -> String`.
   - Função `write_history(report, passo: u32, data: &str) ->
     PathBuf`.

3. **`lab/parity/tests/layout_parity.rs`** materializado:
   - Itera o corpus filtrado pelo subconjunto
     `implementado` + `implementado⁺` + `parcial` do
     inventário 148.
   - Para cada ficheiro: roda pipeline cristalino + pipeline
     vanilla; converte ambos para `FrameDTO`; compara nos 3
     modos.
   - Reporta resultado por modo + ficheiro.

4. **Corpus expandido**: 5-10 ficheiros novos em
   `lab/parity/corpus/visual/` (subdir nova) cobrindo
   features `implementado` visuais. Inicialmente:
   - `heading-simples.typ` — `#heading[título]`.
   - `set-text-bold.typ` — `#set text(weight: 700)` + texto.
   - `set-text-fill.typ` — `#set text(fill: red)` + texto.
   - `set-text-tracking.typ` — `#set text(tracking: 1pt)`.
   - `set-text-size.typ` — `#set text(size: 16pt)`.
   - `show-strong.typ` — `#show strong: it => ...`.
   - `math-basico.typ` — `$x^2 + y^2$`.
   - `paragrafo-justificado.typ` — paragrafo longo (testa
     hyphenation).
   - `multi-font.typ` — 2 fonts diferentes (testa P146).
   - `lista-itens.typ` — não obrigatório se `enum` é
     `ausente` no inventário; **decidir em 150.1**.

   Cada um com `.typ.toml` adjacente (metadata: features
   exercidas, modo P3 esperado).

5. **Primeiro relatório agregado**:
   - `lab/parity/reports/latest.md` — markdown com matriz
     completa.
   - `lab/parity/reports/history/2026-04-25-passo-150.md` —
     cópia imutável.
   - Matriz tem 4 colunas: text_content, structural,
     geometric (experimental, números brutos), total.
   - Linhas por categoria (markup/math/code/visual) +
     totais.

6. **Definições de paridade actualizadas**:
   - `typst-paridade-definicoes.md` §P3 ganha definição da
     classe `experimental` (modo materializado mas não
     contado para %).
   - `typst-paridade-plano-medicao.md` §9 actualizado:
     entrada para Passo 150 = "matriz primeira" (era
     "Implementar `frame_dto.rs`").

7. **Possível abertura de DEBT**: se o relatório revelar
   dado relevante (ex: text_content < 50%; structural < 30%;
   gap material entre cobertura declarada e medida),
   considerar DEBT-53 (ou próximo número) "Calibração de
   FrameDTO + corpus paridade". **Não compromisso**;
   abertura empírica.

8. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-150-relatorio.md`.

Este passo **não**:

- Toca código em L1/L2/L3/L4 cristalino.
- Calibra tolerâncias por ficheiro (decisão futura).
- Implementa P2 (`value_dto.rs`) ou P4 (`pdf_compare.rs`).
  Esses são Passos 151+.
- Importa corpus oficial de `lab/typst-original/tests/`.
- Resolve a tensão FixedMetrics/FontBookMetrics
  arquitecturalmente.
- Cria ADR sobre `experimental` (nomenclatura nova).
  Decisão de formalizar fica para depois se a frequência de
  uso justificar.
- Modifica `DEBT.md` excepto se 7. acima activar abertura
  empírica.

---

## Decisões já tomadas

1. **3 modos materializados** (text_content + structural +
   geometric).
2. **`geometric` marcado experimental** — números brutos sim,
   contagem para % não.
3. **Corpus**: 11 actuais + 5-10 novos visuais.
4. **Relatório**: markdown latest + cópia historic.
5. **`lab/parity/` fora do workspace cristalino** (preservado
   desde Passo 9).
6. **Vanilla como crate path-dep** (`typst` ou subset)
   referenciado de `lab/typst-original/` — confirmar em
   150.1.

## Decisões diferidas (resolvidas neste passo)

7. **Forma exacta de `FrameDTO`**:
   - Campos por modo: `text: Vec<String>` (uma per página);
     `structure: Vec<PageStructure>` com counts por tipo;
     `positions: Vec<Vec<(f64, f64)>>` para geometric.
   - Decisão final em 150.2 com base na inspecção das APIs
     de ambos os lados.

8. **Inclusão de `lista-itens.typ`** depende do estado de
   `enum`/`list` no inventário 148. Se `ausente`, fica fora
   do corpus filtrado por construção. Verificar em 150.1.

9. **Categorização do corpus na matriz**: existente é
   `markup/`, `math/`, `code/`. Novos são `visual/`. Testar
   se faz sentido **fundir** `visual/` com as existentes ou
   manter separado. Decisão default: manter separado.

10. **Vanilla `PagedDocument` vs cristalino `PagedDocument`**:
    confirmar que ambos os tipos têm método ou estrutura
    análoga para extracção. Se nomes diverem (provável),
    `From<&...>` em duas trait-implementations distintas.
    Decisão técnica em 150.2.

11. **Threshold mínimo de "passa" no relatório**: per linha
    da matriz, se `text_content` passa em <X% dos ficheiros,
    sinalizar visualmente. Decisão default: **sem threshold
    visual** no primeiro relatório; números brutos
    apresentados, interpretação humana.

12. **Erros de compilação/eval em ficheiros do corpus
    novos**: se algum ficheiro não compila em cristalino, é
    falha do passo (corpus inválido). Se compila em
    cristalino mas não em vanilla, ou vice-versa, regista-se
    como **divergência fundamental** na matriz (linha extra:
    `failed_to_run`).

---

## Escopo

**Dentro**:

- `lab/parity/src/frame_dto.rs` (novo).
- `lab/parity/src/report.rs` (novo).
- `lab/parity/tests/layout_parity.rs` (novo).
- `lab/parity/corpus/visual/` (subdir nova) com 5-10
  ficheiros + metadata `.typ.toml` adjacentes.
- `lab/parity/Cargo.toml` (possíveis dev-deps; provavelmente
  apenas).
- `lab/parity/reports/latest.md` (novo).
- `lab/parity/reports/history/2026-04-25-passo-150.md`
  (novo; cópia inicial idêntica a `latest.md`).
- Pequeno update em `typst-paridade-definicoes.md` §P3
  (classe `experimental`).
- Pequeno update em `typst-paridade-plano-medicao.md` §9.
- Possível abertura de 1 DEBT empiricamente.
- Relatório do passo.

**Fora**:

- L1/L2/L3/L4 cristalino.
- `00_nucleo/adr/` (sem ADR nova).
- `00_nucleo/DEBT.md` excepto abertura empírica de DEBT.
- README dos ADRs (sem mudança — sem ADR nova).
- `value_dto.rs`, `pdf_compare.rs`, tests P2/P4.
- Importação de corpus oficial vanilla.
- Calibração de tolerâncias por ficheiro.
- Renomeação ou refactor do `frame_dto.rs` proposto na §3
  dos documentos de paridade.

---

## Sub-passos

### 150.1 — Inventário pré-materialização

**A.1.1 — Confirmar estado actual de `lab/parity/`**:

```bash
ls -la lab/parity/src/ lab/parity/tests/ lab/parity/corpus/
ls -la lab/parity/reports/ 2>/dev/null
cat lab/parity/Cargo.toml
```

Confirmar deps actuais; identificar se `typst` (vanilla
crate) já está acessível via `path = "../typst-original/..."`.

**A.1.2 — Confirmar API de `PagedDocument`**:

```bash
grep -rn "pub struct PagedDocument\|pub fn pages\|pub items" \
  01_core/src/entities/ \
  lab/typst-original/crates/typst-layout/src/ 2>/dev/null
```

Registar:
- Forma do `PagedDocument` em cristalino e vanilla.
- Métodos públicos para iterar pages → items.
- Forma do `FrameItem` em ambos.

**A.1.3 — Confirmar features de `lista-itens` (decisão
diferida 8)**:

```bash
grep -rn "enum\|list\|ListElem\|EnumElem" 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
```

Se `enum`/`list` está `ausente`, ficheiro **não entra** no
corpus. Se `parcial` ou `implementado⁺`, entra.

**A.1.4 — Estado das categorias no corpus actual**:

```bash
ls lab/parity/corpus/markup/ lab/parity/corpus/math/ lab/parity/corpus/code/
```

Contar ficheiros; confirmar 11 total. Decidir se `visual/`
fica como subdir nova ou se ficheiros novos vão para
`markup/` e `math/` existentes. **Decisão default**:
subdir nova `visual/`.

### 150.2 — `frame_dto.rs`

**Forma proposta** (a refinar em código):

```rust
//! FrameDTO neutro para comparação entre cristalino e vanilla.
//! Materializa o modo P3 dos documentos de paridade.

#[derive(Debug, Clone, PartialEq)]
pub struct FrameDTO {
    pub pages: Vec<PageDTO>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageDTO {
    pub text: String,                   // text_content mode
    pub items: Vec<ItemDTO>,            // structural mode
    pub item_positions: Vec<(f64, f64)>, // geometric mode
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemDTO {
    Text,    // FrameItem::Text
    Group,
    Glyph,
    Line,
    Image,
    Shape,
    Other(String),  // catch-all com nome do tipo vanilla se diverir
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutTolerance {
    pub text_content: bool,
    pub structural: bool,
    pub geometric_pt: f64,
    pub geometric_experimental: bool,
}

impl Default for LayoutTolerance {
    fn default() -> Self {
        Self {
            text_content: true,
            structural: true,
            geometric_pt: 5.0,
            geometric_experimental: true,
        }
    }
}

impl FrameDTO {
    pub fn from_cristalino(doc: &typst_core::PagedDocument) -> Self;
    pub fn from_vanilla(doc: &typst::layout::PagedDocument) -> Self;

    /// Compara dois DTOs respeitando os modos da tolerância.
    /// Devolve `Vec<ModeResult>` — um por modo (text/structural/geometric).
    pub fn compare(&self, other: &Self, t: LayoutTolerance) -> Vec<ModeResult>;
}

#[derive(Debug)]
pub enum ModeResult {
    TextContent { passed: bool, divergent_pages: Vec<usize> },
    Structural { passed: bool, mismatches: Vec<StructuralMismatch> },
    Geometric { experimental: true, max_dx: f64, max_dy: f64, mean_dx: f64, mean_dy: f64 },
}
```

Notas:
- `Geometric.experimental: true` é **literal** — o tipo
  exprime que este modo é experimental por construção.
  Pode ser refactorado se calibração futura promover para
  `production`.
- Conversões `from_cristalino` e `from_vanilla` requerem que
  ambos os crates tenham `PagedDocument` exposto. Confirmar
  em 150.1.A.2.

### 150.3 — `report.rs`

```rust
//! Agregação de resultados em matriz markdown.

#[derive(Debug)]
pub struct ParityMatrix {
    pub categories: Vec<CategoryRow>,
    pub date: String,
    pub passo: u32,
}

#[derive(Debug)]
pub struct CategoryRow {
    pub name: String,           // "markup", "math", "code", "visual"
    pub total_files: usize,
    pub text_content_passed: usize,
    pub structural_passed: usize,
    pub geometric_avg_dx: Option<f64>,
    pub geometric_avg_dy: Option<f64>,
    pub geometric_pages_within_5pt: usize,  // contagem para calibração
}

impl ParityMatrix {
    pub fn render_markdown(&self) -> String;
    pub fn write_latest(&self) -> Result<PathBuf, std::io::Error>;
    pub fn write_history(&self) -> Result<PathBuf, std::io::Error>;
}
```

Output esperado de `render_markdown`:

```markdown
# Paridade — Passo 150 (2026-04-25)

## Matriz

| Categoria  | Total | text_content | structural | geometric (experimental) |
|------------|------:|-------------:|-----------:|:------------------------:|
| markup     |     N |    M (M/N)   |   M (M/N)  | dx=Xpt; dy=Ypt; <5pt: N% |
| math       |   ... |      ...     |      ...   | ...                      |
| code       |   ... |      ...     |      ...   | ...                      |
| visual     |   ... |      ...     |      ...   | ...                      |
| **Total**  |     T |    P (P/T)   |   Q (Q/T)  | dx=Xpt; dy=Ypt           |

## Notas
- `geometric` é **experimental** ...
- Cobertura declarada (per inventário 148): user-facing 54%,
  arquitectural 72%.
- Esta matriz mede paridade observacional contra vanilla
  para o subconjunto declarado como suportado.
```

### 150.4 — `tests/layout_parity.rs`

```rust
//! P3 — Layout parity: 3 modos.

#[test]
fn corpus_completo_p3() {
    let corpus = read_corpus("lab/parity/corpus/");
    let mut matrix_builder = MatrixBuilder::default();

    for entry in corpus {
        let crist_doc = typst_core::compile(&entry.source);
        let vanilla_doc = typst::compile(&entry.source);
        let crist_dto = FrameDTO::from_cristalino(&crist_doc);
        let vanilla_dto = FrameDTO::from_vanilla(&vanilla_doc);
        let results = crist_dto.compare(&vanilla_dto, entry.tolerance);
        matrix_builder.record(entry.category, results);
    }

    let matrix = matrix_builder.build();
    let _path = matrix.write_latest().unwrap();
    let _hist = matrix.write_history().unwrap();
    eprintln!("{}", matrix.render_markdown());
    // Sem assert: o teste materializa a matriz; falhas individuais
    // são informação, não erros do harness.
}
```

Decisão: **harness não tem `assert!` global**. O passo é de
**medição**, não de **verificação**. Se um ficheiro falha
text_content em cristalino vs vanilla, isso é dado para a
matriz, não causa para `cargo test` falhar.

Trade-off documentado: ` cargo test` passa sempre nesta
infra (excepto se o harness falhar mecanicamente). Isto é
**desejado** — paridade não é regra arquitectural (per
ADR-0033 + ADR-0054). Se utilizadores quiserem `cargo
test` falhar quando paridade regride, isso é decisão
posterior.

### 150.5 — Corpus expandido

Decidido em 150.1.A.4. Default: subdir
`lab/parity/corpus/visual/` com:

- `heading-simples.typ` + `.typ.toml`
- `set-text-bold.typ` + `.typ.toml`
- `set-text-fill.typ` + `.typ.toml`
- `set-text-tracking.typ` + `.typ.toml`
- `set-text-size.typ` + `.typ.toml`
- `show-strong.typ` + `.typ.toml`
- `math-basico.typ` + `.typ.toml`
- `paragrafo-justificado.typ` + `.typ.toml`
- `multi-font.typ` + `.typ.toml`
- (`lista-itens.typ` se 150.1.A.3 confirmar `enum`/`list` é
  `parcial`+ no inventário; senão, não)

Cada `.typ.toml`:

```toml
features = ["text", "set", "heading"]   # exemplo
modo_p3 = "text_content"                # text_content único
notes = "..."
```

### 150.6 — Actualização dos documentos de paridade

**`typst-paridade-definicoes.md`** §P3 ganha:

```markdown
### Modo `experimental`

Nomenclatura introduzida no Passo 150. Modo cuja
implementação está materializada mas que **não conta para a
% agregada** porque diverge estruturalmente face ao vanilla
por razões arquitecturais conhecidas e aceites (ex:
`geometric` diverge porque cristalino usa `FixedMetrics`
enquanto vanilla usa `FontBookMetrics`).

Modo experimental devolve **números brutos** para
calibração futura. Promoção para `production` é decisão de
passo dedicado quando calibração for priorizada.
```

**`typst-paridade-plano-medicao.md`** §9 actualizado:

```diff
- 3. **Passo 150** — Implementar `frame_dto.rs` ...
+ 3. **Passo 150** — Implementado: `frame_dto.rs` +
+    `report.rs` + tests + corpus expandido + primeiro
+    relatório `lab/parity/reports/latest.md`. Matriz P3
+    materializada com modo `geometric` marcado
+    `experimental`.
```

(Renumerar 4/5/6 conforme.)

### 150.7 — Verificação automatizada

```bash
cd lab/parity && cargo test --test layout_parity
ls lab/parity/reports/latest.md lab/parity/reports/history/
cat lab/parity/reports/latest.md
```

Confirmar:
- Test corre sem panic.
- `latest.md` existe e tem matriz formatada.
- Ficheiro histórico criado com nome e data correctos.
- Conteúdo de `latest.md` ==`history/2026-04-25-passo-150.md`
  (cópia idêntica para o primeiro relatório).

### 150.8 — Avaliação empírica do output

Após `cargo test` correr, ler `latest.md` e avaliar:

1. **text_content**: se < 50%, é dado preocupante — abrir
   DEBT-NN para investigar.
2. **structural**: se < 30%, idem.
3. **geometric** (experimental): números brutos esperados
   altos (5-50pt). Registar para calibração futura.

Se algum threshold ultrapassado, **abrir DEBT** com
referência ao relatório:

```markdown
## DEBT-53 (ou próximo) — Paridade observacional baixa em P3

**Aberto em**: Passo 150 (2026-04-25).
...
```

Se thresholds passam confortavelmente, sem DEBT.

### 150.9 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-150-relatorio.md`.

Secções:
1. Sumário executivo (números agregados; 1-2 frases sobre
   o que mostraram).
2. Inventário pré-materialização (resultado de 150.1).
3. `frame_dto.rs` — assinatura final.
4. `report.rs` — assinatura final.
5. `tests/layout_parity.rs` — estratégia + número de
   ficheiros corridos.
6. Corpus expandido — lista final (com qualquer alteração
   face ao default).
7. Matriz primeira (cópia integral do `latest.md`).
8. Avaliação empírica (150.8) — DEBT aberto ou não.
9. Definições de paridade actualizadas — diffs.
10. Próximo passo: **151** (P2 — `value_dto.rs` +
    `tests/eval_parity.rs`).
11. Verificação final.

---

## Verificação

1. ✅ `lab/parity/src/frame_dto.rs` materializado com 3
   modos.
2. ✅ `lab/parity/src/report.rs` materializado com matriz +
   markdown.
3. ✅ `lab/parity/tests/layout_parity.rs` corre o corpus
   filtrado.
4. ✅ Subdir `lab/parity/corpus/visual/` com 5-10 ficheiros
   + `.typ.toml` adjacentes.
5. ✅ `lab/parity/reports/latest.md` produzido.
6. ✅ `lab/parity/reports/history/2026-04-25-passo-150.md`
   produzido (idêntico a `latest.md`).
7. ✅ `geometric` modo marcado **experimental** na matriz
   (linha separada com nota).
8. ✅ `text_content` e `structural` contam para %
   agregada; `geometric` não.
9. ✅ Definições de paridade ganham `experimental` na §P3.
10. ✅ §9 do plano de medição actualizado.
11. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4 cristalino.
12. ✅ Nenhuma ADR criada / revogada / revisada.
13. ✅ `crystalline-lint .` zero violations.
14. ✅ `cargo test --workspace --lib`: 1113 inalterado.
15. ✅ `cd lab/parity && cargo test --test layout_parity`
    corre.
16. ✅ DEBT-53 (ou próximo) aberto se thresholds 150.8 o
    activarem; senão, não.
17. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Matriz primeira existe e tem números reais.
2. Modo `geometric` separado de `text_content`/`structural`.
3. Corpus filtrado pelo subconjunto declarado pelo
   inventário 148.
4. Documentos de paridade actualizados.
5. Próximo passo (151) tem âncora documental clara.
6. Sem código tocado em L1/L2/L3/L4 cristalino.
7. Relatório do passo escrito.

---

## O que pode sair errado

- **Vanilla `PagedDocument` não acessível**: `lab/parity/`
  importa `typst` como path-dep para crate vanilla. Se a
  versão do vanilla em `lab/typst-original/` não expõe
  `PagedDocument` no namespace esperado, ajustar
  importação. Se inviável, registar e pausar — pode exigir
  análise da estrutura do `lab/typst-original/`.

- **Tipo `PagedDocument` cristalino diverge mais do que
  esperado de vanilla**: ex, vanilla tem `Document` em vez
  de `PagedDocument`; ou estrutura interna é diferente de
  forma material. `From<&...>` adapta com perda de
  informação onde necessário; documentar adaptações no
  relatório.

- **Compilação de ficheiros do corpus falha em vanilla**:
  ex, vanilla precisa de `<world>` setup mais complexo
  (fontes embutidas, etc.). Reusar setup já presente em
  `lab/parity/tests/parse_parity.rs` (Passo 9) se possível;
  caso contrário, replicar setup.

- **Ficheiros do corpus não compilam em cristalino**: se um
  dos novos ficheiros usa feature que cristalino tem como
  `parcial` mas não suficientemente para compilar, **remover
  do corpus** ou marcar `expected_to_fail` no `.typ.toml`.
  Ajustar contagens.

- **Medição revela cobertura observacional muito menor que
  declarada**: ex, cobertura declarada `implementado⁺` é 75
  features, mas só 30 passam text_content na matriz.
  Diferença é informação útil — abre DEBT para investigar.
  **Não é falha do passo**.

- **`geometric` produz números absurdos** (e.g., dx=1000pt):
  esperado pela divergência estrutural FixedMetrics vs
  FontBookMetrics. Documentar. Se números são tão
  divergentes que o modo não tem valor diagnóstico,
  considerar removê-lo nesta iteração — mas decisão é
  contra a tua escolha original; pausar se acontecer.

- **`cargo test --test layout_parity` falha por causa
  diferente de paridade** (ex: panic na conversão DTO):
  bug do harness. Corrigir antes de gerar relatório. **Não
  liberar relatório se harness não corre limpo**.

- **`lab/parity/Cargo.toml` precisa de novos dev-deps**:
  candidatos: `walkdir` (já presente segundo relatório
  147), `pretty_assertions` (já presente). Se nada novo,
  zero diff em `Cargo.toml`. Se algo novo (improvável),
  registar como cust adicional.

- **Tensão `lab/parity/` fora do workspace e
  `crystalline-lint`**: lint actua sobre o workspace
  cristalino; `lab/parity/` está fora. **Sem conflito**.
  Confirmar.

- **Cópia historic é trivial mas pode falhar por path**:
  garantir que `lab/parity/reports/history/` existe antes
  de escrever (criar com `fs::create_dir_all`).

- **Conflito de feature filter** entre o corpus actual
  (parse_parity) e o filtro inventário-148: alguns
  ficheiros existentes podem testar features `ausente` no
  inventário (improvável dado que parse não exige
  consumer activo, mas verificar). Se acontecer, **excluir
  do corpus de layout** (são para parse, não para layout).
  Confirmar em 150.1.

- **Relatório `latest.md` é gerado mas tem entrada
  vazia para alguma categoria** (ex: nenhum ficheiro
  visual passa structural). Mostrar `0/N` com nota.
  Não pôr `-` ou esconder.

---

## Notas operacionais

- **Reformulação 3 da série paridade**: 148 (inventário) +
  149 (arqueologia) + 150 (matriz). Cada uma justificada
  por descoberta de pré-condição. **Padrão estabelecido**:
  reformulação por descoberta é regra, não excepção, em
  projecto de longa duração.

- **`experimental` como nomenclatura nova**: documentada
  apenas em `definicoes.md` (P3). Se a frequência de uso
  aumenta (P2/P4 também terão modos experimentais?),
  considerar promoção para vocabulário canónico do projecto
  via ADR — fora deste passo.

- **Modelo: substantivo análogo a 140B/141/144/146**.
  Diferente de administrativo (P145/P147/P148) ou
  arqueológico (P149). Aplica padrão "tudo-num-passo"
  precedido por dois passos de contexto (148 + 149) — modelo
  de "diagnóstico-primeiro distribuído por múltiplos
  passos".

- **Sem ADR criada**. ADR-0033 + ADR-0054 já cobrem o quê
  conta como paridade. ADR específica sobre infra de
  medição é candidato condicional pós-150 se utilidade
  arquitectural for visível ao longo de 151+/152+.

- **DEBT-53 é candidato empírico**. Se aberto, é por
  resultado da medição, não por design. Coerente com
  política do projecto: DEBT regista trabalho real, não
  expectativa.

- **`cargo test` continua a passar inalterado em workspace
  cristalino** (1113). Os tests de paridade vivem em
  `lab/parity/` que é workspace separado. Esta separação é
  estratégica — paridade não bloqueia desenvolvimento de
  L1/L2/L3/L4.

- **Pós-150**: o utilizador tem o **número que pediu** —
  matriz com 4 colunas (text_content/structural/geometric
  experimental/total). Cobertura declarada (54%/72%) e
  paridade observacional (números 150) lado-a-lado dão a
  resposta substantiva à pergunta original.

- **Próximas reformulações antecipáveis**:
  - 151 (P2): `Value` em vanilla diverge de cristalino em
    18 vs 30 variants. ValueDTO terá de mapear ambos
    cuidadosamente. Pode requerer arqueologia análoga a
    149 para variants em scope-out.
  - 152 (P4): `pdf_compare` Opção B é textual.
    Implementação não exige dependências externas
    (`pdftoppm`/`mupdf`); Opção A fica para passo
    posterior dedicado.

- **Histórico `2026-04-25-passo-150.md`** é o primeiro
  ficheiro em `history/`. Convenção de nomes:
  `YYYY-MM-DD-passo-NNN.md`. Permite ordenação cronológica
  via `ls`. Estabelece padrão para evoluções futuras.
