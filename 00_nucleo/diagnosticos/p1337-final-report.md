# P1337 — Bool, None e Auto: diagnóstico de campo corrigido

## Resultado e o que continua faltando

O acesso a campo ausente de um valor booleano agora informa
`cannot access fields on type boolean`, em vez de `bool`. Em Bool, None e
Auto, o sublinhado abrange somente o identificador do campo. None/Auto já
tinham os nomes corretos; neles foi corrigida apenas a origem do diagnóstico.
O lookup puro continua respeitando o span recebido. Não foram criados campos
ou métodos, nem alterados valores válidos, `type()` ou `repr()`.

A mudança produtiva está em somente dois pontos de
`01_core/src/compiler/eval/bindings/field_access.rs`: a seleção de span AST
inclui as três categorias e o ramo de nome canônico existente inclui Bool.
L0 foi atualizado antes de código; três expectativas legadas expressamente
sucedidas foram migradas e o teste foi renomeado. Os demais testes anteriores
permanecem byte a byte. A classificação é correção interna de paridade,
ADR-0127 em fluxo contínuo, sem novo contrato público/default/fase.

Continuam abertas, medidas e preservadas neste recorte:

- Array: `(1,2).nope` ainda tem mensagem e sublinhado diferentes do vanilla.
  Possui ramo próprio; não foi generalizado o reparo a ele.
- Valores-tipo, como `bool.nope`, não são instâncias Bool e têm dívida própria.
- Pré-despacho de métodos: `"abc".len` ainda retorna valor onde o vanilla erra.
- Ordem de avaliação: `true.nope(panic("arg"))`, e os controles None/Auto,
  preservam a divergência anterior ao lookup.
- A dívida de Content do corpus e demais superfícies excluídas não foram
  reabertas. PDF exportado, layout, acessibilidade geral, catálogo de warnings
  e estilos contextuais não receberam nova prova de paridade.

Isto fecha somente o fragmento Bool/None/Auto que chega ao lookup. O inventário
global de paridade não foi reexecutado; não há percentual global novo.

## Evidência executada

Todos os números abaixo têm recibos com UTC, argv, canais completos e estado
da árvore. Estado: working tree **não commitada** sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. `p1337-baseline.json` preserva o
diff/stat e inventário antecedentes; os recibos finais preservam o diff/stat
da medição, incluindo a lista exata dos 16 arquivos tracked alterados. Somente
o par L0/consumer acima mudou produtivamente neste passo; os outros deltas
são herdados e não foram descartados.

| Verificação | Resultado e recibo |
|---|---|
| RED independente antes de C | Falhas reais de assertions; `p1337-unit-red-r1.json`, revisado em `p1337-review-prepatch-go.json`. Transporte exclusivamente mecânico para R2, explicado abaixo. |
| GREEN focal release normal | 5 passaram, 0 falharam; `p1337-unit-green.json`, 19:34:19.907880–19:36:18.952197 UTC em 2026-09-09. |
| Build workspace release --locked | Passou; `p1337-candidate-build.json`, 19:38:39.128179–19:39:22.698325 UTC. |
| Suíte workspace release --locked | 6.743 passaram, 0 falharam, 3 ignorados; soma dos blocos `test result` de `p1337-workspace-tests.json`, 19:39:44.155860–19:42:52.901229 UTC. Inclui o controle LocatedContent antecedente. |
| A/B CLI | 540/540 envelopes satisfizeram as expectativas pré-congeladas; `p1337-tests-candidate.json` e `p1337-tests-comparison.json`, 19:40:04.446853–19:40:22.406447 UTC. |
| Mutação real | Controle C passou; cinco famílias recompiladas foram rejeitadas por assertions, sem Unknown; `p1337-attacks-results.json` e `p1337-attacks-final.json`, recibos individuais C/M1–M5. |
| Formatação e diff | `cargo fmt --all -- --check` e `git diff --check` passaram; `p1337-fmt.json` e `p1337-diff-check.json`. |
| Linhagem estrita | V5/V15/V26 com `--fail-on warning`: zero violations; `p1337-lineage-final.json`. A/B recíprocos calculados em `p1337-lineage-reciprocal.json` e recalculados pelo revisor. |
| Lint geral | Exit 0, zero entradas error; 240 warning e 1.148 info, contadas por prefixo de linha no stdout integral de `p1337-lint-general.json`. Não é ausência de dívida geral. |

O A/B usa 45 expressões, quatro perfis (default/html/a11y/html+a11y) e três
ordens (normal/repetida/inversa). As 540 verificações se dividem em **144
convergências**, **300 preservações de paridade** e **96 preservações de dívida**.
Estas últimas não contam como igualdade com vanilla. Os 1.080 envelopes
bilaterais anteriores a C estão em `p1337-tests-baseline.json`; expectativas
em `p1337-tests-expectations.json`. Comparação de exit/stdout/stderr integral,
sem normalização, sem instabilidade de ordem e sem Unknown obrigatório.
Os controles deliberadamente opacos do harness não são casos do produto.
Sumário efetivo: `p1337-tests-summary-r1.json`.

M1 reintroduziu o nome `bool`; M2/M3/M4 reintroduziram o span agregado em
Bool/None/Auto; M5 extrapolou field-only para Array e foi detectado pela
sentinela que preserva a dívida. Cada execução reteve fonte, patch, logs e
executável próprio com hash; não houve crédito por falha de compilação ou
cache antigo. O perfil instrumental, congelado antes de C, usa somente
`typst-core` com opt-level=0, igualmente no controle C e nos cinco mutantes.
Seus quatro testes locais provam discriminação nesse perfil, não equivalência
com release normal. O GREEN de cinco testes e o workspace acima são a prova
release normal separada. A soma dos tempos Cargo adversariais é 260,1 s,
conforme os seis recibos individuais; não é benchmark do produto.

No lint geral, as 240 entradas warning são as mesmas do P1336 após normalizar
somente coordenadas de linha. Destas, 233 são V16 e sete V17: 233 não é o total
de warnings. Das 1.148 infos, 1.146 permaneceram iguais; as duas diferenças
V19/V20 descrevem justamente o ramo que passou de Int/Str para Bool/Int/Str.
Contagem igual, sozinha, não foi usada como prova de preservação.

## Identidades e limites do processo

Vanilla ratificado: upstream/main **a51e02804**, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Baseline P1336: `/tmp/p1336-target.QiOMGq/release/typst`, SHA-256
`646a8d97400c0abe262a65c9b9559b47a3ecf7d10eafd82fa79a64ade0504497`.
Candidato: `/tmp/p1337-target.F7jPNj/release/typst`, SHA-256
`55b5dc263bba15b050e9caab755c17deb22f617eec35b1e9259a20a57f9b350b`.
Fonte C SHA-256 `fac712d833e2aeeb932605300240f6148edae5aac0225a55b7cc334e35658a64`.
L0 normativo SHA-256 `7ad59e65c0d50b202fd015375bd7f4333688ecf49e558bb66a05cedf46c5df90`;
selos A `0be0e8ef` e B `ee628f5f`. Não houve resync do lab.

A skill `tekt-materializacao-segregada` determinou autoria separada de testes,
ataques e veredito e o congelamento anterior ao patch. Implementador não leu
os novos testes privados antes de C; autor do A/B não leu a fonte C.
Regime **A/B sem atestação técnica de isolamento, sem selo de refinamento**.
As capacidades reais dos processos são mais amplas que as allowlists; não
se confunde divisão de autoria com isolamento técnico demonstrado.

Incidentes e sucessões preservados, sem apagar recibos:

- Limite total de agentes impediu criar novo revisor. Manifesto R1 reutilizou
  somente o contexto anterior de revisão de um agente sem autoria de testes
  ou implementação P1337. Testes e ataques receberam contexto novo.
- Antes de C, a revisão detectou que faltava um discriminador Array sob o
  filtro adversarial. R1 acrescentou essa cobertura e houve novo RED real.
  R2 foi somente formatação canônica, incluindo duas vírgulas opcionais de
  tuplas; o revisor demonstrou identidade após rustfmt e autorizou transportar
  RED-r1. Não se alega uma terceira execução RED com bytes R2. GREEN usa R2.
- A primeira invocação estrita usou flags inválidas e terminou no parser
  (exit 2), em `p1337-lint-strict.json`; não conta como gate. A invocação
  correta está em `p1337-lineage-final.json` e passou.
- O dry-run de resselo final disse `Nothing to fix` apesar de B desatualizado.
  Foi aplicado somente o B calculado do owner e o revisor recalculou A/B
  independentemente. A limitação do reparo automático não foi corrigida nem
  promovida a evidência de ausência de drift.
- `p1337-tests-summary.json` sofreu truncamento ao serializar saída de ferramenta
  e é inválido. Foi preservado; o sucessor compacto `p1337-tests-summary-r1.json`
  aponta para os recibos completos, válidos e inalterados. Corpus, expectativas
  e runner não mudaram depois de C.

## Entrega e decisão delimitada

Os resultados sustentam fechar o diagnóstico Bool/None/Auto neste recorte,
não fields em geral. O veredito independente é publicado separadamente em
`p1337-review-final.json`; `p1337-close.py` só emite `p1337-closure.json` após
validar esse veredito, seus pins, os gates e a preservação de fonte/histórico/
temporários. Auditoria posterior fica em recibo novo do revisor, sem modificar
o fechamento. Este relatório não substitui o veredito.

Sem staging, commit, push ou limpeza destrutiva. Temporários exclusivos em
`/tmp`; antecedentes preservados. O próximo recorte plausível é Array, mas
requer medição e L0 próprios antes de qualquer alteração: não está autorizado
nem implementado por este fechamento.
