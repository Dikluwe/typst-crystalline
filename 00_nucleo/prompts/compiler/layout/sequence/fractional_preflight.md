# L0 — compiler/layout/sequence/fractional_preflight
Hash do Código: 0d692e1f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/block-spacing-resolution.toml sha256:2e2f3cfb51bd0c484e65cc9e5c3e1eee9f3dad12c77aa9330674b474e11f3342
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/sequence.rs:19-72,559-747`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/sequence/fractional_preflight.rs`. Calcular a parcela fracionária da região e medir a cauda fixa antes da emissão. Preservar os guards de região, a base herdada ou restante e a restauração do estado de medição. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-sixth mede reservas finais divergentes quando Text transporta Source: classificação de fragmento antes de Block fracionário e cauda transparente deve inspecionar body, preservando fórmula top/bottom-edge existente e reconhecimento de Block através de wrappers. Nenhum default ou estimativa geométrica nova.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Fronteira fracionária geométrica — 2026-10-14

Medição anterior: implementation/block-boundary-layout-{pre,result}.json.gz
registra HEAD/árvore/UTC/diffstat/fontes; o log mostra caudas After222.3 contra
Tail230 sob FixedMetrics após retirar top7.7 da origem. A sonda bilateral
block-boundaries-before/results.json.gz já demonstrou que a baseline de One
precisa subir o top7.238 das métricas reais, enquanto After/Tail já coincidem.
sequence/fractional_preflight.rs:140–141 reserva prefix.fixed_height, que em
inline_prefix.rs:484 inclui também o top da próxima baseline; a nova origem
geométrica de Block já o retira. Portanto a parcela precisa devolver esse
espaço reservado, não deixar um buraco no fundo. Critério é a geometria dos
filhos e da cauda, não a forma de calcular a distribuição Rust.

Contrato exclusivo: para prefixo certificado que fecha linha, descontar uma
única vez o top-edge exterior da reserva fixa de prefixo antes de distribuir
os pesos fracionários. Prefixo vazio não tem essa reserva. Conservar guards,
caudas, pesos, ausência de efeitos e protocolo de prefixos. Não adicionar
compensação ao resultado de cada bloco: a distribuição partilha a região
livre corrigida. Correção interna ADR-0127, sem API/fase nova. Regressões
existentes de prefixos, wraps e caudas são obrigatórias; L0 primeiro e resselo.


## Margens automáticas de Block — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/transformacoes-2026-10-14/block-par-spacing-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC, diffstat, hashes de fontes/binários e pin vanilla 9f2b6e871. Com fonte10,
top10 e bottom0, PRE seguido de Block20 produz página62 no Tekt contra50/55/70
com par.spacing0/5/2em. Block20 below0 seguido de texto produz página40 contra50.
block.rs:127–130,585–588 e sequence/fractional_preflight.rs:216–227 substituem
par.spacing por 1.2em; cursor.rs:103–110 omite top quando below zero.
Fonte pinada typst-layout/src/flow/collect.rs:253–258 resolve Auto por ParElem::spacing.
`block-par-extra-before/results.json.gz` mede dois Blocks1fr numa região80:
a baseline de B deve ser60/62.5/70 para spacing0/5/2em, contra66 em todos.
São posições/paginação observáveis da língua. Não é obrigação de copiar o Rust.
A inferência é refutada por overrides explícitos, troca de região, margens
terminais, estilos e regressões de Raw/Place/Align. before-edit.json.gz no
diretório block-par-spacing conserva os owners antes da edição.

### Classificação e limites

Correção interna ADR-0127, L0 primeiro, RED→GREEN e resselo restrito; sem
API pública, fase, default novo ou compatibilidade alterada. A amostra
scope19 refuta usar consenso de tamanhos dos runs como escopo de parágrafo:
um wrapper18 e dois wrappers18 adjacentes têm margens distintas no pin.
O escopo lexical continua pendente, assim como a cauda textual após Block1fr,
valores negativos rejeitados pelo constructor e colapso entre margens de
forças diferentes. Não marcar esses casos como corrigidos por esta fórmula.

### Responsabilidade exclusiva

Na reserva de margens de Block e seu vizinho, aplicar o núcleo pinado no
escopo exterior vigente. Não alterar pesos, guards, contribuição de prefixo,
cauda ou força do colapso como parte desta resolução.


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

Medir uma cauda textual certificada como segmento completo, incluindo
Styled/Strong/Emph/Linebreak/Source/Located e wrap, pelo helper isolado de
paragraph. Reservar sua altura causal uma vez, sem descontar vertical_metrics
como se fosse uma folha. Corpos opacos mantêm o percurso anterior.
Quando o prefixo legado recusar um segmento textual integral certificado,
reservar altura causal + margem colapsada com above explícito/automático do
primeiro Block. Em cursor de baseline já preparado, descontar seu top-edge
uma única vez. Entregar um marcador privado de prefixo geométrico ao runtime;
nessa rota não aplicar o ajuste top nem os hooks da fórmula legada.
Não emitir no emissor, reavaliar efeitos ou converter desconhecido em zero.


### Refutador: whitespace antes do primeiro bloco

implementation/scope-tail-first.log registra P1381 e escopo GREEN, mas TAIL
fica5pt acima com spacing5 e a cauda multiline12pt acima no default. O novo
fallback aceitou o prefixo só de whitespace (altura causal zero) e reservou
uma margem inicial inexistente. Prefixo geométrico exige altura causal
positiva; prefixo vazio/transparente não consome margem. Não subtrair esse
erro da cauda nem alterar seus esperados. A medição de cauda vazia continua
zero, distinta da criação de uma fronteira de prefixo.


### Refutadores de extensão e primeira região

scope-tail-after-first/results.json.gz confirma26/28 documentos. Os dois
restantes são caudas com cinco linhas que excedem a primeira região; reservar
altura integral deixa o início32/17pt acima. O primeiro fragmento deve ser
medido sob a capacidade disponível, usando a mesma admissão de viúvas/órfãs,
e somente sua altura causal entra na reserva anterior ao1fr. Próximas regiões
preservam sua capacidade normal; não somar margem terminal de parágrafo.

scope-tail-refutators/results.json.gz mede os três controles antigos que
exigiam caudas desalinhadas. Três fragmentos numa linha agora coincidem
integralmente. Multiword21 e par40/8 com fill ainda divergem em entrelinha:
a medição isolada reproduz essa geometria defeituosa, não a certifica.
O novo fallback de prefixo geométrico exige uma única baseline textual;
prefixos multiline conservam os certificados escalares anteriores ou a rota
opaca. Caudas multiline continuam sob medição integral/primeiro fragmento.
Não alterar as asserções opacas desses dois prefixos para declará-los verdes.
O aninhamento parcial OUTER18 + Styled8 contendo Block também continua
incorreto no pin; um segmento local que entrou com linha exterior aberta
não prova o escopo do parágrafo inteiro e não recebe novo fechamento.


### Linha já fechada conserva sua fronteira

scope-tail-green-final.log refuta que somente ignorar TextSeparator resolve
P1383: One ainda37.45 em vez43.5. O prefixo pode fechar sua última linha
antes de chegar ao Block. close_segment recusava current_line vazio e
perdia a margem, enquanto sub_frame devolvia o cursor da próxima baseline
como altura. Prefixo geométrico certificado usa o fundo textual lógico dos
items, não a baseline preparada. Para linha já fechada, com testemunha
causal válida, segmento completo e cadeia ainda ausente, registrar a margem
fraca compartilhada sobre esse fim, sem emitir outra linha nem repetir flush.
Origem inicial, testemunha inválida e cadeia existente não recebem essa rota.
O ajuste de separadores continua correto por sua neutralidade, mas não era
a causa suficiente deste RED. Cauda multiline mantém sua medição integral.


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

Após recusa legacy e single-line, tentar medição multiline certificada de paragraph. Reservar seu fundo causal + gap - top já preparado, uma vez; retornar marcador multiline privado. Não somar folhas desse prefixo nem aplicar hooks legacy. Se prefixo mais cauda não couber na região, recusar nova rota em vez de emitir parcialmente.
