# Labels de expressões passam a associar conteúdo

Valores retornados por label(), aliases, closures e blocos agora usam a
associação retroativa do fluxo de markup. O label não aparece como texto,
query encontra o alvo e a referência Unicode resolve no leitor PDF.
O trabalho geral de produção continua aberto, inclusive outros casos de
seleção do alvo e realização de labels medidos abaixo.

## Proveniência e medição anterior

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. A coleta inicial foi
2026-10-14T05:28:49.338093+00:00 até 2026-10-14T05:29:10.737409+00:00. before/results.json.gz contém fontes, L0,
status, git diff HEAD --stat, comandos e hashes dos binários. Pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

markup_stream.rs:100–135 tratava somente SyntaxKind::Label; :138–140
encaminhava Value::Label ao fallback de apresentação que usa repr.
A fonte pinada typst-eval/src/markup.rs:57–81 distingue Value::Label no fluxo.
Literal produzia COUNT=1; alias/direct/closure/block imprimiam <alvo> e
COUNT=0. O caso ref com é/😀 abortava antes de exportar. Essa evidência
mede associação/consulta/diagnóstico, não igualdade de estrutura ou bytes.

## Correção e testes

Os owners de markup_stream e dos testes foram atualizados antes do código.
Uma função privada compartilha a associação entre literal e Value::Label;
a expressão é avaliada uma vez e os demais valores seguem seu caminho
anterior. Preservam-se espaços finais, ambiente do bloco, snapshots textuais,
fronteiras de estilo e intercept_labelled, sem alterar a apresentação math.
Correção interna de paridade ADR-0127: sem novo campo/assinatura pública,
flag, mudança de fase ou compatibilidade externa. Resselo restrito aos dois
consumers e seus owners, registrado em seal.log.

Seis regressões novas cobrem formas de expressão, nomes Unicode/delimitados,
espaço residual, warning com severidade/mensagem/hints/trace/range UTF-8,
confinamento do bloco, show por label e texto e execução única. Todas
falharam antes e passaram depois. red-final-state.json.gz e candidate-final.json.gz
conservam as fontes correspondentes, além dos logs RED e GREEN.

A primeira execução do candidato teve quatro sucessos e duas falhas por
espaços iniciais das próprias declarações nas fixtures. fixture-controls.json
verifica no vanilla versões equivalentes com ponto e vírgula. As fixtures
foram corrigidas sem trim de saída; o baseline original foi restaurado
somente para repetir o RED dos seis testes finais, antes de reinstalar o
candidato. Nenhuma expectativa de teste preexistente foi alterada.

## Verificação pelo produto

A recoleta after/results.json.gz ocorreu entre 2026-10-14T05:34:43.541349+00:00 e 2026-10-14T05:35:07.355172+00:00.
Tekt SHA-256 7faa827b77a0b6d21801b98fb16d0e66274cf43d7129fa2ba97c76565b600ce0.
Fontes não mudaram durante a coleta. 10 casos concordam em texto extraído
completo e diagnósticos: literal, alias, direct, closure, block, spaces,
show-text, block-local, block-orphan e ref. Isso não certifica geometria geral.

No caso ref, o PDF contém duas páginas, Section 1 na segunda e um link com
nome é/😀. MuPDF resolve esse link para o índice 0, primeira página, tanto
no Tekt quanto no vanilla. O teste preserva o resultado do leitor e suas
coordenadas em vez de exigir retângulos idênticos. A expressão com mutação
imprime EVAL=1 e COUNT=1. Orphan dentro de bloco produz o aviso e não rotula
um heading exterior.

## Pendências isoladas por controles literais

A coleta targets/results.json.gz (2026-10-14T05:36:02.503680+00:00 a 2026-10-14T05:36:23.850680+00:00)
usa o mesmo candidato e registra estado/fontes/binários. As diferenças
seguintes aparecem também no caminho literal:

- Após quebra de parágrafo, COUNT=0 no Tekt contra COUNT=1 no pin. O seletor
  atual para no Parbreak em vez de procurar o elemento anexável anterior.
- Labels consecutivos conservam OLD=1/NEW=0 e não avisam; o pin dá OLD=0/NEW=1
  e warning de múltiplos labels. Trocar apenas o caminho de Value não corrige
  substituição e metadados do alvo.
- Um set text entre heading e label permite associação retroativa no Tekt;
  o pin mantém o novo escopo, avisa órfão e retorna COUNT=0. É necessário
  respeitar a fronteira lexical ao selecionar o alvo.
- Orphan sem set page, e após set text, avisa corretamente nos dois.
  Com set page, literal e expressão deixam de avisar no Tekt. Hipótese de
  fonte: Content::SetPage sintético é aceito como alvo pelo helper. Não é
  prova de que o pipeline descarta avisos; o controle de bloco os preserva.
- Show por label no heading produz MARCA Alvo na mesma linha; o pin conserva
  heading após MARCA em outra linha. O controle literal reproduz a diferença.
  Investigar morfologia/realização antes de decidir a correção; não basta
  retirar quebras de linha da comparação.

O passo labels-associacao-alvos deve completar seleção, fronteiras e labels
repetidos; a realização de heading com show precisa de seu owner e RED.
O L0 desta etapa declara esses limites, sem legitimar implementação provisória
como paridade completa. As demais pendências da auditoria de produção e da
matriz remota CI/release permanecem; esta rodada não publica artefatos.

## Validação final

Suíte inteira do workspace: **8425 passados, 0 falhas,
7 ignorados**, de 2026-10-14T05:34:22.644743+00:00 a 2026-10-14T05:40:55.107684+00:00, fontes preservadas.
Comando, ambiente /dev/shm, oráculo ratificado e hashes das fontes estão em
workspace-pre.json.gz; log e resultados completos permanecem no arquivo de
evidências. Os testes ignorados não contam como cobertura executada.

Build, check de todos os targets, rustdoc com -D warnings, fmt, diffcheck,
linhagem estrita V1/V5/V15/V26 e selo reverso passaram. A análise arquitetural
registra 0 erros, 382 warnings e 1631 notas.
Clippy foi inventário, não gate -D warnings: 227 antes e
227 depois; delta por regra: {}.
Não chamar build verde de zero violações ou certificação de produção.

## Evidências preservadas

Fontes antes/depois, testes, comandos, diagnósticos, PDFs, scripts e os quatro
arquivos próprios estão em evidence.json.gz com SHA-256 por entrada, verificados
após descompressão. Nenhum commit, push, PR ou release foi realizado.
