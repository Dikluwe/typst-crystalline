# Relatório do passo P218 — `native_columns` stdlib + `extract_count` helper

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-218.md`.
**Tipo**: aditivo trivial à stdlib registando `native_columns`
paridade mecânica P156I/J.
**Magnitude planeada**: S (~1h). **Magnitude real**: S (~50min).
**Marco**: nenhum (sétimo passo pós-M9c; segundo sub-passo
sub-fase (b) DEBT-56).

---

## §1 O que foi feito

P218 registou `native_columns(count, body, gutter: ?)` em
`stdlib/layout.rs::register_layout_natives` (via re-export
+ scope register em `eval/mod.rs`). Helper privado
`extract_count(args, fn_name)` novo para `count` posicional
obrigatório (paridade `extract_usize_or_none_min` P157B mas
para posicional não-Option). Stdlib funcs ~53 → 54. 12
tests adicionados (11 unit + 1 E2E). Tests workspace: 1952
→ 1964 verdes. 0 violations. Sem `P218.div-N`.

---

## §2 Inventário pré-P218

`grep -c "scope.define" 01_core/src/rules/eval/mod.rs` ≈ 53
funcs registadas pré-P218. Pós-P218: **54** (+columns).

`native_repeat` (P156J Fase 3 sub-passo 1) localizado em
`stdlib/layout.rs:733`; scope register em `eval/mod.rs:653`.
P218 adicionou `native_columns` imediatamente após (paridade
ordem ADR-0061 Fase 3 sub-passos: repeat → columns).

---

## §3 Helper `extract_count` novo (N=1)

```rust
fn extract_count(args: &Args, fn_name: &str) -> SourceResult<usize> {
    match args.items.first() {
        Some(Value::Int(n)) => {
            if *n < 1 {
                return Err(...);  // count >= 1 obrigatório
            }
            Ok(*n as usize)
        }
        Some(other) => Err(...),  // tipo errado
        None => Err(...),         // ausente
    }
}
```

**Distinto de**:
- `extract_length` — retorna `Length` não `usize`.
- `extract_usize_or_none_min` (P157B) — named opcional
  `Option<usize>`; `extract_count` é posicional obrigatório
  `usize`.

**Privado em `stdlib/layout.rs`**. Promoção a `pub(super)`
ou helper público diferida a N=2-3 reuso (política consistente
com `extract_tracks` P157A `pub(super)` mas não público;
`extract_length` N=7 ainda privado pre-P218).

---

## §4 `native_columns` function + registo

**Função** em `01_core/src/rules/stdlib/layout.rs` após
`native_repeat`:

```rust
pub fn native_columns(_ctx, args, _world, _file, _figure_numbering)
    -> SourceResult<Value>
{
    let count = extract_count(args, "columns")?;     // posicional [0]
    let body = match args.items.get(1) { ... }?;     // posicional [1] Content/Str
    if args.items.len() > 2 { return Err(...) }      // 2 posicionais max
    let gutter = match args.named.get("gutter") { ... }?;  // named opcional
    // Validar gutter negativo + named arg desconhecido
    Ok(Value::Content(Content::Columns { count, gutter, body: Box::new(body) }))
}
```

**Re-export** em `01_core/src/rules/stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_columns, ...
};
```

**Scope register** em `01_core/src/rules/eval/mod.rs`:
```rust
scope.define("columns", Value::Func(Func::native("columns", native_columns)));
```

Forma: `#columns(2)[body]` ou `#columns(2, gutter: 1em)[body]`.

---

## §5 Decisões substantivas

- **`count: usize` posicional vs `Option<usize>` named**:
  fixado posicional obrigatório. Justificação literal —
  vanilla `columns(count, body)` usa `count` como primeiro
  arg sem default sensato (0 colunas é absurdo).
  `extract_count` novo é helper específico (paridade
  estrutural P157B mas semântica diferente).
- **Validação `count >= 1` em stdlib (P218)** vs construtor
  Rust (`Content::columns()` em P217): fixado em stdlib per
  spec P217 §C4. Construtor Rust aceita `count = 0` como
  caso degenerate (consumer P219 pode tratar como
  passthrough).
- **L0 `stdlib.md` não tocado** — convenção emergente
  consolidada (P208B/P208C/P209D/P210B/P217): stdlib funcs
  P156+ inline-documentadas em código sem L0 separado.
  L0 stdlib.md (Passo 17) não rastreia novos funcs.
  Decisão paridade total com P217 (que não tocou L0
  content.md).
- **Helper `extract_count` privado**: promoção pública
  diferida a N=2-3. **Anti-inflação 13ª aplicação
  cumulativa** pós-P205D — recusa promoção prematura.
- **Body Value::Str → `Content::text(s.as_str())`**:
  paridade `native_repeat` P156J. Permite forma
  `#columns(2, "texto")` aceitar string literal como
  shortcut.
- **E2E test simplificado** (texto single-word):
  layout actual fragmenta `FrameItem::Text` por espaços;
  test E2E original "multi-col body" falhou na concatenação;
  substituído por "p218body" single-word. Não é regressão
  P218 — limitação pre-existente layout. Documenta como
  observação no relatório.

---

## §6 Resultados verificação

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 1963 verdes | **1964 verdes** (1675 + 242 + 24 + 2 + 21 = +1 sobre estimativa porque P218 teve 11 unit tests, não 10) |
| `crystalline-lint .` | 0 violations | **0 violations** |
| `crystalline-lint --fix-hashes` | sync se necessário | "Nothing to fix" (L0 não tocado) |
| Tests P218 novos | 10 unit + 1 E2E = 11 | **11 unit + 1 E2E = 12** (+1 unit `body_str_aceita` adicional para cobrir Str→Content) |
| Mudança observable | 0 | **0** (1952 → 1964 = +12 sentinelas; 0 regressões pre-existente) |
| Stdlib funcs registadas | ~53 → 54 | ✓ 54 |
| Borrow checker quebras | 0 | **0** |
| Ajustes manuais | 0-2 | **1** (E2E "multi-col body" → "p218body" single-word per layout fragmentação pre-existente) |

---

## §7 Inventário 148 anotação progresso + ADR-0078

**Inventário 148**:
- §A.5 Layout linha `columns(n)` **mantém-se `ausente`** —
  reclassificação a `implementado` ocorre só pós-P219+P220
  (consumer real + colbreak). Anotação documentada em ADR-0078
  P218 bloco.
- Tabela B.2 actualização **diferida a P221** encerramento
  série (consistente com decisão empírica P217).

**ADR-0078** §"Plano de materialização" anotada com bloco
`### P218 materializado 2026-05-12`:
- Função + helper `extract_count` documentados.
- Re-export + scope register descritos.
- 6 validações implementadas listadas.
- 12 tests adicionados (11 unit + 1 E2E).
- ADR-0064 Caso C cumulativo via `gutter`.
- 1964 verdes / 0 violations.
- 1 ajuste manual E2E documentado.
- Sub-fase (b) DEBT-56: **2/4 sub-passos materializados**.

**Status ADR-0078**: PROPOSTO mantido. Transição IMPLEMENTADO
só em P221 (6 condições satisfeitas).

---

## §8 Próximo sub-passo

P218 fecha segundo sub-passo sub-fase (b) DEBT-56 (2/4).
Decisão humana sobre próxima sessão entre opções:

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **P219** imediatamente — consumer multi-column **real** no Layouter; iteração N regions; `width / count - gutter`; tests `columns(2)`/`columns(3)` produzem regions correctas | **M+ (~3-4h)** | alta (substantiva sub-fase b; humano fixou "focar no Layout até onde der"; P217+P218 prepararam variant + stdlib) |
| **Caminho 2** | Pivot Bloco C P222 — `measure(body)` stdlib expose | S+ (~1-2h) | média (win rápido §A.9 estricto 83% → 100%; isolado de DEBT-56) |
| **Caminho 3** | Pivot P220 — `Content::Colbreak` variant + native_colbreak (skip P219; stub transparente como P217 columns) | S+ (~1.5h) | baixa-média (rompe ordem natural P219→P220; menos coerente — colbreak sem multi-column real é semantic vazia) |
| **Caminho 4** | Adiar Layout; outro módulo | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (P219)** — primeiro
sub-passo substantivo sub-fase (b); converge `Region`/`Regions`
abstraction (P216A+B) + variant (P217) + stdlib (P218) num
consumer real funcional. Marco interno DEBT-56 (não marco
arquitectónico; isso é P221).

**Estado pós-P218**:
- Sub-fase (b) DEBT-56: **2/4 sub-passos** (P217 ✓, P218 ✓;
  P219 P220 pendentes).
- ADR-0078 PROPOSTO; ADR-0061 PROPOSTO ~50% concluído.
- Layout 78% preservado (`columns` ausente até P219+P220
  consumer real).
- Tests workspace: **1964 verdes**; `crystalline-lint`: **0
  violations**.
- Cumulativo P216A+B+P217+P218 = ~325 substituições mecânicas
  + 1 variant + 10 arms + 1 stdlib func + 1 helper + 18
  tests novos em 4 sessões.
- 13 aplicações cumulativas anti-inflação pós-P205D
  (P218 helper `extract_count` privado).
- Pattern emergente "stub transparente" preservado N=1 (P217
  Layout arm; P218 não muda Layouter; P220 Colbreak provável
  N=2).
- Pattern "refactor stacking" preservado N=1 (P218 não muda
  Layouter; estabilidade preservada).
- Pattern "extract helper" cumulativo: extract_length N=8
  (cumulativo via gutter); extract_count N=1 (novo P218).
