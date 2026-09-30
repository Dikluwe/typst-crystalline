# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/pipeline/context_stabilization.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `d07057f9499436bab385e084329198943a6b96e6e93382d9cb07fbcaba04d910`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — estabilização contextual seletiva
Hash do Código: 482e0e14

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/context_stabilization.rs`

## Medição anterior à individualização

Baseline histórico,
registra HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` e working tree
não commitado com diff integral. `pipeline.rs:244-281` abandona a expansão
no primeiro Err; `:721-760` reexecuta contextos e termina por páginas iguais.
`eval/mod.rs:5786-5915` já expõe validação e diagnóstico de leituras, sem
coordenação das tentativas. Não existe implementação seletiva na pipeline.

## Ownership e integração

Este módulo descendente de pipeline possui exclusivamente a sessão privada
por compilação: descoberta, contribuições por produtor, tentativas e sinks.
Acessa helpers privados do pai por descendência; não expõe API fora de L3.
A pipeline conserva fachadas, exportação, composição de recursos e tratamento
legado quando não há seleção. O módulo recebe World/Source e a topologia
original; chama os helpers existentes de introspecção/layout/math/numbering
e a validação L1. Não duplica algoritmo de matching, counter, state ou
comparação de Value; não importa L2 nem altera entidades públicas.

Esta individualização transfere as cláusulas de coordenação abaixo do owner
monolítico para este consumer; não compartilha um Prompt entre arquivos.
As cláusulas conservam a intenção aprovada no P1339. Não muda assinaturas,
modo por defeito, fases ou políticas para opacidade. Mudança dessas dimensões
exige nova decisão. Sem seleção efetiva, não introduzir ciclo novo no legado.

## Estabilização contextual seletiva

### Medição anterior à decisão

No HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, `pipeline.rs:244-281`
avalia blocos com o mesmo snapshot e só incorpora seus resultados depois que
todos passam. O Err aborta antes de `:309-310` reintrospectar. A medição
histórica registra falha cristalina em assert dependente de update contextual, também
com Set sem callback; vanilla passa. final anterior ao produtor também passa
no vanilla, refutando resolver tudo por uma única passagem em ordem documental.
O controle string é diagnóstico de causa, não autorização de reparar legado.

`pipeline.rs:721-760` já reexpande entre layouts, mas encerra por quantidade
de páginas; este owner P1159 exige também estabilidade de snapshots/conteúdo.
A medição histórica demonstra que hash_content e
Arc de Func não certificam captura estável. A medição no vanilla cobre NaN
estável, produtores aninhados e closure recriada; oscilação e
crescimento devolvem saída acompanhada de warning após cinco tentativas.
Fonte `typst/src/lib.rs:133-180` e
`typst-library/src/introspection/convergence.rs:16,56-64,71-116`: a condição
de encerramento e o diagnóstico são distintos de sucesso de convergência.
O commit, o intervalo UTC e as saídas relevantes ficaram registrados na medição.

### Decisão de fase autorizada e limites

O dono autorizou incluir a estabilização dos contextos dependentes dos novos
contadores filtrados, conforme a decisão registrada.
Permanecem as fachadas públicas de expansão e compilação. A expansão original
continua entre introspecção e layout; não mover show rules para outra fase,
não usar a árvore pós-show como nova referência global nem corrigir P1037
incidentalmente. HTML target continua separado; features não mudam de target.

L3 coordena tentativas privadas; L1 decide quais leituras ocorreram e se seus
inputs continuam válidos, pelo contrato aprovado `compiler/eval.md`. Não
deduzir dependência de variável chamada counter, AST, substring de erro ou
simples presença de Selector no documento. Não duplicar em L3 resolução de
selectors, folds, igualdade de valores ou execução semântica de CounterUpdate.

Cada bloco tem seu ctx, resultado, erro e sink de tentativa. Demanda efetiva
Element pode tornar seu erro provisório; erro sem demanda alcançada conserva
o tratamento legado, inclusive se o código contém uma consulta não alcançada.
Outro bloco filtrado não torna esse erro descartável. Blocos não selecionados
mantêm sua contribuição/erro da passagem ordinária durante as tentativas
adicionais; não reparar sua leitura legada por reexecutá-los com snapshot novo.
Em bloco selecionado que também lê operações legadas, reavaliar seus inputs
com os mesmos helpers legados não autoriza mudar a semântica desses helpers.

Resultados válidos atuais formam a árvore provisória com marcadores estáveis;
reintrospecção incorpora seus eventos e fornece o snapshot candidato. Revalidar
as entradas realmente usadas pelos blocos selecionados; repetir somente os
invalidados. final pode depender de produtor posterior. Um bloco que falha
não conserva seu sucesso antigo como contribuição atual. Nunca acumular
updates/marcadores entre tentativas; a árvore de origem e a relação pai/filho
dos blocos continuam disponíveis para reconstrução. Novos blocos produzidos
devem ser descobertos pela travessia real de Content, não por scan de fonte;
identidade ambígua ou colisão invalida a tentativa, não remove conteúdo.

Não comparar Debug/hash de closures nem resolver callbacks nunca demandadas
para fabricar fingerprint. Quando todas as entradas registradas permanecem
válidas, reutilizar o resultado original do bloco, preservando suas capturas.
Completude do registro é precondição verificável: flag Element isolada, igual
valor final ou páginas iguais não certificam o documento.

Conservar junto da tentativa a identidade causal de seu produtor, captura,
Location, chain de entrada, target/features e recursos de Engine. Os métodos
de validação de eval recebem apenas um snapshot candidato: não podem provar
que esses outros inputs foram conservados pela pipeline. Reutilização é
permitida somente quando eles continuam os da mesma tentativa por construção.
Substituir o produtor/closure/chain invalida também seus registros descendentes;
não comparar Debug/hash ou apenas id numérico do ContextBlock para ignorar
essa substituição. O Engine de replay usa os recursos originais e os estilos
capturados por requisição, não a chain final deixada pelo corpo. Isto não
autoriza corrigir a propagação legada de target/features fora do recorte.

### Erros, warnings e término

Erros de tentativas invalidadas não são erros finais, mas tampouco são sucesso.
Erro persistente em tentativa validada é devolvido com spans/traces originais;
o controle assert provisório seguido de panic deve alcançar o panic final.
Sinks são locais às tentativas: não publicar warnings de validações descartadas
nem duplicar os da contribuição efetivamente conservada. A ordem observável de
diagnósticos exige oráculo próprio antes de confirmação, sem ordenação arbitrária pelo
HashMap ou reclassificação textual de erros.

O ciclo adicional integra a invalidação de pages/positions no ciclo paginado
existente. Nunca exportar uma tentativa com erro pendente como sucesso nem
declarar convergência por limite atingido. Para casos de não convergência
linguística sem erro final, preservar a política vanilla medida de saída com
warnings e história das leituras após cinco tentativas. Isto sucede a hipótese
exploratória de erro universal no teto: não criar tal default silenciosamente.
Limite interno/observabilidade insuficiente não é o mesmo que a não convergência
medida da linguagem e não pode receber esse warning para disfarçar incapacidade.

O contrato de tentativas deve fixar a correspondência de seus snapshots e a
última saída observável; os controles oscilante/crescente não autorizam copiar
um número de passagens sem reproduzir seus valores e diagnósticos. Não reiniciar
orçamento a cada relayout e assim mascarar não convergência. O ciclo P1291 de
math permanece separado e só fornece documentos Complete ao caminho paginado.

### Limite do contrato

Registro e replay pertencem aos owners das leituras; este módulo coordena as
tentativas e não autoriza fallback permissivo quando falta observabilidade.
Assinaturas públicas existentes permanecem nos seus owners.

### P1339 — seed seletivo e correspondência final de tentativas

Medição anterior à decisão: vanilla ratificado `typst/src/lib.rs:140-143`
começa com EmptyIntrospector, `:156-184` valida o documento atual contra o
snapshot que ele produziu, e `introspection/convergence.rs:267-278` distingue
runs da projeção final. O baseline cristalino `pipeline.rs:644-654` usa
introspecção estática já populada. A medição histórica
e o registro de boundaries explicitam essa diferença: os controles oscilante
e crescente observam o snapshot lido pela última tentativa, não o produzido.

Executar a passagem ordinária de descoberta e preservar a contribuição/erro
dos blocos que não alcançaram demanda Element. Para a geração que a alcançou,
descartar a realização/sink de descoberta e iniciar suas tentativas com
vistas observacionais vazias I0, conservando a Location da topologia de origem.
Essa descoberta não é uma das cinco tentativas de estabilização. A seleção
fica associada à geração; um ramo posterior sem leitura não apaga a seleção
anterior. Não executar callback de chave jamais demandada para selecionar.

Esta escolha preserva a fronteira legada já estabelecida: query/state ou
erro anterior que impeça alcançar Element na passagem ordinária não ganha
nova avaliação por análise de AST ou por imaginar outro seed. O P1339 não
afirma equivalência global para programas mistos que dependam dessa dívida.
Contratos devem manter esse caso como controle explícito, não exigir reparo
legado nem chamá-lo de paridade nova. A autorização até o fim do passo em
A decisão registrada cobre essa integração no
escopo seletivo, sem necessidade de outro pedido de aprovação.

Nomear A_k a tentativa que lê I_(k-1), produz árvore C_k/documento D_k e
snapshot completo I_k. Reintrospecção, positions e PageStore do documento
compõem I_k antes de revalidar os registros retidos. Resultados reutilizados
conservam sua execução real/transcript/sink; não contar execução fictícia.
Produtores não selecionados entram com contribuição ordinária congelada;
seus updates não aparecem artificialmente em I0.

Se uma leitura selecionada invalida, A_(k+1) reavalia esse bloco e os
descendentes cujo produtor mudou, usando I_k. Expandir novos blocos reais
recursivamente contra o snapshot da tentativa corrente; sua descoberta não
reinicia o orçamento. Reconstruir desde origem/contribuições atuais sem
acumular markers. Corpo com erro conserva apenas marker, não sucesso antigo.

Com todos os registros validados, devolver erro pendente original se houver,
senão D_k. No teto, sem erro final, devolver D5, calculado lendo I4. Não
executar A6, trocar a saída por replay em I5 ou devolver D4. A história de
diagnóstico é [I0,I1,I2,I3,I4,I5]; somente as requisições efetivas da tentativa
final geram projeções históricas. Resumo de não convergência exige detalhes
comprovados por essas consultas; Unproven não vira diagnóstico linguístico.
Orçamento é compartilhado com invalidações paginadas, não cinco rodadas
internas adicionais por relayout; o ciclo math anterior continua separado.

As fachadas públicas conservam assinatura e seus resultados próprios de
expansão; a compilação paginada coordena os estados privados de tentativa e
layout. Nenhuma fachada isolada é usada como prova de estabilidade de páginas
que ainda não foram produzidas. Sinks de execuções substituídas/validação são
descartados; os das contribuições conservadas são publicados uma vez. Erro
independente sem demanda alcançada permanece legado; erro final impede
exportação e não é removido pelos warnings de não convergência.

## P1340 — política terminal de comprovação insuficiente (APROVADA ADR-0127)

### Medição anterior à proposta

`eval/mod.rs:5797-5804` reduz Different e Unproven ao mesmo booleano;
`:5853-5881` não usa detalhes diagnósticos como detector de opacidade. A
medição histórica confirma a lacuna: as cláusulas precedentes impedem certificar incapacidade
como convergência, mas não escolhem retorno para corpo Ok sem estabilidade
comprovável. A auditoria adversarial permite investigar autorreflexividade
com as APIs existentes; não demonstrou necessidade de assinatura nova.

### Decisão vigente

Se, esgotadas as tentativas permitidas, os corpos atuais não têm erro próprio
mas permanece impossível comprovar a estabilidade exigida, devolver pelo
canal `Err(Vec<SourceDiagnostic>)` existente um diagnóstico próprio:

- severidade Error;
- mensagem exata `contextual stability could not be verified`;
- âncora `source.root().span()` da compilação, sem inventar origem de uma
  leitura específica nem acessar metadata privada de Func;
- sem trace fabricado e sem hints de não convergência vanilla;
- nenhuma exportação do documento provisório.

Não antecipar esse erro por um `Ok(false)` isolado, não inferir opacidade
pela ausência de detalhes e não confundir Different com Unproven. Se a
topologia/identidade ambígua impedir construir qualquer tentativa verificável,
o mesmo erro encerra a coordenação no ponto em que essa impossibilidade é
comprovada, sem descartar silenciosamente uma ocorrência.

Erros originais pendentes têm precedência e conservam mensagens/spans/traces;
não são substituídos pelo diagnóstico acima. Warnings globais anteriores são
preservados; sinks de validação e de tentativas substituídas são descartados;
contribuições finais retidas publicam seus warnings uma vez, mesmo no retorno
Err, sem adicionar warning de não convergência para a incapacidade.

Esta política aplica-se somente ao ramo de comprovação insuficiente. Mudança comprovada
no fragmento fechado conserva a política linguística anterior D5/I4 e seus
diagnósticos. Não acrescenta API, enum público, flag ou correção de semântica
alheia. É comportamento terminal do produto, não paridade inferida.

## Fronteira de instrumentação

Permanecem integrais a descoberta, seleção causal, contribuições, retenção e
descarte, invalidação, replay, comparação de leituras, orçamento de cinco
tentativas, decisão terminal, sinks, layout e numeração de páginas. A remoção
não muda a fase do pipeline, não executa callback adicional e não fabrica
evento substituto.

Não existe neste owner obrigação de módulo de observação, ledger, DTO ou hooks
condicionados por `p1339_observation`. Instrumentação futura requer obrigação
L0 própria e `cfg` formalizado em Cargo/check-cfg.

## P1421 — preparação contextual HTML sem paginação (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `ef33ba5204870646a7c4f3046d8b210d6cd8e8cf`, `Session::execute`
fixa `ctx.target = EvalTarget::Paged`; `prepare` pertence ao caminho paginado e
pode coordenar layout/fixpoint. Sua substituição conserva cada marker
`ContextBlock` antes do resultado para manter Location no layout. Nenhuma dessas
três propriedades pode ser reutilizada sem distinção no target HTML.

### Decisão proposta

A sessão privada passa a conservar o `EvalTarget` causal recebido pelo caller.
Todos os callers paginados fornecem `Paged` explicitamente e preservam o
comportamento atual. Uma entry point privada, usada somente pela fachada HTML,
fornece `Html`, as features efetivas e `full_error` causal, executa a passagem
contextual sem criar documento paginado e devolve conteúdo mais warnings.

Na preparação HTML, target/features não são defaults reconstruídos: a closure
observa `target() == "html"` e conserva acesso ao módulo HTML habilitado. Depois
da substituição, remover somente os markers `ContextBlock` mantidos para o
layout paginado; seus resultados permanecem na posição causal. A remoção é uma
vista terminal específica para serialização, não mudança na entidade, no walk
ou na política paginada de Locations.

Se a sessão detectar dependência cuja estabilidade exige páginas/positions,
identidade ambígua, erro de closure ou marker residual depois da preparação,
devolve Err e impede exportação. Não chamar o fixpoint/layout paginado para
fabricar observabilidade HTML, não converter incapacidade em `Content::Empty` e
não descartar warnings da execução efetivamente publicada.

Contextos aninhados não são gate positivo de P1421, mas também não podem
alcançar o serializer como sucesso. A mudança é uma fase nova do pipeline HTML
e aguarda confirmação humana explícita pelo ADR-0127. Nenhuma assinatura
pública muda; a entry point permanece privada ao módulo pai.

## P1485.2 — estabilização contextual do conjunto bundle (ratificado; ADR-0127)

### Medição anterior à decisão

Na working tree P1485 não commitada sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
`Session::stabilize` constrói `self.content()` e reintrospecta
apenas esse body (`03_infra/src/pipeline/context_stabilization.rs:408-423`).
O caller bundle invoca `prepare` ou `prepare_html` por documento
(`03_infra/src/pipeline.rs:900-955`). A primeira introspecção
compartilhada já permite `query(heading).len() == 3` em dois HTMLs,
mas uma contribuição contextual selecionada pode invalidar o
snapshot de outro documento. Reintrospectar só o body selecionado
não é evidência de estabilidade do bundle. `prepare_html`
(`:586-625`) conserva target HTML sem layout paginado; misturar
os targets numa Session Paged também seria deriva.
Classificação ADR-0107: valores de query/counter/state e a
identidade do documento observado são semântica; quantidade de
tentativas e representação das Sessions são mecânica. A saída
vanilla não prova sua intenção de usar esta mesma coordenação.
Inferência: revalidar todos os produtores contra um único candidato
global evita snapshot misto. Refutação: programa com produtor
posterior ou alvo Html/Paged cruzado ainda divergir após a
reintrospecção global, ou mudar a política de warnings medida.


### Contrato privado ratificado

Introduzir coordenação bundle **privada** neste owner, reutilizando
as regras de descoberta, seleção, replay, invalidação, cinco
tentativas, erros e warnings já aprovadas. Cada body conserva sua
identidade, chain, target efetivo (Paged ou Html), features e
recursos; HTML não herda layout paginado. Após qualquer realização
selecionada, reconstruir o vetor completo de bodies atuais,
reintrospectar o conjunto com um único snapshot causal e recalcular
os inícios de Location pelo contrato de
`00_nucleo/prompts/compiler/introspect.md`. Validar todas as
leituras selecionadas contra esse mesmo snapshot antes de exportar
qualquer documento. Nunca aceitar estados finais provenientes de
gerações distintas nem fazer fallback para introspecção isolada
quando a coordenação global falhar.

O ramo paginado entrega o seed correto ao layout; o ramo HTML
preserva o target semântico e remove apenas seus marcadores de
contexto executados. O início lógico de página é reduzido por body
a partir de suas ações de `CounterKey::Page`; essa redução local
produz somente um escalar de paginação, não é candidata de consultas
nem fallback para validação — o snapshot de leitura permanece global.
Erros pendentes e warnings das tentativas
descartadas obedecem à política atual. Não ampliar orçamento
por documento, não criar uma fase eval↔layout nova e não alterar
as fachadas públicas existentes. Se a topologia de produtores
impedir a validação global, devolver diagnóstico de incapacidade,
não sucesso parcial.

Aceitação bilateral: query/counter/state em ambos os sentidos entre
dois documentos, produtor posterior, heading rotulado, mistura
Paged+Html, contexto aninhado, convergência e erro persistente;
cada arquivo publicado corresponde à mesma geração validada.
A entrypoint L3 permanece privada; os contratos públicos L1 dos
quais depende tiveram o gate ADR-0127 confirmado pelo dono em
2026-09-24.

## P1485.3 — snapshot runtime com fronteiras por body (ratificado; ADR-0127)

### Medição anterior à decisão

`prepare_bundle` usa seeds puros por body, mas entrega Sequence ao
introspector runtime (`03_infra/src/pipeline/context_stabilization.rs:742-745`);
`Session::stabilize` também reintrospecta `self.content()` como Sequence
(`:404-433`). A sonda RED e o vanilla pinado no diagnóstico de integração
P1485 distinguem Page local (1 no HTML posterior) da representação global.
Sonda RED adicional no mesmo working tree:
`p1485_bundle_html_respeita_update_page_proprio` falha com
`HTML export does not yet support this content` porque
`Content::CounterUpdate` chega ao serializer
(`03_infra/src/export/html.rs:518-601`).
Inferência: preservar fronteiras nos dois pontos de reintrospecção permite
o reset local sem quebrar o candidato global. Refutação: produtor posterior
ou aninhado ainda ler página de outro documento, ou snapshot puro/runtime
discordar. Sequência de chamadas é mecânica; valor observado é linguagem
(ADR-0107). Não copiar eventual bug vanilla em casos não medidos.

### Decisão privada condicionada à API L1

Depois da confirmação de `compiler/introspect.md` P1485.3, passar o vetor
completo de bodies à nova variante runtime em cada geração e no prepare
inicial, mantendo target e identidade. A fronteira Page integra o snapshot
global; não construir snapshot por body nem substituir leituras por valor
HTML constante. A redução local de página P1485.2 continua apenas a
inicializar a paginação física Paged. Update Page próprio de HTML continua
legível pelo histórico global na sua faixa. Só depois da validação
global, a projeção terminal HTML remove os marcadores
`Content::CounterUpdate`, que já agiram semanticamente e não têm
representação visual; nunca os remover do candidato de leitura.
Validar produtores contra um candidato, manter cinco tentativas,
erros, warnings e publicação atômica. Não há nova fase nem API L3
pública. O gate da entrypoint L1 foi confirmado pelo dono em
2026-09-24.

## P1486 — PageStore final no bundle sem produtores contextuais

### Medição anterior à decisão

No teste RED `p1486_bundle_preserva_offset_apos_html_svg_png`, um bundle
PDF/HTML/PDF sem produtor contextual chega ao primeiro PDF com duas páginas
renderizadas e `PageStore` de comprimento zero: diagnóstico
`realized page numbers do not match final document (offset 0, rendered 2, store 0)`.
`prepare_bundle` em `03_infra/src/pipeline/context_stabilization.rs:729-771`
invoca `bundle_page_views` apenas via `Session::stabilize`; sem produtores
selecionados, devolve o introspector runtime sem realizar a vista paginada.
O `PageStore` passa a ser necessário também ao exportador PDF para transportar
o número lógico observável; a existência de produtores contextuais é mecânica
interna, não deve determinar o rótulo da linguagem. Inferência: realizar essa
vista uma vez no ramo sem seleção fornece a mesma store que a geração
estabilizada. Refutação: divergência de posições, numeração ou contagem de
páginas num bundle sem contexto após a realização terminal.

### Decisão privada

Depois de validar a introspecção runtime contra a pura e antes da projeção
HTML terminal, executar `bundle_page_views` uma vez para o candidato final
quando não houve estabilização selecionada. Usar o mesmo vetor de bodies,
targets HTML, world, source e input vazio que a primeira tentativa usaria;
propagar erros e warnings. Quando houve estabilização, conservar a store já
produzida pelo candidato validado, sem segunda realização. HTML não consome
página. Não introduzir fase, contrato público nem fallback de índice físico.

## P1493 — fixpoint seletivo de posição em ilha HTML (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

`prepare_html` executa cada produtor contra introspector sem posições e
remove seu marker. Uma leitura de posição não seleciona tentativa adicional;
asserções simples, duas Locations e nesting terminam com `none`. O marker
preservado por `substitute_context_blocks` já permite medir o ponto mesmo
quando o resultado provisório do produtor é erro.
Dois RED adicionais no working tree de 2026-09-24 mostraram que `discover()`
executava só roots: um leitor existente apenas num ContextBlock produzido por
outro ficava sem execução e terminava como marker não resolvido. Uma sonda com
Location publicada por metadata mostrou também a transição de store não vazio
para vazio quando o descendente desaparece; filtrar o latch apenas por nós ativos
aceitava estado obsoleto. HEAD da medição: `b5abe9b288ce168c494a4c5172830e325b9aedae`,
working tree não commitado enumerado no diagnóstico P1493.


### Contrato ratificado

A sessão distingue seleção por counter filtrado da seleção por
`LocationPosition` observada. No HTML standalone, somente a segunda autoriza
o novo ciclo; leituras paginadas já proibidas e outras dívidas P1421 continuam
fail-closed. Erro da descoberta selecionada é provisório e sua sink é
descartada, como nas tentativas existentes.
A descoberta HTML realiza recursivamente produtores descendentes criados pelo
resultado provisório, contra o mesmo snapshot causal, antes de decidir se existe
leitor de posição. Ao iniciar o ciclo, descartar a execução de qualquer produtor
selecionado, inclusive descendente; produtores ancestrais não selecionados podem
reter sua contribuição.

Depois da primeira leitura observada, a fase HTML fica associada à sessão. Nós
inativos não apagam essa seleção e um input com store não vazio obriga produzir
e comparar o store corrente, inclusive vazio. A transição nonempty -> empty
exige uma geração adicional que confirme empty -> empty antes de convergir.


A tentativa HTML A_k lê o snapshot anterior, reconstrói conteúdo com
marker+resultado, reintrospecta a topologia, realiza somente as ilhas
`html.frame`, injeta o store `HtmlPosition` da geração e revalida todas as
leituras do produtor. O seed é o introspector HTML causal original com store
HTML vazio, não `TagIntrospector::empty()`, para não alterar query/state já
medidos. Posição ou resultado diferente invalida o produtor e descendentes.
Estabilidade exige igualdade completa do store HTML e das observações; o teto,
erros, warnings, retenção e descarte seguem o orçamento vigente de cinco
tentativas.

No bundle, cada body HTML contribui suas ilhas ao candidato global da mesma
geração; bodies paginados continuam compondo `positions` e PageStore
separados. Nunca misturar os dois stores nem deslocar coordenadas HTML por
offset de páginas. Após convergência, remover somente markers HTML executados.
Colisão de Location/id, cobertura incompleta ou store de geração mista produz
o diagnóstico terminal de incapacidade existente. A mudança de fase foi
confirmada pelo dono em 2026-09-24.

## P1494 — preservação causal do espaço na estabilização (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, o consumer
03_infra/src/pipeline/context_stabilization.rs não tem diff.
prepare_bundle compõe os bodies originais em uma Sequence, atribui
EvalTarget por body e descobre os produtores antes das tentativas
(:748-839). Depois da estabilização, reintrospecta raw_bodies e remove
markers HTML executados apenas no fim (:835-887).
strip_executed_html_markers reduz uma Sequence só de Content::Space a
Empty (:1058-1077). No P1489-S-001, a origem do newline já se perdeu
em eval antes de esta sessão ser criada; isso não prova que um strip
tardio possa decidir o descarte CJK. Inferência: a sessão deve manter
o carrier em cada tentativa até a normalização L1 causal; refutação:
uma tentativa que mude a origem sem alterar a observação validada.

### Decisão ratificada

Manter SpaceFromNewline no conteúdo de origem, nas contribuições dos
produtores e nos snapshots das tentativas enquanto o owner L1 não
tiver normalizado o grupo textual. Não reclassificar pela grafia,
converter em Space na montagem do bundle nem fazer colapso CJK no
fixpoint. A observação causal pré-normalização deve distinguir as duas
origens, conforme o contrato L1, para não certificar um replay errado.
No strip terminal de markers, uma Sequence composta só de Space e/ou
SpaceFromNewline continua estruturalmente vazia; fora desse caso,
preservar os nós na mesma ordem. Os targets Html/Paged mantêm os
limites de P1493 e P1485, mas usam a mesma normalização linguística L1.
Erros, warnings, PageStore, HtmlPosition e orçamento de tentativas
não mudam.

Testes incluem produtor contextual que devolve newline CJK e espaço
explícito, controle de convergência e bundle misto sem perda de target.
O gate ADR-0127 da variante pública e da fase P1494 foi confirmado
pelo dono em 2026-09-24; esta seção secundária está autorizada.

## Integração após individualização dos targets

Os helpers de introspecção, layout e numbering continuam disponíveis pelo
pai via reexports privados de `00_nucleo/prompts/infra/pipeline/paged.md`;
as posições de ilhas HTML vêm de `00_nucleo/prompts/infra/pipeline/html.md`,
e a introspecção global bundle vem de `00_nucleo/prompts/infra/pipeline/bundle.md`.
A sessão conserva ownership, contribuições e algoritmo; a mudança física
dos helpers não altera fases, orçamento nem política terminal.
