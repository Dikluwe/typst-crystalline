# P1321 — parecer independente final R2

**PASS no recorte congelado.** A validação CSV e a origem agregada de missing
atendem às expectativas independentes; não há pendência de gate nesta revisão.
Regime: A/B executado sem atestação técnica de isolamento. Não é selo de
refinamento, mutation score ou alegação de paridade geral CSV/Typst.

## Mudança e conclusão substantiva

Loading valida fonte/missing, todas delimiter, todas row-type, primeiro
remanescente causal e só então obtém dados/parseia. Missing source nominal
tem o conselho e span completos; positional existente torna source nominal
um remanescente comum. Cast/opções preservam value_span, unknown/excesso usam
span completo e falhas de validação impedem leitura. Testes mantêm parser,
Bytes, fontes inválidas, paths normalizados, identidades Project/Package e
leitura única nas chamadas válidas. Não se expandiu o cast Symbol.

O primeiro candidato R1 falhou em oito missing sem source nominal: Args.span
ainda continha apenas a lista de argumentos. Essa falha permaneceu registrada,
sem alterações nas expectativas. R2 reabriu causalmente os L0 dos dois owners
e acrescentou somente native_csv à allowlist existente de transporte do span
da chamada. O diff produtivo R2 é import + comparação de function pointer;
With recursivo, preservação de preargs/ocorrências e outras identidades
continuam os mesmos. O dispatch transporta dados e não escolhe erro/cast.

Não encontrei mudança de API/entidade/trait, dependência, fase, namespace ou
modo CLI. Ambas as etapas são correções internas de paridade em fluxo
contínuo ADR-0127. L0s proprietários continuam 1:1. A revisão final não abre
exceção para capturar callee por nome ou fabricar range a partir de mensagem.

## Evidência auditada

Reprodução read-only: `node 00_nucleo/diagnosticos/p1321-review-r2-audit.cjs`.
Exit 0; saída integral em `p1321-review-r2-final-audit.json`, SHA-256
`efd070554e370f64c8dd6cc93da37f2f93016aa2e27a1c2041e46b9c5fd34938`.
O checker verifica hashes, testes, estados dos gates e cada observável A/B
diretamente; não aceita apenas o resumo PASS de outra autoridade.

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os únicos arquivos produtivos/L0 modificados são os dois pares loading e
call_dispatch. Cada recibo de gate conserva estado completo, diff/stat e UTC.
O executável final `/tmp/p1321-target.669PuL/release/typst` tem SHA-256
`756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`.
Referência ratificada upstream `a51e02804`, nunca identificada só por versão.

- RED R1 genuíno: 66 passaram/seis falharam; testes completos continuam
  byte-idênticos ao RED depois dos ajustes históricos declarados prépatch.
- RED R2 genuíno: um passou/um falhou no transporte CSV; GREEN R2: dois passam.
  Todos os testes do dispatcher permanecem idênticos ao RED R2.
- A/B focal R2: 140 comparações passam. Integral: 1548 comparações passam
  (129 casos, quatro perfis, normal/repeat/reverse), zero Unknown/falhas.
  Todas as 440 expectativas originais permanecem intactas; R2 acrescenta
  76 expectativas de controles, sem substituir as anteriores.
- Workspace: 6666 passam, zero falhas, três ignorados, exit 0.
  Recibo `p1321-r2-workspace-tests.json`, SHA-256
  `afbc5559ef9efad6a8dcb54dcbbd6686b952eafdb0ba5a3aa6fec460379754df`,
  entre 2026-09-08T20:48:02.903704+00:00 e 20:52:38.930214+00:00,
  com os quatro arquivos finais idênticos antes/depois.
- Build release workspace, fmt e diff-check passam. Lint global: zero erros,
  com 240 warnings/1138 infos legados; não alego corpus global sem avisos.
  Gate V5/V15/V26 com fail-on warning passa, e ambos A/B de linhagem foram
  recalculados independentemente, cobrindo o falso negativo reverso conhecido.

## Linhagem independente e preservação

| Owner | Hash A efetivo | Hash B reverso |
|---|---|---|
| loading | b8c5243a | c0f17d3b |
| call_dispatch | c80d41d0 | 01a0d090 |

Os hashes completos de fonte/L0/norma/testes/A/B estão no recibo independente.
O Núcleo content-snapshot conserva pin efetivo
`5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24`.
R1/freeze/oráculos/falha e binário preservado são inputs imutáveis no freeze
R2. Nenhum relatório anterior foi reescrito pelo revisor.

## Limites que permanecem

O testador exploratório inicial foi descartado após leitura fora da allowlist;
seus artefatos não compõem aceitação. O testador limpo declara não ter lido
produto/patch e registrou cache incidental próprio removido. Ferramentas
compartilhadas não atestam capacidades, portanto o veredito continua sem
atestação de isolamento. O revisor escreveu apenas diagnósticos/checkers.

`$csv()$` direto foi uma sonda exploratória de namespace, excluída antes do
candidato R2: não alcança CSV vanilla. As duas rotas math por alias/With que
alcançam CSV bilateralmente passam. Não se infere paridade do namespace math,
resolução cross-source, texto externo Path, coerção Symbol ou outros loaders
além dos controles expressamente preservados. A primeira reabertura produziu
ganho causal medido; não houve ciclo de duas revisões sem ganho.
