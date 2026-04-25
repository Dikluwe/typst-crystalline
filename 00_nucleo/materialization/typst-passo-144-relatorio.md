# Passo 144 — Relatório (lang hyphenation; ADR-0057; gap 7 DEBT-52)

**Data**: 2026-04-24
**Natureza**: passo **substantivo** (L1 + L0 + ADR + Cargo +
DEBT.md + README ADR). **Modelo "tudo-num-passo"**:
inventário condensado em §2/§3 cumpre espírito de ADR-0034 sem
diagnóstico-primeiro formal.
**Precondição**: Passo 145 encerrado; cabeçalhos dos ADRs
0017, 0027, 0028 e 0038–0051 uniformizados; 56 ADRs no índice
canónico; 10 DEBTs abertos; DEBT-1 + DEBT-52 fechados.

---

## 1. Sumário executivo

Gap 7 do DEBT-52 (lang hyphenation) materializado pós-fecho de
DEBT-1. **ADR-0057** autoriza `hypher = "0.1"` em L1 (pure-data,
`no_std`, zero deps, padrões TeX embebidos em compile-time).
Consumer integrado no algoritmo greedy de quebra de linha em
`01_core/src/rules/layout/cursor.rs::layout_word`: quando uma
palavra não cabe e `style.lang` é `Some(lang)`, tenta-se quebra
com hífen literal antes do `flush_line`.

`lang` muda de scope-out total (relatório 142 §3) para
**parcialmente consumido** — hyphenation activo; shaping
features (rustybuzz) continuam ausentes (DEBT-53 candidato XL).

**Tests**: 1095 → **1104** (+9: 5 unit `hyphenation::tests::*`
+ 4 integração `lang_hyphenation_*`). `cargo build` limpo;
`crystalline-lint .` zero violations.

---

## 2. Inventário das crates (sub-passo 144.1)

| Critério | `hypher` v0.1.7 | `hyphenation` v0.8 |
|----------|-----------------|--------------------|
| Pureza | `no_std`, sem `unsafe`, **zero deps** | depende de `bincode`, `pocket-resources` |
| Padrões | `include_bytes!` em compile-time (tries binários) | embebidos OU loaded em runtime |
| Tamanho do binário | ~1.1 MiB (todas as línguas, default) | ~3-5 MiB |
| Cobertura | 30+ idiomas (PT, EN, ES, FR, DE, IT, NL, …) | 33 idiomas |
| API | `pub fn hyphenate(word: &str, lang: Lang) -> Syllables<'_>` | `Standard` struct + métodos |
| ISO mapping | `Lang::from_iso(code: [u8; 2]) -> Option<Self>` | `Language` enum FromStr |
| `unsafe` | `#![forbid(unsafe_code)]` no source | usa-o internamente |
| Maturidade | Activa (mesmo autor do Typst) | Estável, mais antiga |

**Decisão**: `hypher` (justificação detalhada em ADR-0057 §3).

---

## 3. Inventário do ponto de integração (sub-passo 144.2)

**Quebra de linha**: `01_core/src/rules/layout/cursor.rs:41-53`,
função `layout_word`. Algoritmo **greedy** simples:

```rust
if cursor_x + word_width > right_margin && cursor_x > margin {
    flush_line();
}
emit FrameItem::Text { text: word, … };
cursor_x += word_width + space_width;
```

**`Lang` accessor**: `01_core/src/entities/lang.rs::as_str() -> &str`
existe desde Passo 131B (ADR-0052) — **nenhum accessor novo**
foi necessário em L1.

**Hífen literal**: Sem código pré-existente que emita `-`
explicitamente; PDF aceita hífen ASCII como qualquer outro
codepoint via Helvetica fallback ou CIDFont (ADR-0055).

**`TextStyle.lang`**: campo presente em
`01_core/src/entities/layout_types.rs:122` desde Passo 136
(Fase A DEBT-52). Capturado por eval; consumível por layout.

---

## 4. Decisão final

- **Crate**: `hypher = "0.1"` (default features: `alloc + full`).
- **Localização**: **L1** (`[l1_allowed_external]` autorizado
  em `crystalline.toml`).
- **Pipeline**: helper puro em
  `01_core/src/rules/layout/hyphenation.rs` invocado pelo
  `layout_word` quando word overflow + lang presente.
- **Política de fallback**: silent skip (3 cenários: ISO 3-letras;
  idioma fora do hyph-utf8; palavra sem pontos de quebra; doc
  sem `lang`).
- **Algoritmo**: greedy preserved; hyphenation tenta maior
  prefixo+hífen que cabe.

---

## 5. ADR-0057 produzida

Ficheiro: `00_nucleo/adr/typst-adr-0057-lang-hyphenation.md`.
Status: `IMPLEMENTADO`. Cabeçalho canónico desde a primeira
linha (cumpre P145):

```
# ⚖️ ADR-0057: Lang hyphenation em L1 via crate `hypher`

**Status**: `IMPLEMENTADO`
**Validado**: Passo 144.E — 9 testes (...).
**Data**: 2026-04-24
```

Secções: Contexto, Inventário condensado (cumpre ADR-0034),
Decisão (6 itens), Alternativas (5), Consequências
(positivas/negativas/neutras), Referências (8 ADRs + 2 DEBTs).

---

## 6. Diff resumido em `Cargo.toml` e `crystalline.toml`

**`Cargo.toml` (workspace)**:

```diff
+ hypher               = "0.1"   # ADR-0057 — hyphenation patterns embebidos (no_std, zero deps)
```

**`01_core/Cargo.toml`**:

```diff
+ hypher               = { workspace = true }  # ADR-0057 — hyphenation patterns no_std, zero deps
```

**`crystalline.toml`**:

```diff
[l1_allowed_external]
  ...
+   "hypher",        # ADR-0057 — hyphenation patterns embebidos; no_std, zero deps, sem unsafe
```

**`Cargo.lock`**: `hypher v0.1.7` resolvido (registry resolve
sem `--locked`; `--frozen` testes futuros).

---

## 7. Função `hyphenate`

**Assinatura**:

```rust
pub fn hyphenate(word: &str, lang: &Lang) -> Vec<usize>
```

**Localização**: `01_core/src/rules/layout/hyphenation.rs` —
módulo privado dentro de `layout` (visibilidade `super::`).

**Implementação**: mapeia `lang.as_str().as_bytes()` para
`[u8; 2]`; chama `hypher::Lang::from_iso(...)`; chama
`hypher::hyphenate(word, lang)`; converte iterador de
sílabas em `Vec<usize>` de offsets em **chars**.

**5 unit tests**:

| Test | Cenário | Assert |
|------|---------|--------|
| `hyphenate_palavra_en_devolve_pontos_correctos` | `"extensive"` em `"en"` | `vec![2, 5]` |
| `hyphenate_palavra_pt_devolve_pontos` | `"exemplo"` em `"pt"` | non-empty, in-range |
| `hyphenate_idioma_3_letras_devolve_vazio` | `"exemplo"` em `"por"` | `Vec::new()` |
| `hyphenate_idioma_sem_padroes_devolve_vazio` | `"anything"` em `"xx"` | `Vec::new()` |
| `hyphenate_palavra_curta_devolve_vazio` | `"ao"` em `"en"` | `Vec::new()` (bounds 2,3) |

---

## 8. Modificação no algoritmo de quebra de linha

Em `01_core/src/rules/layout/cursor.rs::layout_word`:

```diff
  pub(super) fn layout_word(&mut self, word: &str) {
      let w = self.word_width(word);
      let right_margin = self.page_config.width - self.page_config.margin;
      if self.cursor_x.0 + w.0 > right_margin && self.cursor_x.0 > self.page_config.margin {
+         // Passo 144 (ADR-0057): tentar hyphenation antes do flush.
+         if let Some(lang) = self.style.lang {
+             let break_points = super::hyphenation::hyphenate(word, &lang);
+             if !break_points.is_empty() {
+                 let available = right_margin - self.cursor_x.0;
+                 for &point in break_points.iter().rev() {
+                     let prefix: String = word.chars().take(point).collect();
+                     let prefix_with_hyphen = format!("{}-", prefix);
+                     let pw = self.word_width(&prefix_with_hyphen);
+                     if pw.0 <= available {
+                         self.current_line.push(FrameItem::Text {
+                             pos:   Point { x: self.cursor_x, y: self.cursor_y },
+                             text:  prefix_with_hyphen.into(),
+                             style: self.style.clone(),
+                         });
+                         self.cursor_x += pw;
+                         self.flush_line();
+                         let rest: String = word.chars().skip(point).collect();
+                         self.layout_word(&rest);
+                         return;
+                     }
+                 }
+             }
+         }
          self.flush_line();
      }
      // ... emit word as before ...
  }
```

**Recursão segura**: a chamada recursiva `layout_word(&rest)`
opera com `cursor_x = line_start_x` (após flush), e a
hyphenation pode disparar de novo no sufixo se necessário.
Sem ciclo: cada nível processa um prefixo estritamente menor.

---

## 9. Tests adicionados

### Unit (5 em `01_core/src/rules/layout/hyphenation.rs`)

`hyphenate_palavra_en_devolve_pontos_correctos`,
`hyphenate_palavra_pt_devolve_pontos`,
`hyphenate_idioma_3_letras_devolve_vazio`,
`hyphenate_idioma_sem_padroes_devolve_vazio`,
`hyphenate_palavra_curta_devolve_vazio`.

### Integração L3 (4 em `03_infra/src/integration_tests.rs`)

| Test | Cenário | Assert |
|------|---------|--------|
| `lang_hyphenation_en_palavra_longa_quebra_com_hifen` | doc 100pt-wide com `lang: "en"` + sentença longa | ≥1 `FrameItem::Text` com `text` terminando em `-` |
| `lang_hyphenation_pt_palavra_longa_quebra_com_hifen` | mesmo em PT | ≥1 |
| `lang_hyphenation_sem_set_lang_comportamento_inalterado` | doc sem `#set text(lang:)` | exactly 0 (regressão) |
| `lang_hyphenation_idioma_sem_padroes_silent_skip` | doc com `lang: "xx"` | exactly 0 (silent skip) |

**Helper privado**: `count_hyphenated_words(&doc) -> usize`
percorre `doc.pages → items → FrameItem::Text` e conta
strings que terminam em `-`.

**Page width nos testes**: 100pt × 400pt (margem 10pt) — força
overflow de palavras longas no FixedMetrics 0.6×size por char
(~13 chars/linha a 12pt).

---

## 10. Edições L0 + hash propagado

**`00_nucleo/prompts/rules/layout.md`**: adicionada secção
"Hyphenation (Passo 144, ADR-0057)" descrevendo o helper, o
fluxo de invocação, e a política de fallback.

**Hash recalculado**: `518a9856 → a78b0adc`. Propagado a 9
ficheiros de `01_core/src/rules/layout/`:

```
hyphenation.rs (novo, criado já com a78b0adc)
cursor.rs, mod.rs, equation.rs, grid.rs, helpers.rs,
metrics.rs, placement.rs, tests.rs (sed -i)
```

`crystalline-lint .`: ✓ No violations found.

---

## 11. DEBT-52 actualizado

`00_nucleo/DEBT.md` Secção 2 — entrada DEBT-52 ENCERRADO ganha
sub-secção "Actualização Passo 144 — Consumer `lang`
hyphenation (gap 7)". **DEBT-52 não reabre**: continua
encerrado; apenas anotação ao histórico análoga às de DEBT-1.
Contagem de DEBTs abertos **inalterada (10)**.

---

## 12. README dos ADRs actualizado

- **Cabeçalho**: 56 → 57 ADRs (55 → 56 números únicos).
- **Tabela "Estado por ADR"**: linha nova para ADR-0057
  (`IMPLEMENTADO`).
- **Distribuição de status**: `IMPLEMENTADO` 17 → 18.
- **Total**: 56 → 57 ADRs.
- **"Passos-chave da história dos ADRs"**: entrada nova
  para Passo 144 (após a entrada do Passo 145, com nota
  sobre a inversão da ordem de execução face à numeração).

---

## 13. Limitações registadas

1. **Algoritmo permanece greedy**, não Knuth-Plass. Justifica
   parágrafos com pontos de quebra simples; sem optimização
   global de "rios" (justificação Knuth-Plass é trabalho
   futuro fora deste passo).
2. **Sem hyphenation contextual**. `co-operate` não-quebra
   como `coop-erate` é responsabilidade do utilizador via soft
   hyphen `\u{00AD}` (não suportado nesta materialização).
3. **Sem soft-hyphen Unicode no input**. Hífen Unicode
   discreto (U+00AD) não é interpretado como sugestão de
   quebra; só `\u{002D}` literal é emitido como output.
4. **Padrões TeX são heurística**. Falsos positivos raros
   (palavras quebradas em pontos não-ideais) são possíveis e
   aceites dentro do perfil observacional graded de ADR-0054.
5. **`lang` é parcialmente consumido**. Hyphenation activo;
   shaping features (ligatures, kern, bidi via rustybuzz)
   permanecem ausentes. **DEBT-53 candidato XL** continua o
   trabalho remanescente para `lang` total.
6. **Crate `hypher` adiciona ~1.1 MiB ao binário** com
   default features. Mitigação opt-in por idioma (e.g.
   `default-features = false, features = ["english", "portuguese"]`)
   é decisão futura, não bloqueia 144.
7. **Padrões PT brasileiros vs europeus**: TeX hyph-utf8
   trata `pt-BR` e `pt-PT` como locales separados, mas
   `hypher::Lang::from_iso(b"pt")` mapeia para um único
   `Portuguese`. Se utilizadores reportam discrepância,
   abrir trabalho dedicado.
8. **Tests fragility**: `lang_hyphenation_*_palavra_longa_*`
   dependem de FixedMetrics (0.6×size por char). Mudanças no
   `metrics::advance` poderão invalidar a contagem de chars
   por linha. Mitigação: tests verificam ≥1 hífen, não posições
   exactas.

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| Inventário das crates registado em §2 e ADR-0057 | ✅ |
| ADR-0057 criada com `IMPLEMENTADO` | ✅ |
| `hypher` autorizado em `Cargo.toml` (workspace + 01_core) | ✅ |
| `hypher` autorizado em `crystalline.toml` (`[l1_allowed_external]`) | ✅ |
| `hyphenate(word, &lang) -> Vec<usize>` em L1 | ✅ |
| 5 unit tests do `hyphenation::tests` | ✅ verdes |
| Integração no `layout_word` (cursor.rs) | ✅ |
| 4 integration tests `lang_hyphenation_*` | ✅ verdes |
| Tests pré-existentes inalterados (zero regressão) | ✅ |
| `cargo test --workspace --lib` | 874 + 206 + 24 = **1104 passed** (+9 vs P145) |
| `crystalline-lint .` | ✅ zero violations |
| L0 `prompts/rules/layout.md` actualizado; hash `a78b0adc` | ✅ |
| Hash propagado a 9 ficheiros de `01_core/src/rules/layout/` | ✅ |
| DEBT-52 com actualização Passo 144 (DEBT continua encerrado) | ✅ |
| Contagem DEBTs abertos: **inalterada (10)** | ✅ |
| README dos ADRs: ADR-0057 na tabela + entrada Passos-chave | ✅ |
| Total ADRs: 56 → 57 | ✅ |
| Distribuição `IMPLEMENTADO`: 17 → 18 | ✅ |
| Limitações registadas (8 itens) | ✅ |

**Pós-144**: gap 7 fechado voluntariamente. Próximas decisões
não-bloqueantes (priorização tua):
- **Passo 142A** — multi-font per document (Passo escopo M).
- **ADR-0055bis** — selecção variant-aware (font-file Bold).
- **ADR-0054bis** — autorizar `regex` para font dict (gap 8).
- **DEBT-53** — rustybuzz integration (XL; cobre shaping
  features remanescentes de `lang`).
