# Relatório P730 — `Array.slice(start, end, count)`

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-730.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir).
**Commit:** `98134380b420f0d4a74d2bf49c87b7dcaa8229eb`
**Proveniência das medições (regra de proveniência):** commit base `baf08758e22d55442313c5321755b4d23ce73d9f` ("P729: preenche hash do commit no relatório"), working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/stdlib/collections.md`, `01_core/src/rules/stdlib/collections.rs`, `01_core/src/rules/eval/tests.rs`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T21:54Z).

---

## Sonda — medições antes de decidir (ADR-0108)

`Array.slice` era o bloqueio do cetz registado em P728/P729 (erro "campo desconhecido em array: 'slice'"; a stdlib só tinha `slice` para `Str`, `collections.rs:79`).

### Assinatura completa no vanilla (`/tmp/p730-slice.typ`, pdftotext)

```
(1,2,3,4).slice(1, 3)         → (2, 3)
(1,2,3,4).slice(-2)           → (3, 4)
(1,2,3,4).slice(1)            → (2, 3, 4)
(1,2,3,4).slice(0, count: 2)  → (1, 2)
(1,2,3,4).slice(-3, -1)       → (2, 3)
().slice(0)                   → ()
```

### Casos-limite e erros (medições vanilla, `/tmp/p730-slice2..6.typ`)

```
(1,2,3,4).slice(1, count: -1)    → ()                       (end efetivo 0 → max(start) → vazio)
(1,2,3,4).slice(3, 1)            → ()                       (end < start → vazio, sem erro)
(1,2,3,4).slice(4)               → ()                       (start == len admitido)
(1,2,3,4).slice(0, 4)            → (1, 2, 3, 4)             (end == len admitido)
(1,2,3,4).slice(1, 2, count: 2)  → Err "`end` and `count` are mutually exclusive"
(1,2,3,4).slice(10)              → Err "array index out of bounds (index: 10, len: 4)"
(1,2,3,4).slice(0, count: 99)    → Err "array index out of bounds (index: 99, len: 4)"
```

### Fonte do vanilla

`lab/typst-original/crates/typst-library/src/foundations/array.rs:279-300`:

```rust
if end.is_some() && count.is_some() {
    bail!("`end` and `count` are mutually exclusive");
}
let start = self.locate(start, true)?;
let end = end.or(count.map(|c| start as i64 + c));
let end = self.locate(end.unwrap_or(self.len() as i64), true)?.max(start);
Ok(self.0[start..end].into())
```

sobre `locate(index, end_ok: true)` (`array.rs:122-137`): negativo → `len + index` (`checked_add`); admite `index == len`; erro `out_of_bounds(index, len)` com o **índice original**. A verificação de exclusão mútua vem **antes** de qualquer `locate`.

### Estado do cristalino

`/tmp/p730-slice.typ` → erro "campo desconhecido em array: 'slice'" (exit 1), confirmando que o dispatcher não reconhecia o par `(Array, "slice")`.

### Critério de fecho da sonda

- [x] Assinatura completa confirmada (posicionais, `count:` nomeado, índices negativos, array vazio, erros com mensagens exactas).

## L0 (Prompt)

`00_nucleo/prompts/rules/stdlib/collections.md` — linha `slice` na tabela de métodos de array, nova secção "Semântica de `slice` (P730)" com o mecanismo vanilla file:line, tabela de medições (incluindo erros exactos) e o consumidor cetz; linha "Atualizado em" actualizada. `crystalline-lint .` → **0 violations** (o ficheiro não usa `@prompt-hash`; nada a sincronizar — "Nothing to fix").

## Implementação

`01_core/src/rules/stdlib/collections.rs`:

1. Braço `(Value::Array(arr), "slice") => Some(array_slice(arr, args))` no dispatcher `try_dispatch_collection_method` (mesmo mecanismo de `array.at`, P714).
2. `fn array_slice(arr: Vec<Value>, args: Args) -> SourceResult<Value>` — mirror do vanilla: parsing de args no padrão de `str_slice` (start posicional; end posicional **ou** nomeado; count nomeado); exclusão mútua primeiro; closure `locate` local (negativo → `len + index` via `checked_add`, admite `== len`, erro com o índice original e a mensagem exacta do vanilla); `end` omitido → `len`; `count` → `start_resolvido + count`; clamp `max(start)`; fatia `arr[start..end]`.

## Validação

- Fail-first confirmado: `cargo test -p typst-core p730` antes da implementação → `error[E0425]: cannot find function array_slice` (5 ocorrências); os E2E medidos via CLI falhavam com "campo desconhecido em array: 'slice'".
- Depois: **14 passed, 0 failed** (10 unitários em `collections.rs` cobrindo todos os casos da sonda + 4 E2E em `tests.rs`: dispatch, idioma cetz `pts.slice(0, 2)`, `count:`+negativo, e `str.slice` sem regressão).
- `cargo test --workspace` — **4717 passed, 0 failed** (4022 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes).
- `crystalline-lint .` — **0 violations**.
- E2E do passo (`/tmp/p730-slice.typ`): cristalino produz `(2, 3) (3, 4) (2, 3, 4) (1, 2) (2, 3) ()` — **idêntico ao vanilla, linha a linha**.

### cetz — avançou; novo bloqueio registado (número exacto)

Com a reprodução exacta do passo (`line((0,0),(2,1))` + `circle((0,0))`), a compilação passa de `array.slice` e pára em:

```
/tmp/p730-cetz.typ:<detached>: error: import: a fonte tem de ser um caminho string ou um módulo, recebeu dictionary
```

Investigação (ADR-0108, caso mínimo **sem cetz**):

| Caso | Vanilla | Cristalino |
|---|---|---|
| `type(calc)` | `module` | `dictionary` |
| `#import calc: min, max` | compila | erro "import: a fonte tem de ser um caminho string ou um módulo, recebeu dictionary" (`modules.rs:187` exige `Value::Module`) |

Os módulos stdlib (`calc`, e candidatos `sys`/`math`/outros a verificar) estão expostos como `Dict` no scope raiz; o cetz faz `import calc: min, max` em `aabb.typ:18` (corpo de função, no caminho do bounds de `line`). Variações medidas: `#import cetz.draw: *` top-level e dentro do canvas compilam (exit 0); o erro só aparece com `line` no canvas. Registado como aberto em `achados-adiados-cetz.md` — **candidato natural a P731**.

**O diff de pixels final não é medível neste passo** (compilação pára no novo bloqueio) e a cadeia P678-730 **não fecha ainda**: P730 removeu o bloqueio `array.slice` e expôs o seguinte. Número registado conforme o critério do passo: erro exacto + caso mínimo medido sem cetz.

## Campos fixos cetz (estado após P730)

- `line` + `circle` no canvas: ambas as closures correm; `line` avança por `pts.slice(0, 2)` (`shapes.typ:620`) e pára no bounds (`aabb.typ:18`, `import calc`).
- `circle` sozinho (com e sem `import cetz.draw: *` no canvas): compila (exit 0).
- `#import cetz.draw: *` top-level: compila.

## Critério de fecho do passo

- [x] Sonda completa, assinatura confirmada.
- [x] Implementado e testado (todos os casos da sonda + erros).
- [x] Sem regressão em `cargo test --workspace` (4717 passed, 0 failed); `Str.slice` sem regressão (teste dedicado).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — próximo bloqueio registado com mensagem exacta, caso mínimo sem cetz e file:line (diff de pixels ainda não medível).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p730.md`, com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (e novo achado `calc` dictionary registado).
- [ ] Resumo completo da cadeia P678-730 — **não aplicável**: a cadeia não fechou (bloqueio `calc` dictionary aberto).
