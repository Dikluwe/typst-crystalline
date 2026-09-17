# Passo 1435 — fortalecer P26, corrigir M21 e retomar o selo P1431

## Estado medido antes da decisão

Medição executada em `2026-09-17T16:07:51-03:00`, sobre HEAD limpo
`a4a4767ca6da40dde825ec952d775ee5310d5798`, tree
`c231fa7635f77e53f7df176144dc5258d0246254`.

O veredito R8 `p1431-verifier-r8-nonseal.json` (SHA-256 `d1903797…`)
confirmou:

- M05 R8 foi corrigido e morreu por `VIOLATED_PADDING`;
- M01–M20 morreram na ordem forward, sem `Unknown`;
- M21 sobreviveu ao corpus completo e à reprodução focal;
- P26 verifica que os tokens direitos `2` e `4` têm o mesmo `x`, mas não
  verifica que os tokens esquerdos `1` e `33` têm a mesma borda direita;
- centralizar cada subcoluna satisfaz as asserções atuais, embora viole o
  alinhamento alternado contratado;
- o patch M21 materializado ainda contém uma segunda alteração não autorizada
  (`boundaries = &[]`) além da troca `Alternating → Center`.

Identidades medidas:

| artefato | SHA-256 |
|---|---|
| L0 test-only layout P1431 | `7dbb6e6e32d830629e1b7eb1c3bf130bdf09c6bcc622ebb0546b5f60b67e50bd` |
| consumer test-only | `1545fee27674d1c8e7cf369cd767b6ade9edcf4b6fc9fdc15c0e501b9d02d205` |
| veredito R8 | `d19037979e595dfca2f1a6c4a9f0f5d6ab1e7cd4507b8132b7391734d99eb7f7` |

## Classificação e decisão

É uma insuficiência do oráculo P26 e uma materialização M21 excessiva, não um
defeito do candidato. O contrato já exige geometria alternada; o teste deve
observar diretamente as duas equações independentes:

1. subcoluna esquerda, alinhada à direita: `x(1) + largura(1) == x(33) +
   largura(33)`;
2. subcoluna direita, alinhada à esquerda: `x(2) == x(4)`.

As larguras devem vir de projeções independentes de cada token sob o mesmo
estilo, não de constantes copiadas da implementação. A fixture deve manter
larguras assimétricas (`1` versus `33`) para que `Center` seja distinguível.

M21 R9 deve trocar somente `GridAlign::Alternating` por `GridAlign::Center` no
branch `any_align`. A seleção de `align_boundaries` permanece byte-idêntica.

## Cadeia invalidada e escopo autorizado

A mudança de P26 invalida oráculos R6 e tudo depois deles. Permanecem
congelados manifesto, contrato, candidato e os demais cinco pares L0/consumer.
Após publicar oráculos R9, o desenho de mutantes também deve ser republicado
para vincular a nova identidade e a receita M21 exata.

Fluxo autorizado:

1. atualizar primeiro o L0 test-only P26;
2. atualizar apenas seu consumer e ressellar `@prompt-hash`;
3. provar o candidato não mutado GREEN e o M21 histórico RED pela nova
   equação, antes de redesenhar o patch;
4. publicar oráculos R9, desenho R9, receita e patch M21 exatos;
5. calibrar M21 e controles P25/P31/P32;
6. publicar pré-selo R9 e reiniciar M01–M30 nas três ordens;
7. parar no primeiro sobrevivente, `Unknown`, identidade ambígua ou reversão
   suja.

## Budget e rendimento

Uma revisão do oráculo e uma da receita são autorizadas. O ganho focal exigido
é `M21: Preserved → Violated`, preservando candidato e controles. Se o M21
exato ainda sobreviver, encerrar sem nova tentativa local e reabrir o desenho.

## Gate ADR-0127

Não há contrato público, default de produto, fase de pipeline ou quebra de
compatibilidade. É correção interna de teste/mutation design; segue fluxo
contínuo com L0 test-only primeiro e resselo, sem paragem humana.

## Regime e autoridades

Aplica-se o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado.

- intenção: este passo;
- oráculo: escreve somente L0/consumer P26 e recibo R9;
- desenho adversarial: escreve somente R9, receita/patch M21 e shim;
- candidato produtivo: somente leitura;
- executor: aplica mutações reversíveis;
- verificador: escreve somente selo ou não-selo final.

## Gates de fecho

- hash L0/consumer P26 consistente;
- candidato não mutado 34/34;
- M21 exato morto pela equação alternada e controles preservados;
- M01–M30 mortos em `forward`, `forward_repeat` e `reverse`;
- zero sobreviventes, zero `Unknown`, score `1.0` e reversão limpa;
- `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e
  `git diff --check` verdes;
- certificado limitado ao fragmento P1431.
