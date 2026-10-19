# L0 — compiler/layout/sequence/inline_prefix
Hash do Código: 730271c3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/prepared-text-carriers.toml sha256:b2899862f439550920cdee8b3aec6a63c441b9bd2770e563d0a8fd49f93572d2
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/sequence.rs:73-558`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/sequence/inline_prefix.rs`. Certificar prefixos inline e a reserva vertical antes do primeiro bloco fracionário. Preservar os limites de estilos, largura, wrap e boundary descritos em sequence.md. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-second refuta Source como novo fragmento tipográfico: certificado de prefixo inspeciona body transparente de Source, inclusive sob Styled/Strong/Emph, preservando medidas/leading/estilos/limites de largura.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Escopo textual e cauda fracionária — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/scope-tail-before/results.json.gz`
registra HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/status/diffstat e hashes de fontes/binários contra vanilla pinado9f2b6e871.
Os lotes block-par-spacing-after e block-par-extra-after anteriores já
refutam escopo por consenso de runs: um wrapper18 em Lead More exige margem
21.6pt, enquanto dois wrappers18 adjacentes mantêm o escopo exterior.
sequence.rs:196–245 fecha prefixos na chain exterior; paragraph.rs:12–90 já
resolve direção pelo ancestral lexical comum. Fonte vanilla flow/collect.rs:
129–145 resolve estilos comuns antes de compor linhas e espaçamento.
A cauda TAIL top10/bottom0 após Block1fr ocupa página2 contra página1 no pin:
cursor.rs:127–135 rejeita igualdade com o limite embora a cauda esteja
certificada. Cauda Styled18 fica10.52pt acima; fractional_preflight.rs:170–211
mede nós individualmente e só corrige a altura de Text direto. Linebreak
explícito também excede a região. A nova sonda inclui wraps, fonte8/18,
Strong, wrappers irmãos, Align e colunas. Isso é geometria e paginação de
linguagem; não exige estrutura ou bytes iguais. before-edit.json.gz em
scope-tail conserva os consumers/owners anteriores.

### Classificação

Correção interna de paridade ADR-0127: L0 primeiro, RED→GREEN e revalidação.
Sem API pública, nova entidade, fase eval/layout ou default novo. Refutar
por wrappers irmãos, preservação da ordem, última linha, corpos opacos,
colunas e ausência de efeitos/estado vazando da medição. Não copiar defeito
upstream nem declarar paginação universal para cauda maior que a região.

### Responsabilidade exclusiva

O registro runtime privado distingue prefixo geométrico certificado pelo
owner paragraph do prefixo escalar legado. Preservar as fórmulas e gates
legados; o marcador permite que Sequence faça somente o fechamento lexical
na nova rota e não repita sua boundary. Sem entidade pública nova.


## Prefixo multiline e aninhamento parcial — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/stat/UTC, fontes e binários contra o pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8. Multiword21 gera duas páginas
contra uma; Long/More estão 6.5/13pt abaixo. O prefixo40/8 com fill gera duas
páginas; More está39.906pt abaixo. Nested OUTER18/INNER8 tem BODY12pt acima
e página auto12pt menor. sequence/fractional_preflight.rs:151–171 recusa
multiline; word_layout.rs:49 fecha dentro do estilo local; line_commit.rs:
398–480 usa esse estilo para floor/leading/top previsto. sequence.rs:239
recusa fechar segmento parcial; Block sem cadeia anterior usa spacing8pt.
Vanilla flow/collect.rs:129–145,179–214 resolve trunk dos filhos, spacing e
leading comuns; cada frame contribui sua geometria. Esses são observáveis
textuais, geométricos e de paginação da linguagem (ADR-0107), não obrigação
de copiar buffers, algoritmos ou bytes. A intenção histórica não é inferida.

### Decisão e limite

Correção interna de paridade ADR-0127 em fluxo contínuo: sem entidade,
trait/assinatura pública, nova flag/default ou fase. O núcleo text-segment-scope
pinado define as invariantes compartilhadas. Aceitação RED→GREEN dos três
casos, controles de fontes, cores, margens, colunas, restauração, checkpoint
e subframes, mais suíte completa e linhagem. A nova rota só amplia prefixos
multiline textuais positivamente certificados; não declara paginação universal,
bidi/floats nem conteúdo opaco. Preservar os certificados escalares anteriores.

### Responsabilidade exclusiva

Adicionar bool multiline privado ao InlinePrefixRuntime; false nos certificados anteriores, true somente na nova medição positiva. Não remover gates legados nem inventar novo campo público.


## R2-S — envelopes preparados e prefixo fracionário (2026-10-18)

### Medição anterior à decisão

No HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, complete5/complete-alias-layout.log em
lab/results/construction/diagnostics/sequence-constructor-2026-10-17/ registra UTC,
estado exato e o controle com/sem show sequence identidade. Prefixo *Lead*
antes de Block1fr em columns passa de uma para duas páginas no Tekt;
o mesmo alias conserva uma página no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
Prefixo simples conserva geometria nos dois. inline_prefix.rs:181–231
inspeciona somente source_body; apply.rs:551–688 localiza a ocorrência
Strong mesmo sem transformação aplicável. paragraph.rs:19–98 aceita Located
mas recusa Realized. Isso refuta transparência da identidade no certificado,
não justifica remover tags ou a localização. carriers-before.json.gz guarda
fontes/L0 antes deste reparo, HEAD, diffstat e UTC.

### Decisão e aceitação

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público,
nova fase/default ou entidade. Projeção visual emprestada conforme núcleo
pinado; nenhum callback executado. Fórmulas, gates de estilo, largura, wrap,
profundidade tipográfica e fallback opaco permanecem. Não ampliar a projeção
ParagraphBody nem promover corpo opaco a Par/vazio. Inferência refutada se o
alias mudar páginas, palavras ou coordenadas, ou se conteúdo opaco for aceito.
RED funcional de prefixo Strong/Emph/Styled com Source/Located/Realized,
escolha visual diferente do original e recusa de Link; GREEN mais controles
PDF com/sem alias dentro de cada compilador, três prefixos, tolerância 1e-6pt.
Conteúdo emitido conserva seus carriers e tags. Suíte completa e gates sob
12GiB, swap zero, jobs1, sem alterar esperados anteriores.

Responsabilidade exclusiva: consultar a projeção do owner paragraph no nó
e no body de Styled/Strong/Emph, nos quatro pontos que antes usavam
source_body. Não reconstruir conteúdo nem aumentar a profundidade dos
wrappers tipográficos admitidos. Testes cfg(test) neste consumer verificam
a reserva legada igual à do conteúdo cru, seleção de visual em vez de original,
recusa de Link e estilo lexical compartilhado através de Realized.

Refinamento da fixture: carriers-green.log confirma a cadeia visual mas
recusa o baseline Styled16 singleton. O gate legado P1381/P1384 exige o
segundo fragmento quando o primeiro é maior; esse teste prospectivo não
constitui regressão. Usar Styled8 na célula admitida e testar recusa de
Styled16 tanto cru quanto preparado. Não ampliar o certificado para fazer
passar a fixture. Os dois REDs anteriores de transparência continuam válidos.

complete-attempt6.json.gz confirma as comparações de certificado, mas a
asserção final da fixture pressupunha 12pt; Layouter inicializa sua StyleChain
vigente em 11pt. Capturar o estilo inicial antes de medir e exigir a sua
restauração exata. Não mudar o constructor ou o estilo de produção. O controle
PDF carriers-pdf confirma delta zero nos seis pares de prefixo/compilador.


## R3 — Sequence com uma unidade visual no prefixo escalar (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/sequence-r3-2026-10-18/markup-geometry-before.json.gz,
markup-geometry-before-resources.json e prefix-before-sources.json.gz registram
UTC, HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/working tree/diffstat,
fontes, argv e hashes de binários contra bd5ebec285505889e8ff1d4f68804ab147a699f3.
O R3 preserva corretamente a Sequence interpolada no valor de markup e
None como filho real. Com prefixo strong e Block1fr em columns, as formas
#(strong[Lead]+[]) e #[#none;*Lead*] passam de uma para duas páginas no Tekt;
vanilla conserva a geometria, assim como os controles de prefixo simples e
Styled. Maior delta observado212.762pt, por migração de página. O dispatch
escalar em inline_prefix.rs:172–246 recusa Sequence mesmo quando contém só
uma unidade visual entre Empty. paragraph::shared_chain já certifica Empty e
Sequence; recorrer ao fallback geométrico perde a reserva escalar vigente.
Isso é geometria da linguagem, não razão para apagar filhos reais do valor.

### Decisão e responsabilidade exclusiva

Reparo interno de certificado, ADR-0127 contínuo. Depois da projeção preparada
Source/Located/Realized existente, helper privado scalar_prefix_body devolve
None para Empty e grupos inteiramente Empty. Para Sequence com exatamente
uma unidade não Empty, devolver por empréstimo essa unidade, recursivamente;
com duas ou mais, devolver a Sequence intacta, que o certificado anterior
recusa. As demais variantes são enumeradas e devolvidas intactas numa única
política de forwarding V19. Não descartar Space, Text vazio, Linebreak,
Styles ou outro elemento. Não emitir, reconstruir ou executar callback.

O loop ignora projeção None como o Empty anterior, mantendo part_index do vetor
original. Os bodies de Styled/Strong/Emph usam a mesma projeção; em seguida
continuam o match Content anterior, sem ampliar profundidade tipográfica,
gates de estilos/size/largura/wrap, fórmulas, hooks, Location ou tags emitidas.
Nenhum campo/assinatura pública, nova fase/default ou normalização de valor.
Não promover grupo com dois textos ou Space+Strong a certificado escalar.

RED de reserva para Strong/Emph/Styled em Sequence singleton, com Empty,
Empty aninhado e carriers preparados, incluindo body Strong com None; controles
de recusa de dois textos, Text vazio real+Strong, Space+Strong e Link.
GREEN e repetição dos18 PDFs/12 comparações geométricas dentro de cada
compilador, páginas/palavras exatas e tolerância1e-6pt. Não é prova de igualdade
geométrica entre Tekt e vanilla. Exigir suíte completa, Clippy/Rustdoc estritos,
linhagem/diferencial e demais gates sob12GiB/High8GiB/swap0/jobs1.


## V16 após tornar a projeção explícita — 2026-10-18

### Medição anterior à decisão

validation-attempt1.json.gz/prefix-exhaustiveness-before.json.gz no diagnóstico
R3 preservam a rodada completa: 8979 testes e5 doctests passaram, Clippy e
Rustdoc estritos passaram, mas o diferencial Tekt falhou. O baseline R2 tem
106 V16 e nenhum neste owner; o candidato tem107, com V16 no wildcard externo
em inline_prefix.rs:363. O match part agora é reconhecido pelo linter como
Content. As três recusas dos bodies também usam wildcard (linhas330/342/354),
embora a ferramenta não as reporte nesta forma. Não se trata de falha da
fórmula nem de razão para ocultar o tipo ao linter.

### Responsabilidade exclusiva e aceitação

Fechar os quatro matches Content do certificado: no externo, manter braços
positivos Text/Styled/Strong/Emph/Space/SpaceFromNewline/Empty e substituir só
o wildcard pela lista explícita de todas as outras variantes. Nos três
bodies, manter Text e enumerar todas as demais variantes na recusa. Cada
grupo V19 compartilha return None; preserve integralmente corpos, guards,
returns, ordem, fórmulas e os critérios do certificado. Sem supressão, if-let
ou alteração do limiar. Variante futura exige decisão Rust explícita.

Registrar replay nominal dos padrões/complementos e fonte anterior. O finding
novo deve desaparecer, sem atribuir remoção de débito do baseline ao reparo:
106→107→106, nenhum novo erro/warning. O forwarding de append, embora agora
exaustivo, não emitia V16 no baseline e não pode receber crédito106→105.
Resselo após ownership/núcleos válidos; repetir os gates completos no mesmo
freeze final e conservar a tentativa anterior, sob12GiB/High8GiB/swap0/jobs1.
