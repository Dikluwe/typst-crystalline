# V16 — walk, OutlineEntry e ordem causal

Etapa concluída; objetivo geral de correções permanece aberto.

## Resultado

**V16: 239 → 233.** Foram fechados os seis pontos W1–W6 de walk.rs: cinco produtivos e um no oráculo do teste de reserva. As alternativas nominalmente agrupadas conservam as consequências anteriores. V19 **900 → 907 notas**: seis grupos de consequência uniforme e um no novo helper privado de receiver. V20 fica em **871 notas**, sem aumento. Três mensagens V19 existentes mudam somente pelas decisões prescritas de OutlineEntry. Nenhuma regra foi silenciada ou rebaixada.

A auditoria corrigiu falhas funcionais do mesmo caminho:

- OutlineEntry já era locatável e fornecia NativeElement, mas extract_payload o descartava. A delegação nativa foi restaurada; o walk indexa a própria entry e conserva element/fill como dados semânticos, sem headings fantasmas.
- Source perdia forced_location e shallow_table ao chamar walk. Agora delega pelo mesmo walk_at e conserva os dois valores. O alias Located(Source(entry)) mantém um único registro na Location recebida; Source(Table) conserva o modo raso.
- body() e a identificação do receptor reconhecem Source/Located, preservando o body morfológico, campos e identidades. Strong/Realized não são novos unwraps por esses helpers; outros tipos continuam com o mesmo erro/fallthrough.
- O seletor bare outline.entry usa o índice por tipo. A presença no índice NativeElement não bastava; agora a entry direta também está no kind_index próprio.
- A síntese continua pós-walk, com os mesmos campos e IDs. Seus ordinais são registrados após a âncora real do outline, preservando a ordem relativa dos demais registros. Os dois helpers pub(crate) mantêm dados de ordem em CounterRegistry/StateRegistry, sem executar callbacks nem alterar históricos/ações. Isso corrige bare/where, múltiplos outlines e leituras do prefixo temporal.
- A última diferença de query --field era apresentação: o resultado direto existia, mas o fallback L2 não projetava level/element/fill e o filtro eliminava o campo ausente. O novo ramo projeta os dados existentes, sem fabricar snapshot L1; Some, inclusive vazio, segue autoritativo.

[Lista de correções e grupos](lista-correcoes.md) · [patch completo](stage.patch) · [lista de pendências](lista-pendencias.md).

## Evidência de linguagem e testes

As nove sondas bilaterais passaram no pin vanilla **9f2b6e8715237cb086899a42873660fe744622e8**: entry direta, sintetizada e mista usando body() ou acesso a campos; bare/where em ordem; state e contador customizado na Location da entry sintetizada. O valor observado antes era 2 onde o prefixo correto era 1; depois ambos retornam 1, mantendo o final 2 nos testes. Query --field level passa a devolver [1] no direto e [1,1] no misto. [parity-result.json](parity-result.json), [measurement-after.json](measurement-after.json) e fixtures conservam comandos, fontes e binários. Comparação de dados semânticos impressos/JSON, não identidade de bytes PDF ou certificação de render/paridade global. A projeção recursiva de outras famílias mantém o alcance legado.

Foram adicionados **18 testes**, e o teste antigo que documentava a pendência de OutlineEntry foi migrado explicitamente para exigir a delegação nativa. Os **16 controles Core e três Shell passaram**. A suíte completa teve **8669 aprovados, zero falhas e 7 ignorados preexistentes**, 18 testes a mais que o baseline completo anterior. [workspace/workspace-result.json](workspace/workspace-result.json).

O RED foi preservado por causa, antes de cada correção: inicialmente sete falhas/um controle; perda de contexto Source com duas falhas/sete controles; receivers/body com duas falhas/dez controles; índice/ordem/prefixo com quatro falhas/dez controles; serialização com duas falhas/um controle. Os dois helpers internos inexistentes produziram seis E0599 em três chamadas por helper antes da implementação. O scaffold cfg(test) do reconhecimento bruto serviu somente para o RED unitário do novo helper; o erro real de método foi medido na CLI e o scaffold foi removido. Logs de erro de tipo da fixture e de contagem incorreta do guard foram preservados e não contaram como RED funcional.

A sonda de nova variante Content foi feita em cópia do workspace real, com lib test compilada: os seis pontos selecionados passaram de **zero para seis E0004**, mais um E0004 no helper novo de receiver. Total **22 → 29**, sem outros erros e sem alterar as fontes produtivas. [probe-green-result.json](probe-green-result.json).

## Gates e integridade

Build workspace, fmt, Clippy all-targets com warnings e expectativas não cumpridas como erro, Rustdoc workspace -D warnings, linhagem V1/V5/V15/V26 estrita, reversa e diff-check passaram. [validation-result.json](validation-result.json). O diferencial SARIF conserva todos os demais resultados, mensagens, níveis, multiplicidades e posições mapeadas: [architecture-delta.json](architecture-delta.json). O gate global continua **exit 1 pelos 233 V16 restantes**.

Mudaram dez consumers e seus dez owners L0 no inventário produtivo. Os L0 precederam cada implementação/refinamento; o resselo foi restrito a esses pares. is_locatable recebeu somente documentação/metadados. API externa, campos públicos, traits, flags/defaults, fases, pin do CI e dependências permanecem. Foram adicionadas duas primitivas crate-private de ordenação e um helper privado de receiver. [code-guard.json](code-guard.json), refinement.json e recibos guardam o escopo. Não houve commit, push, merge ou release nesta etapa.

Os rastros externos [trace-runtime.log](trace-runtime.log) e recibos distinguem índice geral, runtime e L3 do índice por tipo e da apresentação de --field. As sondas não instrumentaram código produtivo. Os executáveis derivados de diagnóstico foram removidos após conferência de SHA; fontes, receitas, hashes das bibliotecas e saídas permanecem em generated-artifact-cleanup.json. A revisão automática de permissão expirou antes de criar a primeira sonda; a única retentativa permitida foi concluída, sem bloqueio de aprovação pendente.

## RAM e proveniência

Comandos pesados sequenciais em cgroups com teto **12 GiB**, swap **0** e **Cargo um job**. Pico registrado: **12884901888 bytes (12.000 GiB)**; sem OOM/OOM kill. Maior contador memory.events/max: **313**; os arquivos *-resources.json preservam pressão e limites inclusive das tentativas. Não se infere ganho de desempenho.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, working tree não commitado. [before.json.gz](before.json.gz) e suplementos preservam UTC, fontes/hashes e lista exata git diff HEAD --stat; [final-state.json.gz](final-state.json.gz) preserva o estado final. Fontes congeladas durante a validação. Linter `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Oráculo SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin acima e proveniência ratificada em CLAUDE.md. Os hashes da medição final coincidem com os binários após os gates.

## Continuidade

Restam **233 V16**. A hipótese Source(Table) em table_root está registrada para medição própria; o fechamento nominal desse match não certifica sua regra de linguagem. O objetivo geral não foi marcado completo.
