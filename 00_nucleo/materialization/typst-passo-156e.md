# Passo 156E — pagebreak manual (Layout Fase 1 sub-passo 3)

**Série**: 156E (passo **substantivo escopo S+**;
materialização Fase 1 Layout, terceira sub-fase granular).
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → P156C (pad+hide) → P156D (h+v) → **P156E
(pagebreak)**.

**Precondição**: Passo 156D encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1192 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 44% (8/18 implementado puro pós-P156D); cobertura
user-facing total 56%.

**Numeração**: P156E segue P156D na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations).

**Natureza**: passo **substantivo escopo S+** (1 feature
estrutural com 2 atributos; ~15-22 testes adicionados
estimados; sem crates novas; sem ADRs novas; sem DEBTs
novos esperados).

**Decisão arquitectural P156E** (per resposta humana
2026-04-25): **`Content::Pagebreak { weak, to }` como variant
novo** em `Content` enum:

- `Content::Pagebreak { weak: bool, to: Option<Parity> }`
  — pagebreak é "event" sem body; coerente com vanilla
  `PagebreakElem`.
- Atributo `to` incluído desde já (não diferido).

Razão: vanilla expõe pagebreak como `#[elem]` proper
(`PagebreakElem` em
`lab/typst-original/crates/typst-library/src/layout/`).
Cristalino segue vanilla. Coerente com decisões P156C
(pad+hide) e P156D (h+v).

**Tipo novo `Parity`**: enum `{ Even, Odd }` em
`01_core/src/entities/parity.rs` (novo ficheiro). Análogo
ao `Sides<T>` criado em P156C — infraestrutura genérica
reutilizável.

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita variants
  novos.
- **ADR-0033**: paridade funcional para pagebreak.
- **ADR-0036**: atomização — consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded — pagebreak
  cumprido com aproximação aceite (`weak` semantic
  simplificada per P156D).
- **ADR-0061** (PROPOSTO): plano de Layout Fase X. Este
  passo aplica-o pela terceira vez. ADR-0061 mantém-se
  PROPOSTO até P156I.

---

## Contexto

P156D fechou h+v (8/18 = 44%). Próximo natural Fase 1 é
pagebreak manual — completa as features estruturais
triviais de Layout Fase 1 antes de entrar em Fase 2
(block, box, stack — granularidade fina).

**`pagebreak(weak, to)` em vanilla**:
- Atributo nomeado `weak: bool` (default false).
- Atributo nomeado `to: Option<Parity>` onde `Parity`
  é `{ Even, Odd }`.
- Semantic: força quebra de página manual.
- `weak: true` colapsa adjacente a outros pagebreaks
  weak.
- `to: Some(Even)` força próxima página a ser par;
  insere página vazia se necessário.
- `to: Some(Odd)` análogo para ímpar.

**Hipóteses a confirmar empiricamente** (não compromisso):

- `Parity` não existe em `01_core/src/entities/` —
  candidata a criar (análogo a `Sides<T>` em P156C).
- `Content::Pagebreak { weak: bool, to: Option<Parity> }`
  é forma final.
- Layouter pagebreak: força flush_page actual; se `to`
  exige paridade específica, verifica número de página
  resultante e insere página vazia se paridade não bate.
- `weak` collapse defere análogo a P156D.

---

## Objectivo

Ao fim do passo:

1. **Tipo `Parity`** criado em
   `01_core/src/entities/parity.rs`:

   ```rust
   pub enum Parity {
       Even,
       Odd,
   }

   impl Parity {
       pub fn matches(&self, page_number: usize) -> bool { ... }
   }
   ```

2. **Variant `Content::Pagebreak { weak, to }`** adicionado
   em `01_core/src/entities/content.rs`.

3. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content`:
   - `entities/content.rs::is_empty()`.
   - `entities/content.rs::plain_text()`.
   - `entities/content.rs::PartialEq::eq`.
   - `entities/content.rs::map_content`.
   - `entities/content.rs::map_text`.
   - `rules/introspect.rs::materialize_time`.
   - `rules/introspect.rs::walk`.
   - `rules/layout/mod.rs::layout_content`.
   - `rules/layout/mod.rs::measure_content_constrained`.

4. **`native_pagebreak`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#pagebreak(weak: ?, to: ?)`.

5. **Layouter pagebreak**: força flush da página actual;
   se `to` exige paridade específica, verifica e insere
   página vazia se necessário.

6. **Testes** unit + eval (~15-22 testes adicionados
   estimados):
   - `Parity` enum + método `matches`.
   - Construtor `Content::Pagebreak`.
   - `is_empty()`, `plain_text()`, `partial_eq`,
     `map_content`/`map_text` cobertura.
   - `eval_pagebreak` defaults (sem args).
   - `eval_pagebreak` com `weak: true`.
   - `eval_pagebreak` com `to: "even"` / `to: "odd"`.
   - `eval_pagebreak` rejeita `to` inválido.
   - `eval_pagebreak` rejeita named arg desconhecido.
   - Layouter: pagebreak força nova página.
   - Layouter: `to: Even` insere página vazia se actual
     é ímpar.
   - Layouter: `to: Odd` análogo.

7. **L0 prompts** + hashes propagados:
   - `00_nucleo/prompts/entities/parity.md` (novo).
   - `00_nucleo/prompts/entities/content.md` ganha secção
     "Variant `Content::Pagebreak` — Passo 156E".
   - Hash `entities/content.rs` recomputado (era
     `4a7e0a99` pós-P156D).
   - Hash `entities/parity.rs` novo.
   - Headers `@updated`: data execução.

8. **Inventário 148 actualizado**:
   - Tabela A.5 Layout: linha `pagebreak` manual
     ausente → `implementado`.
   - Cobertura Layout: 8/18 → **9/18 = 50%**.
   - Tabela A linha "Layout": `8/0/3/7/0=18` →
     `9/0/3/6/0=18`.
   - Total user-facing: 56% → **~57%**.
   - Tabela B Content variants: 47 → **48**.
   - §7 entrada 7: actualizar progresso Layout Fase 1
     (P156E cumprido; restantes 6 entradas Layout).

9. **README dos ADRs actualizado**:
   - Tabela "Estado por ADR": linha ADR-0061 mantém-se
     PROPOSTO.
   - Distribuição inalterada.
   - Total inalterado (61 ADRs).
   - Entrada nova em "Passos-chave da história dos ADRs"
     para P156E.

10. **ADR-0061 NÃO actualizada** neste passo (per decisão
    humana). Mantém-se PROPOSTO.

11. **Sem DEBTs criados/fechados** (esperado).

12. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156e-relatorio.md`.

Este passo **não**:

- Toca outros variants Content excepto adição.
- Toca código fora de `01_core/`.
- Materializa outras features Layout.
- Implementa collapse semantic real para `weak`
  (consistente com P156D).
- Adiciona show rules `#show pagebreak: ...`.
- Toca série paridade (suspensa em P153).
- Modifica ADR-0061.

---

## Decisões já tomadas

1. **Variant Content** (não Style). Coerente com vanilla
   `PagebreakElem` e com decisões de P156C+P156D.

2. **Atributo `to` incluído** desde já (per resposta
   humana 2026-04-25 contra opção mais conservadora de
   adiar `to`).

3. **Tipo `Parity` novo** em `01_core/src/entities/`.
   Análogo a `Sides<T>` criado em P156C — infraestrutura
   reusable.

4. **Granularidade**: 1 feature estrutural com 2 atributos
   num passo. Consistente com "12 passos granulares" mas
   ligeiramente mais escopo que P156C/P156D (que tinham
   2 features triviais cada).

5. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`
   per descobertas P156C/P156D.

6. **Assinatura natives**: 5-param canónica.

7. **`weak` armazenado mas collapse defere** — consistente
   com P156D.

8. **Tests adicionados**: alvo 15-22 (ajustável).

9. **ADR-0061 NÃO anotada** neste passo.

10. **Show rules adiadas**: candidato a passo agregado
    futuro.

## Decisões diferidas (resolvidas neste passo)

11. **Forma de `to` em stdlib**: aceitar como string
    (`"even"` ou `"odd"`) ou como Value::Symbol? Vanilla
    aceita string. **Default**: aceitar string + None
    (default). `extract_parity` helper novo.

12. **`to` com valor inválido**: rejeitar com erro hard
    explícito. `to: "anything"` → Err.

13. **Layouter pagebreak: como forçar quebra de página?**
    Verificar mecânica actual em `rules/layout/mod.rs`.
    Hipótese: existe método `force_pagebreak()` ou
    similar usado por overflow automático. Reusar.
    Se não existir, criar.

14. **Layouter pagebreak: como verificar paridade actual
    de página?**: usar `self.pages.len() + 1` (próxima
    página a ser criada). Trivial.

15. **Layouter pagebreak: inserir página vazia para
    ajustar paridade**: forçar segundo flush_page após
    o primeiro. Documentar em código.

16. **`weak` semantic real**: per P156D, defere. Atributo
    armazenado; layouter ignora.

17. **Página vazia tem cabeçalhos/rodapés?**: scope-out
    (header/footer não existem em Page actual; refino
    futuro em P156M).

---

## Escopo

**Dentro**:

- Criação de `01_core/src/entities/parity.rs`.
- Modificação de `01_core/src/entities/mod.rs` (registo
  `pub mod parity;`).
- Modificação de `01_core/src/entities/content.rs`
  (variant novo + arms cobertura).
- Modificação de `01_core/src/rules/introspect.rs`
  (`materialize_time` + `walk`).
- Modificação de `01_core/src/rules/layout/mod.rs`
  (`layout_content` + `measure_content_constrained`).
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_pagebreak` + helper `extract_parity`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo em `make_stdlib`).
- Tests em `01_core/src/entities/parity.rs::tests`,
  `01_core/src/entities/content.rs::tests`,
  `01_core/src/rules/stdlib/mod.rs::tests`,
  `01_core/src/rules/layout/tests.rs`.
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Implementação de outras features Layout.
- Show rules `#show pagebreak: ...`.
- Implementação real de collapse semantic para `weak`.
- Header/footer em página vazia.
- Crates externas.
- ADRs novas.
- DEBTs novos.
- Modificação de ADR-0061 (anotação cumulativa após
  P156I).
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156E.1 — Verificar pré-condições

```bash
view 01_core/src/entities/content.rs   # confirmar 47 variants pós-P156D
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
ls 01_core/src/entities/parity.rs 2>/dev/null  # NÃO existe
view 01_core/src/rules/layout/mod.rs   # mecânica de page break
grep -nE "fn flush_page|force_pagebreak|new_page" \
  01_core/src/rules/layout/mod.rs
```

Confirmar:
- Content tem 47 variants (HSpace e VSpace em P156D foram
  46º e 47º).
- `Parity` não existe.
- Layouter actual tem método para criar nova página
  (identificar nome real).

### 156E.2 — Criar `Parity` em `01_core/src/entities/parity.rs`

```rust
//! @prompt: prompts/entities/parity.md
//! @updated 2026-04-26
//! @prompt-hash <recompute>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    /// Returns true if `page_number` (1-indexed) matches
    /// this parity.
    pub fn matches(&self, page_number: usize) -> bool {
        match self {
            Parity::Even => page_number % 2 == 0,
            Parity::Odd => page_number % 2 == 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_matches_even_pages() {
        assert!(Parity::Even.matches(2));
        assert!(Parity::Even.matches(4));
        assert!(!Parity::Even.matches(1));
        assert!(!Parity::Even.matches(3));
    }

    #[test]
    fn parity_matches_odd_pages() {
        assert!(Parity::Odd.matches(1));
        assert!(Parity::Odd.matches(3));
        assert!(!Parity::Odd.matches(2));
    }
}
```

Registar em `01_core/src/entities/mod.rs`:

```rust
pub mod parity;
```

### 156E.3 — Adicionar variant Content::Pagebreak

Edição de `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... 47 variants existentes (Text, Strong, ..., HSpace,
    //     VSpace)
    Pagebreak {
        weak: bool,
        to:   Option<Parity>,
    },
}
```

**Variant count**: 47 → **48** (+1).

### 156E.4 — Cobertura exaustiva de arms

Para cada função em `01_core/` que pattern-match sobre
Content, adicionar arm para `Pagebreak`:

| Função | Tratamento |
|--------|-----------|
| `is_empty()` | `false` (pagebreak tem efeito mesmo "vazio") |
| `plain_text()` | `String::new()` (não rende texto) |
| `PartialEq::eq` | comparação 2 fields (weak + to) |
| `map_content` | terminal (clone) |
| `map_text` | terminal (clone) |
| `materialize_time` | clone (leaf, sem body) |
| `walk` | no-op |
| `layout_content` | force flush_page + verifica paridade + insere vazia se necessário |
| `measure_content_constrained` | `(0.0, 0.0)` (event, sem dimensões) |

### 156E.5 — `native_pagebreak` + `extract_parity`

Em `01_core/src/rules/stdlib/layout.rs`:

```rust
fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd" => Ok(Parity::Odd),
            other => Err(invalid_argument(format!(
                "to must be \"even\" or \"odd\", got {:?}", other))),
        },
        _ => Err(invalid_argument("to must be a string")),
    }
}

pub fn native_pagebreak(_ctx: &mut EvalContext, args: &Args,
                        _world: &dyn World, _current_file: FileId,
                        _figure_numbering: Option<&str>)
    -> SourceResult<Value>
{
    // sem argumento posicional
    if !args.items.is_empty() {
        return Err(unexpected_positional(
            "pagebreak takes no positional arguments"));
    }

    let mut weak = false;
    let mut to: Option<Parity> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => weak = expect_bool(value)?,
            "to" => to = Some(extract_parity(value)?),
            other => return Err(unexpected_named(other)),
        }
    }

    Ok(Value::Content(Content::Pagebreak { weak, to }))
}
```

Registo em `make_stdlib`:

```rust
scope.define("pagebreak",
    Value::Func(Func::native("pagebreak", native_pagebreak)));
```

Re-export em `stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_grid, native_h, native_hide,
    native_pad, native_page, native_pagebreak, native_place,
    native_v,
};
```

Stdlib funcs: 36 → **37** (+1).

### 156E.6 — Layouter pagebreak

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    // ... arms existentes (incluindo HSpace, VSpace de P156D)
    Content::Pagebreak { weak: _, to } => {
        // weak collapse defere
        // 1. Força flush da página actual
        if !self.current_items.is_empty() || self.cursor_x.0 > self.line_start_x.0 {
            self.flush_line();
            self.flush_page();  // ou nome real do método
        } else {
            // Se página actual está vazia, ainda força nova página
            self.flush_page();
        }

        // 2. Se `to` exige paridade específica, verifica
        if let Some(parity) = to {
            // self.pages.len() é número de páginas já escritas;
            // próxima página a ser escrita é len + 1
            let next_page_number = self.pages.len() + 1;
            if !parity.matches(next_page_number) {
                // Insere página vazia para ajustar paridade
                self.flush_page();
            }
        }
    }
    // ... fallback
}
```

Em `measure_content_constrained`:

```rust
match content {
    Content::Pagebreak { .. } => (0.0, 0.0),
    // ... fallback
}
```

### 156E.7 — Tests adicionados (alvo 15-22)

| Ficheiro | Testes |
|----------|--------|
| `01_core/src/entities/parity.rs::tests` | (1) parity_matches_even; (2) parity_matches_odd |
| `01_core/src/entities/content.rs::tests` | (3) pagebreak_constructor; (4) pagebreak_is_empty_returns_false; (5) pagebreak_plain_text_vazio; (6) pagebreak_partial_eq; (7) pagebreak_map_text_preserva |
| `01_core/src/rules/stdlib/mod.rs::tests` | (8) `native_pagebreak` defaults (sem args); (9) `native_pagebreak` com weak; (10) `native_pagebreak` com to:"even"; (11) `native_pagebreak` com to:"odd"; (12) `native_pagebreak` com weak+to combinado; (13) `native_pagebreak` rejeita to inválido; (14) `native_pagebreak` rejeita named arg desconhecido; (15) `native_pagebreak` rejeita argumento posicional; (16) `native_pagebreak` rejeita weak não-Bool; (17) `native_pagebreak` rejeita to não-Str |
| `01_core/src/rules/layout/tests.rs` | (18) layout_pagebreak_forca_nova_pagina; (19) layout_pagebreak_to_even_insere_vazia_se_actual_impar; (20) layout_pagebreak_to_odd_insere_vazia_se_actual_par; (21) layout_pagebreak_to_match_nao_insere_extra |

**Total**: ~21 tests novos. Tests cumulativos: **1192 →
~1213**.

### 156E.8 — L0 prompts + hashes

Criar `00_nucleo/prompts/entities/parity.md` com:
- Cabeçalho canónico (camada L1, ficheiro alvo, ADRs).
- Contexto + interface pública + semântica.
- Consumers actuais (Content::Pagebreak) + planeados.

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Variant `Content::Pagebreak` — Passo
156E" após secção P156D h+v:

```markdown
## Variant Pagebreak (Passo 156E)

`Content::Pagebreak { weak: bool, to: Option<Parity> }`:
- event sem body que força quebra de página manual;
- `weak` armazenado mas collapse defere (P156E scope-out
  per P156D);
- `to: Some(Parity::Even)` força próxima página par;
  insere página vazia se necessário;
- `to: Some(Parity::Odd)` análogo.

Segue ADR-0026 perfil "variant novo" porque vanilla expõe
como `#[elem]` proper.
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

Verificar:
- `entities/content.rs`: hash novo (era `4a7e0a99` pós-
  P156D; será diferente).
- `entities/parity.rs`: hash novo.
- L0 prompts hash propagado.

### 156E.9 — Inventário 148 actualizado

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A.5 Layout**:
- linha `pagebreak` manual: ausente → **implementado**
  (referência: Passo 156E).

**Tabela A linha "Layout"**:
- antes: `8 | 0 | 3 | 7 | 0 | 18`
- depois: `9 | 0 | 3 | 6 | 0 | 18`
- cobertura Layout: 44% → **50%**.

**Total user-facing**:
- antes: `58 | 21 | 22 | 38 | 2 | 141`.
- depois: `59 | 21 | 22 | 37 | 2 | 141`.
- cobertura user-facing: (59+21)/141 = **57%** (era 56%).

**Tabela B Content**:
- variants cristalino: 47 → **48**.
- variants vanilla extra ausentes: ~7 → ~6.

**§7 entrada 7**: actualizar progresso Layout Fase 1
(P156E cumprido; restantes 6 entradas Layout: skew,
block, box, stack, repeat, columns/colbreak).

### 156E.10 — README ADRs actualizado

- Tabela "Estado por ADR": linha ADR-0061 mantém-se
  PROPOSTO.
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova:
  ```
  P156E: aplicação terceira de ADR-0061 — pagebreak manual
  com weak+to. Cobertura Layout 44% → 50%. Tipo `Parity`
  criado.
  ```

### 156E.11 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156e-relatorio.md`.

Secções (modelo P156D):
1. Sumário executivo.
2. Inventário pré-materialização.
3. Tipo `Parity` — forma final + diff.
4. Variant `Content::Pagebreak` — forma final + diff.
5. Cobertura exaustiva de arms.
6. `native_pagebreak` + `extract_parity` — assinaturas
   + registo.
7. Layouter — diff (force flush_page + paridade check).
8. Tests adicionados (lista + contagens).
9. L0 prompts + hashes propagados.
10. Inventário 148 actualizado.
11. README ADRs actualizado.
12. Próximo passo (P156F = skew).
13. Limitações registadas.
14. Verificação final.

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1213 passed
   (1192 → +21); zero falhas.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes:
   - `entities/content.rs` ↔ `entities/content.md`.
   - `entities/parity.rs` ↔ `entities/parity.md`.
5. ✅ `Content::Pagebreak` variant em produção (47 → 48).
6. ✅ `Parity` enum em produção.
7. ✅ Stdlib `#pagebreak(weak: ?, to: ?)` invocável
   (36 → 37 funcs).
8. ✅ Cobertura arms exaustiva.
9. ✅ Layouter força nova página com paridade verificada.
10. ✅ Inventário 148 reflecte cobertura aumentada
    (44% → 50%).
11. ✅ README ADRs entrada P156E.
12. ✅ Sem ADR criada / revogada / revisada.
13. ✅ Sem DEBT criado / fechado.
14. ✅ ADR-0061 inalterada.
15. ✅ Sem regressão.
16. ✅ Relatório do passo escrito.

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Parity` enum compila + tests passam | ✅ |
| 2 | `Content::Pagebreak` compila + tests passam | ✅ |
| 3 | Stdlib `#pagebreak()` defaults invocável | ✅ |
| 4 | Stdlib `#pagebreak(weak: true)` invocável | ✅ |
| 5 | Stdlib `#pagebreak(to: "even")` / `to: "odd"` invocável | ✅ |
| 6 | Layouter força nova página | ✅ |
| 7 | Layouter `to: Even` insere página vazia se actual ímpar | ✅ |
| 8 | Layouter `to: Odd` análogo | ✅ |
| 9 | Inventário 148 reflecte cobertura 50% Layout | ✅ |
| 10 | Próximo passo (156F = skew) tem âncora | ✅ |
| 11 | Sem regressão | ✅ |
| 12 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

- **Mecânica de `flush_page` actual diferente do esperado**:
  layouter pode ter método com nome diferente (e.g.
  `commit_page`, `finalize_page`, `new_page`). Verificar
  durante 156E.1 e usar nome real.

- **Página actual vazia + pagebreak**: edge case.
  Comportamento esperado: força nova página mesmo se
  vazia. Implementação em 156E.6 cobre.

- **`to` com paridade já correcta**: se actual é par e
  pedimos `to: Even`, **não inserir página vazia
  adicional**. Layouter verifica `parity.matches()`.

- **Pagebreak no início absoluto do documento (página 1
  vazia)**: aceita; primeira página fica vazia, segunda
  ganha conteúdo. Aceitável; case patológico não comum.

- **`to` aceito como Symbol em vez de String**: vanilla
  pode aceitar `to: even` (symbol) sem aspas. Cristalino:
  apenas string. Se for inconveniente, **registar como
  divergência consciente** análoga a P155 e P156C.

- **`expect_bool` helper já existe**: criado em P156D.
  Confirmar e reusar.

- **Tests Layout pagebreak rendering**: verificação visual
  via PDF é scope-out. Tests verificam contagem de páginas
  e estrutura interna apenas (consistente com P140B/P155/
  P156C/P156D).

- **`weak` collapse semantic real exigida pelo user**:
  per spec, scope-out. Documentar como limitação.

- **Página vazia inserida não tem dimensões correctas**:
  layouter deve usar `page_config` actual (mesmas
  dimensões da página normal). Verificar empiricamente.

- **Volume tests excede 22**: aceitável; ajustar no
  relatório.

- **Volume tests inferior a 15**: investigar; weak + to
  + edge cases (paridade match/no-match) devem gerar
  ~15-22.

- **Pattern-match em todos os locais incluindo
  `walk`/`materialize_time`**: Pagebreak é leaf sem body.
  Adicionar arms no-op. Linter V2 alerta se omitido.

- **Conflito naming `pagebreak`**: improvável; vanilla
  não tem outra função `pagebreak`.

---

## Notas operacionais

- **Padrão "passos granulares"**: terceira aplicação
  consecutiva (P156C+P156D+P156E). N=3 testes hipótese
  da decisão humana 2026-04-25.

- **Localização canónica `stdlib/layout.rs`**: aplicada
  directamente.

- **Assinatura natives 5-param**: aplicada directamente.

- **Helpers reusados**: `expect_bool` de P156D. **Helper
  novo**: `extract_parity`. Antecipável reuso futuro
  improvável (parity é caso isolado).

- **Tipo `Parity` novo**: análogo a `Sides<T>` em P156C.
  Infraestrutura genérica criada em passo material.
  Disponível para futuros consumers se aparecerem.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.

- **Variants count**: 47 → **48** (+1). Após P156F
  (skew): 49. Após P156G (block): 50. Após P156I
  (stack): 51 (sem footnote area).

- **Stdlib funcs**: 36 → **37** (+1). Após P156I: ~40.

- **Pós-156E**:
  - 5 features Layout implementadas total (pad, hide,
    h, v, pagebreak).
  - Cobertura Layout: 44% → **50%**.
  - Cobertura user-facing total: 56% → 57%.
  - **Próximo**: P156F (skew — 1 trivial via Content::Transform
    extension) ou alternativa humana.

- **Padrão emergente — passos consecutivos com helpers
  partilhados**: P156C criou `extract_length`; P156D
  reusou + criou `build_spacing` + `extract_weak`; P156E
  reusa `expect_bool` + cria `extract_parity`. Acumulação
  natural de infraestrutura útil.

- **Quarentena vanilla**: continua opção 3. Sem mudança.

- **Série paridade**: continua suspensa em P153. Sem
  mudança.

- **Granularidade vs paridade rápida — N=3 esperado**:
  P156C+P156D=2 aplicações sem reformulação. P156E é
  N=3. Se mantém padrão, hipótese fica praticamente
  provada por consistência empírica.

- **Halfway point Fase 1 atingido**: P156E fecha 5/6
  features Fase 1 declaradas em ADR-0061 §6.1 (sem
  footnote area, que sai da Fase 1 per decisão humana
  2026-04-25). 50% cobertura Layout marca metade do
  caminho para 72% (target P156I).
