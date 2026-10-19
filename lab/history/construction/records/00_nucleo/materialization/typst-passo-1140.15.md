# Passo 1140.15 — Origem e âncora RTL nas regiões L1

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.14  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.15-origem-ancora-rtl-regioes.md`

## 1. Objetivo

Medir e corrigir, na camada de layout dona da geometria, três divergências RTL
isoladas por P1140.14:

1. margem de página assimétrica usa a borda errada;
2. conteúdo em colunas começa pela coluna esquerda, enquanto o vanilla começa
   pela direita em direção RTL;
3. bloco/subframe com largura explícita ancora à esquerda em vez de respeitar
   o início lógico RTL.

O passo agrupa a auditoria, mas **não presume uma causa única**. Cada eixo só
compartilha implementação se a leitura do código e testes RED provarem a mesma
unidade dona.

## 2. Proveniência da evidência inicial

Medição de P1140.14 em
`45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitado,
**2026-08-24T12:50:53-03:00**; vanilla `/usr/local/bin/typst`, cristalino
`./target/debug/typst`, caixas extraídas por `pdftotext -bbox`.

Depois de remover a fusão pós-layout, vanilla e cristalino preservaram as
mesmas contagens e baselines nas sondas:

- margem assimétrica: 10 palavras;
- duas colunas: 15 palavras;
- bloco de largura explícita: 9 palavras.

As posições absolutas continuaram divergentes:

- na página `width: 180pt`, `left: 12pt`, `right: 38pt`, o vanilla termina
  linhas em `x = 142pt`; o cristalino termina em `x = 168pt`, isto é,
  `180 − 12`, evidência de que a margem esquerda foi reutilizada à direita;
- em duas colunas, o vanilla emitiu a primeira região entre aproximadamente
  `x = 126.51pt` e `230pt`; o cristalino entre `10.51pt` e `114pt`;
- no bloco `width: 82pt`, o vanilla ancorou o conteúdo próximo da borda direita
  da página, enquanto o cristalino o colocou na origem esquerda do subframe.

Esses números são oracles com proveniência, não constantes de implementação.

## 3. Regra de atomização

Executar como três fatias RED→GREEN:

### Fatia A — margens físicas por lado

Auditar o caminho completo:

```text
page(...) → PageConfig → Region → cursor inicial/limites → align_current_line_rtl
```

Determinar onde `left`, `right`, `top` e `bottom` são reduzidos a um único
`margin`. Corrigir a unidade dona, não adicionar exceção RTL no fim do
pipeline.

Aceitação própria:

- LTR e RTL com quatro margens diferentes;
- primeira baseline usa `top`, paginação usa `bottom`;
- cursor/origem usa `left`;
- limite e alinhamento à direita usam `right`;
- margens simétricas permanecem inalteradas.

### Fatia B — ordem lógica das colunas

Auditar `compiler/columns.rs`, `layout_segmented`, offsets e seleção da próxima
região. Separar:

- geometria física das colunas;
- ordem lógica de preenchimento determinada pela direção;
- direção do texto dentro de cada coluna.

RTL deve começar na coluna fisicamente direita e avançar para a esquerda;
LTR mantém esquerda→direita. Não inverter items dentro da coluna para obter
esse efeito.

Aceitação própria:

- 2 e 3 colunas;
- conteúdo curto que ocupa apenas a primeira coluna lógica;
- `colbreak` explícito;
- gutter não uniforme se suportado;
- direção herdada e `#set text(dir: ...)` local.

### Fatia C — bloco/subframe e início lógico

Auditar `entities/elements/block.md`, `layout/block.rs`, criação de subframe,
tradução ao frame pai e alinhamento padrão. Determinar se a divergência está:

- na posição do bloco dentro da região pai;
- na origem do conteúdo dentro do bloco;
- ou em ambas.

Um bloco de largura explícita sem alinhamento físico deve respeitar o início
lógico: esquerda em LTR, direita em RTL. Alinhamentos físicos explícitos
(`left`/`right`, se aplicáveis) vencem a direção e não devem ser invertidos.

Aceitação própria:

- bloco curto e multilinha;
- bloco aninhado;
- bloco dentro de coluna;
- alinhamento explícito versus default lógico;
- `linebreak` e `parbreak` internos preservados.

Se as três fatias tiverem donos diferentes, manter módulos, testes e relatório
separados. Não criar um helper global “corrigir RTL”.

## 4. Auditoria L0 obrigatória

Ler antes de decidir:

1. `00_nucleo/prompts/compiler/layout.md`;
2. `00_nucleo/prompts/compiler/columns.md`;
3. `00_nucleo/prompts/entities/elements/columns.md`;
4. `00_nucleo/prompts/entities/elements/block.md`;
5. L0 de `PageConfig`/tipos de layout se o contrato de margens estiver lá.

Confirmar hashes e decisões vigentes. Não propor contra um L0 existente sem
classificar explicitamente a atualização.

## 5. Gate ADR-0127

Parar após atualizar e guardar os L0s se qualquer fatia exigir:

- novos campos públicos por-lado em `PageConfig`, `Region`, `Frame` ou `Page`;
- alteração de assinatura pública;
- novo comportamento padrão de ordem/âncora;
- quebra de compatibilidade.

Correção interna de paridade que reutilize contratos existentes pode seguir em
fluxo contínuo, com L0 primeiro, resselo e RED→GREEN. Em caso de dúvida sobre
a classificação, parar.

## 6. Testes antes do código

Para cada fatia:

1. produzir fonte Typst mínima;
2. medir vanilla e cristalino;
3. converter a divergência em teste RED no módulo dono;
4. incluir controle LTR e controle simétrico/default;
5. implementar apenas a causa provada;
6. repetir as caixas/baselines diferenciais.

Registrar commit ou working tree, `git diff HEAD --stat`, hora e comandos de
cada medição decisória.

## 7. Restrições

- não recolocar reflow de linhas em L3;
- não corrigir posição final com constante medida;
- não usar `page_width − 2 × margin` quando existem lados distintos;
- não confundir direção lógica com alinhamento físico explícito;
- não mover lógica de layout para entidades;
- não alterar as três fatias simultaneamente antes de obter RED individual.

## 8. Aceitação global

O passo fecha quando:

1. cada divergência inicial está corrigida ou encaminhada, com causa provada;
2. vanilla e cristalino coincidem em morfologia, baselines e âncoras relevantes;
3. controles LTR, margens simétricas e largura de página padrão não regridem;
4. P1140.12–14 permanecem verdes;
5. nenhuma constante empírica entra no código;
6. hashes L0 estão ressellados;
7. passam testes focados, suítes das crates tocadas, `cargo build` e
   `crystalline-lint .` sem violações.

## 9. Relatório

Escrever
`00_nucleo/diagnosticos/typst-p1140.15-origem-ancora-rtl-regioes.md` com:

- matriz antes/depois por fatia;
- proveniência completa;
- unidade dona e causa de cada divergência;
- contratos/L0s alterados e gates aplicados;
- testes e comandos;
- divergências independentes remanescentes;
- recomendação do próximo passo.

## 10. Fora de escopo

- reordenação Unicode bidi intralinha;
- reflow pós-layout;
- igualdade de bytes PDF;
- alinhamento de tabelas/grids não implicado pelas sondas;
- generalização para escrita vertical sem medição própria.

## 11. Resultado da execução

Executado em 2026-08-24. As três fatias fecharam na unidade dona:

1. `PageConfig` preserva margens físicas por lado desde a avaliação;
2. `#columns()` herda a direção RTL da cadeia externa e preenche a coluna
   física direita primeiro;
3. `layout/block.rs` ancora blocos de largura explícita no início lógico RTL.

O gate ADR-0127 foi cumprido antes da alteração pública de `PageConfig` e
`Content::SetPage`. Resultado detalhado no relatório previsto.
