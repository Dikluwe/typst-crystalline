# Passo 411 — Field Access `Version`: `.major`, `.minor`, `.patch`, `.pre`, `.build` (S)

**Tipo**: Materialização (L1 eval; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Análise P410 — `eval_field_access` em `rules/eval/bindings.rs` só trata `Value::Dict` e `Value::Content`; não há ramo para `Value::Version`.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P401 modelou `Version` em L1; P403 ativou o constructor `version("1.2.3")`. O tipo existe e participa em comparações (P406), mas **não tem field access** — `version("1.2.3").major` falha com "field access não suportado em version".

No vanilla:
```typ
#version("1.2.3-alpha+build.1").major  // 1
#version("1.2.3-alpha+build.1").minor  // 2
#version("1.2.3-alpha+build.1").patch  // 3
#version("1.2.3-alpha+build.1").pre    // ("alpha",)
#version("1.2.3-alpha+build.1").build  // ("build", "1")
```

Este passo adiciona o ramo `Value::Version` em `eval_field_access` para os 5 campos acima.

| Campo | Tipo em L1 | Tipo retornado em eval |
|-------|------------|------------------------|
| `.major` | `u64` | `Value::Int` |
| `.minor` | `u64` | `Value::Int` |
| `.patch` | `u64` | `Value::Int` |
| `.pre` | `Vec<EcoString>` | `Value::Array(Str)` |
| `.build` | `Vec<EcoString>` | `Value::Array(Str)` |

---

## 2. Decisão de engenharia

Field access em L1 é **lookup por nome de campo** no eval de `Expr::FieldAccess`. No cristalino:

- `eval_field_access` (em `rules/eval/bindings.rs`) faz match no `target` e delega para o tipo.
- Atualmente só trata `Value::Dict` e `Value::Content`.
- Este passo adiciona `Value::Version(v)` com match no `field` string.

Não criar método genérico `get_field` em `Version` — o vanilla não tem; o field access é resolvido no eval. Não tocar em `entities/version.rs` — os campos já são públicos (`pub major: u64`, etc.).

**Decisão sobre `.pre` e `.build` quando vazios:**
- Vanilla retorna array vazio `()` para `.pre` quando não há pré-release.
- Paridade: `Value::Array([])` (vazio).

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Version existe no enum
grep -n "Value::Version" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Version tipo L1 tem campos públicos
grep -n "pub major\|pub minor\|pub patch\|pub pre\|pub build" entities/version.rs
# Esperado: 5 campos públicos

# 3. Verificar eval_field_access atual
grep -n "eval_field_access" rules/eval/bindings.rs
# Esperado: função existe; match em Value::Dict e Value::Content

# 4. Verificar que Value::Version ainda NÃO está em eval_field_access
grep -A 20 "fn eval_field_access" rules/eval/bindings.rs | grep -i "version"
# Esperado: zero hits (confirma ausência)

# 5. Verificar que Value::Array(Str) já existe (para .pre/.build)
grep -n "Value::Array" entities/value.rs | head -5
# Esperado: variant Array existe
```

**Critério de passagem**: 
- (1) `Value::Version` existe.
- (2) 5 campos públicos em `Version`.
- (3) `eval_field_access` existe e tem padrão Dict/Content.
- (4) `Version` ainda ausente em `eval_field_access`.
- (5) `Value::Array` existe para construir retorno de `.pre`/`.build`.

Se (3) falhar (não há `eval_field_access`), **parar imediatamente** — requer infra de field access primeiro. Se (2) falhar (campos não são públicos), reclassificar para M (refatoração de `Version`).

Documentar resultado no commit: `Sonda P411: Version variant OK; 5 campos públicos OK; eval_field_access existe (Dict/Content); Version ausente; Array variant OK.`

### A.1 — Prompt L0 `version-field-access.md`

Novo em `00_nucleo/prompts/rules/eval/version-field-access.md`:

- **Paridade**: `version("1.2.3-alpha").major` → `1` (Int); `.pre` → `("alpha",)` (Array de Str).
- **Substrato**: ramo `Value::Version(v)` em `eval_field_access` (`rules/eval/bindings.rs`).
- **Sem tipo novo**: reutiliza `Value::Version`, `Value::Int`, `Value::Array`.
- **Campos**: 5 campos fixos (`major`, `minor`, `patch`, `pre`, `build`); campo desconhecido → erro eval.
- **Conversão**: `u64` → `Value::Int(i64)`; `Vec<EcoString>` → `Value::Array` de `Value::Str`.
- **Testes**: 
  - 5 campos × 2 casos (valor presente, valor ausente/vazio) = 10.
  - Campo desconhecido: 1.
  - Total mínimo: 11 testes.

### A.2 — CHECKPOINT

Parar. Apresentar `version-field-access.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Localizar `eval_field_access`** em `rules/eval/bindings.rs`.
2. **Adicionar ramo `Value::Version(v)`** ao match de `target`:
   ```rust
   Value::Version(v) => {
       match field.as_str() {
           "major" => Ok(Value::Int(v.major as i64)),
           "minor" => Ok(Value::Int(v.minor as i64)),
           "patch" => Ok(Value::Int(v.patch as i64)),
           "pre" => Ok(Value::Array(v.pre.iter().map(|s| Value::Str(s.clone())).collect())),
           "build" => Ok(Value::Array(v.build.iter().map(|s| Value::Str(s.clone())).collect())),
           _ => Err(... "campo desconhecido em version: {}" ...),
       }
   }
   ```
3. **Testes** em `rules/eval/tests.rs` ou módulo de testes dedicado:
   - `version_field_major` — `version("1.2.3").major` → `Value::Int(1)`.
   - `version_field_minor` — `version("1.2.3").minor` → `Value::Int(2)`.
   - `version_field_patch` — `version("1.2.3").patch` → `Value::Int(3)`.
   - `version_field_pre` — `version("1.2.3-alpha.1").pre` → `Array([Str("alpha"), Str("1")])`.
   - `version_field_pre_empty` — `version("1.2.3").pre` → `Array([])`.
   - `version_field_build` — `version("1.2.3+build.1").build` → `Array([Str("build"), Str("1")])`.
   - `version_field_build_empty` — `version("1.2.3").build` → `Array([])`.
   - `version_field_unknown` — `version("1.2.3").foo` → erro eval.
4. **Linhagem**: `@prompt` aponta para `version-field-access.md`; `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas `bindings.rs` + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** adicionar campo `.raw` ou `.string` — não existe no vanilla `VersionElem`.
- **Não** implementar field access mutável (set) — o vanilla não tem; field access é read-only.
- **Não** adicionar método `get_field` em `entities/version.rs` — o lookup é no eval, não no tipo.
- **Não** tocar em `entities/version.rs` — campos já são públicos; nenhuma mudança necessária.
- **Não** tocar em `Duration` ou `Decimal` — um passo de cada vez.
- **Não** criar funções stdlib (`version_major`, etc.) — field access é via operador `.`, não call.

---

## 6. Critérios de aceitação

1. `version("1.2.3").major` → `Value::Int(1)`; `.minor` → `Value::Int(2)`; `.patch` → `Value::Int(3)`.
2. `version("1.2.3-alpha.1").pre` → `Value::Array([Str("alpha"), Str("1")])`.
3. `version("1.2.3+build.1").build` → `Value::Array([Str("build"), Str("1")])`.
4. Campos `.pre` e `.build` vazios retornam `Value::Array([])` (não erro).
5. Campo desconhecido (`.foo`) → erro eval com mensagem clara.
6. Zero tipo novo; zero variant novo; zero I/O.
7. Testes verdes (≥ 11 novos); lint zero; hashes propagados.
8. L0 salvo e hashado antes do código; sonda A.0 documentada no commit.

---

## 7. O que pode sair errado

- **`u64` não cabe em `i64` para versiones muito grandes.** Mitigação: `v.major as i64` — o vanilla usa `u64` mas o eval de Typst representa inteiros como `i64`. Se `major > i64::MAX`, o cast trunca. Documentar limitação; versões práticas nunca atingem esse valor.
- **`EcoString` não implementa `Clone` para `Value::Str`.** Mitigação: verificar se `Value::Str` aceita `EcoString` diretamente; se não, converter via `to_string()`.
- **Tentação de adicionar `.raw` (string original).** Mitigação: scope-out; o vanilla não expõe a string original como campo.
- **`eval_field_access` é macro ou pattern complexo.** Mitigação: sonda A.0 detecta; se for macro, o passo cresce para M.

---

## 8. Referências

- `entities/version.rs` (P401) — `Version` tipo L1 com campos `major`, `minor`, `patch`, `pre`, `build`.
- `entities/value.rs` — variants `Value::Version`, `Value::Int`, `Value::Array`, `Value::Str`.
- `rules/eval/bindings.rs` — `eval_field_access` (infra a sondar em A.0).
- P410 — análise de sonda que confirmou ausência de field access para Version.
- ADR-0107 — paridade linguagem (forma do field access).
- ADR-0108 — medir-antes-de-decidir (sonda A.0).

---

## 9. Nota sobre o Tekt

Este passo é **S** porque o tipo já existe, os campos são públicos, e o field access é apenas mais um ramo em `eval_field_access`. O risco principal é a conversão `Vec<EcoString>` → `Value::Array` de `Value::Str` — se `Value::Str` não aceita `EcoString` diretamente, precisa de conversão. A sonda A.0 verifica a infra de field access; se `eval_field_access` for simples (match em `Value`), o passo é minutos.

Se P410 revelou que `eval_field_access` já tem padrão extensível (Dict/Content), `Version` é apenas mais um braço. Se o padrão for fechado (apenas 2 tipos hardcoded), o passo pode crescer para M.
