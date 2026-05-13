# Passo 209C — `Selector::And` + `Selector::Or`

**Série**: 209 (sub-passo `C`).
**Marco**: M9c (Bloco VI — Selector extensions; variants 3-4 de 5).
**Tipo**: implementação de composição N-ária.
**Magnitude**: M (~1-1.5h).
**Pré-condição**: P209B concluído; Selector cristalino
3 variants (`Kind`, `Label`, `Location`); query arms 3;
trait 26 métodos; `EcoVec` em allowlist L1 (ADR-0024);
tests 1915 verdes; 0 violations; ADR-0076 anotado §P209B.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar 2 variants compósitos do Selector enum +
query arms (intersecção/união):

- `Selector::And(EcoVec<Selector>)` — todos os
  sub-selectores devem matchar; query = intersecção
  Vec<Location>.
- `Selector::Or(EcoVec<Selector>)` — pelo menos um
  sub-selector deve matchar; query = união Vec<Location>.

Stdlib API per P209A C3: **Opção (c) Rust API only**.
Sem dispatch via `Value` em `native_query`/`native_locate`
— consumers Rust constroem directamente via
`Selector::And(EcoVec::from_iter(...))`.

Reuso de dados P209A + P209B:

- Estrutura literal fixada em P209A C1.
- `EcoVec<Selector>` paridade vanilla.
- Hash impl derive funciona (EcoVec é Hash quando T é
  Hash; Selector é Hash recursivo).

---

## §2 Cláusulas (5)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. **EcoVec API**: confirmar `ecow::EcoVec` exporta
   `from_iter` ou similar para construção idiomática.
   Verificar pattern já usado em `01_core/` (ex: P207B
   `iter` retorno).
2. **Recursive Hash em enum**: confirmar que `Selector`
   pós-extensão (com `And(EcoVec<Selector>)`) deriva
   Hash automaticamente sem stack overflow. Rust deriva
   Hash recursivo via `discriminant` + fields. Esperado
   funcional.
3. **`query` actual**: confirmar match com 3 arms
   (Kind, Label, Location) per P209B C3. Extensão
   esperada: 2 arms novos.
4. **Helpers de intersect/union de `Vec<Location>`**:
   greps em `01_core/`, `02_shell/`, `03_infra/`. Esperado
   ausente — impl inline em P209C.

Se C1.2 falhar (cycle em derive Hash), registar
`P209C.div-N`.

### C2 — Materializar variants

**L0 primeiro**:

Edição `00_nucleo/prompts/entities/selector.md`:
- +2 variants em Interface.
- +Semântica composição N-ária:
  - `And`: vazio = match-all (intersecção vácua =
    universo).
  - `Or`: vazio = match-none (união vácua = empty).
- +Tests obrigatórios P209C (igualdade, intersecção,
  união).
- +Histórico 2026-05-12 com nota "P209C: +And, +Or per
  C4 P207A; stdlib API Opção (c) Rust-only".

**L1 depois**:

`01_core/src/entities/selector.rs`:
- `+use ecow::EcoVec;` (se não existir).
- 2 variants:
  ```text
  And(EcoVec<Selector>),
  Or(EcoVec<Selector>),
  ```
- Derives preservados.
- 2 tests estruturais (igualdade enum + Hash determinismo
  + composição com self-reference).

### C3 — Query arms (intersecção + união)

`01_core/src/entities/introspector.rs`:

Match exhaustive 3 → 5 arms:

```text
match selector {
    Selector::Kind(kind) => self.query_by_kind(*kind),
    Selector::Label(l) => /* P209B */,
    Selector::Location(loc) => vec![*loc],
+   Selector::And(sels) => {
+       // Intersecção: cada Location deve estar em todos os sub-results.
+       // Vazio: convencionalmente match-all (decisão fixada em P209C).
+       if sels.is_empty() {
+           return vec![]; // empty Vec — semantic "no constraint" é compute-time
+                          // não há "universo" computável sem walk completo.
+       }
+       let mut iter = sels.iter().map(|s| self.query(s));
+       let first: Vec<Location> = iter.next().unwrap();
+       iter.fold(first, |acc, next| {
+           acc.into_iter().filter(|loc| next.contains(loc)).collect()
+       })
+   }
+   Selector::Or(sels) => {
+       // União: Location aparece se em qualquer sub-result.
+       // Vazio: empty Vec.
+       let mut seen = HashSet::new();
+       let mut result = Vec::new();
+       for s in sels {
+           for loc in self.query(s) {
+               if seen.insert(loc) {
+                   result.push(loc);
+               }
+           }
+       }
+       result
+   }
}
```

Notação ilustrativa. Detalhes (HashSet import, perf
optimização para sub-Vec pequenos) decididos durante
implementação.

**Decisão crítica em C3 para `And` vazio**:

- **Opção A** — empty Vec (consistente com "nada match")
  — pragmática para cristalino single-pass.
- **Opção B** — match-all (universo) — paridade vanilla
  semântica mas exige walk completo.

C3 fixa **Opção A** (empty Vec) por simplicidade +
consistência com `Or` vazio. Documentado em L0.

### C4 — Tests

Tests dedicados (~5-7):

- `p209c_selector_and_estrutural` — igualdade enum +
  Hash determinismo recursivo.
- `p209c_selector_or_estrutural` — idem.
- `p209c_query_and_vazio_devolve_empty` — `And(vec![])`
  retorna `vec![]`.
- `p209c_query_or_vazio_devolve_empty` — idem.
- `p209c_query_and_interseccao_de_dois` — intersecção
  de 2 sub-queries via Kind + Label.
- `p209c_query_or_uniao_de_dois` — união de 2
  sub-queries.
- `p209c_query_and_nested` — `And([Or([...]), Kind(...)])`
  composição recursiva (opcional se trivial).

### C5 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1920+ verdes (1915 + 5+); 0 violations.

**Regra empírica P207B §5 não accionada** — Selector
extension não toca trait Introspector. Trait mantém 26
métodos.

Anotar ADR-0076 §P209C: `✅ MATERIALIZADO {data}` +
sumário decisão Opção A em C3.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-209C-relatorio.md`.

Estrutura conciso (~3-5 KB) com 6 §s padrão.

---

## §4 Não-objectivos

- `Selector::Regex` (P209D).
- Stdlib API constructor func `selector(...)` —
  Opção (c) Rust API only fixou.
- `Selector::Where`/`Before`/`After` (fora roadmap M9c).
- Trait method extensions.
- ADR-0077 (P209D).
- Performance optimisation (HashSet para Vec pequenos,
  short-circuit em And vazio middle, etc.) — deferred
  se trivial; documentar em D se feito.

---

## §5 Riscos a evitar

1. **Recursive Hash overflow**: Selector contém
   `EcoVec<Selector>`. Derive Hash recursivo é OK em
   Rust mas C1.2 confirma empíricamente. Se falhar,
   Hash manual via `mem::discriminant` + recursivo.
2. **`And` vazio semântica**: Opção A vs B documentada
   em D. Não inflar com opção B sem consumer real
   imediato — pattern "Caminho 1 anti-inflação" 6ª
   aplicação.
3. **Performance HashSet para n pequeno**: para
   `Or(2-3 selectors)`, `Vec.contains` é mais rápido que
   HashSet. Decidir empíricamente; default HashSet por
   correção semântica.
4. **Composição com Regex**: P209D adiciona `Regex`
   arm — esse arm tem semântica especial (per P209A A3
   stub vec![]). `And`/`Or` que incluem `Regex` ficam
   bem-formados estruturalmente; comportamento de query
   reflecte o stub. Documentar em L0 selector.md como
   limitação herdada.
5. **`Eq` em `EcoVec<Selector>`**: EcoVec deriva Eq
   quando T é Eq. Selector é Eq (P209B). Derive
   funciona.
