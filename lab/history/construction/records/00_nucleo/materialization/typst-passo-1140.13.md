# Passo 1140.13 — Reflow RTL sem calibração empírica

**Estado:** executado — GREEN  
**Data:** 2026-08-24  
**Continua:** P1140.12  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.13-reflow-rtl-sem-calibracao.md`

## 1. Objetivo

Remover da decisão de reflow RTL as aproximações empíricas que ainda tentam
adivinhar se duas linhas pertencem ao mesmo parágrafo, substituindo-as por
evidência semântica e métricas reais já disponíveis no pipeline.

O passo deve preservar a paridade já medida para:

- quebra explícita de linha, com e sem justificação;
- separação por `parbreak`;
- wrapping automático RTL em múltiplas linhas.

Este passo **não** generaliza antecipadamente o marcador introduzido em P1140.12.
Uma nova fronteira semântica só pode ser adicionada após um caso RED que prove
que a estrutura existente é insuficiente.

## 2. Proveniência da medição inicial

Medição realizada em **2026-08-24T12:15:59-03:00**.

- base Git: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- estado: working tree não commitado;
- `git diff HEAD --stat`: **56 ficheiros alterados, 1243 inserções, 211 remoções**;
- `git status --short | wc -l`: **66 entradas**;
- vanilla ratificado executado por `/usr/local/bin/typst`;
- cristalino executado por `./target/debug/typst`;
- observável comparado: caixas de texto extraídas dos PDFs produzidos.

### 2.1 Controles que já coincidem

Para texto RTL com dois parágrafos separados por linha vazia, vanilla e
cristalino produziram as mesmas caixas e duas baselines (`y = 7.64` e
`y = 26.22`). Portanto, esta medição **não legitima** criar um marcador novo
para `parbreak`.

Para wrapping automático RTL numa largura útil de 70 pt, vanilla e cristalino
produziram as mesmas caixas em três baselines (`y = 7.64`, `20.72` e `33.80`).
Logo, remover o reflow por inteiro também não está legitimado: a correção deve
preservar este comportamento.

### 2.2 Dívida comprovada no código

Em `03_infra/src/layout_bidi.rs`, `same_paragraph` ainda decide com:

- proximidade vertical de `1.5 × max_height`;
- largura aproximada por `text.len() × style.size × 0.5`;
- folga fixa de `30.0`.

Esses números não representam uma regra da linguagem, não vêm de métricas da
fonte e não estão vinculados à geometria real da região. Em especial,
`text.len()` mede bytes, não glifos nem avanço tipográfico.

O L0 vigente, `00_nucleo/prompts/infra/layout_bidi.md`, já proíbe tolerância
posicional e a constante `30.0` para inferir a causa de uma quebra. Há,
portanto, deriva entre L0 e implementação a corrigir.

## 3. Auditoria obrigatória antes do código

Ler e confrontar, nesta ordem:

1. `00_nucleo/prompts/infra/layout_bidi.md`;
2. `00_nucleo/prompts/compiler/layout.md`;
3. o L0 da entidade de layout, apenas se a solução exigir transportar limites
   de região que hoje não façam parte do contrato.

Confirmar os hashes de linhagem dos módulos tocados.

Se a decisão exata exigir adicionar campo público, alterar assinatura pública,
mudar comportamento por defeito ou mudar fase do pipeline, aplicar o gate do
ADR-0127: atualizar o L0 e **parar para confirmação humana**. Não aproximar
limites de página, coluna ou subframe para evitar esse gate.

Se nenhuma mudança dessas classes for necessária, atualizar primeiro o L0,
resselar os hashes e prosseguir em fluxo contínuo RED→GREEN.

## 4. Medição RED antes da decisão

Criar uma matriz diferencial mínima que isole a decisão de reflow:

1. wrapping automático RTL que cabe e que não cabe ao recompor;
2. `linebreak` explícito com `justify: true` e `justify: false`;
3. `parbreak` entre trechos RTL;
4. tamanhos de fonte diferentes na mesma linha e entre linhas;
5. texto multibyte e sequências combinantes, para expor o erro de `len()`;
6. mistura RTL/LTR;
7. coluna ou subframe, se o reflow puder atravessar essas regiões.

Para cada divergência, registar:

- fonte Typst mínima;
- comando exato;
- caixas/baselines vanilla e cristalinas;
- hash/estado da árvore e hora;
- classificação semântica, sintática ou morfológica;
- hipótese e o caso que a refutaria.

Não escolher a nova fórmula antes dessa medição.

## 5. Implementação pretendida

### 5.1 Barreiras semânticas

Manter `SemanticKind::ExplicitLinebreakBoundary` como barreira determinística.
Usar outras barreiras somente quando já existirem semanticamente no layout ou
quando um teste RED provar a necessidade de representá-las.

Não inferir `linebreak`, `parbreak`, mudança de coluna ou mudança de região por
distância entre coordenadas.

### 5.2 Larguras reais

Eliminar de `same_paragraph` e auxiliares:

```text
text.len() as f64 * style.size.0 * 0.5
word_w + 30.0
```

Quando a decisão precisar de largura textual, usar avanço tipográfico real
calculado pelas métricas/fontes do próprio item. Quando precisar de espaço
disponível, usar o limite real da região dona da linha — página, coluna ou
subframe — e não o extremo do conteúdo como substituto.

Não introduzir outra constante calibrada, mesmo que reproduza os fixtures.

### 5.3 Proximidade vertical

Auditar separadamente o fator `1.5 × max_height`:

- se ele apenas tenta descobrir uma relação estrutural, substituí-lo por essa
  relação explícita;
- se representa uma tolerância necessária por arredondamento, derivá-la da
  unidade/precisão numérica e demonstrá-la com teste de fronteira;
- se não for possível justificar semanticamente, removê-lo.

Não trocar `1.5` por outro número obtido por tentativa e erro.

### 5.4 Responsabilidade e atomização

Conservar a lógica bidi em `03_infra/src/layout_bidi.rs` ou atomizá-la num
submódulo dono dentro de `03_infra`, se a unidade puder ser lida e testada
isoladamente. Não mover lógica de render para entidades L1 e não introduzir
despacho dinâmico.

Separar, se isso tornar a decisão verificável:

- identificação de barreira semântica;
- obtenção da geometria real;
- decisão pura de continuidade;
- aplicação do reflow.

## 6. Testes de aceitação

O passo fecha somente quando:

1. o caso RED motivador fica GREEN e coincide com o vanilla no nível da
   linguagem;
2. os controles de `linebreak`, `parbreak` e wrapping automático continuam a
   coincidir;
3. há cobertura para texto multibyte/combining e tamanhos de fonte distintos;
4. nenhuma linha é fundida através de barreira semântica ou região;
5. `layout_bidi.rs` já não contém a aproximação `len × size × 0.5`, a folga
   `30.0` nem uma constante substituta calibrada;
6. qualquer tolerância numérica restante tem derivação documentada e teste de
   fronteira;
7. os hashes L0 dos ficheiros tocados estão atualizados;
8. passam os testes focados, a suíte das crates tocadas, `cargo build` e
   `crystalline-lint .` com zero violações.

## 7. Relatório

Escrever
`00_nucleo/diagnosticos/typst-p1140.13-reflow-rtl-sem-calibracao.md` com:

- matriz antes/depois;
- proveniência completa de cada número usado para decidir;
- explicação de quais heurísticas foram removidas;
- fundamento de toda tolerância que permaneça;
- ficheiros e L0s tocados;
- testes e comandos executados;
- falhas conhecidas remanescentes e a frente seguinte.

## 8. Fora de escopo

- redesenhar todo o algoritmo Unicode bidi;
- perseguir igualdade de bytes ou de estrutura interna com o vanilla;
- adicionar marcadores estruturais sem falha observável;
- corrigir divergências não relacionadas descobertas durante a matriz — devem
  ser registadas e encaminhadas para passo próprio.
