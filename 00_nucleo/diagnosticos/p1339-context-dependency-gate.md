# P1339 — aprovação aplicada; dependência entre contextos ainda impede implementação

## Resultado e limite real

A autorização mais recente foi registrada em
`p1339-where-counter-phase-approval.json`: callbacks de contadores com
Selector::Element serão resolvidos sob demanda contextual, sem execução global
antecipada e sem alterar assinaturas públicas existentes ou chaves legadas.
Os L0 correspondentes agora reconhecem essa autorização.

Isso não conclui o desenho. Foi demonstrado outro impedimento: um update
criado por um bloco context não existe no snapshot usado pelo bloco que o
consulta. O erro da consulta dependente aborta a compilação antes que o
conteúdo novo seja reintrospectado. Resolver Func sob demanda não pode
recuperar um evento ainda ausente desse snapshot.

Não há implementação produtiva, contrato selado ou GREEN do P1339. As outras
rotas do lote também não foram implementadas. Não se trata de trabalho pronto
faltando somente commit ou relatório.

## Medição antes da decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
O recibo `p1339-context-dependency-probe-runs.json` registra o diff/stat e a
lista exata de arquivos antes/depois, fontes, hashes, argv, cwd, UTC e canais
integrais. SHA-256:
`4da38b499916ed3d5946bb16c904f2eaa268a4ec2f788cc0149927f0d83260d1`.
Execução em 2026-09-10, 01:41:58.226479–01:42:00.368356 UTC.

- Vanilla ratificado upstream `a51e02804`, `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Cristalino antecedente P1338, `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
  `f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.
- Runner `p1339-context-dependency-probe.py`; fixtures em
  `/tmp/p1339-context-dependency.i6a4w4sn`, criadas por apply_patch.

Foram compiladas seis fixtures em cada binário. O observável é o sucesso do
assert ou o diagnóstico alcançado; não a igualdade de PDF. As saídas PDF não
foram inspecionadas nem usadas como prova visual.

| Fixture | Vanilla | Cristalino antecedente |
|---|---|---|
| Set fora de context, leitura posterior | sucesso | sucesso |
| Set criado em context, leitura posterior | sucesso | assertion failed |
| final antes do context que cria o Set | sucesso | assertion failed |
| where(level:1), Set contextual entre headings | sucesso | assertion failed |
| where(level:1), Func contextual entre headings | sucesso | assertion failed |
| assert dependente seguido de panic("stable-error") | stable-error | assertion failed |

Os controles string e filtro antigo isolam o problema sem depender da variante
Element ainda não implementada. Não autorizam corrigir os contadores legados.
O controle Set refuta que a causa seja apenas a execução tardia de Func;
o final antes do produtor refuta que ordenar uma única passagem por posição
resolva a dependência geral. O controle de erro refuta descartar todos os erros
contextuais e aceitar a compilação.

### Fonte cristalina intacta

- `03_infra/src/pipeline.rs:244-248`: todos os blocos usam clones do mesmo
  introspector recebido, sem ver resultados dos demais.
- `:270-278`: erro de apply_func propaga imediatamente; sucesso só entra no
  mapa resolved, ainda sem reintrospecção.
- `:281`: substituição somente após terminar todos os blocos.
- `:310-312`: reintrospecção somente depois de expansão bem-sucedida.
- `:651-662`: produção retorna erro antes do runtime e do layout.
- `:723-760`: o ciclo posterior reexpande contextos, mas só existe se a
  primeira expansão passou; a condição implementada compara apenas páginas.

O L0 vigente `prompts/infra/pipeline.md`, seção P1159, já exige convergência
por páginas, snapshots lógicos, objetos crus e conteúdo realizado. Sua
implementação parcial não autoriza mudar silenciosamente o tratamento de
erros da primeira expansão. O snapshot de EvalContext é read-only durante
eval (`01_core/src/compiler/eval/mod.rs:148-155`).

### Correções do desenho sob demanda já autorizado

A medição independente `p1339-where-counter-runtime-probe-runs.json`, SHA-256
`178a8637df7eb75d21131a46ad68005b9f7e35670644c9cb9b7eeb93031d5ae3`,
levou a corrigir o L0 antes de qualquer código:

- get/at devolvem o prefixo, mas a demanda resolve a sequência completa da
  mesma chave: callback posterior pode produzir erro em leitura anterior;
- callback de update executa sem contexto introspectivo, mesmo quando sua
  closure foi criada dentro de context; capturar Counter continua permitido;
- igualdade e associação de updates usam igualdade da linguagem: ordem dos
  fields distingue, int/float equivalem, NaN não é reflexivo. Derives/Hash de
  entidades não passam a decidir essa identidade.

## Decisão necessária, não solução materializada

O próximo desenho precisa estabilizar os contextos dependentes dos novos
contadores: conservar resultados provisórios válidos, reintrospectar o
conteúdo produzido e reavaliar a leitura dependente antes de decidir seu erro
final. Não pode exportar estado provisório, transformar falha em zero, executar
todos os callbacks antecipadamente nem considerar páginas iguais suficientes.
Erros persistentes continuam erros; ciclos precisam de término explícito.

Essa é ampliação da orquestração contextual e do tratamento de erros, além de
mover a execução de Func para a demanda já autorizada. A forma seletiva exige
identificar dependências por dados, não por texto do erro, nome de variável,
scan sintático ou suposição de que toda consulta é provisória.

Antes de código, pedir ao dono autorização para incluir essa estabilização no
escopo P1339 e redigir seu contrato L0 completo. Manter assinaturas públicas
existentes e não aproveitar os controles para reparar contadores legados.
Se a fronteira L1→L3 exigir novo tipo/campo/método público, especificá-lo e
submeter o gate correspondente; este diagnóstico não o inventa nem aprova.

Não reduzir os casos obrigatórios do P1339 para contornar a dependência.
Não ressellar L0 ainda incompleto como se estivesse pronto para o contrato.

## Regime e verificação

Skill `tekt-materializacao-segregada`, protocolo completo ainda pré-contrato,
executado sem atestação de isolamento. O medidor vanilla recebeu somente as
fontes e recibos permitidos; o coordenador escreveu a sonda bilateral depois
de ler fontes cristalinas, portanto ela é exploratória, não oráculo
independente. Revisão de causalidade separada em
`p1339-context-dependency-review.md`.

Checks e hashes finais são registrados em `p1339-context-dependency-receipt.json`.
V15/V26 e diff-check são verificações de integridade, não GREEN funcional.
V5 permanece pendente pelo L0 em redação; não houve resselo, build do candidato,
teste de mutações, certificado de paridade ou commit.
