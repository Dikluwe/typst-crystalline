# Passo 126 — Relatório (DEBT-1 subset: `text.weight` numérico)

**Data**: 2026-04-24
**Precondição**: Passo 125 encerrado; 1042 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo XS em L1; primeiro desde Passo 111.
Fracciona DEBT-1 pagando 1 propriedade isolada.
**ADR tocada**: **ADR-0038** anotada (2026-04-24).

---

## Sumário

`StyleDelta.weight: Option<u16>` adicionado em L1. `eval_set_text`
captura `weight` de `#set text(weight: 700)` sem warning.
`font`/`lang`/outras propriedades continuam a emitir warning —
canary DEBT-50 preservado.

**Efeito visível**: **nenhum** (documentado). `weight` é
capturado mas inerte; pipeline de layout não consome.

**813 L1 (+2) + 24 L2 + 186 L3 + 21 L4 + 6 ignorados =
**1044 total** (+2 novos testes). Zero violations. **51 ADRs**
(0038 anotada). **11 DEBTs** abertos (DEBT-1 permanece partial
— subset pago).

---

## 126.A — Inventário

Completo em
`00_nucleo/diagnosticos/inventario-weight-passo-126.md`.

### Findings

1. **`StyleDelta`** em `entities/style_chain.rs:25-35` — 5
   campos (bold, italic, size, fill, heading_level).
2. **`eval_set_text`** em `rules/eval/rules.rs:271-312` —
   `match key.as_str()` com arms para 4 propriedades activas;
   default arm emite warning via `unsupported_property_warn`.
3. **Canary DEBT-50** usa `font`, não `weight` — sem colisão.
4. **`FontWeight(u16)`** já existe em `entities/font_book.rs:45`
   mas desligado de `StyleDelta` (usa em catálogo).
5. Vanilla usa `FontWeight(u16)` com parsing de string. Este
   passo só cobre numeric — forma simbólica deferida.

### Gate 126.A

**Passa**. 2 ficheiros tocados:
- `01_core/src/entities/style_chain.rs` (+1 campo).
- `01_core/src/rules/eval/rules.rs` (+1 match arm).

XS confirmado. Zero ripple em `Style` enum, `push_styles`,
resolvers, pipeline layout, export.

---

## 126.B — ADR-0038 anotada

Nota "Passo 126 — `weight` como primeira propriedade numérica"
adicionada ao final do ADR-0038. Estabelece pattern:
**propriedades de `text` podem ser adicionadas uma a uma como
`Option<T>` em `StyleDelta`**, sem exigir materialização de tipos
Font/Lang/Par nem entrada em enum `Style` (bake-in usa
`push(delta)`, não `push_styles(&Styles)`).

Ausência de efeito layout documentada. Range (0-1000 CSS) fica
para consumer validar quando existir.

---

## 126.C — Implementação

### Diff `StyleDelta`

```rust
// + campo:
pub weight: Option<u16>,

// StyleDelta::empty() ganha `weight: None`
```

Nada em `StyleChain` — sem `weight()` resolver (inerte).
Nada em `push_styles` — bake-in não passa por aí.

### Diff `eval_set_text`

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    }
}
```

Tipo errado ou out-of-range → silent (coerente com outros
arms). Sem emissão de warning.

### Diff teste L3 `debt49_set_text_multiplas_propriedades_desconhecidas`

Input alterado de `weight: 700` para `stroke: 1pt` — test
continuava a contar 3 warnings mas `weight` já não é
desconhecido. `stroke` é próxima propriedade not-yet-implemented
que preserva a asserção semântica (3 warnings distintos).

### Testes novos (2)

- `eval_set_text_weight_passo_126`: `#set text(weight: 700)`
  → zero warning com "'weight'".
- `eval_set_text_font_canary_passo_126`: `#set text(font: "X")`
  → warning com "'font'" preservado.

Ambos usam harness inline com `sink.into_diagnostics()` após
`eval(...)` — mesma técnica que os testes DEBT-49 em L3 mas em L1.

---

## 126.D — Verificação

### Cargo tests

```
test result: ok. 813 passed ...       (L1 +2)
test result: ok. 186 passed, 6 ign    (L3 — test DEBT-49 adaptado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

**Total**: 1042 → **1044** (+2).

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

```bash
$ typst /tmp/w.typ -o /tmp/w.pdf 2>&1
# stdin input: "#set text(weight: 700)\nOlá"
# stderr: (vazio)
exit=0

$ typst /tmp/f.typ -o /tmp/f.pdf 2>&1
# stdin input: "#set text(font: \"X\")\nOlá"
# stderr:
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

Comportamento alinhado com especificação.

---

## 126.E — Encerramento

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/style_chain.rs` | +campo `weight`; init |
| `01_core/src/rules/eval/rules.rs` | +arm `"weight"` em match |
| `01_core/src/rules/eval/tests.rs` | +2 testes L1 |
| `03_infra/src/integration_tests.rs` | adapt test DEBT-49 (stroke substitui weight) |
| `00_nucleo/adr/typst-adr-0038-...md` | nota Passo 126 |
| `00_nucleo/prompts/entities/style_chain.md` | actualiza StyleDelta no prompt |

### Números finais

| Métrica | Antes (125) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | **813** (+2) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 (1 adaptado) |
| L4 tests | 21 | 21 |
| **Total** | **1042** | **1044** (+2) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 (0038 anotada) |
| DEBTs abertos | 11 | 11 (DEBT-1 subset pago) |

---

## Limitações aceites

1. **`weight` inerte**: capturado em `StyleDelta` mas não
   consumido por layout nem exposto via `TextStyle`. Não há
   resolver `StyleChain::weight()`. Decisão: resolver sem
   consumer é peso morto — adicionar quando fizer diferença
   visível.
2. **Forma simbólica não suportada**: `#set text(weight: "bold")`
   não mapeia para 700. `Value::Str` → cast falha → silent skip.
   Sem warning (coerente com outros tipos errados). Passo
   dedicado pode adicionar mapeamento.
3. **Range 0-1000 não validado**: `u16::try_from(i64)` aceita
   0-65535. Out-of-range CSS/OpenType não é reportado. Consumer
   deve clamp (tal como `FontWeight::from_number` faz para o
   catálogo de fontes).
4. **Zero ripple em DEBT-50**: verificado empiricamente —
   canary test passes. Input misto `font + weight` agora emite
   só 1 warning (antes 2); teste L3 DEBT-49 adaptado.

---

## Lições

1. **XS só funciona com gate empírico**: o inventário 126.A
   confirmou que 2 ficheiros bastam. Sem isso, tentação de
   "enquanto cá estou activar resolver + consumer" multiplica
   tamanho por 4. Disciplina pagou-se.

2. **Canary DEBT-50 tem dupla função**: protege migração
   bake-in → wrapping E protege este tipo de passo. Se o
   teste canary falhasse após adicionar `weight`, seria sinal
   de ripple inesperado. Passou sem esforço — confirmação.

3. **Teste que expõe sink é caro mas paga-se**: os harnesses
   existentes (`eval_for_test`) swallow o sink. Inline com
   `eval(...)` + `sink.into_diagnostics()` adiciona ~15 linhas
   por teste mas materializa invariante que antes era só
   behavioural.

4. **Inputs de teste DEBT-49 eram frágeis**: o teste usava
   `weight` como um dos "3 desconhecidos". Ao activar `weight`,
   o teste quebra — mas a quebra é benigna (asserção numérica).
   Substituir por `stroke` (próximo candidato). Lição: se um
   teste usa propriedade X como canary de "desconhecidas",
   comentar o porquê — torna futuras migrações rápidas.

5. **`u16::try_from(i64)` é pattern honesto**: `as u16` com
   wrap seria inseguro; `clamp` muda semântica. `try_from` +
   silent skip em erro é coerente com os outros arms
   (`Value::Bool`, `Value::Color`) que silenciosamente ignoram
   tipo errado. Não engana utilizador — dá-lhe zero feedback,
   igual aos outros casos.

6. **ADR-0038 absorve o padrão sem ADR nova**: a anotação
   documenta que "propriedade X como `Option<T>` em `StyleDelta`
   + match arm em eval" é o template para futuros passos DEBT-1
   subset. Próxima propriedade (`stroke`, `tracking`, ou forma
   simbólica de `weight`) tem trilhos desenhados — acelera
   roadmap DEBT-1.

---

## Estado pós-Passo 126

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold (Passo 30)
- ✓ italic (Passo 30)
- ✓ size (Passo 30)
- ✓ fill (Passo 102)
- ✓ **weight (Passo 126 — este)**
- ✗ font, lang, leading, stroke, tracking, ... (pendente)

### Candidatos de próximo passo DEBT-1

1. **`stroke` (width + cor)** — exige tipo `Stroke` em L1.
2. **`tracking` (letter-spacing)** — `f64` simples.
3. **Forma simbólica de `weight`** (`"bold"` → 700, `"regular"` → 400):
   mapeamento string → u16. XS.
4. **Consumer de `weight` em layout**: `StyleChain::weight()`
   resolver + exposição via `TextStyle` + (a) selecção de
   variante de fonte ou (b) faux-bold. Grande, M/L.
5. **`font` + `lang`**: exige tipos `Font` / `Language` em L1.
   Bloqueado.

### Pattern disponível para uso

```rust
// 1. Adicionar campo:
pub struct StyleDelta {
    // ...
    pub <prop>: Option<<T>>,
}

// 2. Init em empty():
pub const fn empty() -> Self {
    Self { ..., <prop>: None }
}

// 3. Arm no eval:
"<name>" => {
    if let Value::<Variant>(v) = val {
        delta.<prop> = Some(v);
    }
}

// 4. Testes: captura sem warning + canary outra propriedade preserva warning.
```
