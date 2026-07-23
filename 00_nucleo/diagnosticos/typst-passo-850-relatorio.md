# Relatório — typst-passo-850: `Duration` sem representação com sinal — decisão do dono

**Data:** 2026-07-22  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-850.md`).  
**Proveniência das medições:** commit HEAD `92daa9c66` (P847). Medições de P832 reutilizadas; sonda do constructor executada neste passo.  
**Estado:** aguardando decisão do dono — **nenhuma implementação foi feita**.

---

## 1. Problema (reconfirmado)

`#repr(-duration(seconds: 3))` diverge do vanilla:

- **Vanilla:** `duration(seconds: -3)`.
- **Cristalino:** `error: cannot apply Neg to duration`.

Causa: a entidade `Duration` do cristalino usa `u64` de nanossegundos (`01_core/src/entities/duration.rs:17-19`). O vanilla usa representação com sinal.

---

## 2. Sonda do constructor (Passo 1 do passo 850)

Teste executado:

```typ
#repr(duration(seconds: -3))
```

Resultado no cristalino:

```
error: duration(): 'seconds' não pode ser negativo
```

**Conclusão:** o constructor **também rejeita componentes negativos**. Não basta adicionar o braço `Neg` em `operators.rs`; é necessário migrar a representação interna de `Duration` para um tipo com sinal, o que afecta constructor, aritmética, `repr`, ordenação e `cast_duration`.

---

## 3. Escopo real da mudança, se for feita

Migrar `Duration` de `u64` para um tipo com sinal (recomendação: `i128` de nanossegundos, para manter margem de overflow nas operações de multiplicação) toca os seguintes pontos, confirmados por `grep`:

### 3.1. Entidade (`01_core/src/entities/duration.rs`)

- Campo `nanos: u64` → `nanos: i128` (ou `i64`).
- Constantes (`SECOND`, `MINUTE`, `HOUR`, `DAY`, `ZERO`).
- Construtores `from_nanos`, `from_seconds`, `from_minutes`, `from_hours`, `from_days`.
- Acessores `as_seconds`, `as_minutes`, `as_hours`, `as_days` — devem devolver valor assinado.
- `to_string()` — deve representar durações negativas (ex.: `-3d2h30m`).
- Adicionar `impl Neg for Duration`.
- `PartialOrd`/`Ord` continuam a funcionar com `i128`.

### 3.2. Constructor (`01_core/src/engine/stdlib/primitives_constructors.rs`)

- `extract_nonneg` → permitir `Int` negativo.
- Cálculo do total com aritmética assinada.
- Overflow check ajustado aos limites de `i128`.
- `parse_duration("-1h30m")` deve passar a aceitar sinal inicial.

### 3.3. Operadores (`01_core/src/engine/eval/operators.rs`)

- Adicionar `(UnOp::Neg, Value::Duration(d)) => Ok(Value::Duration(-d))`.
- `Add`/`Sub` de `Duration`: usar aritmética assinada; remover rejeição de underflow.
- `Mul`/`Div` de `Duration` por `Int`/`Float`: remover rejeição de negativos; resultado com sinal.
- `Div` `Duration / Duration`: resultado `Float` pode ser negativo.

### 3.4. Repr (`01_core/src/engine/eval/repr.rs`)

- `repr_duration`: componentes podem ser negativos. Vanilla medido:
  - `repr(-duration(seconds: 3))` → `duration(seconds: -3)`
  - `repr(duration(seconds: 3) - duration(seconds: 5))` → `duration(seconds: -2)`

### 3.5. Field access (`01_core/src/engine/eval/bindings.rs`)

- `.seconds`, `.minutes`, `.hours`, `.days` devem devolver `Float` com sinal (podem ser negativos).

### 3.6. Cast (`01_core/src/entities/value.rs`)

- `cast_duration`: remover rejeição de `Int`/`Float` negativos.

### 3.7. Tests

- Actualizar testes em `entities/duration.rs`, `engine/stdlib/primitives_constructors.rs`, `engine/eval/operators.rs`, `engine/eval/repr.rs`, `entities/value.rs`.
- Adicionar testes para `Neg`, constructor negativo, aritmética com sinal, `repr` negativo.

---

## 4. Estimativa de esforço

- **Ficheiros afectados:** ~6-8.
- **Linhas alteradas:** ~80-120.
- **Risco:** baixo-médio. A mudança é mecânica, mas requer atenção a:
  - overflow com `i128` (especialmente em multiplicação);
  - `parse_duration` e `to_string()` para strings negativas;
  - paridade exacta do `repr` com componentes negativas.
- **Tempo estimado:** ~1 passo dedicado.

---

## 5. Opções para decisão

### Opção A — Migrar `Duration` para representação com sinal e fechar o achado

Implementar agora a migração listada na Secção 3. Entregáveis:
- Código alterado nos 6-8 ficheiros identificados.
- `#repr(-duration(seconds: 3))` → `duration(seconds: -3)` (paridade com vanilla).
- Testes novos e existentes a passar.
- Suíte completa verde.

### Opção B — Manter representação sem sinal e formalizar como débito

Não implementar. Criar entrada **DEBT-70** em `00_nucleo/diagnosticos/debt/DEBT.md` documentando:
- `Duration` continua `u64` de nanossegundos.
- Constructor e `Neg` rejeitam negativos.
- Critério de reabertura: decisão do dono ou necessidade de paridade com durações negativas.

---

## 6. Recomendação

A **Opção A** é recomendada porque:

1. O trabalho de P832 deixou `Duration` como a **única** entidade numérica do vanilla sem suporte a sinal (`Angle`, `Ratio`, `Fraction`, `Relative`, `Length` já suportam).
2. A mudança é predominantemente mecânica e de baixo risco.
3. Corrige dois observáveis de língua: `duration(seconds: -3)` e `-duration(seconds: 3)`.
4. Não requer decisões arquitecturais novas — segue o mesmo padrão das entidades já migradas.

---

## 7. Validação

Nenhum código foi alterado neste passo. A suíte mantém-se verde:

- `cargo test --workspace`: **5417 passed; 0 failed** (estado de P847).
- `crystalline-lint .`: exit 0 (aviso pré-existente V7 não relacionado).

---

## 8. Próximo passo

**Decisão do dono necessária.** Por favor indicar:

- **A** — migrar `Duration` para representação com sinal (implementação).
- **B** — formalizar como débito (sem código).
