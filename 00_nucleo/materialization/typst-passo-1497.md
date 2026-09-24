# Passo 1497 — converter observações inconclusivas em sondas de linguagem

**Estado:** PLANEADO; investigação após P1490–P1496, sem pressupor novas falhas produtivas.

## Medição anterior à decisão

No inventário P1489 default há 469 `UNVERIFIED_METADATA`, 49 `EXTRA_BINDING` brutos (43 após normalizar defaults duplicados de `format.*`) e dois `DIFFERENCE` antigos na matriz. Os 469 não são 469 bugs; os extras não são automaticamente membros a remover. O PNG difere em 11/2.005.644 pixels por até um canal, e o PDF em nome interno de fonte, sem testemunho semântico. As contagens valem para HEAD `9e3a267f554b89107d66e5001400de56d3cc6bb7` e worktree/hashes do relatório `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md`, não para um estado futuro.

Há candidatos ainda não sondados no delta upstream (SVG, MathML, kern matemático e CLI), além do round-trip de `stroke` com argumentos posicionais identificado na auditoria P1490. Um diff de fonte não demonstra paridade nem violação (ADR-0107/0108).

## Trabalho delimitado

1. Recolher inventário e matriz atuais, deduplicar por membro/feature/perfil e guardar proveniência de cada número. Priorizar API alterada no novo pin, metadados verificados e membros usados no corpus; não somar duplicações `sym`/`math` como defeitos independentes.
2. Para cada candidato priorizado, escrever sonda bilateral executável do observável de linguagem — valor, tipo, erro, sintaxe ou morfologia — com controles negativos e feature/target explícitos. `MISSING_OBSERVATION` continua inconclusivo até haver probe. Registrar hipótese e evidência que a refutaria.
3. Classificar `MATCH`, `VIOLATED`, `DISABLED_BY_PROFILE` ou `DIFFERENCE` de formato sem testemunho semântico; se a sonda provar violação de linguagem, marcar `VIOLATED`, não `DIFFERENCE`. Registrar separadamente divergência mecânica/intencional. Preservar extras compatíveis até decisão ADR-0127; `join` segue P1495. Não transformar diferenças de byte, pixels limítrofes ou simples presença de binding em crédito de paridade.
4. Para cada nova violação comprovada, abrir passo numerado próprio com owner L0, RED, gate e aceitação. Se uma sonda revelar bug do vanilla, documentar a razão de **não** o copiar. Atualizar só o diagnóstico/instrumentação desta triagem; alterações de ferramenta com consumer produtivo requerem L0 próprio.

**Saída:** fila de lacunas confirmadas e observações ainda inconclusivas, com comandos, hashes e estado do código. Não há meta artificial de zerar 469 sem semântica medida, nem percentual global extraído da amostra.
