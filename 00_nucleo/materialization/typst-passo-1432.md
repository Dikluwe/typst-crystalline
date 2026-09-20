# Passo 1432 — reparar o oráculo raster P35 e retomar o selo P1431

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

Medição executada em `2026-09-17T13:16:15-03:00`, sobre HEAD limpo
`c9c720bb4b2311c50cf5deeeb2aca00431a62e14`, tree
`2d5ef352ad836d0c827afc5bc8c4502ec5750035`. `git status --short` e
`git diff HEAD --stat` não produziram linhas.

O candidato P1431 está em `cf92c3d07c05a564b9d089e005b04e9daf03cb62`
e passou 33 dos 34 oráculos congelados. O recibo adversarial
relatório consolidado da limpeza (`7a392753…`) e a verificação
independente relatório consolidado da limpeza (`49ca754f…`) localizaram a única
falha em P35:

- o contrato P1431 exige witnesses raster **otherwise equal** e stroke auto
  `0.05em` com cap `Square`;
- o tamanho default do texto é `11pt`, logo o stroke auto mede `0.55pt`;
- a fixture explícita fixa `0.60pt` ao comparar `Butt`, `Round`, `Square` e
  auto;
- `assert_eq!(auto, square)` compara, portanto, dois caps `Square` com
  espessuras diferentes e não isola a propriedade declarada.

Identidades de entrada:

| artefato | identificador administrativo |
|---|---|
| L0 test-only raster/PDF | [dado administrativo removido] |
| teste independente raster/PDF | [dado administrativo removido] |
| oráculos R5 | [dado administrativo removido] |
| mutantes R5 | [dado administrativo removido] |
| pré-selo R5 | [dado administrativo removido] |

## Classificação e decisão

É um defeito do oráculo, não do produto. A implementação candidata preserva
literalmente `0.05em` e `Square`, conforme o L0 produtivo e a fonte vanilla
pinada. Trocar o candidato para `0.60pt`, arredondar a espessura no exporter ou
detetar auto por cor/proveniência violaria a obrigação e seria adaptação ao
teste.

O reparo autorizado é:

1. atualizar primeiro o L0 test-only de P35 para declarar que todos os
   witnesses de cap usam a mesma espessura efetiva;
2. no teste independente, derivar uma única espessura de `0.05 × 11pt =
   0.55pt` para `Butt`, `Round` e `Square`; o auto continua vindo da fronteira
   pública sem stroke explícito;
3. preservar as duas observações independentes: áreas cobertas estritamente
   `Butt < Round < Square` e bytes `auto == Square`;
4. não alterar código produtivo, contrato, manifesto, desenho de mutantes ou
   os outros 33 oráculos;
5. publicar oráculos R6, invalidando somente R5 e os artefatos posteriores;
6. executar primeiro P35 e os controles P34/P36/U10; só depois ressellar e
   retomar a campanha M01–M30 contra o mesmo candidato;
7. `Unknown`, timeout, identidade ambígua ou mutação não atribuível continuam
   fail-closed e sem crédito.

## Gate ADR-0127

Não há mudança de contrato público, comportamento por defeito, fase de
pipeline ou compatibilidade. É correção interna de um teste que não implementou
o próprio contrato congelado. Segue fluxo contínuo: L0 test-only primeiro,
resselo de hash, RED/GREEN focal e revalidação, sem nova paragem humana.

## Regime e autoridades

Aplica-se o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado.

- intenção: este passo;
- contrato: manifesto e contrato P1431 permanecem byte-idênticos;
- oráculos: autoridade independente altera somente o L0 test-only, seu
  consumer e o recibo R6;
- adversário: valida poder discriminatório, materializa e executa mutantes sem
  corrigir o candidato;
- implementador: não altera código produtivo neste passo;
- verificador: lê entradas seladas e recibos, mas escreve somente o veredito e
  certificado.

## Gates de fecho

- P35 GREEN em release e, se couber no budget, debug;
- P34, P36 e U10 preservados;
- identidade dos outros 33 testes inalterada;
- pré-selo R6 vincula os novos hashes e invalida expressamente R5;
- candidato não mutado 34/34 antes de M01;
- M01–M30 rejeitados com `mutation_score = 1.0`, três ordens determinísticas e
  reversão limpa;
- `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e
  `git diff --check` verdes;
- certificado final limita a alegação ao fragmento P1431 já contratado.

