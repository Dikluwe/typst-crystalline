# L0 — compiler/realize/bare_math
Hash do Código: e2cc18f8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff

**Layer:** L1.
**Exclusive consumer:** `01_core/src/compiler/realize/bare_math.rs`.
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
00_nucleo/diagnosticos/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
00_nucleo/diagnosticos/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Owner puro da normalização de inserção matemática. Descendente de compiler/realize,
sem imports reversos entities→compiler. Expor apenas helper interno ao compiler:

```rust
pub(in crate::compiler) fn prepare(
    content: &crate::entities::content::Content,
    styles: &crate::entities::style_chain::StyleChain,
) -> crate::entities::source_result::SourceResult<crate::entities::content::Content>;
```

Percorrer por fronteiras antes dos filhos usando o helper interno pré-ordem do
owner traversal. Equation existente é fronteira matemática: manter corpo sem
Equation adicional. Em Mathy nu suportado, envolver a raiz uma vez com
Content::equation(body_original, block_efetivo), sem normalizar seus descendentes.
block vem de equation.block booleano do escopo de inserção, default false;
numbering, number-align, supplement e alt conservam o transporte estilístico
existente, sem campos públicos novos. O helper não aloca Location nem chama Func;
identidade e aplicação de regra pertencem aos coordenadores atuais.

Usar classificação fechada por variante do enum e catálogo Mathy medido da fonte
pinada, não todo nome iniciado por Math. Frac/Matrix/Root/Lr/Mid/Attach/Primes/
Stretch/Op/Cases/Vec/Accent/Cancel/Underover e seus carriers concretos implementados
são estruturais. MathIdent/MathText isolados não equivalem a Mathy upstream;
MathSequence é fluxo a percorrer, não Equation sintética envolvendo tudo.
MathStyled é estilo transparente: texto isolado fica texto; descendente Mathy
fora de Equation vira Equation. ClassOverride/LimitsOverride representam os
wrappers Mathy da fonte quando efetivamente existentes no enum. Confirmar cada
entrada com file:line e teste pertinente antes da materialização; variante sem
correspondência não recebe aceitação por semelhança do nome.

Styled conserva o wrapper e desce com a chain sobreposta; ShowRecipe usa seu
snapshot lexical próprio e conserva regras/guards/file/textual/context_id_floor.
Realized conserva original intocado e processa somente o visual ainda não
preparado. Located preparado conserva sua Location e não é reinterpretado como
inserção nova. Labels ficam no elemento original; não se promovem à Equation
criada. Sequence, containers de conteúdo comum e contribuições contextual/show
visitam as inserções reais, preservando ordem e regiões matemáticas.

Não alterar constructors, repr ou Value cru. Não juntar duas raízes Mathy em
uma Equation por igualdade/adjacência e não envolver conteúdo normal por ter
font/math style. Idempotência: uma árvore preparada não ganha equações extras.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

### Limite de captura contextual

Capturar o snapshot contextual adicional somente quando existe regra de
Equation ativa por identidade da nativa, inclusive bases where/and/or. Classificar
selector estático no helper desta unidade, sem string ou constructor. O snapshot
nessa posição conserva todas as regras lexicais pertinentes; contextos sem regra
de equação mantêm o transporte existente. Não ampliar o reparo a contexto/show
de biblioteca inteira nem alterar terminação de outras famílias.

### Catálogo de fonte pinada conferido

mathy-catalog.json registra file:line de cada elemento Mathy no pin atual.
MathUnderover é extensão legada criada por native_underover (stdlib/structural/
math/underover.rs:19), sem elemento Mathy correspondente na fonte upstream.
A menção genérica inicial a Underover nesta proposta é supersedida: não preparar
esse carrier por suposição; conservar seu fallback nu existente e não certificar
paridade da extensão. MathDelimited corresponde ao carrier legado de Lr; os
wrappers de class/limits correspondem a ClassElem/LimitsElem/ScriptsElem.

## Prova fechada de ausência de Mathy — 2026-10-09

### Medição antes da decisão

`00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/phases-after-reuse.json.gz` mede o release SHA-256 `5b4b34340a5f637d9e99c0ad3ee9e209c14b22c18c8890b23d344ea0a2778ecf`, HEAD/working tree não commitado/status/diffstat/UTC/fixtures/CPU em três execuções por caso. Em 1.600 células, a avaliação de `local-set` é 150,42 ms contra 118,99 ms de `no-fill`; layout 90,54 contra 68,47 ms. `stacks-after-reuse.json.gz` amostra uma em cada 64 solicitações glibc: Content::styled e with_source_span surgem nas reconstruções por map_content, incluindo `bare_math::prepare`. `bare-math-red-gate.json.gz` registra 635.978 pedidos locais contra limite pré-código 600.000, falha RED, com binário/fonte/comando/UTC/estado. `bare_math.rs:45-84` sempre chama map_content_pre e reconstrói Styled/TableCell mesmo quando toda a subárvore é texto simples. A inferência de benefício será refutada por gate, tempo pareado ou saída diferente.

### Decisão interna

Antes de transformar, provar recursivamente ausência de Mathy apenas nas folhas Empty/Text/Space/SpaceFromNewline e nos carriers Source, Styled, Sequence, ShowRecipe.body, Realized.visual, TableCell.visual, Table resolvida cells+caption ou Table crua children+header/footer/caption. Equation e Located são fronteiras já preservadas pelo algoritmo vigente. Sob prova positiva, devolver clone do nó original sem atravessar ou reconstruir filhos; não consultar igualdade visual, não executar regra/callback, não alterar fonte ou escopo. Em qualquer outro carrier, executar integralmente map_content_pre vigente; a prova nunca trata variante desconhecida como vazia. Corpo Mathy, Label com Mathy, MathStyled e toda matemática não listada seguem o caminho atual. A mudança é só mecânica de custo no mesmo owner/fase, sem contrato público/default/unsafe/I/O (ADR-0127).

### Aceitação

Teste RED→GREEN de identidade estrutural Arc para tabela de texto/estilo simples e transformação efetiva de Mathy em célula, caption e receita. Gate glibc pré-fixado em 600.000 pedidos `local-set-1600`, benchmark pareado local/no-fill e vanilla, 23 fixtures com texto/páginas/caixas/raster/avisos inalterados. Reverter se o excesso de tempo não cair; suites, build, lint e resselo restrito do owner após qualificação.

### Resultado medido e decisão revista — 2026-10-09

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; os recibos `bare-math-{red,green}-gate.json.gz`, `measurements-{local-reuse,bare-math}.json.gz` e `validation-bare-math.json.gz` na pasta diagnóstica acima guardam o status e diffstat completos, UTC, comandos, fontes, fixtures e SHA-256 dos binários. O gate glibc predefinido **falhou**: 635.978→607.135 solicitações no caso local, 7.135 acima do limite de 600.000, embora tenha caído 28.843. A mediana CLI pareada passou de 262,72 para 249,00 ms; o excesso sobre `no-fill` caiu de 50,16 para 40,10 ms e a razão pareada Tekt/vanilla local de +53,6% para +45,8%. O controle `no-fill` passou de 212,56 para 208,90 ms. Janelas distintas sofrem variação do host; a redução de alocações e a direção conjunta de excesso/razão sustentam, mas não isolam com precisão, o ganho da transformação. A comparação das 23 fixtures encontrou zero diferença em texto, páginas, caixas, raster e avisos.

Como o critério primário desta busca é reduzir o excesso de tempo com equivalência de linguagem e render, manter a otimização sob as demais validações. O limite de 600.000 permanece registrado como hipótese quantitativa **não atingida**, sem renomear o gate falho como GREEN. Sua falha não prova regressão: o contador caiu. Revisitar o resíduo de alocações em ensaio próprio; se os gates de correção ou de build falharem, retirar esta implementação.


## Seletores de show — exaustividade e agrupamento, 2026-10-16

### Medição anterior à decisão

01_core/src/compiler/realize/bare_math.rs:168 (captures_equation_rule, emitted-v16). O enum em 01_core/src/entities/show.rs:63-96 contém Text, NodeKind, NativeElement, DynKind, Regex, Where, And, Or e Label. Fonte/linhas e funções atuais foram confrontadas com os avisos SARIF da revisão pinada e com o inventário AST anterior, verificando presença literal dos matches na árvore atual. Proveniência: 00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, hashes/fontes, status e git diff HEAD --stat; classification.json distingue avisos emitidos de predicado latente. Classificação: padrões/exaustividade são mecânica; seleção, texto, efeitos e erro de linguagem permanecem conforme os contratos vigentes. Não há nova medição de paridade ou inferência de ganho de leitura/performance.

### Decisão interna

captures_equation_rule conserva reconhecimento por endereço de native_math_equation e recursão Where/And/Or (any). Nomear Text/NodeKind/DynKind/Regex/Label como false. Não ampliar captura a NodeKind::Equation nem ao nome equation; alias real permanece elegível, nativa homônima falsa não. Não tocar mathy, needs_preparation, fronteiras de inserção, escopo, wrappers ou realização. Teste S3: nove variantes, alias executável, homônimo falso, Where aninhado, And/Or mistos e vazios, NativeWithEngine sem identidade elegível. Captura é conservadora por presença; filtro e conjunção não são avaliados aqui.

Alternativas com o mesmo resultado conservam um corpo compartilhado. Grupos explícitos informam a política vigente e exigem decisão quando Selector cresce; não duplicar corpos para zerar V19. V19 permanece informativo, sem afirmar déficit de cobertura ou fator de multiplicação de testes. Guard e políticas recursivas merecem controles próprios; uma execução do corpo não prova todas as alternativas. Nenhum allow/expect, exceção, ajuste de severidade ou helper genérico para esconder V16.

### Aceitação e limites

Fluxo contínuo ADR-0127: nenhuma assinatura/campo/trait público, default de produto, compatibilidade ou fase nova. Não reabrir propostas históricas deste owner. L0 antes dos testes/código e resselo restrito bilateral. Escrever os controles no módulo e confirmar o comportamento anterior; o RED estrutural é a ausência de E0004 nos seis pontos na cópia real do core com uma variante sentinela acrescentada apenas ao enum copiado. Após nominalização, os seis pontos devem exigir E0004 nessa cópia, sem stubs nem mutação da fonte produtiva. Os cinco V16 selecionados devem desaparecer da emissão, preservando diagnósticos externos e notas V19. Reexecutar testes reais, build, formato, Clippy estrito, Rustdoc e linhagem direta/reversa sob 12 GiB, swap zero e um job Cargo. Não declarar gate global verde enquanto V16 externos permanecerem.
