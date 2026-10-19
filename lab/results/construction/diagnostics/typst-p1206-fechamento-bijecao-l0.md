# P1206 — fechamento da bijeção L0

## Resultado

O coletivo `compiler/atomizacao_elementos.md` foi removido do namespace de
Prompts L0 e preservado integralmente em
`typst-atomizacao-elementos-l0-historico.md`. Seus 29 consumers receberam
owners específicos: dois em `compiler/introspect/` e 27 em
`compiler/layout/`.

`_nuclei/layout/element-form-b.toml` concentra exclusivamente a disciplina
ADR-0109: braço magro e exaustivo, delegação estática à free function
descendente, ausência de import reverso/dyn/wildcard e preservação de
comportamento. Todos os 29 owners pinam o núcleo e conservam seu contrato de
feature no próprio prompt.

## Inventário final

- Introspecção: `heading`, `labelled`.
- Layout: `block`, `boxed`, `cite`, `colbreak`, `curve`, `decorations`,
  `divider`, `grid_cell`, `grid_footer`, `grid_header`, `h_space`, `hide`,
  `pad`, `pagebreak`, `place`, `quote`, `repeat`, `smartquote`, `stack`,
  `table_cell`, `table_footer`, `table_header`, `term_item`, `terms`, `text`,
  `transform`, `v_space`.

Comparação mecânica dos 29 sources contra HEAD, removendo somente linhas
`@prompt`/`@prompt-hash`: 29 idênticos, zero diferentes. Nenhuma lógica,
import, assinatura, default ou fase mudou.

## Medições reproduzíveis

Medição em `2026-08-26T08:55:03-03:00`, HEAD
`00f402e875956304aa435f749a251f359287e2ba`, working tree não commitado. O
`git diff HEAD --stat` acumulado P1181–P1206 registrou 142 ficheiros, 493
inserções e 9.376 remoções; havia 283 entradas em `git status --short`. Novos
owners e históricos ainda untracked não entram integralmente no stat.

- Baseline P1205: V15=1, V26=0, V5 global=340.
- Fechamento P1206: V15=0, V26=0, V5 global=312.
- Os 29 sources estão sem V5 focal; a queda global é 28 porque `curve.rs` não
  possuía metadata canônica e passou a possuir.
- Hash efetivo do núcleo:
  `6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5`.
- `cargo test -p typst-core compiler::introspect::`: 147 passados, zero falhas.
- `cargo test -p typst-core compiler::layout::`: 788 passados, zero falhas.
- `cargo build`: GREEN, com warnings preexistentes.
- `git diff --check`: GREEN; índice vazio (`git diff --cached --quiet`).

## Dry-run duplo e novo gate

Após V15=V26=0, `crystalline-lint --fix-hashes --dry-run .` foi executado duas
vezes. Ambos falharam no mesmo preflight:

```text
canonical hash metadata must occur exactly once
```

O digest do diff antes, entre e depois foi invariavelmente
`54041f08f6440ef10493ffcb32d3ace79636c978b8f2bb04f746da7184198e0d`:
zero escritas. A varredura localizou 21 consumers preexistentes com um
`@prompt` e zero `@prompt-hash`: `layouter_runtime_state`, `rel`, `numbering`,
sete entities/elements (`curve`, `grid_hline`, `grid_vline`, `label`, `ref`,
`table_hline`, `table_vline`), `eval/cast`, `layout/footnote_flush`, nove
stdlib (`collections`, `gradients`, `layout`, `numbering`, `transforms`,
`shapes`, `ref`, `label`) e `infra/fontdb`.

Nenhum pertence aos 29 consumers de P1206. Corrigi-los seria mutação global
fora do lote, explicitamente reservada pelo passo para trabalho posterior.

## Decisão

A bijeção L0 fecha GREEN: V15=V26=0. O reparo global permanece bloqueado por
metadata canônica ausente, agora alcançável e inventariada. P1206 não a corrige
nem executa `--fix-hashes` sem `--dry-run`.
