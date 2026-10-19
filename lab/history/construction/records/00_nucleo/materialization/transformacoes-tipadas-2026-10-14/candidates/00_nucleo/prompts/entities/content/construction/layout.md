# L0 — entities/content/construction/layout
Hash do Código: a5b5711a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/entities/content/construction.rs:837-1102,1193-1350`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/content/construction/layout.rs`. Construtores de blocos, geometria, espaçamento, quebras, posicionamento e colunas; preservam variantes e valores originais. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

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

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Content::smartquote(double:bool) preserva sua assinatura e cria double Some(double),
enabled/alternative/quotes None. Ajustar somente inicializações SmartQuoteElem,
sem trocar campos/semântica dos demais constructors. Nenhum glyph no constructor.

## Transformações tipadas — contrato substitutivo

## Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/{before,results}.json.gz` e
`controls/results.json`, `binding-controls/results.json`, `context-controls/results.json`
preservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC,
diffstat, fontes, comandos e hashes de binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8.
TransformElem em entities/elements/transform.rs:19 armazena apenas matrix/body;
stdlib/transforms.rs:19/51/91/191 descarta argumentos e reduz valores à matriz;
layout/transform.rs calcula pivô central e usa has_orphaned como desvio de layout.
No pin, typst-library/src/layout/transform.rs conserva os quatro elementos e
typst-layout/src/transforms.rs resolve estilos, região, corpo, origin e reflow.

Sondas: fields vazio versus body/argumentos; func falso versus função nativa;
rotate(0deg)[X] == scale(100%)[X] true versus false; defaults explícitos colapsam.
Origin/reflow são ignorados ou rejeitados; scale(x:150%) escala também y no
Tekt, enquanto y permanece100% no pin; move perde percentual/em. Campos ausentes
devem continuar desconhecidos lexicalmente. set rotate(23deg) é válido, mas
angle nomeado não é; getters contextuais refletem estilo no pin. Os primeiros
controles contextuais rotularam o wrapper e são inválidos para medir value;
context-controls corrige a posição da label. HTML pinado avisa e elimina o body;
Tekt rejeita a operação, de acordo com seu contrato HTML estrito.

Semântica, sintaxe, morfologia, posições/avanços e diagnósticos são linguagem;
enum versus structs upstream e número de passos de medição são mecânica.
A hipótese é refutável por campos/presença que ainda colapsem, estilo assado,
origem incorreta, erro perdido, reflow falso alterando avanço ou body descartado.

Esta seção substitui cláusulas anteriores conflitantes somente para a família
Move/Rotate/Scale/Skew; demais responsabilidades permanecem.

## Construção Rust genérica

Content::transform(matrix: TransformMatrix, body: Content) -> Content mantém a
assinatura e passa a construir TransformElem { operation:
TransformOperation::Matrix(matrix), body }. Nenhum argumento nativo é inferido
da matriz. Demais constructors desta unidade permanecem. NativeMove/Rotate/
Scale/Skew são construídos pelo owner stdlib, com presença tipada independente.
