# Passo 1140.23 — Fechar canvas físico e camadas de página

**Estado:** executado — fechado  
**Data:** 2026-08-24  
**Continua:** P1140.22  
**Numeração:** este passo não cria subdivisões adicionais

## 1. Estado recebido

O Prompt L0 `00_nucleo/prompts/entities/page_canvas.md` já foi confirmado no
gate ADR-0127. O owner `01_core/src/entities/page_canvas.rs` contém o domínio
inicial de bleed e fill, com testes específicos verdes.

Ainda não existe suporte completo de ponta a ponta. `PageConfig`, `Page`,
`Content::SetPage`, `PageRunElem`, eval, composição de layers e exporters não
transportam integralmente `bleed`, `fill`, `background` e `foreground`.

Uma tentativa anterior de migrar automaticamente literals de `Page` confundiu
campos `items` aninhados. Todas essas inserções foram removidas e a árvore
voltou a passar `cargo check --workspace` e `git diff --check`. Este passo não
repete transformação textual sem delimitação sintática comprovada.

## 2. Objetivo

Concluir P1140.22 e fechar P1140.20.2 de ponta a ponta:

- aceitar e validar os quatro argumentos de página;
- preservar omissão, `auto`, `none` e paint onde semanticamente distintos;
- resolver bleed físico por binding e paridade;
- compor `fill → background → body → foreground`;
- exportar canvas físico e TrimBox corretamente;
- manter layers decorativos fora dos observáveis semânticos.

Não expor `page` ou `std.page`; isso permanece fora deste passo.

## 3. Método obrigatório

### 3.1 RED de contrato interno

Antes de alterar cada estrutura, escrever testes que caracterizem:

1. `PageBleedSpec` uniforme, dict, fold, zero e conflito lógico/físico;
2. `PageFill::Auto`, `PageFill::None` e `PageFill::Paint` distintos;
3. transporte sem perda em `SetPage` e `PageRunElem`;
4. restauração LIFO após page-run aninhado;
5. snapshot físico diferente em páginas pares quando aplicável.

Registrar a falha RED específica antes do GREEN correspondente.

### 3.2 Migração estrutural controlada

Adicionar os campos necessários a `PageConfig` e `Page`. Migrar constructors
em lotes pequenos por owner:

1. production L1;
2. testes L1;
3. production L3;
4. testes L3.

Após cada lote, executar `cargo check` no crate afetado. Não usar substituição
global baseada apenas no texto `items:`. Constructors legados recebem defaults
explícitos que preservem o comportamento anterior.

### 3.3 Eval e transporte

Implementar no owner do set-rule de página:

- bleed escalar relativo ou dict;
- chaves `left`, `right`, `top`, `bottom`, `inside`, `outside`, `x`, `y` e
  `rest` conforme L0;
- rejeição de chaves desconhecidas, `auto` e mistura lógico/físico;
- fill `auto`, `none`, Color, Gradient e Tiling convertidos em `Paint`;
- background/foreground `none` ou content;
- nenhum named argument aceito pode ser ignorado.

Transportar os deltas em `Content::SetPage` e `PageRunElem`, incluindo clone,
hash, igualdade, `map_content`, `map_text`, repr e matches exaustivos.

### 3.4 Layout atomizado

Criar `01_core/src/compiler/layout/page_canvas.rs` como owner forma B. O módulo
deve:

- aplicar e restaurar os quatro deltas;
- resolver bleed somente no fechamento da página, com número físico;
- manter width/height como TrimBox;
- resolver layers contra o canvas completo;
- produzir listas distintas de background, body e foreground;
- impedir que layers decorativos entrem em plain text, query, introspecção,
  estrutura acessível ou MCID.

O match central permanece estático, exaustivo e magro conforme ADR-0109.

### 3.5 Exporters

PDF:

- `MediaBox` usa canvas completo;
- `TrimBox` só aparece com bleed não zero;
- origem trimada é transladada por left/top no sistema do layout e pela
  transformação equivalente no sistema PDF;
- Auto e None são transparentes; Paint é emitido antes dos layers;
- ordem final é fill, background, body, foreground.

SVG e raster:

- opção `render_bleed` seleciona canvas completo ou TrimBox;
- Auto é branco; None é transparente; Paint usa a cor/pintura suportada;
- background e foreground cercam o body na ordem definida.

Não usar igualdade de bytes como critério de paridade. Fixtures binárias só
podem ser atualizadas depois de prova estrutural ou visual da alteração.

## 4. Testes de aceitação

Adicionar testes para:

- bleed uniforme e por dict;
- percentuais por eixo;
- inside/outside em binding left e right, páginas ímpares e pares;
- ausência de TrimBox com bleed zero;
- MediaBox e TrimBox com quatro lados diferentes;
- fill por target nos três estados;
- ordem de background, body e foreground;
- page-run vazio, multipágina e aninhado;
- restauração da configuração exterior;
- layers ausentes de plain text, query, introspecção e tags PDF.

## 5. Gates

Executar nesta ordem:

1. testes específicos do domínio;
2. testes específicos de eval e layout;
3. testes estruturais dos exporters;
4. `cargo test -p typst-core`;
5. `cargo test -p typst-infra`;
6. `cargo test --workspace`;
7. `cargo build --workspace`;
8. `crystalline-lint .`;
9. `git diff --check`.

Qualquer falha conhecida mantém o passo aberto. Warnings preexistentes não são
contados como falha, mas warnings novos introduzidos pelo passo devem ser
eliminados.

## 6. Proveniência e relatório

Produzir
`00_nucleo/diagnosticos/typst-p1140.23-fecho-canvas-page.md` contendo:

- HEAD ou indicação de working tree não commitado;
- `git diff HEAD --stat` no estado medido;
- hora exata das medições finais;
- matriz argumento → owner → consumer → teste;
- MediaBox e TrimBox medidas estruturalmente;
- comportamento por target;
- comandos, resultados e contagens dos gates;
- lacunas que pertencem às frentes seguintes, sem criar subnumeração.

## 7. Condição de fecho

P1140.23, P1140.22 e P1140.20.2 fecham juntos somente quando os quatro
argumentos chegam aos consumers finais, todos os testes e gates passam e o
relatório de proveniência existe. Implementação parcial não autoriza marcar
nenhum deles como executado.
