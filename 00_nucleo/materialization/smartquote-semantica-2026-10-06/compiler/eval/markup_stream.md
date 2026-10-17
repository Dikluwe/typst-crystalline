# L0 — compiler/eval/markup_stream
Hash do Código: 972f0c1f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:1351-1650,2238-2261`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/markup_stream.rs`. Coordenação da sequência de markup, avaliação do corpo e apresentação de valores; preserva escopos, ordem e spans. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

## Inserção de aliases na realização — 2026-10-06

Medição: core-second.log registra ocorrência local devolvendo Original onde o oracle em controls.py devolve Local. markup_stream.rs:178-184 insere valores sem aplicar o snapshot de regras locais: a receita exterior perde esse snapshot ao devolver. Na realização produtiva, interceptar o conteúdo inserido no próprio ambiente lexical antes de append_markup_sequence; a proteção por ocorrência e produtos concluídos é delegada ao owner apply, não duplicada aqui. Fora de realizing_show_rules preservar o caminho existente. Preservar spans já conhecidos; erros sem origem desta inserção recebem o span real da expressão, sem procura por mensagens.

core-third.log registra a regressão P790 quando a intercepção geral repete a regra textual. A inserção produtiva delega exclusivamente a fronteira interna realize_inserted_occurrences do owner apply; literal/regex/text e preparação de parágrafos permanecem no caminho existente. Esta delimitação supersede a indicação genérica intercept_content acima.


## Aspa semântica e resolução tardia — proposta aguardando ADR-0127

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão proposta e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. PARAR
antes de testar/materializar este pacote até o dono confirmar. Candidatos
são cópias completas para revisão, não os L0s vigentes. Após confirmação,
substituir cláusulas conflitantes do contrato histórico apenas para aspas;
ADRs/núcleos existentes e propostas alheias permanecem intactos.

SyntaxKind::SmartQuote cria Content::smartquote(kind.double) com propriedade
double explícita, sem consultar lang/quotes/adjacência/enabled para escolher
glyph. Preservar Source/spans, snapshots léxicos de Styles e sequência.
Aplicar fronteira estrutural de show existente aos SmartQuote, com guards,
identidade causal e captura, sem encaminhar a folha como Text para regras
textuais. Raw Text e strings code continuam literais. Aliases/receitas usam
fronteira realize_inserted_occurrences existente; não executar show duas vezes.
As decisões contextuais saem desta unidade; byte offsets ainda usados por
outros observáveis não são removidos indiscriminadamente.
