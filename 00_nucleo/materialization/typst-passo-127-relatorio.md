# Passo 127 — Relatório (DEBT-1 subset: `text.tracking`)

**Data**: 2026-04-24
**Precondição**: Passo 126 encerrado; 1044 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos; DEBT-1 subset
`weight` pago.
**Natureza**: passo XS em L1; **segunda aplicação do pattern
Passo 126**. Valida generalização para tipos semânticos.
**ADR tocada**: **ADR-0038** anotada (segunda nota).

---

## Sumário

`StyleDelta.tracking: Option<Length>` adicionado. `eval_set_text`
captura `tracking` de `#set text(tracking: 0.5pt)` e
`#set text(tracking: 0.1em)` sem warning. `Length { abs + em }`
inteiro preservado — não colapsa para pt como `size` legado.

**Canary DEBT-50 preservado** em segunda iteração do pattern
(teste `eval_set_text_font_canary_passo_127`).

**815 L1 (+2) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1046 total** (+2 novos testes). Zero violations. **51 ADRs**
(ADR-0038 com segunda nota). **11 DEBTs** — DEBT-1 subset pago
+1 propriedade.

---

## 127.A — Inventário

Completo em `00_nucleo/diagnosticos/inventario-tracking-passo-127.md`.

**Findings-chave**:
- `Length { abs: Abs, em: f64 }` em `entities/layout_types.rs:527`
  — `Copy + Clone + PartialEq`, fiel ao vanilla.
- `Value::Length(Length)` — variante directa em `value.rs:53`.
- Vanilla `pub tracking: Length` em `typst-library`.
- Teste L3 DEBT-49 actual usa `font/lang/stroke` — **zero
  rotação necessária**.

**Decisão T = `Length`**: preserva `abs + em` sem cast para
pt. Diverge deliberadamente de `size` (que colapsa para
`Option<f64>` por legado). Argumento: `tracking` é nova;
quando consumer chegar, terá informação completa.

**Gate 127.A**: passa. 2 ficheiros L1 tocados; zero ripple.

---

## 127.B — ADR-0038 anotada

Segunda nota adicionada ao final do ADR-0038:
**"Passo 127 — `tracking` como primeira propriedade com tipo
semântico"**.

Extensão do pattern do 126: propriedades podem usar tipo
**semântico** de L1 (não só primitivos `u16`). Preserva-se
o valor sem colapso. Contraste explícito com `size` (legacy)
— futuras propriedades novas devem preferir `Option<Length>`.

---

## 127.C — Implementação

### Diff `StyleDelta`

```rust
// + campo:
pub tracking: Option<crate::entities::layout_types::Length>,

// empty() ganha `tracking: None`
```

### Diff `eval_set_text`

```rust
"tracking" => {
    if let Value::Length(l) = val {
        delta.tracking = Some(l);
    }
}
```

Simples, zero cast intermédio. Tipo errado → silent skip.

### Testes novos (2)

- `eval_set_text_tracking_passo_127`: captura OK + zero
  warning "'tracking'".
- `eval_set_text_font_canary_passo_127`: font continua a
  emitir warning (canary).

Ambos usam harness inline com `sink.into_diagnostics()` — mesma
técnica do 126.

### L3 DEBT-49 test: **intacto**

Input actual `font, lang, stroke` — sem rotação. Pattern
rotativo do 126 (substituir por próxima propriedade
desconhecida) aplicou-se então; hoje não.

---

## 127.D — Verificação

### Cargo tests

```
test result: ok. 815 passed ...       (L1 +2 vs 813)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

```bash
$ typst tp.typ -o tp.pdf           # #set text(tracking: 0.5pt)
# stderr: (vazio)
exit=0

$ typst te.typ -o te.pdf           # #set text(tracking: 0.1em)
# stderr: (vazio)
exit=0

$ typst f.typ -o f.pdf             # #set text(font: "X")
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

Tanto `pt` como `em` aceites sem warning — comportamento
alinhado com preservação de `Length` inteiro.

---

## 127.E — Encerramento

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/style_chain.rs` | +campo `tracking: Option<Length>`; init |
| `01_core/src/rules/eval/rules.rs` | +arm `"tracking"` em match |
| `01_core/src/rules/eval/tests.rs` | +2 testes L1 |
| `00_nucleo/adr/typst-adr-0038-...md` | segunda nota |
| `00_nucleo/prompts/entities/style_chain.md` | actualiza StyleDelta no prompt |

### Números finais

| Métrica | Antes (126) | Depois |
|---------|------:|-------:|
| L1 tests | 813 | **815** (+2) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1044** | **1046** (+2) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 (0038 com 2 notas) |
| DEBTs abertos | 11 | 11 (DEBT-1 subset cresce) |

---

## Limitações aceites

1. **`tracking` inerte**: capturado em `StyleDelta` mas não
   consumido por layout. `StyleChain` **não tem** resolver
   `tracking()` — adicionar sem consumer seria peso morto.
2. **`size` permanece legado f64**: decisão deliberada —
   migrar `size` para `Option<Length>` é refactor dedicado
   (afecta `TextStyle.size: Pt(f64)`, `resolve_f64`, muitos
   call-sites). Este passo não toca.
3. **`em` sem font-size**: `Length { em: 0.1 }` precisa de
   font-size para resolve. Captura preserva valor abstracto;
   consumer resolve quando tiver contexto.
4. **Valor negativo aceite sem validação**: `tracking: -5pt`
   (aperta glyphs) capturado raw. CSS aceita; OpenType aceita.
   Consumer valida se precisar.

---

## Lições

1. **Tipo semântico de L1 é extensão natural**: passo 126
   validou `Option<u16>`; 127 estende para `Option<Length>`
   sem nova ADR (só nota). Próximas propriedades com tipos
   semânticos (`Stroke`, `Paint`, `HorizontalAlignment`)
   seguem o mesmo template.

2. **Preservar Length inteiro é mais honesto que size legado**:
   `size` capturado como `f64` perde `em`. Para nova
   propriedade, capturar `Length` puro documenta a ambição
   de paridade vanilla. Quando consumer chegar, trabalho é
   só "resolve com font-size" — zero recuperação de precisão
   perdida.

3. **Pattern rotativo do DEBT-49 nem sempre dispara**:
   neste passo, input actual já não usa `tracking` — rotação
   desnecessária. Próxima propriedade pode disparar se
   coincidir com o canary actual (`stroke`). Lição:
   registar os nomes dos canary tests em candidatos futuros
   para rotação prever.

4. **Segunda aplicação valida o pattern**: XS confirmado
   duas vezes (126: +2 ficheiros, 127: +2 ficheiros). Se o
   pattern falhasse na segunda, seria sinal de over-fitting.
   Passou — confiança para próximas propriedades.

5. **Harness de test inline é reutilizável**: bloco de
   `use comemo::Track; let ... let mut sink = Sink::new(); let route = Route::root(); eval(...); sink.into_diagnostics()`
   funciona identicamente 126 e 127. Candidato a virar helper
   `eval_with_warnings(source) -> (Result, Vec<SourceDiagnostic>)`
   em test harness — registado em lições mas não executado
   (fora de escopo).

---

## Estado pós-Passo 127

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold (Passo 30)
- ✓ italic (Passo 30)
- ✓ size (Passo 30; f64 legado)
- ✓ fill (Passo 102)
- ✓ weight (Passo 126; u16)
- ✓ **tracking (Passo 127; Length inteiro)** — este
- ✗ font, lang, leading, stroke, alignment, ...

### Candidatos próximos DEBT-1

1. **`leading`** (`Length`) — pattern idêntico a tracking; XS.
   Mas pode exigir coordenação com `par` target (se `#set par`
   for ambiente natural vs `#set text`).
2. **Forma simbólica de `weight`** (`"bold"` → 700):
   `Value::Str` match + tabela. XS-S.
3. **`stroke`** — exige tipo `Stroke` em L1 (composto com
   `width + paint`). S-M.
4. **`alignment` / `align`** — tipo `HorizontalAlignment` em L1.
   Exige enum. S.
5. **Consumer de `weight` em layout** — selecção de variante
   de fonte OU faux-bold via stroke. M-L.
6. **Consumer de `tracking` em layout** — offset inter-glyph
   em `layout_text`. M.

### Pattern L1 documentado (duas iterações)

```rust
// 1. Campo em StyleDelta:
pub <prop>: Option<<T>>,   // T primitivo OU tipo semântico L1

// 2. Init em empty():
pub const fn empty() -> Self {
    Self { ..., <prop>: None }
}

// 3. Arm no eval_set_text:
"<name>" => {
    if let Value::<Variant>(v) = val {
        delta.<prop> = Some(v);   // ou cast se T diferir de Variant
    }
}

// 4. Testes:
// - <prop>_capturado_sem_warning
// - font_canary_preservado (idempotente; 1 por passo)
```
