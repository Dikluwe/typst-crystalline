# Paridade Produção — P686 — Caminhos absolutos `/...` em `#import`/`#include`

**Estado:** fechado (resolução de `/...` implementada e validada; cetz desbloqueado no
ponto de path; próximo bloqueio medido é `gray`, fora de escopo).

**Commit do trabalho:** `__P686_COMMIT__`
**Commit base (HEAD antes deste passo):** `81034f7103b31dc7d5aa68a1ec2945ef5c97b16a`
**Hora da medição:** `2026-07-10T16:33:13-03:00` (saída de `date -Is`)
**Árvore:** detached HEAD; working tree com apenas 4 ficheiros tracked alterados
(untracked pré-existentes em `materialization/`, `adr/`, `diagnosticos/`, `temp_p*`
não foram tocados).

---

## Proveniência da medição (regra de P569/P574)

- **Código medido:** este commit `__P686_COMMIT__`, sobre a base `81034f710`.
- `git diff HEAD --stat` no momento da medição:

```
 00_nucleo/prompts/contracts/world.md    |  25 ++++++-
 00_nucleo/prompts/infra/system-world.md |  32 ++++++++-
 01_core/src/contracts/world.rs          |   2 +-
 03_infra/src/world.rs                   | 121 ++++++++++++++++++++++++++++++--
 4 files changed, 172 insertions(+), 8 deletions(-)
```

- `cargo test --workspace`: **4356 passed, 0 failed** (3687 + 610 + 28 + 2 + 27 + 2;
  3 doc-tests ignorados). P685 tinha 4352; **+4** = os 4 testes novos deste passo.
- `crystalline-lint .`: **0 violations**.
- Binários: cristalino `target/debug/typst` (CLI posicional; `--root` opcional);
  vanilla `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)`
  (precisa de `compile`).

---

## O problema (medido antes de decidir — ADR-0108)

Em P685, `cetz` falhava com `include: ficheiro não encontrado: /src/process.typ`.
A causa exacta (lida no código, não inferida):

- `03_infra/src/world.rs:315-321` (`read_bytes`) e `03_infra/src/world.rs:323-328`
  (`include_source`) faziam `directory_of(current_file).join(path)`.
- Em Rust, `PathBuf::join` com `path` começado por `/` **substitui** a base
  (comportamento de path absoluto do std). Logo `/src/process.typ` era procurado na
  **raiz do filesystem** (`/src/process.typ`), que não existe → `NotFound`.

**Sonda vanilla (medida):**

1. *Fora de pacote* — `/src/util.typ` resolve a `--root` (default = dir do main):
   `--root temp_p686` (um nível acima) → vanilla `file not found (.../temp_p686/src/util.typ)`;
   `--root projecto` → imprime `de util`; sem `--root` (root default = dir do main) → `de util`.
2. *Dentro de pacote* — cetz em `~/.cache/typst/packages/preview/cetz/0.2.2`; ficheiros
   em subdirs (`src/draw/grouping.typ:1`, `src/draw/shapes.typ:11`, `src/lib/axes.typ:6`)
   fazem `#import "/src/process.typ"` → resolve à **raiz do pacote**, não ao dir do ficheiro.

**Classificação (ADR-0107):** a regra "absoluto vs relativo" e a noção de "raiz de
pacote" são **semântica da linguagem** (resolução de módulos) → paridade exigida. A
forma em disco `{namespace}/{name}/{version}` e os roots de procura são **mecânica**
de L3 → divergem de propósito. Aceitação medida **ao nível da língua** (o path resolve
para o ficheiro certo), nunca por igualdade de bytes.

---

## O que foi feito

**L0 (Trava Arquitetural — antes de código):**

- `00_nucleo/prompts/contracts/world.md` — nova secção "Resolução de caminhos
  absolutos (`/...`) em `include_source` e `read_bytes` (P686)".
- `00_nucleo/prompts/infra/system-world.md` — nova secção "Resolução de caminhos
  absolutos (`/...`) — P686" (helpers `path_of`/`package_root_of`/`resolve_path`).
- `crystalline-lint --fix-hashes .` → actualizou `@prompt-hash`:
  `01_core/src/contracts/world.rs` → `a791b2fa`; `03_infra/src/world.rs` → `0bb63043`.

**Código (`03_infra/src/world.rs`):**

- `path_of(id) -> Option<PathBuf>` — path registado em `slots`.
- `package_root_of(&Path) -> Option<PathBuf>` — por **prefixo** sobre
  `package_candidate_dirs()`; se `{base}/{ns}/{name}/{version}/...`, devolve
  `{base}/{ns}/{name}/{version}` (sem I/O de rede; coerente com `resolve_package`).
- `resolve_path(current_file, path) -> PathBuf` — se `path.strip_prefix('/')`:
  base = `package_root_of(path_of(current_file))` ou `self.root` (projecto);
  `base.join(path sem '/')`. Senão: `directory_of(current_file).join(path)` (sem regressão).
- `read_bytes` e `include_source` passam a chamar `resolve_path`.

**Testes (`#[cfg(test)]` em `world.rs`, +4):**

- `system_world_include_source_absoluto_no_projecto` — `/src/util.typ` → root do projecto.
- `system_world_read_bytes_absoluto_no_projecto` — `/assets/data.bin` → root do projecto.
- `system_world_include_source_absoluto_em_pacote` — pacote fake
  `{tmp}/typst/packages/preview/foo/0.1.0`; `src/sub/x.typ` importa `/src/lib.typ` e
  resolve à **raiz do pacote** (não a `src/sub`). Usa `XDG_DATA_HOME` apontado para o
  temp dir (restaurado no fim).
- `system_world_include_source_relativo_sem_regressao` — `sub/x.typ` continua relativo.

---

## Validação cristalino vs vanilla

**Projecto (`/...` fora de pacote) — PARIDADE:**

| Compilador | comando | exit | texto extraído |
|---|---|---|---|
| vanilla | `compile --root temp_p686/projecto …/main.typ` | 0 | `de util` |
| cristalino | `--root temp_p686/projecto …/main.typ` | 0 | `de util` |
| cristalino | (sem `--root`, root default = dir do main) | 0 | `de util` |

`main.typ`: `#import "/src/util.typ": msg` / `#msg`; `src/util.typ`: `#let msg = [de util]`.

**Pacote (`/...` dentro de pacote) — fix verificado no cetz real:**

- Antes (P685): `include: ficheiro não encontrado: /src/process.typ`.
- Depois (P686): o erro de path **desapareceu**. `grep` sobre o output do cristalino
  para `não encontrado` / `not found` / `/src/` / `process.typ` → **vazio**. O pacote
  agora resolve `/src/...` à raiz do pacote e avança para o `eval`.
- Confirmado também pelo teste `include_source_absoluto_em_pacote` (raiz do pacote,
  não `src/sub`).

**cetz end-to-end — NÃO é gate deste passo (medido com honestidade):**

- Cristalino (`cetz.typ` mínimo): `error: unknown variable: gray` em `<detached>` —
  novo bloqueio **não relacionado** com paths (símbolo de cor `gray` não ligado no
  escopo de `eval` destacado; paridade de stdlib/cores). Registado como débito.
- Vanilla 0.15.0 (969087ec) **também não renderiza** o mesmo doc mínimo: falha em
  `canvas.typ:24` (`style(st => {`) / `coordinate.resolve-system` para `line`/`circle`
  trivial. Logo cetz 0.2.2 não produz PDF neste ambiente sequer no vanilla para docs
  mínimos — paridade end-to-end de cetz fica fora de escopo e não é critério de P686.

---

## Débitos (fora de escopo)

- `gray` desconhecido no `eval` destacado de cetz — paridade de builtins de cor /
  stdlib; investigar num passo próprio (não é resolução de caminho).
- Sandbox: caminhos relativos com `../` ainda podem escapar a raiz do pacote/projecto
  (pré-existente; não introduzido nem agravado por P686).
- cetz end-to-end: depende de múltiplos bloqueios a montante (`gray`, e no vanilla
  deste ambiente o próprio render mínimo falha); não é objectivo de P686.

## Conclusão

A resolução de caminhos absolutos `/...` está em paridade com a **linguagem** Typst
(ADR-0107): projecto (root/`--root`) e pacote (raiz `{ns}/{name}/{version}`), medido
com proveniência (ADR-0108). O bloqueio de cetz em `/src/process.typ` (P685) está
removido; o próximo bloqueio medido (`gray`) é independente e fica registado.
