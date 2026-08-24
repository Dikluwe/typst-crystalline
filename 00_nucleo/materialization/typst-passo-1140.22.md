# Passo 1140.22 — Concluir canvas físico e camadas de página

**Estado:** executado — fechado por P1140.23  
**Data:** 2026-08-24  
**Continua:** P1140.20.2 parcialmente implementado  
**Gate:** confirmação de P1140.20.2 já concedida; não requer novo gate se o contrato L0 vigente não mudar

## 1. Estado recebido

P1140.20.2 concluiu a Fase A e recebeu confirmação ADR-0127. A implementação
começou em `01_core/src/entities/page_canvas.rs` com:

- `PageBleedSpec`;
- `PageBleed`;
- resolução relativa por eixo e troca por paridade;
- `PageFill::{Auto, None, Paint}`;
- um teste inicial de percentual/paridade.

Nada disso fecha suporte público isoladamente. `bleed`, `fill`, `background` e
`foreground` continuam incompletos até todos os consumers abaixo existirem.

## 2. Objetivo

Concluir P1140.20.2 de ponta a ponta, sem expor `page`/`std.page` e sem aceitar
argumentos ignorados.

## 3. REDs restantes

### Domínio

- bleed uniforme, dict, fold e zero default;
- inside/outside em páginas ímpares/pares sob binding left/right;
- percentuais horizontais contra width trimado e verticais contra height;
- distinção estrutural entre PageFill Auto, None e Paint.

### Eval e transporte

- parsing de bleed escalar/dict, aliases e chaves inválidas;
- rejeição de bleed auto e conflito físico/lógico;
- fill auto/none/paint e tipo inválido;
- background/foreground none/content e tipo inválido;
- presença versus omissão preservada em SetPage e PageRunElem;
- map_content/map_text, igualdade, hash, repr e walkers exaustivos.

### Layout

- snapshot/restauração LIFO dos quatro campos;
- canvas = trim + bleed por lado;
- body continua no TrimBox;
- background e foreground resolvem 100% contra canvas completo;
- ordem e separação das três listas de FrameItem;
- página vazia e run multipágina;
- layers decorativos ausentes de plain text, query e introspecção.

### Export

- PDF: MediaBox expandida, TrimBox somente com bleed não zero;
- PDF: origem trimada transladada por left/top;
- PDF: fill Auto transparente, None transparente e Paint suportado;
- SVG/raster: render_bleed ligado/desligado;
- SVG/raster: Auto branco, None transparente e Paint suportado;
- ordem fill→background→body→foreground nos três targets;
- não comparar bytes como paridade; snapshots binários só são regenerados
  depois de testes estruturais/visuais demonstrarem a mudança intencional.

## 4. Implementação restante

1. Completar testes do owner `entities/page_canvas.rs`.
2. Adicionar módulo ao lineage definitivo e resselo.
3. Estender PageConfig e Page com bleed/fill/layers.
4. Estender Content::SetPage e PageRunElem; fechar todos os constructors e
   matches exaustivos.
5. Implementar parsing em eval_set_rule(page), sem ignorar named.
6. Criar `compiler/layout/page_canvas.rs` como owner forma B.
7. Integrar fechamento de Page e resolução por paridade física.
8. Adaptar visitors: visual percorre layers; semântico percorre somente body.
9. Implementar PDF builder/stream, SVG e raster.
10. Atualizar fixtures somente quando a nova saída estiver semanticamente
    comprovada.

## 5. Travas

- Se for necessário mudar o contrato definido em
  `00_nucleo/prompts/entities/page_canvas.md`, atualizar L0 primeiro. Se a
  mudança afetar contrato/default/fase além do já confirmado, parar em novo
  gate ADR-0127.
- Não colapsar Auto em branco no L1.
- Não usar Option simples quando omitido e none precisarem ser distintos.
- Não achatar background/foreground no body.
- Não inserir valores empíricos derivados de snapshots.
- Não importar L3 em L1.

## 6. Validação

Executar, nesta ordem:

1. testes RED específicos e registrar falha;
2. testes específicos GREEN;
3. `cargo test -p typst-core`;
4. `cargo test -p typst-infra`;
5. `cargo test --workspace`;
6. `cargo build --workspace`;
7. `crystalline-lint .`;
8. `git diff --check`.

## 7. Fecho

Produzir `00_nucleo/diagnosticos/typst-p1140.20.2-canvas-page.md` com:

- matriz final dos quatro argumentos;
- owners e consumers tocados;
- testes e contagens com proveniência;
- caixas PDF medidas estruturalmente;
- comportamento por target;
- fixtures regeneradas e motivo;
- lacunas restantes para P1140.20.3/.4 e P1140.21.

P1140.20.2 só muda para executado quando todos os itens acima estiverem
verdes. Até lá, o módulo parcial não legitima suporte público.
