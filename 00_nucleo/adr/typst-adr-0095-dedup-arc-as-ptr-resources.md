# ⚖️ ADR-0095: Dedup `Arc::as_ptr` resources (PDF reusable objects)

**Status**: `EM VIGOR` (criada directamente; paridade pattern P271
ADR-0093/0094 — meta-ADR formalizando sub-padrão empírico N≥3
cumulativo)
**Data**: 2026-05-18
**Autor**: Humano + IA
**Validado**: Passo P273.17 — formaliza sub-padrão empírico N=3
cumulativo crossing limiar formalização N=3-4 sobre como L3 export
deduplica resources PDF reutilizáveis cuja identidade é dada por `Arc<T>`.
**Diagnóstico prévio**: `00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`
§A.2.1 — verificação literal N=3 aplicações P73 + P263 + P273.12.
**Passo origem**: P273.17 (passo administrativo S+ — reflexão metodológica formal + 3 ADRs meta novas).
**Cluster**: Metodologia / Resources PDF / Deduplicação
**Tipo**: meta-ADR formalizando sub-padrão empírico N=3 cumulativo

---

## Contexto

L3 export (`03_infra/src/export.rs`) gera PDF-1.7 manualmente sem
`crates` externas de PDF (invariante L0 `prompts/infra/export.md`
linha 18). Múltiplas estruturas no `PagedDocument` referenciam
resources reutilizáveis via `Arc<T>` (imagens, gradients, fontes
parsed). PDF emit eficiente exige **dedup**: cada resource único é
serializado uma vez como object referenciado N vezes em vez de N
vezes embutido.

**3 sub-passos consecutivos** (P73 → P263 → P273.12) inauguram e
reaplicam o mesmo mecanismo de dedup baseado em
`Arc::as_ptr(x) as usize` como chave de identidade:

1. **P73 — image_resources**: `Content::Image { data: PtrEqArc<Vec<u8>> }` →
   `FrameItem::Image { data: Arc<Vec<u8>> }` → PDF object dedup via
   `HashMap<usize, ImageResource>` indexado por `Arc::as_ptr(data) as usize`.

2. **P263 — pattern_resources**: gradients (Linear/Radial/Conic)
   embebidos em `Paint::Gradient(Gradient)` → mesma mecânica para
   pattern PDF objects. `HashMap<usize, usize>` em `scan_all_gradients`
   indexado por `Arc::as_ptr(linear/radial/conic) as usize`.

3. **P273.12 — pattern_resources bbox-aware**: pendência P273.6 §9
   ("gradient com mesmo Arc usado em contextos distintos: actualmente
   primeiro wins") corrigida via chave estendida
   `DedupKey { arc_ptr: usize, bbox: Option<RectKey> }` onde
   `RectKey(i32, i32, i32, i32)` é quantização milipontos. Mesmo Arc
   + bboxes diferentes → patterns distintos; mesmo Arc + mesmo bbox
   → dedup preserved.

**N=3 cumulativo** — limiar formalização N=3-4 atingido.

### Marco metodológico

Sub-padrão empírico N=3 demonstra que o mecanismo
**`Arc::as_ptr` → chave HashMap** é **paradigma estável** do L3 export
para dedup de resources PDF reutilizáveis. Não é coincidência
local: cluster Image P73 + cluster Gradient P263 + refino estrutural
P273.12 mostram **paradigma cross-cluster** consolidado.

---

## Decisão

Quando L3 export precisa de deduplicar resources PDF reutilizáveis
cuja identidade canónica é dada por `Arc<T>`, **a chave de dedup é
`Arc::as_ptr(x) as usize`** (cast pointer-to-int).

### Forma canónica (chave simples — N=2 aplicações P73 + P263)

```rust
let ptr = Arc::as_ptr(arc_t) as usize;
// HashMap<usize, ResourceData> ou HashMap<usize, idx_in_Vec<Resource>>
```

### Forma estendida (chave com contexto — N=1 aplicação P273.12)

Quando contexto adicional **distingue ocorrências do mesmo Arc**
(e.g. gradient `relative=parent` em containers com bbox diferente),
chave evolui para tuplo:

```rust
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct DedupKey {
    arc_ptr: usize,
    contexto: Option<TipoContexto>,  // ex.: Option<RectKey>
}
// HashMap<DedupKey, idx_in_Vec<Resource>>
```

**Trade-off PDF size**:
- Mesmo Arc + mesmo contexto → 1 resource (dedup preserved).
- Mesmo Arc + contextos N distintos → N resources (semântica
  correcta).
- Caso comum (contexto único ou idêntico) preserved sem inflação.

### Tipos de contexto candidatos (futuro)

- **`RectKey` quantizado em milipontos** — bbox dependency (P273.12).
- **`ColorSpace` enum** — se gradient renderiza diferente por
  `space` runtime.
- **`Transform matrix quantizado`** — se transform matters.
- **Outros**: aplicar mesmo padrão `Hash + Eq + Copy` para tipo
  contexto.

---

## Análise pureza paridade ADR-0029

L3 puro (export.rs). **L1 não toca**.

`Arc::as_ptr(x) as usize` é cast pointer-to-int — operação puramente
sintáctica sem deref do `T`. `Arc` continua vivo via `PagedDocument`
durante todo o export (per P73 garantia documentada em
`prompts/infra/export.md:27-30`).

Quantização de contexto (P273.12 `rect_to_key`) é matemática pura
sobre `f64`. Sem I/O. **ADR-0029 preserved literal**.

---

## Consequências

### Positivas

- **Dedup PDF resources funcional** para qualquer `Arc<T>`-identidade.
- **PDF size eficiente** — N resources únicos serializados 1×;
  referenciados N× via object references.
- **Custo lookup O(1)** — HashMap padrão.
- **Chave estendida preserva correctness** quando contexto matters
  sem perder dedup quando contextos idênticos.
- **Cross-cluster consolidado** — paradigma uniforme L3 export.

### Negativas

- **`Arc::as_ptr` colisão hipotética**: dois Arcs alocados em endereços
  reusados (após drop) poderiam colidir. Mitigado por `PagedDocument`
  manter Arcs vivos durante export inteiro. Garantia documental.
- **Chave estendida PDF size inflation** pior-caso quando contexto
  varia: N callsites mesmo Arc + N contextos → N resources. Aceito
  por semântica correcta.

### Neutras

- Padrão limita-se a resources Arc-identidade. Resources sem
  Arc-identidade (e.g. derivados puros) usam outros mecanismos
  (e.g. cache por valor).

---

## Alternativas consideradas

| Alternativa | Prós | Contras | Decisão |
|---|---|---|---|
| **`Arc::as_ptr` chave (esta ADR)** | O(1) lookup; identity-based; pureza preserved | Colisão hipotética post-drop | **Escolhido** (N=3 empírico) |
| Dedup por valor (`PartialEq + Hash` no `T`) | Detecta valores equivalentes (não só identidades) | Custo hash + eq por valor; pode dedupar identidades distintas semanticamente | Rejeitado (sobre-dedup; perde identidade) |
| Sem dedup (1 resource por uso) | Simples | PDF cresce N×; tabela de objects bloated | Rejeitado (impacto size inaceitável) |
| Dedup híbrido (Arc + fallback valor) | Cobre ambos casos | Complexo; sem caso empírico | Rejeitado (over-engineering) |

---

## Precedentes citáveis

**3 aplicações empíricas cumulativas**:

- **P73** (2025/2026-XX-XX) — `image_resources` em export.rs.
  Inauguração. Chave: `Arc::as_ptr(data) as usize`.
- **P263** (2026-05-XX) — `pattern_resources` gradients (Linear
  inaugural; Radial/Conic adopção P264+P267). Chave: idem.
- **P273.12** (2026-05-17) — `pattern_resources` bbox-aware via
  `DedupKey { arc_ptr, bbox: Option<RectKey> }`. Chave estendida.

**Documentos cross-reference**:
- `00_nucleo/prompts/infra/export.md:27-30` — garantia `PagedDocument`
  mantém Arcs vivos durante export.
- ADR-0091 §"Anotação cumulativa P273.12" — dedup bbox-aware.
- `00_nucleo/diagnosticos/typst-passo-273-12-diagnostico.md` §A.3 —
  análise decisão DedupKey.

---

## Próximos passos

- **Aplicação a fontes**: P266 já usa pattern similar para font
  resources (não verificado empíricamente para esta ADR — candidato
  inventário XS futuro).
- **Aplicação a outros resources que vierem a surgir** (e.g.
  embedded ICC profiles se P273.14 reabrir como GO; tiling patterns
  se P-Tiling materializar).
- **Reaplicação futura** consolidará sub-padrão para N=4+ cumulativo.

---

## Critério de revisão

ADR-0095 será revisada apenas se:
- Padrão `Arc::as_ptr` mostrar limitações arquitecturais não-
  antecipadas (e.g. colisão post-drop empíricamente observada).
- Alternativa cross-cluster surgir com vantagens demonstráveis.

Caso contrário, padrão preserved como **paradigma L3 export L+
estável**.

---

## Sub-padrão "Passo administrativo XS/S criar/promover ADRs meta"

P273.17 cria 3 ADRs meta (0095 + 0096 + 0097) simultaneamente.
**N=3 cumulativo cluster passes administrativos**:
- **N=1 P156K** — ADR-0064 + ADR-0065 EM VIGOR.
- **N=2 P271** — ADR-0093 + ADR-0094 EM VIGOR.
- **N=3 P273.17 (este passo)** — ADR-0095 + ADR-0096 + ADR-0097 EM VIGOR.

Sub-padrão atinge limiar formalização N=3-4 mas **NÃO formalizado
nesta ADR** (anti-padrão over-formalização per spec §0). Reflexão
meta-meta documentada em `typst-cluster-gradient-reflexao.md` §7
para futuro hipotético.

---

*ADR-0095 imutável produzido em 2026-05-18 como output legítimo
do passo administrativo P273.17. Sub-padrão empírico "Dedup
`Arc::as_ptr` resources" N=3 cumulativo crossing limiar formalização
N=3-4 — paradigma L3 export consolidado cross-cluster.*
