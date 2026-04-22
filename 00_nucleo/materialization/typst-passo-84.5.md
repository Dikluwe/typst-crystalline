# Passo 84.5 — `Value::Align` com composição simbólica (DEBT-36)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — `HAlign`, `VAlign`,
  `Align2D`, `Align2D::from_string`. Introduzidos no Passo 82.
- `01_core/src/entities/value.rs` — enum `Value`. Variante
  `Value::Align` será adicionada.
- `01_core/src/rules/stdlib.rs` — `native_align` e `native_place`.
  Consomem `Value::Str` via `Align2D::from_string`.
- `01_core/src/rules/eval.rs` — `eval_binary_op`. Precisa de tratar
  `BinOp::Plus` entre dois `Value::Align`.
- `00_nucleo/DEBT.md` — DEBT-36 em Secção 1.
- `00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md` —
  regra de fidelidade ao vanilla para tipos tipográficos. A lista
  explícita não inclui `Alignment`, mas o espírito aplica-se.

Pré-condição: `cargo test` — 904 testes (734 L1 + 170 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.4 concluído. DEBT-22
encerrado, DEBT-39 aberto.

---

## Decisão de design herdada do Passo 82

O enunciado original do Passo 82 registou no próprio DEBT-36:

> "Resolução: quando o parser suportar `Value::Align` com composição,
> substituir `Align2D::from_string` pelo parse directo da variante."

A **decisão arquitectural já existe**. Este passo é a **materialização**
dessa decisão, não uma revisão. Não há escolha a fazer sobre
*se* `Value::Align` entra no enum `Value` — entra. A pergunta é
*como* migrar sem quebrar código existente.

---

## Natureza deste passo

Duas fases separadas:

**Fase de diagnóstico (bloqueante)**: Tarefa 1 — inventariar a
estrutura vanilla de `Alignment`, o estado actual de `Value`, e
todos os call sites de `Align2D::from_string` / sintaxe string de
`align`/`place`. Reportar ao utilizador antes de qualquer
alteração.

**Fase de implementação (dependente)**: Tarefa 2 executa a versão
mínima (compatibilidade com sintaxe string preservada). Tarefa 3
actualiza DEBT-36.

**Regra absoluta**: Claude Code **não escolhe entre versão mínima
e completa** — este passo é explicitamente versão mínima. Se o
diagnóstico revelar algo inesperado que torne a versão mínima
inviável, reportar ao utilizador e parar.

---

## Restrições arquiteturais

1. **ADR-0029** — tipos tipográficos seguem a arquitectura vanilla
   sem simplificações de conveniência. `Alignment` não está na lista
   explícita da ADR-0029, mas é tipo do mesmo género que `Length`,
   `Angle`, `Color`. Esta lacuna será corrigida no Passo 84.7
   (auditoria dos ADRs). Até lá, aplicar o espírito: **diagnosticar
   a estrutura vanilla antes de materializar**.

2. **Clone via `Arc` é permitido, cópia profunda no hot path deve
   ser evitada.** `Align2D` é `Copy` (8 bytes: dois `Option<u8>`
   efectivamente), portanto não precisa de `Arc`. Excepção legítima
   à regra do `Arc` — o tipo é suficientemente pequeno.

3. **Sem `unsafe`**.

4. **Sem cascata de alterações em testes existentes.** Se o
   diagnóstico revelar que muitos testes L1 ou L3 usam a sintaxe
   string de `align("center", ...)`, **essa sintaxe é preservada**.
   A remoção da sintaxe legacy fica para um passo dedicado,
   posterior.

---

## Tarefa 1 — Diagnóstico obrigatório

### 1.1 — Estrutura vanilla de `Alignment`

```bash
# Definição do tipo no vanilla
grep -B 2 -A 25 "pub enum Alignment\|pub struct Alignment\|pub enum Align" \
  lab/typst-original/crates/typst-library/src/layout/align.rs 2>/dev/null

# Se o path acima não existir, localizar
find lab/typst-original -name "align*.rs" -type f 2>/dev/null | head -5

# HAlignment e VAlignment no vanilla
grep -B 2 -A 15 "pub enum HAlignment\|pub enum VAlignment\|HAlign\|VAlign" \
  lab/typst-original/crates/typst-library/src/layout/align.rs 2>/dev/null \
  | head -60

# Operador + entre alinhamentos (se existir no vanilla)
grep -B 2 -A 20 "impl Add for Alignment\|impl Add<.*> for Alignment\|Align.*Add" \
  lab/typst-original/crates/typst-library/src/layout/align.rs 2>/dev/null
```

Reportar o output. Comparar com `Align2D` actual no projecto. Os
dois devem ser isomorfos (mesmo conteúdo informativo); se diferirem
em forma, documentar no relatório.

### 1.2 — Enum `Value` actual

```bash
# Variantes existentes de Value
grep -B 2 -A 40 "pub enum Value" 01_core/src/entities/value.rs

# Métodos de conversão actuais (cast_str, cast_int, etc.)
grep -n "pub fn cast_\|fn cast_str\|fn as_str" 01_core/src/entities/value.rs
```

Verificar padrão de nomenclatura das variantes actuais (`Value::Str`,
`Value::Int`, etc.). A nova variante seguirá o mesmo padrão:
`Value::Align(Align2D)`.

### 1.3 — Call sites de `Align2D::from_string`

```bash
# Todos os usos directos
grep -rn "Align2D::from_string\|from_string" 01_core/src/ 03_infra/src/

# Call sites de native_align e native_place
grep -B 3 -A 15 "fn native_align\|fn native_place" 01_core/src/rules/stdlib.rs
```

Inventariar:
- Quantas ocorrências de `from_string` existem.
- Se `from_string` é chamado fora de `native_align`/`native_place`.

### 1.4 — Testes que usam sintaxe string de `align`/`place`

```bash
# Testes L1 em 01_core
grep -rn "align(\"" 01_core/src/
grep -rn "place(\"" 01_core/src/

# Testes L3 em 03_infra
grep -rn "align(\"" 03_infra/src/
grep -rn "place(\"" 03_infra/src/
```

Inventariar:
- Número aproximado de testes que usam sintaxe string.
- Testes críticos identificados (os 3 do Passo 82, os 3 do Passo 83
  de Grid).

**Confirmação crítica**: a versão mínima **preserva** a sintaxe
string. Estes testes devem continuar a passar sem alteração. Se
o diagnóstico revelar que muitos testes dependem da sintaxe string,
isso é sinal de que a decisão de preservar está correcta.

### 1.5 — `eval_binary_op` e tratamento de `BinOp::Plus`

```bash
# Como Plus é tratado hoje entre Value's
grep -B 2 -A 30 "BinOp::Plus\|eval_binary_op" 01_core/src/rules/eval.rs \
  | head -80

# Padrão de match existente (para acrescentar braço de Align)
grep -B 1 -A 5 "Value::Int.*Value::Int\|Value::Str.*Value::Str" 01_core/src/rules/eval.rs \
  | head -30
```

Identificar o ponto exacto onde adicionar o braço:
```rust
(Plus, Value::Align(a), Value::Align(b)) => ...
```

### 1.6 — Registo de constantes top-level

```bash
# Como constantes como `none`, `auto` são registadas no scope global
grep -n "ctx.register\|scope.insert\|register_const\|stdlib" 01_core/src/rules/eval.rs \
  | head -20

# Se existe `none` como constante pré-registada
grep -rn "Value::None\|register.*none\|\"none\"" 01_core/src/rules/
```

Identificar o mecanismo pelo qual constantes são expostas ao utilizador
(`none`, `auto`, booleans). As constantes de alinhamento (`left`,
`center`, `right`, `top`, `horizon`, `bottom`) seguirão o mesmo
mecanismo.

---

## Tarefa 1.5 — Classificação e decisão

Com base no diagnóstico:

### Cenário esperado (comum)

- `Alignment` vanilla é estruturalmente equivalente a `Align2D`
  actual (dois campos opcionais, horizontal e vertical).
- `Value` tem ~10–15 variantes actuais; adicionar uma mais é
  trivial.
- `from_string` usado em 2 call sites (apenas `native_align` e
  `native_place`).
- 5–15 testes usam sintaxe string — preservar.

Se este cenário é verdadeiro, prosseguir para Tarefa 2.

### Cenário inesperado — reportar ao utilizador

Qualquer destes sinais faz parar:

- `Alignment` vanilla tem estrutura materialmente diferente (ex:
  um `enum` plano em vez de dois campos opcionais).
- Existe `Value::Alignment` ou similar já no enum (DEBT-36 pode
  estar parcialmente resolvido sem actualização do DEBT.md).
- `from_string` é chamado em mais sítios que `native_align`/`native_place`
  — indica acoplamento não previsto.
- Mais de 30 testes usam a sintaxe string — versão mínima continua
  válida, mas o relatório deve documentar o tamanho real da
  migração pendente.

---

## Tarefa 2 — Implementação (versão mínima)

### 2.1 — Adicionar `Value::Align(Align2D)`

Em `01_core/src/entities/value.rs`:

```rust
use crate::entities::layout_types::Align2D;

pub enum Value {
    // ... variantes existentes ...
    Align(Align2D),
}
```

Actualizar todos os `match` exaustivos sobre `Value` (comparação,
display, conversão) para incluir o braço `Align`. O compilador
Rust sinaliza cada um — o trabalho é mecânico.

Adicionar método `cast_align`:

```rust
impl Value {
    pub fn cast_align(self) -> Option<Align2D> {
        match self {
            Value::Align(a) => Some(a),
            _ => None,
        }
    }
}
```

`Value::Str` permanece válido — a compatibilidade com sintaxe
string é preservada.

### 2.2 — `native_align` e `native_place` aceitam ambos

Em `01_core/src/rules/stdlib.rs`:

```rust
pub fn native_align(_ctx: &mut EvalContext, args: &Args)
    -> Result<Value, String>
{
    // Primeiro argumento: Value::Align preferencial, Value::Str fallback.
    let alignment = match args.positional_items().first() {
        Some(Value::Align(a)) => *a,
        Some(Value::Str(s))   => Align2D::from_string(s.as_str()),
        Some(_) => return Err(
            "align() primeiro argumento deve ser alinhamento ou string".to_string()
        ),
        None => Align2D::default(),
    };

    let body = args.positional_items()
        .get(1)
        .and_then(|v| v.clone().cast_content())
        .ok_or_else(|| "align() exige um bloco de conteúdo".to_string())?;

    Ok(Value::Content(Content::Align {
        alignment,
        body: Box::new(body),
    }))
}
```

Aplicar o mesmo padrão a `native_place` — aceitar `Value::Align`
preferencialmente, cair para `Value::Str` via `from_string`.

Não remover `Align2D::from_string`. Não marcar como `deprecated`
neste passo — a sintaxe string continua a ser um caminho legítimo
até que um passo dedicado faça a migração dos call sites. O
comentário de `from_string` pode ser actualizado:

```rust
/// Parse de uma string composta por partes separadas por '-'.
///
/// Suportado para compatibilidade com a sintaxe legacy
/// `align("center", ...)`. A sintaxe preferida pós-Passo 84.5 é
/// `align(center, ...)` usando constantes de `Value::Align` com
/// composição via `+`.
pub fn from_string(s: &str) -> Self {
    // ... implementação existente ...
}
```

### 2.3 — `eval_binary_op` — `Plus` entre `Align`

Em `01_core/src/rules/eval.rs`, braço `BinOp::Plus` de
`eval_binary_op`:

```rust
// Adicionar ANTES do braço wildcard que retorna Err para tipos não suportados.
(BinOp::Plus, Value::Align(a), Value::Align(b)) => {
    // Combinar componentes: o segundo operando tem prioridade
    // quando ambos definem o mesmo eixo. Isto espelha o comportamento
    // do vanilla — `center + bottom` define H=Center V=Bottom,
    // `center + right` define H=Right (right sobrescreve center).
    let combined = Align2D {
        h: b.h.or(a.h),
        v: b.v.or(a.v),
    };
    Ok(Value::Align(combined))
}
```

**Nota sobre a semântica de prioridade**: a expressão `center + bottom`
deve dar `HAlign::Center + VAlign::Bottom` (combinação). A expressão
`center + right` deve dar `HAlign::Right` (sobrescrita). O `b.h.or(a.h)`
implementa este comportamento: se `b` define o eixo, ganha; caso
contrário, usa `a`.

Confirmar no diagnóstico 1.1 se o vanilla usa esta semântica. Se
o vanilla tiver regra diferente (ex: erro em conflito em vez de
sobrescrita), reportar e adaptar.

### 2.4 — Constantes top-level

Identificar o mecanismo de registo no diagnóstico 1.6. Registar
6 constantes:

```rust
// Em eval.rs ou stdlib.rs, na inicialização do scope global:
ctx.register_const("left",    Value::Align(Align2D { h: Some(HAlign::Left),   v: None }));
ctx.register_const("center",  Value::Align(Align2D { h: Some(HAlign::Center), v: None }));
ctx.register_const("right",   Value::Align(Align2D { h: Some(HAlign::Right),  v: None }));
ctx.register_const("top",     Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
ctx.register_const("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
ctx.register_const("bottom",  Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));
```

Se o método exacto de registo for diferente (ex: scope pré-populado
numa função `stdlib::default_scope()`), adaptar o padrão.

### 2.5 — Testes novos

Três testes que exercitam o caminho novo:

```rust
#[test]
fn align_aceita_constante_simbolica() {
    // align(center, rect(...)) — sem string.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #align(center, rect(width: 100pt, height: 20pt))\n\
    ").unwrap();

    // ... pipeline de eval + layout ...
    // Validar: rect centrado (x = 150pt, mesma conta do Passo 82).
}

#[test]
fn align_aceita_composicao_via_plus() {
    // align(center + bottom, rect(...)).
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #align(center + bottom, rect(width: 100pt, height: 20pt))\n\
    ").unwrap();

    // ... pipeline ...
    // Validar: rect em x = 150pt (center) e ancorado no fundo via VAlign::Bottom.
}

#[test]
fn align_composicao_sobrescreve_eixo() {
    // center + right → apenas right; o eixo horizontal é sobrescrito.
    let a = Align2D { h: Some(HAlign::Center), v: None };
    let b = Align2D { h: Some(HAlign::Right),  v: None };

    // Simular eval_binary_op via combinação directa.
    let combined = Align2D {
        h: b.h.or(a.h),
        v: b.v.or(a.v),
    };
    assert_eq!(combined.h, Some(HAlign::Right));
    assert_eq!(combined.v, None);
}
```

O terceiro teste é unitário puro — não precisa de pipeline. Os
dois primeiros precisam de `eval` + `layout` funcional. Adaptar
à estrutura dos testes existentes no ficheiro destino.

### 2.6 — Testes existentes devem continuar a passar

Todos os testes que usam sintaxe string continuam válidos:

- `align_center_reposiciona_no_eixo_x` (Passo 82) — usa `"center"` string.
- `align_right_ancora_a_margem_direita` (Passo 82) — usa `"right"`.
- `place_nao_altera_cursor_y` (Passo 82) — usa `"bottom-right"`.
- `grid_valign_bottom_ancora_ao_limite_inferior_da_celula` (Passo 83)
  — usa `"bottom"`.

Nenhum destes testes é alterado neste passo. Se algum quebrar, é
regressão — investigar antes de avançar.

---

## Tarefa 3 — Encerrar DEBT-36

Mover DEBT-36 da Secção 1 para Secção 2:

```markdown
## DEBT-36 — Operadores simbólicos de alinhamento — **ENCERRADO (Passo 84.5)** ✓

**Registado no Passo 82. Resolvido no Passo 84.5.**

`Value::Align(Align2D)` adicionado ao enum `Value`. Constantes
top-level `left`, `center`, `right`, `top`, `horizon`, `bottom`
registadas como valores de `Align2D`. `eval_binary_op` trata
`BinOp::Plus` entre dois `Value::Align` combinando componentes
com prioridade ao operando direito em caso de conflito de eixo.

Sintaxe preferida: `align(center + bottom, ...)`, `place(top + right, ...)`.

Sintaxe legacy `align("center", ...)` preservada — `Align2D::from_string`
continua a ser usada como fallback em `native_align` e `native_place`.
Remoção da sintaxe legacy e dos call sites de teste associados fica
para passo dedicado posterior.
```

---

## Tarefa 4 — Verificação

```bash
# Testes
cargo test

# Linter
crystalline-lint .

# Grep de confirmação: Value::Align existe
grep -n "Value::Align" 01_core/src/entities/value.rs

# Grep de confirmação: from_string ainda existe (versão mínima preserva)
grep -n "from_string" 01_core/src/entities/layout_types.rs

# Grep de confirmação: call sites de from_string inalterados em número
grep -rn "Align2D::from_string\|from_string" 01_core/src/ | wc -l
```

Esperado:
- `Value::Align` aparece em `value.rs` e em `stdlib.rs`.
- `from_string` continua a existir (não removido).
- Número de call sites de `from_string` pode ter aumentado em 0
  (se já era só chamado via `native_align`/`native_place`) ou
  permanecido igual.

---

## Critérios de conclusão

- [ ] Tarefa 1 (diagnóstico) executada e reportada ao utilizador
  antes de qualquer edição.
- [ ] Cenário classificado (esperado ou inesperado).
- [ ] Se inesperado, decisão do utilizador registada.
- [ ] `Value::Align(Align2D)` adicionado ao enum.
- [ ] Todos os `match` exaustivos sobre `Value` actualizados.
- [ ] `Value::cast_align()` implementado.
- [ ] `native_align` e `native_place` aceitam `Value::Align` e
  `Value::Str` (compatibilidade preservada).
- [ ] `Align2D::from_string` mantido; doc actualizado.
- [ ] `eval_binary_op` trata `BinOp::Plus` entre `Value::Align`
  com semântica `b.h.or(a.h)`.
- [ ] 6 constantes top-level (`left`, `center`, `right`, `top`,
  `horizon`, `bottom`) registadas como `Value::Align`.
- [ ] 3 testes novos (2 de integração, 1 unitário).
- [ ] Testes existentes com sintaxe string continuam a passar.
- [ ] DEBT-36 movido para Secção 2 com nota de resolução.
- [ ] `cargo test` — 906 ou mais testes (2 novos L1 mínimo).
- [ ] `crystalline-lint .` zero violations.
- [ ] Nenhum uso de `unsafe`.

---

## Ao terminar, reportar

Bloco 1 — Diagnóstico:
- Estrutura vanilla de `Alignment` (copiar literal da saída).
- Relação `Align2D` (projecto) vs `Alignment` (vanilla): isomorfos
  ou divergentes?
- Número de call sites de `Align2D::from_string`.
- Número de testes que usam sintaxe string.
- Semântica de `Plus` no vanilla (`or` com prioridade direita,
  erro em conflito, ou outra).
- Mecanismo de registo de constantes top-level.

Bloco 2 — Implementação:
- Ficheiros alterados.
- Variante `Value::Align` adicionada; quantos `match` precisaram
  de actualização.
- Testes novos adicionados.

Bloco 3 — Verificação:
- Número total de testes (esperado 906+).
- Resultado do linter.
- DEBT counts (Secção 1 = 10, Secção 2 = 31 após o passo).

**Go/No-Go para o Passo 84.6** (DEBT-37 — Place relativo ao
contentor pai):

- **GO — `Value::Align` com composição funciona**: os 3 testes
  novos passam, os testes antigos com sintaxe string continuam
  a passar.
- **NO-GO — regressão em testes antigos**: se algum teste com
  `align("center", ...)` ou similar falha, verificar que a
  compatibilidade foi preservada — `native_align` deve aceitar
  ambos os tipos de argumento.
- **NO-GO — semântica de `Plus` errada**: se
  `center + bottom` produz `HAlign::Center` apenas (sem V), ou
  se `center + right` produz conflito em vez de `Right`, rever
  o braço de `BinOp::Plus` em `eval_binary_op`.
- **NO-GO — cascata de alterações maior que prevista**: se
  adicionar `Value::Align` forçou mais de ~15 actualizações de
  `match` em ficheiros não esperados, parar e reportar — pode
  indicar que `Value` é usado em sítios não diagnosticados.
