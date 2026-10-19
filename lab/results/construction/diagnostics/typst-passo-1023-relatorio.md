# Passo 1023 — `flow`/`sectioning` fechados, com fronteiras medidas

**Data**: 2026-08-13

## Proveniência

Medições na árvore de trabalho, não commitada, sobre `HEAD = cf2680f76`, às
2026-08-13T01:27-03:00. Ferramenta: `tools/analysis/cochange_metrics.py` na versão
corrigida (a que fez a auditoria retroactiva). Saída bruta em
`temp/p1022/` e a medição deste passo reproduzível com:

```
python3 tools/analysis/cochange_metrics.py \
  01_core/src/compiler/stdlib/structural.rs '(rules|engine|compiler)/stdlib/structural\.rs$'
```

**Correcção à pré-condição do plano**: o plano dizia "nada foi commitado pela auditoria
retroactiva". Foi — `cf2680f76`, a pedido do dono. Os dois L0s corrigidos estão como a
auditoria os deixou; a intenção da pré-condição (não construir sobre L0s alterados por
outra mão) está satisfeita.

## Resultado

`structural` passa de **9 para 14 nós**. Os nós `flow` e `sectioning` deixaram de existir.

| Nó novo | Nativas | Commits de corpo reais | O que decidiu |
|---|---|---:|---|
| `outline` | `outline`, `lof`, `lot` | 3 / 1 / 1 | critério 3 (`2ca61c873`) + mecanismo |
| `heading` | `heading` | 7 | critério 3 (anda sozinha) + critério 4 |
| `title` | `title` | 1 | critério 3 mudo → critério 4 |
| `divider` | `divider` | 2 | critério 3 (anda sozinha) + critério 4 |
| `par` | `par` | 1 | critério 3 mudo → critério 4 |
| `quote` | `quote` | 2 | critério 3 (anda sozinha) + critério 4 |
| `footnote` | `footnote` | 3 | critério 3 (anda sozinha) + critério 4 |

## Parte 1 — `flow`

Critério 3 mudo entre as três: os únicos commits que `par`, `quote` e `footnote` partilham
são `0f5575cd0` e `6636c5ea6` (renames de módulo), `c4978547e` (lote CSL, 8 nós) e
`e1f09cc24` (de-bake de fonte única, 7 nós). Critério 4 separa —
`model/{par,quote,footnote}.rs`, três ficheiros. Três nós.

**Fronteira de fase verificada, não presumida** (o plano pedia): `native_footnote` (stdlib)
e `compiler/layout/footnote.rs` **não duplicam responsabilidade**.

| Nível | O que faz |
|---|---|
| `stdlib/structural/footnote.rs` | valida argumentos, emite `Content::footnote_with_numbering(body, numbering)` |
| `compiler/layout/footnote.rs` | `pub(super) fn layout(...)`, estado `pending_footnote_bodies`, marcador via `format_pattern` + `Content::superscript` |

A nativa não numera nem posiciona; o layout não valida argumentos. A fronteira é a fase do
pipeline e mexer nela é gate ADR-0127. Registado no L0 do nó.

## Parte 2 — `sectioning`

### O núcleo confirmado

`2ca61c873` (2026-06-27) é commit de corpo real das três nativas de sumário, e **não é
inserção pura**:

```
$ git show 2ca61c873 -U0 -- <path> | grep '^@@'
@@ -267 +269,61 @@ pub fn native_outline(      ← -267: uma linha removida
```

O corpo de `native_outline` foi alterado (`Content::outline_with(title, depth, indent)`
removido, `OutlineElem::with_target(...)` no lugar) e `lof`/`lot` nasceram nesse commit como
aliases do `target:` novo. Um commit só — evidência fina — mas **com mecanismo**: partilham
`OutlineElem` + `OutlineTarget`, e os aliases não podiam existir sem a mudança em `outline`.
É isso que distingue este caso do padrão "duas funções nascidas no mesmo lote sem se
tocarem", que não é sinal de fronteira. Nó `outline`.

### Os três que sobravam

Medição par a par, excluindo os commits já confirmados artefacto:

| Par | Commits comuns | Todos ruído/lote? |
|---|---|---|
| `heading` ∩ `title` | `0661aef91`, `0f5575cd0`, `6636c5ea6` | sim (fmt + 2 renames) |
| `heading` ∩ `divider` | `0f5575cd0`, `6636c5ea6`, `c4978547e`, `e1f09cc24` | sim (2 renames + 2 lotes) |
| `title` ∩ `divider` | `0f5575cd0`, `6636c5ea6` | sim (2 renames) |
| cada um ∩ `outline`/`lof`/`lot` | só fmt e renames | sim |

**Nenhum dos três liga a nenhum outro.** Pela regra do plano (passo 3, terceira alínea):
três nós individuais. O critério 4 concorda — `model/{heading,title,divider}.rs`, ficheiros
próprios — e foi aplicado só onde o critério 3 ficou em **silêncio**, nunca para desfazer
cluster medido (passo 4 do plano, respeitado: o núcleo `outline` não foi tocado apesar de
`lof`/`lot` não terem homólogo vanilla).

### Achado lateral — `title` não é pass-through

Das sete nativas revistas, seis recebem `_ctx`/`_world`/`_current_file` e não os usam.
`native_title` **usa**: lê `ctx.document_info.title` como terceira fonte do corpo, depois do
named e do posicional. É classe "contexto real" do critério 2, medido na cadeia de resolução
em `title.rs`. Ficou escrito no L0 do nó — é a única do grupo em que a assinatura não engana.

## Decisão de topologia — achatar, não aninhar

O plano escrevia `structural/flow/par.rs` ("ou nome equivalente"). Materializei
`structural/par.rs`: **achatado, sem sub-hub**.

Razão: manter um directório `flow/` reafirmaria em topologia exactamente o agrupamento que a
medição refutou, e um sub-hub de reexportação sem lógica acrescenta um nível sem acrescentar
fronteira. O hub `structural/mod.rs` já é a fronteira de reexportação, e passou a declarar os
14 nós directamente. A regra ficou registada no método: **desfazer um agrupamento refutado
desfaz também o nome do grupo**.

## Materialização

Corte e cola, com prova item a item:

```
itens na origem (flow.rs 3 + sectioning.rs 6): 9
itens nos nós:                                  9
byte-idênticos ao original:                     9/9
linhas distribuídas:                            669
```

Zero helpers privados nos dois ficheiros de origem — o corte não teve de decidir visibilidade
de nada. Dois imports redundantes removidos depois do primeiro build (`Length` em `outline`,
`expect_no_named` em `divider`, que o corpo já chamava por caminho completo); zero avisos do
compilador nos sete ficheiros novos.

`flow.md` e `sectioning.md` foram **removidos** — os nós que descreviam deixaram de existir.
Sete L0s novos, sem referência a número de passo, com a fronteira medida declarada em cada
um.

## Parte 3 — método

`00_nucleo/prompts/auditar-fatiamento.md`, três entradas novas na tabela de proveniência:

- **"Par de co-mudança sem mecanismo plausível é para verificar, não para explicar"** —
  origem: auditoria retroactiva pós-fatiamento de `stdlib/text`, achado de segunda ordem (o
  bug estava na própria ferramenta de auditoria, detectado porque `heading` +
  `native_table_vline` não tinha explicação plausível).
- **Fronteira já materializada em código não é imune a revisão quando a ferramenta muda** —
  duas fronteiras commitadas foram desfeitas depois de medidas de novo.
- **Desfazer um agrupamento refutado desfaz também o nome do grupo** (achatar, não aninhar).

Mais duas notas em prosa: um nó de uma nativa só é resultado legítimo; e o critério 4 decide
nesses casos **só** porque o 3 está em silêncio, distinção que tem de estar escrita no L0 —
"não medido" não é "medido e refutado".

## Validação

```
cargo build                → Finished, zero avisos nos ficheiros novos
cargo test --workspace     → 5842 passed; 0 failed
#[test] em HEAD vs árvore  → 6138 = 6138
crystalline-lint .         → 0 erros; 3 avisos V7 pré-existentes
                             (auditar-fatiamento.md e auditar-spec.md são workflows, não
                              materializáveis; infra/package_version_resolution.md)
```

## Estado da auditoria retroactiva

Fechada. As duas fronteiras que a auditoria diagnosticou como decididas por ruído estão
medidas de novo e materializadas: `outline`+`lof`+`lot` como núcleo real, as outras seis
nativas em nós próprios. Nenhuma fronteira de `structural` fica agora sustentada por
inspecção sem registo — cada uma das 14 declara no L0 o que a sustenta.
