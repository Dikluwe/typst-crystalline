# Passo 1503 — separar o Cursor por responsabilidade verificável

**Estado:** IMPLEMENTADO em 2026-09-25; o microperfil exclusivo de
`flush_line` ficou não medido por restrição de `perf` no host. Este passo é
coordenação e registro de execução, não Prompt L0.

## Medição antes da decisão

Em 2026-09-25T19:48:28Z, HEAD
`0de5edabed197d23b647cfc68ebefda2f5c2a8d5`, working tree limpa
(`git status --short` e `git diff HEAD --stat` vazios):

- `01_core/src/compiler/layout/cursor.rs` tem 1.990 linhas, SHA-256
  `f8e5694658685b08f9f4f84dd5b1aec0f73a20860dd0436eb66d714b1651462e`.
  No próprio arquivo, `layout_word`/`layout_chunk` estão em `:281-360`,
  expansão fracionária em `:406-528`, justificação e fechamento da linha
  em `:529-1038`, transições de página/coluna em `:1039-1420`, coordenação
  de floats e replay em `:1421-1764`, tails de células em `:1765-1877` e
  segmentação textual/CJK em `:1878-1985`. Estes intervalos são um mapa
  inicial de funções, não prova de independência entre elas.
- `01_core/src/compiler/layout/mod.rs` tem 3.667 linhas, SHA-256
  `de7e09c3e188c16e5bf3f074dc315472337caf7e8ea81ebc430ef1941f99f53c`.
  Define o estado compartilhado de `Layouter` em `:326-790` e o despacho
  exaustivo em `:1788-2395`. Extrair um método do Cursor não retira
  automaticamente seu estado nem o custo algorítmico deste centro.
- O vanilla pinado é `9f2b6e8715237cb086899a42873660fe744622e8`.
  `lab/typst-original/crates/typst-layout/src/inline/line.rs` tem 925 linhas;
  `flow/mod.rs`, 445. São referências de localização de responsabilidades,
  **não** exigência de copiar sua estrutura ou igualdade de linhas.

O L0 vigente `00_nucleo/prompts/compiler/layout/cursor.md` (SHA-256 bruto
`4edb908f0f88a31722389d3b8be4b8c2c2be676a10a25b70074cece4bdc625bd`)
atribui explicitamente ao Cursor palavras/chunks, wrap, baseline,
`flush_line`, paginação e footnotes. O L0
`00_nucleo/prompts/compiler/layout.md` (SHA-256 bruto
`d3738fad10b9346f312bd4975d385e892885bf4f0a1f9e16da00a3650767842f`)
atribui o estado compartilhado e o dispatcher ao módulo raiz. **A divisão
proposta só pode existir depois de individualizar esses contratos, sem dois
prompts para o mesmo consumer nem um prompt compartilhado por dois consumers
(ADR-0129).**

Esta medição é estrutural. Não demonstra que `flush_line` seja o maior custo
de CPU, que o Tekt seja mais/menos rápido por causa do número de arquivos ou
que uma extração melhore performance. Os benchmarks recentes dos documentos
pequenos indicam tempo substancial fora do pipeline cronometrado; portanto
não atribuir a diferença de arranque ao Cursor sem uma medição nova.

## Hipótese, classificação e refutadores

**Hipótese de engenharia, ainda não decisão L0:** o fechamento de linha é a
primeira responsabilidade candidata a sair de `cursor.rs`. Seu ciclo inclui
expansão horizontal pendente, justificação, consolidação de baseline/items e
commit da linha; wrap/segmentação, avanço de página/coluna e política de floats
permaneceriam nos seus donos atuais. O candidato só é válido se seu contrato
for legível no próprio arquivo, com entradas/saídas e estado causal claros,
sem duplicar decisões de fitting, transação ou paginação.

Refutam a escolha: o mapa de leituras/escritas mostrar que fechar uma linha
exige conhecer indistintamente a política de página/float; a extração apenas
criar wrappers e saltos entre arquivos; aumento material de acoplamento ou
visibilidade; ou perfis reproduzíveis mostrarem que outro ponto concentra a
latência relevante. Nesse caso, registrar o achado em `diagnosticos/` e
escolher outra fronteira medida, **sem** materializar `line_commit` por inércia.

O objeto de paridade é a linguagem — semântica, sintaxe e morfologia
(ADR-0107). Nomes de métodos, bytes do PDF, contagem de arquivos e igualdade
de `FrameItem` são mecânica. Uma reorganização content-preserving não ganha
licença para alterar páginas, posições, texto, links, estilos ou mensagens de
erro observáveis.

## Execução proposta

1. **Mapa de responsabilidade e estado.** Inventariar, com `file:line`, cada
   função do Cursor, suas chamadas, campos de `Layouter` lidos/escritos e
   checkpoints que os capturam. Marcar explicitamente as interações com
   `sequence.rs`, `repeat.rs`, `place.rs`, `columns.rs`, `page_run.rs` e
   `layout/mod.rs`. Registrar o mapa e os conflitos em `diagnosticos/`;
   não converter este passo em Prompt L0.
2. **Baseline reproduzível.** Congelar HEAD, `git diff HEAD --stat`, hashes
   das fontes e binários, pin do vanilla, comandos, ambiente e horário.
   Selecionar somente documentos aceitos pelos dois compiladores com saída
   semanticamente equivalente. Cobrir linha simples, wrap, estilos/tamanhos
   mistos, `fr`/`repeat`, coluna, float/flush e HTML carrier, além de casos
   de erro. Registrar página, texto, geometria e diagnósticos; medir release
   com warmup, repetições, CPU, tempo e pico de RAM. Instrumentar ou perfilar
   **separadamente** inicialização/fontes, layout, fechamento de linha e
   transição de região; não subtrair tempos de relógios distintos como se
   fossem uma decomposição exata.
3. **Decisão de fronteira antes do código.** Se a hipótese sobreviver, propor
   uma única unidade de fechamento de linha (nome provisório
   `01_core/src/compiler/layout/cursor/line_commit.rs`), mantendo wrap, página/coluna,
   floats e segmentação no `cursor.rs`. Auditar todos os L0s afetados,
   redigir o Prompt L0 **proprietário**
   `00_nucleo/prompts/compiler/layout/cursor/line_commit.md` da unidade
   nova e atualizar `compiler/layout/cursor.md` antes de qualquer código;
   atualizar `compiler/layout.md` somente se seu consumer mudar. Não criar
   Núcleo Tekt para obrigações que só uma unidade consome. Depois de criar
   o consumer, verificar V15/V26 **antes de ressellar hashes**; um L0 novo
   ainda sem consumer é transitório e não pode ser declarado GREEN. Se o
   mapa pedir outro recorte, registrar primeiro a
   nova medição/decisão e rever os L0s correspondentes.
4. **Caracterização e materialização.** Escrever controles de comportamento
   e um critério arquitetural verificável antes da extração; registrar o
   estado inicial de cada um. Os controles de paridade devem estar GREEN
   antes e depois; o RED deste refactor é a unidade nova ainda sem a
   capacidade/contrato testado, não uma regressão funcional fabricada.
   Confirmar RED→GREEN para esse critério antes de declarar a extração
   concluída. Mover somente a lógica do owner decidido,
   conservar o `match` exaustivo, despacho estático e pureza L1. Não introduzir
   vtable, `dyn`, import reverso `entities→compiler`, clone de estado por
   conveniência ou nova fase de pipeline. Não alargar visibilidade sem
   justificar o acesso no L0.
5. **Comparação e aceite.** Repetir as sondas com as mesmas entradas e bins
   identificados por hash; comparar semântica/morfologia, tempo, CPU e RAM,
   incluindo dispersão. Relatar também raio de mudança: arquivos e owners
   tocados por uma correção focal de linha antes/depois da extração. Aceitar
   atomização somente se o fechamento puder ser entendido e alterado no
   seu owner, com invariantes partilhadas explicitadas e sem regressão
   observável. Ganho de performance é resultado medido, **não** condição
   presumida da extração. Concluir com `cargo build`, testes focais,
   `crystalline-lint .`, V5/V15/V26, dry-run de hashes e `git diff --check`.

## Gate ADR-0127 e limites

Se a fronteira permanecer interna, sem mudar assinatura pública,
comportamento por defeito, fase ou compatibilidade, aplicar L0 primeiro e
seguir em fluxo contínuo com revalidação. Se qualquer uma dessas quatro
classes aparecer — ou a classificação for duvidosa — **parar depois do L0 e
pedir confirmação humana antes de código**. Este passo não antecipa essa
confirmação.

Não misturar a extração com otimização algorítmica ou correção de paridade.
Uma eventual melhora de tempo/memória ou bug do vanilla terá hipótese,
medição e passo próprios. A finalidade aqui é obter uma fronteira de
responsabilidade que permita atribuir com precisão o efeito dos passos
seguintes.

## Execução e decisão final P1503

No HEAD inicial `0de5edabed197d23b647cfc68ebefda2f5c2a8d5`,
com somente este passo não rastreado, a auditoria de
`cursor.rs:35-1985` produziu
`00_nucleo/diagnosticos/typst-p1503-responsabilidades-cursor.md`.
A fronteira sobreviveu com uma correção ao nome provisório:
`01_core/src/compiler/layout/line_commit.rs` é irmão de `cursor.rs`,
não filho. Assim os métodos conservam `pub(super)`, os caminhos
`super::` e o acesso já existente ao `Layouter`, sem alargar
visibilidade ou duplicar estado. A transição de página, floats,
colunas, replay e segmentação continuam no Cursor.

Os L0s `compiler/layout/cursor.md` e `compiler/layout.md` foram
atualizados, e `compiler/layout/line_commit.md` criado, **antes**
dos testes e do movimento de código. Não houve contrato público,
default, fase ou compatibilidade novos; aplicou-se fluxo contínuo
ADR-0127. O teste arquitetural no Cursor falhou antes (RED 0/1,
exit 101, log de 2026-09-25T20:03:59Z) e passou depois (GREEN 1/1,
20:06:11Z). Um teste funcional de justificação reside no owner novo;
os dois testes P1503 passam. O refactor moveu 639 linhas de métodos e
39 de helper sem trocar suas fórmulas. `cursor.rs` ficou com 1.324
linhas, contra 1.990 antes; `line_commit.rs` tem 724 com teste.

V15/V26 passaram antes do resselo. `--fix-hashes --dry-run` listou
somente `cursor.rs`, `line_commit.rs` e `layout/mod.rs`; após
o reparo, V5/V15/V26 passaram e o dry-run devolveu
`Nothing to fix`. A suíte L1 final passou 6.196 testes, dois
testes antigos de recursão filtrados; P1407 L3 5/5, P1493 L3 6/6,
P1502 L3 4/4. `cargo build --offline`, release offline,
`crystalline-lint .` e `rustfmt --check` dos três arquivos tocados
passaram. O lint completo ainda emite avisos preexistentes, mas
nenhuma violação bloqueante.

O release final SHA-256
`aa1128d20fb3a8ef83bd8155560bbba6116414fd4a3bb39931b704c2c89b5de2`
foi reconstruído após o último ajuste de cabeçalho e manteve
identidade de bytes com o release já medido. Nas sondas `lorem` e
`repeat`, texto e bbox de palavras do cristalino são iguais
antes/depois (ignoradas apenas `CreationDate`/`ModDate` do
cabeçalho PDF); o texto continua igual ao vanilla pinado.
Tempo e RAM não tiveram melhora demonstrável. Medições, dispersão,
fontes, hashes, comandos, raio de mudança e limites constam do
diagnóstico P1503.

Um harness externo mediu 10 vezes o catálogo do sistema: ~270,2 ms
no cristalino contra ~123,2 ms no vanilla, ambos com 1.112 faces.
`with_fonts_and_system` consumiu ~275,5 ms e o layout inteiro de
`repeat` ~0,6 ms numa sonda do release final. Isto localiza uma
prioridade futura **fora** de `line_commit`; não se misturou
otimização de fontes a esta extração. `perf stat` foi bloqueado
por `perf_event_paranoid=4`, logo o tempo exclusivo de
`flush_line` e de transição regional permanece não medido.
Não alegar que a atomização o reduziu.

Raio de mudança para uma correção puramente local de justificação:
antes `cursor.rs` + seu L0; depois `line_commit.rs` + seu L0,
ainda 2 arquivos. O ganho verificado é **localidade e ownership**
da responsabilidade, não menor contagem de arquivos tocados nem
performance. Mudanças em checkpoint/estado continuam transversais.
