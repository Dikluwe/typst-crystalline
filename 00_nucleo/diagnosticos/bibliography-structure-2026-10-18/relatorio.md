# A2 — estrutura CSL, colunas e continuação

Concluída esta subetapa; **a bibliografia e o objetivo geral continuam abertos**.
Prefixo e corpo ficam em colunas estáveis, o espaçamento entre entradas obedece
a par.spacing e a continuação conserva o recuo. O prefixo acompanha a página
real de admissão do corpo. A pontuação adjacente permanece unida na quebra.

## Implementação

- Um plano privado distingue Columns/Hanging/Plain e preserva prefixo, corpo e
  fronteiras de cada entrada. É preparado junto do cache e compartilhado por
  Arc nas tentativas/checkpoints. BibRenderCache e as duas fachadas públicas
  mantêm seus campos, assinaturas e Content legado.
- A renderização usa Blocks quebráveis e prefixos Place privados, emitidos na
  primeira folha admitida, após widow/orphan. A recusa de preparar a sequência
  exterior não se propaga ao parágrafo próprio de Block. Simple e optimized
  conservam suas escolhas; o corpo não é renderizado novamente para antecipar
  uma página, nem há Location/Grid semântico sintetizado.
- Block conserva sua geometria horizontal nas regiões seguintes e restaura a
  geometria atual do pai ao sair. Subframes suspendem/restauram os contextos;
  checkpoints conservam plano, prefixo pendente e região. Não houve API pública,
  flag ou fase nova. L0s precederam as alterações; ownership e núcleos passaram.
- 20 arquivos existentes alterados no lote (dez fontes e seus dez
  L0s), mais o novo módulo/owner e três núcleos. Os demais
  2073 arquivos-base produtivos/L0 foram preservados.

## Medições e controles

IEEE12: o corpo passou de x25.124 para x29.924; IEEE18, de x33.686 para x40.886,
como o oracle. O gutter é 0.65em. par.spacing 2em muda a distância entre entradas
em 24pt comparado a zero; cores de show-set bare/where e OUTSIDE foram preservadas.
A lista sem prefixo passa a ter um Block por entrada, com margens de parágrafo.
A continuação IEEE90 mantém x29.924, mas ainda tem y5.012 contra 5.167996;
a margem abaixo do título também permanece pendente. Esses valores são
[medições residuais](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/geometry-summary.json.gz), não tolerâncias ocultas.

O primeiro candidato Grid passou nos testes locais de coluna, mas a medição
IEEE90pt revelou três páginas contra duas. Ele foi rejeitado; seu binário,
PDFs e fontes exatas foram arquivados. A primeira revisão Block/Place corrigiu
a contagem mas deixou a continuação em x8 e podia deixar o prefixo na página
anterior ao corpo. Esses REDs foram reproduzidos e corrigidos: a continuação
agora usa x29.924 e o prefixo acompanha a admissão. Os seis controles de
paginação 90/150/250pt mantêm conteúdo e contagem do oracle.

A primeira auditoria completa também detectou 2020 no fim da linha e ponto
isolado no caso 18pt. O controle não foi relaxado: a inspeção confirmou adjacência
no Content e supressão herdada do preparo. O corpo de Block passou a abrir seu
escopo próprio. A fixture inicial 54pt não cruzava a fronteira; o probe público
calibrou 52pt, obteve RED funcional e depois GREEN. Uma fixture anterior de
paginação também foi corrigida: exigir sempre a primeira página desrespeitaria
widow/orphan; o contrato válido é prefixo acompanhar a primeira linha real.
Houve um import ausente na fixture de inset, corrigido antes de contar RED.
Todas as tentativas estão preservadas e não contam como provas que não foram.

[Auditoria bilateral](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/audit.json.gz),
[primeira tentativa](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/attempt1-artifacts.tar.gz),
[fontes Grid verificadas](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/grid-sources.json.gz),
[proveniência do probe](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/block-adjacent-probe.json).

## Validação final

**9005 testes passaram, zero falhas, 7 ignorados;
cinco doctests.** Build workspace, Clippy all-targets com warnings e expectativas
não cumpridas negados, Rustdoc com warnings negados, quatro índices (incluindo
**typst**), fmt, V1/V5/V15/V26, reversa 988 e diff-check passaram.
Os 2098 hashes ficaram congelados em cada rodada. Depois da
suíte ampla, o Tekt apontou um V17 numa fixture. Foram alterados somente dois
módulos de teste e seus dois L0s: o predicado foi simplificado, as identidades
foram verificadas e a projeção Plain/Hanging ganhou cobertura explícita.
Os corpos produtivos dos dois arquivos foram comparados byte a byte, omitindo
somente o comentário de prompt-hash; todo o restante produtivo ficou idêntico.
Os 26 testes focados, Clippy e gates finais foram revalidados. A suíte ampla e
os PDFs permanecem provas do mesmo código produtivo, sem uma repetição sem
mudança de produto. [Comparação](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/production-equivalence.json).
As duas notas V19 novas são intencionais e prescritas: projeção pública comum
a Columns/Hanging/Plain e renderer temporariamente comum a Hanging/Plain.
Conservamos as notas e verificamos as alternativas; nenhuma supressão foi criada.

Além dos 25 controles anteriores deste lote (agora 26 com pontuação), foram
repetidos 12 casos de estrutura, 6 de paginação, 3 extras, 60 do corpus anterior,
44 textuais, 6 CSL customizados e 3 de cores. O relatório de cada caso distingue
texto/lista, geometria e divergências anteriores; igualdade de extração não
certifica toda a linguagem.

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 erro | 105 | 105 |
| V19 nota | 1120 | 1122 |
| V20 nota | 876 | 876 |

Nenhum erro/warning novo; o gate global continua bloqueado pelos V16 restantes.
O diferencial completo registra qualquer nota nova sem supressão.

## Pendências e próxima ordem

1. Aplicação física de hanging/default/overrides por ocorrência e parágrafos
   genéricos, preservando estratégias e direções. O plano já conserva o modo,
   mas Hanging ainda compartilha o ramo de Blocks simples; não foi silenciado
   o aviso de suporte incompleto em set par.
2. Margem entre título e corpo e diferenças verticais residuais medidas nos
   JSONs; não declarar paridade geométrica universal.
3. A2-Multi e clusters adjacentes de citações (MLA Silva/Doe ainda diverge).
4. A inspeção visual RTL refutou o suposto defeito de colchetes: ambos mostram
   [1]/[2] da mesma forma; a diferença de pdftotext é mecânica neste controle.
   Há outra diferença visível na posição do ponto final, já presente no Tekt
   anterior. Ela continua aguardando classificação pelo contrato de direção,
   sem inferir intenção ou copiar um possível defeito upstream.
   [Evidência e classificação](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-structure-2026-10-18/rtl-classification.json).
5. Alias-style ainda depende da realização/inserção R3-T1, cuja aprovação
   continua pendente. A proposta selada e suas bases mantiveram o estado
   registrado pelo overlay predecessor. E01–E20 e frentes públicas RM1/RG/FG/
   Paint/Symbol permanecem no objetivo integral.

## Recursos e proveniência

Pico dos jobs limitados: **4760051712 bytes (4.433 GiB)**;
MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1/debug 0/incremental 0. Nenhum evento
high/max/OOM/OOM-kill. Não é consumo total da máquina nem benchmark de ganho
pareado com o vanilla. Os comandos pesados foram executados em sequência.

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch `Tekt`, working tree não commitado.
before-sources.json.gz e after-sources.json.gz preservam fontes, L0s, UTC,
status e `git diff HEAD --stat`; change.patch.gz guarda o delta restrito.
complete-results.json/complete-freeze.json.gz preservam a rodada ampla;
final-gates-results.json/final-freeze.json.gz registram a rodada de fixtures
e gates, com sua comparação de produção. Os recibos têm comandos/UTC/recursos;
final-state.json.gz consolida o estado. O build final adicional e o diferencial
de lint com entrada explícita estão em supplement-results.json; o congelamento
final foi conferido novamente após esses comandos.
O manifesto predecessor foi verificado contra seus artefatos e, para arquivos
produtivos agora alterados, contra os bytes iniciais preservados deste lote.

Oracle pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Tekt final
`8c355c1208e0c2eb98ceeae0e4bbf37387ff20b30643d8f34d28c47c97ec4471`. Pin/binário do linter preservado.
Sem commit, push, merge, publicação ou release.
