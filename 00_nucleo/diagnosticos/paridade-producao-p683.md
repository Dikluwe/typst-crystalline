# P683 — `#import` a partir de módulo / field-access

**Estado:** fechado.
**Commit do trabalho:** `cacc26c54cc936effdb14e2f258b3c20243f5a00`.
**Data/hora da medição:** 2026-07-10T14:26:04-03:00.
**Commit base (HEAD antes do trabalho):** `c447f17c39db98d6831d3bb1508d6843db8754fc` — "P682: adiciona hash do commit ao relatório".
**Vanilla de referência:** `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)`.

## Proveniência da medição

- `cargo test --workspace` → **4337 passed, 0 failed** (soma de todos os `test result:`). P682 tinha 4333; os **+4** são os novos testes de import por módulo.
- `crystalline-lint .` → `✓ No violations found`. `crystalline-lint --fix-hashes .` actualizou o header dos 10 ficheiros `rules/eval/*` para `@prompt-hash 79decd9b` (0 drift).
- `git diff HEAD --stat` (estado exacto que gerou os números):

```
 00_nucleo/prompts/engine/eval.md        |  47 +++++++++++-
 01_core/src/engine/eval/modules.rs      | 127 ++++++++++++++++++++-------------
 01_core/src/engine/eval/tests.rs        |  60 +++++++++++++++-
 ... (8 ficheiros eval/* só com @prompt-hash, 2 linhas cada)
 11 files changed, 190 insertions(+), 60 deletions(-)
```

Binário usado: `target/debug/typst` (debug build do commit de trabalho).

## Sonda — formas de fonte em `#import` (medido)

| Forma | Vanilla 0.15.0 | Resultado |
|-------|----------------|-----------|
| `#import "a.typ" as m` + `#import m: valor` | ✅ | `valor` ligado |
| `#import "a.typ" as p` + `#import p.valor` (valor = string, não-módulo) | ❌ | `file not found (searched at …/<valor>)` — vanilla usa o valor como caminho |
| `cetz/src/deps.typ` | — | `#import "@preview/oxifmt:0.2.0"` (bare) → `deps.oxifmt` é o módulo `oxifmt` via field-access |

Conclusão (ADR-0108): o padrão real de `cetz` é **fonte que resolve para `Value::Module`**
(identificador `util`, ou field-access `deps.oxifmt`) com `: items`. É **sintaxe/morfologia**
(paridade); o texto do erro e o algoritmo são **mecânica** (divergem, P329).

## Implementação

`eval_module_import` (`01_core/src/engine/eval/modules.rs`) foi reestruturado para resolver a
fonte para `(Module, default_bind_name)` antes de aplicar os bindings:

- `Expr::Str` → fluxo P679/P681 (ficheiro local / pacote `@preview`), inalterado; nome por
  omissão = `bare_name()` (file_stem).
- qualquer outra expressão → `eval_expr(source_expr, scopes, ctx, engine)`:
  - `Value::Module(m)` → usa `m` directamente (sem I/O, sem ciclo, sem `eval_imported_file`);
    nome por omissão = `m.name()`.
  - outro `Value` → erro `import: a fonte tem de ser um caminho string ou um módulo, recebeu {tipo}`.
- a aplicação de bindings (`None`/`Wildcard`/`Items`) é a mesma; o bare import liga sob
  `new_name` (`as`) ou o nome por omissão.

Reutiliza o suporte já existente a `Value::Module` em field-access (`bindings.rs:636`, P679) e
a resolução de `Expr::Ident` no scope. L0 `eval.md` ganhou a secção **§P683**.

## Validação (cristalino `target/debug/typst` vs vanilla)

| Caso | Cristalino | Vanilla | Paridade |
|------|-----------|---------|----------|
| `#import "a.typ" as m` + `#import m: valor, saudacao` → `#valor / #saudacao("Mundo")` | `de A / Olá, Mundo` | `de A / Olá, Mundo` | ✅ idêntico |
| field-access estilo cetz (`deps.inner: strfmt`) | `strfmt-ok` | — | ✅ (forma coberta) |
| `#import x: foo` com `x = 5` (não-módulo) | `import: a fonte tem de ser um caminho string ou um módulo, recebeu int` (span `bad.typ:2:9`) | `file not found` | ✅ ao nível de "é erro" (ADR-0107) |
| `#import "a.typ": saudacao` (string, P679) | exit 0, PDF | — | ✅ sem regressão |

Testes novos (4): `import_modulo_por_identificador`, `import_modulo_field_access`,
`import_modulo_bare_por_identificador`, `import_fonte_nao_modulo_retorna_err`.

## cetz re-testado — avança mais; próximo problema registado

```typst
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({ draw.line((0,0), (1,1)) })
```

- **P681:** `pacote … não encontrado` (resolução) → corrigido em P681.
- **P682:** `version(): … recebeu 1` (linha 1) → corrigido em P682.
- **P683 (antes):** `import: caminho deve ser uma string literal` (`util.typ:2`, `anchor.typ:5`) → **corrigido agora**.
- **P683 (depois):** avança e falha com **`error: unknown variable: length`**.

Origem (medida no pacote): cetz usa o **tipo `length` como valor** em comparações de tipo —
ex.: `src/coordinate.typ:75` → `if type(anchor) == length {` e `src/canvas.typ:33` →
`type(length) in (typst-length, ratio)`. No vanilla, `length`/`ratio`/`int`/… são **tipos
acessíveis como valores** no scope global; no cristalino, `length` não está ligado como valor
(só `type()` existe), logo `type(x) == length` falha com `unknown variable: length`. **É o
próximo débito de linguagem**, fora do scope de P683 (não se assume cetz resolvido).

## Débitos que permanecem (não são P683)

- **Tipos como valores (`length`, `ratio`, …)** — `type(x) == length` exige `length` ligado no
  scope como valor-tipo. Próximo bloqueio de cetz (`coordinate.typ:75`, `canvas.typ:33`).
- **`#import pacote.valor` (valor não-módulo)** — o vanilla trata como caminho; o cristalino
  recusa com erro claro. Paridade ao nível de "é erro", não de texto (ADR-0107).

## Ficheiros tocados (commit)

- `00_nucleo/prompts/engine/eval.md` (L0: §P683)
- `01_core/src/engine/eval/modules.rs` (fonte-módulo em `eval_module_import`)
- `01_core/src/engine/eval/tests.rs` (+4 testes)
- 8 ficheiros `01_core/src/engine/eval/*.rs` — só `@prompt-hash` (fix-hashes)
