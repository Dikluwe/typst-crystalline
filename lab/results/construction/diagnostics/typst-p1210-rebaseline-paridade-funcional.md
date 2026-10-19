# P1210 — rebaseline integral de paridade funcional pós-saneamento

**Resultado:** RED de medição, com cobertura parcial e lacunas de harness explícitas.  
**Data:** 2026-08-26  
**Vanilla:** `upstream/main a51e02804`  
**Cristalino:** HEAD `7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568` + working tree identificada em `p1210-proveniencia.md`.

## Conclusão executiva

O subconjunto diferencial declarado está majoritariamente verde, mas P1210 não fecha paridade integral. Dos 19 casos atuais, 14 deram `MATCH`, 3 `DIFF`, 1 `ABSENT` e 1 `ERROR` de harness. P1 passou 50/50 testes. A superfície binária focal passou 28/29 probes; `html` foi a única ausência. Não há base para declarar percentual global: o catálogo vanilla amplo não foi reenumerado e o corpus vanilla amplo ainda não tem sampler P1–P4 determinístico.

## Matriz medida

| Universo / nível | passou | total | não executável | falha de harness | leitura |
|---|---:|---:|---:|---:|---|
| P1 parse, corpus declarado | 50 | 50 | 0 | 0 | árvores compactadas iguais |
| S, sintaxe/diagnóstico | 4 | 4 | 0 | 0 | todos `MATCH` |
| E+B, eval | 3 | 3 | 0 | 0 | versão/capability/valor tipado `MATCH` |
| I, introspecção | 1 | 1 | 0 | 0 | query estrutural `MATCH` |
| L, layout | 2 | 2 | 0 | 0 | presença e geometria `MATCH`; a expectativa histórica de `DIFF` ficou stale |
| X, export | 2 | 7 | 0 | 1 | 3 `DIFF`, 1 `ABSENT`, 1 erro de harness |
| C, CLI/produto | 2 | 2 | 0 | 0 | superfície e package local `MATCH` |
| vanilla amplo P1–P4 | 0 | 0 | 1 universo | 1 | sampler/adapter ausente |

Os denominadores acima não são agregados num percentual único. A lista caso a caso está em `p1210-matriz-pipeline.tsv`.

## Superfície pública

O inventário runtime cristalino foi reexecutado e enumerou 1.151 entradas. As 29 probes binárias foram reexecutadas contra os dois binários: 28 coincidiram; `html` existe no vanilla com feature HTML e falha no cristalino quando avaliado pelo harness corrente.

O catálogo amplo vanilla não pode ser reexecutado pelo harness atual. Para preservar informação nominal, `p1210-superficie-publica.json` contém a enumeração cristalina nova comparada ao catálogo vanilla histórico de P1140.26. Suas contagens amplas — 820 `MATCH`, 45 `EXTRA_BINDING`, 1.028 `MISSING_MEMBER` e 286 `UNVERIFIED_METADATA` — são provisórias e classificadas como `HARNESS_LIMITATION`; não são evidência de fechamento nem autorização para implementar.

## Divergências verificadas

| Caso | Classe P1210 | Observável |
|---|---|---|
| `P1137-X-002` | `PRODUCT-GAP` | HTML vanilla compila com `--features html`; cristalino sai 1 sem artefato |
| `P1138-X-001` | `LANGUAGE-GAP` | árvore SVG semântica difere |
| `P1138-X-002` | `PRODUCT-GAP` | raster PNG difere |
| `P1138-X-003` | `PRODUCT-GAP` | observáveis PDF — páginas/caixa, texto e/ou fontes — diferem |
| `P1138-X-004` | `HARNESS-GAP` | lado cristalino não produz HTML; extrator esperava exatamente um artefato |

`P1138-L-001` mediu `MATCH` embora o manifesto esperasse `DIFF`; isto é candidato a `STALE-SCOPE-OUT`/baseline stale, não falha funcional.

## Probes focais recentes

O conjunto disponível cobre `path`, `page`, famílias de membros, kinds, `emoji.heart` e HTML. Foram 28/29 matches. O harness atual não contém probes nominais suficientes para provar separadamente raiz virtual de `path`, identidade consultada de `location`, `page.numbering` por função, `Symbol` multi-codepoint e todos os construtores HTML P1165–P1178. Esses itens permanecem `HARNESS-GAP`, não sucesso implícito.

## Scope-outs L0

A busca estrita encontrou 420 ocorrências textuais em 112 L0s. Todas estão listadas com `file:line`, consumer resolvido quando o header `@prompt` permitiu e texto original em `p1210-scope-outs.tsv`. Como o passo não dispõe de 420 sondas focais reproduzíveis, as ocorrências não medidas ficaram `HARNESS-GAP`. Isso evita promover texto histórico a dívida real ou a resolução sem evidência.

## Eixos de produto

| Eixo | Evidência atual | Estado |
|---|---|---|
| layout paginado | fixture simples, geometria igual | cobertura insuficiente para bidi, regiões, colunas, floats e footnotes |
| math | smoke PDF simples | morfologia/geometria ampla não amostrada |
| introspecção | query estrutural simples | `MATCH`, cobertura focal |
| recursos externos | package local | `MATCH`; fontes/imagens/bibliografia/plugins/paths amplos não medidos |
| PDF | observáveis normalizados | `DIFF` |
| HTML | capability e árvore | `ABSENT` + `HARNESS-GAP` |
| CLI | superfície e package | 2/2 `MATCH`; não é inventário exaustivo de mensagens/defaults |

## Caminho crítico e próximos clusters

1. Corrigir primeiro o harness HTML para passar features simetricamente e distinguir ausência de backend de falha do extrator.
2. Criar enumerador vanilla fresco para superfície pública, inclusive feature HTML, eliminando a dependência do catálogo P1140.26.
3. Materializar sampler determinístico do corpus vanilla ratificado com promoção honesta P1→P4.
4. Abrir clusters separados para SVG, PNG e PDF observáveis; cada um começa pelo L0 proprietário e RED focal.
5. Criar probes nominais para as frentes P1141–P1178 e para scope-outs priorizados.

Nenhuma correção funcional, alteração de contrato, default, fase ou compatibilidade foi feita neste passo.

## Baseline futura de manutenibilidade

Para cada cluster futuro, acumular: owners L0 e consumers tocados; tamanho do diff funcional sem resselo; número de testes RED; tempo RED→GREEN; regressões fora do owner; violações arquiteturais criadas/removidas; gates ADR-0127; reincidência da classe; e owners afetados por extensão posterior. Esses dados ainda não existem e nenhuma conclusão de manutenibilidade é inferida deste rebaseline.

## Gates

- proveniência, binários, comandos e denominadores: registrados;
- índice Git: permaneceu vazio;
- build workspace release: terminou com sucesso, com warnings preexistentes;
- testes unitários do runner: 16/16;
- parse parity: 50/50;
- inventários nominais: produzidos;
- paridade integral: **não fechada** por `HARNESS-GAP` e divergências reais;
- validações finais de build debug, lint e diff-check: registradas após a geração dos artefatos.

