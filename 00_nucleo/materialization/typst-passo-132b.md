# Passo 132B — Materialização de `FontList` em L1

**Série**: 132B (passo **S-M** em L1; segundo de dois sub-passos
para materializar `font`).
**Precondição**: Passo 132A encerrado; diagnóstico
`00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`
aprovado; ADR-0053 em `PROPOSTO` em
`00_nucleo/adr/typst-adr-0053-font-tipo-composto.md`; 1069
total tests; zero violations; 52 ADRs activas (ADR-0052
`IMPLEMENTADO`); 11 DEBTs abertos.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — este passo cumpre 2 de
  3 forms do vanilla (string, array); dict rejeitada
  conscientemente.
- **ADR-0034** — diagnóstico cumprido em 132A.
- **ADR-0037** (coesão por domínio) — ficheiro dedicado
  `entities/font_list.rs`.
- **ADR-0038** — ganha quinta nota.
- **ADR-0053** — transita `PROPOSTO` → `IMPLEMENTADO`.

**Natureza**: passo L1. Código novo + arm com validação +
migração de canary em 10 testes (5 L1 + 3 L3 + 2 L4) +
rotação de pool DEBT-49. Pattern DEBT-1 XS **não se aplica** —
refactor com âmbito alargado.

---

## Contexto

Passo 132A produziu diagnóstico com 9 decisões numeradas e
ADR-0053 `PROPOSTO`. Decisões aprovadas entrando neste passo:

1. **Forma interna**: réplica vanilla com adaptação
   deliberada — `struct FontList(Vec<FontFamily>)` newtype
   non-empty; `struct FontFamily { name: EcoString, covers:
   Option<Covers> }`.
2. **`Covers` como enum inabitado**:
   ```rust
   pub enum Covers {}
   ```
   Zero variantes. `Option<Covers>` só pode ser `None`.
   Reserva forma estrutural para futuro (quando `regex` for
   autorizado ou tipo próprio de coverage for criado) sem
   custo de código activo.
3. **3 forms do vanilla, 2 capturadas + 1 rejeitada**:
   - `Value::Str` → `FontList` de 1 elemento (covers=None).
   - `Value::Array<Str>` → `FontList` de N elementos.
   - `Value::Dict` → `Err` com mensagem literal
     `"dict form of font not yet supported — use string or array of strings"`.
4. **Erro hard em inválido**: alinhado 131B. Array vazio,
   array com elementos não-string, dict, ou outros tipos
   produzem `Err`.
5. **Nome lowercased**: vanilla normaliza name para lowercase.
   Replicar.
6. **API mínima**: `FontList::new`, `FontList::as_slice`,
   `FontList::len`, iteração implícita via `AsRef<[FontFamily]>`
   ou método `.families()`. Decisão final no ficheiro.
7. **Localização**: `01_core/src/entities/font_list.rs`.
8. **Canary substituto**: `hyphenate` (Value::Bool). Input
   dos 10 testes passa de `#set text(font: "X")` para
   `#set text(hyphenate: true)`. Assertion muda de `'font'`
   para `'hyphenate'`.
9. **Pool DEBT-49 L3**: `font/alignment/stroke` →
   `hyphenate/alignment/stroke`.

---

## Contexto estratégico

Sub-passo 2 de 4 restantes para fechar DEBT-1:

- **132B** (este): materialização `FontList` + captura de
  `font` + migração canary DEBT-50 + rotação pool DEBT-49.
- **133**: activar target `par` em `eval_set_rule`.
- **134**: migrar `leading` de `text` para `par`.
- **135**: fechar DEBT-1 no DEBT.md.

Após 132B, a lista canónica DEBT-1 ("text.font, text.lang,
par.leading, text.weight como string") fica toda capturada
com a ressalva da divergência temporal de `leading`
(capturado em `text` em vez de `par`, a resolver em 133+134).

---

## Objectivo

Ao fim do passo:

1. Tipo `FontList` + `FontFamily` + `Covers` (inabitado)
   materializados em `01_core/src/entities/font_list.rs`.
2. `StyleDelta.font: Option<FontList>` adicionado (novo campo).
3. Arm `"font"` em `eval_set_text` valida:
   - `Value::Str` → captura.
   - `Value::Array` → captura se todos elementos são `Str`.
   - `Value::Dict` → `Err` com mensagem literal.
   - Outros tipos → `Err`.
4. 10 testes migrados de canary `font` para canary `hyphenate`:
   - 5 L1: `eval_set_text_font_canary_passo_{126,127,128,129,131b}`.
   - 3 L3: `debt49_set_text_font_emite_warning`,
     `debt49_set_text_multiplas_propriedades_desconhecidas`,
     `debt49_dedup_warnings_identicos`.
   - 2 L4: `cli_sucesso_com_warning`,
     `disciplina_warnings_antes_de_errors`.
5. Novos testes unit em `font_list.rs` (12-15 casos).
6. Novos integration tests em `rules/eval/tests.rs` (4-5).
7. ADR-0053 transita `PROPOSTO` → `IMPLEMENTADO`.
8. ADR-0038 ganha quinta nota.

Este passo **não**:

- Suporta `covers` (dict form rejeitada).
- Autoriza `regex` em L1.
- Valida formato de nomes de fonte (aceita qualquer string).
- Integra com `FontBook` (sem lookup, sem resolução).
- Implementa consumer em layout.
- Fecha DEBT-1.

---

## Escopo

**Dentro**:
- `01_core/src/entities/font_list.rs` (**NOVO**).
- `01_core/src/entities/mod.rs` — expor `pub mod font_list;`.
- `01_core/src/entities/style_chain.rs` — campo novo + import.
- `01_core/src/rules/eval/rules.rs` — arm `"font"` + imports.
- `01_core/src/rules/eval/tests.rs` — 5 testes canary
  migrados + 4-5 integration tests novos.
- `03_infra/src/integration_tests.rs` — 3 testes migrados +
  pool DEBT-49 actualizado.
- `04_wiring/src/tests.rs` (ou equivalente) — 2 testes L4
  migrados.
- `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md` —
  status `PROPOSTO` → `IMPLEMENTADO`.
- `00_nucleo/adr/typst-adr-0038-*.md` — quinta nota.
- `00_nucleo/prompts/entities/style_chain.md` + hash.
- `00_nucleo/prompts/entities/font-list.md` + hash (**NOVO**
  se L0 padrão exige prompt por tipo).

**Fora**:
- `regex` crate authorization.
- `covers` implementation.
- Integração com `FontBook`.
- Consumer em layout.
- `region`, `script`, `dir`.

---

## Sub-passos

### 132B.A — Inventário confirmatório

Leitura rápida. Sem edição.

**A.1 — Confirmar estado do diagnóstico vs realidade**:
- `grep -n "pub font:" 01_core/src/entities/style_chain.rs` —
  confirmar que campo `font` **não existe** (será adicionado).
- `grep -n "\"font\"" 01_core/src/rules/eval/rules.rs` —
  confirmar que arm `"font"` **não existe** (será adicionado).
- `ls 01_core/src/entities/font_list.rs` — confirmar que
  ficheiro não existe.

**A.2 — Confirmar `Value::Array` disponível em L1**:
- `grep -rn "Value::Array\|Array(" 01_core/src/entities/value.rs`
  (ou caminho equivalente).
- Registar variante exacta e como aceder ao conteúdo (`Vec<Value>`?
  `Arc<[Value]>`? outro?).
- Se `Value::Array` não existe, **parar e reportar** — forma
  array não pode ser capturada sem o tipo base.

**A.3 — Confirmar `Value::Dict` disponível em L1**:
- Similar a A.2. Registar variante e acesso.
- Se não existe, rejeitar dict é trivial (não há match arm).

**A.4 — Confirmar `hyphenate` não é propriedade capturada**:
- `grep -n "\"hyphenate\"" 01_core/src/rules/eval/rules.rs` —
  esperado: nenhum resultado.
- Se existe arm, o canary novo já é propriedade conhecida e
  não emite warning. **Parar e reportar**.

**A.5 — Localizar testes L4 afectados**:
- `grep -rn "font_canary\|font.*canary\|font.*warning" 04_wiring/`
  (ou caminho real de L4).
- Confirmar lista do diagnóstico 132A: 2 testes
  (`cli_sucesso_com_warning`, `disciplina_warnings_antes_de_errors`).
- Registar caminho exacto dos ficheiros.

**A.6 — Contagem de testes base**:
- L1: 838 (pós-131B).
- L2: 24.
- L3: 186.
- L4: 21.
- Total: 1069.

**Gate 132B.A**:
- Se A.2 revela `Value::Array` inexistente: **parar**. Fundação
  do passo em dúvida.
- Se A.4 revela `hyphenate` já capturado: **parar, escolher
  canary alternativo** (candidatos: `first-line-indent`,
  `dir`, `region`).
- Se A.5 revela testes L4 em caminho diferente do esperado:
  ajustar enunciado antes de executar.
- Outros casos: prosseguir para 132B.B.

### 132B.B — ADR-0038 quinta nota

Adicionar ao final de ADR-0038:

```markdown
### Nota Passo 132B — `font` como tipo composto com divergência consciente

Campo `StyleDelta.font` foi materializado como
`Option<FontList>` (tipo composto agregando `FontFamily` e
`Covers` inabitado). Arm `"font"` emite `Err` hard em inválido
e rejeita forma dict com mensagem clara.

**Paridade ADR-0033 parcial**: string e array capturadas;
dict rejeitada por ausência de suporte a `regex` em L1. Ver
ADR-0053 para contexto completo e plano de evolução.

Primeira materialização de tipo agregador em L1 (precedentes
131B/ADR-0052 foi tipo folha `Lang`). Segunda aplicação do
padrão "diagnóstico separado + materialização" (131A/B →
132A/B).

Futuras materializações análogas (Stroke, Region, Dir) seguem
padrão 131/132: diagnóstico obrigatório, ADR dedicada,
materialização em passo seguinte.
```

### 132B.C — Materializar `FontList`

**Ficheiro novo**: `01_core/src/entities/font_list.rs`.

```rust
//! Tipo `FontList` — lista priorizada de famílias de fonte
//! com suporte opcional a coverage filtering.
//!
//! Réplica estrutural de `typst::text::FontList` vanilla com
//! divergência consciente:
//! - `covers` é inabitado neste passo (tipo `enum Covers {}`).
//! - Dict form do vanilla é rejeitada até `regex` ser
//!   autorizado em L1 (ver ADR-0053).
//!
//! Ver ADR-0053 e diagnóstico
//! `00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`.

use ecow::EcoString;

/// Enum inabitado. Reserva forma estrutural para futuro
/// suporte a coverage filtering. Ver ADR-0053 decisão 2.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Covers {}

/// Família de fonte com coverage opcional.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FontFamily {
    /// Nome da família, lowercased na construção.
    pub name: EcoString,
    /// Coverage filter. Sempre `None` neste passo (Covers
    /// inabitado).
    pub covers: Option<Covers>,
}

impl FontFamily {
    /// Constrói família a partir de string, normalizando name
    /// para lowercase.
    pub fn new(name: EcoString) -> Self {
        Self {
            name: name.to_lowercase().into(),
            covers: None,
        }
    }
}

/// Lista priorizada de famílias. Non-empty por construção.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FontList(Vec<FontFamily>);

impl FontList {
    /// Constrói lista a partir de vector non-empty.
    /// Devolve `None` se vazio.
    pub fn new(families: Vec<FontFamily>) -> Option<Self> {
        if families.is_empty() {
            None
        } else {
            Some(Self(families))
        }
    }

    /// Constrói lista com uma única família (forma string do
    /// vanilla).
    pub fn single(name: EcoString) -> Self {
        Self(vec![FontFamily::new(name)])
    }

    /// Slice das famílias.
    pub fn as_slice(&self) -> &[FontFamily] {
        &self.0
    }

    /// Número de famílias.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_family_new_normaliza_lowercase_passo_132b() {
        let f = FontFamily::new(EcoString::from("Arial"));
        assert_eq!(f.name, "arial");
        assert!(f.covers.is_none());
    }

    #[test]
    fn font_family_new_preserva_case_insensitive_passo_132b() {
        let f1 = FontFamily::new(EcoString::from("arial"));
        let f2 = FontFamily::new(EcoString::from("ARIAL"));
        let f3 = FontFamily::new(EcoString::from("Arial"));
        assert_eq!(f1.name, f2.name);
        assert_eq!(f2.name, f3.name);
    }

    #[test]
    fn font_family_covers_sempre_none_passo_132b() {
        // Covers é enum inabitado; só pode ser None.
        let f = FontFamily::new(EcoString::from("any"));
        match f.covers {
            None => {} // OK, único caso possível.
            Some(_) => unreachable!("Covers é inabitado"),
        }
    }

    #[test]
    fn font_list_single_tem_um_elemento_passo_132b() {
        let list = FontList::single(EcoString::from("Arial"));
        assert_eq!(list.len(), 1);
        assert_eq!(list.as_slice()[0].name, "arial");
    }

    #[test]
    fn font_list_new_rejeita_vector_vazio_passo_132b() {
        let result = FontList::new(vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn font_list_new_aceita_um_elemento_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("arial")),
        ]);
        assert!(list.is_some());
        assert_eq!(list.unwrap().len(), 1);
    }

    #[test]
    fn font_list_new_aceita_multiplos_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("inria serif")),
            FontFamily::new(EcoString::from("noto sans")),
            FontFamily::new(EcoString::from("libertinus")),
        ]);
        assert!(list.is_some());
        assert_eq!(list.unwrap().len(), 3);
    }

    #[test]
    fn font_list_preserva_ordem_passo_132b() {
        let list = FontList::new(vec![
            FontFamily::new(EcoString::from("primeira")),
            FontFamily::new(EcoString::from("segunda")),
        ]).unwrap();
        assert_eq!(list.as_slice()[0].name, "primeira");
        assert_eq!(list.as_slice()[1].name, "segunda");
    }

    #[test]
    fn font_list_partial_eq_passo_132b() {
        let a = FontList::single(EcoString::from("arial"));
        let b = FontList::single(EcoString::from("arial"));
        let c = FontList::single(EcoString::from("helvetica"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn font_list_clone_passo_132b() {
        let a = FontList::single(EcoString::from("arial"));
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn covers_inabitado_passo_132b() {
        // Não é possível construir Covers. Este teste valida
        // estruturalmente via tipagem — se Covers tivesse
        // variantes, o código abaixo não compilaria.
        fn _nunca_chamado(c: Covers) -> ! {
            match c {} // match vazio é exaustivo para enum inabitado.
        }
    }
}
```

**Expor em `entities/mod.rs`**:

```rust
pub mod font_list;
```

### 132B.D — Migrar `StyleDelta`

**Ficheiro**: `01_core/src/entities/style_chain.rs`.

Adicionar:

```rust
use crate::entities::font_list::FontList;

pub struct StyleDelta {
    // ... campos existentes ...
    /// Lista priorizada de famílias de fonte. `None` = herdado.
    pub font: Option<FontList>,
}

// empty() ganha font: None.
```

### 132B.E — Adaptar arm `"font"` em `eval_set_text`

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

Imports:

```rust
use crate::entities::font_list::{FontList, FontFamily};
```

Arm novo (entre arms existentes):

```rust
"font" => {
    match val {
        Value::Str(s) => {
            delta.font = Some(FontList::single(s));
        }
        Value::Array(arr) => {
            let mut families = Vec::with_capacity(arr.len());
            for item in arr.iter() {
                if let Value::Str(s) = item {
                    families.push(FontFamily::new(s.clone()));
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        named.expr().span(),
                        "font array must contain only strings".to_string(),
                    )]);
                }
            }
            match FontList::new(families) {
                Some(list) => delta.font = Some(list),
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        named.expr().span(),
                        "font array must not be empty".to_string(),
                    )]);
                }
            }
        }
        Value::Dict(_) => {
            return Err(vec![SourceDiagnostic::error(
                named.expr().span(),
                "dict form of font not yet supported — use string or array of strings".to_string(),
            )]);
        }
        _ => {
            // Outros tipos: Err hard coerente com 131B.
            return Err(vec![SourceDiagnostic::error(
                named.expr().span(),
                "font expects a string or array of strings".to_string(),
            )]);
        }
    }
}
```

**Notas**:
- Adaptação ao tipo real de `Value::Array` (confirmado em 132B.A.2).
- `arr.iter()` assume iteração sobre `&Value`. Se `Value::Array(Vec<Value>)` direct, OK.
- Clone de `EcoString` é O(1) (refcounted).

### 132B.F — Migrar canary L1 (5 testes)

**Ficheiro**: `01_core/src/rules/eval/tests.rs`.

Para cada um dos 5 testes listados no diagnóstico:

- `eval_set_text_font_canary_passo_126`
- `eval_set_text_font_canary_passo_127`
- `eval_set_text_font_canary_passo_128`
- `eval_set_text_font_canary_passo_129`
- `eval_set_text_font_canary_passo_131b`

Renomear para `eval_set_text_hyphenate_canary_passo_132b_N` (onde
N é um sufixo numérico, ou apenas manter 1 teste consolidado).

**Alternativa recomendada**: consolidar os 5 em **1 teste único**
— o propósito do canary é valer como sinal de vida. 1 teste
consolidado reduz ruído.

```rust
#[test]
fn eval_set_text_hyphenate_canary_passo_132b() {
    // Canary migrou de `font` para `hyphenate` no Passo 132B.
    // `font` agora é capturado com FontList; `hyphenate` é a
    // propriedade conhecida não-capturada que serve como sinal
    // de vida do mecanismo de warnings.
    let src = "#set text(hyphenate: true)\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'hyphenate'")));
}
```

Decisão consolidação: se consolidar, **remover** os 5 canaries
antigos e adicionar apenas este. Se não, substituir input de
cada um.

### 132B.G — Novos integration tests em L1

Adicionar em `01_core/src/rules/eval/tests.rs`:

```rust
#[test]
fn eval_set_text_font_string_simples_passo_132b() {
    let src = r#"#set text(font: "Arial")"#;
    let (delta, warnings) = eval_inline(src);
    let font = delta.font.expect("font captured");
    assert_eq!(font.len(), 1);
    assert_eq!(font.as_slice()[0].name, "arial");
    assert!(warnings.iter().all(|w| !w.message.contains("'font'")));
}

#[test]
fn eval_set_text_font_array_passo_132b() {
    let src = r#"#set text(font: ("Inria Serif", "Noto Sans"))"#;
    let (delta, warnings) = eval_inline(src);
    let font = delta.font.expect("font captured");
    assert_eq!(font.len(), 2);
    assert_eq!(font.as_slice()[0].name, "inria serif");
    assert_eq!(font.as_slice()[1].name, "noto sans");
    assert!(warnings.iter().all(|w| !w.message.contains("'font'")));
}

#[test]
fn eval_set_text_font_dict_rejeitado_passo_132b() {
    let src = r#"#set text(font: (name: "X", covers: "latin-in-cjk"))"#;
    let result = eval_full(src);
    assert!(result.is_err());
    let errs = result.unwrap_err();
    assert!(errs.iter().any(|e|
        e.message.contains("dict form of font not yet supported")
    ));
}

#[test]
fn eval_set_text_font_array_vazio_rejeitado_passo_132b() {
    let src = r#"#set text(font: ())"#;
    let result = eval_full(src);
    assert!(result.is_err());
    let errs = result.unwrap_err();
    assert!(errs.iter().any(|e|
        e.message.contains("must not be empty")
    ));
}

#[test]
fn eval_set_text_font_array_com_nao_string_rejeitado_passo_132b() {
    let src = r#"#set text(font: ("Arial", 42))"#;
    let result = eval_full(src);
    assert!(result.is_err());
    let errs = result.unwrap_err();
    assert!(errs.iter().any(|e|
        e.message.contains("only strings")
    ));
}
```

### 132B.H — Migrar canary L3 (3 testes)

**Ficheiro**: `03_infra/src/integration_tests.rs`.

**H.1** — `debt49_set_text_font_emite_warning:2180` → renomear
para `debt49_set_text_hyphenate_emite_warning_passo_132b`:
- Input: `#set text(font: "Arial")` →
  `#set text(hyphenate: true)`.
- Assertion: `'font'` → `'hyphenate'`.

**H.2** —
`debt49_set_text_multiplas_propriedades_desconhecidas:2229`:
- Input trio: `font:"A", alignment, stroke` →
  `hyphenate: true, alignment, stroke`.
- Assertion dos 3 warnings adaptada.

**H.3** — `debt49_dedup_warnings_identicos:2285+2295`:
- Input: 2× `font:"A"` → 2× `hyphenate: true`.
- Assertion actualizada.

### 132B.I — Migrar canary L4 (2 testes)

**Ficheiro**: confirmado em 132B.A.5 (esperado
`04_wiring/src/...` ou similar).

**I.1** — `cli_sucesso_com_warning:65`:
- Input: `#set text(font: "X")` → `#set text(hyphenate: true)`.
- Warning esperado: `'hyphenate'` em vez de `'font'`.

**I.2** — `disciplina_warnings_antes_de_errors:591`:
- Mesmo ajuste.

### 132B.J — ADR-0053 transição

Editar `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md`:

- `Status: PROPOSTO` → `Status: IMPLEMENTADO`.
- Adicionar `**Materializado em**: Passo 132B`.
- Actualizar secção "Consequências" / "Estado final" com
  números reais:
  - Linhas de código em `font_list.rs`.
  - Número de unit tests.
  - Número de integration tests.
  - Canary migration (10 testes — 5 L1 + 3 L3 + 2 L4).

### 132B.K — Prompts L0 (se aplicável)

Se padrão L0 exige prompt por tipo:

- Criar `00_nucleo/prompts/entities/font-list.md` com
  specification de `FontList` + `FontFamily` + `Covers`.
- Actualizar `00_nucleo/prompts/entities/style_chain.md` com
  campo `font: Option<FontList>`.
- Correr `crystalline-lint --fix-hashes .`.

### 132B.L — Verificação

1. `cargo test -p typst-core` — L1:
   - Base: 838.
   - +11 unit tests em `font_list.rs`.
   - +5 integration tests novos em `rules/eval/tests.rs`.
   - -4 tests removidos (5 canaries consolidados em 1).
   - Ou: +0 tests se canaries são apenas renomeados e input
     alterado (5 canaries continuam, agora com hyphenate).
   - **Estimativa a**: 838 + 11 + 5 - 4 = **850** (consolidação).
   - **Estimativa b**: 838 + 11 + 5 = **854** (sem consolidação).
   - Registar no relatório qual estratégia foi usada.

2. `cargo test -p typst-infra` — L3: **186** (inalterado,
   apenas 3 testes migram input).

3. `cargo test -p typst-wiring` — L4: **21** (inalterado,
   2 testes migram input).

4. `cargo test --workspace` — total ≥ 1081 (consolidado) ou
   1085 (sem consolidação).

5. `crystalline-lint` zero violations.

6. Manual:

```bash
$ cat s.typ
#set text(font: "Arial")
Texto
$ typst s.typ -o s.pdf
exit=0, stderr: (vazio)

$ cat a.typ
#set text(font: ("Inria Serif", "Noto Sans"))
Texto
$ typst a.typ -o a.pdf
exit=0, stderr: (vazio)

$ cat d.typ
#set text(font: (name: "X", covers: "latin-in-cjk"))
Texto
$ typst d.typ -o d.pdf
d.typ:1:11: error: dict form of font not yet supported — use string or array of strings
exit=1

$ cat e.typ
#set text(font: ())
Texto
$ typst e.typ -o e.pdf
e.typ:1:11: error: font array must not be empty
exit=1

$ cat h.typ
#set text(hyphenate: true)
Texto
$ typst h.typ -o h.pdf       # canary novo
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

### 132B.M — Encerramento

Relatório em `typst-passo-132b-relatorio.md`:

- Confirmações 132B.A (tipos Value disponíveis, hyphenate
  livre, caminho L4).
- Diff por ficheiro e contagem de linhas.
- Decisão sobre consolidação de canary (uma vs cinco).
- Números finais de testes por camada.
- ADR-0053 transição confirmada.
- ADR-0038 quinta nota adicionada.
- Pool DEBT-49 L3 actualizado.
- **Estado DEBT-1**:
  - `font` capturado (2 de 3 forms).
  - Lista DEBT-1 canónica agora com todas as propriedades
    capturadas (ressalva: `leading` em contexto errado).
  - Próximos passos: 133 (par target), 134 (leading
    migration), 135 (fechar DEBT-1).
- Candidatos futuros:
  - Autorizar `regex` em L1 quando consumer de shaping chegar.
  - Implementar `Covers` (variantes concretas) com coverage
    filtering.
  - Integrar `FontList` com `FontBook` para lookup real.

---

## Critério de conclusão

1. Inventário 132B.A escrito (6 verificações).
2. ADR-0038 quinta nota adicionada.
3. `entities/font_list.rs` criado com 3 tipos + 11 unit tests.
4. `entities/mod.rs` expõe `font_list`.
5. `StyleDelta.font: Option<FontList>` adicionado.
6. Arm `"font"` aceita 2 forms (string, array) e rejeita
   dict + outros com Err hard + mensagens literais.
7. 5 canaries L1 migrados/consolidados (decisão documentada).
8. 3 testes L3 migrados.
9. 2 testes L4 migrados.
10. 5 novos integration tests em L1.
11. ADR-0053 transita `PROPOSTO` → `IMPLEMENTADO`.
12. L1 tests: **850** ou **854** (conforme consolidação).
13. L3: 186 (inalterado).
14. L4: 21 (inalterado).
15. `cargo test --workspace` passa (≥ 1081).
16. `crystalline-lint` zero violations.
17. Teste manual confirma 5 cenários.
18. Relatório 132B.M escrito.

---

## O que pode sair errado

- **`Value::Array` tem forma inesperada** (ex: `Arc<[Value]>`
  em vez de `Vec<Value>`): adaptar iteração no arm. Confirmar
  em 132B.A.2. Baixo risco.

- **`Value::Dict` não existe em L1 ainda**: arm do dict não é
  necessário — basta rejeitar em `_ =>`. Mas o teste
  `eval_set_text_font_dict_rejeitado_passo_132b` pode precisar
  de ajuste de mensagem (não é "dict form" mas "unsupported
  type").

- **Canary consolidação vs preservação**: se consolidar reduz
  5 testes para 1, pode haver perda de sinal. Alternativa:
  manter 5 testes distintos com a mesma assertion,
  diferenciados apenas por sufixo numérico. Decidir no
  relatório com base em peso relativo.

- **`eval_full` vs `eval_inline`** para testes de Err: harness
  pode ter inconsistência. O 131B já lidou com isto; replicar.

- **Pool DEBT-49 L3 fica com 2 propriedades desconhecidas
  após 132B**: `hyphenate/alignment/stroke`. Pool saudável
  para mais 1-2 passos. Candidato "substituir rotativo por
  positivos específicos" continua pendente.

- **Número de linhas em `font_list.rs` maior que estimado**:
  se passa de 200 linhas com tests, ainda é S. Se passa de
  400, registar como M no relatório mas completar.

- **Hash de prompt L0 muda**: `crystalline-lint --fix-hashes`
  cobre. Se não existem prompts para estes tipos, decisão
  no relatório.

---

## Notas operacionais

- **Primeiro tipo agregador em L1**. Até agora os tipos L1
  eram folhas (`FontWeight`, `Length`, `Lang`). `FontList`
  contém `FontFamily` que contém `Covers`. Estrutura aninhada
  mas simples (2 níveis).

- **`Covers` inabitado é truque elegante**: forma estrutural
  presente, zero custo de código activo, compatibilidade
  forward garantida (adicionar variantes é additive).

- **Migração de canary é risco concentrado**: 10 testes em 3
  camadas. Se algum é esquecido, o mecanismo de warning fica
  sem cobertura. O inventário 132A enumerou todos — seguir
  literalmente.

- **ADR-0033 com divergência documentada** não é violação —
  é decisão consciente registada. O ADR-0053 documenta o
  trade-off e o caminho de resolução (autorizar regex quando
  consumer precisar). DEBT-1 pode fechar com esta divergência
  activa porque a decisão está capturada numa ADR dedicada,
  não escondida como dívida.

- **Após 132B, DEBT-1 está a um passo estrutural de fechar**
  (132B + 133 + 134 + 135). Momentum acumulado é o maior de
  sempre nesta série — 6 passos consecutivos numa direcção
  consistente (126, 127, 128, 129, 130+131, 132).

- **Candidato `eval_with_warnings` continua pendente**.
  Testes em 132B.G repetem pattern de `eval_inline`/`eval_full`.
  Refactor vale cada vez mais. Priorizar após 135 (fecho
  DEBT-1).
