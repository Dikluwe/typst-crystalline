# Alcance de warnings de markup vazio — 2026-10-06

Os dois negativos da coleta de identidade de Source foram corrigidos. O warning
para ** agora destaca ambos os delimitadores, no documento certo, com expressão
válida ou com erro posterior. __ tinha a mesma causa e recebeu a mesma correção.
A coleta dedicada passou de **14/16 para 16/16** coincidências integrais de
stdout, stderr e status com o vanilla, sem projeção de texto/caminhos/carets.

## Evidência anterior à decisão

`empty-markup-warning-2026-10-06/before.json` conserva fontes, comandos, saídas,
UTC, HEAD, estado alterado e hashes dos binários para seis controles. A medição
mostra caret de dois caracteres no vanilla e um no Tekt em pares vazios,
Unicode e adjacência. Os controles não vazio e aberto já coincidiam.
`01_core/src/compiler/parse/parser/diagnostics.rs:93-103` insere um Error de
texto vazio; `compiler/parse/markup.rs:143-177` o embrulha em Strong/Emph.
A Source numerada conserva o marcador vazio e seu pai com len 2. O eval
publicava o span do marcador. O upstream pinado em
`lab/typst-original/crates/typst-syntax/src/parser.rs:145-169` associa o warning
ao construct já embrulhado. A forma da CST é mecânica; o alcance publicado do
diagnóstico é observável de linguagem (ADR-0107).

## Correção

O owner `00_nucleo/prompts/compiler/eval.md` foi atualizado antes do código;
a correção interna de paridade segue fluxo contínuo ADR-0127. Ao publicar
NoTextWithinStars/NoTextWithinUnderscores, o eval encontra o nó pelo span e
seleciona o span do pai direto Strong/Emph quando o marcador é vazio e o pai
possui comprimento dois. O vínculo vem da árvore numerada, sem texto de
mensagem, soma aproximada de offsets ou mudança da Source/CST. Na ausência
do vínculo esperado, mantém a origem original. Nenhuma assinatura, campo,
trait, fase ou default muda; nenhum unsafe foi acrescentado.

O despacho por SyntaxErrorKind é exaustivo. Um primeiro match com wildcard
produziu um aviso V16 adicional; foi substituído por braços explícitos dos
três tipos de erro restantes, e a versão final voltou ao total anterior de
avisos. Severity, mensagens, hints, ordem, quantidade de warnings e erros de
delimitadores abertos permanecem nos caminhos anteriores.

## Resultados e validação

- Coleta de identidade de Source: **16/16**, antes 14/16, incluindo os dois
  warnings antes negativos, imports, callbacks e leituras relativas.
- Seis controles adicionais: **6/6** integrais, incluindo __, Unicode,
  delimitadores adjacentes, conteúdo não vazio e delimitador aberto.
- Matriz macro: **36 MATCH/36**, com os critérios por modo anteriores:
  valores de query, DOM HTML, texto/páginas/bbox abaixo de 0,05 pt, raster com
  vizinhança de um pixel e stderr completo nos erros. Não é paridade global.
- O novo teste L1 passou e sete regressões P786a passaram; cinco testes CLI
  selecionados por warning passaram. O novo teste CLI faz parte dos cinco;
  a suíte completa de 112 testes não foi repetida neste reparo.
- Build debug/release e crystalline-lint passaram: **0 erros, 531 warnings,
  1578 notes**, sem V5/V6/V15/V26 nos três owners alterados. Não afirmar zero
  violations globais. Não foi medido ganho de tempo.

`red-core-fixed.log` mostra o teste falhando em 0..0 contra 0..2 antes da
implementação; `red-cli.log` conserva o caret curto. `red-state.json` registra
HEAD/árvore/fontes desse estágio. A primeira tentativa do teste L1 usou o nome
inexistente Source::range; foi corrigida para span_byte_range antes do RED
significativo, e o log de compilação foi preservado em red-core.log. Os testes
cobrem ranges UTF-8, newline fora do range, Source id, hints e controles.

## Proveniência e reprodução

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`.
Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`.
SHA-256: baseline `8af04155a7bacfa7dec3bb94cfc6faaafb74ed1a46abc010ac18a3a43144f668`,
Tekt final `11263dff535e04c00c15c8df48013bd4160431487e9b2c4ed25cc2bfd7ed12a7`,
vanilla `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

measurement.json.gz, controls.json e macro.json.gz conservam a fonte, comando,
UTC, cwd/estado de árvore e saídas de cada execução. measure.py, controls.py e
macro.py reproduzem as coletas usando os caminhos de binários declarados;
controls.py recria seus inputs a partir de before.json. execution.json.gz
preserva fontes/L0s finais e hashes dos logs. O release final ficou congelado
em /tmp/tekt-empty-warning/typst-final.

Warnings de depreciação/ordem em query, compile stdin, matemática automática e
table.cell continuam em frentes separadas. Este resultado fecha apenas os
negativos declarados deste conjunto.
