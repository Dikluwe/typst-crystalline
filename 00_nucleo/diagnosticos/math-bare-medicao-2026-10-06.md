# D06 — medição e proposta de inserção matemática

Estado: diferença comprovada; pacote L0 completo preparado, aguardando gate
ADR-0127 por nova fronteira de realização e fachada pública L1. Código e L0
vigentes permanecem inalterados nesta etapa. Não é fechamento de D06.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Coletas em `math-bare-2026-10-06/` guardam UTC, git diff HEAD --stat, lista exata
de alterações, hashes das fontes, fontes Typst, comandos/cwd e binários congelados.
`baseline-state.json.gz` guarda também conteúdo de fontes/L0 modificados.
Ambiente: NO_COLOR=1, SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt release SHA-256
`c2fd3dfa765e2f1135036b20bfd78d4d776011d4821a49a9a5a52dc87db747b1`.

- before: 2026-10-06T20:36:31.650793+00:00 a 20:36:43.579935+00:00;
  41 controles, nove coincidências integrais em status/stdout/stderr/texto PDF.
- additional: 2026-10-06T20:38:47.429164+00:00 a 20:38:53.604645+00:00;
  20 controles, quatro coincidências integrais nesses mesmos observáveis.
- refuters: 2026-10-06T20:39:21.356113+00:00 a 20:39:22.724812+00:00;
  quatro controles, quatro coincidências integrais.

Essas coletas são conjuntos diferentes; não representam percentual de paridade
global ou prova de correção. `results.json` lista cada diferença/coincidência.
additional guarda também bbox XML de PDF como observável geométrico auxiliar;
nenhum número geométrico é usado nesta etapa para declarar paridade.

## Medição anterior à decisão

before prova que frac, mat, sqrt, lr e op nus produzem uma Equation consultável
no vanilla e zero no Tekt. Na fração, o texto PDF Tekt é `(1)/(2)`; o vanilla
compõe numerador/denominador. A Equation explícita de additional já coincide em
query e texto extraído, isolando a preparação da inserção do algoritmo de fração.
Igualdade textual desse controle não certifica geometria ou bytes PDF.

Duas inserções de um mesmo valor nu produzem duas Equation no vanilla. Conteúdo
já dentro de `$...$` mantém uma Equation; não envolver cada descendente. Texto
comum e math.bold([x]) têm zero nos dois, mas math.bold(frac(...)) tem uma no
vanilla. Envolver toda variante Math* ou todo texto com math style seria errado.

block herdado e local são efetivos: o controle local tem duas equações, uma de
bloco e uma inline no vanilla. Numbering efetivo produz número da equação de
bloco. Alt é consultável. Conteúdo de box, produto de heading show e produto
contextual também passam pela fronteira externa de inserção.

show com .where transforma a equação automática no vanilla. A regra posterior
transforma só a segunda inserção. Alias it conserva uma ocorrência e compila;
it.body inserido fora de math cria novas equações e o vanilla rejeita com
maximum show rule depth exceeded. Não copiar ponto-fixo por corpo igual.

refuters confirma o valor cru frac e repr antes de inserção: ambos coincidem,
sem Equation no Value. Label da fração nua não vira label da Equation: o filtro
Equation.where(label:<f>) retorna zero nos dois, enquanto Equation explicitamente
rotulada retorna um. Não promover label por conveniência.

Inspeção da fonte pinada: `lab/typst-original/crates/typst-realize/src/lib.rs:245–265`
executa kind rules antes de show; `:302–334` distingue contexto Math e envolve
Mathy externo. O catálogo Mathy está declarado nos elementos de
`lab/typst-original/crates/typst-library/src/math/`, não pelo prefixo do nome.
Isso mostra comportamento e ordenação; a intenção de upstream não foi inferida.

Tekt `01_core/src/compiler/layout/equation.rs:680–689` projeta math externo em
plain_text. `compiler/realize/mod.rs:155–199` prepara receitas e ocorrências sem
essa normalização; `compiler/eval/rules/apply.rs:564–607` realiza inseridos só
quando o elemento já é localizável. `entities/content/traversal.rs:14–108` visita
bottom-up inclusive corpo Equation: envolver math em callback genérico criaria
wrappers internos incorretos. `03_infra/src/pipeline/context_stabilization.rs:213–237`
guarda contribuição após apply_func sem realizar math externa.

## Classificação e proposta

Identidade consultável, show, escopo, papel de bloco, render e conservação de
valor cru são observáveis de linguagem. A solução proposta normaliza a inserção
antes de show/índice, usando enum fechado e helpers puros, com contexto math e
lexical explícitos. Não copia dyn Mathy, vtables ou a estrutura upstream.
A hipótese é refutada por wrapping dentro de math, retroação de regras, perda de
estilo ou contagem, efeito duplicado, valor cru alterado ou regressão geométrica.

Pacote revisável: `00_nucleo/materialization/math-bare-2026-10-06.md`.
Nove L0s completos: oito owners existentes auditados e um owner novo
compiler/realize/bare_math. Núcleo compartilhado proposto com pins SHA-256 completos,
sem instalação no namespace produtivo enquanto o gate estiver pendente.
Assinatura pública nova da fronteira de contribuição está escrita integralmente
no L0 candidato compiler/realize/mod. Travessia pré-ordem adicional é interna ao
crate; map_content/map_text públicos conservam comportamento.

Não foram executados RED→GREEN, build ou lint de implementação nova porque não
há implementação ratificada. `l0-audit.json` confirma os oito selos vigentes;
`preservation.json` verifica que fontes/L0 não mudaram nesta etapa.

## Limites e achados separados

show math.equation direto é rejeitado no Tekt, embora .where seja admitido.
Mapping por identidade estática é correção interna separável; está incluído no
pacote para que direto/alias/where tenham a mesma aceitação após materialização.

query(<f>) sobre a fração rotulada retorna frac no vanilla e vazio no Tekt;
a localização do Mathy original é achado independente da Equation automática.
Morfologia pública de mat continua D07. Aviso de arquivo vazio dos primeiros
raw-eval é outro observável; não confundir com diferença de frac cru.
As sondas additional raw-value-func/body usaram variável do arquivo na expressão
auxiliar de eval --in: unknown variable f nos dois não prova preservação do valor;
refuters substitui essa inferência por bloco avaliado diretamente.
HTML/math completo, variantes ainda não medidas, callback/rich-numbering e
geometria ampla não foram certificados. D05 e aspas já corrigidos não voltam a
ser classificados como abertos por esses negativos.
