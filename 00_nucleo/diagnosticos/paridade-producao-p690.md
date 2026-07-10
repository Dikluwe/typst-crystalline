# Paridade em produção — P690 — indexação de `str` unificada para bytes

**Passo:** P690 (`00_nucleo/materialization/typst-passo-690.md`)
**Data:** 2026-07-10
**Commit deste relatório:** `__P690_COMMIT__` (preenchido no 2.º commit)
**ADRs:** ADR-0107 (paridade com a linguagem — aqui a linguagem **é** bytes),
ADR-0108 (medir antes de decidir), ADR-0114 (sonda obrigatória; mudança
fundamental na semântica de um tipo usado em todo o lado).

---

## 0. Proveniência das medições (regra de P569)

- **Commit em que os testes/sondas foram corridos:** `a0a242ff8e075d4c8544918bd93ea7acbcbd057e`
  (HEAD de P689; trabalho feito em detached HEAD sobre este commit).
- **Working tree na medição:** alterações não commitadas em apenas 2 ficheiros tracked:
  `00_nucleo/prompts/rules/stdlib/collections.md` e `01_core/src/rules/stdlib/collections.rs`.
  `git diff --stat` no momento da medição:
  `2 files changed, 271 insertions(+), 21 deletions(-)`.
- **Hora da validação final:** 2026-07-10T21:51:50Z (`date -u`).
- **Binários usados:**
  - vanilla 0.15.0: `lab/typst-original/target/release/typst` (commit 969087ec; build pré-existente, 51 MB).
  - cristalino: `target/debug/typst` (recompilado com `cargo build` após a edição; pacote `typst-wiring`, bin `typst`).
  - extracção de texto: `pdftotext -layout` (`/usr/bin/pdftotext`).
- **Scratch:** `temp_p690/` (dentro do repo), apagado no fim do passo.

Qualquer número abaixo foi obtido por compilação real nos dois motores nesta sessão,
neste estado de código. Não é estimativa.

---

## 1. Resumo

`str.len()`, `str.at()` e `str.slice()` do cristalino indexavam por **carácter**, enquanto o
vanilla — e os `position()`/`match()` adicionados em P689 — indexam por **byte**. A API de
`str` estava internamente inconsistente: `s.at(s.position("m"))` em `"café mais texto"`
dava `"a"` no cristalino e `"m"` no vanilla. Este passo migrou `len/at/slice` para bytes
(paridade vanilla, nomes iguais = comportamento igual — regra P662-P664) e preservou a
indexação por carácter sob nomes próprios (`char-len`, `char-at`, `char-slice`),
documentados como extensão cristalina não-portável (padrão `table.numbering` de P459).

Resultado: `s.at(s.position("m")) == "m"` nos dois motores; `cargo test --workspace`
4368 passed / 0 failed; `crystalline-lint .` 0 violations.

---

## 2. Sonda

### 2.1 Prova concreta da inconsistência

Documento `"café mais texto"`, `#s.at(s.position("m"))`:

| motor | valor | leitura |
|-------|-------|---------|
| vanilla | `"m"` | `position("m")`=6 (byte), `at(6)` em bytes = `m` |
| cristalino **antes** | `"a"` | `position("m")`=6 (byte), `at(6)` em **chars** = `a` (c-a-f-é-espaço-m-**a**) |
| cristalino **depois** | `"m"` | `at(6)` em bytes = `m` |

A diferença pré-P690 não era só de `len()` — era uma inconsistência **dentro** do
cristalino: combinar `position` (byte) com `at` (char) sobre texto não-ASCII produzia
resultado errado, silenciosamente.

### 2.2 Classificação byte/char de **todos** os métodos de `str`

Medidos um a um contra o vanilla 0.15.0 (string de prova com `é` = 2 bytes UTF-8).

| Método | vanilla | cristalino antes | classe | acção |
|--------|---------|------------------|--------|-------|
| `len` | `"café"`→**5** | 4 | **byte** | migrar → byte |
| `at` | `"éabc".at(2)`→**"a"** | "b" | **byte** (+erro se non-boundary/OOB) | migrar → byte |
| `slice` | `"éabc".slice(2,4)`→**"ab"** | "bc" | **byte** (+erro se non-boundary/OOB; sem clamp) | migrar → byte |
| `first` / `last` | `"é"`/`"c"` | `"é"`/`"c"` | N/A (devolvem char, não índice) | manter |
| `clusters` / `codepoints` | `("c","a","f","é")` | igual | N/A (array de chars) | manter |
| `contains` / `starts-with` / `ends-with` | bool | igual | N/A (bool) | manter |
| `replace` / `trim` / `split` / `rev` | igual | igual | N/A (str/array) | manter |
| `repeat` | **não existe** | existe | extensão cristalina (N/A) | manter (registar) |
| `position` / `match` | byte | byte | já paridade (P689) | manter |
| `find` | devolve **substring** (`"y"`) | devolve **int** (3) | ⚠️ débito de **semântica** (tipo de retorno), não byte/char | **fora do alcance** (registar) |

Comportamento de erro do vanilla, replicado no cristalino (medido):

| caso | vanilla | cristalino depois |
|------|---------|-------------------|
| `"éabc".at(10)` | erro `out of bounds (index: 10, len: 5)` | erro `índice fora de limites (índice 10, len 5)` |
| `"éabc".at(1)` | erro `index 1 is not a character boundary` | erro `índice 1 não é uma fronteira de carácter` |
| `"café".at(-1)` | erro `index -1 is not a character boundary` | erro `índice -1 não é uma fronteira de carácter` |
| `"éabc".slice(0,100)` | erro `out of bounds (index: 100, len: 5)` | erro `índice fora de limites (end, len 5)` |
| `"éabc".slice(1,4)` | erro `not a character boundary` | erro `índice não é uma fronteira de carácter` |
| `"abcdef".slice(4,2)` | `""` (start>end) | `""` |

(As mensagens ficam em português, como o resto do módulo; o observável de paridade é
"erro vs valor nos mesmos casos", não a string exacta — ADR-0107.)

**Nota sobre `find`:** a sonda mostrou que o vanilla `str.find(substr)` devolve a
**substring encontrada** (`str | none`), não um índice. O cristalino devolve `int`
(índice em bytes). Isto é uma diferença de **semântica** (tipo de retorno), não de
byte/char, e está **fora do alcance** deste passo (que trata de indexação). Fica
registada como débito em §6.

### 2.3 Call-sites internos que dependem da convenção antiga

- Os helpers `str_len`/`str_at`/`str_slice` são `fn` privadas chamadas **apenas** pelo
  dispatcher (`collections.rs`, braços `("len"|"at"|"slice")`) e pelos testes do próprio
  módulo. Nenhum outro código Rust do cristalino os consome.
- O `grep` sugerido pelo passo (`\.len()|\.at(|\.slice(` filtrado por `str|text`) devolve
  111 linhas, mas são `.len()` de `Vec`/`String` em código Rust (já bytes para `&str`,
  count para `Vec`) e `.at()`/`.slice()` que não são os métodos Typst — falsos positivos
  para o risco de char-indexing.
- Fixtures `.typ`: apenas `benches/corpus/b4_code.typ` usa `.at()`/`.len()`, e **sempre**
  em `array`/`dict` (`numbers.len()`, `cache.get().at(key, …)`, `t.at("children", …)`),
  nunca em `str`.

**Conclusão de risco interno:** baixo. O único consumidor da semântica é o próprio módulo;
a migração não tem efeito em cascata fora dele.

---

## 3. Decisão de alcance (medir antes de decidir — ADR-0108)

Com a tabela §2.2 e o mapeamento §2.3 (medições feitas **antes** da decisão):

- **Migrar (obrigatório):** `len`, `at`, `slice` → bytes, com a convenção exacta do
  vanilla (negativo conta do fim em bytes; erro se fora de limites; erro se o índice não
  for fronteira de carácter; `slice` sem clamp; `start > end` → `""`).
- **Preservar sob nome novo (critério de fecho do passo):** `char-len`, `char-at`,
  `char-slice` — mantêm a convenção por carácter (útil para texto humano), como extensão
  cristalina não-portável.
- **Manter inalterados:** `first`, `last`, `clusters`, `codepoints`, `contains`,
  `starts-with`, `ends-with`, `replace`, `trim`, `split`, `rev`, `repeat`, `position`,
  `match` — não indexam (ou já são bytes), logo a questão byte/char não se aplica.
- **Fora do alcance (débito):** `find` (tipo de retorno `int` vs `str` no vanilla) —
  semântica, não indexação; passo separado.

O alcance coube num único passo (3 helpers reescritos + 3 novos + 3 braços no dispatcher
+ testes + L0); **não foi necessário dividir**.

---

## 4. Implementação

`01_core/src/rules/stdlib/collections.rs`:

- `str_len` (≈L484): `s.chars().count()` → `s.len()` (bytes).
- `str_at` (≈L502): reescrito — índice em bytes, `is_char_boundary`, erro OOB/non-boundary;
  devolve o char que começa no índice (`st[i..].chars().next()`).
- `str_slice` (≈L519): reescrito — índices em bytes, sem clamp, validação de limites e
  fronteiras, `start > end` → `""`.
- `char_len` / `char_at` / `char_slice` (novos): preservam a lógica pré-P690 por carácter.
- Dispatcher (≈L73-95): 3 braços novos `("char-len"|"char-at"|"char-slice")`.

`00_nucleo/prompts/rules/stdlib/collections.md`: tabela de `str` actualizada
(`len/at/slice` em bytes; `char-*` registados como extensão; `find` com débito;
`repeat` marcado como extensão), nota P690 (ADR-0107), scope-outs revistos.
`crystalline-lint --fix-hashes .` → "Nothing to fix" (`collections.rs` não tem linha
`@prompt-hash`, pré-existente; o lint tolera).

---

## 5. Testes

5 testes unitários novos em `collections.rs::tests` (P690), todos verdes:

- `p690_str_len_bytes` — ASCII (sem efeito) + multi-byte (`"café"`→5, `"éabc"`→5).
- `p690_str_at_bytes` — ASCII, boundary multi-byte, negativo non-boundary→erro,
  non-boundary→erro, OOB→erro, negativo além do início→erro.
- `p690_str_slice_bytes` — ASCII, multi-byte, `count` em bytes, negativo, `start>end`→`""`,
  `end`+`count` simultâneos→erro, OOB (sem clamp)→erro, negativo além do início→erro,
  non-boundary→erro.
- `p690_str_at_position_consistencia` — a prova: `at(position("m"))` em `"café mais texto"`
  → `"m"` (pré-P690 dava `"a"`).
- `p690_char_len_at_slice_preserva_chars` — `char-len/char-at/char-slice` por carácter.

`cargo test --workspace` (2026-07-10T21:51:50Z): **4368 passed / 0 failed**
(3699 typst-core + 610 + 28 + 2 + 27 + 2; +5 relativamente a P689, os 5 testes novos).
`crystalline-lint .`: **0 violations**.

ASCII puro é regressão neutra (char == byte): confirmado pelos ramos ASCII dos testes e
pela ausência de qualquer falha no workspace (nenhum teste E2E existente dependia de
char-indexing em `str`).

---

## 6. Validação end-to-end

`temp_p690/sonda_main.typ` compilado nos dois motores após a mudança — os métodos que
indexam agora coincidem:

| expr | vanilla | cristalino depois |
|------|---------|-------------------|
| `"café".len()` | 5 | **5** |
| `"éabc".at(2)` | "a" | **"a"** |
| `"éabc".slice(2,4)` | "ab" | **"ab"** |
| `"éabc".slice(0, count:2)` | "é" | **"é"** |
| `"café mais texto".at(s.position("m"))` | "m" | **"m"** |

`char-*` end-to-end (cristalino): `"café".char-len()`→4, `"éabc".char-at(2)`→"b",
`"café".char-at(-1)`→"é", `"éabc".char-slice(2,4)`→"bc", `"éabc".char-slice(0,count:2)`→"éa".

RTL/devanágari: o shaping é independente de `str.len/at/slice` (operações de string em
`eval`, não de layout); nenhum fixture usa esses métodos em `str` (§2.3); o corpus geral
(`cargo test --workspace`) ficou verde. Sem regressão por essa via.

---

## 7. Débitos e notas

- **`str.find` (semântica):** cristalino devolve `int` (índice byte); vanilla devolve a
  substring (`str | none`). Diferença de tipo de retorno, não de byte/char — fora do
  alcance de P690, a corrigir num passo próprio (muda contrato público do método).
- **`str.repeat`:** não existe no vanilla 0.15; é extensão cristalina pré-existente. N/A
  para indexação; agora registado como extensão no L0.
- **Mensagens de erro em português:** comportamento (erro vs valor) paritário com o
  vanilla; strings não são contrato (ADR-0107). A mensagem do vanilla inclui
  `(index: N, len: M)`; o cristalino inclui `(índice N, len M)` para `at` — equivalente
  em substância.
- **Inconsistência de representação interna:** nenhuma. `position`/`match` (P689) e
  `len/at/slice` (P690) são agora todos bytes — a API de `str` voltou a ser consistente.

---

## 8. Critério de fecho do passo

- [x] Sonda completa, alcance da inconsistência confirmado com prova concreta (§2.1).
- [x] Métodos de `str` que indexam unificados para byte (§2.2, §4); os que não indexam
      classificados N/A e mantidos.
- [x] Testado com ASCII puro (sem efeito) e multi-byte (correcção) — §5.
- [x] `s.at(s.position("m")) == "m"` nos dois motores — §6.
- [x] Indexação por carácter preservada sob `char-len/char-at/char-slice`, registada como
      extensão com razão (§4, L0).
- [x] Call-sites internos revistos, sem regressão (§2.3).
- [x] `cargo test --workspace` sem regressão (4368/0).
- [x] `crystalline-lint .` limpo (0 violations).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p690.md`, com hash do commit.
- [x] Alcance coube num passo — não dividido (justificação em §3).
