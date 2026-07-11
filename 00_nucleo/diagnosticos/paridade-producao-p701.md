# Paridade Produção — P701 — `cbor` com `.encode`, `cbor(bytes)` além de caminho

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-701.md`
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `9720dfcc0` (fim de P700, detached HEAD).
**Estado:** ÂMBITO DE P701 FECHADO — `cbor.encode`/`cbor(bytes)` implementados, testados, paridade vanilla confirmada. `cetz` **continua bloqueado**, mas por um gap diferente e não relacionado (`Func::with`), só visível depois de desbloquear `cbor`.

---

## 1. Proveniência das medições

- Working tree no momento da medição: 4 ficheiros de código+L0 (`loading.md`,
  `loading.rs`, `mod.rs` de stdlib, `mod.rs` de eval) + este relatório.
- `~/.cache/typst/packages/preview/cetz/0.5.2` idêntico a P700 (mesmo fetch de
  sessão anterior).
- `cargo test --workspace` corrido antes (herdado de P700: 3728 passed) e
  depois (3736 passed — +8 testes novos de P701) da implementação.

---

## 2. Sonda — confirmado contra o vanilla, com `file:line`

- **Tabela `Value → CBOR`**: `foundations/value.rs:345-366` (`impl Serialize
  for Value`) + `foundations/bytes.rs:364-374` (`impl Serialize for Bytes`,
  byte-string em contexto não-human-readable). Casos dedicados vanilla:
  `Symbol`/`Content` têm `Serialize` próprio; tudo o resto cai no fallback
  `serializer.serialize_str(&other.repr())` (`value.rs:363`).
- **Round-trip vanilla** confirmado com documento `.typ` real:
  ```
  #let bytes_codificados = cbor.encode((a: 1, b: "texto", c: (1, 2, 3)))
  #type(bytes_codificados)
  #cbor(bytes_codificados)
  ```
  → `bytes (a: 1, b: "texto", c: (1, 2, 3))` (vanilla, `lab/typst-original`).
- **Cristalino antes do fix**: mesmo documento → `error: esta função não tem
  campos` (reprodução exacta do bloqueio isolado por P700).
- **Decisão de âmbito**: generalizar `Bytes` aos 5 loaders que partilham a
  macro `native_loader!` (`json`/`yaml`/`toml`/`cbor`/`xml`) — confirmado que
  todos os 5 usam `DataSource` (`Str | Bytes`) no vanilla
  (`loading/{json,yaml,toml,cbor,xml}.rs`, `source: Spanned<DataSource>`), e
  que no cristalino partilham literalmente a mesma macro (`loading.rs:353`),
  logo é uma mudança única, não 5 repetidas. `read`/`csv` não tocados (função
  própria, fora do âmbito medido).

---

## 3. Implementação

### 3.1 `cbor` — de `Value::Func` plano para namespace

`rules/eval/mod.rs`: `cbor` passa de
`Value::Func(Func::native("cbor", native_cbor))` para
`Func::native_with_namespace("cbor", native_cbor, Arc::new(ns))` com
`ns.define("encode", Value::Func(Func::native("cbor.encode",
native_cbor_encode)))`. **Mesmo padrão já em uso** para `curve`/`grid`/`table`
(P512/P513/P493b) — zero mudança no dispatch de field access
(`rules/eval/bindings.rs:618`, já tratava `Func` com namespace); só o registo
do valor mudou.

### 3.2 `resolve_data` — path OU bytes, um só ponto de mudança

`01_core/src/rules/stdlib/loading.rs`: nova função `resolve_data` substitui
`arg_path` dentro da macro `native_loader!` — aceita `Value::Str` (lido via
`World::read_bytes`, como antes) ou `Value::Bytes` (usado directamente, zero
I/O). `arg_path` mantido intacto para `native_read` (não tocado).

### 3.3 `value_to_cbor` — `Value` → CBOR

Nova função em `loading.rs`, direcção inversa de `cbor_to_value` já existente.
Tabela implementada: `None→Null`, `Bool`, `Int→Integer`, `Float`, `Str→Text`,
`Bytes→Bytes` (byte-string), `Array` (recursivo), `Dict→Map` (chave sempre
`Text`, ordem `IndexMap` preservada). **Divergência documentada** (não
lacuna): `Symbol`/`Content` não ganham serialização dedicada — caem, com todo
o resto, no fallback `Text(repr_value(v))`, igual ao "other" do vanilla.
Justificação: nenhum consumidor real medido (`cetz`) usa `cbor.encode` com
esses dois tipos.

### 3.4 Testes (8 novos, `loading.rs::tests`)

`cbor_encode_tipos_basicos_ida_e_volta`, `cbor_encode_bytes_como_byte_string_nao_texto`,
`cbor_encode_tipo_opaco_cai_em_repr`, `cbor_encode_requer_exactamente_1_arg`,
`native_json_aceita_bytes_alem_de_path`, `native_cbor_aceita_bytes_alem_de_path`,
`native_json_path_continua_a_funcionar` (regressão do caminho já suportado),
`native_json_tipo_invalido_erro`.

---

## 4. Validação — âmbito de P701 confirmado

```
#let bytes_codificados = cbor.encode((a: 1, b: "texto", c: (1, 2, 3)))
#type(bytes_codificados)
#cbor(bytes_codificados)
```
→ **cristalino, pós-fix**: `bytes (a: 1, b: "texto", c: (1, 2, 3))` —
**idêntico ao vanilla** medido em §2.

- `cargo test --workspace` → **3736 passed**, 0 failed (3728 de P700 + 8
  novos de P701); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (drift V5 do L0 realinhado com
  `--fix-hashes`, hash `loading.rs` `3d5d7f23` → `43c9012d`).

---

## 5. Repetição da reprodução de P700 — `cetz` continua bloqueado, por outra causa

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```
→ **ainda falha**: `error: esta função não tem campos`. Mesma mensagem de
P700, mas **não é mais o bug de `cbor`** — isolado passo a passo:

1. `cbor.encode`/`cbor(bytes)` isolados: **funcionam**, confirmado (§4).
2. `#import "@preview/cetz:0.5.2"` **sozinho** (sem chamar `canvas`): já
   falha com o mesmo erro.
3. Isolado a `matrix.typ` (importado directamente por `lib.typ:8`, um dos 3
   consumidores de `cetz-core`): falha sozinho.
4. Causa raiz isolada — `matrix.typ:8`: `#let _round =
   calc.round.with(digits: precision)`. Reproduzido minimamente:
   ```
   #let f = calc.round.with(digits: 2)
   #(type(f))
   ```
   → `error: esta função não tem campos`.

**`Func::with(...)` (aplicação parcial de argumentos) não existe no
cristalino.** Isto é uma *feature de linguagem* do Typst — qualquer função
(nativa, closure, elemento) suporta `.with(args)` devolvendo uma nova função
com esses argumentos pré-ligados — **não é específico de `cbor`, `plugin` ou
`cetz`**. Estava escondido atrás do bloqueio de `cbor` até agora: P700 nunca
chegou a `matrix.typ:8` porque `wasm.typ`/`aabb.typ`/`bezier.typ` batiam em
`cbor.encode` primeiro na ordem de avaliação de `lib.typ`. Não encontrado em
nenhum L0 existente (`grep -rn ".with(" 00_nucleo/prompts/` só devolve
`f_fronteira_e1.md:190`, sobre `it.with(..)` em `Content`/`#show`, um
mecanismo diferente — field-access em elemento, não aplicação parcial de
`Func`).

**Não corrigido neste passo** — fora do âmbito de P701 (que era
especificamente `cbor.encode`/`cbor(bytes)`, já confirmado e fechado acima).
Mesma disciplina de P700: registar o bloqueio com precisão, não forçar uma
correcção improvisada sob um âmbito que não é o dela.

### Próximo passo sugerido (P702, não iniciado)

1. Confirmar no vanilla como `Func::with` funciona (provavelmente injecta um
   novo `FuncRepr`/wrapper de closure que pré-preenche `Args` e delega à
   função original na chamada).
2. L0 novo para `Func::with` — provavelmente em `entities/func.md`.
3. Reexecutar a reprodução deste §5 como critério de fecho; se passar,
   repetir a reprodução completa de `cetz` (P700/P701) para ver se aparece
   mais algum bloqueio adiante.

---

## 6. Estado da cadeia P678–701

Progresso real e verificado: `cbor.encode`/`cbor(bytes)` (P701) e plugin WASM
com binário real (P699/P699b/P700) estão fechados e testados. `cetz` continua
sem renderizar — não por regressão nem por um bloqueio antigo reaberto, mas
porque desbloquear `cbor` revelou o bloqueio seguinte na cadeia de imports de
`lib.typ`. Pausada aqui, com P702 sugerido e a causa exacta já isolada
(`matrix.typ:8`, `calc.round.with`).

## 7. Critério de fecho do passo

- [x] Sonda completa, tabela de conversão e comportamento confirmados contra
      o vanilla (`file:line`).
- [x] `cbor.encode`/`cbor(bytes)` implementados e testados (8 testes novos).
- [x] Âmbito decidido e documentado (generalizar aos 5 loaders da macro
      partilhada; `read`/`csv` fora).
- [x] `cetz` re-testado — **próximo bloqueio identificado e isolado**
      (`Func::with`, não relacionado a `cbor`), não sucesso completo.
- [x] Sem regressão em `cargo test --workspace` (3736 passed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório com resultado exacto (este ficheiro).
