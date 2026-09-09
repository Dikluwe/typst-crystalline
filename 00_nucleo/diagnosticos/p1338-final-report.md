# P1338 — erro de campo ausente em Array corrigido

## Efeito entregue e limites

`(1,2).nope`, `().absent` e aliases Unicode/multilinha agora informam
`cannot access fields on type array`, sublinhando somente o identificador
do campo. Antes, o cristalino publicava `array does not contain field "…"`
e sublinhava o acesso inteiro. O lookup puro preserva o span recebido.

A implementação mudou apenas dois pontos de
`01_core/src/compiler/eval/bindings/field_access.rs`: Array participa da
seleção de span AST e o ramo de campo ausente publica a mensagem correta.
Não foram criados campos, métodos, assinaturas ou fases. O L0 proprietário
`00_nucleo/prompts/compiler/eval/bindings/field_access.md` foi atualizado antes
do código. Uma sentinela Array antecedente foi expressamente sucedida, sem
apagar o caso ou relaxar comparadores; os demais testes antigos permaneceram.

**Ainda não há paridade total de Array.** Permanecem separadas e preservadas:

- `len`, `first` e `last` como valor: a AST pode despachar um método antes do
  lookup; por exemplo, `(1,2).len` ainda devolve valor onde o vanilla erra.
- `().first` e `().last` continuam com erro antecipado de array vazio; no
  lookup puro, first/last vazios continuam a devolver None. Essa diferença
  entre rotas não foi mascarada pelo reparo.
- `array.nope` é acesso ao valor-tipo, com dívida própria de mensagem/âncora.
- `(1,2).nope(panic("arg"))` preserva a divergência anterior de ordem de
  avaliação. Length, como `(1pt).nope`, conserva sua dívida de sublinhado.
- Content e demais superfícies excluídas não foram reabertas. Não se validou
  PDF exportado, layout, acessibilidade geral ou todo o catálogo contextual/
  de warnings. O inventário global de paridade não foi reexecutado.

Chamadas normais, wrappers estáticos, lookup puro len/first/last e diagnósticos
Bool/None/Auto e Int/Str foram protegidos. A correção se limita ao acesso
Array ausente que efetivamente chega ao lookup deste owner.

## Verificação executada

Proveniência comum: working tree **não commitada** sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. O baseline
`p1338-baseline.json` (SHA-256
`6d85aece9e323950a8f722b11e87eb125b4a342a7404f969f60ea69595790b3c`)
e cada recibo de execução preservam diff/stat, lista exata dos arquivos
alterados, inventário, argv, UTC e saídas completas. Os 16 arquivos tracked
alterados incluem deltas herdados; somente o par proprietário acima recebeu
mudança produtiva neste passo. Nenhum resultado é atribuído apenas ao HEAD.

| Gate | Resultado | Evidência |
|---|---|---|
| RED anterior a C | 4 passaram, 3 falharam por assertions reais, sem falha de compilação | `p1338-unit-red.json` e `p1338-review-prepatch-go.json` |
| GREEN release normal | 7 passaram, 0 falharam | `p1338-unit-green.json`, 2026-09-09 20:21:48.121095–20:23:50.044276 UTC |
| Build workspace release --locked | Passou | `p1338-candidate-build.json`, 20:24:23.286696–20:25:06.598661 UTC |
| Suíte workspace release --locked | 6.749 passaram, 0 falharam, 3 ignorados | `p1338-workspace-tests.json`, 20:25:37.540053–20:28:29.006536 UTC |
| A/B CLI | 540/540 Satisfied; zero Unknown obrigatório ou instabilidade | `p1338-tests-candidate.json`, `p1338-tests-comparison.json` e `p1338-tests-summary.json`, 20:26:03.714938–20:26:19.783401 UTC |
| Mutação real | Controle C passou; quatro famílias recompiladas rejeitadas por assertions | `p1338-attacks-results.json`, `p1338-attacks-final.json` e recibos C/M1–M4 |
| Formatação/diff | Passaram | `p1338-fmt.json`, `p1338-diff-check.json` |
| Linhagem estrita | V5/V15/V26: zero violations | `p1338-lineage-final.json`; cálculo recíproco em `p1338-lineage-reciprocal.json` |
| Lint geral | Exit 0, zero error; 240 warning e 1.148 info preexistentes | `p1338-lint-general.json`; multiset idêntico ao P1337 após normalizar somente coordenadas de origem |

O total do workspace é a soma dos blocos `test result` do recibo, incluindo
o controle LocatedContent antecedente. Isso não cria nova cobertura de
introspecção pela CLI. A contagem do lint usa prefixos de linha do stdout
integral; warnings e infos existentes não são anunciados como quitados.

O A/B cobre 45 expressões, quatro perfis (default/html/a11y/html+a11y) e
ordens normal/repetida/inversa. São **120 convergências**, **300 preservações
de paridade** e **120 preservações de dívida**. Dívida preservada não conta
como igualdade com vanilla. Os 1.080 envelopes bilaterais anteriores a C e
as expectativas estão em `p1338-tests-baseline.json` e
`p1338-tests-expectations.json`. Foram comparados exit/stdout/stderr completos,
UTF-8/base64, sem normalização; controles opacos deliberados testam somente
o harness e não ganham crédito de produto.

Os ataques detectaram: mensagem Array errada, span Array agregado, first puro
trocado pelo último item e extrapolação indevida da âncora para Length.
Cada rodada preserva fonte, patch, logs e executável realmente usado com
hash, timestamp fresco e prova de recompilação na cópia. Não houve retry,
cache antigo, falha de compilação ou Unknown obrigatório nas cinco rodadas.
Tempo Cargo somado: 229,95 s, conforme recibos individuais; não é benchmark.

O perfil adversarial foi congelado antes de C: somente typst-core com
opt-level=0, dependências release, jobs=2. Controle C e mutantes executam os
mesmos seis testes nesse perfil. Essa evidência prova discriminação
instrumental, não substitui os gates release normal acima.

## Identidade, sucessões e preservação

Vanilla ratificado upstream/main **a51e02804**, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline P1337, `/tmp/p1337-target.F7jPNj/release/typst`, SHA-256
`55b5dc263bba15b050e9caab755c17deb22f617eec35b1e9259a20a57f9b350b`.
Candidato, `/tmp/p1338-target.vlNAmp/release/typst`, SHA-256
`f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1`.
Fonte C SHA-256 `67eee58a73878438d54be8044c97d570e276889d6637ad43b305dd30c6a51937`.
L0 normativo efetivo SHA-256
`c76d40542c2f94f9ff57c4ea67e79eac1920432c7e33637e97c22321ff2c9d20`;
selos A `7e89a8ce`, B `a4267e57`. Não houve resync do lab.

A skill `tekt-materializacao-segregada` separou autoria de implementação,
testes/oráculos, ataques e veredito. Regime **A/B sem atestação técnica de
isolamento e sem selo de refinamento**. Os contextos anteriores dos três
autores foram explicitamente retidos por limite total de agentes; não eram
contextos novos. O implementador não leu os novos testes privados antes de C;
o autor do A/B não leu a fonte C. As allowlists declaradas não equivalem a
restrições técnicas demonstradas.

Incidentes foram mantidos em evidência, não apagados:

- O revisor encontrou a referência L0 herdada `bindings/access.md`, incompleta
  sob ADR-0130. R1 corrigiu-a para `compiler/eval/bindings/access.md`, com
  novo hash causal, antes de C. A obrigação Array não mudou. Manifesto R0
  foi preservado; efetivo R1 SHA-256
  `14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843`.
- Houve integração provisória e retirada somente dos testes novos antes de
  R1, por cruzamento de mensagens. A fonte voltou exatamente ao baseline
  R0 antes do resselo, depois houve reintegração R1; nunca existiu C naquele
  intervalo. A janela causal, o check provisório de formatação e um patch
  rejeitado sem escrita parcial estão em `p1338-tests-operational-sequence.md`.
  O freeze final já estava formatado antes do único RED funcional.
- O dry-run final de resselo informou `Nothing to fix` com B ainda antigo.
  Foi corrigida somente a metadata B calculada; o revisor recalculou A/B
  independentemente. A limitação do reparador externo permanece aberta.

O baseline protege 3.910 arquivos produtivos fora do par proprietário,
5.203 diagnósticos antecedentes e 499 temporários anteriores; o inventário
produtivo total continua com 3.912 paths. As verificações de integridade não
são métrica de paridade. Artefatos adversariais novos permanecem em
`/tmp/p1338-attacks-d1dqn71s`; o target do produto é exclusivo do P1338.

## Decisão delimitada

Os resultados sustentam fechar o erro Array ausente neste recorte, não a
superfície Array inteira. O parecer independente é publicado separadamente
em `p1338-review-final.json`; `p1338-close.py` só emite `p1338-closure.json`
após verificar esse parecer, seus pins e os gates. O relatório não substitui
o veredito, e a conferência posterior usa recibo novo sem reescrever a cadeia.

Sem staging, commit, push ou limpeza destrutiva. Preservadas alterações e
evidências anteriores. As dívidas listadas acima exigem nova medição e L0
antes de qualquer próximo reparo; não foram silenciosamente incluídas aqui.
