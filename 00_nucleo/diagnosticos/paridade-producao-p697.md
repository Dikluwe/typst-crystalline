# Paridade produção — P697 — builtin `plugin()`: sintaxe e leitura de bytes (nível 2 de P696)

**Commit deste passo:** `__P697_COMMIT__` (preenchido no 2º commit; ver §Proveniência).
**Passo:** `00_nucleo/materialization/typst-passo-697.md`. **Tamanho:** S.
**ADRs:** ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
ADR-0109 (feature no seu ficheiro), ADR-0111 (leitura reusa `World::read_bytes`).
**Dependências:** P696 (sonda/mapa), P679/P686 (leitura de ficheiros).

---

## Proveniência das medições (regra de proveniência)

- **Estado medido:** working tree **não commitado** sobre `HEAD = 78c44c1b5726ff85f5ac25bc63c9503cdb3c98ff` (P696).
- **Hora:** `2026-07-10T22:13:59-03:00` (ambiente).
- **Ficheiros alterados no momento da medição** (`git diff HEAD --stat`):

```
 01_core/src/rules/eval/mod.rs   | 4 ++++
 01_core/src/rules/stdlib/mod.rs | 4 ++++
 2 files changed, 8 insertions(+)
```

Novos ficheiros (não no diff acima): `00_nucleo/prompts/rules/stdlib/plugin.md`,
`01_core/src/rules/stdlib/plugin.rs`, este relatório. A fixture `hello.wasm`
(192 B) é a de P696, regenerada em scratch e apagada no fim.

## Objectivo

Primeiro passo da divisão de P696: fixar `plugin(path|bytes)` como builtin e
**ler os bytes** do `source`, reaproveitando `World::read_bytes` (a mesma de
`read()`/`#include`/`#import`). O runtime WASM fica para P698 — por isso, após
ler os bytes, o builtin devolve um **erro provisório claro** (não um `Module`),
provando que a leitura funciona sem fingir o resto.

## Decisões (com medição — ADR-0108)

1. **Feature no seu ficheiro** (ADR-0109): `01_core/src/rules/stdlib/plugin.rs`
   com `native_plugin`, registado no scope global via `make_stdlib`
   (`scope.define("plugin", …)`), L0 próprio `rules/stdlib/plugin.md`.
2. **Leitura reusa `World::read_bytes`** (sem I/O em L1, sem duplicar
   resolução): `Value::Str(path)` → `world.read_bytes(current_file, path)`
   (relativo ao ficheiro actual; `/...` à raiz do pacote/projecto, P686);
   `Value::Bytes(b)` → usa os bytes directos. A not-found usa a **forma** do
   helper de `loading.rs` (`"plugin(): não foi possível ler 'X': …"`), para ser
   do mesmo tipo que `read()`/`#import`.
3. **Erro provisório distinto:** após ler os bytes, devolve
   `plugin: runtime WASM ainda não implementado (P697); {n} bytes lidos com
   sucesso`. Isto separa três observáveis (mensagem = mecânica observável,
   ADR-0107): `unknown variable: plugin` (antes), `não foi possível ler`
   (inexistente), e o provisório (lido com sucesso).

## Medições E2E (cristalino; mensagem em stderr)

| Prova | Resultado | Confirma |
|-------|-----------|----------|
| `plugin("hello.wasm")` | `plugin: runtime WASM ainda não implementado (P697); 192 bytes lidos com sucesso` | leitura relativa OK; erro provisório, **não** `unknown variable` |
| `plugin("nao-existe.wasm")` | `plugin(): não foi possível ler 'nao-existe.wasm': erro ao ler … No such file or directory (os error 2)` | not-found na forma de `read()`/`#import` |
| `plugin(read("raw.bin"))` (bytes não-UTF8) | `plugin: runtime WASM ainda não implementado (P697); 3 bytes lidos com sucesso` | braço `Value::Bytes` |
| `plugin("/pkg.wasm")` dentro de pacote fake | `plugin: runtime WASM ainda não implementado (P697); 192 bytes lidos com sucesso` | `/...` resolve à **raiz do pacote** (P686) — forma do caminho real de `cetz` (`/cetz-core/cetz_core.wasm`) |

**Achado ortogonal (honestidade):** `bytes("abc")` literal falha com
`type bytes does not have a constructor` — o cristalino tem `bytes` como
**tipo** (`Value::Type(Type::Bytes)`, P685) mas **não** como construtor
chamável. A capacidade de `plugin` aceitar `Value::Bytes` está implementada e
testada (unit + E2E via `read()`); só a *forma literal* `bytes(...)` não existe
aqui. É débito separado (construtor `bytes()`), fora do escopo de P697, e não
bloqueia o caminho real (pacotes passam bytes lidos, não literais).

## Testes e lint

- 4 testes novos em `rules/stdlib/plugin.rs`: bytes directo (contagem), caminho
  existente (leitura OK), caminho inexistente (erro de leitura), tipo/aridade
  errados.
- `cargo test --workspace`: **4390 passed / 0 failed** (3716 + 610 + 33 + 2 + 27
  + 2). P695 era 4386; o delta +4 são os testes de `plugin`. **Sem regressão.**
- `crystalline-lint .`: **0 violations** (após `--fix-hashes`; `native_plugin`
  reusa `World::read_bytes` — L1 não faz I/O; sem V3/V4/V14).

## Língua vs mecânica (ADR-0107)

Paridade (linguagem): `plugin(path|bytes)` é builtin chamável; leitura
relativa/`/...` igual a `read()`/`#import`; erro de ficheiro inexistente na
mesma forma. Diverge por agora (semântica): devolve erro provisório em vez de
`Module` funcional — fica para P698.

## Próximo passo

P698 (nível 3): host `wasmi` em L3 atrás de trait `PluginHost`, protocolo
`typst_env` completo, testado com `hello.wasm`; `native_plugin` passa a devolver
`Module` real.
