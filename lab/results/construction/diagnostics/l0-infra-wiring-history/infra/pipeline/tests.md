# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/tests.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `e7cedf2519cafbe449fce4dcefa91bfb8dd58e79c52f51f7e273153fb260410b`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Pipeline — tests
Hash do Código: ba6cc8a0

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/tests.rs`

## Medição anterior à individualização

Baseline: HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree
não commitado observado em 2026-09-26. Os corpos vigentes do pipeline
e a medição por função estão no diagnóstico
`00_nucleo/diagnosticos/pipeline-atomization.md`. A classificação é
reorganização interna: assinatura, defaults, fases, diagnósticos e
observáveis de linguagem são conservados. Não é medição nova de paridade.

## Contrato proprietário

Possui a suíte integrada anteriormente embutida em pipeline.rs. Conserva integralmente os oráculos e fixtures; somente a localização física muda. Exercita as fachadas públicas e os helpers internos por descendência do módulo. Não legitima alteração de código produtivo.

A fachada `00_nucleo/prompts/infra/pipeline.md` conserva os caminhos
públicos por reexport explícito. Helpers compartilhados têm visibilidade
`pub(super)`, limitada à família pipeline; nenhuma API externa é adicionada.
Os corpos são transferidos sem alterar algoritmo, fases ou políticas.
As medições e os rótulos históricos de gate nas seções transferidas
conservam a proveniência original; esta individualização não reabre gates
nem atribui nova conclusão de paridade. Referências ao antigo pipeline.rs
com números de linha são medições históricas anteriores à extração.

## P1408 — provas de largura fracionária pública de `box`

O pipeline prova a morfologia integrada de `box(width: Nfr)` numa linha
finita: uma caixa `1fr` leva o suffix à margem, duas caixas positivas repartem
o restante segundo a razão dos pesos, `0fr` não recebe restante e `-1fr`
isolado recebe o restante conforme a regra `Fr::share` medida no vanila.

Os testes percorrem somente os itens do documento produzido e não serializam
ou acumulam dumps. Casos focais usam uma página pequena; o controle com
`repeat` conserva o teto de 1000 cópias. Soma zero e razão não finita devem
terminar sem deslocamento, loop ou alocação dependente da magnitude do peso.

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs`.

## P1407 — oráculos focais do fill realizado

O pipeline não ganha fase nem regra de negócio. Seus testes paginados verificam
o fragmento público P1407: uma caixa `1fr` repete mais de uma vez, a largura
maior produz estritamente mais cópias que a menor e as cópias do fill default
permanecem dentro do mesmo link de destino. Esses oráculos não inspecionam o
estado privado nem fixam estrutura Rust.

## P1502 — controles de morfologia do repeat após box

### Medição anterior à decisão

No HEAD `f6d4f890141203bbdbbb91dd0ac8d5009c757abc`, worktree limpa,
o comando `cargo test -p typst-infra --lib p1407_` termina exit 101:
0/5 testes passam. A sonda bilateral P1498 mediu 23/1 pontos com
prefixo em repeat e 23/23 sem prefixo. A sonda de 2026-09-25
14:36:52–14:37:43 UTC mediu 18/18 sem segunda box e 18/1 ao inserir
body `rect` depois do repeat. O contrato produtivo é do owner boxed;
esta suíte observa somente o documento paginado.

### Obrigação da suíte

Conservar os cinco testes P1407 de unicidade A/B, sensibilidade à
largura, fill dentro do link, teto 1000 e gap/justify. Acrescentar
controle com e sem prefixo na mesma largura, duas boxes `1fr` irmãs
com repeats distintos e suffixes únicos, e body de bloco após repeat
pendente sem prefixo. Box aninhada é controle adicional para índices
fr/repeat locais. A aceitação é texto, escopo do link e posições
observáveis, não índices privados nem bytes de PDF. Nenhuma fase ou
regra de negócio é adicionada ao pipeline.
