# Passo 102 — Relatório de encerramento (Activação de `#set`)

**Data**: 2026-04-23
**Precondição**: Passo 101 encerrado; `Content::Strong`/`Emph`
removidos; 783 L1 + 174 L3 + 6 ignorados; zero violations.
**ADR criada**: ADR-0040 "Activação de `#set` em eval" — **PROMOVIDA A
EM VIGOR** em 102.E.

---

## Sumário

**Descoberta do 102.A**: `#set text(bold/italic/size)` já estava
activo desde o Passo 30 via arquitectura bake-in (StyleDelta
empilhado em `*styles`; TextStyle capturado em `Content::Text`
produzido). A activação foi feita há muito; faltava validação
end-to-end e extensão para a propriedade `fill` (adicionada no
Passo 100 mas sem consumidor em `#set`).

Este passo formalizou a decisão (ADR-0040), validou end-to-end com
6 testes de integração novos, e activou `#set text(fill: color)`
via uma única linha em `eval_set_rule`.

Zero regressão funcional: **783 → 790 L1** (+7), 174 L3 + 6
ignorados inalterados. `crystalline-lint .` → zero violations.

---

## 102.A — Inventário

Inventário em `00_nucleo/diagnosticos/inventario-set-rule-passo-102.md`.

### Estado actual de `#set`

| Target | Arquitectura | Implementado em |
|--------|-------------|-----------------|
| `text` | **bake-in** via `StyleDelta` em `*styles` | Passo 30 |
| `heading` | `Content::SetHeadingNumbering { active }` | Passo 57 |
| `page` | `Content::SetPage { width, height, margin }` | Passo 81 |
| `figure` | Muta `*figure_numbering` + emite `Content::SetFigureNumbering` | Passos 75 + 98 |

`#set` **já estava activo**. A spec do 102 partiu de premissa
incorrecta (que `#set` não estava activo). Decisão: **formalizar** em
ADR em vez de refactorizar.

### Catálogo de propriedades em `#set text(...)`

| Propriedade | Antes do Passo 102 | Depois do Passo 102 |
|-------------|-------------------:|---------------------:|
| `bold` | ✓ | ✓ |
| `italic` | ✓ | ✓ |
| `size` | ✓ | ✓ |
| `fill` | ✗ (enum `Style::Fill` existia mas `StyleDelta` não capturava) | ✓ **activado** |

Propriedades adiadas (bloqueadas por tipos não materializados):
`font`, `lang`, `region`, `weight` como string, `par.leading`, etc.

---

## 102.B — ADR-0040

Criada em `00_nucleo/adr/typst-adr-0040-activacao-set-rule.md` com
status `PROPOSTO`. **Promovida a EM VIGOR em 102.E**.

Conteúdo:

- Documenta a arquitectura bake-in actual.
- Lista catálogo de propriedades suportadas (bold, italic, size,
  fill).
- Regista propriedades adiadas com razões técnicas.
- **Decisão**: manter bake-in em vez de refactorizar para
  `Content::Styled` wrapping. Razão: zero regressão é o critério;
  substituir a arquitectura com 6+ testes dependentes tem risco
  sem ganho funcional imediato.
- Coexistência bake-in + wrapping: ambos produzem o mesmo
  `FrameItem::Text` para texto inline. Unificação fica para
  passo futuro ligado a `Introspection`.

---

## 102.C — Implementação

### Modificação única em `eval_set_rule`

`01_core/src/rules/eval/rules.rs`:

```rust
"fill" => {
    // Passo 102 (ADR-0040): activar `#set text(fill: color)`.
    // Captura `Value::Color` em `StyleDelta.fill`; propaga para
    // `TextStyle.fill` via `TextStyle::from(&StyleChain)`.
    if let Value::Color(c) = val {
        delta.fill = Some(c);
    }
}
_ => {
    // DEBT: propriedades de #set text não suportadas (font, lang,
    // weight como string, etc.) são silenciosamente ignoradas.
    // Substituir por warning quando `Sink` materializar.
}
```

Uma linha semântica, com comentário a documentar DEBT-49 (aberto
neste passo).

---

## 102.D — Testes de integração

### Novos em `rules/layout/tests.rs::tests_set_rule_integration` (6)

1. `set_text_size_propaga_ao_frame` — `#set text(size: 18pt)\nHello` →
   `FrameItem::Text.style.size == Pt(18.0)`.
2. `set_text_bold_propaga_ao_frame` — `#set text(bold: true)\nHello` →
   `style.bold == true`.
3. `set_text_italic_propaga_ao_frame` — análogo para italic.
4. `set_text_bold_afecta_conteudo_seguinte_nao_anterior` —
   `antes\n#set text(bold: true)\ndepois` → "antes" sem bold;
   "depois" com bold.
5. `set_combinado_com_emph_sintactico` — `#set text(bold: true)\n_italic_
   normal` → todos os items com bold; pelo menos um com italic
   (do `_italic_`).
6. `bold_syntax_sem_set_continua_a_funcionar` — regressão do Passo
   101: `*importante* normal` continua a funcionar após todas as
   mudanças.

### Novo em `rules/eval/tests.rs` (1)

- `eval_set_text_fill_passo_102` — `#set text(fill: rgb(255, 0, 0))\nred text`
  eval sem erro.

---

## 102.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 790 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found
```

### DEBTs

- **DEBT-1** actualizado com "Actualização Passo 102" — `#set`
  validado; `fill` activado. Das pendências: `#set` concluído;
  `#show` pendente; propriedades adicionais bloqueadas.
- **DEBT-49 aberto** — "Propriedades de `#set` não suportadas
  silenciadas". Depende de materialização de `Sink` para emitir
  warnings em vez de silenciar.

### ADR

- **ADR-0040** promovida de `PROPOSTO` para **EM VIGOR**.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 783 | **790** (+7) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Propriedades suportadas em `#set text` | 3 | **4** (+fill) |
| ADRs activas | 39 | **40** (+0040) |

---

## Lições

1. **"Activar" pode significar "validar que já está activo"**: o
   passo partiu de premissa que `#set` não estava activo; o
   inventário revelou que estava desde o Passo 30. A acção certa
   foi formalizar em ADR + adicionar testes end-to-end que validam
   o output, não apenas o parsing.

2. **Bake-in vs. wrapping — decisão pragmática**: a arquitectura
   bake-in funciona e tem testes dependentes. Wrapping via
   `Content::Styled` é mais alinhado com vanilla mas adiciona
   duplicação. Adiado para quando `Introspection` materializar e
   forçar decisão natural.

3. **Fill activado com 1 linha**: o enum `Style::Fill` existia desde
   o Passo 99; o campo `StyleDelta.fill` existia desde o Passo 99;
   o campo `TextStyle.fill` existia desde o Passo 100. A activação
   final do `#set text(fill: ...)` foi uma linha de `match` em
   `eval_set_rule`. Validação de que as fundações dos Passos 99–100
   foram bem construídas.

4. **DEBT-49 para UX futura**: ignorar silenciosamente propriedades
   não suportadas é pragmático (sem `Sink` não há como avisar), mas
   compromete a UX. O DEBT fica registado com critério objectivo —
   será resolvido quando `Sink` materializar.

---

## Estado pós-Passo 102

### Pipeline `#set` funcional end-to-end

```
#set text(size: 18pt)     →  StyleDelta.size = Some(18.0)
#set text(bold: true)     →  StyleDelta.bold = Some(true)
#set text(italic: true)   →  StyleDelta.italic = Some(true)
#set text(fill: rgb(255,0,0))  →  StyleDelta.fill = Some(Color::rgb(255,0,0))
                          ↓
         *styles = styles.push(delta)
                          ↓
    Content::Text(s, TextStyle::from(&*styles))  (bake-in)
                          ↓
         Layouter::layout_content
                          ↓
   FrameItem::Text { style: TextStyle { bold, italic, size, fill, ... } }
                          ↓
         export.rs (L3) → PDF com fontes/tamanhos/cores correctos
```

### Trabalho futuro identificado

1. **Activar `#show`**: próximo candidato. A dívida latente do show
   selector (relatório Passo 101) será exposta e obrigará a
   refactorização do `matches!(kind, NodeKind::Strong) &&
   is_bold_styled` pattern.
2. **DEBT-49** (propriedades silenciadas): depende de `Sink`.
3. **Refactorização bake-in → wrapping** (DEBT futuro): pode
   justificar-se quando `Introspection` materializar e
   `Content::Heading` for colapsado em `Content::Styled`.
4. **Propriedades adicionais** (`text.font`, `text.lang`): bloqueadas
   por tipos não materializados.
