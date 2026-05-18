# Passo P273.11 — Extract Stack measurement helper (cleanup §9 P273.9)

**Tipo**: cleanup XS — substituir replicação inline P273.9 Stack measurement pela chamada ao handler `measure_content_constrained` existente.
**Magnitude estimada**: XS literal (~5-10 LOC L1 net negativo; remoção ~25 LOC + inserção ~5 LOC = -20 LOC líquido).
**Pré-requisitos**: P273.10 fechado (cluster Gradient L1 + L3 cobertos até Group).
**Cluster**: Visualize / Gradient (segundo de até 6 sub-passos para fechar cluster).
**Aplica ADRs**: nenhuma nova; herda contexto P273.9.

---

## §0 — Contexto

P273.9 §A.6 (Stack bbox) materializou medição inline replicando o handler Stack do `measure_content_constrained`:

> P273.9 §9 segundo bullet: "Stack bbox medido replica handler `measure_content_constrained` Stack arm inline em vez de chamar via `Content::Stack {...}` construído — evita alocação de Content temporário; ~30 LOC L1 vs 5 LOC se houvesse helper extraído. Refino candidato XS futuro."

P273.9 §2.2 mostra a replicação literal (~30 LOC inline):

```rust
// P273.9 inline replication
let (stack_w, stack_h) = if dir.is_vertical() {
    let mut max_w = 0.0_f64;
    let mut sum_h = 0.0_f64;
    for child in children.iter() {
        let (w, h) = self.measure_content_constrained(child, stack_avail_w);
        max_w = max_w.max(w);
        sum_h += h;
    }
    (max_w, sum_h + ((n - 1) as f64) * space_pt)
} else {
    let mut sum_w = 0.0_f64;
    let mut max_h = 0.0_f64;
    for child in children.iter() {
        let (w, h) = self.measure_content_constrained(child, stack_avail_w);
        sum_w += w;
        max_h = max_h.max(h);
    }
    (sum_w + ((n - 1) as f64) * space_pt, max_h)
};
```

Esta lógica é literal-equivalente ao handler `Content::Stack` em `measure_content_constrained` (P156I). A replicação inline existe **apenas para evitar reconstruir um `Content::Stack { children, dir, spacing }` temporário** que seria alocado e dropped logo de seguida.

P273.11 extrai a lógica para helper privado partilhado entre o arm Stack do `measure_content_constrained` e o save/restore de `parent_bbox` em P273.9.

### Por que XS

- Lógica idêntica em dois sítios é refactor mecânico.
- Helper é função pura (recebe `&[Content], Dir, Option<Length>, max_w` → `(w, h)`) — sem state, sem side-effects.
- LOC net negativo (~25 LOC removidos + ~10 LOC adicionados = -15 a -20 LOC líquido).
- Sem decisão arquitectural — refactor "menor mudança suficiente".

### Distinção do mecanismo escolhido

§9 P273.9 referenciou "5 LOC se houvesse helper extraído". O caminho proposto:

```rust
// Helper privado em mod.rs ou stack.rs:
fn measure_stack(
    children: &[Content],
    dir: Dir,
    spacing: Option<Length>,
    max_w: f64,
    font_size_pt: f64,
    measure_fn: impl Fn(&Content, f64) -> (f64, f64),
) -> (f64, f64) { ... }
```

Ou alternativa mais simples (método em Layouter):

```rust
impl Layouter {
    fn measure_stack(
        &self,
        children: &[Content],
        dir: Dir,
        spacing: Option<Length>,
        max_w: f64,
    ) -> (f64, f64) { ... }
}
```

Decisão Fase A: função livre com `measure_fn` callback (1α — pura, testável) vs método em Layouter (1β — acoplado mas mais simples; reutiliza `self.measure_content_constrained`).

---

## §1 — Sub-passo P273.11.A — Fase A diagnóstico

**Magnitude**: XS-XXS documental (~10-15 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-11-diagnostico.md`.

### §A.1 — Localização literal dos dois sítios

Listar literal:

- **Sítio 1**: `01_core/src/rules/layout/mod.rs` — handler Stack arm em `measure_content_constrained`. Per P156I prompt confirmação: "sum heights + (n-1)*spacing para vertical; sum widths para horizontal". Confirmar linha exacta.
- **Sítio 2**: `01_core/src/rules/layout/mod.rs` — replicação inline em P273.9 Stack arm save/restore `parent_bbox`. Per P273.9 §2.2 + §A.7 estimativa estimada ~30 LOC.

### §A.2 — Verificação que a lógica é literal-equivalente

Confirmar empírico que:

- Ambos os sítios fazem `for child in children` + `measure_content_constrained(child, max_w)`.
- Ambos diferem por `dir.is_vertical()`.
- Ambos somam `(n-1) * spacing` no eixo principal.
- Resultados são `(f64, f64)` representando `(w, h)` em pt.

Se a lógica divergir em algum aspecto (e.g. P273.9 usa `cursor_x` em vez de `line_start_x` para `max_w` source), documentar a divergência e decidir se o helper unifica ou se a divergência é semântica real.

### §A.3 — Decisão 1 — Forma do helper

Opções:

- **1α — Função livre**: `measure_stack(children, dir, spacing, max_w, measure_fn) -> (f64, f64)`. Mais pura; testável isolada; recebe `measure_fn` callback que injecta `measure_content_constrained` do Layouter. **5 args** no signature.
- **1β — Método em Layouter**: `impl Layouter { fn measure_stack(&self, children, dir, spacing, max_w) -> (f64, f64) }`. Reutiliza `self.measure_content_constrained` internamente. Mais simples; menos args (3). Acoplado a `Layouter` mas dois callers já são métodos de Layouter.
- **1γ — Função em módulo `stack.rs` separado**: Refactor maior; extrai lógica para sub-módulo de layout dedicado a Stack. Magnitude S em vez de XS. **Rejeitada** (excede escopo cleanup).

Recomendação spec: **1β** (método em Layouter). Razões:

1. Dois callers já são métodos de Layouter — sem custo de signature noise.
2. `measure_content_constrained` é `&self` — método encaixa naturalmente.
3. Menos args = menos signature noise = mais legível.
4. Trade-off pureza vs simplicidade — pureza não tem demanda funcional aqui.

Decisão final na Fase A.

### §A.4 — Decisão 2 — Risco "regressão tests P273.9 Stack"

Critério crítico do cleanup: **resultado bit-exact**. Helper extraído deve produzir exactamente o mesmo `(f64, f64)` que a replicação inline.

Verificação:

- Tests P273.9 Stack (`p273_9_*_stack_*`) executam medição via inline e validam observable diff PDF.
- Pós-extracção, mesmos tests devem passar bit-exact.
- Tests P156I `measure_content_constrained` Stack handler também devem passar bit-exact.

Se a Fase A descobrir que os dois sítios usam `max_w` diferente (cursor.x vs line_start_x vs page_width − cursor.x), o helper precisa aceitar `max_w` como parâmetro (não inferir do estado).

### §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P273.9 Stack | Helper bit-exact não-equivalente | Tests P273.9 + P156I bit-exact é critério §A.4 |
| Regressão tests P156I `measure_content_constrained` | Helper devia comportar-se idêntico | §A.2 confirma equivalência |
| Cap LOC L1 estourado | Cleanup é mecânico — sem creep | Cap hard 15 / soft 10 — net -20 LOC esperado (folga 200%) |
| Sub-padrão "Extract helper" inaugural sem precedente | Novo sub-padrão emergente | §A.3 documenta recomendação fundamentada |

### §A.6 — Decisões a fixar na Fase A

1. **Decisão 1** (forma helper): 1α / 1β / 1γ. Recomendação: 1β.
2. **Decisão 2** (max_w source): inferir de `self.regions.current.cursor_x` vs `self.line_start_x` vs param explícito. Critério: §A.2 análise empírica.

### §A.7 — Critério de aceitação Fase A

- §A.1 cita os dois sítios literal (path:linha).
- §A.2 confirma equivalência lógica bit-exact (ou documenta divergência se houver).
- §A.5 risco "regressão" mitigado por critério bit-exact §A.4.
- §A.6 Decisões 1+2 fixadas.

---

## §2 — Sub-passo P273.11.B — Sem anotação ADR

**Magnitude**: zero documental.

Cleanup XS sem decisão arquitectural; sem padrão metodológico novo (Extract helper é prática standard); sem ADR para anotar.

ADR-0091 décima primeira anotação (P273.10) permanece a anotação corrente do cluster Gradient.

---

## §3 — Sub-passo P273.11.C — Materialização

**Magnitude**: XS literal (~5 min execução).

### Ordem literal

1. Fase A §1 produzida.
2. `crystalline-lint --fix-hashes` (refactor cosmético; sem alteração L0 esperada).
3. **Testes-primeiro** — confirmar que tests P273.9 Stack + P156I `measure_content_constrained` Stack handler executam verdes em árvore actual.
4. Código:
   - **Adicionar** método helper em `Layouter` (Decisão 1β): `fn measure_stack(...)`.
   - **Substituir** inline replication em P273.9 Stack arm (~30 LOC → ~5 LOC chamada).
   - **Substituir** handler arm Stack em `measure_content_constrained` se também beneficiar (~25 LOC → ~5 LOC chamada).
5. Verificação final.

### Cap LOC (ADR-0094 Pattern 1)

- **L1 hard cap**: ≤ 15 LOC net adicionado (ou negativo).
- **L1 soft cap**: ≤ 10 LOC net adicionado.
- **L1 net esperado**: -15 a -20 LOC (remoção > adição).
- **L3 hard cap**: 0 LOC.
- **Tests hard cap**: 0 novos (cleanup; tests existentes validam).
- **Tests soft cap**: 0.

### Tests propostos

**Sem novos tests propostos** — cleanup é mecânico; tests P273.9 Stack + P156I existentes validam bit-exact equivalence. Adicionar tests novos seria over-engineering para refactor "menor mudança suficiente".

Excepção opcional (Decisão Fase A): se Fase A descobrir divergência empírica entre os dois sítios (§A.2), pode justificar 1-2 tests unitários do helper extraído para clarificar semântica.

### Alterações esperadas no código

```rust
// L1 — 01_core/src/rules/layout/mod.rs

impl Layouter {
    // P273.11 — método novo (Decisão 1β)
    fn measure_stack(
        &self,
        children: &[Content],
        dir: Dir,
        spacing: Option<Length>,
        max_w: f64,
    ) -> (f64, f64) {
        if children.is_empty() {
            return (0.0, 0.0);
        }
        let space_pt = spacing
            .map(|s| s.resolve_pt(self.font_size_pt.0))
            .unwrap_or(0.0);
        let n = children.len();
        if dir.is_vertical() {
            let mut max_child_w = 0.0_f64;
            let mut sum_h = 0.0_f64;
            for child in children {
                let (w, h) = self.measure_content_constrained(child, max_w);
                max_child_w = max_child_w.max(w);
                sum_h += h;
            }
            (max_child_w, sum_h + ((n - 1) as f64) * space_pt)
        } else {
            let mut sum_w = 0.0_f64;
            let mut max_child_h = 0.0_f64;
            for child in children {
                let (w, h) = self.measure_content_constrained(child, max_w);
                sum_w += w;
                max_child_h = max_child_h.max(h);
            }
            (sum_w + ((n - 1) as f64) * space_pt, max_child_h)
        }
    }
}

// P273.9 Stack arm save/restore — substituição:
- let (stack_w, stack_h) = if dir.is_vertical() {
-     // ... 25 LOC inline ...
- };
+ let (stack_w, stack_h) = self.measure_stack(children, *dir, *spacing, stack_avail_w);
if stack_w > 0.0 && stack_h > 0.0 {
    self.parent_bbox = Some(/* ... */);
}

// measure_content_constrained Stack arm — substituição análoga (se aplicável):
- Content::Stack { children, dir, spacing } => {
-     // ... 25 LOC inline ...
- }
+ Content::Stack { children, dir, spacing } => {
+     self.measure_stack(children, *dir, *spacing, max_w)
+ }
```

### Verificação final

- L1 hard cap respeitado (net negativo, com folga).
- `cargo build` sem novos warnings.
- `cargo test --workspace` verde — 2632 → 2632 (zero tests novos; refactor preserved bit-exact).
- `crystalline-lint .` zero violations.
- Tests P273.9 Stack inalterados bit-exact.
- Tests P156I `measure_content_constrained` Stack handler inalterados bit-exact.
- Hashes L0 — preserved (refactor L1 puro sem alterar interface tipo).
- DEBT saldo 10 preserved.

---

## §4 — Sub-padrões cumulativos pós-P273.11

| Sub-padrão | Pós-P273.10 | Pós-P273.11 |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 18 | 18 (preserved — sem anotação) |
| Reutilização literal helpers cross-passos | 17 | 17 (preserved) |
| Cap LOC hard vs soft explícito | 13 | 14 |
| Aplicação meta-ADR (ADR-0093) | 7 | 7 (preserved — sem anotação) |
| Aplicação meta-ADR (ADR-0094) | 9 | 10 |
| Diagnóstico imutável | 26 | 27 (22º consumo) |
| Sub-passos consecutivos do mesmo cluster | N=6 emergente | **N=7 cumulativo emergente** |
| **Extract helper de replicação inline** | N=0 | **N=1 inaugural emergente** |

Sub-padrão "Extract helper de replicação inline" inaugurado por P273.11. Precedente: cleanup explícito de replicação inline (acordo no momento de criar; tratado como refino subsequente). Limiar formalização N=3-4 longe.

### Sub-padrão NÃO aplicado (anotado por contraste)

- **Pattern DEBT-37**: preserved N=4.
- **Template-passo replicado literal**: preserved N=2.
- **Layout duplo arquitectural aceite**: preserved N=1.
- **L3-only parent_bbox**: preserved N=1.
- **Bug latent corrigido em scope creep**: preserved (N=1 ou N=2 consoante calibração — ver discussão P273.10).
- **Anotação cumulativa em vez de ADR nova**: preserved (cleanup XS sem decisão arquitectural).

---

## §5 — Limitações conscientes P273.11

- Helper é método de `Layouter` (Decisão 1β) — não é "puro" no sentido funcional estrito. Refino para função livre (1α) candidato XS futuro se houver demanda de testabilidade isolada.
- Refactor é mecânico; preserva semântica bit-exact. Sem refino de comportamento.
- Cleanup intra-cluster Gradient (P273.9 §9 segundo bullet) — não toca outros sítios análogos noutros clusters.

---

## §6 — Workflow operacional

**Nota operacional**: P273.11 é XS literal — passo é pequeno o suficiente para potencialmente unir Fase A + materialização num único batch executado de uma vez em Claude Code (precedente P273.8). Spec preserva separação metodológica formalmente.

1. Utilizador lê esta spec.
2. Utilizador executa Fase A + C em Claude Code (single-shot opcional).
3. Utilizador upload do relatório.
4. Claude web analisa + propõe **P273.12** (próximo da sequência: Dedup bbox-aware).

---

## §7 — Pendências preservadas pós-P273.11

Inalteradas vs P273.10 (nível cluster):

- **P273.12** — Dedup bbox-aware (S-M; refino arquitectural per P273.6 §9).
- **P273.13** — CMYK-ICC krilla paridade (S-M; **VERIFICAR Fase A se krilla API existe**).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por DEBT-56).

Pendências preservadas no nível geral inalteradas vs P273.10.

---

## §8 — Critério de fecho do passo

P273.11 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.7 cumprido.
- L1 alterado dentro do cap LOC (net negativo esperado).
- Tests workspace verdes 2632 preserved bit-exact.
- Lint zero.
- Tests P273.9 Stack inalterados bit-exact.
- Tests P156I `measure_content_constrained` Stack handler inalterados bit-exact.
- DEBT saldo 10 preserved.
- Sub-padrões §4 atualizados (mínimos — passo XS).

---

## §9 — Numeração

Spec usa **P273.11** continuando a sequência decimal. Segundo passo da sub-sequência "terminar cluster Gradient" (escopo máximo).

Sequência prevista preserved:
- ✓ P273.10 — Group L3-only (S; fechado).
- **P273.11 — Extract Stack helper** (XS; este passo).
- P273.12 — Dedup bbox-aware (S-M).
- P273.13 — CMYK-ICC krilla (S-M; verificar API).
- P273.14 — Bbox medido pós-layout (M).
- P273.15 — Bbox.y topo-exacto inline (M-L; bloqueado DEBT-56).
