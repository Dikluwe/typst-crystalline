# Passo 1140.14 — Limites reais de região no reflow RTL

**Estado:** executado — GREEN  
**Data:** 2026-08-24  
**Continua:** P1140.13  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.14-limites-regiao-reflow-rtl.md`

## 1. Objetivo

Determinar se `try_fuse_paragraph` confunde o extremo direito do conteúdo já
desenhado com o limite horizontal da região que contém a linha e, se houver
divergência de linguagem comprovada, corrigir a decisão de reflow RTL usando
geometria estrutural real.

O passo começa como diagnóstico diferencial. Não adicionar campos, constantes
ou marcadores antes de um caso RED.

## 2. Medição antes da decisão

Medir vanilla ratificado e cristalino, preservando fonte, tamanho e texto, em:

1. página com margens simétricas — controle;
2. página com margens esquerda/direita assimétricas;
3. duas colunas;
4. bloco ou caixa com largura explícita;
5. subframe deslocado horizontalmente;
6. combinação com `linebreak` e `parbreak`;
7. texto misto RTL/LTR e linha predominantemente RTL.

Registrar caixas, baselines, quebras e morfologia. Cada número decisório deve
incluir commit/estado da árvore, hora, comandos e binários.

## 3. Hipótese

Em `03_infra/src/layout_bidi.rs`, `try_fuse_paragraph` calcula:

```text
content_right = max(line_content_right(linha))
available_width = content_right - x_min
```

`line_content_right` mede o conteúdo efetivamente emitido, não necessariamente
o limite da página, coluna, caixa ou subframe. A hipótese é que uma linha curta
pode reduzir artificialmente a largura disponível, enquanto uma posição
deslocada pode fazer o extremo do conteúdo parecer um limite estrutural.

A hipótese é refutada se a matriz mostrar paridade e a inspeção provar que o
layout já materializa um item cuja extensão coincide, por contrato, com a
região dona em todos os contextos relevantes.

## 4. Classificação e gate L0

Quebra/fusão de linha altera morfologia e é paridade de linguagem, ADR-0107.

Antes de código, ler:

- `00_nucleo/prompts/infra/layout_bidi.md`;
- `00_nucleo/prompts/entities/layout_types.md`;
- `00_nucleo/prompts/compiler/layout.md`;
- o L0 específico de colunas/subframes somente se a medição os implicar.

Se a correção exigir campo novo em `Page`, `Frame`, `FrameItem`, assinatura
pública ou mudança de comportamento por defeito, atualizar o L0 e parar no
gate ADR-0127. Não aproximar a região para evitar a parada.

Uma correção interna que reutilize geometria já contratada segue em fluxo
contínuo: L0 primeiro, resselo, RED→GREEN.

## 5. Implementação condicionada

Somente após RED:

- separar `content_extent` de `region_extent` na decisão;
- obter o limite da região pela unidade dona, não por margem simétrica
  inferida nem pelo último item;
- impedir reflow entre regiões, colunas, páginas e subframes distintos;
- manter `ExplicitLinebreakBoundary` e `ParbreakBoundary` como barreiras;
- não introduzir tolerância ou constante calibrada;
- atomizar a geometria num helper puro de L3 se a unidade ficar legível
  isoladamente.

Se não houver RED reproduzível, não alterar código: fechar com diagnóstico,
testes de caracterização e a próxima hipótese mensurável.

## 6. Aceitação

1. matriz diferencial registrada com proveniência;
2. hipótese confirmada ou refutada explicitamente;
3. qualquer correção nasce de teste RED e preserva P1140.12–13;
4. nenhuma constante empírica nova;
5. testes focados e suítes das crates tocadas verdes;
6. `cargo build` e `crystalline-lint .` sem violações;
7. relatório em `00_nucleo/diagnosticos/`.

## 7. Fora de escopo

- reescrever o algoritmo Unicode bidi;
- igualdade de bytes/PDF com o vanilla;
- alterar classificação por maioria RTL sem evidência causal;
- corrigir divergências independentes encontradas durante a matriz.
