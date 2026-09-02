# P1293 — recibo L0 pré-gate humano

## Veredito desta fase

`L0_READY_AWAITING_HUMAN_GATE`.

O contrato A–D foi redigido antes de qualquer código candidato P1293. Esta
autoridade para aqui: não criou selo final, contrato/oráculo executável, teste,
código produtivo, ataque, mutation score ou veredito de implementação. O gate
humano P1293 continua obrigatório e confirma somente estes L0s; confirmações de
P1292 não se transferem.

## Autoridade, regime e limites

- papel: `autor_contrato_p1293`, sequência causal 2;
- regime: protocolo completo da skill `tekt-materializacao-segregada`;
- execução: segregada por capacidades e artefatos, sem isolamento técnico de
  leitura porque o filesystem é compartilhado;
- entradas permitidas: passo, manifesto, baseline/medição independentes, ADRs,
  L0s afetados, consumers baseline e vanilla pinado;
- escritas exercidas: sete owners L0 e este recibo;
- não foi lido patch candidato: nenhum código P1293 existia nesta fase;
- não foram escritos `01_core/**`, `02_shell/**`, `03_infra/**`, `04_wiring/**`,
  testes, oráculos, ataques, selo ou certificado;
- política de `Unknown`: nunca é sucesso; medição recebida com `Unknown = 0`.

## Proveniência congelada

| Entrada | Identidade |
|---|---|
| instante do baseline independente | `2026-09-01T13:24:01-03:00` |
| instante da captura de hashes L0 | `2026-09-01T13:45:21-03:00` |
| instante de fecho pré-gate | `2026-09-01T13:47:42-03:00` |
| `HEAD` / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| árvore | não commitada; baseline exato em `p1293-baseline-status.txt` |
| passo P1293 | SHA-256 `d577a03c27731d11cdcc161a3e93646e90bdb62f0a7f9cfb8d1b4f5980a9dc3a` |
| manifesto atual recebido | SHA-256 `81e17f9904577c50898acc178740512be223d2f918572713984731d18eacb97e` |
| manifesto causal recebido pelo medidor | SHA-256 `1cb2bf59d9294e71bb9e94480386e46151fd08e0facb8bb6327ac3642e428ff7` |
| baseline independente | SHA-256 `682cad4bbef22ed6364fd0a5fcb9a8994230e7d22eb716a631f2f2fba6a294b2` |
| recibo de medição | SHA-256 `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7` |
| vanilla ratificado | `a51e02804`; binário SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| cristalino baseline | binário SHA-256 `ed5f85e03e6fe473c1a7a9a42cceafe8c5fbe075de4dad56a7c88c07ad2c3b6f` |
| diff apenas dos sete L0s | SHA-256 `8d134eaf737a5b67ff15d7b46a84cc4d67575f06b5ad64ac1675732b2851f4a7` |

O manifesto atual difere do hash causal porque o medidor registrou nele os
resultados concluídos. Ambos são preservados para não fingir que a medição
recebeu bytes posteriores.

## Medição antes da decisão

A superfície fresca reproduziu `111 total / 99 MATCH / 12 diferenças` no
perfil default e `115 total / 89 MATCH / 26 diferenças` no HTML. As 15
diferenças acionáveis foram reproduzidas nominalmente: 12 membros ausentes e
3 nomes públicos errados. A medição conteve 203 casos, 624 tentativas de cast,
48 atributos específicos, 5 vetores DOM, 10 SVG e zero `Unknown`.

Fontes que precederam cada decisão:

| Lote | Fonte medida | Decisão produzida |
|---|---|---|
| A | vanilla `foundations/float.rs:12-30,32-39,67-80`; cristalino `compiler/stdlib/foundations/float.rs:18-81` | adicionar `float.is-nan` estático/ligado à arquitetura fechada existente, sem trait ou fórmula duplicada |
| B | vanilla `math/mod.rs:42-92`, `attach.rs:19-49`, `frac.rs:133-150`, `style.rs:134-145,208-227`, `ir/resolve.rs:402-415,470-570,714-768`; cristalino `eval/math.rs:822-929,963-1008`, `math_attach.rs:15-84`, `layout/attach.rs:21-120`, `layout/frac.rs:66-128` | expor quatro funções curtas, convergir syntax/qualificada e reutilizar os sete slots, `MathFrac(line:false)` e as nativas style vigentes |
| C | vanilla `typst-html/src/typed.rs:30-160,162-247`, `tag.rs:123-141`, `convert.rs:165-245`, `encode.rs:111-167`; cristalino `compiler/stdlib/html.rs:23-29,39-237,546-590` | registrar sete constructors no dispatcher estático, com 48 casts fechados, body normal/void e DOM genérico |
| D | vanilla `layout/grid/mod.rs:422-438,574-677,767-768`, `model/table.rs:289-305,494-613,732-733`; cristalino `compiler/eval/mod.rs:1804-1827,2059-2110` | mudar somente a identidade textual das dez instâncias namespaced, preservando pointers, calls e aliases flat |

## Matriz Prompt L0 ↔ consumer baseline

V15 confirma uma relação `1:1` para todos os owners auditados. Não foi criado
owner `1:N`, Núcleo novo ou pin novo.

| Prompt L0 | Consumer único | SHA-256 L0 após auditoria | Ação |
|---|---|---|---|
| `compiler/stdlib/foundations/float.md` | `01_core/src/compiler/stdlib/foundations/float.rs` | `12e130e6a727898cfc175265044acfb7b68d0296cbad99994ad367b315586617` | alterado: lote A |
| `compiler/stdlib/structural/math.md` | `01_core/src/compiler/stdlib/structural/math.rs` | `23dbf2b1eddb31777876dd578c03b606796ab67a65d12278ebf0965c136dd2b6` | alterado: módulo/constructors B |
| `compiler/eval/math.md` | `01_core/src/compiler/eval/math.rs` | `18e83e80c8b6039c64a0f29b8d3b56ba833f8b3544252a2fcd0989f05cfa4430` | alterado: convergência sintática B |
| `entities/elements/math_attach.md` | `01_core/src/entities/elements/math_attach.rs` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` | alterado: corrigido de 5 para 7 campos já existentes |
| `entities/elements/math_frac.md` | `01_core/src/entities/elements/math_frac.rs` | `a9ea29fabacea327e895f78a3494307ddee58629cba25fec8e26be01699a20af` | mantido: já contrata `line:false` para binom |
| `compiler/math/layout/attach.md` | `01_core/src/compiler/math/layout/attach.rs` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` | alterado: seis slots e `none` explícito |
| `compiler/math/layout/frac.md` | `01_core/src/compiler/math/layout/frac.rs` | `9664f7d3320690a96129cd5ec39b405a0a4f499186559907fd27238b4d597003` | mantido: pilha sem barra já contratada |
| `compiler/stdlib/math_style.md` | `01_core/src/compiler/stdlib/math_style.rs` | `fd33cd0c921eec632e122aa475aa28d845a1e6cb4345458bfe68a5e78c53390f` | mantido: `mono`/`script`, casts/defaults já completos |
| `compiler/stdlib/html.md` | `01_core/src/compiler/stdlib/html.rs` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` | alterado: lote C; metadata corrigida para owner único |
| `entities/html.md` | `01_core/src/entities/html.rs` | `4d1c5bbe7005154f3fc01099d6e79b8100d39fa12df467c3c0a3c75a84a0260f` | mantido: `HtmlBody::{Unset,None,Content}` já suficiente |
| `compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` | `9649e150a7a8222ed1e04fc91801a96cab8c7bbec060b87d9e5dcee210a55ba0` | alterado: dez nomes D e divergência preservada |
| `compiler/eval/table.md` | `01_core/src/compiler/eval/table.rs` | `aaa2c9a188f66802732403e290f254641ee9e514d388ee2d424213a8e4990fc0` | mantido: owner do helper de célula, não dos namespaces em `eval/mod.rs` |
| `compiler/stdlib/structural/table_grid.md` | `01_core/src/compiler/stdlib/structural/table_grid.rs` | `153a22b018f247e1b37943a257afa3ad55da414b92886655a1415079118c3edd` | mantido: calls/payloads não mudam |

O owner adicional `compiler/eval/repr.md` foi auditado somente leitura, SHA-256
`8f4f3f874b2c08d8e05278763ea795fe8d125ccb67509ce93ec4f5cfda53880d`.
Seu contrato geral em `:9-14` já exige `repr` determinístico em forma
construtora e morfologia da linguagem; não foi duplicado nem convertido em
owner compartilhado. Se RED posterior provar uma obrigação fora desse contrato,
esse L0 deve ser atualizado antes do respectivo código e o selo reiniciado.

## Hashes dos consumers baseline não alterados

| Consumer dos sete L0s alterados | SHA-256 |
|---|---|
| `compiler/stdlib/foundations/float.rs` | `fcf93a0bfc26a73d2b150d05089a0ff2ae5f2a55fbeb587ac402f196c7aad130` |
| `compiler/stdlib/structural/math.rs` | `6b58286a0e99ae4acd7e2a32156c4da7ad091fdcec163ff6c14ed7c71060f241` |
| `compiler/eval/math.rs` | `e5dde4dfdeec1a07744129b0f5cd59e883f0a2eadca8c29c34a2d7c2b0904a87` |
| `entities/elements/math_attach.rs` | `d015957f20bea85c907cf141a3eb2e4a3f00d2f92bcf094cbbf7c3a6795672aa` |
| `compiler/math/layout/attach.rs` | `9c5b6b3f1d57ee1e5c0d6def9296597c1ce56afb846299cb4769d93682c5c9f1` |
| `compiler/stdlib/html.rs` | `2c9e1b855ec91e5c63da6f9ed7e1e1b47050619101c0ce6da2221ef0ed9adb9c` |
| `compiler/eval/mod.rs` | `f774abf709988864c2b0ae462f52427215fad599cbe8189453f577ce22422c75` |

## Contrato congelado por lote

### A — `float.is-nan`

Função pública curta `is-nan`; chamada estática aceita Float e coerção Int;
receiver ligado aceita somente Float; NaN é verdadeiro e finitos/inteiro/
`+/-inf` são falsos. Aridade, named, tipo e spans são fechados. Obter o método
ligado sem chamada continua ausente. Uma única nativa serve as duas chamadas.

### B — quatro membros math

`math.attach` expõe base e seis slots. `none` explicitamente fornecido é
morfologia presente (`Some(Content::Empty)`/`t:none`) e tem layout equivalente
à omissão; a equivalência de render não autoriza apagar a morfologia.
`math.binom` exige upper e pelo menos um lower, preserva ordem/vírgulas, usa
fração `line:false` e parênteses esticados. `math.mono` e `math.script`
reutilizam as nativas globais; `script.cramped` é bool default true. Sintaxe e
função qualificada convergem sem novo payload ou fase. Erros e spans medidos
são observáveis.

### C — sete constructors HTML

`button`, `col`, `iframe`, `select`, `template`, `video`, `wbr` têm nomes
curtos, 76 globais e exatamente os 48 específicos/casts enumerados no L0.
Cinco tags normais usam body opcional/`HtmlBody::None`; `col` e `wbr` são void,
sem body e com `HtmlBody::Unset`. Unknown named, `data-*` e attr de outra tag
são erros. Presence true é atributo vazio e false é omissão. Ordem, escaping,
nesting e DOM genérico são preservados; void não recebe end tag. Feature HTML
e target são eixos independentes, e target nunca liga feature.

### D — dez nomes públicos curtos

`grid.{cell,header,footer,hline,vline}` e
`table.{cell,header,footer,hline,vline}` imprimem o nome curto. Function
pointers, chamadas diretas/`.with`, payloads e seis aliases flat permanecem.
Fica explicitamente fora das 15 diferenças a divergência preexistente de
aridade/diagnóstico de grid/table header/footer; P1293 a preserva e não a
declara equivalente ao vanilla.

## Inferências e refutadores

| Inferência marcada | Evidência que a refutaria |
|---|---|
| o lote A cabe no match/nativa fechados existentes | necessidade de trait, registry, receiver Int ligado ou fórmula distinta |
| `none` explícito cabe nos sete campos existentes como conteúdo vazio presente | perda de `t:none` no `repr`, materialização de texto/glifo, alteração de layout ou necessidade de oitavo campo |
| binom cabe em `MathFrac(line:false)` + delimitadores vigentes | barra emitida, ordem/vírgula perdida, fase/payload adicional requerido |
| os sete HTML cabem no dispatcher/entidade/exporter genéricos | necessidade de estado novo, browser/CSS/mídia/rede, end tag void ou diferença DOM não expressável |
| D é somente identidade textual | pointer, tipo de conteúdo, call/`.with` ou alias flat alterado; algum sibling ainda com underscore |

Nenhuma dessas inferências é veredito de implementação. Seus refutadores devem
ser testados após o gate por autoridades separadas.

## Classificação ADR-0107/0108/0127/0128/0129

- ADR-0107: presença, casts, identidade, morfologia e DOM são linguagem;
  `PartialEq`, bytes SVG/HTML e passos internos são mecânica, salvo texto/spans
  de diagnóstico quando estes são o observável.
- ADR-0108: todas as decisões acima são posteriores a medições `file:line`;
  inferências e refutadores estão explícitos. Os fingerprints só refutam efeito
  visual, não provam equivalência geral.
- ADR-0127: **PARAGEM OBRIGATÓRIA**. P1293 adiciona 12 membros públicos e corrige
  10 identidades públicas de namespace. Não apareceu campo público Rust novo,
  entidade, default de produto, fase de pipeline ou quebra adicional; mesmo
  assim a superfície pública exige confirmação humana antes do código.
- ADR-0128: feature e target HTML permanecem ortogonais; nenhuma conversão de
  layout paginado, habilitação implícita ou comportamento de browser foi
  autorizado.
- ADR-0129: os 13 pares auditados permanecem `1:1`; o texto histórico
  “Owners candidatos” de `html.md` foi corrigido para seu único owner real.
  Nenhum Núcleo foi criado porque não surgiu claim compartilhada nova que
  justificasse `1:N`.

## Gates executados sem escrever código

| Gate | Comando | Resultado |
|---|---|---|
| ownership/núcleos baseline | `crystalline-lint --checks v15,v26 --fail-on warning .` antes dos edits | exit 0, `No violations found` |
| hash baseline | `crystalline-lint --fix-hashes --dry-run .` antes dos edits | exit 0, `Nothing to fix` |
| diff | `git diff --check` após L0 | exit 0 |
| ownership/núcleos pós-L0 | `crystalline-lint --checks v15,v26 --fail-on warning .` | exit 0, `No violations found` |
| dry-run pós-L0 | `crystalline-lint --fix-hashes --dry-run .` | exit 0; sete updates de header seriam necessários, nenhum aplicado |
| V5 pós-L0 | `crystalline-lint --checks v5 --fail-on warning .` | exit 1 esperado; exatamente sete drifts L0→consumer, porque o gate proíbe ressellar código agora |

O dry-run pós-L0 propôs estes `@prompt-hash`: `float.rs 996763cf`,
`structural/math.rs 73443b4c`, `eval/math.rs 34a81f38`,
`math_attach.rs 45fcff7a`, `layout/attach.rs d0e91016`, `html.rs c7e51513` e
`eval/mod.rs 79a8abaf`. O V5 não é verde nesta fronteira por desenho: torná-lo
verde exigiria escrever os headers dos consumers antes da confirmação humana.
V15/V26 estão verdes e o dry-run prova a lista fechada de sete futuros resselo.

## Paragem

Estado entregue ao dono: sete L0s alterados, seis L0s §5.1 mantidos, matriz
`1:1` íntegra, nenhuma condição de expansão acionada e gate ADR-0127 pendente.
Próxima ação autorizável é a confirmação humana explícita. Antes dela, não
criar selo, RED, oráculos, implementação, ataques ou veredito.
