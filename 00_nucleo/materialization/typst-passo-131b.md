# Passo 131B — Materialização de `Lang` em L1

**Série**: 131B (passo **S** em L1; segundo de dois sub-passos
para materializar `Lang`).
**Precondição**: Passo 131A encerrado; diagnóstico
`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md` aprovado;
ADR-0052 em `PROPOSTO` em
`00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md`; 1057 total
tests; zero violations; 52 ADRs (51 activas + ADR-0052
`PROPOSTO`); 11 DEBTs abertos.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — este passo é o cumprimento.
- **ADR-0034** — diagnóstico cumprido em 131A.
- **ADR-0037** (coesão por domínio) — ficheiro dedicado
  `entities/lang.rs`.
- **ADR-0038** — ganha quarta nota (recomendação do diagnóstico).
- **ADR-0052** — transita `PROPOSTO` → `IMPLEMENTADO` ao fim do
  passo.

**Natureza**: passo L1. Código novo + migração + inversão de
teste. Pattern DEBT-1 XS **não se aplica** — refactor pontual
por razões de paridade.

---

## Contexto

Passo 131A produziu diagnóstico e ADR proposta. Decisões
aprovadas entrando neste passo:

1. **Forma interna**: `struct Lang([u8; 3], u8)` — réplica
   vanilla exacta, Copy, zero alocação.
2. **Validação**: `impl FromStr` idêntica ao vanilla. Mensagem
   literal: `"expected two or three letter language code (ISO 639-1/2/3)"`.
3. **Constantes**: apenas `Lang::ENGLISH`. Restantes on-demand.
4. **Localização**: `01_core/src/entities/lang.rs` (novo).
5. **Erro hard em inválido**: arm `"lang"` emite `Err` imediato,
   aborta avaliação da set rule — não coleta, não continua.
6. **Sem default**: `StyleDelta.lang` mantém `None` = "herdado".
   `Lang::ENGLISH` é constante disponível, não default activo.
7. **API mínima**: `Lang::ENGLISH`, `as_str()`, `from_str` via
   trait. **Não** incluir `dir()` (requer `Dir`, fora de escopo).

---

## Objectivo

Ao fim do passo:

1. Tipo `Lang` materializado em `01_core/src/entities/lang.rs`.
2. `StyleDelta.lang: Option<EcoString>` migrado para
   `Option<Lang>`.
3. Arm `"lang"` em `eval_set_text` valida via `Lang::from_str` e
   emite `Err` hard em inválido.
4. Testes L1 do Passo 130 adaptados (1 renomeado, 1 invertido,
   1 inalterado).
5. Novos testes unitários em `lang.rs` (11 casos do diagnóstico).
6. Novo integration test para erro hard.
7. ADR-0052 transita `PROPOSTO` → `IMPLEMENTADO`.
8. ADR-0038 ganha quarta nota.
9. `cargo test --workspace` passa. `crystalline-lint` zero.

Este passo **não**:

- Adiciona outros tipos (`Region`, `Script`, `Dir`).
- Adiciona outras constantes além de `ENGLISH`.
- Implementa `Lang::dir()`.
- Implementa hint "put region in region parameter" do vanilla.
- Adiciona default à `StyleDelta.lang`.
- Fecha DEBT-1.

---

## Escopo

**Dentro**:
- `01_core/src/entities/lang.rs` (**NOVO**).
- `01_core/src/entities/mod.rs` — expor `pub mod lang;`.
- `01_core/src/entities/style_chain.rs` — campo migrado,
  imports ajustados.
- `01_core/src/rules/eval/rules.rs` — arm `"lang"` validador.
- `01_core/src/rules/eval/tests.rs` — 3 testes adaptados +
  1 novo integration test.
- `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md` —
  status `PROPOSTO` → `IMPLEMENTADO`.
- `00_nucleo/adr/typst-adr-0038-*.md` — quarta nota.
- `00_nucleo/prompts/entities/style_chain.md` + hash (se existe).
- `00_nucleo/prompts/entities/lang.md` + hash (**NOVO** se L0
  exige prompt por tipo).

**Fora**:
- L2, L3 (excepto se integration test L3 exige adaptação).
- L4.
- `Region`, `Script`, `Dir`, outras constantes.
- Consumer em layout (shaping, hyphenation).

---

## Sub-passos

### 131B.A — Inventário confirmatório

Leitura rápida. Sem edição.

**A.1** — Confirmar que o diagnóstico 131A é fiel ao estado
actual:
- `grep -n "lang: Option<EcoString>" 01_core/src/entities/style_chain.rs`
  — confirmar campo existe com tipo actual.
- `grep -n "\"lang\"" 01_core/src/rules/eval/rules.rs` —
  confirmar arm actual.
- `ls 01_core/src/entities/` — confirmar que `lang.rs` não
  existe.

**A.2** — Confirmar pattern de `Err` em `eval_set_rule`:
- Ler função `eval_set_rule` (ou equivalente) para perceber:
  - Como é o loop de argumentos?
  - Qual é o tipo de retorno? (`Result<_, Vec<SourceDiagnostic>>`?)
  - Há `return Err(...)` noutro sítio que serve de template?
- Se **não há precedente** de `Err` dentro do arm loop,
  **parar e reportar** — pode exigir adaptação da assinatura da
  função, o que excede S.

**A.3** — Confirmar span disponível no arm:
- `named.expr().span()` é o span do valor. É o que queremos
  para o erro.
- Se `named` não está em escopo no match arm (nome diferente),
  registar variável real.

**A.4** — Confirmar acesso a `SourceDiagnostic::error`:
- `grep -n "SourceDiagnostic::error\|SourceDiagnostic::warn" 01_core/src/`
  — confirmar pattern de construção.
- Imports necessários em `rules.rs`.

**A.5** — Confirmar número actual de testes:
- L1: 826 (conforme relatório 130).
- L3: 186 (2 rotados no 130, inalterados neste passo).

**Gate 131B.A**:

- Se A.2 revela que `Err` no meio do arm loop é novo pattern
  sem precedente: **parar, reportar, aguardar decisão**. Pode
  exigir sub-passo adicional ou adaptação arquitectural.
- Se A.1 revela que o estado do 130 não está como o
  diagnóstico assume: **parar, reportar**. Base do passo fica
  em dúvida.
- Outros casos: prosseguir para 131B.B.

### 131B.B — ADR-0038 nota quarta

Adicionar ao final de ADR-0038:

```markdown
### Nota Passo 131B — `lang` não segue pattern DEBT-1 XS

Campo `StyleDelta.lang` foi materializado como `Option<Lang>`
(tipo semântico L1) para obter paridade com vanilla. Arm
`"lang"` em `eval_set_text` valida e emite `Err` hard em
inválido.

Esta mudança **não é variante do pattern DEBT-1 XS**
(documentado nas notas Passos 126/127/129) — é refactor por
razões de paridade ADR-0033. Ver ADR-0052 para contexto
completo.

Futuras propriedades com necessidade análoga de paridade
semântica (validação, erro hard) seguem padrão do 131B:
diagnóstico obrigatório (ADR-0034) → ADR dedicada →
materialização. Não agregar no ADR-0038.
```

### 131B.C — Materializar `Lang`

**Ficheiro novo**: `01_core/src/entities/lang.rs`.

```rust
//! Tipo `Lang` — código ISO 639-1/2/3 para language tag.
//!
//! Réplica estrutural de `typst::text::lang::Lang` vanilla.
//! Forma interna: `[u8; 3]` + length (2 ou 3). Copy, zero
//! alocação.
//!
//! Ver ADR-0052 e diagnóstico
//! `00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`.

use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);

impl Lang {
    /// Inglês (`en`). Único constant incluído na materialização
    /// inicial; outras línguas adicionam-se on-demand quando
    /// consumer as exigir.
    pub const ENGLISH: Self = Self(*b"en ", 2);

    /// Devolve o código ISO como slice ASCII (sem padding).
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0[..self.1 as usize])
            .expect("Lang guarda apenas bytes ASCII válidos")
    }
}

impl FromStr for Lang {
    type Err = &'static str;

    fn from_str(iso: &str) -> Result<Self, Self::Err> {
        let len = iso.len();
        if matches!(len, 2..=3) && iso.is_ascii() {
            let mut bytes = [b' '; 3];
            bytes[..len].copy_from_slice(iso.as_bytes());
            bytes.make_ascii_lowercase();
            Ok(Self(bytes, len as u8))
        } else {
            Err("expected two or three letter language code (ISO 639-1/2/3)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_from_str_iso_639_1_aceita_2_letras_passo_131b() {
        assert!(Lang::from_str("pt").is_ok());
        assert!(Lang::from_str("en").is_ok());
        assert!(Lang::from_str("de").is_ok());
    }

    #[test]
    fn lang_from_str_iso_639_3_aceita_3_letras_passo_131b() {
        assert!(Lang::from_str("por").is_ok());
        assert!(Lang::from_str("fil").is_ok());
    }

    #[test]
    fn lang_from_str_normaliza_case_passo_131b() {
        assert_eq!(Lang::from_str("PT").unwrap().as_str(), "pt");
        assert_eq!(Lang::from_str("En").unwrap().as_str(), "en");
    }

    #[test]
    fn lang_from_str_vazio_devolve_erro_passo_131b() {
        let err = Lang::from_str("").unwrap_err();
        assert!(err.contains("two or three letter"));
    }

    #[test]
    fn lang_from_str_1_letra_devolve_erro_passo_131b() {
        assert!(Lang::from_str("e").is_err());
    }

    #[test]
    fn lang_from_str_4_letras_devolve_erro_passo_131b() {
        assert!(Lang::from_str("engl").is_err());
    }

    #[test]
    fn lang_from_str_nao_ascii_devolve_erro_passo_131b() {
        assert!(Lang::from_str("日本").is_err());
    }

    #[test]
    fn lang_from_str_com_hyphen_devolve_erro_passo_131b() {
        // "en-GB" tem hyphen; FromStr vanilla aceita apenas
        // letters + length 2-3. Hyphen faz length 5, rejeita.
        assert!(Lang::from_str("en-GB").is_err());
    }

    #[test]
    fn lang_as_str_preserva_canonico_passo_131b() {
        assert_eq!(Lang::ENGLISH.as_str(), "en");
    }

    #[test]
    fn lang_as_str_trim_padding_3_letter_passo_131b() {
        let fil = Lang::from_str("fil").unwrap();
        assert_eq!(fil.as_str(), "fil");
        // Sem espaço trailing — length=3 não padded.
    }

    #[test]
    fn lang_english_constante_passo_131b() {
        assert_eq!(Lang::ENGLISH.as_str(), "en");
        // Copy: pode ser usado sem clone.
        let copia = Lang::ENGLISH;
        assert_eq!(copia, Lang::ENGLISH);
    }
}
```

**Expor em `entities/mod.rs`**:

```rust
pub mod lang;
```

### 131B.D — Migrar `StyleDelta`

**Ficheiro**: `01_core/src/entities/style_chain.rs`.

Mudanças:

1. Remover `use ecow::EcoString;` se não é usado por outro campo.
   Confirmar com `grep EcoString` no ficheiro; só remover se
   `lang` era único uso.
2. Adicionar `use crate::entities::lang::Lang;`.
3. Campo:

```rust
// antes:
pub lang: Option<EcoString>,

// depois:
pub lang: Option<Lang>,
```

4. `StyleDelta::empty()` — `lang: None` continua válido. Zero
   mudança aí.

5. Actualizar comentário do campo:

```rust
/// Identificador de língua (ISO 639-1/2/3). `None` = herdado.
pub lang: Option<Lang>,
```

### 131B.E — Adaptar arm `"lang"` em `eval_set_text`

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

Imports a adicionar:

```rust
use crate::entities::lang::Lang;
use std::str::FromStr;
```

Arm actual (confirmar em 131B.A.1):

```rust
"lang" => {
    if let Value::Str(s) = val {
        delta.lang = Some(s);
    }
}
```

Arm novo:

```rust
"lang" => {
    if let Value::Str(s) = val {
        match Lang::from_str(&s) {
            Ok(lang) => delta.lang = Some(lang),
            Err(msg) => {
                return Err(vec![SourceDiagnostic::error(
                    named.expr().span(),
                    msg.to_string(),
                )]);
            }
        }
    }
}
```

**Notas**:
- `named.expr().span()` — span do valor do argumento. Confirmar
  nome da variável em 131B.A.3.
- `return Err(...)` aborta `eval_set_rule`. Outros args na mesma
  set rule não são processados — semântica decidida (erro
  imediato, não colecta).
- Se `Value::Str` não é `EcoString` directo mas wrapper
  (ex: `Str(EcoString)`), ajustar match — confirmar em 131B.A.

### 131B.F — Adaptar testes L1 existentes

**Ficheiro**: `01_core/src/rules/eval/tests.rs`.

**F.1** — `eval_set_text_lang_passo_130` → **renomear e adaptar**:

```rust
// antes:
#[test]
fn eval_set_text_lang_passo_130() {
    let src = r#"#set text(lang: "pt")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.lang, Some(EcoString::from("pt")));
    // ...
}

// depois:
#[test]
fn eval_set_text_lang_valido_passo_131b() {
    let src = r#"#set text(lang: "pt")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.lang, Some(Lang::from_str("pt").unwrap()));
    assert!(warnings.iter().all(|w| !w.message.contains("'lang'")));
}
```

**F.2** — `eval_set_text_lang_bcp47_composto_passo_130` →
**inverter**: deixa de assertar silent, passa a assertar erro
hard.

```rust
// antes (silent):
#[test]
fn eval_set_text_lang_bcp47_composto_passo_130() {
    let src = r#"#set text(lang: "en-GB")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.lang, Some(EcoString::from("en-GB")));
    assert!(warnings.iter().all(|w| !w.message.contains("'lang'")));
}

// depois (erro hard):
#[test]
fn eval_set_text_lang_composto_emite_erro_passo_131b() {
    let src = r#"#set text(lang: "en-GB")"#;
    // eval_inline pode precisar de adaptação se não devolve Err
    // visível; alternativa: eval_full que devolve Result.
    let result = eval_full(src);
    assert!(result.is_err());
    let errs = result.unwrap_err();
    assert!(errs.iter().any(|e| e.message.contains("two or three letter")));
}
```

**Nota**: se `eval_inline` hoje devolve `(delta, warnings)` mas
não expõe `Err`, pode ser necessário usar outra função do
harness de teste. Confirmar em 131B.A.2.

**F.3** — `eval_set_text_font_canary_passo_130` → **inalterado**
(renomear apenas se convenção exige):

```rust
#[test]
fn eval_set_text_font_canary_passo_131b() {
    // Canary DEBT-50 — sexta iteração.
    let src = "#set text(font: \"X\")\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'font'")));
}
```

### 131B.G — Novo integration test

```rust
#[test]
fn eval_set_text_lang_invalido_emite_erro_hard_passo_131b() {
    // Replica vanilla: valor não-ISO devolve erro com mensagem
    // literal.
    let src = r#"#set text(lang: "xxxx")"#;
    let result = eval_full(src);
    assert!(result.is_err());
    let errs = result.unwrap_err();
    assert!(errs.iter().any(|e|
        e.message.contains("expected two or three letter language code")
    ));
}
```

### 131B.H — ADR-0052 transição

Editar `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md`:

- `Status: PROPOSTO` → `Status: IMPLEMENTADO`.
- Adicionar linha `**Materializado em**: Passo 131B`.
- Actualizar secção "Consequências" com estado final (número de
  linhas de código, estimativa confirmada).

### 131B.I — Prompts L0 (se aplicável)

Se L0 exige prompt por tipo L1 materializado:

- Criar `00_nucleo/prompts/entities/lang.md` — especificação
  pura do tipo (sem código exemplo).
- Actualizar hash: `crystalline-lint --fix-hashes .`.
- Se `entities/style_chain.md` descreve `StyleDelta` campo a
  campo, adicionar secção `lang: Option<Lang>`.

### 131B.J — Verificação

1. `cargo test -p typst-core` — L1: 826 → **826 + 11 unit + 3
   integration - 0 removidos = 840**.
   - 11 unit tests em `lang.rs`.
   - 3 integration tests adaptados em `rules/eval/tests.rs`
     (1 renomeado, 1 invertido, 1 canary — +0 net).
   - 1 integration test novo (`xxxx` erro).
   - Total esperado: **840 L1** (+14).

2. `cargo test --workspace` — total ≥ 1071.

3. `crystalline-lint` zero violations.

4. Manual:

```bash
$ cat ok.typ
#set text(lang: "pt")
Olá
$ typst ok.typ -o ok.pdf
exit=0, stderr: (vazio)

$ cat case.typ
#set text(lang: "PT")
Olá
$ typst case.typ -o case.pdf
exit=0, stderr: (vazio)       # normalizado para "pt"

$ cat comp.typ
#set text(lang: "en-GB")
Hello
$ typst comp.typ -o comp.pdf
comp.typ:1:17: error: expected two or three letter language code (ISO 639-1/2/3)
exit=1

$ cat inv.typ
#set text(lang: "xxxx")
Texto
$ typst inv.typ -o inv.pdf
inv.typ:1:17: error: expected two or three letter language code (ISO 639-1/2/3)
exit=1

$ cat f.typ
#set text(font: "X")
Texto
$ typst f.typ -o f.pdf         # Canary preservado
f.typ:1:11: warning: text: propriedade 'font' ainda não suportada
exit=0
```

### 131B.K — Encerramento

Relatório em `typst-passo-131b-relatorio.md`:

- Confirmação 131B.A (pattern Err precedente? variável `named`?).
- Diff por ficheiro.
- Números finais (tests, linhas).
- Divergências face ao diagnóstico 131A (se as houver).
- **Mudança observável face ao 130**: `"en-GB"` deixa de ser
  silent e passa a erro — registar como breaking semantic
  change (sem utilizador real afectado; documentar).
- ADR-0052 transição confirmada.
- ADR-0038 quarta nota adicionada.
- Candidatos futuros:
  - Materializar `Region` (precondição para hint vanilla
    "put region in region parameter").
  - Materializar `Dir` + método `Lang::dir()`.
  - Expandir constantes `Lang::*` on-demand (e.g. PORTUGUESE,
    GERMAN, etc.) quando consumer chegar.

---

## Critério de conclusão

1. Inventário 131B.A escrito (confirmação das 5 verificações).
2. ADR-0038 quarta nota adicionada.
3. `entities/lang.rs` criado com tipo + constante + FromStr +
   11 unit tests.
4. `entities/mod.rs` expõe `pub mod lang;`.
5. `StyleDelta.lang: Option<EcoString>` → `Option<Lang>`.
6. Arm `"lang"` emite Err hard em inválido.
7. Testes do 130 adaptados (F.1 renomeado, F.2 invertido, F.3
   canary).
8. 1 novo integration test para `"xxxx"`.
9. ADR-0052 transita `PROPOSTO` → `IMPLEMENTADO`.
10. L1 tests: **840** (+14 vs 826).
11. `cargo test --workspace` passa (≥ 1071).
12. `crystalline-lint` zero violations.
13. Teste manual confirma 5 cenários.
14. Relatório 131B.K escrito.

---

## O que pode sair errado

- **`Err` no meio do arm loop não tem precedente**
  (detectado em 131B.A.2): sub-passo arquitectural adicional
  pode ser necessário. Parar antes de escrever código novo.
  Opções: (a) adaptar assinatura de `eval_set_rule` para
  suportar early Err; (b) sub-passo dedicado para refactor
  anterior.

- **`Value::Str` wrapper inesperado**: match arm precisa de
  ajuste. Se `Value::Str` tem variante com `EcoString` interno,
  o `&s` de `Lang::from_str(&s)` pode precisar ser
  `s.as_str()` ou similar.

- **`eval_inline` não expõe Err**: harness de teste actual
  devolve `(delta, warnings)` assumindo sucesso. F.2 e G
  precisam de harness alternativo. Confirmar em 131B.A e
  adaptar — pode exigir adicionar helper ou usar função
  existente do `rules/eval/tests.rs`.

- **Contagem de testes diverge do esperado**: estimei +14 L1.
  Se o real é diferente, registar no relatório e confirmar que
  cada teste listado no enunciado foi contemplado.

- **Prompt L0 obrigatório mas não claro onde vive**: se L0
  exige prompt por tipo e o padrão não está documentado,
  escolher caminho mínimo (`entities/lang.md` com specification
  curta) e registar decisão no relatório. Não bloquear.

- **ADR-0052 número já usado**: confirmar em 131A. Se colisão,
  ADR tem de ser renumerada e todas as referências actualizadas
  (relatório 131A, diagnóstico, este enunciado). Baixo risco
  porque 131A atribuiu.

---

## Notas operacionais

- Primeiro passo com **`Err` hard** em `eval_set_text`. Se sai
  limpo, estabelece precedente para futuras validações
  (propriedades com erro vanilla em inválido).

- **`Lang::ENGLISH` sem consumer** é investimento. Justifica-se
  como simetria com vanilla e como base para futuro default,
  sem custo material (1 linha).

- **Reminder: Copy type é importante**. `StyleDelta.lang:
  Option<Lang>` custa `size_of::<Option<Lang>>() = 5 bytes` em
  vez de `size_of::<Option<EcoString>>() = 16-24 bytes`.
  Melhoria marginal mas no sentido certo (ADR-0030).

- **Pattern DEBT-1 XS foi 5× seguido (126-130). Este é
  refactor**. Relatório 131B.K deve ser claro sobre a
  distinção para evitar que passos futuros "herdem" expectativa
  errada de cadência XS.

- **Fecho de DEBT-1 continua viável em 4 passos após este**:
  132 font + canary, 133 par target, 134 leading migration,
  135 DEBT.md. Sem ramificações adicionais planeadas.

- **Candidato "extract helper `eval_with_warnings`" (Passo 127)
  ganha urgência**: o harness de teste agora tem pelo menos
  duas modalidades (com/sem Err). Um helper uniforme resolve.
  Registar candidato com prioridade elevada após fecho de
  DEBT-1.
