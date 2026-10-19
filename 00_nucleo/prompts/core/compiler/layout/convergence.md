# L0 — convergência das tentativas de layout
Hash do Código: b466a9dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:cec5b93a8888658ac90461deea96799dfd56393be3d63c874e6cf93f4d57032c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/convergence.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

`01_core/src/compiler/layout/mod.rs:1859-2035` prepara cache bibliográfico e callbacks, recria Layouter, injeta páginas conhecidas, compara os mapas e devolve Complete/Pending. O L0 vigente do hub já fixa cinco tentativas e a necessidade de convergir mesmo sem outline. Medição estrutural no commit `320c2149b`; não mede custo nem intenção vanilla.

## Contrato

Possuir a coordenação privada da tentativa: a mesma introspecção tracked, seed do Locator, cache CSL e diagnósticos iniciais em cada candidato; conhecidos de label/figura/tabela/page store transportados na mesma ordem; máximo de cinco tentativas. Aceitar somente mapas estáveis com transcript math e par.line consumidos. Uma tentativa rejeitada descarta seu transcript. Exaustão devolve Pending com requests vazios. A fachada pública mantém assinaturas; o dispatcher exaustivo Content e estado do Layouter permanecem no hub. Descoberta do primeiro style bibliográfico permanece auxiliar local à coordenação, enquanto interpretação CSL fica em `layout/bib_csl.md`.

## Aceitação

Páginas, posições, conteúdo, diagnósticos e requests de callback iguais para convergência, não convergência, cache inválido e seed injetado. Sem alteração de fase ou default. Build, testes focais e lint válidos.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C010`: `01_core/src/compiler/layout/convergence.rs:9`, `run`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C010` — `run`: Exceção pontual justificada

Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed. Separar os dois protocolos e repetir a mesma seed é requisito L0.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C010: Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C010. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.

## A2-CSL — descoberta semântica do estilo bibliográfico (2026-10-18)

### Medição anterior à decisão

`lab/results/construction/diagnostics/bibliography-a2-2026-10-18/before.json.gz` registra
16 fontes/pares PDF, argv, texto/posições, UTC, HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree não commitado;
`before-sources.json.gz` contém fontes/L0, status e diff HEAD --stat completos.
Oracle pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, binário SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt SHA-256 57154005875bca986a1c4eb44038c8bd9ad2b3580e5534059574895c94e62f82.

MLA simples e IEEE simples conservam seu texto nos dois compiladores. Com
show bibliography.where():set text(fill:red), o Tekt troca (Silva) e a lista
MLA por [1] e fallback com seta; a regra bare agora admitida tem o mesmo
problema. Identidade, Block e show-set de hanging-indent também perdem CSL.
Bibliography substituída por REPLACEMENT ou [] mantém (Silva) no vanilla,
mas vira [1] no Tekt: só buscar a árvore visual não basta. O controle
Block que oculta a Bibliography rejeita @audit nos dois; não ressuscitar a
bibliografia de um filho não emitido. Multiple mostra um problema distinto:
o Tekt duplica a mesma lista MLA, enquanto o vanilla atribui as citações à
bibliografia pertinente. Esse controle permanece aberto, não é paridade por
compartilhar uma árvore de render.

convergence.rs:197–249 navega um catálogo parcial e omite Source/Located/
Realized/ShowRecipe; :57–91 só constrói cache se ele acha a declaração.
bibliography.rs:112–123 usa fallback quando esse cache inexiste. O índice
TagIntrospector já transporta Bibliography por ocorrência (kind_index/elements)
e o BibStore transporta o IndependentStyle resolvido por style_key.
No vanilla model/bibliography.rs:885–909 prepara bibliografias obtidas da
introspecção, com atribuição por ocorrência; :361–369 fixa show-set de recuo.
São dados de linguagem/efeitos. Inferência causal: a busca parcial perde a
seleção de CSL; testes por carrier, declaração ocultada, ordem e cache podem
refutá-la. A implementação não precisa copiar as estruturas upstream.

### Responsabilidade exclusiva

Dentro da preparação privada já existente, descobrir primeiro a Bibliography
registrada no índice semântico da tentativa, em ordem kind_index, lendo seu
snapshot em elements. Assim uma declaração realizada e depois substituída
continua a fornecer CSL às citações; não percorrer originais ocultos arbitrários
para fabricar uma ocorrência. Preservar style/locale e a tabela lateral
resolved_style de sua própria style_key. A primeira declaração com style None
continua sendo a primeira: não saltar para outra só porque tem style explícito.

Quando não há snapshot bibliográfico disponível no índice fornecido, conservar
compatibilidade da entrada de layout cru descobrindo o primeiro elemento
navegável por empréstimo com visit_content_static, cujo contrato pertence a
00_nucleo/prompts/core/entities/content/traversal.md. Source/Located/ShowRecipe
seguem body, Realized somente visual, e Table/Cell usam a vista ativa existente;
não executar recipes, Func, ContextBlock ou Dynamic. Podar Document/Asset no
hospedeiro e parar de descer após o primeiro achado. Não manter outro catálogo
parcial de containers, clonar Content para navegar ou mudar o visitante.

A closure observa somente ocorrências Bibliography já visitadas pelo dispatcher
exaustivo do domínio; o if-let é uma observação de evento, não um substituto
parcial da enumeração de filhos. A fronteira Document/Asset é explícita.
Não classificar Content como enum aberto/dinâmico nem suprimir V16/V19/V20.

Esta correção de paridade é interna/contínua ADR-0127. Mesma fase de cache antes
do layout, mesmas fachadas públicas e mesma política explícita de fallback
quando não há style; sem modo/flag, campo/trait, I/O ou mudança do pipeline.
Não alterar a interpretação CSL ou o estado do Layouter neste owner.

### Aceitação e divisão explícita

Antes do código, testes locais RED para carriers e containers antes omitidos,
prioridade do índice/ordem, declaração substituída/ocultada registrada,
conservação de style_key/resolved_style/locale e fallback cru sem navegar
Realized.original ou Document. Conferir ausência de efeito de receita.
GREEN bilateral de CSL sob show-set/identity/Block e de citações sob
substituição; IEEE continua IEEE e o parágrafo externo conserva estilo.
Revalidar suites/build, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa, diferencial SARIF sem novo erro/warning; remover o V16 desta busca
por delegação ao visitante exaustivo, sem converter a dívida em supressão.
12 GiB/High8 GiB/swap0/jobs1/debug0/incremental0 e comandos pesados sequenciais.
Sem commit/push/release.

A2-CSL é incompleto no escopo A2: A2-Indent deve completar hanging-indent
(default MLA e override lexical, geometria real e isolamento de OUTSIDE),
e A2-Multi deve completar atribuição/cache por Bibliography quando há várias.
Não declarar E05 nem toda bibliografia encerrados por esta busca; não aceitar
as coordenadas atuais só porque o formato MLA voltou. A correção não ratifica
Symbol, Styles público ou a proposta de ordem R3-T1 ainda pendente.

## A2-Structure — entradas CSL e colunas (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-structure-2026-10-18/before.json.gz registra
12 pares, fontes/argv/UTC/HEAD/status/diff HEAD --stat, textos/caixas/binários;
before-sources.json.gz conserva as fontes e L0s anteriores. HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada, Tekt SHA-256
c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899;
oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
IEEE12 põe A. em x25.124 contra29.924 e a segunda entrada em y89.936
contra112.232. IEEE18 usa x33.686 contra40.886. No CSL sem prefixo, a segunda
entrada começa em y184.424 contra191.024. As sondas com par.spacing0/2em
separam margens de espaçamento dentro das linhas. Há também diferença de
agrupamento de citações adjacentes (MLA Silva/Doe), fora da composição da lista;
manter o RED explícito, não chamar igualdade só da lista de paridade geral.

bib_csl.rs:342–355 concatena first_field, espaço literal, content e Linebreak,
perdendo a fronteira de cada entrada e o hanging_indent. convergence.rs:58–82
transporta só BibRenderCache público; bibliography.rs:107–151 emite um Block
único. O pin typst-layout/src/rules.rs:463–531 compõe duas colunas Auto com
gutter0.65em e row-gutter par.spacing, ou um Block por entrada sem prefixo;
o formato autoral do código Rust não é requisito. Observáveis são posição,
ordem, conteúdo/estilo e escopo. Hipótese: transportar fronteiras reais e
prefixos corrige a coluna e a separação sem reformatar; controles de fontes,
ordem, campos vazios, sem prefixo, zero/2em, show-grid e estilo podem refutá-la.

### Classe e escopo explícito

Correção interna/de paridade ADR-0127 em fluxo contínuo, dentro do layout,
sem API pública, tipo público novo, flag/default, compatibilidade ou fase nova.
L0 antes de testes/código. A2-Structure preserva hanging no discriminante do
plano, mas sua aplicação física, overrides por ocorrência e composição
simple/optimized/RTL continuam em A2-HangingTransport/A2-ParIndent. Não forçar
fallback greedy para contornar o suporte de avanço inicial ainda ausente da
preparação. Margens de título, CSL completos, atribuição A2-Multi e agrupamento
de citações adjacentes não ficam certificados por esta etapa.

### Responsabilidade exclusiva

Preparar o par cache/plano uma vez a partir da mesma primeira declaração efetiva, locale/resolved_style/ordem já decididos. Propagar diagnósticos como antes. Cada tentativa recebe o cache e um clone Arc do plano; descoberta/ordem/escopo A2-CSL não mudam. Não mudar fachada pública nem executar outro driver de bibliografia para criar o plano.

### Aceitação

RED funcional de coluna com gutter relativo e margem entre entradas antes de
implementar. Controles de cache público (Content anterior e assinaturas), CSL
sem prefixo, preservação de modo hanging/plain, vazio, ordenação/citações e
rollback. PDF bilateral com caixas e texto da lista, conservando divergências
de recuo e agrupamento de citações no relatório. Build/workspace/doctests,
Clippy e Rustdoc estritos, fmt, V1/V5/V15/V26 e reversa; SARIF sem erro/warning
novo. Resselo só após ownership/grafo válido. MemoryMax12GiB/High8GiB/swap0,
jobs1/debug0/incremental0, comandos pesados sequenciais; sem commit/push/release.
