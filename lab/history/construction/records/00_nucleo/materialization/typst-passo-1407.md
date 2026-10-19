# Passo 1407 — expandir `repeat` na parcela horizontal fracionária

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** implementado e verificado no recorte.

## Objetivo

Fazer o `Content::Repeat` preencher a largura finita recebida de uma caixa
`1fr`, preservando gap, justify, conteúdo estruturado e limite defensivo. Com
isso, o fill default de `outline.entry` deve alcançar o número da página antes
de o destino interno ser selado em `FrameItem::Link`.

## Proveniência congelada

- HEAD inicial: `ed7340ada45ea16f1561c20a35eb9887b433e6a4`.
- Referência vanila: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
- Binário cristalino: `target/debug/typst`, identificador administrativo
  [dado administrativo removido].
- Binário vanila: `lab/typst-original/target/release/typst`, identificador administrativo
  [dado administrativo removido].
- Medição: `2026-09-15T11:47:20-03:00`.
- Working tree inicial: somente três fixtures novas em
  `lab/parity/corpus/p1407/`; `git diff HEAD --stat` vazio.
- Hashes das fixtures:
  - `repeat.typ`: [dado administrativo removido];
  - `outline.typ`: [dado administrativo removido];
  - `narrow.typ`: [dado administrativo removido].

## Medição antes da decisão

Todas as compilações terminaram com código zero e stderr vazio sob 512 MiB,
um thread e timeout de 15 segundos.

| Fixture | Cristalino | Vanila ratificado |
|---|---:|---:|
| caixa `1fr` de 100 pt | 1 ponto | 23 pontos |
| outline em página de 240 pt | 8 pontos no PDF inteiro | 76 pontos |
| outline em página de 150 pt | 3 pontos no PDF inteiro | 16 pontos |

Fontes medidas antes da classificação:

- `01_core/src/compiler/layout/repeat.rs:14-20` dispõe o body exatamente uma
  vez e ignora gap/justify/largura;
- `01_core/src/compiler/layout/boxed.rs:140-165` dispõe o body antes de
  registrar a fronteira `width_fraction` em `pending_fr`;
- `01_core/src/compiler/layout/cursor.rs:400-451` só conhece a parcela livre
  no fechamento da linha;
- `01_core/src/compiler/layout/outline.rs:20-62` sela os itens no link antes
  desse fechamento;
- `lab/typst-original/crates/typst-layout/src/repeat.rs:12-60` mede uma peça,
  calcula `floor((fill + gap) / (width + gap))`, justifica o resíduo e limita
  a 1000 cópias.

## Classificação e decisão

- Quantidade e distribuição visível das cópias são morfologia da linguagem;
  a estrutura do estado adiado é mecânica livre.
- O Prompt L0 de `repeat` já obriga preencher o espaço horizontal e terminar
  defensivamente. A implementação vigente está desatualizada, não o contrato.
- Esta é uma correção interna de paridade sob ADR-0127: L0 primeiro, resselo e
  fluxo contínuo, sem novo gate humano.
- **Inferência:** a largura correta só fica disponível quando `pending_fr` é
  resolvido. É refutada se um cálculo anterior demonstrar conhecer também a
  largura dos siblings posteriores sem antecipação ou segunda composição.

## Contrato executável

1. `repeat` dispõe a peça uma vez e registra, em estado privado do layouter,
   sua faixa de itens, origem, largura, gap e justify.
2. Ao resolver um `fr`, a parcela atribuída à caixa que termina imediatamente
   após aquela faixa completa a largura da repetição.
3. A quantidade é `floor((fill + gap) / (piece_width + gap))`, com mínimo da
   peça já válida e máximo de 1000 cópias.
4. Com `justify: true` e pelo menos duas cópias, o resíduo é distribuído
   igualmente entre os gaps; com `false`, o gap pedido não muda.
5. Peça vazia, largura não finita, gap inválido ou ausência da fronteira
   fracionária não criam loop nem alocação proporcional ilimitada; preservam a
   única disposição já produzida e limpam o estado pendente.
6. O link do outline força a resolução horizontal antes de drenar seus filhos,
   para que todas as cópias fiquem dentro do mesmo destino causal.
7. A página e os siblings à direita são deslocados uma única vez pela parcela
   fracionária; inserir cópias não altera a soma das parcelas.

## Testes RED → GREEN e ataques

- caixa `1fr` de 100 pt produz mais de uma cópia e mantém `A`/`B` uma vez;
- página mais larga produz mais cópias que a estreita;
- outline default contém múltiplos pontos dentro do link de destino;
- `justify: false` mantém espaçamento regular; `justify: true` alcança o fim;
- peça vazia e peça maior que a parcela terminam sem repetição ou divisão por
  zero;
- largura extrema continua limitada a 1000 cópias;
- mutantes: single-render, fórmula sem gap, `ceil`, divisão por `count - 1`
  quando count é um, cópias fora do link, deslocamento duplo do suffix e teto
  ausente devem ser rejeitados.

## Recursos e Unknown

- Probes: 512 MiB, um thread, 15 s e saída abaixo de 64 KiB.
- Builds/testes focais: 2 GiB, um job, incremental/debug desligados, 512
  codegen units e linker gold.
- Proibidos: suíte monolítica de `typst-core` e `cargo check --tests`.
- Repeat fora de região fracionária finita, direção RTL e bodies que quebram
  linha permanecem `Unknown` neste recorte; não serão declarados em paridade.
- Sem isolamento de autoridades neste ambiente, o veredito será “executado
  sem atestação de isolamento”.

## Execução final

- Estado final medido em `2026-09-15T11:59:25-03:00`, ainda sobre HEAD
  `ed7340ada45ea16f1561c20a35eb9887b433e6a4` e working tree não commitado
  descrito por `git diff HEAD --stat`.
- Binário cristalino final: identificador administrativo
  [dado administrativo removido].
- O owner `repeat.rs` registra somente peças inline finitas e multiplica clones
  por translação, sem reavaliar o body ou duplicar estado da linguagem.
- Cursor desloca primeiro o suffix, usa a parcela fracionária exata quando ela
  existe e expande pendentes em ordem reversa de índice. Caixa de largura
  absoluta usa a fronteira geométrica do primeiro sibling à direita.
- O link de outline resolve esses pendentes antes de drenar seus filhos; os
  pontos repetidos e a página permanecem sob um único destino interno.
- Primeira revisão GREEN produziu a contagem exata da caixa absoluta, mas uma
  cópia excedente em outlines estreitos. O ataque mostrou que a distância ao
  próximo item incluía o espaço depois do fill. A segunda revisão passou a usar
  `piece_width + fr_share` e removeu o excesso.

## Evidência reproduzível

- RED focal: 0/3 aprovados. Duas falhas testemunharam single-render; a terceira
  revelou um oráculo inválido (`box(width: 1fr)` ainda não é aceito pelo parser
  cristalino) e foi corrigida antes da implementação para duas larguras
  absolutas finitas.
- GREEN final: **5/5 testes P1407 aprovados**, 936 filtrados, em 0,05 s de
  execução do binário de testes. Cobertura: multiplicação, resposta à largura,
  permanência no link, degenerados/teto e justify.
- Controles P1406: **8/8 aprovados**, 933 filtrados.
- Provas bilaterais finais sob 512 MiB, um thread e 15 s, todas com código zero
  e stderr vazio:

| Fixture | Cristalino | Vanila ratificado |
|---|---:|---:|
| caixa de 100 pt | 23 pontos | 23 pontos |
| outline em página de 240 pt | 76 pontos | 76 pontos |
| outline em página de 150 pt | 16 pontos | 16 pontos |

- Peça maior que a parcela permaneceu em uma cópia. A sonda extrema parou
  exatamente em **1000 cópias**, sem aumentar o teto de memória.
- `justify: true` preservou a contagem e deslocou a última cópia além da posição
  observada com `justify: false`, testemunhando a distribuição do resíduo.
- `cargo build --bin typst`: código zero sob 2 GiB, um job, incremental/debug
  desligados e 512 codegen units.
- `cargo fmt --check` e `git diff --check`: código zero.
- `crystalline-lint --fix-hashes .`: cinco owners ressellados, zero drift.
- `crystalline-lint .`: código zero, zero violations e 249 warnings
  preexistentes; nenhum warning novo permaneceu.

## Ataques e veredito

Mutantes materializados e mortos: single-render, fórmula sem largura, uso da
distância incluindo espaço posterior, divisão por `count - 1` no singleton,
cópias fora do link, teto ausente e justify inerte. **Score: 7/7 = 1.0.**
Nenhuma prova monolítica foi executada e nenhum core dump foi produzido.

**Veredito:** `PASS` para repeat inline em região finita absoluta ou carrier
fracionário do outline. RTL, body que quebra linha e repeat ilimitado continuam
`Unknown`. Protocolo Tekt executado sem atestação de isolamento.
