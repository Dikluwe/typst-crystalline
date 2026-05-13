# Passo 209B — `Selector::Label` + `Selector::Location`

**Série**: 209 (sub-passo `B`).
**Marco**: M9c (Bloco VI — Selector extensions; 2 dos 5
variants).
**Tipo**: implementação trivial (variants + query arms +
stdlib dispatch).
**Magnitude**: S (~45min).
**Pré-condição**: P209A concluído; Selector cristalino
1 variant (`Kind`); trait 26 métodos; `Introspector::query`
match exhaustive sobre 1 variant; stdlib funcs ~52;
`Value::Location` (P179); `Value::Str` parsing
`<label>` ausente; tests 1907 verdes; 0 violations.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar 2 variants triviais do Selector enum +
query arms + stdlib dispatch (Q3=α + C4 P207A scope):

- `Selector::Label(Label)` — reusa tipo L1 existente.
- `Selector::Location(Location)` — reusa tipo L1
  existente.
- `Introspector::query` match arms: 1 → 3 variants.
- `native_query` + `native_locate` type dispatch:
  `Value::Str` → Label (se `<...>`) ou Kind (caso
  contrário); `Value::Location(loc)` → Location.

Reuso de dados P209A:

- Estrutura literal fixada em C1 (`Label(Label)`,
  `Location(Location)`).
- Stdlib API fixada em C3.
- Plano sub-passos fixado em C4 (Caminho 2).

---

## §2 Cláusulas (4)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. **`Selector` actual**: confirmar
   `01_core/src/entities/selector.rs` ainda tem 1 variant
   `Kind(ElementKind)` + derives standard (`Debug,
   Clone, PartialEq, Eq, Hash`).
2. **`Introspector::query` actual**: confirmar match
   sobre `Selector::Kind(kind)` em `introspector.rs:419`
   (per P209A A3).
3. **`Label` + `Location` API**: confirmar tipos
   acessíveis em `entities::label` + `entities::location`
   com Hash impls existentes.
4. **`native_query` + `native_locate` actual**: confirmar
   pattern de parsing args + dispatch (per P208C).

Se algum falhar, registar `P209B.div-N`.

### C2 — Materializar variants

**L0 primeiro**:

Edição `00_nucleo/prompts/entities/selector.md`:
- Adicionar 2 variants à definição literal.
- Histórico 2026-05-12 com nota "P209B: +Label,
  +Location per Q3=α (humano)".
- Manter referência a P175 minimal mas marcar evolução.

**L1 depois**:

`01_core/src/entities/selector.rs`:
- 2 variants novos em enum.
- Derives preservados (`Hash` automático — Label e
  Location são Hash; verificar).
- 2 tests novos (igualdade estrutural de cada
  variant).

### C3 — Query arms + stdlib dispatch

**`01_core/src/entities/introspector.rs:419`**:

```text
match selector {
    Selector::Kind(kind) => self.query_by_kind(*kind),
+   Selector::Label(label) => self.query_by_label(label)
+       .into_iter().collect(),
+   Selector::Location(loc) => vec![*loc],
}
```

Tests existentes que invocam `query` continuam verdes;
+ 2 tests para os arms novos.

**`01_core/src/rules/stdlib/foundations.rs`**:

`native_query` + `native_locate` dispatch refactor:

```text
let selector = match &args.items[..] {
    [Value::Str(s)] if s.starts_with('<') && s.ends_with('>') => {
        let label_name = &s[1..s.len()-1];
        Selector::Label(Label::new(label_name))
    }
    [Value::Str(s)] => {
        let kind = ElementKind::from_name(s)
            .ok_or_else(|| ...)?;
        Selector::Kind(kind)
    }
    [Value::Location(loc)] => Selector::Location(*loc),
    _ => err,
};
```

Notação ilustrativa. Detalhes (parsing `<...>`, error
messages) decididos durante implementação.

Verificar: `Label::new(...)` ou construtor equivalente
existe; se não, identificar pattern usado em P207B
(`LabelRegistry::iter` retorna `(&Label, &Location)`).

### C4 — Tests + verificação final

Tests dedicados (~4-6):

- `p209b_selector_label_estrutural` — igualdade enum +
  Hash determinismo.
- `p209b_selector_location_estrutural` — idem.
- `p209b_introspector_query_label_via_selector` —
  invocação via Selector::Label retorna mesma Location
  que `query_by_label`.
- `p209b_introspector_query_location_devolve_singleton`
  — Selector::Location(loc) retorna `vec![loc]`.
- `p209b_native_locate_label_syntax` — `locate("<foo>")`
  retorna Location ou None.
- `p209b_native_query_location_arg` — `query(value::location)`
  funciona via Value::Location dispatch.

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1911+ verdes (1907 + 4+); 0 violations.

**Regra empírica P207B §5 não accionada** — Selector
extension não toca trait Introspector. Trait mantém 26
métodos.

Anotar ADR-0076 §P209B: `✅ MATERIALIZADO {data}`.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-209B-relatorio.md`.

Estrutura conciso (~3-5 KB) com 6 §s padrão.

---

## §4 Não-objectivos

- `Selector::And`/`Or` (P209C).
- `Selector::Regex` (P209D).
- `Selector::Where`/`Before`/`After` (fora roadmap M9c).
- Trait method extensions.
- ADR-0077 (P209D).
- L0 prompts stdlib funcs (convenção emergente P208B).

---

## §5 Riscos a evitar

1. **Test sentinela P208C `locate_label_args_pendente_p209`**
   torna-se regressão potencial em P209B. Per relatório
   P208C D4: "P209 deverá remover/actualizar". Verificar
   este test durante C4 — actualizar para reflectir
   semântica `<label>` activa.
2. **Parsing `<label>` syntax simplista**: `<foo>` é
   trivial; `<foo bar>` ou `<>` são edge cases. Tests
   cobrir os casos relevantes; rejeitar inválidos com
   erro contextual.
3. **`Label::new` API**: pode não existir tal qual.
   C1.3 verifica; se ausente, usar pattern alternativo
   (e.g. via `LabelRegistry`).
4. **Hash determinism**: derive automático funciona se
   Label e Location são Hash. Não inventar Hash manual
   sem necessidade. Tests confirmam determinismo.
