# Relatório P184C — `Introspector::figure_number_at_index` trait method

**Data**: 2026-05-03
**Passo**: P184C — método trait + impl + helper `value_at_index` no
`CounterRegistry`
**Resultado**: trait estendido com `figure_number_at_index`; helper
adicionado ao `CounterRegistry`; 5 tests unit do método trait + 3 tests
unit do helper (Δ +8 vs P184B baseline 1.756); zero violations linter.

---

## §1 Resumo

Trait `Introspector` ganhou método novo:

```rust
fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>;
```

Impl em `TagIntrospector` constrói `format!("figure:{}", kind)` e delega
ao `CounterRegistry::value_at_index` (helper novo P184C `.C`); extrai o
número 1-based via `.last().copied()` (counter flat tem snapshot de
tamanho 1).

`CounterRegistry` ganhou helper:

```rust
pub fn value_at_index(&self, key: &str, idx: usize) -> Option<&[usize]>;
```

Acesso por posição (0-indexed) na `history` interna, em vez de por
`Location`. Resolve a necessidade do consumer C3 que itera figures
por kind sem conhecer Locations.

Sem consumer migrado em P184C — C3 (`mod.rs:435–439`) continua a ler de
`state.counter.figure_numbers` legacy (dead code; migra em P184D com
substitution-with-fallback).

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria L0 | ✅ | Trait com 15 métodos; field `counters: CounterRegistry` em `TagIntrospector`; `CounterRegistry` expõe `value_at(key, location)` mas não acesso por idx → **helper necessário**. Decisão registada. |
| `.B` Actualizar L0s | ✅ | `entities/introspector.md` adiciona `figure_number_at_index`; `entities/counter_registry.md` adiciona `value_at_index`. Ambos com entrada nova no histórico. |
| `.C` Helper `value_at_index` | ✅ | `pub fn value_at_index(&self, key: &str, idx: usize) -> Option<&[usize]>` com lookup `self.history.get(key)?.get(idx).map(...)`. 3 tests unit (vazio→None, sequência por idx, kinds isolados). |
| `.D` Trait method + impl | ✅ | Declaração no trait após `is_numbering_active`; impl em `TagIntrospector` delega via `format!("figure:{}", kind)` + `value_at_index` + `.last().copied()`. |
| `.E` 5 tests unit | ✅ | Vazio→None, populate→Some(N), kinds distintos isolados, idx fora de range, default kind via `figure:image`. |
| `.F` Verificação | ✅ | `cargo check` ✓; `cargo test --workspace` 1.504 + 215 + 24 + 21 = **1.764** verdes (Δ vs P184B baseline 1.756: **+8**); `crystalline-lint .` zero violations após `--fix-hashes`. |
| `.G` Encerramento | ✅ | Este relatório. |

---

## §3 Confirmação `.F` — 11 verificações

1. ✅ `cargo check --workspace` passa.
2. ✅ `cargo test --workspace` passa: **1.764 verdes** (Δ +8 vs P184B baseline 1.756).
3. ✅ `crystalline-lint .` zero violations.
4. ✅ `figure_number_at_index` accessível via trait `Introspector`.
5. ✅ `TagIntrospector` impl delega correctamente via `format!("figure:{}", kind)` + `value_at_index`.
6. ✅ Documento sem figures → `figure_number_at_index(*, *) == None` (test `figure_number_at_index_em_introspector_vazio_devolve_none`).
7. ✅ Documento com figures → valor correcto (test `figure_number_at_index_apos_populate_devolve_some` reproduz semântica de P184B arm).
8. ✅ Walk em `introspect.rs` **NÃO modificado**.
9. ✅ Layouter **NÃO modificado** (esperado em P184D).
10. ✅ Snapshot tests ADR-0033 verdes (parte do conjunto 1.764).
11. ✅ Linter passa final (`✓ No violations found`).

---

## §4 Hashes finais L0 modificados

- `00_nucleo/prompts/entities/introspector.md`: Hash do Código `2007e307` (anterior `3f5b73cc`).
- `01_core/src/entities/introspector.rs` `@prompt-hash`: `27c46d3b` (anterior `30bd91d8`).
- `00_nucleo/prompts/entities/counter_registry.md`: Hash do Código `6b03a16c` (anterior `c567fe3a`).
- `01_core/src/entities/counter_registry.rs` `@prompt-hash`: `0147218d` (anterior `885a4296`).

Sincronizados via `crystalline-lint --fix-hashes .`.

---

## §5 Decisões de execução notáveis

1. **Helper foi necessário**: P184A previa cláusula gate trivial em
   `.A.4` para decidir entre delegar directamente ou adicionar
   helper. Inspecção empírica confirmou que o `history` HashMap é
   privado em `CounterRegistry` e os métodos públicos existentes
   (`value`, `value_at`, `format`) não suportam acesso por posição.
   Helper `value_at_index` adicionado em `.C`. Custo extra: +3 tests
   unit em `counter_registry.rs`. Magnitude continua S.

2. **`.last().copied()` no impl trait**: para counters flat (figure,
   equation), o snapshot é `[N]` com tamanho 1, logo `.last()` extrai
   `Some(&N)`. Documentado inline. Para counters hierárquicos
   (heading), `.last()` daria o nível mais profundo — não é problema
   porque figures são sempre flat (`apply_at` com `CounterUpdate::Step`,
   nunca `apply_hierarchical_at`).

3. **Default kind responsabilidade do caller**: o método trait recebe
   `kind: &str` (não `Option<&str>`). O caller (Layouter em P184D)
   resolve `kind.as_deref().unwrap_or("image")` antes de chamar.
   Mantém a interface trait simples e explícita; o default fica
   centralizado em quem já o tem (per `mod.rs:431` e
   `introspect.rs:391` que ambos resolvem `unwrap_or("image")`).

4. **Test `default_kind_image` é trivial mas explícito**: documenta
   o invariante de que a chave correcta para figures sem kind é
   `"figure:image"`. Não testa a transformação `Option<None> →
   "image"` (responsabilidade do caller) — testa que a chave
   resultante é encontrada no registry.

5. **Sem `value_at_index` exposto no trait `Introspector`**:
   `value_at_index` fica como helper público do `CounterRegistry`
   (acesso directo via `self.counters.value_at_index`). Não é
   exposto via trait para evitar duplicação semântica
   (`figure_number_at_index` é a forma trait-level apropriada para
   o uso real).

---

## §6 Estado actual

- **P184 série**: A ✅ B ✅ C ✅ | D–F pendentes.
- **C3 desbloqueio**: eixo 2 atendido (dados em sub-store + método
  de acesso); eixo 1 já era OK; falta consumer migrado (P184D).
- **Trait `Introspector`**: 16 métodos (15 + `figure_number_at_index`).
- **`CounterRegistry`**: 6 métodos públicos (5 + `value_at_index`).
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso**: 5 read-sites migrados (sem mudança em
  P184C; consumer C3 migra em P184D).
- **40 passos executados** (P184B = 39 + P184C = 40).

---

## §7 Pendências cumulativas

Inalteradas em relação ao estado pós-P184B:

- Lacuna #3 (TOC entries via Introspector) — bloqueada, separada da
  série P184.
- DEBT M4-residual a abrir em P183F cobrindo C1+C2+C3 (P184F
  reduzirá para C1+C2 após P184D–E completarem).
- Pendência paralela P182E §5.2 (location-aware Layouter para
  desbloquear C1+C2) — espera M6+.

---

## §8 Próximo passo — P184D

Migrar consumer C3 em `mod.rs:435–439` com substitution-with-fallback:

```rust
let figure_number = self.introspector
    .figure_number_at_index(kind_key, idx)
    .or_else(|| self.counter.figure_numbers
        .get(kind_key).and_then(|v| v.get(idx)).copied())
    .unwrap_or(idx + 1);
```

Trait import local de `Introspector` se necessário. Cabeçalho
`@prompt-hash` actualiza após edit do L0 `rules/layout.md` (se a
alteração afectar a documentação L0).

Pré-condição P184D: este passo concluído. C3 desbloqueio em curso —
infra pronta; falta consumer.
