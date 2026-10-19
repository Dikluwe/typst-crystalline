# P1170.1 — materializar `html.a`

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`  
**Baseline:** working tree P1169.1 GREEN + P1170 aprovado  
**Vanilla:** `a51e02804`

## Objetivo

Ressellar o L0 aprovado, obter RED e materializar somente `html.a`: oito
atributos específicos medidos, os 76 globais existentes e body content
opcional. Não alterar entidade, exporter, default ou pipeline.

## Execução

1. preservar working tree e índice vazio;
2. ressellar `compiler/stdlib/html.md`;
3. escrever teste de presença e confirmar RED causal;
4. acrescentar especificações estáticas para `download`, `href`, `hreflang`,
   `ping`, `referrerpolicy`, `rel`, `target` e `type`;
5. testar casts, enums/listas, isolamento, ordem, body e diagnósticos;
6. testar repr e DOM com anchor aninhado em `div` contra o vanilla;
7. confirmar que o gap de agrupamento phrasing no topo permanece aberto;
8. executar formato, testes focados e integrais, build, lint e diff;
9. escrever diagnóstico e parar antes de staging/commit.

`target` aceita qualquer string, mas tipo inválido mantém o diagnóstico
medido que enumera `_blank`, `_self`, `_parent`, `_top` ou string. `ping` é
lista de strings separada por espaço; `rel` restringe cada item aos 27 tokens;
`referrerpolicy: none` vira atributo vazio.

## Aceitação

- exatamente um binding e oito atributos específicos novos;
- 76 globais reutilizados sem duplicação;
- body omitido é `HtmlBody::None`;
- matrizes válida/inválida coincidem em classe observável;
- anchor aninhado tem DOM idêntico ao vanilla;
- agrupamento phrasing de topo não é alterado;
- suíte e lint GREEN; índice vazio.

## Próximo passo

Após GREEN, P1171 continua reservado para `br`, tabela void e whitespace. O
agrupamento phrasing de topo exige auditoria/gate separado.
