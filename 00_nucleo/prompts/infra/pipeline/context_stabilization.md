# Prompt L0 — estabilização contextual seletiva
Hash do Código: 49eccf73

**Camada:** L3
**Consumer único:** `03_infra/src/pipeline/context_stabilization.rs`

## Medição anterior à individualização

Baseline `diagnosticos/p1340-baseline.json`, SHA-256
`0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a`,
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
todos passam. O Err aborta antes de `:309-310` reintrospectar. O recibo
`diagnosticos/p1339-context-dependency-probe-runs.json`, SHA-256
`4da38b499916ed3d5946bb16c904f2eaa268a4ec2f788cc0149927f0d83260d1`,
registra falha cristalina em assert dependente de update contextual, também
com Set sem callback; vanilla passa. final anterior ao produtor também passa
no vanilla, refutando resolver tudo por uma única passagem em ordem documental.
O controle string é diagnóstico de causa, não autorização de reparar legado.

`pipeline.rs:721-760` já reexpande entre layouts, mas encerra por quantidade
de páginas; este owner P1159 exige também estabilidade de snapshots/conteúdo.
`diagnosticos/p1339-stabilization-design.md` demonstra que hash_content e
Arc de Func não certificam captura estável. O recibo vanilla
`diagnosticos/p1339-stabilization-boundaries-runs.json`, SHA-256
`4d82648d895ec5dba23a9775c2bf09dfae84a175603f3ce6d742c8c7f2a09241`,
mede NaN estável, produtores aninhados e closure recriada; oscilação e
crescimento devolvem saída acompanhada de warning após cinco tentativas.
Fonte `typst/src/lib.rs:133-180` e
`typst-library/src/introspection/convergence.rs:16,56-64,71-116`: a condição
de encerramento e o diagnóstico são distintos de sucesso de convergência.
Os recibos conservam árvore, UTC, binários e saídas integrais.

### Decisão de fase autorizada e limites

O dono autorizou incluir a estabilização dos contextos dependentes dos novos
contadores filtrados, conforme `diagnosticos/p1339-stabilization-approval.json`.
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
diagnósticos exige oráculo próprio antes de selo, sem ordenação arbitrária pelo
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
introspecção estática já populada. `diagnosticos/p1339-observation-design-resolution.md`
e o recibo de boundaries explicitam essa diferença: os controles oscilante
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
`diagnosticos/p1339-completion-authorization.md` cobre essa integração no
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
`:5853-5881` não usa detalhes diagnósticos como detector de opacidade.
`diagnosticos/p1340-verifier-feasibility-decision-r1.md`, SHA-256
`91fc8f38dce007297a3d6d99a885fbda837000424d5d24adfd549e45d959d4ce`,
confirma a lacuna: as cláusulas precedentes impedem certificar incapacidade
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
