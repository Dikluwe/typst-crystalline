# Paridade em produção — P692 — `str.matches()` e `str.normalize()`

**Passo:** P692 (`00_nucleo/materialization/typst-passo-692.md`)
**Data:** 2026-07-10
**Commit deste relatório:** `e731f6a2c8ff0aa7e5804f155e1a55ef72960375` (preenchido no 2.º commit)
**ADRs:** ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
regra P662-P664 (nome igual ao vanilla obriga a comportamento igual).
**Dependência:** P689 (`match()` singular e `RegexMatch`/`captures_first`, reaproveitados).

---

## 0. Proveniência das medições (regra de P569)

- **Commit em que os testes/sonda foram corridos:** `5b5e7ff73e232ba58fc77a82ff16bc0ff1df07ad`
  (HEAD de P691; trabalho em detached HEAD sobre este commit).
- **Working tree na medição:** alterações não commitadas em 7 ficheiros tracked:
  `Cargo.toml`, `Cargo.lock`, `crystalline.toml`, `01_core/Cargo.toml`,
  `01_core/src/entities/regex.rs`, `01_core/src/engine/stdlib/collections.rs`,
  `00_nucleo/prompts/engine/stdlib/collections.md`. `git diff --stat` no momento da medição:
  `7 files changed, 236 insertions(+), 22 deletions(-)`.
- **Hora da validação final:** 2026-07-10T23:15:05Z (`date -u`).
- **Binários:** vanilla 0.15.0 (`lab/typst-original/target/release/typst`, commit 969087ec);
  cristalino (`target/debug/typst`, recompilado com `cargo build` após a edição); extracção
  com `pdftotext -layout`.
- **Scratch:** `temp_p692/` (dentro do repo), apagado no fim.

---

## 1. Resumo

Dois métodos de `str` da documentação oficial do Typst 0.15 estavam **totalmente
ausentes** do cristalino: `matches()` (plural — array de todas as ocorrências) e
`normalize()` (normalização Unicode). Implementados. Com eles, o cristalino cobre
**todos** os métodos de instância oficiais de `str` (ver §3). `cargo test --workspace`
4376 passed / 0 failed; `crystalline-lint .` 0 violations.

---

## 2. Sonda mínima (medida antes de decidir — ADR-0108)

### 2.1 `matches()` (vanilla 0.15.0)

| expr | vanilla |
|------|---------|
| `("um dois três quatro").matches(regex("\w+"))` | 4 dicts; "três" em `start:8, end:13` (bytes) |
| `("abc").matches(regex("z"))` | `()` |
| `("a1b2c3").matches(regex("[a-z]"))` | 3 dicts (a/b/c, bytes 0/2/4) |
| `("a1b2").matches(regex("([a-z])(\d)"))` | 2 dicts, `captures: ("a","1")` / `("b","2")` |
| `("abab").matches("ab")` | 2 dicts (start 0 e 2) — **aceita str literal** |

Confirma: aceita `str | regex`; devolve **array de dicts** `{start, end, text, captures}`
(índices em bytes), mesma estrutura de `match()` singular; `()` se nenhuma ocorrência.

### 2.2 `normalize()` (vanilla 0.15.0)

| expr | vanilla (`repr`) | `len()` |
|------|------------------|---------|
| `"café".normalize()` | `"café"` | 5 |
| `"café".normalize(form: "nfc")` | `"café"` | 5 |
| `"café".normalize(form: "nfd")` | `"cafe\u{301}"` | **6** |
| `"café".normalize(form: "nfkc")` | `"café"` | 5 |
| `"café".normalize(form: "nfkd")` | `"cafe\u{301}"` | **6** |

Confirma (e cruzado com a fonte `foundations/str.rs:329`): `form` é **named**, default
**NFC**, formas aceites `nfc`/`nfd`/`nfkc`/`nfkd`. O `len` em bytes muda com a forma
(5 NFC vs 6 NFD) — prova de que a representação interna muda, não só a apresentação.

### 2.3 Estado anterior do cristalino

Ambos os métodos davam erro ("type string has no method matches/normalize") — ausência
total, não comportamento errado.

---

## 3. Lista oficial de métodos de `str` (critério de fecho do passo)

Fonte: `lab/typst-original/crates/typst-library/src/foundations/str.rs` (todos os
`#[func]` de instância) cruzada com sondas no vanilla.

| Método oficial | Cristalino antes de P692 | Cristalino depois de P692 |
|----------------|--------------------------|----------------------------|
| `len` | ✓ (P690, bytes) | ✓ |
| `first` / `last` | ✓ | ✓ |
| `at` / `slice` | ✓ (P690, bytes) | ✓ |
| `clusters` / `codepoints` | ✓ (P689) | ✓ |
| `to-unicode` | ✓ | ✓ |
| `contains` / `starts-with` / `ends-with` | ✓ | ✓ |
| `find` | ✓ (P691) | ✓ |
| `position` / `match` | ✓ (P689) | ✓ |
| `replace` / `trim` / `split` | ✓ | ✓ |
| `rev` | ✓ | ✓ |
| **`matches`** | ✗ ausente | **✓ (P692)** |
| **`normalize`** | ✗ ausente | **✓ (P692)** |

**Conclusão:** após P692, **nenhum** método de instância oficial de `str` fica por
implementar. Os símbolos do cristalino que não estão na lista oficial são **extensões**
(não ausências), confirmadas como não-portáveis:

- `char-len` / `char-at` / `char-slice` (P690) — indexação por carácter.
- `repeat` — confirmado em P690: vanilla "type string has no method repeat".
- `to-upper` / `to-lower` — confirmado neste passo (2026-07-10): vanilla "type string has
  no method to-upper/to-lower".

(`from-unicode` é constructor de tipo, não método de instância — fora deste mapa.)

---

## 4. Decisão de alcance (medir antes de decidir — ADR-0108)

- **`matches`:** aceita `str | regex`, devolve array de dicts (paridade exacta com a
  fonte, que usa `match_indices` para str e `captures_iter` para regex).
- **`normalize`:** `form` named, default NFC, 4 formas; usa a crate `unicode-normalization`
  (NFC/NFD/NFKC/NFKD) — computação pura sobre strings, sem I/O, admissível em L1.
- **Crate:** `unicode-normalization` já era dependência **transitiva** de `typst-core`
  (via `hayagriva → biblatex`, `cargo tree -i`); P692 torna-a **directa** e declara-a em
  `[l1_allowed_external.unicode_normalization]` (`crystalline.toml`), seguindo o padrão
  das outras crates Unicode (ADR-0010 a ADR-0013). Não foi criada ADR nova (o passo não a
  exige; a decisão fica registada aqui e no L0).
- **Fora do alcance (débito):** `match` singular do cristalino (P689) só aceita `regex`,
  mas o vanilla aceita `str | regex` (`foundations/str.rs:456`). Variação de assinatura —
  registada em §8, não corrigida aqui (manter o passo focado em `matches`/`normalize`).

---

## 5. Implementação

- `01_core/src/entities/regex.rs`: `Regex::captures_all(&self, text) -> Vec<RegexMatch>`
  (generalização de `captures_first` sobre `captures_iter`).
- `01_core/src/engine/stdlib/collections.rs`:
  - `use unicode_normalization::UnicodeNormalization;`
  - helper `match_dict(start, end, text, captures) -> Value` (constrói o dict), partilhado
    por `str_match` (refactorizado para o usar, comportamento inalterado) e `str_matches`.
  - `str_matches(s, args)`: str → `s.match_indices(pat)`; regex → `re.captures_all(s)`.
  - `str_normalize(s, args)`: rejeita posicionais; `form` named ∈ {nfc,nfd,nfkc,nfkd}
    (default nfc); forma desconhecida/tipo errado → erro.
  - dispatcher: braços `("matches")` e `("normalize")`.
- Config: `Cargo.toml` (workspace.dependencies + `unicode-normalization = "0.1"`),
  `01_core/Cargo.toml` (dependência directa), `crystalline.toml`
  (`[l1_allowed_external.unicode_normalization]`, `types = ["UnicodeNormalization"]`).
  `Cargo.lock` actualizado (+1 entrada de dependência directa).
- L0 `collections.md`: linhas de `matches`/`normalize`, nota P692 (crate + critério de
  cobertura), cabeçalho. `crystalline-lint --fix-hashes .` → "Nothing to fix"
  (`collections.rs` sem `@prompt-hash`; `regex.rs` tem hash mas `regex.md` não mudou).

---

## 6. Testes

5 testes unitários novos em `collections.rs::tests` (P692), todos verdes:

- `p692_str_matches_regex` — `regex("\w+")` em "um dois três quatro" → 4 dicts; "três" em
  bytes 8..13.
- `p692_str_matches_str_literal_e_vazio` — `"abab".matches("ab")` → 2 dicts (captures
  vazias); sem match → `[]`.
- `p692_str_matches_captures` — `regex("([a-z])(\d)")` em "a1b2" → captures ("a","1")/("b","2").
- `p692_str_normalize_forms` — default==nfc ("café", 5 bytes); nfd ("cafe\u{301}", 6
  bytes, verificado via `str_len`); nfkc==nfc; nfkd==nfd.
- `p692_str_normalize_erros` — forma desconhecida, tipo errado, e posicional → erro.

`cargo test --workspace` (2026-07-10T23:15:05Z): **4376 passed / 0 failed**
(3707 typst-core + 610 + 28 + 2 + 27 + 2; +5 relativamente a P691).
`crystalline-lint .`: **0 violations** (a whitelist `unicode_normalization` resolveu sem
V14/V3 — o tipo externo não aparece em contrato público).

---

## 7. Validação end-to-end

`matches` — cristalino == vanilla em todos os casos da sonda (m_words com "três" em
8..13, m_none `()`, m_letters, m_cap com captures, m_str literal). Idêntico em valor (a
única diferença é o wrapping do `pdftotext`).

`normalize` — paridade em **valor**, confirmada por `len`:

| expr | vanilla len | cristalino len |
|------|-------------|----------------|
| `normalize()` / `nfc` / `nfkc` | 5 | 5 |
| `nfd` / `nfkd` | 6 | 6 |

Nota de apresentação (ADR-0107): o `repr` do vanilla mostra `"cafe\u{301}"` (escapa o
combining acute), enquanto o cristalino mostra `"café"` (não escapa; o `pdftotext`
recombina visualmente). O **conteúdo** é o mesmo (6 bytes decompostos, provado por `len`);
a diferença é só de escape/render no `repr`, mecânica de apresentação, não de valor.

---

## 8. Débitos e notas

- **`match` singular só aceita `regex`** (vs `str | regex` no vanilla) — variação de
  assinatura pré-existente (P689), fora do alcance de P692; registada no L0 e aqui para
  passo próprio. (`matches`, introduzido aqui, já aceita `str | regex` correctamente.)
- **`repr` de combining characters** não escapa `\u{…}` no cristalino (vs vanilla que
  escapa) — diferença de apresentação, não de valor; fora do alcance.
- Nenhum método oficial de `str` fica por implementar (§3).

---

## 9. Critério de fecho do passo

- [x] Sonda mínima completa, os dois métodos confirmados contra o vanilla (§2).
- [x] `matches()` implementado e testado (array de dicts, str|regex).
- [x] `normalize()` implementado e testado (4 formas, default nfc).
- [x] Métodos existentes sem regressão (workspace verde; `str_match` refactorizado sem
      mudança de comportamento).
- [x] Sem regressão em `cargo test --workspace` (4376/0).
- [x] `crystalline-lint .` limpo (0 violations).
- [x] Lista de métodos de `str` actualizada — nenhum oficial por implementar (§3).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p692.md`, com hash do commit.
