# L0 — Origem lógica textual de bidi
Hash do Código: 3de6e098

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:dd8c7661f2246385e2aed1737e8494d778b9f563b7ac709fc3c750a23fe50596
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8


**Layer:** L1.
**Exclusive consumer:** `01_core/src/compiler/layout/bidi_origin.rs`.

## Medição anterior à decisão

`00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` registra a refutação de resolução por fragmento com base igual. `paragraph.rs:15–51` já certifica direção integral; `text.rs:66` divide palavras e `space.rs:31` emite separadores. `initial.json.gz` guarda fonte, estado e binários. O contrato público foi ratificado em `bidi-contexto-aprovacao-2026-10-05.json`.

## Decisão e escopo

Helper privado de descendência de layout, sem ampliar API pública. Cursor Clone possui Arc input e próximo índice; next retorna span completo de Text/Space por ordem, Linebreak avança sem span. Preparar por travessia estrutural integral, Styled atualiza chain; Strong/Emph preservam ordem. Text guarda EcoString original e Dir horizontal efetiva; Space/Newline guardam origem; Linebreak guarda fronteira. Unsupported recusa toda preparação. Não usar busca de strings para descobrir origem.

Restringir a região simples e incremental: sem colunas/subframes genéricos/justificação (célula isolada exige capacidade especificada abaixo)/hifenização/smallcaps/tracking/optimized, Text sem newline/tab e sem words que pedem segmentação, sem transformação cujo texto não corresponda. Ativar nesta etapa apenas quando base RTL, run RTL ou run difere da base; LTR puro continua no planejamento anterior. Base fornecida pela direção lexical certificada, não pelo primeiro caractere. Cursor não é global, checkpoint clona, subframe suspende. O fragmento exato fica em campo privado do Layouter apenas durante emissão, depois é restaurado.

## Aceitação

RED→GREEN para travessia com duas palavras iguais e offsets UTF8 distintos, Space causal, override e unsupported None. Testes de fonte real por Sequence/Par, wraps e checkpoint. Zero I/O e unsafe.

Recusa integral não permite recertificar um filho parcial: guard privado de escopo em Sequence/Par impede reentrada; subframe suspende o guard com o cursor. Validar None para sequências com objetos opacos e folhas separadas.


## Envelopes de identidade no pipeline completo

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05/verification.json.gz` preserva o primeiro candidato; RTL mixed/LTR numeric ainda coincidem com baseline errado. `pipeline-context.log` registra RED pelo pipeline completo: todas as folhas sem span; `realize/mod.rs:38–44` insere Located em ocorrências. Avaliação direta não exercitava esse envelope. Correção interna: Located é transparente somente para certificação da sua body textual integral, sem ignorar corpos opacos nem alterar registro de Location. Direção lexical e coleta devem atravessar o mesmo envelope. O teste do pipeline exige spans em todas as folhas textuais e ordem RTL com primeiro trecho latino; preservar artefatos do primeiro candidato.

Linebreak justify=true está fora do certificado nesta rodada: o carrier unitário ratificado não transporta essa distinção. A fonte vanilla collect.rs:191–194 exige LF no caso padrão; a medição numeric-explicit está no receipt da implementação.

Certificação de Styled considera também deltas tipados: tracking, highlight, subscript/superscript e baseline deslocada produzem tinta/posição fora da reconstrução conhecida, logo recusam input inteiro, inclusive quando aparecem somente dentro do wrapper. Font/size/bold/italic/font-variation sem transformação de texto continuam permitidos. Par line numbering e floats pendentes recusam o certificado porque acrescentam material/alteram região. Medição de código: word_layout.rs:89–121 emite Shape para highlight; text/style.rs decodifica tracking tipado e baseline, rejeição só em chain.custom não cobrira o delta tipado. Teste RED direto de Styled com Tracking/Highlight/Superscript.

Medição da fronteira de Styled: style_scope.rs:42–69 trata pdf.marker.tag=label com subframe inline e outline.link.destination com Link. Esses canais internos também recusam a certificação integral, porque o reorder atual opera nas folhas diretas da linha; não certificar suas folhas por transparência presumida.

## Célula textual certificada — refutação 2026-10-06

### Medição anterior à decisão

focused-candidate-verification.json.gz e candidate-source.json.gz em
`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/` preservam candidato
com largura/âncora corretas mas ordem ABC/123/árabe incorreta em RTL.
bidi_origin.rs:111-117 recusa todas as células/subframes; o input integral
não chega à reordenação L3. cell_body.rs:223-235 classifica Text e TextShaped
como texto para rebase de baseline, mas não TextSeparator, introduzindo
um delta de top-edge nos separadores da mesma linha. São duas causas
internas refutáveis por ordem/baseline ainda divergentes após certificação
limitada e recomposição consistente. Correção de paridade interna, sem
contrato público, fase, flags de produto ou unsafe novo.

### Responsabilidade exclusiva

Permitir a região de célula somente com cell_bidi_allowed e regions.cell existente; subframes genéricos continuam recusados. Não mudar collect, gramática, chain, base lexical, ranges ou guards de transforms. Isso certifica a entrada original completa da célula, sem texto virtual autoral.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Certificação bidi atravessa Source sem alterar texto lógico, direção, ranges completos, chain ou ordem. wrapper de origem não causa Unsupported.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.
