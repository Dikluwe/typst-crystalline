# Passo 1140.20.3 — Running matter e numeração de página

**Estado:** escrito — não executado  
**Data:** 2026-08-24  
**Continua:** P1140.20.2  
**Gate:** ADR-0127 obrigatório antes de código

## Objetivo

Completar `numbering` e implementar `number-align`, `header`,
`header-ascent`, `footer` e `footer-descent` como um sistema de marginais por
página. `supplement` fica explicitamente fora e pertence a P1140.20.4.

## Fase A — L0 e gate

1. Revalidar `page.rs:280-311,326-450` e `pages/run.rs:141-143,157-237`.
2. Medir o contador lógico atual/final, patterns de uma/duas peças e funções
   com aridade contextual; determinar o fixpoint necessário antes de desenhar.
3. Fixar alinhamentos válidos e rejeição de horizon.
4. Especificar `auto`/`none`/content, precedência do marginal explícito sobre
   numbering e artefatos Header/Footer na acessibilidade.
5. Derivar 30% somente do default normativo; ratios são relativos às margens.
6. Atualizar L0, declarar incompletude até .4, resselo e **parar**.

## Fase B — RED→GREEN, após confirmação

- RED para numbering pattern/função, atual/total e multipágina;
- RED para top/bottom + alinhamento horizontal e horizon inválido;
- RED para marginais auto/none/content, precedência e contexto;
- RED para ascent/descent absolutos/relativos;
- RED para repetição, restauração e acessibilidade;
- implementar composição atomizada, sem callback empírico ou estado global.

## Aceitação

Nenhuma numeração é apenas decorativa quando o contrato exige contador lógico;
header/footer aparecem uma vez por página e como artefatos; precedência e
restauração batem com a linguagem; testes, build, lint e diff check passam.
