# Passo 414 — Resolver DEBT-52: `text.font` dict (gap 8) (M)

**Tipo**: Materialização (L1 stdlib + eval + layout; zero tipo novo no Value enum; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla), ADR-0054bis condicional (font dict scope-out).  
**Sonda fonte**: DEBT-52 — `text.font` dict agora desbloqueado por `Value::Regex` (P402) + `Selector::Regex` (P393); gap 8 mais antigo do projeto.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

DEBT-52 foi registrado como gap 8 no Inventário 148: `text.font` aceita string, array de strings, mas **não aceita dict** no cristalino. No vanilla:

```typ
#set text(font: (
  "Linux Libertine",
  "Times New Roman",
))
```

O dict de fonte no vanilla permite especificar:
- `family`: string (nome da família)
- `variant`: string (e.g., "regular", "bold", "italic")
- `weight`: int/string
- `style`: string
- `stretch`: string
- `fallback`: bool

Este passo resolve DEBT-52, desbloqueando tipografia real com fontes configuráveis via dict — pré-requisito para muitos documentos vanilla.

---

## 2. Decisão de engenharia

O cristalino já tem:
- `FontList` (P141, P146) — lista de famílias para fallback
- `FontBook::select` (P140B) — seleção de fonte por nome
- `Value::Regex` (P402) — para matching de nomes
- `Style::Font(FontList)` (P292) — aplicação de fonte via Style chain

O que falta é **a ponte entre dict e FontList**: o stdlib `text.font` precisa aceitar `Value::Dict` e converter para `FontList` interno.

No cristalino:
- `native_text_font` (ou equivalente em set rule) aceita `Value::Str`, `Value::Array`, e agora `Value::Dict`.
- Dict é convertido para `FontList` com `FontFamily { name, variant, weight, style }`.
- Se `fallback: true` (ou ausente), a fonte entra na lista de fallback; se `false`, é fonte única.
- Regex em `family` é suportado via `Value::Regex` (P402) — matching no `FontBook`.

**Decisão de não criar `Value::FontDict`**: reutiliza `Value::Dict` existente. O parsing do dict acontece no stdlib (eval-time), não no Value enum.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Dict existe
grep -n "Value::Dict" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que FontList existe e tem FontFamily
grep -n "pub struct FontList\|pub struct FontFamily" entities/font_list.rs
# Esperado: ambos existem; FontFamily tem name: String

# 3. Confirmar que FontBook::select existe
grep -n "pub fn select" entities/font_book.rs
# Esperado: método de seleção por nome

# 4. Confirmar que Value::Regex existe (desbloqueio DEBT-52)
grep -n "Value::Regex" entities/value.rs
# Esperado: variant existe (P402)

# 5. Confirmar que text.font stdlib existe e aceita Str/Array
grep -n "native_text_font\|text.*font" rules/stdlib/text.rs | head -10
# Esperado: função existe; match em Str/Array

# 6. Verificar que Dict ainda NÃO é aceito em text.font
grep -A 20 "native_text_font" rules/stdlib/text.rs | grep -i "dict"
# Esperado: zero hits (confirma ausência)
```

**Critério de passagem**: 
- (1)-(5) todos OK (infra existe).
- (6) Dict ausente em text.font (confirma DEBT-52).

Se (2) falhar (FontList não existe), **parar** — requer P140B/P141/P146 primeiro. Se (4) falhar (Regex não existe), **parar** — DEBT-52 ainda bloqueado. Se (6) já tiver Dict, **parar** — DEBT-52 já resolvido.

Documentar resultado no commit: `Sonda P414: FontList OK; FontBook::select OK; Regex OK; text.font stdlib OK; Dict ainda ausente em text.font → DEBT-52 válido.`

### A.1 — Prompt L0 `text-font-dict.md`

Novo em `00_nucleo/prompts/rules/stdlib/text-font-dict.md` (ou integrar em `text.md` existente):

- **Paridade**: `text.font: (family: "Linux Libertine", weight: "bold")` ≡ `FontList` com `FontFamily { name: "Linux Libertine", variant: "bold" }`.
- **Substrato**: extensão de `native_text_font` (ou set rule handler) para aceitar `Value::Dict`.
- **Sem tipo novo**: reutiliza `Value::Dict`, `FontList`, `FontFamily`.
- **Campos do dict**: `family` (Str/Regex), `variant` (Str, opcional), `weight` (Int/Str, opcional), `style` (Str, opcional), `fallback` (Bool, default true).
- **Conversão**: Dict → `FontFamily` → `FontList`.
- **Regex**: `family: regex("Lin.*")` → `FontBook::select` com regex matching (P402).
- **Erros**: campo desconhecido → erro eval; tipo errado → erro eval.
- **Testes**: 
  - Dict básico: 3 casos (family str, family regex, variant+weight).
  - Dict com fallback true/false: 2 casos.
  - Dict inválido: 2 casos (campo desconhecido, tipo errado).
  - Array de dicts: 2 casos (fallback chain).
  - Total mínimo: ~15 testes.

### A.2 — CHECKPOINT

Parar. Apresentar `text-font-dict.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Localizar `native_text_font`** em `rules/stdlib/text.rs` (ou equivalente set rule handler).
2. **Adicionar ramo `Value::Dict(d)`** ao match de argumento:
   ```rust
   Value::Dict(d) => {
       let family = extract_font_family_from_dict(d)?;
       Ok(Value::FontList(FontList::from_family(family)))
   }
   ```
3. **Implementar `extract_font_family_from_dict`**:
   - Extrair `family` (Str ou Regex) → `String` ou `regex::Regex`.
   - Extrair `variant` (Str, opcional) → `Option<String>`.
   - Extrair `weight` (Int ou Str, opcional) → `Option<String>`.
   - Extrair `style` (Str, opcional) → `Option<String>`.
   - Extrair `fallback` (Bool, default true) → `bool`.
   - Construir `FontFamily { name, variant, weight, style }`.
   - Se `fallback: false`, retornar `FontList` com única fonte (sem fallback).
4. **Integrar com `FontBook::select`**:
   - Se `family` é `String`, usar `FontBook::select(&family)`.
   - Se `family` é `Regex`, iterar fontes disponíveis e fazer `regex.is_match(font_name)`.
5. **Registar em `make_stdlib`** se for nova função; se for extensão de existente, atualizar.
6. **Testes** em `rules/eval/tests.rs` ou `rules/stdlib/tests.rs`:
   - `text_font_dict_basic` — `text(font: (family: "Arial"))` → `FontList` correto.
   - `text_font_dict_regex` — `text(font: (family: regex("Ar.*")))` → match regex.
   - `text_font_dict_variant_weight` — `text(font: (family: "Arial", variant: "bold", weight: 700))`.
   - `text_font_dict_fallback_false` — `text(font: (family: "Arial", fallback: false))` → single font.
   - `text_font_dict_unknown_field` — erro eval.
   - `text_font_dict_array_of_dicts` — `text(font: ((family: "A"), (family: "B")))` → fallback chain.
7. **Linhagem**: `@prompt` aponta para `text-font-dict.md`; `@prompt-hash` via `--fix-hashes`.
8. **Validação**:
   - `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas extensão de `native_text_font` + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar `Value::FontDict` — reutiliza `Value::Dict`.
- **Não** implementar `stretch` (condensado/estendido) — não existe no vanilla baseline; scope-out futuro.
- **Não** implementar `features` (OpenType features) — requer shaping (DEBT-53).
- **Não** tocar em `FontBook` além de `select` — não requer mudança no FontBook.
- **Não** resolver DEBT-53 (shaping) — este passo é DEBT-52 apenas.
- **Não** criar função stdlib separada (`font_dict`) — integra em `text.font` existente.

---

## 6. Critérios de aceitação

1. `text(font: (family: "Arial"))` aceita `Value::Dict` e produz `FontList` correto.
2. `text(font: (family: regex("Ar.*")))` faz matching regex via `FontBook`.
3. `text(font: (family: "Arial", variant: "bold", weight: 700))` parseia campos opcionais.
4. `text(font: (family: "Arial", fallback: false))` produz fonte única (sem fallback).
5. Array de dicts funciona como fallback chain: `text(font: ((family: "A"), (family: "B")))`.
6. Campo desconhecido ou tipo errado → erro eval.
7. Zero tipo novo no Value enum; zero I/O.
8. Testes verdes (≥ 15 novos); lint zero; hashes propagados.
9. L0 salvo e hashado antes do código; sonda A.0 documentada no commit.
10. DEBT-52 fechado no inventário; gap 8 removido.

---

## 7. O que pode sair errado

- **`FontList` não tem `FontFamily` com variant/weight/style.** Mitigação: sonda A.0 detecta; se `FontFamily` for apenas `String`, adicionar campos opcionais (refino S).
- **`FontBook::select` não suporta regex.** Mitigação: implementar iterador de fontes + regex match inline; ou adicionar `select_regex` em `FontBook` (refino S).
- **`native_text_font` é set rule, não função stdlib.** Mitigação: sonda A.0 detecta; se for set rule handler, adicionar ramo Dict no handler de set rule (mesmo padrão, local diferente).
- **Tentação de resolver DEBT-53 junto.** Mitigação: scope-out claro; shaping é XL separado.

---

## 8. Referências

- `entities/font_list.rs` (P141, P146) — `FontList` e `FontFamily`.
- `entities/font_book.rs` (P140B) — `FontBook::select`.
- `entities/value.rs` — variants `Value::Dict`, `Value::Regex`.
- `rules/stdlib/text.rs` — `native_text_font` (infra a sondar em A.0).
- P402 — `Value::Regex` (desbloqueio DEBT-52).
- P393 — `Selector::Regex` (regex matching infra).
- ADR-0054bis condicional — scope-out font dict até Regex existir.
- ADR-0107 — paridade linguagem (forma do dict).
- ADR-0108 — medir-antes-de-decidir (sonda A.0).

---

## 9. Nota sobre o Tekt

Este passo é **M** porque requer: (a) parsing de Dict com múltiplos campos tipados, (b) integração com FontBook, (c) regex matching, (d) fallback chain. O risco principal é a estrutura de `FontList`/`FontFamily` — se já tiver os campos necessários, o passo é parsing+conversão; se não, cresce para M+ (refino de FontList).

A sonda A.0 determina o tamanho real antes do código. Se `FontFamily` já tem `variant`/`weight`/`style`, o passo é ~80% parsing; se não, ~50% parsing + 50% refino de tipo.

DEBT-52 é o **gap 8 mais antigo** do projeto. Fechá-lo é marco significativo para tipografia real.
