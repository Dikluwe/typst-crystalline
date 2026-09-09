# Passo 1337 — diagnóstico de campo em Bool, None e Auto

## Problema medido e recorte

O diagnóstico P1336 deixou abertas as fronteiras de tipos não abrangidas por
Int/Str. A nova medição `00_nucleo/diagnosticos/p1337-measurement.json`, SHA-256
`f318ee743fa9adfc372f1b44946897438ffdbd5eba0ba329f57ddee10bb8eab2`,
concluída em `2026-09-09T19:09:59.299611+00:00`, confirma que `true.nope`
publica `bool` em vez de `boolean`; Bool, None e Auto sublinham o acesso inteiro
em vez do identificador. None/Auto já possuem a mensagem correta. Valores
válidos permanecem iguais. Array tem ramo próprio e diagnóstico distinto;
valores-tipo, pré-despacho de métodos e ordem de argumentos são outras causas.

Baseline `p1337-baseline.json`, SHA-256
`d84ebca44348e4d12d3a7d243a35751edda1b898599a03a898634302cd0ec672`,
working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. Preserva diff/stat, inventário,
diagnósticos e temporários antecedentes. Vanilla ratificado a51e02804;
não sincronizar o lab nem recalcular paridade global.

## Autoridade e execução

Atualizar primeiro `00_nucleo/prompts/compiler/eval/bindings/field_access.md`.
Único consumer autorizado: `01_core/src/compiler/eval/bindings/field_access.rs`.
Correção interna de diagnóstico, ADR-0127 em fluxo contínuo, sem API/default,
fase ou compatibilidade nova. O passo coordena; somente L0 legitima código.

1. Congelar baseline, L0 normativo, papéis, oráculos e testes antes de C.
2. Autor independente escreve testes locais puro/AST para Bool/None/Auto,
   casos positivos e fronteiras; migra somente as três expectativas P1336
   expressamente sucedidas pelo L0. Preserva a evidência antiga no baseline.
3. Confirmar RED de assertions antes do patch. Implementador não lê testes
   privados novos antes de produzir C. Não adaptar expectativas ao candidato.
4. Corrigir nome Bool e âncora Bool/None/Auto somente quando chegam ao lookup.
   Manter puro o lookup e conservar o span recebido por ele.
5. Executar A/B nos quatro perfis, ordens normal/repetida/inversa, comparando
   todos os canais. Dívida preservada nunca conta como paridade.
6. Aplicar cinco famílias adversariais pré-congeladas em cópia exclusiva:
   nome Bool, cada uma das três âncoras e extrapolação a outra categoria.
   Exigir fonte fresca, recompilação real, identidade do executável e
   testemunha específica. Preservar binários/logs por rodada; não repetir
   o erro de cache do P1336 nem contar compile failure como mutante eliminado.
7. Build e testes workspace release --locked; fmt, diff-check, lint geral,
   V5/V15/V26 estritos e cálculo recíproco A/B. Revisor julga o resultado.

Regime da skill: A/B com autoria separada de intenção/implementação,
testes/oráculos, ataques e veredito; sem atestação técnica de isolamento e
sem selo de refinamento. Testes/oráculos e adversário recebem contexto novo;
por limite total de agentes, o revisor reutiliza apenas seu contexto anterior
de revisão, conforme manifesto sucessor R1. Allowlists preservam as autoridades.
Budget: cinco famílias, no máximo duas revisões instrumentais por causa;
30s/processo CLI e 2700s/build, até quatro workers CLI, sem gates globais
repetidos enquanto existir falha focal. Duas revisões sem ganho na mesma causa
exigem revisão de método; Unknown obrigatório impede fechamento.

## Preservação e entrega

Preservar Int/Str P1336, Array, Type, métodos/chamadas, ordem panic, Dict,
Content/LocatedContent, Module, nativas/closures/With, Float, PDF/features,
warnings, text contextual e todos os arquivos herdados fora do par proprietário.
Nenhum novo membro, fallback reflexivo ou mudança em outro owner.

Temporários exclusivos em `/tmp/p1337-target.F7jPNj`; cache copiado sem
hardlinks mutáveis. `/dev/shm` estava somente leitura na execução antecedente;
não alterar mounts. Artefatos novos somente `00_nucleo/diagnosticos/p1337-*`,
este passo e o par L0/consumer. Sem staging, commit, push ou limpeza destrutiva.
Relatório em diagnósticos deve começar pelo efeito corrigido e o que continua
faltando; depois listar evidência, incidentes, proveniência e veredito delimitado.

## Execução

L0, testes e implementação realizados. Bool publica `boolean`; Bool/None/Auto
ancoram o erro somente no campo. Array, valores-tipo e pré-despacho permanecem
fora do recorte. Resultado, proveniência, gates e incidentes em
`00_nucleo/diagnosticos/p1337-final-report.md`. O veredito independente e o
fechamento são artefatos separados em `p1337-review-final.json` e
`p1337-closure.json`, na mesma pasta de diagnósticos. Sem commit neste pedido.
