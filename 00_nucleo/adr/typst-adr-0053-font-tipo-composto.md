# ⚖️ ADR-0053: Font como tipo composto em L1

**Status**: `IMPLEMENTADO`
**Data**: 2026-04-24
**Materializado em**: Passo 132B (2026-04-24)
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`](../diagnosticos/diagnostico-font-list-passo-132a.md)

---

## Contexto

`text.font` é a propriedade residual mais complexa da lista
DEBT-1. Vanilla aceita 3 formas: string simples, array, e dict
com `covers` (keyword `"latin-in-cjk"` ou `regex::Regex`).
Captura raw como `EcoString` violaria ADR-0033 no mesmo espírito
do Passo 130 (lang, resolvido via ADR-0052 em 131B). Este ADR
propõe materialização análoga — `FontList` como tipo composto
em L1 com paridade **parcial**: string + array, com dict
rejeitado explicitamente (deferido).

ADR-0034 (diagnóstico obrigatório) cumprido em
`diagnostico-font-list-passo-132a.md`.

## Decisão

1. **Materializar `FontList` em L1** como newtype sobre
   `Vec<FontFamily>`:
   ```rust
   pub struct FontFamily { name: EcoString }      // lowercased
   pub struct FontList(Vec<FontFamily>);
   ```

2. **Sem `covers` neste passo**. `FontFamily` não tem campo
   `covers`. Dict form do `font` é **rejeitada explicitamente**
   com erro hard e mensagem clara
   (`"dict form of font not yet supported — use string or array
    of strings"`).

3. **Paridade parcial (2 de 3 formas vanilla)**:
   - ✅ String simples: `#set text(font: "Arial")`.
   - ✅ Array de strings: `#set text(font: ("A", "B"))`.
   - ❌ Dict com ou sem covers: **erro hard** explícito.

4. **Validação / parser**: `FontList::from_value(&Value) ->
   Result<Self, &'static str>` substitui `cast!` vanilla.

5. **Erro hard em valor inválido** (paridade 131B):
   - Tipo errado → `"expected string or array of strings"`.
   - Array vazio → `"font fallback list must not be empty"`
     (réplica literal vanilla).
   - Item de array não-string → `"font family must be a
     string"`.
   - Dict → `"dict form of font not yet supported — use string
     or array of strings"`.

6. **`StyleDelta.font: Option<FontList>`** — novo campo.

7. **Localização**: `01_core/src/entities/font_list.rs` (novo,
   ADR-0037 coesão por domínio). Separa de `font_book.rs`
   (catálogo).

8. **Zero crates novas**. `regex` **não** é autorizada em L1.
   Decisão deferida para quando consumer (shaping) chegar.

9. **Canary DEBT-50 migra** de `font` para **`hyphenate`** em
   10 testes (5 L1 + 3 L3 + 2 L4).

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Capturar só string como `EcoString` raw | XS | Viola ADR-0033; perde array form (uso comum) |
| Suportar string + array, com `covers` keyword-only | Paridade próxima | Ambíguo ("meio covers"); requer `Covers` enum |
| **Suportar string + array, dict deferido** ✓ | S-M; zero crates novas; erro claro | Rejeita dict (divergência "strict reject") |
| Suportar 3 formas com `regex::Regex` + autorização | Paridade total | Inclui autorização de crate `regex` (4-6 deps transitivas) sem consumer |
| Suportar 3 formas com `Coverage` próprio (sem regex) | Sem `regex` | Regex de vanilla não replicável; divergência complexa |

**Escolha**: string + array com dict deferido. Combina
paridade real (90%+ uso real) com zero adição de deps.

## Consequências

### Positivas

- **ADR-0033 parcialmente satisfeito**: forms comuns têm
  paridade; form avançada (`covers`) é **rejeitada
  explicitamente** em vez de silent-captured (menos grave que
  130 pre-131B).
- **Base para consumer futuro**: shaping engine pode iterar
  `FontList` e aplicar fallback chain.
- **Precedente 131B reaplicado em contexto composto**:
  validação Err hard + tipo dedicado funciona para tipos
  agregadores.
- **DEBT-1 subset `font` fica com paridade explícita**;
  roadmap DEBT-1 progride para fecho.
- **Zero crates novas**: sem pressão sobre `regex` authorization.

### Negativas

- **Dict form deferido**: utilizadores que escrevem
  `#set text(font: (name: "A", covers: ...))` recebem erro.
  Mitigação: mensagem de erro clara recomenda string ou array.
- **Custo migração**: 132B = S-M (maior que 131B). 1 ficheiro
  novo + 2 modificados L1 + 10 testes adaptados em 3 camadas.
- **Primeira rejeição de form suportada vanilla**: ADR-0033
  semi-violado mas com trade-off explícito (rejeitar é menos
  grave que aceitar-e-mentir).

### Neutras

- **`StyleDelta` ganha primeiro campo com tipo composto
  agregador** (FontList contém FontFamily). Precedente para
  `Stroke { width, paint }`, `Spacing`, etc.
- **Canary DEBT-50 rotado**: migração atomica com captura de
  `font`. Zero janela sem canary.
- **ADR-0038 não ganha nova nota**: nota do 131B já explica
  que futuras paridades seguem pattern "ADR dedicada".

## Plano de materialização (para 132B)

1. Criar `01_core/src/entities/font_list.rs`:
   - `struct FontFamily { name: EcoString }` + construtor +
     `as_str`.
   - `struct FontList(Vec<FontFamily>)` + `new` (valida
     non-empty) + `families` + `IntoIterator`.
   - `fn from_value(val: &Value) -> Result<Self, &'static str>`.
   - 10-12 unit tests.
2. `entities/mod.rs`: `pub mod font_list;`.
3. `StyleDelta.font: Option<FontList>` em `style_chain.rs`.
4. Arm `"font"` em `rules/eval/rules.rs` com Err hard.
5. **Canary migration** em 10 testes (5 L1 + 3 L3 + 2 L4):
   `font → hyphenate`.
6. **L3 DEBT-49 test `debt49_set_text_font_emite_warning`
   invertido** para `..._valido_captura_passo_132b` (paridade
   mudou).
7. 4-5 integration tests novos em `rules/eval/tests.rs`.
8. Transição ADR-0053 → `IMPLEMENTADO`.

## Referências

- **ADR-0033** (paridade) — parcialmente satisfeito.
- **ADR-0034** (diagnóstico obrigatório) — cumprido 132A.
- **ADR-0036/0037** (atomização, coesão por domínio).
- **ADR-0038** (StyleChain) — nota 131B cobre pattern 132.
- **ADR-0052** (`Lang` precedente) — mesmo padrão
  diagnóstico → ADR → materialização.
- **Passo 131A/131B** — precedente directo.
- **Passo 132A** (este diagnóstico).
- **Vanilla**:
  `lab/typst-original/crates/typst-library/src/text/mod.rs:835-976`.

## Futuros candidatos relacionados

- **ADR-0054 (potencial)**: autorização de `regex` em L1 se
  consumer (shaping) exigir suporte completo de `covers`.
- **`Covers` enum**: materializar se autorização de `regex`
  concedida.
- **`Stroke` tipo composto** (`{ width: Length, paint: Paint }`):
  próxima materialização complexa — reutiliza pattern.

---

## Estado final (Passo 132B encerrado)

- **Ficheiro materializado**: `01_core/src/entities/font_list.rs`
  (~180 linhas: 95 código + 85 tests).
- **3 tipos** criados: `FontFamily`, `FontList`, `Covers`
  (inabitado).
- **12 unit tests** todos verdes em `font_list.rs`.
- **6 integration tests** em `rules/eval/tests.rs`:
  string simples, array, dict rejeitado (unit), array vazio,
  array com não-string, tipo inválido.
- **1 canary consolidado** (`hyphenate_canary_passo_132b`)
  substituiu 5 canaries anteriores (126/127/128/129/131b).
- **10 testes migrados** ao todo: 5 L1 consolidados em 1 +
  3 L3 (`debt49_set_text_hyphenate_emite_warning`, multiplas,
  dedup) + 2 L4 (`cli_sucesso_com_warning`,
  `disciplina_warnings_antes_de_errors`).
- **L1 tests**: 838 → 852 (+14).
- **Workspace total**: 1069 → 1083 (+14).
- **ADR-0038 quinta nota** adicionada.
- **Pool DEBT-49 L3**: `hyphenate/alignment/stroke`
  (rotacionado).
- **Zero crates novas**. `regex` continua não autorizado.
- **Primeira materialização de tipo agregador em L1**:
  `FontList` contém `FontFamily` contém `Option<Covers>`.
- **Primeira rejeição explícita de form suportada vanilla**
  (`Value::Dict` → erro hard com mensagem clara).
- **`Covers` inabitado** (`enum Covers {}`) valida pattern
  "forma estrutural sem código activo" — compile-time test
  garante que adição de variantes futuras é additive.
