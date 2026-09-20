# Passo 1419 — estilo compartilhado antes da primeira candidata de página

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Capturar o estilo tipográfico compartilhado de `par.line` no fechamento físico
da primeira coluna, antes de avançar além do `colbreak()`. Assim, uma primeira
coluna vazia não transfere para os números o estilo local da segunda coluna.

## Regime Tekt e proveniência inicial

- regime: protocolo completo em rehearsal, executado sem atestação de
  isolamento; uma única autoridade exerce os papéis e não alega segregação;
- baseline: commit `3a3d16604`, árvore limpa antes das quatro fixtures P1419;
- medição inicial: `2026-09-16T09:53:41-03:00`;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário identificador administrativo
  [dado administrativo removido];
- working tree na medição: somente `lab/parity/corpus/p1419/` não rastreado.

Todas as sondas receberam 512 MiB de espaço virtual, uma thread e timeout de
15 s. O maior RSS inicial foi 43.008 KiB no cristalino e 31.252 KiB no vanila;
swap permaneceu zero.

## Medição antes da decisão

Com primeira coluna vazia, estilo exterior de 18 pt e segunda coluna local de
8 pt, o vanila produz somente dois números, ambos com morfologia de 18 pt. O
cristalino P1418 captura 8 pt na primeira candidata e produz três. Invertendo
os tamanhos, ambos produzem dois ordinais, mas o vanila usa 8 pt e o cristalino
18 pt: igualdade de cardinalidade isolada esconderia a divergência.

Em três colunas, com primeira vazia e estilo exterior de 18 pt, o vanila usa
18 pt nas duas regiões seguintes e aceita dois números por coluna. O cristalino
usa 8 pt e aceita três por coluna. Callback equivalente permanece opaco e
continua divergente, sem ser promovido por este passo.

No fluxo contínuo, `colbreak()` alcança `close_current_column` enquanto a chain
anterior ao evento ainda está ativa. Esse ponto já é o owner da fronteira
física P1418 e possui `self.style`, mesmo quando a região não tem candidata.

## Oráculos congelados

| Fixture | identificador administrativo | Classificação |
|---|---|---|
| `page-empty-first-large-to-small.typ` | [dado administrativo removido] | positivo: shared 18 pt + dedup |
| `page-empty-first-small-to-large.typ` | [dado administrativo removido] | positivo: morfologia shared 8 pt |
| `page-three-empty-first-shared-large.typ` | [dado administrativo removido] | positivo: N=3, primeira vazia |
| `page-empty-first-callback-unknown.typ` | [dado administrativo removido] | opaco `Unknown` |

Os casos foram escritos antes do candidato. Eles rejeitam “capturar da primeira
candidata”, “usar sempre 11 pt”, “validar somente cardinalidade” e “promover
callback a pattern”. Sem autoridades isoladas nem campanha formal de mutação,
isso constitui poder discriminatório de rehearsal, não selo independente.

## Contrato interno

1. P1418 continua preferindo a chain da primeira candidata quando ela existe.
2. Se a coluna física fecha sem candidata e o estilo compartilhado ainda está
   ausente, capturar `self.style` no início de `close_current_column`, antes da
   translação e antes de avançar para a próxima coluna.
3. Instalar o snapshot uma única vez; candidata posterior nunca o substitui.
4. Não percorrer `Content`, não procurar linha visível e não interpretar o body.
5. O modo segmentado direto com primeira região vazia permanece `Unknown`.
6. Deduplicação, contador, scope, margem final e callback não mudam.

Uma primeira tentativa atravessou wrappers `Styled` exteriores no início de
`layout_flow`; os positivos 18→8 e N=3 permaneceram RED porque esse wrapper já
carregava o estilo posterior. A classificação não ganhou nenhum caso e a causa
pública mudou para `wrong-owner`; a hipótese foi abandonada antes de repetir a
matriz completa.

## Limites e `Unknown`

Callback heterogêneo, modo segmentado direto com primeira região vazia,
configuração `par.line` local entre colunas, RTL e columns aninhadas permanecem
`Unknown`.
Timeout, OOM, swap ou ausência de output nunca contam como preservação.

A mudança é fórmula interna de paridade, sem contrato público, default,
assinatura ou fase nova. Pelo ADR-0127 segue em fluxo contínuo com L0 primeiro.

## Protocolo de memória

- probes: 512 MiB, uma thread, timeout 15 s, outputs em `/tmp`;
- check/build: 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512`, linker `gold` no release;
- lint: 1 GiB, uma thread, timeout 60 s;
- sem testes de workspace ou matrizes amplas antes do GREEN focal.

## Aceitação

Os três positivos coincidem bilateralmente em cardinalidade, sequência e
morfologia dos números; o callback continua `Unknown`. P1418, P1417 e os sete
controles P1416 não regridem. Formatação, diff check, build e lint passam nos
limites sem OOM ou swap.

## Resultado

Medição final em `2026-09-16T10:16:09-03:00`, ainda sobre o commit
`3a3d16604c6cfd56f048e32f71e95daf7818c979` e working tree não commitado. Os
arquivos alterados eram exatamente:

- `00_nucleo/prompts/compiler/columns.md`;
- `00_nucleo/prompts/compiler/layout/cursor.md`;
- `01_core/src/compiler/layout/columns.rs`;
- `01_core/src/compiler/layout/cursor.rs`;
- este passo;
- as quatro fixtures congeladas em `lab/parity/corpus/p1419/`.

O binário cristalino final tem identificador administrativo
[dado administrativo removido];
o vanila permaneceu
[dado administrativo removido].
Nos três positivos, extração textual, sequência e caixas dos ordinais ficaram
iguais ao vanila: dois números a 18 pt no caso 18→8, dois a 8 pt no caso 8→18
e dois números a 18 pt em cada coluna ocupada no caso N=3. O callback conservou
a divergência conhecida (`L3` adicional no cristalino) e permaneceu `Unknown`.

As quatro sondas de cada binário terminaram sob 512 MiB e sem swap. O maior RSS
final foi 43.036 KiB no cristalino e 31.284 KiB no vanila. As três provas P1418
coincidiram no observável de linguagem; as três provas positivas P1417 e os sete
controles P1416 coincidiram com os respectivos oráculos vanila. `cargo check
-p typst-core` passou sob 2 GiB. O primeiro slice do build release atingiu o
timeout de 180 s sem OOM e o segundo concluiu em 1 min 21 s reutilizando somente
artefatos de compilação; nenhuma execução ampla ou gulosa foi iniciada.
`cargo fmt --all --check`, `git diff --check` e `crystalline-lint .` passaram;
o lint terminou com zero violações e somente avisos preexistentes.

Rehearsal final: obrigação, implementação, ataques e veredito foram exercidos
pela mesma autoridade. Os oráculos foram congelados antes do candidato e a
primeira hipótese foi efetivamente refutada, mas não há atestação de isolamento
nem selo de independência.
