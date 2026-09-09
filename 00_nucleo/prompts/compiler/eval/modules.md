# Prompt L0 — `compiler/eval/modules`
Hash do Código: d714fe3f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/modules.rs`

## Medição e contrato

Imports/includes resolvem somente por World, detectam ciclos na Route e
preservam FileId. Import avalia em Engine/Scopes isolados e exporta bindings;
include produz conteúdo na rota filha. Bare, rename, wildcard e items seguem a
linguagem; path enraizado não é re-resolvido.

## Aceitação

Ficheiro, pacote, módulo, aliases, include, ausência e ciclos têm testes focais.

## P1327 — warning do bare import de identificador

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1327-baseline.json`, SHA-256
`f8cee37f7f3556db93f935deb977790a0a13ddd232639334e3d3931cf8b504f8`,
fixa HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não
commitado com diff/stat integral, UTC inicial `2026-09-09T10:40:09.228317+00:00`,
binários e argv. O vanilla ratificado `a51e02804` emite
`this import has no effect` no identificador fonte em imports bare de std,
alias e calc; o baseline P1326 preserva os valores, mas omite o warning.
O aviso permanece quando um erro posterior aborta a expressão. Fontes
field, as, items e wildcard não recebem esse aviso. Rename redundante e
erro de tipo já divergem e permanecem fora desta correção.

`01_core/src/compiler/eval/modules.rs:207-219` liga o Module sem avisar.
`lab/typst-original/crates/typst-eval/src/import.rs:82-103` exige fonte
Ident e ausência de rename/lista, após resolução e validação do bare name.
Isso não distingue módulos nativos de ordinários: a condição é sintática.

### Decisão e aceitação

Depois de resolver com sucesso uma fonte Module já suportada e validar seu
bare name, se não houver `as` nem lista e a fonte for Ident, emitir no sink
um warning `this import has no effect`, sem hints ou trace, com span exato
do identificador fonte. Continuar ligando o mesmo módulo sob o nome lexical;
não abortar, omitir binding, alterar lookup ou exigir origem nativa.
Aliases, sombras, closures e módulos importados obedecem à mesma regra.
Cada import distinto conserva sua âncora; o sink mantém sua deduplicação
vigente. Erro posterior não apaga warning já emitido.

Não emitir esse warning para field access, literal de arquivo, rename
(mesmo redundante), items, wildcard, bare dinâmico rejeitado ou falha na
resolução/tipo da fonte. Não ampliar tipos importáveis, paths dinâmicos,
rename com items, includes, resolução, Route, ciclos, avaliação isolada,
ordem de erros ou APIs. Warning de rename redundante permanece dívida.

Esta seção sucede somente a dívida de warning bare identifier de P1305-r2
abaixo. Correção interna de paridade ADR-0127 em fluxo contínuo: L0 primeiro,
RED→GREEN e revalidação. Testes exigem mensagem, severidade, cardinalidade,
hints/trace e ranges integrais, valores e erros posteriores preservados,
perfis default/html/a11y/html+a11y e controles sem aviso. É inferência que
este owner basta; qualquer necessidade de carrier/proveniência, alteração
de API ou outro consumer produtivo refuta o escopo. Unknown não fecha nada.

## P1305-r2 — global em imports e nome lexical de ligação

### Medição anterior à decisão

`01_core/src/compiler/eval/modules.rs:66` constrói o global do arquivo
importado como `Module::new("std", stdlib.clone())`. O arquivo importado
é construído separadamente com o nome do arquivo em `:113-114`. Em
`:184-188`, importar uma expressão Module extrai seu `name()` como nome
de ligação padrão; `:203-213` usa esse nome quando não há `as` nem lista.
Trocar apenas o nome do objeto global faria `import std` criar indevidamente
um binding `global`.

O vanilla `a51e02804`, em
`lab/typst-original/crates/typst-eval/src/import.rs:82-105`, usa o nome
lexical da fonte; sua construção global usa o nome público `global`
(`typst-library/src/lib.rs:374`). A API AST já existente neste projeto,
`01_core/src/entities/ast/code.rs:184-206`, retorna nome do identificador,
nome do field ou stem do literal, e classifica outras formas como Dynamic.

`00_nucleo/diagnosticos/p1305-r2-pre-measurement.json`, SHA-256
`fd6354824e500e61f18b14116dd54b4f4838691289226a7f7e04236258dd9da0`,
registra todas as execuções entre `2026-09-07T13:48:28.580684+00:00` e
`2026-09-07T13:51:19.266604+00:00`, no HEAD
`8eb41b769eb840c7ab1063f981f98fdd4047952b` mais diff P1303 não commitado,
com status/diff/stat. Binários: baseline
`4a4e1bd46c053dd19bfcae2230537c9478fbfb2ff26a94dfd87d9f29fee2835d`;
vanilla `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
`global-bare-unbound` e `global-alias-bare-unbound` falham bilateralmente
por variável `global` desconhecida; vanilla ainda emite warning de import
sem efeito, ausente no baseline. `global-field-bare` liga `saved` no vanilla,
mas o baseline erra por `saved` desconhecido. `global-dynamic-unnamed`,
`global-dynamic-block` e `global-dynamic-call` medem a rejeição vanilla
`dynamic import requires an explicit name`, hint
``you can name the import with `as` ``, com span somente na expressão fonte.
O baseline aceita a importação dinâmica e pode falhar depois; não é paridade.
Com `as`, items ou wildcard, os controles dinâmicos são válidos.
`imported-route`/`nested-import-route` exercem o global construído neste owner.

### Decisão estreita

O global usado para avaliar cada arquivo importado guarda o nome público
`global` no Module existente e continua ligado lexicalmente como `std`.
O Module retornado para o arquivo conserva o nome próprio derivado do
arquivo/pacote. `std.typ` e reexport de `std: *` continuam módulos `std`;
conteúdo não serve de discriminante. Não alterar carrier ou construção de
módulos ordinários.

Para fontes que avaliam ao Module já suportado, a ligação bare sem `as` e
sem lista de imports usa a forma sintática existente, independentemente do
nome público guardado: identificador liga sob seu nome lexical; field access
liga sob o nome do field; literal de arquivo conserva o nome de arquivo.
Importar um alias do global não cria `global` nem troca seu nome público.
Não especializar `std`, `global`, nomes de arquivos ou testemunhas.

Somente nesse modo bare sem `as` nem lista, uma fonte Module dinâmica cuja
AST não possui bare name deve falhar na fonte com um erro
`dynamic import requires an explicit name`, exatamente um hint
``you can name the import with `as` `` e sem diagnostics laterais novos.
Preservar o span da expressão fonte integral, não da instrução inteira ou
de um uso posterior. Não criar binding antes desse erro. Esta validação
impede que a correção do nome torne bem-sucedido um programa inválido.

`as` explícito continua ligando sob o identificador escolhido. Items e
wildcard continuam copiando os bindings solicitados, inclusive quando a
fonte é uma expressão dinâmica válida de Module. O guard de bare name não
se aplica a esses modos. Resolver fonte, erro de tipo, World, Route, ciclos,
ordem de avaliação, includes e demais contratos permanecem os vigentes.
Não ampliar imports para tipos/funções/paths dinâmicos ainda não suportados.

Warnings vanilla de import sem efeito e rename redundante são dívida
histórica medida, não serão implementados nesta correção. O diagnóstico
nominal preexistente de field ausente em import `std.typ` também permanece
dívida do owner `bindings/field_access`; este passo não o edita.

O dono autorizou a ampliação dos owners de construção. Preservar a ligação
lexical e rejeitar bare dinâmico são ajustes privados necessários neste
mesmo owner para não introduzir novos bindings ou aceitações pela mudança
do nome. Classificação: correção de paridade em fluxo contínuo ADR-0127,
sem nova entidade, assinatura, default ou fase. Refutam-na binding baseado
no nome público, perda de rename/items/wildcard, guard em modo válido,
regressão em import ordinário ou novo warning/erro alheio à fronteira.
Nenhum `Unknown` obrigatório satisfaz aceitação.
