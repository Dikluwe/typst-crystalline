# A1-H2 — suplemento de Heading antes da consulta e no HTML

## Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/fontes/diffstat em final-freeze.json.gz do diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17.
Oráculo bd5ebec285505889e8ff1d4f68804ab147a699f3, SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
target-controls.json.gz registra a regressão introduzida pelo transporte Styled:
heading(supplement:[Custom])[A].func() passa a styled; vanilla e baseline dão
heading. A consulta de Heading também expõe o wrapper causal temporário.
No PDF a callback panic já propaga; no HTML continua sendo ignorada.
Raw supplement continua ausente antes/depois, embora exista no vanilla.

Fonte: compiler/introspect/walk.rs:930–945 guarda Styles no próprio Content;
from_tags/headings.rs retira o wrapper somente no pós-passe runtime.
infra/pipeline/paged.rs:419–427 prepara a primeira consulta por walk puro;
infra/pipeline/html.rs:101–106 também usa o walk puro e não faz o pós-passe
runtime final do paginado. Isto é identidade/valor/erro observável da linguagem,
não necessidade de copiar a mecânica upstream. Os testes e gates de Rust da
revisão intermediária passaram, mas não certificam esses observáveis adicionais.

## Contrato proposto — gate ADR-0127 §2.1/§2.3

Adicionar exatamente esta fachada pública em compiler::introspect:

pub fn realize_heading_supplements(
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()>;

Ela finaliza somente entradas Heading com suplemento pendente e styles causais
capturados. Não executa State, Counter, equações nem faz paginação. Consome o
marcador privado de pendência: a mesma entrada já finalizada não reaplica a
callback. Nova introspecção pode produzir nova entrada pendente, como nas
passagens runtime existentes. Erro propaga antes da consulta/exportação.

Paged e HTML chamam essa fachada no snapshot inicial, antes de preparar context,
inclusive quando o snapshot veio da realização. O pós-walk runtime continua
finalizando ocorrências criadas na expansão contextual. Usar Engine com target
e features reais, World e Source causais; não chamar o pós-passe completo de
contadores/equações antecipadamente. Nenhum callback vai para walk puro,
layout ou encoder. O HTML continua target semântico separado, sem paginação.

A mudança de ordem é deliberada: o suplemento precisa estar realizado antes de
query observá-lo. Isso requer gate mesmo sem mudar os campos públicos existentes.

## Reparo interno de transporte e identidade

IntrospectedContent continua opaco e com as mesmas assinaturas públicas. Seu
ponteiro fields passa a guardar cabeçalho privado contendo mapa e styles pendentes
opcionais. Não acrescentar slot a Value/IntrospectedContent: preservar sizeof e
alinhamento medidos. O mapa continua único, ordenado e compartilhado por Arc;
fields() devolve o mesmo tipo. Styles auxiliares ficam privados, fora dos campos
públicos e do Content semântico. Clones são imutáveis. Não criar segundo store.

No construtor Heading, marcar privadamente somente seu transporte de propriedades
no Styles existente. As projeções reconhecem esse marcador e Heading direto;
projetam func/campos presentes com body por último. Styled de texto ou wrapper
externo continua sendo Styled: sem transparência geral, teste negativo obrigatório.
A igualdade semântica do suplemento (dívida A3) continua frente separada e não
será declarada fechada por esse transporte/reflexão.

## L0s a atualizar antes do código

- 00_nucleo/prompts/entities/value.md: cabeçalho privado, imutabilidade e budget.
- 00_nucleo/prompts/compiler/introspect/walk.md: capturar styles fora do Content.
- 00_nucleo/prompts/compiler/introspect/from_tags/headings.md: finalização por
  índice causal, pendência consumida e snapshot canônico; não inferir pelo corpo.
- 00_nucleo/prompts/compiler/introspect/entrypoints.md: fachada pública e delegação.
- 00_nucleo/prompts/compiler/introspect/fixpoint.md e from_tags.md: chamadas internas.
- 00_nucleo/prompts/compiler/stdlib/structural/heading.md e
  00_nucleo/prompts/entities/content/fields.md: marcador exclusivo de construção.
- 00_nucleo/prompts/compiler/eval/bindings/field_access/content_methods.md e fields.md:
  identidade/presença nominal, sem tornar Styled em geral transparente.
- Novo owner 00_nucleo/prompts/infra/pipeline/runtime.md, consumer
  03_infra/src/pipeline/runtime.rs: composição privada de Engine/ctx com target e
  features, e chamada da fachada. 00_nucleo/prompts/infra/pipeline.md declara o filho.
- 00_nucleo/prompts/infra/pipeline/paged.md e html.md: preparar snapshot antes de
  context, propagar erro e warnings. Context_stabilization.md: target/features
  corretos no runtime candidato, preservando política de tentativas.
- Prompts dos testes proprietários: somente controles medidos e budget privado.

## Aceitação H1–H8

H1 func e campos raw do construtor, preservando omissão e ordem; H2 wrappers
Styled ordinários continuam opacos; H3 query devolve Heading e Content supplement,
inclusive arrays/closures e mudanças de estilo posteriores; H4 captura léxica,
body/depth/lang e escopo local, duas ocorrências com mesmo corpo; H5 panic em PDF
e HTML propaga; H6 default/auto/none e numeração preservados; H7 mesma entrada
finalizada não reaplica callback, novas ocorrências causais são independentes;
H8 sizeof/alinhamento de Value/IntrospectedContent preservados, clones sem mutação,
workspace/doctests/build/Clippy/Rustdoc/linhagem/fmt sob 12 GiB, High8GiB, swap0,
jobs1 e tarefas pesadas sequenciais. Comparar semântica/morfologia/erro, não bytes.
Não ampliar outras APIs públicas, política de fixpoint, publicação ou release.
