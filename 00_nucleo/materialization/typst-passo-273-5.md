# Passo P273.5 — P-Gradient-Relative-Callsite

**Tipo**: refino estrutural fecho-de-pendência (activa `apply_parent_transform` deixado em `#[allow(dead_code)]` em P273).
**Magnitude estimada**: S (Layouter ganha thread de parent_bbox + callsite emit; sem L1 touch; stdlib intocado).
**Pré-requisitos**: P273 fechado (3/3 campos cross-variant) + P274 fechado (adaptive N).
**Cluster**: Visualize / Gradient (encerra refino estrutural do cluster Gradient).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; sétima anotação cumulativa); ADR-0029 (pureza física L1); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P273 materializou `relative: Option<RelativeTo>` cross-variant (Linear/Radial/Conic). L3 ganhou dois helpers:

- `resolve_relative(relative) -> RelativeTo` — `Auto → Self_` per default.
- `apply_parent_transform(local_coords, parent_bbox)` — calcula coordenadas transformadas em Rust antes de emit (decisão arquitectural P273: não usar PDF `/Matrix`).

P273 §7 deixou explicitamente `apply_parent_transform` como `#[allow(dead_code)]` — função existe, é testada via unit tests de transformação, mas **nenhum callsite real do Layouter fornece `parent_bbox` real**. O dispatcher emit ignora o caminho Parent (resolve sempre para Self_ na prática porque `parent_bbox: Option<Rect>` chega sempre `None`).

P273.5 fecha esta pendência: thread `parent_bbox` real do Layouter até ao callsite de emit gradient, activando o caminho `RelativeTo::Parent` por construção.

### Precedente no Layouter cristalino (decisivo)

DEBT-37 (Passo 84.6) introduziu `cell_origin_x`, `cell_origin_y`, `cell_origin_w` (todos `Option<f64>`) no Layouter para `Content::Place` scope Parent. Padrão estabelecido:

- Campo opcional no Layouter populado quando o contexto pai é conhecido.
- Save/restore por entrada/saída do contexto pai.
- Consumer dispatcha entre "tem contexto" (`Some` → usa) vs "não tem" (`None` → fallback).

P273.5 replica o mesmo padrão — campo `parent_bbox: Option<Rect>` no Layouter, populado quando se entra num scope que define um contentor (shape com gradient é o consumer relevante).

### Semântica `RelativeTo::Parent` em vanilla

Em vanilla, `RelativeTo::Parent` para gradient em shape significa **a bbox do contentor imediato da shape** (não a página global). Quando a shape é top-level a "página" é o contentor; quando a shape está aninhada num `Block`/`Boxed`/`Group`, é a bbox desse Block/Boxed/Group.

Cristalino actual já tem:
- bbox local da shape no momento de emit (callsite `emit_shape_with_fill` ou análogo).
- bbox da página via `Size::a4()` (ou page size dinâmico).
- bbox de células Grid via `cell_origin_*` (DEBT-37 padrão).

Falta: **propagar a bbox do contentor imediato** ao emit gradient. P273.5 faz essa propagação.

---

## §1 — Sub-passo P273.5.A — Fase A diagnóstico

**Magnitude**: S documental (~20-30 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-5A-diagnostico.md`.

### §A.1 — Inventário do callsite real

Listar literal em `03_infra/src/export.rs`:

- Função(ões) que chamam `emit_gradient_objects` (ou equivalente — confirmar nome real do callsite pós-P272/P273).
- Parâmetros actuais recebidos pelo callsite.
- Origem dos `gradient_objs` — vêm de quê (FrameItem? Conteúdo da shape?).
- Sítio onde a bbox local da shape é conhecida — é o mesmo callsite ou anterior?

### §A.2 — Inventário do propagação cristalino existente

Listar literal:

- `cell_origin_x/y/w: Option<f64>` no Layouter (DEBT-37 P84.6).
- Save/restore patterns aplicáveis — Grid `cell_*`, Place scope Parent.
- Como bbox da página é conhecida no L3 (export) — `Frame.size` ou equivalente.
- Onde o Layouter emite shapes com fill — `FrameItem::Shape` ou variante; campo `fill` actual.

### §A.3 — Definição de "Parent" para gradient cristalino

Decisão a fixar na Fase A:

- **Opção 3α**: Parent == página sempre (simplificação; gradient ancora à página independentemente do contentor).
- **Opção 3β**: Parent == contentor imediato (paridade vanilla; precisa thread de bbox local desde o Layouter).
- **Opção 3γ**: Parent == contentor imediato com fallback página (Some(contentor_bbox) usa contentor; None usa página).

Critério: paridade vanilla observable favorece 3β/3γ. 3α é simplificação registável per ADR-0054 graded mas perde semântica vanilla. 3γ é o caminho de menor compromisso (preserva comportamento legítimo "shape top-level com gradient relative=parent ancora à página") **e** ganha o caso aninhado.

**Recomendação spec**: 3γ (default sugerido; decisão final na Fase A).

### §A.4 — Mecanismo de propagação (depende de §A.3)

Se 3α (página) — refactor trivial:
- Sem thread; emit consulta page size directo.
- LOC esperado: ~20-30 L3.

Se 3β/3γ (contentor imediato):
- Layouter ganha campo `current_container_bbox: Option<Rect>` (padrão DEBT-37).
- Save/restore ao entrar/sair de Block/Boxed/Group ou outro contentor (definir lista exacta na Fase A).
- `FrameItem` que carrega shape gradient transporta bbox no `pos/size` (já carrega; aproveitar).
- Callsite emit gradient consulta `Option<Rect>` no contexto + bbox da shape no `FrameItem`; passa o que estiver disponível a `apply_parent_transform`.

LOC esperado 3β/3γ: ~50-80 L3 + ~20-30 L1 (campo no Layouter + save/restore arms).

### §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P273 (resolve sempre Self_ pré-P273.5) | Defaults preservados | Inputs sem `relative=parent` continuam a dispatchar para Self_; bit-exact preserved |
| Regressão tests P262-P272 | Pipeline emit alterado | Toda a thread só dispara para `RelativeTo::Parent`; resto preserved literal |
| Bbox errada propagada (off-by-one, contentor errado) | Decisão 3β/3γ propagação | Tests E2E com `relative=parent` aninhado em Block validam transform |
| Pureza física L1 quebrada | Campo `current_container_bbox` no Layouter (L1) | Rect já é tipo L1; campo é gestão de memória RAM, não I/O — ADR-0029 §"O que NÃO é I/O" cobre |
| `#[allow(dead_code)]` continua | Activação parcial (apenas alguns callsites) | §A.6 critério: pelo menos 1 callsite real fornece bbox não-None |

### §A.6 — Critério de fecho `#[allow(dead_code)]`

`apply_parent_transform` deixa de ter `#[allow(dead_code)]` quando:

1. Pelo menos um callsite real do Layouter passa `Some(rect)` ao dispatcher de gradient.
2. Pelo menos um test E2E exercita a path `RelativeTo::Parent` com bbox real diferente da página inteira (3β/3γ) ou da página (3α).
3. Compilador não dispara warning de dead code sem `#[allow]` — confirmação empírica.

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 3** (semântica Parent): 3α / 3β / 3γ. Recomendação spec: 3γ.
2. **Lista de contentores** que disparam save/restore (se 3β/3γ): Block? Boxed? Group? Grid cell? Page? Decidir literal na Fase A com inventário §A.2.
3. **Tipo Rect cristalino**: confirmar se já existe em L1; se não existir, adicionar struct mínima `pub struct Rect { x: f64, y: f64, w: f64, h: f64 }` em `entities/geometry.rs` (junto a `ShapeKind`/`Stroke`).

### §A.8 — Critério de aceitação Fase A

- §A.1 confirma callsite real (nome função + path:linha).
- §A.2 confirma precedente DEBT-37 directamente aplicável.
- §A.3 decisão fixada com fundamento (recomendação 3γ ou alternativa justificada).
- §A.5 risco "regressão P262-P273" mitigado por defaults.
- §A.7 decisões 1/2/3 fixadas.

---

## §2 — Sub-passo P273.5.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental.

**Não criar ADR-0095**. Anotar ADR-0091 cumulativamente (sétima anotação consecutiva).

Template:

```
## Anotação cumulativa P273.5 — Parent bbox callsite (fecha `#[allow(dead_code)]` P273)

**Data**: 2026-05-XX.
**Motivo**: P273 deixou `apply_parent_transform` em `#[allow(dead_code)]`
— função existe + tests unit transformação, mas sem callsite real fornecendo
`parent_bbox`. P273.5 fecha pendência: thread parent_bbox do Layouter até
ao emit gradient.

**Semântica Parent escolhida**: [3α / 3β / 3γ — preencher pós-Fase A].

**Mecanismo de propagação**: campo `current_container_bbox: Option<Rect>`
no Layouter populado por save/restore em [lista contentores fixada Fase A].
Padrão replicado de DEBT-37 P84.6 (`cell_origin_*` para Place scope Parent).

**Defaults preservam P273 bit-exact**:
- Inputs sem `relative=parent` continuam a resolve Self_.
- Path Self_ é pipeline P272 literal preserved.
- 2597 baseline preserved (regressão zero).

**Sub-padrão "Reutilização literal helpers cross-passos"** N=13 → 14
cumulativo — `apply_parent_transform` P273 reused literal (sem alteração);
padrão `cell_origin_*` DEBT-37 P84.6 reused estructuralmente.

**`#[allow(dead_code)]` fechado**: P273 §7 pendência resolvida; cluster
Gradient refino estrutural encerrado.
```

---

## §3 — Sub-passo P273.5.C — Materialização (testes primeiro)

**Magnitude**: S (~60-100 LOC consoante Decisão 3).

### Ordem literal

1. Fase A §1 produzida.
2. ADR-0091 anotação §2 escrita pós-fixação Decisão 3.
3. `crystalline-lint --fix-hashes` (refactor preserva hashes L0; eventual touch L0 se Rect for adicionado).
4. **Testes-primeiro**.
5. Código:
   - L1 — campo `parent_bbox: Option<Rect>` no Layouter (se 3β/3γ); eventual Rect struct em `entities/geometry.rs` se não existir.
   - L1 — save/restore arms em Block/Boxed/Group/etc. (lista Fase A).
   - L3 — callsite emit gradient lê o campo + passa a `apply_parent_transform`; remove `#[allow(dead_code)]`.
6. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

- **L1 hard cap**: ≤ 60 LOC (campo + save/restore arms + eventual Rect struct).
- **L1 soft cap**: ≤ 40 LOC.
- **L3 hard cap**: ≤ 80 LOC (dispatcher emit + remover allow(dead_code)).
- **L3 soft cap**: ≤ 50 LOC.
- **Tests hard cap**: ≤ 12 novos.
- **Tests soft cap**: ≤ 8.

### Tests propostos (lista mínima — completar pós-Fase A)

1. `p273_5_apply_parent_transform_no_longer_allow_dead_code` — verificação que `cargo build` não dispara warning de dead code sem `#[allow]` (test pode ser commentário + linter check).
2. `p273_5_linear_relative_parent_top_level_uses_page_bbox` — Linear `relative=parent` top-level emit usa page size como bbox (3α ou 3γ fallback).
3. `p273_5_linear_relative_parent_inside_block_uses_block_bbox` — Linear `relative=parent` aninhado em Block emit usa bbox do Block (3β/3γ).
4. `p273_5_radial_relative_parent_mirrors_linear` — paridade Radial.
5. `p273_5_conic_relative_parent_coons_transformed` — Conic Coons usa bbox transformado.
6. `p273_5_save_restore_block_bbox` — entrar em Block guarda bbox actual; sair restaura.
7. `p273_5_save_restore_nested_blocks` — Block dentro de Block — restore por LIFO.
8. `p273_5_relative_self_unchanged` — `relative=self` continua a usar bbox local da shape (regressão P272 zero).
9. `p273_5_relative_auto_resolves_self_unchanged` — `relative=auto` resolve Self bit-exact P273.
10. `p273_5_emit_gradient_objects_parent_bbox_threaded` — unit test do dispatcher real passar `Some(rect)`.
11. Regressão integrada: rodar suite P262-P273 — 2597 verdes inalterados.
12. (Tests adicionais consoante Decisão 3 — completar pós-Fase A.)

### Alterações esperadas no código

```rust
// L1 — 01_core/src/rules/layout/mod.rs (Layouter struct)
pub struct Layouter<M: FontMetrics> {
    // ... fields existentes ...
    /// P273.5 — bbox do contentor imediato; populated em Block/Boxed/Group/etc.
    /// Consumed em emit gradient quando relative=parent (DEBT-37 pattern).
    parent_bbox: Option<Rect>,  // novo (se 3β/3γ)
}

// L1 — arm Block (e.g.)
Content::Block { body, breakable, .. } => {
    let saved_parent = self.parent_bbox;
    self.parent_bbox = Some(Rect {
        x: self.cursor_x.0,
        y: self.cursor_y.0,
        w: /* block width resolved */,
        h: /* block height resolved */,
    });
    // recurse no body
    self.layout_content(body);
    self.parent_bbox = saved_parent;  // restore LIFO
}

// L3 — 03_infra/src/export.rs callsite
fn emit_shape_with_fill(&mut self, shape: &FrameItem::Shape, parent_bbox: Option<Rect>) {
    if let Some(gradient) = shape.fill_as_gradient() {
        let relative = resolve_relative(gradient.relative);
        let coords = match relative {
            RelativeTo::Self_ => P272_coords_local(gradient),
            RelativeTo::Parent => apply_parent_transform(
                P272_coords_local(gradient),
                parent_bbox,  // P273.5: agora populated, não sempre None
            ),
        };
        // emit /ShadingType X + coords
    }
}

// Remover #[allow(dead_code)] de apply_parent_transform
- #[allow(dead_code)]
  fn apply_parent_transform(...) { ... }
+ fn apply_parent_transform(...) { ... }
```

### Verificação final

- Cap LOC respeitado.
- `cargo build` sem warning de dead code em `apply_parent_transform`.
- `cargo test --workspace` verde — 2597 → 2597 + tests novos.
- `crystalline-lint .` zero violations.
- Hashes L0 — `layout.md` provavelmente propaga (campo novo no Layouter); `geometry.md` propaga se Rect for adicionado lá. Refactor estrutural observable; L0-anotação justifica propagação (lição P159A/C/D + P274 alinhada).
- Tests P262-P274 inalterados bit-exact.
- DEBT saldo 10 preserved.

---

## §4 — Sub-padrões cumulativos pós-P273.5

| Sub-padrão | Pós-P274 | Pós-P273.5 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 13 | 14 |
| Reutilização literal helpers cross-passos | 13 | 14 |
| Cap LOC hard vs soft explícito | 7 | 8 |
| Aplicação meta-ADR (ADR-0093) | 2 | 3 |
| Aplicação meta-ADR (ADR-0094) | 3 | 4 |
| Diagnóstico imutável | 20 (15º consumo) | 21 (16º consumo) |
| Auditoria condicional | 19 | 20 |
| Auto-aplicação ADR-0065 inline | 19 | 20 |
| Pattern DEBT-37 `cell_origin_*` replicado | N=1 (P84.6 original) | **N=2** (novo — P273.5 reusa pattern) |

Sub-padrão emergente "Pattern DEBT-37 `cell_origin_*` replicado" atinge N=2 — meio caminho do limiar formalização N=3-4. Promoção a ADR meta candidato futuro NÃO reservado.

---

## §5 — Limitações conscientes P273.5

- Lista de contentores que disparam save/restore — fixada na Fase A; pode não cobrir 100% dos contentores vanilla (e.g. Pad, Stack). Refino incremental aceitável; per ADR-0054 graded.
- Coordenadas Rect populated com aproximação ao cursor actual no momento da entrada do contentor; refino com bbox calculado exactamente após layout do body fica fora de escopo (precisaria de pass adicional).
- 3α (página apenas) registado como simplificação aceitável se Fase A demonstrar 3β/3γ impraticável. Per ADR-0054 graded.
- `apply_parent_transform` mantém-se função pura — fórmula transformação P273 inalterada. P273.5 só fornece bbox real.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → `00_nucleo/diagnosticos/typst-passo-273-5A-diagnostico.md`.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8 + revê Decisão 3.
5. Utilizador executa P273.5.B + P273.5.C em Claude Code → `00_nucleo/materialization/typst-passo-273-5-relatorio.md`.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe próximo passo.

---

## §7 — Pendências preservadas

Inalteradas vs P274:

- P-Gradient-CMYK-ICC (S-M).
- ADR-0055bis variant-aware fonts (M).
- P-Footnote-N (M).
- DEBT-33 Bézier bbox (S+M).
- Stroke\<Length\> / Curve / Polygon (S+M).
- Tiling activação.
- Outro cluster — saída Visualize/Gradient.

**Pós-P273.5 fecha cluster Gradient refino estrutural** — pendência P273 §7 (allow(dead_code)) resolvida. Cluster Gradient feature-complete user-facing + adaptive N qualitativo + parent bbox real callsite. Próximo passo natural: sair do cluster Gradient para outro domínio.

---

## §8 — Critério de fecho do passo

P273.5 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.8 cumprido.
- ADR-0091 anotada com Decisão 3 final.
- L1 + L3 alterados dentro do cap LOC.
- `#[allow(dead_code)]` removido de `apply_parent_transform`; `cargo build` sem warning.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P274 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Sub-padrões §4 atualizados.

---

## §9 — Numeração — nota

Spec usa **P273.5** (sub-passo decimal de P273 — pendência directa) em vez de P275 (novo passo principal). Justificação: refino estrutural fechando pendência interna do próprio P273; magnitude S; sem expansão de feature user-facing. Pattern decimal está estabelecido (P156C-L, P157A-C, P159A-G, P270.1-P270.4, P268.1-P268.2 — todos sub-passos decimais). Alternativa P275 viável se preferires numeração principal — decidir antes da Fase A.
