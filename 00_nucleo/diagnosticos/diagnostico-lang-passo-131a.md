# Diagnóstico de `Lang` — Passo 131A

**Data**: 2026-04-24
**ADR alvo**: **ADR-0052** — "Lang como tipo semântico em L1"
(criado em 131A.4).
**Motivação**: Passo 130 capturou `lang` como `EcoString` raw,
violando ADR-0033 (divergência semântica activa:
`"xx-invalid"` passa silencioso no cristalino, erra no vanilla).
Este diagnóstico cumpre ADR-0034 antes de materializar tipo
`Lang` em L1 e obter paridade.

---

## 1. Localização no vanilla

- **Ficheiro principal**: `lab/typst-original/crates/typst-library/src/text/lang.rs`
- **Linhas-chave**:
  - `154-156`: declaração `pub struct Lang([u8; 3], u8)`.
  - `159-475`: **≈260 constantes `pub const X: Self`** (ABKHAZIAN…ZULU).
  - `477-480`: `pub fn as_str(&self) -> &str`.
  - `482-498`: `pub fn dir(self) -> Dir` — directionality.
  - `501-516`: `impl FromStr for Lang`.
  - `518-537`: `cast!` macro (Typst `FromValue` equivalente).

- **Outros sítios relevantes**:
  - `text/mod.rs:440`: `pub lang: Lang` em `TextElem`.
  - `text/item.rs:23`: `pub lang: Lang` em `TextItem` (layout).
  - `text/lang.rs:155`: `Locale { lang: Lang, region: Option<Region> }`
    — composto; `Region` vive no mesmo ficheiro (fora do escopo
    deste passo).

## 2. Campos / variantes

**Declaração exacta**:
```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);
```

- **Forma interna**: tuple newtype com `[u8; 3]` (bytes ASCII
  padded com `b' '` se 2-letter) + `u8` (length, 2 ou 3).
- **Ex**: `ENGLISH = Self(*b"en ", 2)` — "en" + space.
- **Ex**: `FILIPINO = Self(*b"fil", 3)` — 3-letter, no padding.
- **Derives**: Debug, Copy, Clone, Eq, PartialEq, Ord,
  PartialOrd, Hash. **Copy tem significado**: cost-free pass
  by value.

## 3. Operadores / métodos

### Métodos públicos

```rust
pub fn as_str(&self) -> &str
// Reconstroi slice ASCII a partir de self.0[..self.1].

pub fn dir(self) -> Dir
// Retorna Dir::RTL para árabe/hebraico/persa/etc.; Dir::LTR default.
```

### `impl FromStr for Lang`

```rust
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
```

**Regras**:
- **Comprimento**: 2 ou 3 caracteres.
- **Charset**: ASCII only (7-bit).
- **Case**: normalizado para lowercase.
- **Mensagem de erro** (literal): `"expected two or three letter language code (ISO 639-1/2/3)"`.

### `cast!` macro

```rust
cast! {
    Lang,
    self => self.as_str().into_value(),
    string: EcoString => {
        let result = Self::from_str(&string);
        if result.is_err()
            && let Some((lang, region)) = string.split_once('-')
                && Lang::from_str(lang).is_ok()
                && Region::from_str(region).is_ok() {
                    return result.hint(eco_format!(
                        "you should leave only \"{}\" in the `lang` parameter \
                         and specify \"{}\" in the `region` parameter",
                        lang, region,
                    ));
                }
        result?
    }
}
```

**Nuance**: se user passa `"en-GB"` (composto), cast detecta
que `"en"` parseia e `"GB"` é region válida, e emite hint
"put 'GB' in region parameter". Sempre erro hard; só a mensagem
ganha hint.

### Display / Debug / PartialEq / Eq / Hash

Derivados. Comparação por bytes + length.

## 4. Dependências

### Crates externas usadas em `lang.rs` vanilla

- `ecow::{EcoString, eco_format}` — string tipo.
- `rustc_hash::FxHashMap` — `TRANSLATIONS_MAP` (não relevante
  para `Lang` básico).
- `std::str::FromStr` — trait.

### Tipos internos vanilla referenciados

- `Dir` (de `layout::Dir`) — enum `{ LTR, RTL, TTB, BTT }`.
  Usado em `Lang::dir(self) -> Dir`.
- `Region` (mesmo ficheiro, linhas 539-572) — tipo análogo.
- `WritingScript` (mesmo ficheiro, linhas 575+) — tipo análogo.
- `StyleChain`, `TextElem` — context para `Locale::get_in`.

### Crates autorizadas em L1 (cristalino)

`01_core/Cargo.toml`:
- `thiserror` (via ADR padrão).
- `rustc-hash` (ADR-0018).
- `ecow` (ADR-0024 — `EcoString` em `Value::Str`).

**Nenhuma crate nova necessária**. `std::str::FromStr` é stdlib.

## 5. Semântica

- **Representação canónica**: ASCII lowercase, 2 ou 3 bytes,
  padded com espaço se 2-letter.
- **Regras de validação**: ISO 639-1 (2-letter) ou ISO 639-2/3
  (3-letter). Aceita tudo que cumpra formato; não valida
  contra registo real de línguas (ex: `"zz"` é aceite mesmo
  não sendo língua real).
- **Case**: entrada case-insensitive; storage lowercase.
- **Casos edge**:
  - `""` (empty) → erro.
  - `"e"` (1-letter) → erro.
  - `"enx"` (3-letter) → aceite literalmente.
  - `"EN"` / `"En"` → aceite, normalizado para `"en"`.
  - `"en-GB"` → erro (contém hyphen; FromStr só aceita letters).
    Mas `cast!` macro de vanilla intercepta e emite hint.
  - `"日本"` → erro (não-ASCII).
  - `"en1"` / `"123"` → **aceite** literalmente (FromStr só
    verifica ASCII + length, não digit-vs-letter). Relaxed
    validation.

## 6. Mensagens de erro

- **Forma exacta vanilla**:
  `"expected two or three letter language code (ISO 639-1/2/3)"`.
- **Hint condicional**:
  `"you should leave only \"{}\" in the `lang` parameter and specify \"{}\" in the `region` parameter"`.
- **Span**: no cast do valor de `FromValue` — span do argument.
  No cristalino usamos span do `named.expr()`.

## 7. Divergências propostas para L1

### Forma interna: réplica exacta vanilla

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);
```

**Razão**: Copy free-pass; zero alloc; fiel ao vanilla (ADR-0033).
Zero crates externas novas.

### Validação: FromStr idêntico

```rust
impl std::str::FromStr for Lang {
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
```

### Constantes: **mínimas** — apenas `ENGLISH`

```rust
impl Lang {
    pub const ENGLISH: Self = Self(*b"en ", 2);
}
```

**Razão**: vanilla tem ~260 constantes. Cristalino não tem
consumer que precise delas hoje. Adicionar `ENGLISH` basta
como simetria (possível default futuro). Restantes **on-demand**
quando consumer (shaping / hyphenation / translations) precisar.
Evita 260 linhas de constantes sem uso.

### API mínima

```rust
impl Lang {
    pub const ENGLISH: Self;
    pub fn as_str(&self) -> &str;
    // from_str via FromStr trait.
}
```

**Não replicar** (por agora):
- `dir()` — requer tipo `Dir`, não existe em L1; propriedade
  `text.dir` também não capturada. Adicionar quando
  consumer/propriedade chegar.
- 259 constantes restantes — on-demand.

### Mensagem de erro proposta

Réplica literal vanilla: `"expected two or three letter language code (ISO 639-1/2/3)"`.

**Sem hint "put region in region parameter"** no 131B — esse
requer conhecer que `Region` existe. Cristalino ainda não tem
`Region`. Para `"en-GB"` cristalino **erra hard** com a
mensagem base, sem hint adicional. Futuro passo quando `Region`
for materializado pode alinhar.

### Localização em L1

**`01_core/src/entities/lang.rs`** — novo ficheiro dedicado.

**Razão (ADR-0037 coesão por domínio)**:
- Vanilla isola `Lang` + `Region` + `WritingScript` +
  `TRANSLATIONS` em `text/lang.rs` (domínio i18n).
- Cristalino `font_book.rs` é domínio **font catalog** —
  `FontWeight`, `FontStretch` são variantes tipográficas
  de *fonte*, não de *língua*. Misturar `Lang` lá confunde
  domínio.
- Futuras adições (`Region`, `WritingScript`) vivem no mesmo
  ficheiro `lang.rs`.

---

## Itens adicionais

### Impacto em call-sites

1. **`01_core/src/entities/style_chain.rs`**:
   - `use ecow::EcoString;` — remove (já não usado para lang)
     OU mantém se outros campos futuros precisarem. Actualmente
     só `lang` usa; **remove**.
   - `use crate::entities::lang::Lang;` — adicionar.
   - `StyleDelta.lang: Option<EcoString>` → `Option<Lang>`.
   - `StyleDelta::empty()` — `lang: None` continua válido.
   - Comentário do campo actualizado.

2. **`01_core/src/rules/eval/rules.rs`**:
   - `use crate::entities::lang::Lang;` — adicionar.
   - `use std::str::FromStr;` — adicionar se não existe.
   - Arm `"lang"` adaptado:
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
   - **Mudança de sub-função para erro-hard**: actualmente
     `eval_set_rule` **não** emite `Err` de dentro do loop de
     args; só para targets desconhecidos. Introdução de `Err`
     aqui quebra o pattern "warnings só" do set rule. Aceite
     em ADR-0033 (paridade vanilla obriga).

3. **`01_core/src/entities/mod.rs`**:
   - `pub mod lang;` — expor novo módulo.

### Plano de teste para 131B

#### Unit tests em `01_core/src/entities/lang.rs`

- `lang_from_str_iso_639_1_aceita_2_letras`: "pt", "en", "de".
- `lang_from_str_iso_639_3_aceita_3_letras`: "por", "fil".
- `lang_from_str_normaliza_case`: "PT" → "pt"; "En" → "en".
- `lang_from_str_vazio_devolve_erro`: "" → Err com mensagem.
- `lang_from_str_1_letra_devolve_erro`: "e" → Err.
- `lang_from_str_4_letras_devolve_erro`: "engl" → Err.
- `lang_from_str_nao_ascii_devolve_erro`: "日本" → Err.
- `lang_from_str_com_hyphen_devolve_erro`: "en-GB" → Err.
- `lang_as_str_preserva_canonico`: `ENGLISH.as_str() == "en"`.
- `lang_as_str_trim_padding`: `FILIPINO` (3-letter) vs
  `ENGLISH` (2-letter) — ambos sem espaço trailing.
- `lang_english_constante`: `Lang::ENGLISH.as_str() == "en"`.

#### Integration tests em `01_core/src/rules/eval/tests.rs`

Adaptar:
- `eval_set_text_lang_passo_130` — input `"pt"` continua OK.
  Renomeado como `eval_set_text_lang_valido_passo_131b` e
  pattern ajustado.
- `eval_set_text_lang_bcp47_composto_passo_130` — `"en-GB"`
  deixa de ser silent; teste **invertido** para assertar erro
  hard. Renomeado como `eval_set_text_lang_composto_emite_erro_passo_131b`.
- `eval_set_text_font_canary_passo_130` — inalterado.

Adicionar:
- `eval_set_text_lang_invalido_emite_erro_hard_passo_131b` —
  `"xx-invalid-nonsense"` → erro no compile, exit 1 (em
  integration L3 pode precisar adaptar `do_eval_with_sink`
  para distinguir Err vs warning).

### L3 adaptar (se necessário)

- `debt49_set_text_alignment_emite_warning`: `alignment`
  continua unknown, sem rotação.
- `debt49_set_text_multiplas_propriedades_desconhecidas`:
  trio `font/alignment/stroke` — inalterado.

### ADR-0038 — nota quarta?

**Sim, recomendada**. Passos 126/127/129 adicionaram 3 notas
sobre *variações do pattern DEBT-1 XS*. Passo 131B **muda
semântica** de um campo (de raw para tipado com validação
erro-hard) — **não é variante do pattern XS**, é refactor.

Recomendação: ADR-0038 ganha nota curta: *"Passo 131B alterou
`StyleDelta.lang` de `Option<EcoString>` para `Option<Lang>`.
Pattern DEBT-1 XS não se aplica — ver ADR-0052 para paridade
vanilla dedicada."* Evita confusão em futuros leitores sobre
porque `lang` foi tratado diferente dos outros.

---

## Resumo executivo

**Decisões-chave**:
1. Réplica vanilla exacta: `struct Lang([u8; 3], u8)`.
2. `impl FromStr` idêntico — mesma mensagem de erro literal.
3. Apenas `Lang::ENGLISH` como constante inicial.
4. Ficheiro dedicado `entities/lang.rs`.
5. Arm `"lang"` emite **Err hard** em parse-failure.
6. Zero crates novas. Paridade ADR-0033 integral.

**Estimativa 131B**: **S** — 1 ficheiro novo (`lang.rs` ~50
linhas + tests), 2 ficheiros modificados (`style_chain.rs` +
`rules.rs`), 3-4 testes L1 adaptados, 8-10 tests L1 novos.
Total: 1-2h de implementação + revisão.

**Bloqueios**: nenhum. Todas as deps disponíveis em L1.
`Dir` e hint de region deferidos (fora de escopo).
