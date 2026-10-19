# Passo 1408 — aceitar largura fracionária pública em `box`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** implementado e verificado no recorte.

## Objetivo

Aceitar `box(width: Nfr)` na stdlib e transportar o peso fracionário até o
layout inline, usando o mecanismo `width_fraction` já materializado em P1404.
A distribuição deve seguir a semântica observável do vanila ratificado,
inclusive para pesos zero e negativos, sem criar loop ou alocação dependente
da magnitude do número.

## Proveniência congelada

- HEAD inicial: `363883a89d0e756d9cd701eca76ac3b980b02f4e`.
- Referência vanila: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
- Binário cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido].
- Binário vanila: `lab/typst-original/target/release/typst`, identificador administrativo
  [dado administrativo removido].
- Medição: `2026-09-15T12:35:00-03:00`.
- Working tree inicial: somente as quatro fixtures novas em
  `lab/parity/corpus/p1408/`; `git diff HEAD --stat` vazio porque eram
  untracked.
- Hashes das fixtures: `one.typ` [dado administrativo removido],
  `ratio.typ` [dado administrativo removido],
  `zero.typ` [dado administrativo removido]
  e `negative.typ` [dado administrativo removido].

## Medição antes da decisão

Cada processo recebeu teto de 512 MiB, um thread e timeout de 15 segundos.
Todos os quatro casos terminaram no vanila com código zero e stderr vazio. O
cristalino rejeitou todos com `box(width:) espera length, recebeu fraction`.

| Caso | Vanila ratificado | Cristalino inicial |
|---|---|---|
| `box(width: 1fr)` | aceita; suffix na margem direita | rejeita |
| caixas `1fr` e `2fr` | aceita; reparte na razão 1:2 | rejeita |
| `box(width: 0fr)` | aceita; caixa não recebe restante | rejeita |
| `box(width: -1fr)` isolado | aceita; ocupa o restante | rejeita |

Fontes medidas antes da classificação:

- `01_core/src/compiler/stdlib/layout.rs:1858-1864` só extrai `Length` para
  `width`; `layout.rs:1957` força `width_fraction: None`;
- `01_core/src/entities/elements/boxed.rs` já possui o carrier
  `width_fraction: Option<f64>`;
- `01_core/src/compiler/layout/boxed.rs:162-165` já registra o peso no
  `pending_fr` depois do body;
- `01_core/src/compiler/layout/cursor.rs:422-449` só distribui se a soma for
  estritamente positiva e não aplica o clamp por parcela;
- `lab/typst-original/crates/typst-library/src/layout/container.rs:476-527`
  tipa a largura como `Sizing` e aceita `Fr`;
- `lab/typst-original/crates/typst-library/src/layout/fr.rs:45-52` define a
  parcela como `max(peso / total * restante, 0)`, ou zero se a razão ou o
  restante não forem finitos;
- `lab/typst-original/crates/typst-layout/src/inline/collect.rs:228-235` adia
  caixas fracionárias, e `inline/line.rs:577-588` resolve cada uma pela mesma
  regra de parcela.

## Classificação e decisão proposta

- Aceitação de `fr` em `box(width:)`, proporção e posição do suffix são
  sintaxe, semântica e morfologia da linguagem; a representação `Option<f64>`
  e a lista pendente são mecânica livre.
- O contrato vigente registra deliberadamente a rejeição pública em P1404.
  Portanto P1408 muda comportamento público e exige confirmação humana após
  o L0, conforme ADR-0127; a solicitação genérica para continuar não elimina
  esse ponto de decisão informado.
- **Inferência:** o carrier existente é suficiente e não requer novo campo ou
  assinatura Rust. É refutada se um teste mostrar que a origem pública precisa
  de comportamento diferente da origem interna para o mesmo peso.

## Contrato proposto

1. `native_box(width:)` aceita os tipos históricos de comprimento e também
   `Value::Fraction(weight)`. Os dois carriers são mutuamente exclusivos:
   comprimento preenche `width`; fração preenche `width_fraction`.
2. Qualquer `f64` representável por `Value::Fraction` é transportado sem
   normalização por sinal ou magnitude. `0fr` e pesos negativos não são erros.
3. Numa linha finita, `total` é a soma dos pesos. Cada parcela é
   `max(weight / total * remaining, 0)` quando razão e restante são finitos;
   nos demais casos é zero. Não se exige `total > 0`.
4. Caixa fracionária dispõe o body dentro da parcela calculada e desloca cada
   sibling posterior uma única vez. Duas caixas positivas preservam a razão
   dos pesos; `0fr` não consome restante.
5. O algoritmo faz trabalho linear no número de fronteiras e itens, não cria
   cópias proporcionalmente ao peso e reutiliza o teto de 1000 apenas quando
   o body contém `repeat`.
6. `box(height: Nfr)` permanece rejeitado. Comprimentos, percentuais e os
   cosméticos históricos não mudam neste passo.

## Testes RED → GREEN e ataques

- unitário de `native_box`: `1fr`, `0fr` e `-1fr` produzem somente
  `width_fraction`; largura absoluta continua produzindo somente `width`;
- pipeline: uma caixa `1fr` põe o suffix na margem; duas caixas `1fr:2fr`
  recebem parcelas proporcionais; `0fr` não desloca o suffix;
- pipeline: `-1fr` isolado reproduz a ocupação do restante observada no
  vanila; soma zero e razão não finita terminam com parcela zero;
- controle P1407: repeat dentro de caixa fracionária continua limitado a 1000;
- mutantes a matar: manter rejeição, preencher ambos os carriers, usar
  `abs(weight)`, exigir peso/soma positivos, omitir clamp e aplicar a parcela
  mais de uma vez.

## Recursos e Unknown

- Probes: 512 MiB, um thread, 15 s e saída abaixo de 64 KiB.
- Builds/testes focais: 2 GiB, um job, incremental/debug desligados, 512
  codegen units e linker gold.
- Proibidos: suíte monolítica de `typst-core` e `cargo check --tests`.
- Combinações de pesos mistos que excedem geometricamente o restante, RTL e
  quebra interna permanecem `Unknown` até prova bilateral específica; a
  fórmula normativa ainda deve terminar nesses casos.
- Sem isolamento de autoridades neste ambiente, o veredito futuro será
  “executado sem atestação de isolamento”.

## Gate ADR-0127

P1408 substitui uma rejeição pública explícita de P1404. O código, os testes e
o resselo só podem começar depois de o humano confirmar este L0.

**Confirmação recebida:** 2026-09-15, antes de qualquer alteração em L1/L3.

## Execução final

- Estado final medido em `2026-09-15T12:55:45-03:00`, ainda sobre HEAD
  `363883a89d0e756d9cd701eca76ac3b980b02f4e` e working tree não commitado
  descrito por `git diff HEAD --stat`.
- Binário cristalino final: `target/debug/typst`, identificador administrativo
  [dado administrativo removido].
- `native_box` separa largura histórica e peso fracionário em carriers
  mutuamente exclusivos. Não aceita fração em `height`.
- Layouter mantém uma largura natural por fronteira em lista privada e finita.
  O cursor calcula a parcela sobre o espaço total, substitui a largura natural
  pela parcela e atualiza o cursor pela soma dos deltas. `Region` não mudou.
- Primeira revisão GREEN aceitava a sintaxe, mas a prova bilateral encontrou
  `Y=69,004pt` contra `66,274pt`: o algoritmo somava a parcela à largura do
  body. A segunda revisão passou a substituir essa largura e produziu
  `66,276pt`, diferença de `0,002pt`.

## Evidência reproduzível

- RED integrado: **0/3 aprovados**, todos rejeitados pelo parser com
  `espera length, recebeu fraction`.
- GREEN final: **5/5 testes P1408 aprovados**, 941 filtrados, em 0,03 s de
  execução do binário de testes. Cobertura: peso positivo, negativo, zero,
  razão `1:2`, soma zero e rejeição de fração em `height`.
- Controles P1407: **5/5 aprovados**, 941 filtrados, inclusive degenerados e
  teto de 1000 cópias.
- Provas bilaterais finais sob 512 MiB, um thread e 15 s, todas com código zero
  e stderr vazio:

| Caso | Cristalino | Vanila ratificado |
|---|---:|---:|
| `1fr`, posição de B | 163,527 pt | 163,532 pt |
| `1fr:2fr`, posição de Y | 66,276 pt | 66,274 pt |
| `1fr:2fr`, posição de B | 163,527 pt | 163,532 pt |
| `0fr`, posição de B | 17,645 pt | 17,645 pt |
| `-1fr`, posição de B | 163,527 pt | 163,532 pt |

- `cargo build --bin typst`: código zero sob 2 GiB, um job,
  incremental/debug desligados e 512 codegen units.
- O primeiro intento de compilar o harness monolítico `typst-core --lib`
  atingiu corretamente o teto de 2 GiB antes de executar testes. A rota foi
  removida, não repetida nem compensada com mais memória; as provas focais
  ficaram em L3. Nenhum core dump permaneceu no repositório.
- `cargo fmt --check` e `git diff --check`: código zero.
- `crystalline-lint --fix-hashes .`: owners ressellados sem drift residual.
- `crystalline-lint .`: código zero, zero violations e 249 warnings do
  baseline; o wildcard novo do helper P1408 foi removido antes do commit.

## Ataques e veredito

Foram materializados e rejeitados durante a execução dois defeitos: manter a
rejeição pública e repartir apenas o restante após os bodies. Os testes finais
também atacam normalização por `abs`, exigência de peso/soma positivos, soma
zero não terminante e aceitação acidental de `height: Nfr`. Não houve alocação
proporcional ao peso nem execução de suíte monolítica.

**Veredito:** `PASS` para largura fracionária inline finita positiva, zero,
negativa isolada e razão positiva `1:2`. Combinações mistas que excedem o
restante, RTL e quebra interna continuam `Unknown`. Protocolo Tekt executado
sem atestação de isolamento.
