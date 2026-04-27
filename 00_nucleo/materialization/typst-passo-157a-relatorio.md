# Relatório P157A — `Content::Table` minimal (Model Fase 2 sub-passo 1)

Primeiro sub-passo substantivo de Model Fase 2 declarada em
ADR-0060 §"Decisão 1" sub-passo 3. **Décima aplicação consecutiva
de materialização** desde início da série granular P156C.
**Padrão cross-domínio confirmado**: granularidade granular
Layout (P156C-L) replica-se a Model (P157A) sem reformulação.

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo: `00_nucleo/diagnosticos/diagnostico-table-passo-157a.md`
(7 itens canónicos ADR-0034 + 2 itens específicos de primeiro
passo de novo módulo Model Fase 2).

**Decisões arquitecturais documentadas**:
- **Módulo stdlib**: `stdlib/structural.rs` (continuação do
  módulo Model existente — heading/terms/quote já lá vivem;
  sem novo `stdlib/model.rs`).
- **Field name `children`** (não `cells` como Grid): paridade
  vanilla `Vec<TableChild>`; pequena divergência intra-cristalino
  documentada.
- **Helper `extract_tracks`**: promoção a `pub(super)` para
  reuso cross-módulo (sibling-module access).
- **Layout delegação**: clone simples de `layout_grid`; sem
  modificação de `grid.rs`.

**ADR-0064 NÃO aplicável directamente**: subset minimal sem
`Smart<T>` em vanilla. Aplicações futuras em P157B (Caso A
para `TableCell.x/y`) e P157C (Caso D para `TableHeader.repeat`
default true).

### 1.2 Variant `Content::Table` (.2)

Adicionado a `01_core/src/entities/content.rs` (52 → **53**
variants).

```rust
Table {
    columns:  Vec<TrackSizing>,
    rows:     Vec<TrackSizing>,
    children: Vec<Content>,
}
```

Construtor `Content::table(columns, rows, children)` análogo
a `Content::grid(...)` existente.

Cobertura exaustiva de **9 sítios pattern-match estruturais**
(paridade P156I Stack / P156J Repeat / P156L Pad):
- Variant declaration + construtor.
- `is_empty()` proxy via `children.is_empty()`.
- `plain_text()` concatena com space (paridade Grid).
- `PartialEq` cobre columns/rows/children.
- `map_content` recurse em children.
- `map_text` recurse em children.
- `introspect.rs::materialize_time` recurse + preserva tracks.
- `introspect.rs::walk` walk em cada child.

### 1.3 Stdlib `native_table` (.3)

Adicionado a `01_core/src/rules/stdlib/structural.rs` (módulo
Model existente; **sem novo módulo `stdlib/model.rs`** per
decisão .1).

```rust
pub fn native_table(_ctx, args, _world, _file, _fig) -> SourceResult<Value>
```

Comportamento:
- `columns: Vec<TrackSizing>` (named); default `[Auto]`
  (paridade Grid em P83).
- `rows: Vec<TrackSizing>` (named); default `[Auto]`.
- `children: Vec<Content>` (variadic posicional; aceita Content
  ou Str).
- Validações: named arg desconhecido rejeitado com diagnóstico
  claro mencionando ADR-0054 graded; child Int rejeitado.

`extract_tracks` reusado cross-módulo via `pub(super)`. Helper
agora atinge **N=2** aplicações concretas — subpadrão emergente
análogo a `extract_length` N=7.

Registado em `eval/mod.rs::make_stdlib` como `table` →
`Func::native("table", native_table)`. Re-exportado em
`stdlib/mod.rs`.

### 1.4 Layout para `Content::Table` (.4)

Pattern arm novo em `layout_content` (`01_core/src/rules/layout/mod.rs`)
delega a `layout_grid` clone simples:

```rust
Content::Table { columns, rows, children } => {
    self.layout_grid(columns, rows, children);
}
```

**Sem modificação de `01_core/src/rules/layout/grid.rs`** —
algoritmo Grid existente (272 linhas) reutilizado integralmente.
Confirmado por inspecção: zero diff em `grid.rs` durante P157A.

### 1.5 Tests (.5)

**+16 tests novos** em três camadas:

**6 unit tests `Content::Table`** em `entities/content.rs`:
- Constructor default (vec![], vec![], vec![]).
- Constructor com tracks e children.
- `is_empty` proxy via children.
- `plain_text` concatena com space.
- `PartialEq` cobertura (3 vias: children, columns, rows).
- `map_text` recurse em children.

**8 stdlib tests `native_table`** em `stdlib/mod.rs`:
- Defaults columns/rows = Auto.
- columns Int (3 → 3 tracks Auto).
- columns Array Length (Fixed).
- Children variádicos.
- Aceita Str como child.
- Named arg desconhecido rejeitado.
- Child Int rejeitado.
- Paridade estrutural Table↔Grid (variants ≠ mas mesma
  estrutura).

**2 layout E2E tests** em `layout/tests.rs`:
- `layout_table_renderiza_children_como_cells` — cells
  aparecem como FrameItems.
- `layout_table_paridade_com_grid_equivalente` — Table e Grid
  com mesmos campos produzem **posições idênticas** (validação
  empírica da delegação).

**Δ tests = +16** (within range esperado +12-18).

### 1.6 Hashes + cobertura (.6)

`crystalline-lint --fix-hashes .` reportou **"Nothing to fix"**
(refactor preserva hash do prompt L0; mudança puramente
aditiva).

Tabela cobertura actualizada:
- A.6 Model: `table` `ausente → implementado` (footnote ²²).
- A total: 6/4/5/7/0=22 → **7/4/5/6/0=22** Model.
- A user-facing total: 63/22/22/32/2=141 → **64/22/22/31/2=141**.
- B Content variants: 52 → **53** (+`Table`; footnote ²³).
- B total: 68/13/5/19/1=106 → **69/13/5/18/1=106**.
- Cobertura user-facing total: ~60.3% → **~61.0%** (+0.7pp).
- Cobertura arquitectural total: 76-77% → **77-78%**.

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P157A.
ADR-0060 ganha anotação P157A. README ADRs ganha entrada P157A
antes de P157.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1319 + Δ; zero falhas (Δ esperado +12-18) | **Δ=+16** (1319 → 1335 lib+integ+diag); zero falhas; workspace total 1357 |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 53 (52 → 53) | **✓ 53** (`Table` adicionado) |
| 4 | Stdlib funcs: 43 (42 → 43) | **✓ 43** (`table` registado) |
| 5 | Cobertura Model: ~50% (~45% → ~50%) | **✓** entrada `table` `ausente → implementado`; Model 7/4/5/6/0=22 = 50% (impl + impl⁺) |
| 6 | Hash actualizado em prompts L0 (`crystalline-lint --check-hashes` passa) | **✓** `crystalline-lint --fix-hashes` reportou "Nothing to fix"; lint clean |
| 7 | Decisão de módulo documentada no relatório | **✓** §1.1: `stdlib/structural.rs` continuação per critérios em diagnóstico .1 §8 |
| 8 | `layout_grid` original NÃO modificado | **✓** zero diff em `01_core/src/rules/layout/grid.rs` durante P157A |

**Build limpo**: `cargo build` 1.17s sem warnings novos.

---

## 3. Análise de risco — peso real (oitava aplicação consecutiva; primeiro Model Fase 2)

P157A é **primeiro passo de domínio Model Fase 2** após série
Layout completa P156C-L. §análise de risco preserva precedente
N=7 (P156F-L + P157) → **N=8**.

### 3.1 Riscos materializados durante o passo

| Risco | Materializado? | Mitigação aplicada |
|-------|:--------------:|---------------------|
| Decisão de módulo (`stdlib/structural.rs` vs `stdlib/model.rs`) ser não-trivial | Sim, breve | Inventário .1 §8 inspeccionou estrutura existente e confirmou `structural.rs` como módulo Model natural; decisão deferida explícita per pré-decisão da spec |
| Field naming (`children` vs `cells`) divergir intra-cristalino | Sim, aceite | Diagnóstico .1 §3.2 documenta divergência consciente; refactor escopo XS futuro se padrão emergir |
| `Value::Array` API ter mudado (test compile error) | Sim, breve | Erro de tipo em test stdlib paridade Grid; corrigido com `Vec<Value>` directo (não `Arc::from`); ciclo único |
| Paridade observável Table↔Grid quebrada | Não | Test `layout_table_paridade_com_grid_equivalente` confirma posições idênticas — delegação correcta |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| `layout_grid` exigir refactor para aceitar Table | Médio | Inventário confirmou assinatura `&[TrackSizing], &[TrackSizing], &[Content]` compatível directamente |
| Pattern-match exhaustive falhar em sítios fora de `content.rs` | Baixo | Cobertura sistemática 9 sítios (paridade P156I/J/L) detectou `layout/mod.rs` em primeira tentativa de build |
| `extract_tracks` ter assinatura incompatível | Nulo | `Option<&Value> → Vec<TrackSizing>` exactamente o que `native_table` precisa |
| Quebra de tests pré-existentes Grid | Nulo | Grid não tocado; tests Grid passam inalterados |
| ADR-0064 ser aplicável de forma surpresa (Caso A/B/C/D) | Não | Inventário §3 confirmou subset minimal sem `Smart<T>` — caso aplicação futura P157B/C |

### 3.3 Riscos não-aplicáveis

- **Algoritmo dinâmico de runtime**: P157A delega a Grid; sem
  algoritmo novo.
- **Quebra de paridade observável vs vanilla**: Table emite
  cells em ordem (paridade); divergência aceite per ADR-0033 é
  apenas **estrutural** (variant dedicado).

### 3.4 Conclusão de risco

**Risco residual: muito baixo após inventário**. O risco
principal era decisão de módulo da stdlib func — neutralizado
pelo sub-passo .1 que confirmou `structural.rs` como módulo
Model natural antes de execução. **Auto-validação ADR-0065**:
critério #1 (naming módulo) implícito + critério #5 (scope) do
diagnóstico foram aplicados com sucesso pela primeira vez em
domínio Model.

**§análise de risco preserva precedente cross-domínio**:
P156L primeiro com peso real (refactor); P157A primeiro com
peso real (Model). Padrão #4 cresce para N=8 com diversidade
de domínios.

---

## 4. Slope cumulativo Model (mesa P155-P157A; primeira mesa Model dedicada)

| Passo | Feature(s) | Slope Model | Cobertura Model cumulativa | Tests Δ |
|-------|-----------|------------:|---------------------------:|--------:|
| P154A | (diagnóstico Model) | — | 36% (recálculo empírico) | 0 |
| P154B | terms + divider | +5%  | 36% → 41% | +10 |
| P155 | quote | +4%  | 41% → 45% (Fase 1 fechada) | +21 |
| P157 | (diagnóstico Model Fase 2) | — | — (sem código) | 0 |
| **P157A** | **table minimal (Fase 2 sub-passo 1)** | **+5%** | **45% → 50%** | **+16** |

**Total cumulativo P154A-P157A** (Model): **+14pp** Model
em 5 passos (3 materialização + 2 diagnóstico). Fase 1 fechada
em P155 (45%); **Fase 2 inicia em P157A** com primeiro sub-passo
materializado.

**Mesa cross-domínio P156C-P157A** (combinada Layout + Model):

| Domínio | Passos | Slope total | Tests Δ |
|---------|-------:|------------:|--------:|
| Layout (P156C-L) | 9 materialização + 1 meta | +56pp Layout | +174 |
| Model (P157A) | 1 materialização + 1 diagnóstico | +5pp Model | +16 |
| **Cross-domínio total** | **10 materialização + 2 meta + 1 diagnóstico** | — | **+190** |

**Padrão granular universal confirmado**: cadência 1-2 features
por passo replicada cross-domínio sem reformulação. **N=10
materialização sem reformulação** — patamar empírico forte.

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P157A:

### 5.1 Padrões metodológicos pós-P157A

| # | Padrão | Pré-P157A | Pós-P157A |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 9 | **10** (cross-domínio) |
| 2 | "Inventariar primeiro" pré-decisão | 7 | **8** |
| 3 | "Smart→Option/default" | 7 | 7 (inalterado — não aplicável directamente em P157A) |
| 4 | "§análise de risco no relatório" | 7 | **8** (primeiro Model Fase 2) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 7 | 7 (inalterado) |
| 8 | **Reuso `Sides<T>`** | 2 | 2 (inalterado) |
| 9 | **Reuso `extract_tracks`** (novo subpadrão P157A) | — | **2** |

### 5.2 Auto-validação cumulativa de ADRs meta P156K

P157A confirma utilidade dos ADRs meta com aplicação cross-domínio:

- **ADR-0064**: NÃO aplicável directamente em P157A (subset
  sem Smart<T>); aplicações futuras em P157B Caso A
  (`TableCell.x/y`) e P157C Caso D (`TableHeader.repeat`
  default true). N=7 implícito mantido.
- **ADR-0065**:
  - Critério #1 (naming módulo) — **implícito** em decisão
    `stdlib/structural.rs` continuação vs `stdlib/model.rs`
    novo (P157A diagnóstico §8).
  - Critério #5 (scope) — **reforçado** pela aplicação P157
    diagnóstico precedendo P157A.
  - Outros critérios sem aplicação isolada concreta neste passo.

**Padrão emergente**: cada passo de novo domínio (Layout em
P156G; Model em P157A) cumpre cumulativamente critérios
adicionais de ADR-0065. ADR meta cresce em utilidade empírica
sem nova ADR.

---

## 6. Estado pós-P157A

- **Cobertura Layout**: **78%** (inalterada — escopo Model).
- **Cobertura Model**: ~45% → **~50%** (primeiro sub-passo
  Fase 2; +5pp).
- **Variants Content**: **53** (era 52; +`Table`).
- **Stdlib funcs**: **43** (era 42; +`table`).
- **Helper promovido**: `extract_tracks` agora `pub(super)`
  para reuso cross-módulo (subpadrão N=2).
- **Tests**: **1097** typst-core lib (era 1081; +16). Workspace:
  1357.
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0060**: `IMPLEMENTADO` mantido; ganha anotação P157A.
- **ADR-0061**: `PROPOSTO` mantido; §"Aplicações cumulativas"
  actualizada.
- **README ADRs**: entrada P157A adicionada antes de P157.
- **Reservas P158/P159/ADR-0062**: inalteradas.
- **Hash `content.rs`**: `ec58d849` (preservado — refactor
  aditivo preserva contrato L0).
- **Total ADRs**: **63** (inalterado).

### 6.1 Cobertura user-facing total

(64 + 22) / 141 = **~61.0%** (era ~60.3% pós-P156L; +0.7pp).

---

## 7. Decisão pós-P157A

Per spec do passo §"Pós-passo", próximas opções pré-acordadas
para Model Fase 2:

1. **P157B** — `Content::TableCell` + colspan/rowspan armazenados
   per ADR-0054 graded. Continua série Model Fase 2.
   Granularidade N=11. Tests ~12-18. Aplicação concreta de
   ADR-0064 Caso A (`x/y: Smart<usize>` → `Option<usize>`).
2. **P157C** — `Content::TableHeader` + `Content::TableFooter`
   par simétrico. Granularidade N=12. Tests ~10-15. Aplicação
   concreta de ADR-0064 Caso D (`repeat: bool` default true,
   reforço de N=7 → 8).

Outras direcções pendentes (per relatórios anteriores):
3. Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
   quebra granularidade).
4. Footnote area.
5. Promover ADR-0061 a IMPLEMENTADO.
6. Promover `extract_length` a helper público.
7. Promover `extract_tracks` a helper público (apenas N=2;
   prematuro per política de promoção mínima N=3-4).
8. Atacar Introspection (17% cobertura).

ADR-0060 mantém-se `IMPLEMENTADO` (Fase 2 em curso não muda
status de Fase 1).

ADR-0061 mantém-se `PROPOSTO` (Layout não tocado).

**Padrão granularidade N=10 NÃO é formalizado** — continua
candidato a ADR meta futura. P157A consolida o padrão sem
quebra.

---

## 8. Fechamento

P157A fecha como **primeiro passo Model Fase 2** materializado
após série Layout completa P156C-L. **Padrão cross-domínio
confirmado**: granularidade granular Layout → Model sem
reformulação. **Décima aplicação consecutiva** de materialização
desde início da série granular. **§análise de risco N=7 → 8**
com primeiro precedente Model.

**Decisão arquitectural-chave**: módulo `stdlib/structural.rs`
continuação (não novo `stdlib/model.rs`) — preserva estrutura
ADR-0037 estabelecida em P96.5 e evita redundância semântica.

**Auto-validação cross-domínio dos ADRs meta P156K**:
ADR-0065 critério #1 (naming módulo) + critério #5 (scope)
aplicados pela primeira vez em domínio Model. ADR-0064 não
aplicável directamente — futuras aplicações em P157B/C.

**Subpadrão emergente novo**: reuso `extract_tracks` N=2 com
promoção a `pub(super)`. Análogo a `extract_length` em fase
inicial. Promoção a helper público diferida até N=3-4
(política consistente).

ADR-0060 mantém `IMPLEMENTADO`; ADR-0061 mantém `PROPOSTO`.

**Pausa natural após P157A — primeiro sub-passo Model Fase 2
materializado; padrões cross-domínio consolidados; N=10 sem
reformulação. Decisão humana sobre próxima direcção (P157B
TableCell ou outras 7 candidatas) tem máxima informação.**
