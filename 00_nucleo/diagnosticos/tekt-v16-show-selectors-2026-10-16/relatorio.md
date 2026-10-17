# V16 — lote de seletores de show concluído

**Cinco V16 corrigidos: 267 → 262 erros emitidos.** Um sexto ponto, o predicado
is_text_rule antes expresso por matches!, também passa a exigir uma decisão
quando o enum cresce. O lote abrange quatro consumers e seus quatro L0.

## Comportamento preservado

- native_element_base recorre apenas por Where; And/Or não extraem uma base nativa.
- O caminho textual dedicado aceita somente Text/Regex; preserva offsets UTF-8,
  ordem e exclusão de matches vazios. O grupo vazio de textual_match_ranges inclui Text
  porque o guard do braço anterior pode falhar.
- A captura de equação e tabela conserva identidade executável, aliases reais,
  rejeição de funções homônimas falsas e a política any em And/Or. Filtros Where
  não são avaliados para decidir a necessidade de captura contextual.

As alternativas com comportamento comum permanecem agrupadas, com um único
corpo. Há **7 notas V19 novas**, mantidas como informação.
Elas registram alternativas escritas, não déficit de testes nem combinações
alcançáveis. Não houve supressão, alteração de severidade, corpo duplicado ou
helper genérico para contornar o linter. O gate global continua vermelho pelos
262 V16 externos ao lote. T027 e T158/T160 continuam rastreados sem emissão.

Não houve nova API, variante/campo produtivo, default de produto, mudança de fase
ou quebra de compatibilidade. O fallback interno de FuncRepr em table_cells e decisões de
Content/query não fazem parte desta correção. O lote não certifica paridade geral
ou ganho de leitura/performance.

## L0, testes e prova estrutural

Os quatro L0 foram atualizados antes dos testes/código, sob fluxo contínuo
ADR-0127, com ownership/núcleos validados e resselo restrito bilateral. A medição
e decisão por função estão em classification.json, scope.json e
l0-before-code.json. stage.diff inclui somente os oito arquivos declarados;
pattern-changes.diff separa a nominalização dos novos controles de teste.

Os quatro testes novos passaram no código anterior. Eles verificam as nove
variantes de show, identidade/aliases, composições e seus vazios, Where aninhado,
UTF-8, literal vazio e regex de match vazio. Não há alegação de RED funcional:
o comportamento vigente é preservado. O RED→GREEN desta manutenção é estrutural.

Em cópias descartáveis do workspace real, acrescentar uma variante sentinela
somente ao Selector copiado produziu **0 → 6 erros E0004 nos seis pontos alvo**.
O total da cópia foi 3 → 9, sem outro tipo de
erro. A prova compila o core real com cargo check, sem stubs. O enum produtivo
permanece idêntico; os recibos e fontes mutadas estão em probe-*-result.json e
probe-*-input.json.gz. code-guard.json verifica escopo, corpos esperados e
preservação dos arquivos externos.

## Validação final

**8626 testes aprovados, 0 falhas,
7 ignorados** na suíte workspace, incluindo os quatro novos
controles. Passaram build workspace, formato, Clippy workspace/all-targets com
-D warnings e -D unfulfilled_lint_expectations, gates V1/V5/V15/V26 estritos,
linhagem reversa, Rustdoc -D warnings e git diff --check. Os quatro índices HTML
foram verificados e registrados em rustdoc-indices.json.

O SARIF completo retorna exit 1 pelos V16 restantes. A comparação conserva regra,
nível, arquivo e mensagem, desconsiderando deslocamentos de linhas: removeu apenas
os cinco V16 previstos; não houve diagnóstico novo fora de V19. O inventário
posterior tem 871 notas V19 e 871 notas V20.
architecture-delta.json e remaining-v16.json contêm a evidência completa.

## RAM e proveniência

Execuções pesadas sequenciais em cgroup, MemoryMax=12 GiB, MemorySwapMax=0 e
CARGO_BUILD_JOBS=1. Maior pico observado: 12884901888 bytes
(12.000 GiB), zero OOM e OOM-kill; recibos *-resources.json.
O pico atingiu o teto configurado; a execução não teve folga de memória nesse
momento. Os contadores memory.events, inclusive max, foram preservados nos recibos.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Snapshot inicial UTC `2026-10-17T00:42:25.717991+00:00`; validação final UTC
`2026-10-17T00:46:38.434090+00:00`–`2026-10-17T00:56:11.091466+00:00`. before.json.gz e final-state.json
registram hashes, arquivos/fontes, status completo e git diff HEAD --stat.
Fontes/L0/configuração ficaram congelados durante a validação final.
Linter pin `e53de6fd2bff9f3a87a3e8a076f5e470ce4bb7a0`, binário SHA-256
`9bc3a8521bc7db1dc0afffa04fc5d24012cf7ffbd5376bb8e9729242af2c40aa`. A suíte usa o vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 ratificado
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea;
o recibo workspace/workspace-pre.json.gz conserva o ambiente e a receita.
Nenhum commit, push ou release nesta etapa.

[Lista de correções](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/lista-correcoes.md) · [Pendências](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/lista-pendencias.md).
