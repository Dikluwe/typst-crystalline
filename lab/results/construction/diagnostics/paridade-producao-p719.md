# Paridade Produção — P719 — For-loop sobre `Dict` (`for (key, value) in dict`)

**Data:** 2026-07-12
**Passo:** `00_nucleo/materialization/typst-passo-719.md`
**Hash do commit (implementação):** `3ab08fc54`.
**HEAD base:** `f571a3644` (fim de P718).
**Estado:** FECHADO — `for (key, value) in dict`, forma de um só nome, e
dict vazio implementados com paridade ao vanilla, ordem de inserção
confirmada.

---

## 1. Sonda

### 1.1 Comportamento completo no vanilla

Binário `lab/typst-original/target/release/typst`, repositório em
`f571a3644`. Documento do passo (`/tmp/p719-for-dict.typ`):

```
for (k, v) in (a: 1, b: 2) [#k=#v ]     → "a=1 b=2 "
for k in (a: 1, b: 2) [#k ]              → "(\"a\", 1) (\"b\", 2) "
for v in (a: 1, b: 2).values() [#v ]     → "1 2 "
```

**Achado central da sonda:** a forma de um só nome (`for k in dict`)
**não** itera as chaves — liga `k` ao **par (chave, valor) inteiro**
como array de 2 elementos (por isso `#k` mostra `("a", 1)`, não `"a"`).
Iterar só as chaves exige `.keys()`; só os valores, `.values()` (ambos
já implementados no cristalino, fora do scope deste passo).

Ordem de iteração confirmada como inserção (`(z: 1, a: 2, m: 3)` →
`z a m` — não alfabética) nos dois lados.

### 1.2 Mecanismo vanilla (`typst-eval/flow.rs:114-190`, `cast.rs:186-189`)

`ast::ForLoop::eval` despacha por **tipo do iterável** (`Value::Array`
vs `Value::Dict`), ambos através da mesma macro `iter!` — a única
diferença é a fonte do iterador (`dict.iter()`, que devolve `(&Str,
&Value)`). `IntoValue for (&Str, &Value)` (`cast.rs:186-189`) converte
cada par em `Value::Array([Str(key), value])` — **o mesmo formato** que
a iteração de array com destructuring de 2 elementos já usa. O bind
("um nome → item inteiro; N nomes → destroem posicionalmente") é
genérico, não específico de array nem de dict.

### 1.3 Mecanismo cristalino existente (`control_flow.rs`, P540)

`eval_for` só tinha braço para `Value::Array(items)`; o resto (`None`
→ vazio, outro → erro) já existia. `Pattern::bindings()` devolve lista
plana de idents (mecanismo mais simples que o `destructure_pattern`
recursivo de P715/716, usado só por `let`/atribuição — o `for` nunca
usou essa função). Confirmado: bastava reaproveitar o corpo do loop já
escrito para array, alimentando-o com uma sequência diferente — **sem**
tocar em `destructure_pattern`/`Scopes`.

### 1.4 Nota de divergência pré-existente (medida, não corrigida)

`for (a, b, c) in (x: 1, y: 2) [x]` → vanilla: `"not enough elements to
destructure"` + hint (`length of 2, but the pattern expects 3 elements`,
mesma família de `wrong_number_of_elements` de P715). Cristalino (já
antes deste passo, só para array): `"cannot destructure {n} values into
{m} bindings"` — mensagem diferente, confirmada reproduzível já com
`for (a, b, c) in ((1,2),) [x]` (puro array, sem dict). **Pré-existente
de P540, fora do scope deste passo** (que é "for-loop sobre Dict", não
"paridade da mensagem de aridade do for"). Como `Dict` reaproveita o
mesmo `run_for_loop`, esta divergência passa a aplicar-se também ao
caminho de dict — inevitável dado o reaproveitamento, mas não introduzida
por este passo. Registada como candidata a passo futuro dedicado.

### 1.5 Estado do cristalino antes

`/tmp/p719-for-dict.typ` → `error: não é possível iterar sobre
dictionary` na primeira linha, confirmando o bloqueio relatado por P718.

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/eval.md` §P719 (hash
`605a11fb` via `crystalline-lint --fix-hashes`). Testes escritos antes
do código; confirmados a falhar por reversão temporária de
`control_flow.rs` para o estado pré-P719 (5 de 6 falhavam com "não é
possível iterar sobre dictionary"; o de não-regressão de array passava,
como esperado por ainda não tocar nesse caminho).

- **`control_flow.rs` `eval_for`**: extraído `run_for_loop(items: Vec
  <Value>, loop_expr, scopes, ctx, engine)` — corpo do antigo braço
  `Value::Array`, **inalterado** (bind de padrão, corpo, `break`/
  `continue`/`return`).
  - `Value::Array(items)` → `run_for_loop(items, ...)`.
  - `Value::Dict(dict)` (**novo**) → `dict.into_iter().map(|(k, v)|
    Value::Array(vec![Value::Str(k), v])).collect()`, depois
    `run_for_loop(...)` — mirror de `IntoValue for (&Str, &Value)`.
    Zero código de bind novo: o braço `bindings.len() == 1` vs `> 1`
    já existente cobre "um nome liga o par" e "dois nomes destroem"
    automaticamente, sem casos especiais para dict.
- Refactor (extrair `run_for_loop`) justificado pela reutilização
  genuína entre dois braços com ~70 linhas de lógica de controlo de
  fluxo idêntica — evita duplicar `break`/`continue`/`return` em dois
  sítios que teriam de ficar sincronizados manualmente.

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste
passo, commitado de seguida como `3ab08fc54` (o diff do commit é o
estado medido). Re-verificação pós-commit em §3.4.

### 3.1 Documento do passo vs vanilla

`/tmp/p719-for-dict.typ` → `pdftotext` **idêntico** ao vanilla:
`a=1 b=2 ("a", 1) ("b", 2) 1 2`.

### 3.2 Suites

- `cargo test --workspace` → **3912 passed, 0 failed** no `typst-core`
  (3906 em P718 + 6 novos), restantes crates verdes.
- `crystalline-lint .` → **0 violations**.

### 3.3 Reprodução `cetz` — campos fixos de progresso

Documento do passo (`/tmp/p719-cetz.typ`):

- **Exit code:** 1. **Tempo:** 51,6s (ver §3.4 para a medição
  pós-commit).
- **Bloqueio de P718 resolvido** — `não é possível iterar sobre
  dictionary` desapareceu.
- **Próximo bloqueio, com `file:line`:** `error: cannot apply Add to
  array and array` — o cristalino não suporta `array + array`
  (concatenação). Isolado por redução: `#let a = (1,2) + (3,4)` →
  `(1, 2, 3, 4)` no vanilla, mesmo erro no cristalino. Consumidor real,
  múltiplos sítios: `path-util.typ:423,430`, `bezier.typ:413`,
  `hobby.typ:77,78,126`. `operators.rs` não tem nenhum braço
  `(Value::Array, Value::Array)` para `BinOp::Add`. Candidato a P720.

### 3.4 Re-verificação pós-commit (proveniência)

Executada no commit `3ab08fc54`, working tree limpa:

- `/tmp/p719-for-dict.typ` → mesmo resultado: `a=1 b=2 ("a", 1) ("b", 2) 1 2`.
- `cetz` → mesmo bloqueio (`cannot apply Add to array and array`), tempo
  real 52,4s (51,6s pré-commit — variação normal de ruído de máquina; o
  documento continua a falhar na fase de eval, antes de qualquer layout).

---

## 4. Gate das ADRs (critério do passo)

Grep a `for`/iteração/`Dict` em `00_nucleo/adr/*.md`: ocorrências
encontradas são genéricas e sem relação ao mecanismo de `for`-loop deste
passo (ex.: "for invisível" em ADR-0107 §funcional, menções a
`Dict`/iteração noutros contextos de layout/introspecção não
relacionados com o mecanismo de eval de `for`). ADR-0107/0108/0109 não
mencionam o tema. **Nada contradiz o decidido.**

---

## 5. Critério de fecho do passo

- [x] Sonda completa — comportamento confirmado (ordem de inserção,
  forma de um nome liga o par inteiro, aridade errada); estrutura de
  iteração existente para array confirmada e reaproveitada.
- [x] Implementado e testado (6 testes novos, confirmados a falhar por
  reversão temporária antes da implementação).
- [x] Sem regressão — `cargo test --workspace` verde (3912/0); iteração
  sobre array (P540, incluindo `.enumerate()`) confirmada intacta.
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (tempo 51,6s, exit 1,
  próximo bloqueio com `file:line`: `path-util.typ:423,430`,
  `bezier.typ:413`, `hobby.typ:77,78,126` — `array + array`).
- [x] Grep às ADRs pelos termos centrais — nada contradiz (§4).
- [x] Relatório com hash do commit.
