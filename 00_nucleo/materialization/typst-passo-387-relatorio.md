# Passo 387 — relatório: materialização do data-loading

**Tipo:** materialização (L1 + composição L3). **Data:** 2026-06-21. **HEAD:** `08fed76d3`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se o cluster **`loading`** (data import) — `read` + 6 parsers
(`csv`/`json`/`yaml`/`toml`/`cbor`/`xml`) — fechando o único achado líquido da Lista B do
Passo 386 (módulo ausente em L1, não catalogado, bloqueante de `bibliography`). Novo ficheiro
`01_core/src/rules/stdlib/loading.rs` (decode L1 puro + funções nativas), registado em
`make_stdlib`. `cargo build --workspace` + `crystalline-lint .` **verdes**; **2760** testes da
suíte + **13** novos de decode passam.

## Protocolo de Nucleação cumprido (a trava arquitetural)

Por ser materialização, seguiu-se o fluxo obrigatório do CLAUDE.md, com **paragem real**:

1. Auditoria L0 → **não existia** L0 para `loading` nem ADR de autorização de crates.
2. Redigiu-se o **L0** (`prompts/rules/stdlib/loading.md`) + **ADR-0111** e **PAROU-SE**.
3. Dono aprovou e ordenou save+hash; só então se escreveu código (TDD).
4. Linhagem `@prompt`/`@prompt-hash` (`12e906aa`) propagada via `crystalline-lint --fix-hashes`.

## Decisão que mudou na execução (ADR-0108 em ação)

O dono fixou inicialmente **yaml = `serde_yml`** (na crença de ser o fork mantido). A sonda de
viabilidade **refutou a premissa pela fonte**: a crates.io marca `serde_yml` como **DEPRECATED/
unmaintained** (tal como `serde_yaml`, arquivada). O facto foi surfaçado em vez de aceite; o dono
escolheu então **`saphyr`** (parser YAML genuinamente mantido), com **mapa `Yaml → Value` manual**.
A paridade não depende da crate — fica travada pelos testes de bytes literais (ADR-0107). Sem a
checagem, L1 teria ganho uma dependência morta sob aparência de viva.

## Arquitetura (o estrato força a engenharia melhor — ADR-0029)

O vanilla acopla leitura + parsing por função. O perfil L1 (zero I/O) **proíbe** o acoplamento, o
que produz, de graça, a separação testável:

- **L1 — decode puro:** `decode_{json,yaml,toml,cbor,xml}(&[u8]) → SourceResult<Value>` e
  `decode_csv(&[u8], delim, row_type)`. Zero I/O; prova-se com bytes literais, sem disco.
- **L3 — leitura (já existia, reusada):** `World::read_bytes(current_file, path)` — o único sítio
  que toca disco; **não se criou L3 novo**.
- **Composição:** `native_{read,csv,json,…}` (ABI `(ctx, args, world, current_file)`, igual a
  `native_image`) — único ponto que conhece os dois estratos.

## Paridade por formato (o `Value` de saída, não a mecânica — ADR-0107)

| Formato | Crate (ADR-0111) | Mapa de saída |
|---------|------------------|---------------|
| json | `serde_json` (`preserve_order`) | árvore → `Value`; dict em ordem de inserção (IndexMap) |
| yaml | **`saphyr`** | mapa manual `Yaml::Value(Scalar)`/`Sequence`/`Mapping` → `Value` (`early_parse=true`) |
| toml | `toml` (`preserve_order`) | tabela → `Dict`; datetime → `Str` (graded) |
| cbor | `ciborium` | árvore → `Value`; integer via `i128`→`i64` |
| xml | `roxmltree` | nó → `Dict{tag, attrs, children}` |
| csv | `csv` | `Array` 2D; `row-type` array/dictionary; `delimiter` 1 char |
| read | — | bytes → `Str` (utf8) |

Regra de fronteira (V14): nenhum tipo de parser aparece em contrato L1 — as funções devolvem
`Value` cristalino, conversão interna.

## Subsets graded (ADR-0054) e o DEBT que abriram

`Value::Bytes` **não existe** (ADR-0017 proíbe variant sem tipo migrado). Consequência declarada,
não silenciosa:

- `read` materializa **só modo texto** (`Str`); binário deferido.
- `decode_cbor` byte-strings → `Err` graded.
- `decode_toml` datetime → `Str` RFC 3339 (mapa rico deferido).

Aberto **DEBT-62** (`Value::Bytes`, magnitude S) — passo dedicado de modelagem de tipos; ao fechar,
promove os três graded a paridade plena. **Não-bloqueante**: os 6 formatos + `read` texto
materializam sem ele.

## Critérios de aceitação (§6 do passo) — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `read` + 6 parsers; `Value` de paridade provado em L1 | ✓ 13 testes de bytes literais |
| 2 | Zero I/O em L1 | ✓ grep limpo; decode recebe bytes |
| 3 | Leitura isolada em L3; stdlib func única composição | ✓ reusa `read_bytes` |
| 4 | Erros distintos por estrato (I/O vs malformado) | ✓ prefixo `{fname}(): não foi possível ler` vs `json inválido` |
| 5 | ADR documenta escolha **por formato** | ✓ ADR-0111 (tabela + critério pureza/manutenção) |
| 6 | Inventário ganha Loading; Lista B marca resolvido | ✓ A.8 + nota de fecho na Lista B |
| 7 | Tests verdes; lint zero; hashes propagados | ✓ 2760+13; `✓ No violations`; `12e906aa` |

## Artefactos (commit `08fed76d3`)

- **Código:** `01_core/src/rules/stdlib/loading.rs` (novo); `stdlib/mod.rs` + `eval/mod.rs`
  (registo); `01_core/Cargo.toml` + `Cargo.toml` + `Cargo.lock` + `crystalline.toml` (7 crates
  autorizadas em `[l1_allowed_external]`).
- **L0:** `prompts/rules/stdlib/loading.md`.
- **ADR-0111** — `IMPLEMENTADO` no fecho (autorização de crates por formato).
- **DEBT-62** — `Value::Bytes` (EM ABERTO).
- **Inventário 148** — entrada Loading (A.8).
- este relatório.

## Alavanca (o porquê deste passo)

`loading` é **pré-condição de `bibliography`/`cite`** — o cluster mais pesado de Model (XL,
DEBT-55), que precisa de carregar ficheiros de dados (CSL/`.bib`). Fechar a lacuna de loading
**destrava** essa frente sem a perseguir aqui (scope-out explícito do passo).
