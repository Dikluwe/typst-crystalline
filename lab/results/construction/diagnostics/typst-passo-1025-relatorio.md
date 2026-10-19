# Passo 1025 — Alvo de paridade reconciliado: hipótese 1, com uma divergência nova

**Data**: 2026-08-13

## Proveniência

Árvore de trabalho não commitada sobre `HEAD = b89509522`. Medições com os binários e a
fonte no estado em que estavam nesta data.

## Fase A — hipótese 1 confirmada

`lab/typst-original` **está** no baseline ratificado; o erro era só de identificação, no
relatório do Passo 1024.

Quatro medições:

| O que | Resultado | Nota |
|---|---|---|
| `git log -1` dentro de `lab/typst-original` | `b895095227…  HEAD -> Tekt` | **o lab não tem `.git` próprio** — devolve o HEAD do repo principal (2856 ficheiros do lab são rastreados por este repo) |
| `lab/typst-original/target/release/typst --version` | `typst 0.15.1 (e0e8ca4d)` | binário de 2026-08-07 20:22, data do sync `e0e8ca4dc` |
| `/usr/local/bin/typst --version` | `typst 0.15.1 (e0e8ca4d)` | **string idêntica** — é o mesmo build ratificado, não outra versão |
| `lab/typst-original/crates/typst-library/src/math/ir/` | existe (`item.rs`, `mod.rs`, `multiline.rs`, `process.rs`, `resolve.rs`) | o refactor do IR de math não está na tag 0.15.1 → a fonte é main+93, consistente com `a51e02804` |

A retificação P990-P992 documenta exactamente isto: o sync `e0e8ca4dc` (2026-08-07) levou o
lab da tag v0.15.1 (`9dfd3a085`) para upstream/main (`a51e02804`, +93 commits) e substituiu
**os dois** binários de referência; a string `typst 0.15.1 (e0e8ca4d)` "parece 0.15.1 mas é
main+93 com o hash do nosso repo". Adendo do dono (2026-08-11): baseline **ratificado** como
`a51e02804`, pinado.

**Nada a resincronizar. Nada a remedir.** As medições do Passo 1024 (margem de 10%) foram
feitas contra a fonte vendorizada — reconfirmadas nesta data: `DELIM_SHORT_FALL` em
`math/lr.rs:17` e `short_target = target - short_fall` em `glyph.rs:271` continuam lá, na
fonte ratificada. Só o rótulo estava errado.

## A origem do erro — e é pior do que o plano supunha

O plano atribuía a confusão à string enganosa de `/usr/local/bin/typst`. Não foi isso. O que
eu li como "o alvo é 0.15.0" foi **`./target/release/typst` na raiz do repo — que é o
cristalino**:

```
04_wiring/Cargo.toml:  [[bin]] name = "typst"
02_shell/src/cli.rs:   version = PARITY_VERSION + hash do HEAD do nosso repo
→ ./target/release/typst --version  →  typst 0.15.0 (00cd5bc5)
```

O `0.15.0` não é uma versão do vanilla: é a constante `PARITY_VERSION`
(`entities/version.rs:22`), a versão da **linguagem** que o cristalino declara implementar,
com o hash do nosso commit. Ou seja: usar esse binário como oráculo é medir o cristalino
contra si mesmo, e a sua string de versão é a *afirmação* de paridade, não uma prova dela.
Um erro dentro do outro — a armadilha do plano é real, mas não foi a que me apanhou.

## Achado novo — `sys.version` diverge do baseline (paridade, não mecânica)

Medição directa, mesmo documento (`#repr(sys.version)`) nos dois binários, texto extraído do
PDF com `pdftotext`:

```
cristalino          → version(0, 15, 0)
vanilla ratificado  → version(0, 15, 1)
```

`PARITY_VERSION = (0, 15, 0)` (`entities/version.rs:22`) é fonte única de `sys.version` e do
`--version` do CLI. O baseline ratificado reporta `(0, 15, 1)` — e a constante já estava
atrás **antes** do sync, porque o baseline anterior era a tag 0.15.1.

Isto **não é** mecânica: `sys.version` é superfície de linguagem — um documento pode
imprimi-la, compará-la (`sys.version >= version(0, 15, 1)`) ou ramificar com ela. Logo é
paridade no sentido de ADR-0107, e é divergência real, não escolha registada.

**Não corrigido aqui**: mudar a constante muda um observável por defeito em dois sítios
(`sys.version` e `--version`) → gate ADR-0127. Registado em `entities/version.md` §9a como
item aberto com dono, com as três perguntas que o passo dono tem de responder (seguir o
baseline automaticamente ou por passo explícito; impacto em documentos que comparam versões;
testes que fixam a string do CLI).

## Fase C — o que ficou escrito, e onde

1. **`CLAUDE.md` §"Referência de paridade"** (nova secção, logo após ADR-0107): o baseline
   ratificado `a51e02804` pinado, e as três armadilhas medidas — string do lab enganosa,
   binário da raiz é o cristalino, `sys.version` divergente. É o sítio que teria evitado o
   erro, porque está sempre em contexto.
   **Nota**: `AGENTS.md` é um **symlink** para `CLAUDE.md` (medido: `AGENTS.md ->
   /home/dikluwe/Documentos/Antigravity/typst-crystalline/CLAUDE.md`). Escrever "nos dois"
   escreve duas vezes no mesmo ficheiro — aconteceu e foi corrigido nesta sessão. Quem editar
   um está a editar o outro.
2. **`auditar-spec.md` Bloco 3**: o detalhe operacional junto da regra que prescreve medir o
   vanilla — qual binário é qual, e a instrução de citar o hash pinado em vez de uma tag.
3. **`entities/version.md` §9a**: a divergência de `sys.version`, medida, com o item aberto.
4. **Relatório do Passo 1024**: declaração corrigida no próprio ficheiro, com a nota de que
   as medições se mantêm (foram contra a fonte certa, mal rotulada).
5. **`entities/font_variations.md`**: era o único prompt activo a identificar o binário de
   referência por versão ("vanilla 0.15.0, `lab/typst-original/target/release/typst`") —
   passou a datar a medição (2026-07-22, antes do sync) em vez de rotular o binário actual.

## O que deliberadamente não mexi

37 prompts mencionam `0.15.x`/`0.14.x`. Triados: são **medições datadas** ("medição vanilla
0.15.0 (969087ec)", "P357 vanilla 0.14.2 oráculo") — registo do que foi medido na altura,
que o plano manda preservar. Só `font_variations.md` afirmava a identidade do binário
*actual*.

## Validação

```
crystalline-lint .         → 0 erros; 3 avisos V7 pré-existentes
crystalline-lint --fix-hashes → 2 ficheiros resselados (version.rs, font_variations.rs)
cargo test --workspace     → 5842 passed; 0 failed
```

Zero alteração de código de produção — a divergência de `PARITY_VERSION` fica registada, não
corrigida.
