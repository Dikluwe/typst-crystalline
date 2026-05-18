# ⚖️ ADR-0096: Pattern DEBT-37 — campo Layouter consumer-pending (refino incremental sustentável)

**Status**: `EM VIGOR` (criada directamente; paridade pattern P271
ADR-0093/0094 — meta-ADR formalizando sub-padrão empírico N≥4
cumulativo)
**Data**: 2026-05-18
**Autor**: Humano + IA
**Validado**: Passo P273.17 — formaliza sub-padrão empírico N=4
cumulativo crossing limiar formalização N=3-4 com folga sobre
refino estrutural via campos Layouter introduzidos com consumer
adiado.
**Diagnóstico prévio**: `00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`
§A.2.2 — verificação literal N=4 aplicações P84.6 + P273.5 + P273.6 + P273.9.
**Passo origem**: P273.17 (passo administrativo S+).
**Cluster**: Metodologia / Layouter / Refino estrutural incremental
**Tipo**: meta-ADR formalizando sub-padrão empírico N=4 cumulativo

---

## Contexto

Refino estrutural do `Layouter` (`01_core/src/rules/layout/`) por
vezes requer introduzir **campo novo na struct `Layouter`** para
persistir estado entre arms de `Content::*` ou entre Layouter L1 e
emit L3. Materializar **introdução + consumer real no mesmo passo**
cria magnitude M+ com risco de scope creep.

**Pattern DEBT-37**: refino divide-se em **2 passos sequenciais**:

1. **Passo introdução** (magnitude S): campo `Option<T>` adicionado
   ao struct `Layouter`. Set/reset (save/restore LIFO) no arm
   consumidor estrutural. `#[allow(dead_code)]` enquanto consumer
   real não existe (consumer-pending). DEBT registado em
   `00_nucleo/DEBT.md` documentando consumer adiado.

2. **Passo consumer** (magnitude S separadamente): consumer real
   activado (e.g. L3 dispatcher consulta campo; cascade ~N sites
   FrameItem destructure). `#[allow(dead_code)]` removido. DEBT
   fechado.

**4 aplicações cumulativas** (P84.6 → P273.5 → P273.6 → P273.9):

1. **P84.6 (Grid cell)** — Inauguração. `cell_origin_x/y/w: Option<f64>`
   adicionado ao Layouter. Save/restore no arm `Content::Grid` cell
   loop. DEBT-37 registado consumer-pending. Consumer real activado
   em passos subsequentes do cluster Grid.

2. **P273.5 (apply_parent_transform + parent_bbox)** — Reaplicação
   inaugural cluster Gradient. `parent_bbox: Option<Rect>` adicionado
   ao Layouter. `apply_parent_transform` em L3 (consumer fallback
   `page_bbox` identity). `#[allow(dead_code)]` no campo Layouter
   (consumer pending real).

3. **P273.6 (Block save/restore + cascade ~86 sites)** — Consumer
   real activado para P273.5. `FrameItem::Shape` ganha campo
   `parent_bbox_at_emit: Option<Rect>` via cascade ~86 sites bulk-
   patched. Arm `Content::Block` save/restore real do `parent_bbox`.
   L3 dispatcher consume `parent_bbox_at_emit` real
   (`effective_parent_bbox`). `#[allow(dead_code)]` removido P273.6
   §"`#[allow(dead_code)]` no Layouter fechado".

4. **P273.9 (Grid cell parent_bbox paralelo)** — Reaplicação P84.6
   pattern para `parent_bbox`. `parent_bbox` save/restore no arm
   `Content::Grid` paralelo ao `cell_origin_*` save/restore existente.
   Consumer real (L3 dispatcher P273.6) reused.

**N=4 cumulativo** — limiar formalização N=3-4 crossado com folga.

### Marco metodológico

Sub-padrão empírico N=4 mostra que **refino estrutural Layouter
incremental** é **paradigma sustentável** quando dividido em passos
de magnitude S separados. Não é coincidência local: cluster Grid
(P84.6) + cluster Gradient (P273.5/6/9) demonstram cross-cluster.

---

## Decisão

Quando refino estrutural do `Layouter` precisa de **campo novo que
será consumido em passo subsequente** (não no passo da introdução),
o pattern canónico é:

### Passo introdução (magnitude S)

```rust
// 1. Campo Option<T> adicionado ao Layouter struct.
pub struct Layouter {
    // ... campos existentes ...

    /// **P<X> — refino estrutural**: descrição do estado a persistir.
    /// Consumer real activado em P<Y> futuro (DEBT-NN registado).
    #[allow(dead_code)]  // consumer-pending; remover quando consumer activo
    pub(super) campo_novo: Option<TipoT>,
}

// 2. Init no construtor:
campo_novo: None,

// 3. Save/restore LIFO no arm consumidor estrutural (Content::*):
let saved = self.campo_novo;
if condição_arquitectural_clara {
    self.campo_novo = Some(valor_computado);
}
self.layout_content(body);
self.campo_novo = saved;
```

### DEBT registado durante consumer-pending

```markdown
# em 00_nucleo/DEBT.md

## DEBT-NN — Campo Layouter `campo_novo` consumer-pending

**Aberto em**: Passo P<X> (data).
**Status**: consumer-pending; `#[allow(dead_code)]` activo.
**Bloqueia**: passo P<Y> futuro que active consumer real.
**Fechamento**: quando `#[allow(dead_code)]` removível.
```

### Passo consumer (magnitude S separadamente)

```rust
// 1. Campo `Option<T>` no FrameItem ou structura de saída para L3:
pub enum FrameItem {
    // ...
    Shape {
        // ... campos existentes ...
        /// **P<Y> — refino consumer P<X>**: estado capturado do
        /// `Layouter.campo_novo` no momento do emit.
        campo_at_emit: Option<TipoT>,
    },
}

// 2. Cascade ~N sites FrameItem destructure (bulk-patch via script
//    quando N>10).

// 3. Emit shape sites populam `campo_at_emit: self.campo_novo`.

// 4. L3 dispatcher consume `campo_at_emit` via
//    .or(parent_bbox_override).or(fallback) per Inner-wins paridade.

// 5. `#[allow(dead_code)]` removido do campo Layouter.

// 6. DEBT-NN fechado em 00_nucleo/DEBT.md.
```

---

## Análise pureza paridade ADR-0029

**L1 puro absoluto**. Campo é **tipo dado** (`Option<f64>` ou
`Option<Rect>`); save/restore LIFO é gestão de RAM. Sem I/O. Sem
side-effects sistémicos.

`#[allow(dead_code)]` é dívida visível **mas não viola pureza** —
apenas marca consumer-pending. Disciplina: dívida força fechamento
no passo subsequente.

---

## Consequências

### Positivas

- **Refino incremental sustentável** — campo introduzido com
  magnitude S; consumer activado com magnitude S separadamente.
  Total: 2× S < 1× M+.
- **Risco scope creep reduzido** — cada passo isolado tem critério
  de fecho claro.
- **DEBT registado durante consumer-pending** — dívida visível;
  força fechamento.
- **`#[allow(dead_code)]` disciplina** — sem warning silencioso;
  remoção é critério de fecho.
- **Cross-cluster consolidado** — Grid + Gradient demonstram
  paradigma uniforme.

### Negativas

- **Estado intermédio temporário**: entre passo introdução e
  passo consumer, código tem campo "morto" (mas marcado).
- **Coordenação entre passos**: passo consumer depende de passo
  introdução. Gestão sequencial necessária.

### Neutras

- Padrão limita-se a campos persistidos no `Layouter`. Estado
  emit-time L3 (e.g. P273.10 L3-only parent_bbox) usa padrão
  diferente (parameter threading).

---

## Alternativas consideradas

| Alternativa | Prós | Contras | Decisão |
|---|---|---|---|
| **Pattern DEBT-37 (esta ADR)** | Refino incremental; magnitude controlada; DEBT visível | Estado intermédio 1 passo | **Escolhido** (N=4 empírico) |
| Materializar campo + consumer mesmo passo | Sem estado intermédio | Magnitude M+; scope creep | Rejeitado (per ADR-0054 graded) |
| Não introduzir campo (refactor estrutura existente) | Sem campo novo | Refactor pode ser maior; estado existente pode não suportar | Rejeitado (depende caso a caso) |
| Campo `Option<T>` sem `#[allow(dead_code)]` | Dívida menos visível | Warning silencioso; risk de não fechar | Rejeitado (disciplina perdida) |

---

## Precedentes citáveis

**4 aplicações empíricas cumulativas**:

- **P84.6** (Cluster Grid) — Inauguração. `cell_origin_x/y/w: Option<f64>`.
  DEBT-37 registado.
- **P273.5** (Cluster Gradient) — `apply_parent_transform` + `parent_bbox`.
  `#[allow(dead_code)]` consumer-pending.
- **P273.6** (Cluster Gradient) — Consumer real Block + cascade ~86
  sites. `#[allow(dead_code)]` removido. DEBT-37 fechado.
- **P273.9** (Cluster Gradient) — Reaplicação Grid cell paralelo a
  `cell_origin_*` (consumer real reused).

**Documentos cross-reference**:
- `00_nucleo/DEBT.md` DEBT-37 (Grid cell) — original; pattern named
  por origem.
- ADR-0091 §"Anotação cumulativa P273.6" §"Pattern DEBT-37
  `cell_origin_*` replicado N=3" — registo formal pre-P273.17.
- ADR-0091 §"Anotação cumulativa P273.9" §"Pattern DEBT-37 N=4
  cumulativo crossing limiar".
- `00_nucleo/diagnosticos/typst-passo-273-6A-diagnostico.md` — Fase
  A do consumer real Block.

---

## Próximos passos

- **Aplicação a outros campos Layouter** que requerem
  consumer-pending. Candidatos:
  - `cell_available_h` (existe em Grid; consumer já activo via
    `regions.cell.height` per P246).
  - Futuros campos para refino multi-region (se Fase 4 ADR-0078 for
    materializada).
- **Reaplicação futura** consolidará sub-padrão para N=5+
  cumulativo.

---

## Critério de revisão

ADR-0096 será revisada apenas se:
- Pattern demonstrar limitações arquitecturais não-antecipadas
  (e.g. `#[allow(dead_code)]` esquecido = dívida invisível persistente).
- Alternativa cross-cluster surgir com vantagens demonstráveis.

Caso contrário, padrão preserved como **paradigma refino estrutural
sustentável**.

---

*ADR-0096 imutável produzido em 2026-05-18 como output legítimo
do passo administrativo P273.17. Sub-padrão empírico "Pattern
DEBT-37 `cell_origin_*` replicado" N=4 cumulativo crossing limiar
formalização N=3-4 com folga — paradigma refino estrutural Layouter
incremental consolidado cross-cluster (Grid + Gradient).*
