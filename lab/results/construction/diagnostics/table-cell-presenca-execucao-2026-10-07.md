# D08B2 — presença lexical ratificada e materializada

O escopo ratificado de D08B2 está materializado. Os 72 controles lexicais
coincidem com o vanilla pinado. D08B3 de realização permanece aberto.

## Aprovação e medição anterior à decisão

Confirmação humana `confirmo` recebida para o manifesto SHA-256
`35f73c60301e1e4b27ae8b3ed2928b6d421dbcd0f32502131c3e2458260301e6`.
Recibo: `00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`.
Os hashes atuais dos 15 owners e dos candidatos foram verificados antes de
instalar L0s/núcleo, sem reconciliar alterações externas não autorizadas.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`table-cell-presenca-2026-10-07/{baseline-state,before,red-code,empty-red-code,tested-final-code,final-code}.json.gz`
registra estado, conteúdo/hash de fontes, diff HEAD --stat, status e UTC.
As coletas before/after/final incluem binários, pin, argv/cwd/ambiente e
stdout/stderr/PDF text por controle. Não usar a versão pública como proveniência.

Oráculo: pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`lab/typst-original/target/release/typst`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt antes: `53cf87137baa268e30effaef978455611b3e2dbdfba90d43d1f446e5eef0e456`.
Tekt final: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Upstream model/table.rs:734–778 declara os campos de célula. A entidade Tekt
anterior table_cell.rs:40–51 conservava somente Option de valor; o constructor
stdlib/structural/table_grid.rs:438–482 colapsava omissão/auto/none. São
presença e morfologia da linguagem, sem exigir a estrutura Rust do upstream.

## Implementação e limites de responsabilidade

TableCellElem recebe o carrier público aprovado TableCellPresence; os tipos
anteriores dos valores permanecem. Omitted, Auto e Explicit são nominais;
fill/stroke none explícitos ficam conhecidos. Body permanece conhecido.
X/y admitem auto ou inteiro não negativo; spans exigem inteiro positivo;
none e auto ilegais são rejeitados no value_span. O extrator global de grid
não mudou. Stroke Length/Color assenta somente o componente fornecido.

Getter, fields/has/at e JSON/YAML conservam presença e ordem dos dez campos.
Body Empty projeta uma sequência vazia, sem mutar o body guardado. A fonte
antes do refinamento e o contraexemplo estão em after.json.gz, empty-body;
empty-red.log registra falha real antes de alterar essa projeção.

Clone, map_content/map_text, travessia pré-ordem, normalização e materialização
temporal conservam presence. ObservationEq destructura a entidade e o carrier
exaustivamente e compara seus estados nominais; pixels/corpo iguais não provam
igualdade causal de células com presença distinta. Fixtures de layout/HTML
migram para o campo novo sem alterar observáveis/tolerâncias.

L0s foram instalados antes de código, com Núcleo Tekt
`00_nucleo/prompts/_nuclei/table/cell-presence.toml`, pin completo
`afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c`.
Resselo foi restrito aos 15 owners. final-results.json e seals.json confirmam
zero alteração produtiva/L0 fora do escopo, preservando a árvore anterior.
Não há NodeKind cell, promoção a locatable, registry ou fase nova.

## RED→GREEN e validação final

red.log registra três testes executados e falhando: none explícito, auto com
transformação e rejeições/span. Não são falhas de compilação usadas como RED.
empty-red.log registra o segundo RED de morfologia do corpo vazio.
Tentativas intermediárias de integração permanecem arquivadas; a aceitação
usa exclusivamente os logs finais limpos e snapshots correspondentes.

Números abaixo vêm de final-results.json, logs finais e coletas com UTC,
HEAD/working tree, diffstat e hashes exatos. A mesma identidade de binário final
foi verificada nas regressões bilaterais.

- Core: 6.588 passaram, 5 ignorados; infra: 1.194 passaram.
- Shell completo: 90 passaram; integração query: 10 passaram.
- Cargo build debug/release concluídos; git diff --check passou.
- Crystalline-lint: zero erros, 532 warnings e 1.607 notas; os warnings legados
  não foram escondidos por resselo global.
- Coleção lexical D08B2: 72/72. Coleção completa: 75/92, com 17 negativos
  de realização preservados. Os três shows coincidentes anteriores ficam
  registrados sem certificar campos resolvidos.
- D08A: 21/21; seus dois negativos D08B históricos continuam fora do escopo.
- D05: 29/29; D07: 29/29 + 14/14; query bilateral: 36/36.
- D06: 38/41, com root-render, raw-eval e raw-mat-eval ainda divergentes.

As duas coletas de matriz verificadas rodaram sequencialmente porque os scripts
usam o mesmo diretório temporário. As coletas concorrentes anteriores não são
usadas como prova de aceitação; verified-cell-presence e seu additional são
as evidências finais registradas em final-results.json.

## O que continua aberto

Os 17 negativos da coleção completa são show-bare/show-explicit/
show-selector-explicit/show-padding/show-body, local/late scope, reuso, nested,
header/footer e as sete coletas de propriedades resolvidas (default, padding,
colspan, rowspan, styles, posição explícita e linhas fixas). Pertencem a D08B3:
posição lógica, defaults/herança, padding e momento de show. Não fechar esses
casos com normalização de whitespace ou defaults inventados.

Os refutadores de casts separados dão 4/7 coincidências. Continuam distintos
stroke:0pt (vanilla aceita), align:string e inset:integer (vanilla rejeita,
Tekt conserva seus caminhos legados). Não houve nova mudança de compatibilidade
para resolver esses casos. Inset relativo/dict, paints e strokes ricos além dos
carriers atuais permanecem escopos futuros; 72/72 não é paridade universal de cell.

D08B3 exige proposta concreta própria para o contrato/fase, sem duplicar o
algoritmo de placement, introduzir identidade por nome ou promover cell ao índice.
Esta ratificação de presença lexical não autoriza aquela mudança de fase.
