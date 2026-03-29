# ⚖️ ADR-0025: `Int == Float` — desvio de `PartialEq` vs semântica do original

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-27

---

## Contexto

O compilador Typst original compara `Value::Int(i)` e `Value::Float(f)`
com coerção: `i as f64 == f`. Assim `1 == 1.0` é `true` no Typst.

O cristalino usa `#[derive(PartialEq)]` em `Value`, que compara
variantes por estrutura sem coerção. Consequência: `Value::Int(1) == Value::Float(1.0)` é `false` no cristalino.

O Passo 14 implementou `Eq`/`Neq` como `Ok(Value::Bool(a == b))`,
delegando para `PartialEq` derivado. O teste `paridade_int_eq_float`
verifica que retorna `false` — mas isso diverge do oráculo.

---

## Diagnóstico obrigatório

```bash
# Confirmar a semântica exacta do original para Eq com tipos mistos
grep -n "Int.*Float\|Float.*Int\|Eq\|eq.*value" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs \
  | head -30

# Como PartialEq está implementado no Value original (manual ou derive?)
grep -n "PartialEq\|fn eq\b" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | head -15
```

---

## Decisão (após diagnóstico)

### Opção A — implementar PartialEq manual em Value (paridade total)

```rust
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a),   Self::Float(b)) => (*a as f64) == *b,
            (Self::Float(a), Self::Int(b))   => *a == (*b as f64),
            // todas as outras variantes por estrutura:
            _ => core::mem::discriminant(self) == core::mem::discriminant(other)
                 && /* comparação campo-a-campo */,
        }
    }
}
```

Prós: paridade completa com o oráculo.
Contras: PartialEq manual é verboso e precisa de manutenção quando
novas variantes forem adicionadas.

### Opção B — manter derive(PartialEq), implementar coerção em eval_binary_op

```rust
// Em eval_binary_op, casos Eq/Neq com tipos mistos:
(Eq,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool(a as f64 == b)),
(Eq,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a == b as f64)),
(Neq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool(a as f64 != b)),
(Neq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a != b as f64)),
// wildcard geral depois:
(Eq,  a, b) => Ok(Value::Bool(a == b)),
(Neq, a, b) => Ok(Value::Bool(a != b)),
```

Prós: `derive(PartialEq)` mantido (mais simples); coerção apenas onde
o Typst a faz; testes unitários de Value com `assert_eq!` ainda usam
igualdade Rust sem surpresas.
Contras: dois sistemas de igualdade coexistem (`PartialEq` Rust e
operador `==` do Typst) — requer documentação clara.

**Decisão preferida: Opção B** — separa igualdade Rust (para testes
e estruturas de dados) da igualdade Typst (para eval). O comentário
em `eval_binary_op` documenta o desvio explicitamente.

---

## Impacto no teste existente

O teste `paridade_int_eq_float` do Passo 14 verifica `false`.
Com Opção B, este teste precisa de ser corrigido:

```rust
// Antes (Passo 14 — comportamento Rust):
assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
           Ok(Value::Bool(false)));

// Depois (ADR-0025 Opção B — comportamento Typst):
assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
           Ok(Value::Bool(true)));
```

---

## Consequências

**Positivas**: paridade com o oráculo em comparações numéricas mistas;
o desvio deixa de existir.

**Negativas**: Opção A é verbosa; Opção B requer dois sistemas de
igualdade com documentação clara.

**Neutras**: `Value::Int(1) == Value::Float(1.0)` em Rust continua
`false` (PartialEq derivado) — útil para testes unitários de Value
que não querem semântica Typst.

---

## Referências

- Passo 14 — descoberta do desvio de PartialEq
- `lab/typst-original/.../foundations/ops.rs:296` — semântica de divisão
- `lab/typst-original/.../foundations/value.rs` — PartialEq do original
