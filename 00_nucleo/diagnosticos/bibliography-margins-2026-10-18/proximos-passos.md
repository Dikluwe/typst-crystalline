# Próxima etapa do objetivo integral

A prioridade de margens relativas entre Block/Heading é a correção deste lote.
O fechamento exige consultar summary.json, audit.json.gz e manifesto; não usar
este plano como prova de conclusão ou reduzir o objetivo Faça todas as correções.

1. Completar o recuo francês (hanging-indent): transporte causal de Length no
   constructor par, aplicação em parágrafos e defaults CSL de 1.5em por ocorrência,
   overrides zero/negativos/abs+em (bare set par não vence o default interno;
   show bibliography:set par deve vencer), estratégias simple/optimized e hifenização,
   RTL e continuação em região. HSpace não pode forçar fallback greedy. O corpo
   de Styles suportados por paragraph_prepare ainda recusa propriedades par;
   não esconder essa recusa para passar teste estreito. Ler owners vigentes de
   par, set, paragraph, prepare, plan, select, emit e hyphen antes da proposta.
2. Baselines de linhas com runs itálicos e regiões têm resíduo de 0.156pt em
   controles da bibliografia. Medir por estilo lexical comum e runs, não aplicar
   offset global nem confundir geometria de bbox de fonte com avanço entre linhas.
3. Múltiplas bibliografias e clusters adjacentes de citações ainda abertos;
   fonte, cache por ocorrência, full/target e contratos devem ser medidos.
4. Pontuação RTL preexistente continua requerendo classificação de língua.
   Colchetes visualmente iguais no lote anterior são diferença de extração,
   sem obrigação de reproduzir a mecânica PDF do vanilla.
5. R3-T1 continua aguardando a aprovação já solicitada, sem nova pergunta.
   Proposta e overlay predecessor preservados. E01–E20, frentes públicas
   RM1/RG/FG/Paint/Symbol e 105 V16 não foram encerrados por este lote.

Manter limite 12 GiB/High 8 GiB/swap 0/jobs 1, L0 primeiro, RED funcional, grafos válidos
antes de resselo; não publicar ou modificar APIs ainda não aprovadas.

A instalação global do linter levantou nove V16 extras em consumers inalterados.
Investigar com seus L0s em global-linter-findings.json. O CI mantém 1279707;
não atualizar pin ou fechar esses casos por serem diferenças de ferramenta.
