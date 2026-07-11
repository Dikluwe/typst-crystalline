# Paridade Produção — P700 — Validação real: `cetz_core.wasm` e `#import "@preview/cetz:0.5.2"`

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-700.md`
**Hash do commit (este relatório):** a preencher no commit seguinte.
**HEAD base:** `e3e886f4e` (fim de P699b, detached HEAD).
**Estado:** SONDA FECHADA COM PRÓXIMO BLOQUEIO IDENTIFICADO — não é sucesso completo, não é regressão. Cadeia P678–700 pausada aqui, com o próximo passo definido.

---

## 1. Proveniência das medições

- Working tree no momento da medição: nenhum ficheiro de código alterado por
  este passo (sonda pura) — `git status` idêntico ao fim de P699b mais o
  backlog untracked já conhecido.
- `~/.cache/typst/packages/preview/cetz/0.5.2` já presente localmente (fetch
  de sessão anterior, confirmado por P686/P696) — `cetz_core.wasm` real,
  343529 bytes.
- `cargo test --workspace` e `crystalline-lint .` corridos **antes** da sonda
  para confirmar baseline limpo (herdado de P699b: 3728 passed, 0 violations).

---

## 2. Resultado — chamada isolada a `cetz_core.wasm`: SUCESSO

```
#let core = plugin("cetz-core/cetz_core.wasm")
#(type(core))
```
→ exit 0, `pdftotext` → `module`.

```
#let core = plugin("cetz-core/cetz_core.wasm")
#(type(core.aabb_func))
```
→ exit 0, `pdftotext` → `function`.

**Confirmado:** o plugin real do `cetz` (não um `hello.wasm` de teste) carrega
e expõe os seus exports corretamente através do mecanismo de P699/P699b. O
runtime WASM (`wasmi`, P698) e o despacho de `Module`/`PluginFunc` (P699)
funcionam com um binário WASM real de produção, não só com o fixture
hand-rolled dos passos anteriores.

---

## 3. Resultado — `#import "@preview/cetz:0.5.2"` completo: BLOQUEADO

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```
→ exit 1: `error: esta função não tem campos`.

### 3.1 Isolamento da causa

`cetz` chama plugins através de `src/wasm.typ`:
```typst
#let call_wasm(func, args) = {
  let encoded = cbor.encode(args)
  cbor(func(encoded))
}
```
Reproduzido isoladamente:
```
#let encoded = cbor.encode((pts: ((0.0, 0.0), (1.0, 1.0)), init: none))
```
→ **mesmo erro**, `esta função não tem campos`, na linha de `cbor.encode`.

**Causa raiz confirmada:** `01_core/src/rules/eval/mod.rs:1047` regista
`cbor` como `Value::Func(Func::native("cbor", native_cbor))` — um `Func`
plano, sem campos. `native_cbor` (`loading.rs:372`, via macro
`native_loader!`) só aceita **caminho** (`arg_path`), nunca bytes em memória.
Não existe `cbor.encode` no cristalino.

### 3.2 Confirmação de paridade vanilla

`lab/typst-original/crates/typst-library/src/loading/cbor.rs`:
- `#[func(scope, title = "CBOR")] pub fn cbor(source: Spanned<DataSource>)`
  — aceita **caminho OU bytes crus** (`DataSource` é enum `Str | Bytes`,
  via `Load`).
- `#[scope] impl cbor { pub fn encode(value: Spanned<Value>) -> SourceResult<Bytes> }`
  — serializa **qualquer** `Value` para CBOR via `ciborium::into_writer`.

Isto não é feature nova do vanilla — é o mecanismo pelo qual `cetz` (e,
presumivelmente, outros pacotes que usam plugins WASM com payloads
estruturados) passa dados complexos para lá e para cá do plugin. `aabb.typ`,
`bezier.typ` e `matrix.typ` — os 3 consumidores de `cetz-core` identificados
por P696 — usam todos `call_wasm`, logo todos batem neste bloqueio.

### 3.3 Achado lateral (não bloqueia `cetz`, registado por precaução)

`bytes(())` (construtor literal) → `error: type bytes does not have a
constructor`. `cetz` não usa este construtor directamente (só produz/consome
`Bytes` via `cbor.encode`/plugin), por isso **não** faz parte do bloqueio
crítico — mas é outro ponto de paridade em falta na mesma família (ADR-0111).
Não investigado a fundo; registo é suficiente para não perder o achado.

---

## 4. Porque isto não é uma "correcção pontual"

O passo permite correcção directa "se algo pequeno falhar". Isto não é
pequeno:

- Implica mudar a **representação** de `cbor` no scope global, de `Value::Func`
  para um valor com campos (mesmo padrão que `str`/`int`/`float`/`type` já
  usam desde P685 — `Value::Type` chamável com despacho especial — mas aqui
  precisa de um *método*, `encode`, não de uma chamada directa alternativa).
- Implica um `native_cbor` que aceite **Bytes ou caminho** (o precedente
  `DataSource`/`Load` do vanilla não existe no cristalino; teria de se decidir
  se se generaliza para os outros loaders da família — `json`/`yaml`/`toml`/
  `xml` — ou se se resolve só para `cbor`, com risco de duplicar o padrão
  depois).
- Implica serialização **genérica** de `Value` para CBOR (`cbor.encode`),
  espelhando a tabela de conversão do vanilla (`Value` → CBOR: symbol→text,
  content→map, outros→texto via `repr`) — trabalho de superfície razoável,
  não um one-liner.

Isto é exactamente o tipo de decisão que a Trava Arquitetural do CLAUDE.md
pede para passar por L0 novo antes de código — e o próprio P700 pede para
**não forçar uma correcção improvisada só para "fechar"**.

---

## 5. Estado da cadeia P678–700

**Pausada aqui, com o próximo passo claro — não fechada por sucesso, não
regressiva.**

- Plugins WASM puros (âmbito mínimo de P696): **validados com binário real**
  (`cetz_core.wasm`), não só com fixtures de teste — isto **é** progresso
  novo desta sessão, mesmo sem o pacote completo renderizar.
- O bloqueio restante é **preciso e único**: `cbor.encode` + `cbor(bytes)`.
  Não há mistério por trás — os 3 consumidores de `cetz-core` usam todos o
  mesmo `call_wasm`, logo um único ponto de trabalho desbloqueia os três.
- **Não medido** (por não ter chegado lá): tempo de compilação de `cetz`
  completo, comparação visual com P688. Fica para quando o bloqueio de
  `cbor` estiver resolvido.

### Próximo passo sugerido (P701, não iniciado)

1. L0 novo ou revisão de `00_nucleo/prompts/rules/stdlib/loading.md` (ou
   ficheiro dedicado a `cbor`) especificando: `cbor` como scope-value com
   `.encode`; `native_cbor` aceitando `Bytes` além de caminho.
2. Decidir explicitamente (medir antes de decidir, ADR-0108): generalizar a
   aceitação de `Bytes` a `json`/`yaml`/`toml`/`xml` já nesse passo, ou
   scope-out explícito para só `cbor` agora.
3. Reexecutar exactamente a reprodução do §3 deste relatório como critério de
   fecho.

---

## 6. Verificação (números com proveniência do §1)

- `cargo test --workspace` → 3728 passed, 0 failed (sem alteração de código
  nesta sonda).
- `crystalline-lint .` → 0 violations (sem alteração de código nesta sonda).

## 7. Critério de fecho do passo — estado final

- [x] Chamada isolada a `cetz_core.wasm` confirmada (module + function reais).
- [x] `cetz` re-testado com honestidade — **próximo bloqueio identificado e
      isolado** (`cbor.encode`), não sucesso completo.
- [ ] Tempo de compilação — não aplicável (compilação falha antes do PDF).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório com resultado exacto (este ficheiro).
- [x] Estado da cadeia P678–700 declarado com precisão — pausada, próximo
      passo (P701) definido.
