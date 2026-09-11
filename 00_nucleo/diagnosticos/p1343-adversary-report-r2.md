# P1343 — revalidação adversarial R2

## Veredito

**ADVERSARIAL_SURVIVORS_BLOCK_PRESEAL**.

Regime: **executado sem atestacao de isolamento**. A autoridade adversarial
recebeu apenas o Passo 1343, baseline/contrato e artefatos R1/R2 pinados; não
editou contrato, baseline, source verifier, checker, corpus, L0 ou código
produtivo. O workspace é compartilhado e por isso não há atestação técnica de
isolamento.

O R2 corrigiu os onze controles do relatório R1, mas não discrimina 16 de 20
novos negativos válidos. O score focal total foi `14/30 = 0.4666666666666667`.
Logo é proibido executar o corpus completo, emitir preseal ou iniciar RED e
implementação.

## Proveniência reproduzível

Medição final em `2026-09-10T22:25:23-03:00`:

- HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitado;
- SHA-256 de `git status --porcelain=v1 -z`:
  `c246a90c42403e61cf9b07dc50403f72ca96d946d6ec208cd251f09ff9804997`;
- SHA-256 de `git diff --binary HEAD`:
  `f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`;
- stat tracked: 72 arquivos, 10889 inserções e 830 remoções;
- comando:
  `python3 -B 00_nucleo/diagnosticos/p1343-adversary-runner-r2.py`;
- temporários exclusivamente em `/dev/shm`;
- execuções do corpus completo: **zero**;
- runner SHA-256:
  `8f93520298b8751fd8fbea3430e014ce6e9b938a05431d929538538773402aaf`;
- relatório JSON SHA-256:
  `9900f35702456c59d1183a9d43ab3c3c32afe6a10c3c82c64f8903a1f4488586`.

Os hashes R2 recebidos conferiram antes da execução: source verifier
`fa7c15ee…512d`, checker `ff6d83c9…8ab9`, corpus `72eb4117…fcce`, authorship
`eaf46ddb…171c` e receipt `d1b24084…dc2`. O JSON registra os pins completos.

## Reexecução de ADV01–ADV11

Os dez negativos antigos foram corretamente `Violated` e o controle positivo
ADV11 foi `Preserved`; `rustfmt` analisou sintaticamente os 10/10 arquivos do
positivo sintético. Em particular, ADV10 com bytes alterados e o digest R2
original foi rejeitado por `CORPUS_PIN`.

Esse resultado confirma ganho discriminatório real de R1 para R2. Ele não
elimina os survivors novos: R2A17 demonstra uma fronteira diferente, na qual o
atacante fornece bytes e digest autoconsistentes ao mesmo tempo.

## Resultado dos novos ataques

Foram executados vinte negativos novos e finitos. Todos os mutantes de source
que chegaram a `Preserved` também passaram pelo parse do `rustfmt`; nenhum foi
descartado por sintaxe inválida.

| Ataque | Resultado | Witness mínimo |
|---|---|---|
| R2A01 writer qualificado | **survived** | `decoy::p1343_append_raw` satisfaz o suffix matcher |
| R2A02 `retain` | **survived** | mutador não consta da regex fechada |
| R2A03 `get_mut` + assignment | **survived** | assignment indireto não é enumerado |
| R2A04 hook em closure morta | **survived** | somente `fn` aninhada é rejeitada |
| R2A05 literal H01 em comentário | **survived** | chamada vem do mask, literal vem dos bytes originais |
| R2A06 literal em string | Violated | literal não satisfez o matcher concreto |
| R2A07 token extra no ramo normal | Violated | `CFG` detectou alteração efetiva |
| R2A08 `cfg /* split */ !` | **survived** | comentário rompe o substring guard de `cfg!` |
| R2A09 fachada sob `if false` | **survived** | callsites exatos sem prova de reachability |
| R2A10 três nomes, mesmos objetos | **survived** | repeat/reverse clonam normal e ainda passam |
| R2A11 runtime formatado e fabricado | **survived** | joins de strings não provam execução causal |
| R2A12 `Event` pós-hoc por struct literal | **survived** | nome neutro evita duas palavras proibidas |
| R2A13 `Event` pós-hoc por macro | **survived** | macro neutra não é enumerada |
| R2A14 projeção muta via helper | **survived** | helper usa `retain`, também não enumerado |
| R2A15 snapshot sombreado | **survived** | clone exato existe, mas o valor usado é substituído |
| R2A16 clone exato em closure morta | **survived** | regex não prova caminho executado nem uso |
| R2A17 corpus/digest autoconsistentes | **survived** | checker saiu 0 com corpus de um positivo |
| R2A18 duplicate JSON key | **survived** | `json.loads` aceitou dois `budget`, checker saiu 0 |
| R2A19 source allowlisted como symlink | Violated | `PATH` rejeitou o symlink |
| R2A20 decoy fora da cápsula | Violated | hash integral normalizado divergiu |

## Causas medidas

1. **A identidade do writer não está ligada à chamada.** Em
   `p1343-source-verifier-r2.py:222-230`, a regex aceita o nome como suffix de
   uma qualificação arbitrária. Além disso, localiza a chamada no `code_mask`,
   mas procura o literal de hook nos bytes originais de uma janela de 240
   bytes. Isso explica R2A01 e R2A05.

2. **Reachability é uma exclusão nominal incompleta.** Em `:317-323`, somente
   uma declaração `fn` dentro de certas cápsulas é tratada como morta. Closure,
   `if false` e outros fluxos não executados continuam observáveis como texto.
   R2A04, R2A09 e R2A16 demonstram a mesma classe em hook, fachada e freeze.

3. **A propriedade append-only foi reduzida a uma lista de métodos.** A regex
   em `:27` enumera `push|extend|append|insert|splice|clear|truncate|drain`; as
   verificações de `:304-316` acrescentam apenas assignment direto e uma forma
   de alias. `retain`, `get_mut`, helpers e várias formas equivalentes ficam
   fora. R2A02, R2A03 e R2A14 sobrevivem sem alterar o único `push` esperado.

4. **Ausência de construção pós-hoc virou busca por duas palavras.** Em
   `:324-332`, apenas identifiers que contêm `posthoc` ou
   `forged_after_execution` são recusados. Struct literals, macros e funções de
   nome neutro permanecem invisíveis, como R2A12/R2A13.

5. **Freeze prova presença e ordem textual, não def-use.** Em `:338-341`, basta
   existir o clone exato antes do primeiro texto do writer. O scanner não prova
   que o snapshot não foi sombreado, que saiu de uma closure executada ou que a
   projeção consumiu o objeto clonado. R2A15/R2A16 são witnesses diretos.

6. **Frescor/runtime são identidades autodeclaradas.** Em `:233-266`, os três
   challenges e run IDs precisam ser strings distintas e os receipts precisam
   obedecer ao formato derivado dessas strings. Em `:342-362`, a fachada exige
   nomes e três chamadas textuais. Nenhuma relação prova que os objetos são
   diferentes ou que os receipts nasceram dessas chamadas. Isso explica
   R2A09–R2A11.

7. **O pin externo não é uma raiz fechada no checker.** Em
   `p1343-oracle-checker-r2.py:309-323`, path e digest são argumentos irmãos;
   qualquer par autoconsistente é aceito. O próprio corpus então define
   `cases`, ordem, budget e pins internos (`:323-356`). R2A17 reduziu-o a um
   positivo e obteve `agreement=true`. O parse permissivo de `json.loads` em
   `:323` também explica R2A18. Isso não refuta ADV10: digest original contra
   bytes alterados continua bloqueado; refuta a alegação mais forte de que o
   checker sozinho fixa qual corpus a autoridade deve usar.

8. **As fronteiras de bytes continuam fortes.** Os ataques de token efetivo no
   ramo normal, symlink allowlisted e byte fora da cápsula foram rejeitados.
   A normalização integral continua discriminando escopo externo; os survivors
   concentram-se na semântica interna e na composição de autoridade.

## Condição de parada e próximo passo permitido

Um survivor válido já impede preseal. O conjunto solicitado foi fechado após
31 provas (replay de 11 + 20 novas); não houve busca aberta nem execução full.
Não cabe a esta autoridade corrigir o material julgado.

Uma nova revisão precisa mudar a hipótese, não apenas ampliar regexes. Em
particular, ela precisa decidir quais propriedades pertencem a uma prova Rust
executável/compilada em vez de tentar inferir reachability, def-use e
append-only por scanner lexical. Também deve fixar a identidade do corpus fora
do par path/hash controlável pela invocação e usar JSON estrito antes de nova
calibração focal. Somente uma revalidação independente survivor-free poderá
autorizar a execução completa de preseal.

## Limitações

- A execução cobre somente o binding focal P1343/P1342; não julga P1340,
  NT01–NT06, retenção ou política terminal.
- Não houve build do candidato, teste A/B, race, corpus completo ou selo.
- `rustfmt` prova parse sintático dos mutantes, não resolução de nomes nem
  compilação de cada árvore sintética.
- O adversário produziu diagnóstico e ataques; não escreveu correções.
