# Passo 96.7 — Reestruturação de `layout/mod.rs` em submódulos por fase de layout

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR` com 7 regras + 4 ajustes + nota de visibilidade
  (Passo 96.6). Aplicar a ordem de preferência de visibilidade:
  privado → método `pub(super)` → `pub(in path)` → campo
  `pub(super)` → `pub(crate)` → `pub`.
- `00_nucleo/DEBT.md` — DEBT-46 com checkbox 96.7 pendente.
  DEBT-47 aberto (auditoria futura de visibilidade).
- `01_core/src/rules/layout/mod.rs` — ficheiro actual, 2848
  linhas. Provavelmente contém `Layouter<M>` (struct central
  genérica sobre `FontMetrics`), `FixedMetrics`, e muitos
  métodos `impl`.
- `01_core/src/rules/layout/` — directório já existe. Pode
  conter outros ficheiros (ex: `text.rs`, `grid.rs`?). Verificar
  em Fase 0.

Pré-condição: `cargo test` — 748 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.6 concluído.

---

## Natureza deste passo

Passo único de reestruturação. Aplica ADR-0037 a
`layout/mod.rs`: divisão por **fase de layout** (medição,
posicionamento, emissão, paginação, grid, etc.).

Diferenças face aos Passos 96.1, 96.4, 96.5:

1. **Já está dentro de um submódulo** (`layout/`). Não é
   `git mv` de ficheiro único — é extracção incremental de
   pedaços do `mod.rs` para ficheiros irmãos (que podem já
   existir ou ser criados).

2. **Struct central genérica** (`Layouter<M: FontMetrics>`).
   Diferente do `Parser` (não genérico) e do `EvalContext`
   (não genérico na parte relevante). Genéricos complicam a
   extracção de métodos — cada `impl<M: FontMetrics> Layouter<M>`
   tem de ficar num ficheiro que saiba do parâmetro genérico.

3. **Muitos métodos interrelacionados**. `layout_content` chama
   `measure_content`, que chama `measure_word`, que usa
   `metrics.advance()`. Separar estes métodos requer cuidado:
   manter a cadeia de chamadas funcional.

4. **Nota de visibilidade em vigor**. Este é o primeiro passo
   com a Regra 3 actualizada — o Claude Code deve **preferir
   métodos sobre campos**, documentar escolhas, e evitar bulk
   replace.

---

## Clusters propostos

Com base em análise prévia (conhecimento do `Layouter`):

```
01_core/src/rules/layout/mod.rs (antes: 2848 linhas)
    ↓ transforma-se em:
01_core/src/rules/layout/
    mod.rs         — Layouter struct + ::new + entry points
                     públicos (layout, etc.), declarações de
                     submódulos, re-exports.
    metrics.rs     — FixedMetrics, FontMetrics trait (se
                     estiverem aqui e não noutro sítio),
                     helpers de medição.
    measure.rs     — measure_content, measure_word,
                     measure_content_constrained.
    cursor.rs      — gestão de cursor (flush_line, new_page,
                     advance_line), transições entre páginas.
    text.rs        — layout_word, layout_text,
                     line_breaking (se existir).
    blocks.rs      — layout_content, recursão sobre
                     Content::Sequence, blocos e recursão.
    grid.rs        — algoritmo de grid do Passo 80
                     (layout_grid, resolver_cols, etc.).
    shapes.rs      — layout_rect, layout_ellipse, layout_line
                     (se existirem separado de blocks).
    transforms.rs  — layout com transformações afins (move,
                     rotate, scale) do Passo 78.
```

**Ajustes durante execução**: a lista é hipótese baseada em
conhecimento geral do ficheiro. A estrutura real pode mostrar
clusters diferentes. Reportar a decisão em Fase 0.

---

## Fase 0 — Diagnóstico obrigatório

### 0.1 — Inventário do ficheiro actual

```bash
# Tamanho e armazenamento actual:
wc -l 01_core/src/rules/layout/mod.rs
ls -la 01_core/src/rules/layout/

# Se já há outros ficheiros no directório:
find 01_core/src/rules/layout/ -name "*.rs"

# Estrutura top-level:
grep -n "^pub fn\|^fn\|^pub struct\|^struct\|^impl\|^pub enum\|^enum\|^pub trait\|^trait" \
    01_core/src/rules/layout/mod.rs | head -60

# Impls do Layouter (normalmente vários blocos impl):
grep -n "^impl" 01_core/src/rules/layout/mod.rs

# Métodos por impl block (tamanho):
grep -cn "^\s*fn \|^\s*pub fn \|^\s*pub(super) fn \|^\s*pub(crate) fn " \
    01_core/src/rules/layout/mod.rs

# Testes:
grep -c "^\s*#\[test\]" 01_core/src/rules/layout/mod.rs
```

Reportar:
- Linhas confirmadas (esperado 2848).
- Lista de ficheiros no directório `layout/`.
- Se há `layout/grid.rs`, `layout/text.rs`, ou outros
  submódulos já criados.
- Número de impls e métodos.
- Campos do `Layouter` (quantos, de que tipo).
- Se `FixedMetrics` e `FontMetrics` trait estão aqui ou noutro
  lugar.

### 0.2 — Análise da struct `Layouter`

```bash
# Definição da struct:
grep -B 2 -A 30 "^pub struct Layouter" 01_core/src/rules/layout/mod.rs
```

Reportar:
- Número de campos.
- Campos com nomes de estado (ex: `cursor_x`, `cursor_y`,
  `current_line`) vs. campos de configuração (ex: `metrics`,
  `font_size_pt`).
- Se é `Layouter<M>` genérico.

Esta análise é importante porque determina:
- Se alguns campos podem ficar privados (com métodos expostos).
- Se alguns métodos podem mover inteiros para submódulos
  (tornando campos correspondentes privados).

### 0.3 — Mapear métodos por fase

Olhar os nomes dos métodos e agrupar por cluster proposto.
Reportar tabela:

| Cluster | Métodos candidatos |
|---------|---------------------|
| measure | measure_content, measure_word, ... |
| cursor | flush_line, new_page, ... |
| text | layout_word, ... |
| blocks | layout_content, layout_sequence, ... |
| grid | layout_grid, resolve_columns, ... |
| shapes | layout_rect, layout_ellipse, ... |
| transforms | layout_transform, ... |

Se algum método não encaixa claramente em cluster, reportar.
Candidatos ambíguos podem ficar no `mod.rs` inicialmente.

---

## Fase 1 — Extracções por cluster

### Ordem recomendada

Do mais isolado para o mais entrelaçado:

1. **metrics** — `FixedMetrics`, `FontMetrics` trait. Isolado,
   não depende de `Layouter` (provavelmente).
2. **shapes** — layout de formas (rect, ellipse, etc.). Chamam
   `cursor` mas não são chamados por outros.
3. **transforms** — layout com transformações. Relativamente
   isolado.
4. **text** — layout de palavras. Chama `metrics` e `cursor`.
5. **measure** — medição de conteúdo. Chamado por `blocks` e
   por `grid`.
6. **cursor** — gestão de cursor. Consumido por quase todos.
7. **grid** — algoritmo grande. Extracção beneficia muito.
8. **blocks** — `layout_content`, dispatcher central. Último
   porque consome dos outros.

### Procedimento por cluster

Para cada cluster:

#### Passo N.a — Identificar métodos a mover

```bash
# Lista métodos do cluster:
grep -n "fn <nome_metodo>" 01_core/src/rules/layout/mod.rs
```

#### Passo N.b — Criar ou reutilizar submódulo

Se `layout/<cluster>.rs` já existe: abrir e adicionar ao
existente. Se não existe: criar com cabeçalho:

```rust
//! Fase de <cluster> do layout. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

use super::*;
```

#### Passo N.c — Mover métodos

Para cada método a mover, decidir visibilidade pela ordem de
preferência da ADR-0037 (nota do Passo 96.6):

- **Se usado apenas pelo próprio cluster**: fica `fn` privado
  ao submódulo.
- **Se usado por outros submódulos**: `pub(super) fn`.
- **Se usado fora do módulo `layout/`** (ex: `eval/` ou `03_infra/`):
  `pub(crate)` ou `pub`.

**Atenção especial**: não mover para `pub(super)` por conveniência
de bulk replace. Cada elevação deve ter razão registada no
comentário.

#### Passo N.d — Sobre o `Layouter<M>` genérico

Métodos de `Layouter<M>` movem para submódulos como `impl<M: FontMetrics> Layouter<M>`:

```rust
// Em layout/measure.rs:
impl<M: FontMetrics> super::Layouter<M> {
    pub(super) fn measure_content(&self, content: &Content) -> Metrics {
        // ...
    }
}
```

Isto é o padrão Rust para "spread impls across files". Funciona
bem desde que o submódulo importe os tipos necessários.

#### Passo N.e — Sobre acesso a campos

Se um método movido precisa de acesso a um campo do `Layouter`,
há dois padrões:

**Padrão preferido** (nota de visibilidade):

```rust
// Em layout/mod.rs (struct principal):
pub struct Layouter<M> {
    metrics: M,              // privado
    cursor_x: Pt,            // privado
    cursor_y: Pt,            // privado
    // ...
}

impl<M> Layouter<M> {
    pub(super) fn cursor_x(&self) -> Pt { self.cursor_x }
    pub(super) fn set_cursor_x(&mut self, x: Pt) { self.cursor_x = x }
    // etc.
}

// Em layout/text.rs:
impl<M: FontMetrics> super::Layouter<M> {
    pub(super) fn layout_word(&mut self, word: &str) {
        let x = self.cursor_x();       // método
        // em vez de: let x = self.cursor_x;  (acesso directo a campo privado)
    }
}
```

**Padrão aceitável quando inevitável**:

```rust
pub struct Layouter<M> {
    pub(super) metrics: M,           // campo pub(super) — documentado
    // ...
}
```

Com comentário no código:

```rust
// Campo `pub(super)` porque múltiplos submódulos (text, measure,
// blocks) precisam de passar referência ao trait FontMetrics. Um
// getter duplicaria código sem ganho.
pub(super) metrics: M,
```

#### Passo N.f — Verificar

```bash
cargo check --package typst-core 2>&1 | tail -10
cargo test --package typst-core 2>&1 | tail -5
```

Se falhar, rollback do cluster (`git checkout` dos ficheiros
afectados) e reportar.

### Ordem de execução

Executar os clusters um a um, **verificando entre cada** que
os testes passam. Se qualquer cluster falhar:
- Rollback desse cluster.
- Reportar a fricção.
- Decidir se continuar com os restantes ou se é necessário
  repensar a divisão.

---

## Fase 2 — Verificação final

### 2.1 — Tamanhos

```bash
wc -l 01_core/src/rules/layout/*.rs | sort -rn
```

Alvo:
- `mod.rs` abaixo de 800 linhas.
- Cada submódulo abaixo de 800 linhas.
- Se `grid.rs` ficar acima (algoritmo de grid é grande),
  justificar Regra 6 (algoritmo coeso) ou subdividir em
  `grid/mod.rs` + `grid/columns.rs` + `grid/cells.rs`.

### 2.2 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 748 L1 + 174 L3 + 6 ignorados (ou variação por smoke
tests V2 dos submódulos novos, se criados). Zero violations.

### 2.3 — Testes específicos de layout

```bash
cargo test --package typst-core layout 2>&1 | tail -15
cargo test --package typst-core grid 2>&1 | tail -10
cargo test --package typst-core page 2>&1 | tail -10
```

Todos devem passar sem alteração.

### 2.4 — Visibilidades aplicadas

```bash
# Quantos pub(super) foram introduzidos:
grep -rn "pub(super)" 01_core/src/rules/layout/*.rs | wc -l

# Quantos pub(in ...) foram usados:
grep -rn "pub(in " 01_core/src/rules/layout/*.rs

# Campos pub(super) vs métodos pub(super):
grep -rn "pub(super)\s*\w*:\s" 01_core/src/rules/layout/*.rs | wc -l    # campos
grep -rn "pub(super)\s*fn" 01_core/src/rules/layout/*.rs | wc -l        # métodos
```

Reportar as contagens. A expectativa (pela nota de visibilidade)
é que **métodos `pub(super)` superem campos `pub(super)`**. Se
for o inverso, merece reavaliação.

---

## Fase 3 — Actualizar DEBT-46

Marcar o sétimo checkbox:

```markdown
- [x] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). Passo 96.7. **Concluído** — N
      submódulos, todos abaixo de 800 linhas (ou excepção
      Regra 6 com justificativa). Visibilidade seguiu a nota
      da Regra 3: X métodos `pub(super)`, Y campos
      `pub(super)` (cada um com comentário de razão).
```

DEBT-46 não fecha (3 checkboxes restantes).

---

## Critérios de conclusão

- [ ] `layout/mod.rs` reduzido de 2848 para abaixo de 800
      linhas (ou excepção Regra 6 documentada).
- [ ] Submódulos criados/actualizados com métodos do
      `Layouter<M>`.
- [ ] Visibilidade seguiu ordem de preferência da ADR-0037:
      privado → métodos `pub(super)` → `pub(in path)` → campos
      `pub(super)` (justificados) → `pub(crate)` → `pub`.
- [ ] Sem bulk replace de `pub(super)` — cada elevação tem
      razão.
- [ ] Testes de layout passam sem alteração.
- [ ] `cargo test --workspace` preservado (748 ± smoke tests
      V2).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo.
- [ ] DEBT-46 com sétimo checkbox marcado.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Fase 0:
- Lista de ficheiros pré-existentes em `layout/`.
- Número de campos e métodos do `Layouter<M>`.
- Se `FixedMetrics` e `FontMetrics` estão neste ficheiro ou
  noutro.

Fase 1 (por cluster):
- Cluster extraído, tamanho final do submódulo.
- Visibilidade aplicada aos métodos movidos (resumo).
- Campos do `Layouter` que precisaram de `pub(super)` (listar
  com justificativa).
- Rollback se necessário.

Fase 2:
- Tamanhos finais de todos os ficheiros em `layout/`.
- Contagens de `pub(super)` fn vs `pub(super)` campo.
- Testes verdes, zero violations.

Fase 3:
- Confirmação DEBT-46 actualizado.

Observações sobre nota de visibilidade (primeira aplicação):
- A ordem de preferência foi facilmente aplicável?
- Houve casos ambíguos (ex: campo que podia ser método mas
  o método ficaria trivial)?
- Fricções novas não previstas pela ADR?

Go/No-Go para Passo 96.8:
- **Go incondicional** se reestruturação foi limpa e
  visibilidade seguiu a nota. Passo 96.8 = `math/layout.rs`
  (1806 linhas), análogo ao 96.7 mas para o layout matemático.
- **Go com observações** se houve fricção com nota de
  visibilidade que merece ajuste futuro.
- **No-Go parcial** se `Layouter<M>` genérico complicou
  extracção de forma impraticável. Nesse caso, reverter o
  cluster problemático e reportar — pode exigir ajuste da
  ADR-0037 para structs genéricas.
