# Passo 150 — Relatório (`FrameDTO` + matriz P3 baseline)

**Data**: 2026-04-25
**Natureza**: passo **substantivo** em `lab/parity/` (workspace
separado fora do cristalino). **Zero código tocado em L1, L2,
L3, L4 cristalino**. **Zero ADRs criadas / revogadas /
revisadas**. **1 DEBT aberto** (DEBT-53) por resultado empírico.
**Precondição**: Passo 149 encerrado; 59 ADRs; inventário 148
+ arqueologia 149; 1113 tests cristalino; zero violations.

---

## 1. Sumário executivo

Passo 150 entregou a **infraestrutura completa de medição P3**
em `lab/parity/`:

- `lab/parity/src/frame_dto.rs` — `FrameDTO` neutro com 3
  modos (text_content, structural, geometric); conversão
  cristalino real, vanilla stub.
- `lab/parity/src/report.rs` — `ParityMatrix` + render
  markdown + write latest/history.
- `lab/parity/tests/layout_parity.rs` — harness do corpus.
- `lab/parity/corpus/visual/` — **9 ficheiros novos** com
  metadata `.typ.toml` (heading, set/show, math, hyphenation,
  multi-font).
- `lab/parity/reports/latest.md` + `history/2026-04-25-passo-150.md`
  — primeira matriz agregada.

**Matriz produzida** (cristalino-only baseline): **19/19
ficheiros compilam** em cristalino sem panic (categorias:
markup 6, math 2, code 2, visual 9). Colunas `text_content`,
`structural`, `geometric` ficam `N/A` — comparação contra
vanilla movida para **DEBT-53 aberto**.

**Tests cristalino**: inalterados em 1113. **Tests
`lab/parity`**: `corpus_completo_p3` corre limpo (sem panic;
gera matriz).

**Pergunta original respondida (parcialmente)**: o número que o
utilizador pediu existe agora em forma de matriz; o
**preenchimento real** das colunas requer DEBT-53.

---

## 2. Inventário pré-materialização (sub-passo 150.1)

### 2.1 — Estado de `lab/parity/`

```
lab/parity/
├── Cargo.toml                      (deps: typst-syntax + typst-core)
├── corpus/
│   ├── code/      (2 ficheiros)
│   ├── markup/    (7 ficheiros, 1 = error.typ excluído)
│   └── math/      (2 ficheiros)
└── tests/parse_parity.rs
```

`typst` (vanilla compilador inteiro) **não estava** em
deps — apenas `typst-syntax` (sub-crate de parse). Confirmação
empírica.

### 2.2 — API do `PagedDocument`

- **Cristalino**: `typst_core::entities::layout_types::PagedDocument`
  com `pub pages: Vec<Page>` e `Page` com `pub items: Vec<FrameItem>`.
- **Vanilla**: `typst_layout::PagedDocument` exportado em
  `typst_layout::lib.rs:20` (`pub use self::document::{Page, PagedDocument}`).
  Estrutura interna divergente.

### 2.3 — Decisões diferidas resolvidas

- **Listas/enum**: inventário 148 classifica `enum`/`list`
  como `parcial`. Suficiente para entrar no corpus, mas
  decidiu-se **não criar** `lista-itens.typ` nesta iteração
  para manter foco em features `implementado⁺` claras. Pode
  ser adicionado num passo futuro.
- **Categorias**: subdir nova `visual/` mantida separada de
  `markup/`/`math/`/`code/` (per default da spec).
- **`from_vanilla`**: stub (`from_vanilla_stub`); integração
  real é DEBT-53.

### 2.4 — Setup vanilla é trabalho dedicado

Tentativa preliminar revelou que adicionar `typst` (vanilla)
como dep + setup de World adapter ultrapassaria o escopo
prático deste passo:

- Vanilla `typst_library::World` trait diverge de cristalino
  `typst_core::contracts::world::World` (assinaturas e
  semântica diferentes).
- Vanilla espera fonts embebidas via crate dedicada ou
  descobertas no sistema; setup difere de cristalino
  `SystemWorld::with_fonts`.
- Conversão de `typst_layout::PagedDocument` (vanilla) para
  `FrameDTO` exige inspecção da estrutura interna vanilla.

Decisão: **infraestrutura entregue, vanilla integration
movida para DEBT-53**. Coerente com spec §"O que pode sair
errado": "Se inviável, registar e pausar — pode exigir análise
da estrutura do `lab/typst-original/`."

---

## 3. `frame_dto.rs` — assinatura final

```rust
pub struct FrameDTO { pub pages: Vec<PageDTO> }

pub struct PageDTO {
    pub text:           String,        // text_content
    pub items:          Vec<ItemDTO>,  // structural
    pub item_positions: Vec<(f64, f64)>,  // geometric
    pub width:  f64,
    pub height: f64,
}

pub enum ItemDTO {
    Text, Group, Glyph, Line, Image, Shape, Other(String),
}

pub struct LayoutTolerance {
    pub text_content:           bool,
    pub structural:             bool,
    pub geometric_pt:           f64,
    pub geometric_experimental: bool,
}

impl FrameDTO {
    pub fn from_cristalino(doc: &typst_core::PagedDocument) -> Self;
    pub fn from_vanilla_stub() -> Self;       // DEBT-53
    pub fn compare(&self, other: &Self, t: LayoutTolerance) -> Vec<ModeResult>;
}

pub enum ModeResult {
    TextContent { passed: bool, divergent_pages: Vec<usize> },
    Structural  { passed: bool, mismatches: Vec<StructuralMismatch> },
    Geometric   {
        experimental: bool,
        max_dx: f64, max_dy: f64, mean_dx: f64, mean_dy: f64,
        within_tolerance: bool, sample_count: usize,
    },
}
```

**Notas**:
- `Geometric.experimental` é campo literal — exprime a
  classe explicitamente.
- `from_vanilla_stub` devolve `FrameDTO` vazio; deve ser
  substituído por `from_vanilla(&typst_layout::PagedDocument)`
  em DEBT-53.

---

## 4. `report.rs` — assinatura final

```rust
pub struct ParityMatrix {
    pub categories: Vec<CategoryRow>,
    pub date:       String,
    pub passo:      String,
    pub summary:    String,
}

pub struct CategoryRow {
    pub name:                String,
    pub total_files:         usize,
    pub compiled_ok:         usize,
    pub text_content_passed: Option<usize>,  // None = N/A
    pub structural_passed:   Option<usize>,
    pub geometric_max_dx:    Option<f64>,
    pub geometric_max_dy:    Option<f64>,
    pub geometric_mean_dx:   Option<f64>,
    pub geometric_mean_dy:   Option<f64>,
}

impl ParityMatrix {
    pub fn render_markdown(&self) -> String;
    pub fn write_latest(&self, base: &Path) -> std::io::Result<PathBuf>;
    pub fn write_history(&self, base: &Path) -> std::io::Result<PathBuf>;
}
```

`Option<usize>` em `text_content_passed`/`structural_passed`
permite distinguir "0/N (todos falharam)" de "N/A (modo não
medido)" — relevante enquanto vanilla integration está
pendente.

---

## 5. `tests/layout_parity.rs` — estratégia + corrida

**Estratégia**:

1. Lê corpus em `lab/parity/corpus/{markup,math,code,visual}/*.typ`
   (filtra `error*.typ` — fixtures parse-only).
2. Para cada ficheiro:
   - `compile_cristalino(src)` via `SystemWorld` (tempdir) +
     `eval_to_module_with_sink` + `introspect` + `layout`.
   - Em sucesso: `FrameDTO::from_cristalino(&doc)`; matriz
     incrementa `compiled_ok`.
   - Em falha: `eprintln!` com path + nome.
3. Constrói `ParityMatrix` com `summary` explicando baseline
   + DEBT-53.
4. Escreve `latest.md` + `history/2026-04-25-passo-150.md`.

**Sem `assert!` global** (per spec §150.4): o harness é
**medição**, não verificação. Falhas individuais entram na
matriz; `cargo test` passa sempre.

**Corrida**: `cargo test --test layout_parity` (em
`lab/parity/`) → 1 test passed; matriz gerada.

**Trade-off documentado**: spec autoriza esta escolha. Se
utilizadores quiserem `cargo test` falhar por regressão de
paridade, é decisão arquitectural posterior (não bloqueia
P150).

---

## 6. Corpus expandido

`lab/parity/corpus/visual/` (subdir nova) com **9 ficheiros**:

| Ficheiro | Features | Notas |
|----------|----------|-------|
| `heading-simples.typ` | heading, text | nível 1 + parágrafo |
| `set-text-bold.typ` | set, text.weight | faux-bold (Passo 139) |
| `set-text-fill.typ` | set, text.fill, rgb | `rgb(220,20,60)` literal (cristalino sem `red` constant) |
| `set-text-tracking.typ` | set, text.tracking | Passo 137; Tc operator |
| `set-text-size.typ` | set, text.size | Passo 30 |
| `show-strong.typ` | show, strong | Passo 103 |
| `math-basico.typ` | math, math.attach | superscripts |
| `paragrafo-justificado.typ` | set, text.lang, hyphenation | Passo 144 (ADR-0057) |
| `multi-font.typ` | set, text.font, multi-font | Passo 146 (ADR-0055 dec.5) |

Cada ficheiro com `.typ.toml` adjacente (features + modo_p3 +
notes).

**Ajuste durante materialização**: `set-text-fill.typ` foi
inicialmente escrito com `red` (constante de cor por nome) —
cristalino stdlib **não tem** constantes de cor por nome (só
`rgb`/`luma`). Substituído por `rgb(220, 20, 60)` literal.
**Sinaliza limitação real**: cristalino expõe construtores
mas não constantes de cor — coerente com inventário 148
("Foundations stdlib" — `cmyk`/`oklab` `ausentes`).

`lista-itens.typ` **não criado** (spec o tornava condicional;
omitido por foco).

---

## 7. Matriz primeira (cópia integral de `latest.md`)

```
# Paridade — Passo 150 (2026-04-25)

**Primeira matriz agregada (Passo 150)**. ... [summary] ...

## Matriz

| Categoria | Total | Compila (cristalino) | text_content | structural | geometric (experimental) |
|-----------|------:|---------------------:|-------------:|-----------:|:------------------------:|
| code      |     2 |                  2/2 |          N/A |        N/A |                      N/A |
| markup    |     6 |                  6/6 |          N/A |        N/A |                      N/A |
| math      |     2 |                  2/2 |          N/A |        N/A |                      N/A |
| visual    |     9 |                  9/9 |          N/A |        N/A |                      N/A |
| **Total** |    19 |                19/19 |          N/A |        N/A |                        — |
```

Notas no markdown explicam que `geometric` é experimental,
referenciam cobertura declarada do inventário 148 (54% / 72%),
e clarificam que vanilla integration → DEBT-53 a ser
materializado.

---

## 8. Avaliação empírica (sub-passo 150.8)

**Compilação cristalino**: 19/19 (100%). Todos os ficheiros do
corpus filtrado compilam sem panic — sinal positivo.

**Threshold spec 150.8**:
- `text_content < 50%` → abrir DEBT.
- `structural < 30%` → idem.

Como ambos estão `N/A` (vanilla integration pendente), os
thresholds não disparam por baixos números observados — disparam
**por ausência de medição**. Decisão: **abrir DEBT-53** para
endereçar a ausência (não a baixa cobertura).

**DEBT-53 aberto**:
- Título: "Integração de pipeline vanilla em `lab/parity` para
  medição P3".
- Aberto em: Passo 150 (2026-04-25).
- Plano: World adapter + setup duplo + `from_vanilla` real +
  populate matriz.
- Critério de fecho: matriz com números reais (substitui
  `N/A`).
- Nota de abertura adicionada ao topo de DEBT.md ("Total
  abertos: 10 → 11").

---

## 9. Definições de paridade actualizadas

### 9.1 — `typst-paridade-definicoes.md`

§P3 ganha sub-secção **"Classe `experimental` (introduzida no
Passo 150)"**:

- Modos cuja implementação está materializada mas que
  divergem estruturalmente face ao vanilla por razões
  arquitecturais conhecidas e aceites.
- Aplicação a `geometric`: cristalino `FixedMetrics` vs
  vanilla `FontBookMetrics`. Posições absolutas divergem por
  construção; tolerâncias `5pt`/`10pt` não cobrem.
- Devolve **números brutos** para calibração futura.
- Promoção para `production` é decisão de passo dedicado.
- ADR-0054 cobre.

### 9.2 — `typst-paridade-plano-medicao.md`

§9 actualizado: item 3 (Passo 150) ganha texto "Implementado"
+ resumo dos artefactos + nota de DEBT-53. Itens 4/5/6
inalterados (mantém numeração 151+/152+/decisão corpus).

---

## 10. Próximo passo

**151+** — P2 (`value_dto.rs` + `tests/eval_parity.rs`) ou
materialização de DEBT-53 (vanilla World adapter), consoante
priorização. Se DEBT-53 for priorizado, abrir passo dedicado
(escopo M-L: ~150-300 linhas).

Sequência conforme §9 actualizado:
- **150** ✓ (este).
- **151+** — P2 (eval parity) — quando expansão do corpus
  semantic/ for priorizada.
- **152+** — P4 (export parity, Opção B textual; A visual
  futura).
- **DEBT-53** — endereça vanilla integration (qualquer
  momento; passo dedicado).

---

## 11. Verificação final

| Item | Estado |
|------|--------|
| `lab/parity/src/frame_dto.rs` materializado (3 modos + tolerância) | ✅ |
| `lab/parity/src/report.rs` materializado (matriz + render markdown + write latest/history) | ✅ |
| `lab/parity/tests/layout_parity.rs` corre o corpus filtrado | ✅ (`cargo test --test layout_parity` → ok) |
| Subdir `lab/parity/corpus/visual/` com 9 ficheiros + `.typ.toml` adjacentes | ✅ |
| `lab/parity/reports/latest.md` produzido | ✅ |
| `lab/parity/reports/history/2026-04-25-passo-150.md` produzido (idêntico) | ✅ |
| `geometric` modo marcado **experimental** na matriz | ✅ |
| `text_content` e `structural` separados de `geometric` | ✅ |
| Definições de paridade ganham `experimental` na §P3 | ✅ |
| §9 do plano de medição actualizado | ✅ |
| Nenhum ficheiro tocado em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` cristalino | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| `crystalline-lint .` zero violations | ✅ (intacto) |
| `cargo test --workspace --lib`: 1113 inalterado | ✅ |
| `cd lab/parity && cargo test --test layout_parity` corre limpo | ✅ |
| **DEBT-53 aberto** (vanilla integration pendente; total abertos 10 → 11) | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-150**: o utilizador tem **infraestrutura de medição** +
**primeira matriz baseline** + **plano claro** (DEBT-53) para
completar a comparação contra vanilla. A pergunta original
("em que paridade estamos?") tem resposta parcial — em
forma de matriz com colunas `N/A` que serão preenchidas
quando DEBT-53 for materializado.

**Reformulações da série paridade até P150**: 3 (P148
inventário, P149 arqueologia, P150 baseline). Cada uma
justificada por descoberta de pré-condição. **Padrão
estabelecido**: reformulação por descoberta é regra.
