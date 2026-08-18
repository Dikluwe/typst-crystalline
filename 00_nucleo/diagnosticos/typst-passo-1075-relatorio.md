# Relatório de Execução — Passo 1075: Captura Regex Não-Participante Devolve `none` em vez de `""` — Achado #10 do P1031

**Data**: 2026-08-18
**Passo**: 1075 — Captura Regex Não-Participante Devolve `none` (Achado #10 do P1031)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)
**Status**: CONCLUÍDO COM ÊXITO (Tipo refinado para `Vec<Option<String>>`, paridade com Vanilla comprovada, 100% PASS na suíte de testes)

---

## 1. Contexto e Motivação (Achado #10 do P1031)

No compilador Typst oficial (`crates/typst-library/src/foundations/str.rs:936-940`), grupos de captura opcionais que não participam do match são explicitamente mapeados para `Value::None`:
```rust
"captures" => cap.iter().skip(1)
    .map(|opt| opt.map_or(Value::None, |m| m.as_str().into_value()))
    .collect::<Array>(),
```

No Crystalline ([`01_core/src/entities/regex.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/regex.rs)), a struct `RegexMatch` utilizava `captures: Vec<String>` e invocava `.unwrap_or_default()`, colapsando grupos ausentes em string vazia `""`:
* `"ab".match(regex("a(x)?(b)")).captures`:
  * Vanilla: `(none, "b")`
  * Crystalline (Antes): `("", "b")` ❌

---

## 2. Auditoria e Diagnóstico de Chamadores (§1 e §2 do L0)

### 2.1 Análise das Duas Ocorrências (`:65` e `:85`)
As duas ocorrências identificadas pertenciam a duas funções legítimas da API de `Regex`:
1. `Regex::captures_first(&self, text: &str) -> Option<RegexMatch>` (linha 65): Usada por `str.match()` e `str.position()`.
2. `Regex::captures_all(&self, text: &str) -> Vec<RegexMatch>` (linha 85): Usada por `str.matches()`.

Não se tratava de citação duplicada por engano, mas sim do mesmo padrão aplicado a `captures_first` (primeiro match) e `captures_all` (todos os matches).

### 2.2 Refatoração do Tipo
* Em [`01_core/src/entities/regex.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/regex.rs):
  * `RegexMatch.captures` alterado de `Vec<String>` para `Vec<Option<String>>`.
  * `caps.get(i).map(|g| g.as_str().to_string())` agora preserva `None` quando o grupo opcional não participa.
* Em [`01_core/src/compiler/stdlib/collections.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/collections.rs):
  * A função construtora `match_dict` mapeia `Some(s) => Value::Str(s.into())` e `None => Value::None` dentro do `Value::Array` retornado em `"captures"`.

---

## 3. Medição Diferencial e Paridade com Vanilla Typst (§4 e §5 do L0)

Documento de teste com grupos opcionais não participantes, grupos nomeados e correspondências múltiplas:
```typst
#let m1 = "ab".match(regex("a(x)?(b)"))
#let m2 = "a".match(regex("(?P<name>x)?(a)"))
#let m3 = "ab axb".matches(regex("a(x)?(b)")).map(m => m.captures)
#repr(m1.captures)
#repr(m2.captures)
#repr(m3)
```

### 3.1 Resultados Comparativos

| Caso de Teste | Saída Vanilla (`/usr/local/bin/typst`) | Saída Crystalline (Antes) | Saída Crystalline (P1075) | Paridade |
| :--- | :--- | :--- | :--- | :---: |
| `"ab".match(regex("a(x)?(b)"))` | `(none, "b")` | `("", "b")` | `(none, "b")` | **100% IDÊNTICO** |
| `"a".match(regex("(?P<name>x)?(a)"))` | `(none, "a")` | `("", "a")` | `(none, "a")` | **100% IDÊNTICO** |
| `"ab axb".matches(regex("a(x)?(b)"))` | `((none, "b"), ("x", "b"))` | `(("", "b"), ("x", "b"))` | `((none, "b"), ("x", "b"))` | **100% IDÊNTICO** |

---

## 4. Alterações Implementadas

1. **Código-Fonte**:
   * [`01_core/src/entities/regex.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/regex.rs): `RegexMatch.captures` passa para `Vec<Option<String>>`; `captures_first` e `captures_all` agora retornam `None` para grupos não participantes.
   * [`01_core/src/compiler/stdlib/collections.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/collections.rs): `match_dict` traduz `Option<String>` para `Value::Str` / `Value::None`.
2. **Prompts L0**:
   * [`00_nucleo/prompts/entities/regex.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/regex.md) atualizado e selado (Hash: `29ce084d`).
3. **Testes Unitários**:
   * Adicionado teste unitário dedicado [`p1075_str_match_optional_group_returns_none`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/collections.rs) cobrindo `str.match`, grupos nomeados opcionais e `str.matches`.

---

## 5. Validação Final

* `crystalline-lint .`: APROVADO (0 erros, 0 avisos de drift).
* `cargo test --workspace`: APROVADO (5.948 testes, 100% PASS).
