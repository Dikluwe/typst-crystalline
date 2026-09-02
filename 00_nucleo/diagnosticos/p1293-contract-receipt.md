# P1293 — contrato candidato canônico A–D

## Estado

```text
status: reopened-after-lot-b-independent-red
seal: invalidated-a1eba37c94e70b7f8e9576c18f83a9f354db402932a627a32202a2d5eeeb229f
oracle: materialized
discriminatory-gate: replacement-required-after-three-l0-lineage-reseal-and-independent-red-revalidation
```

Este artefato conserva o mesmo contrato observável entregue ao autor
independente do oráculo e registra os resultados posteriores. Não é selo nem
veredito de implementação. A revisão causal do owner exige resselo de lineage
e repetição do gate antes de um verificador separado emitir o selo.

## Autoridade e causalidade

- papel: `autor_contrato_p1293`, sequência causal 2;
- regime: materialização Tekt completa, segregada por capacidades e artefatos,
  sem isolamento técnico de leitura;
- gate humano P1293: confirmado em `2026-09-01T13:51:42-03:00`;
- contrato congelado em `2026-09-01T13:55:40-03:00`;
- gate discriminatório recebido em `2026-09-01T14:35:43-03:00`;
- atualização materializada pré-resselo concluída em
  `2026-09-01T14:42:51-03:00`;
- esta autoridade não leu implementação/teste candidato e não escreveu produto,
  teste, ataque, selo ou veredito;
- próxima ação: coordenador ressella somente os três consumers cujos L0s
  foram corrigidos; novo selo pode autorizar os REDs de morfologia/diagnóstico
  e o diagnóstico bilateral de layout, mas não o lote C.

## Inputs e hashes

| Input | SHA-256 / identidade |
|---|---|
| manifesto pós-gate | `db84b936b2a158b1d97b2542c7d1291b302b156df788c94b93babe8633d0cd43` |
| manifesto atual pré-selo | `290c183c16e451fff9bfb16ba2a51b4e69ad5d7885f1f4c952062f997c48703b` |
| baseline independente | `682cad4bbef22ed6364fd0a5fcb9a8994230e7d22eb716a631f2f2fba6a294b2` |
| medição vanilla independente | `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7` |
| recibo L0 pré-gate | `5f9c38512416a9d6e2c5924207836367f195f3a2ce5f2538f08bb0b33050f15f` |
| owner candidato recebido pelo oráculo | `567ed112b99f897b09f3ed1a4bd7da89391d57f21bff4c8d858ed98b43c5d455` |
| owner materializado após registro causal | raw SHA-256 `6e3355f87a69366fab64bd0160c8e4f49f6c4b5fb95e82e82991fbef199b779f`; hash canônico L0 `fc7f79aa`; `Hash do Código` `3a8a3c3e` |
| consumer/oráculo protegido | `9369fce1e216e7dd5b2845a79bfb3cddc2505a3da196efde43fc01543892606a` |
| recibo RED independente | `84a3c31ae1a12df159015b1c243cfe16011d9534a8e76c5c1f6b14dac2866917` |
| recibo discriminatório pré-selo | `05244f748002e7485f5f22554696158d6c9771394c7fce47ac6da5325875b4bb` |
| HEAD / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| vanilla ratificado | `a51e02804`; binário `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| cristalino baseline | binário `ed5f85e03e6fe473c1a7a9a42cceafe8c5fbe075de4dad56a7c88c07ad2c3b6f` |
| probes brutos | `7f4ad7928dfa1be88cf51da141b8a9dbb01330e46eda4c468acdbb19d1a9c36f` |
| harness de medição | `3f4a4e96ddd1fff309de1aa6bbd02e8355193ab9ad0a4e2673c3aff97c861ab9` |

L0s de produto já ressellados, recebidos como somente leitura:

| Lote | L0 | SHA-256 pós-gate |
|---|---|---|
| A | `compiler/stdlib/foundations/float.md` | `b07ba6be955e28f01e9e12f3f41d3977e5728c13942b381c13428375aff3b57e` |
| B | `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` |
| B | `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| B | `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| B | `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| C | `compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| D | `compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |

## Resultado e política de `Unknown`

O oráculo devolve exatamente um dos estados:

- `Preserved`: caso positivo válido coincide no observável congelado;
- `Violated { witness }`: caso negativo/mutação diverge e registra transcript;
- `Unknown { cause }`: identidade/construção deliberadamente opaca, parser sem
  suporte, timeout, crash ou ferramenta ausente.

`Unknown` nunca é sucesso. Só os quatro IDs `A-O01`, `B-O01`, `C-O01` e
`D-O01` esperam opacidade. `Unknown` em qualquer `P-*`, `N-*`, mutação, controle
ou transporte bloqueia selo e lote. Denominador zero também bloqueia.

## Política canônica de comparação

| Classe | Comparação permitida | Excluído |
|---|---|---|
| presença/identidade | tipo e nome público exatos | símbolo/endereço Rust |
| booleans | JSON tipado; boolean não equivale a string | stdout sem parse de tipo |
| morfologia | `repr` raw exato, inclusive named explicitamente presente | `PartialEq` Rust |
| erros | severidade, mensagem e intervalo do span | ANSI/path temporário |
| layout | viewBox/dimensões e primitives semânticas, tolerância `0.01pt` | bytes SVG, glyph/path IDs |
| HTML | DOM ordenado, attrs/valores, escaping, nesting e void | bytes completos/whitespace mecânico |
| D fora das 15 | transcript cristalino baseline | convergência oportunista ao vanilla |

Cada suíte roda forward e reverse. Ordem de execução não pode alterar o
transcript normalizado. O vanilla é verificado pelo hash antes de qualquer
comparação; mismatch é falha, não fallback.

## Contrato A

### Positivos → `Preserved`

| ID | Observável |
|---|---|
| `A-P01` | `float.is-nan` é function de nome `is-nan` |
| `A-P02` | estática: NaN true; Float finito, Int e `+/-inf` false |
| `A-P03` | ligada em Float: NaN true, finito/inf false; mesma nativa observável |

### Negativos → erro preservado / mutação `Violated`

`A-N01..06`: missing self; positional extra; named ligado; string; receiver Int;
acesso ligado sem chamada. Mensagem e span seguem a medição pinada.

### Opaco → `Unknown`

`A-O01`: bit pattern, sinal e quiet/signaling bit de NaN.

## Contrato B

### Positivos → `Preserved`

| ID | Observável |
|---|---|
| `B-P01` | funções curtas `attach`, `binom`, `mono`, `script` |
| `B-P02` | attach preserva base e os seis slots em ordem |
| `B-P03` | `t:none` permanece morfologia presente; layout equivale à omissão |
| `B-P04` | binom exige lower, preserva ordem/vírgula, não tem barra e usa parênteses extensíveis |
| `B-P05` | mono/script reutilizam estilo; script default true e false observável |
| `B-P06` | sintaxe e qualificada convergem nos quatro membros |
| `B-P07` | oito viewBoxes medidos preservados dentro de `0.01pt` |

ViewBoxes B-P07: `23.2705x19.4843`, `19.7681x9.6041`,
`46.797666667x24.057`, `30.3325x7.8815`, `25.029888889x8.921`,
`14.0987x6.2447`, `8.3578x5.9708`, `8.3578x6.5406` para, respectivamente,
attach display/inline, binom display/inline, mono display/em script e script
cramped true/false.

### Negativos

- attach: base ausente, extra, unknown named, Int em base/slot;
- binom: zero, só upper, Int Content, unknown named e upper named sem lower;
- mono/script: missing, extra, body named, Int; script também segundo positional,
  unknown named e cramped não bool.

### Opaco

`B-O01`: bytes SVG, IDs e path data.

## Contrato C

### Positivos → `Preserved`

| ID | Observável |
|---|---|
| `C-P01` | sete funções curtas |
| `C-P02` | cinco bodies opcionais `None`; `col`/`wbr` void `Unset` |
| `C-P03` | 76 globais; representantes `id,class,hidden` |
| `C-P04` | 48 específicos cobertos por classe e domínio |
| `C-P05` | Presence true vazio/false omitido; ordem após omissões |
| `C-P06` | DOM all-seven/order/void/escaping/nesting; sem end tag void |
| `C-P07` | feature off rejeita paged/HTML; on aceita ambos; target não liga feature |

Os 48 são fixados por tag: button 14, col 1, iframe 10, select 7, template 5,
video 11, wbr 0. Tipos, enums/listas e domínios são os enumerados integralmente
no owner L0. Cada classe recebe ao menos um válido e um inválido; todos os 48
têm presença nominal. Presence false e attrs omitidos não aparecem no DOM.

### Negativos

Unknown named, `data-*`, attr específico de outra tag, tipo inválido, enum/lista
fora do conjunto, inteiro negativo e zero onde positivo são rejeitados com
span. `col`/`wbr` rejeitam body. Nenhum erro pode ser convertido em omissão.

### Opaco

`C-O01`: browser, CSS, mídia/rede e acessibilidade além do DOM serializado.

## Contrato D

### Positivos → `Preserved`

| ID | Observável |
|---|---|
| `D-P01` | dez nomes curtos `cell/header/footer/hline/vline` nos dois namespaces |
| `D-P02` | direta e `.with` preservam Content/payload baseline |
| `D-P03` | seis aliases flat mantidos; hline/vline flat continuam ausentes |
| `D-P04` | aridade/diagnóstico cristalinos divergentes fora das 15 permanecem |

P1293 não reivindica paridade da divergência D-P04. Corrigir nomes não pode
alterar function pointer observável por call, payload, `.with`, alias ou erro.

### Negativos

Underscore namespaced remanescente, nome grid em table, alias flat renomeado ou
criado, `.with` perdido, payload/call alterado ou relaxamento de D-P04 é
`Violated`.

### Opaco

`D-O01`: endereço interno do function pointer. Chamabilidade não é opaca.

## Casos positivos, negativos e opacos — contagem congelada

| Classe | IDs obrigatórios | Resultado esperado |
|---|---|---|
| positivos | A 3, B 7, C 7, D 4 | `Preserved` |
| negativos | A 6; B 14 classes; C 7 classes por todas as tags; D 6 classes | diagnóstico preservado ou mutação `Violated` |
| opacos | `A-O01,B-O01,C-O01,D-O01` | `Unknown`, sem crédito |

As contagens de classes não substituem expansão nominal: os dez siblings D e
os 48 attrs C são obrigatórios mesmo quando compartilham parametrização.

## Matriz mínima de mutações congelada

| Lote | IDs | Obrigações rejeitadas |
|---|---|---|
| A | `MA1..MA5` | sempre falso; inf=NaN; ligada divergente; named ignorado; repr qualificado |
| B | `MB1..MB7` | slot trocado; none perdido/textual; barra; lower fora de ordem; syntax divergente; wrapper style; cramped ignorado |
| C | `MC1..MC9` | named livre; Presence false; body void; body omitido Unset; attr cruzado; enum aberta; target liga feature; end tag void; ordem/escaping |
| D | `MD1..MD5` | só 3 corrigidos; alias flat curto; nome grid em table; call muda; sibling underscore |

Gate exigido: `26/26 = 1.0`. Cada mutação deve ter witness identificado no
owner L0. Mutante equivalente sai do denominador somente após decisão do
adversário aceita pelo adjudicador antes do cálculo. Sobrevivente ou `Unknown`
em mutação válida impede selo.

## Gates desta autoria

Antes de criar o owner: `crystalline-lint --checks v5,v15,v26 --fail-on warning .`
e `git diff --check` terminaram exit 0. Depois de criar o owner:

- `git diff --check`: exit 0;
- `crystalline-lint --checks v15,v26 --fail-on warning .`: exit 0;
- `crystalline-lint --checks v5,v15,v26 --fail-on warning .`: exit 0;
- `crystalline-lint --fix-hashes --dry-run .`: exit 0, `Nothing to fix`.

Esses checks antecederam a criação do consumer. O estado transitório terminou:
o owner possui agora exatamente o consumer protegido registrado abaixo e não
autoriza segundo consumer. A única pendência de lineage é atualizar o
`@prompt-hash` desse consumer para o novo hash do L0; qualquer V15/V26 adicional
é blocker.

### Gate discriminatório recebido

O oráculo protegido foi materializado pelo testador independente com SHA-256
`9369fce1e216e7dd5b2845a79bfb3cddc2505a3da196efde43fc01543892606a` e
lineage para o owner candidato `567ed112...`. O RED independente tem SHA-256
`84a3c31ae1a12df159015b1c243cfe16011d9534a8e76c5c1f6b14dac2866917`.

O verificador discriminatório independente repetiu baseline `9/9` duas vezes
e executou 52 runs: 26 mutações forward e 26 reverse. Resultado: 26 válidas,
26 rejeitadas, score `1.0`, zero equivalentes, zero survivors, zero `Unknown`
e ordem estável. Recibo SHA-256
`05244f748002e7485f5f22554696158d6c9771394c7fce47ac6da5325875b4bb`.

A presente atualização do owner muda apenas estado, ownership materializado e
proveniência; o hash da secção canônica de comparação/lots/mutações permanece
`df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`,
idêntico ao anterior. Mesmo assim, o `@prompt-hash 567ed112` do consumer ficou
stale. O testador independente deve atualizá-lo e o gate deve ser revalidado;
esta autoridade não edita o teste.

## Condição para selo futuro

O consumer e o gate discriminatório existem e satisfizeram o contrato, mas a
revisão de metadata causal do L0 exige novo `@prompt-hash` e revalidação
byte-identificada. Só depois dessa repetição e de V5/V15/V26 verdes um
verificador separado pode criar `p1293-contract-seal.json`. Até lá:
**ready-for-seal-after-lineage-reseal**.

## Reabertura pós-selo — P1293 lote A / owner gap de spans

### Estado causal

O selo de contrato anterior, SHA-256
`5996cd712dd33def5873e207a0fe3067b9f6c701c12398ca09bd67aa81619402`,
fica **explicitamente invalidado** por mudança posterior de inputs L0. Não é
apagado nem reescrito: permanece recibo histórico do contrato que existia
antes da descoberta do owner gap. O gate discriminatório
`bb264d7a54850385b70f0a6d66643f5f081bf32d9ac6586b49c365c0eb2a352b`
também não pode ser reutilizado como gate do novo lineage.

Esta reabertura não altera o núcleo canônico A–D, os 26 mutantes, a política de
comparação nem `Unknown`. O owner do oráculo já exige severidade, mensagem,
source e início/fim de span exatos; os cinco REDs novos exercem precisamente
essa obrigação. Portanto o hash conceitual de lots/comparison/mutações
permanece `df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`
e o oráculo protegido não precisa nem pode ser alterado por esta autoria. Se a
revalidação mostrar que o oráculo não discrimina os cinco ranges, isso refuta
esta conclusão e bloqueia a cadeia antes de qualquer adaptação do contrato ou
oráculo.

### Medição anterior à decisão

O recibo segregado `p1293-implementation-receipt-a.md`, SHA-256
`14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`,
registra medição em `2026-09-01T15:13:15-03:00`, HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, working tree não commitida.
Quatro positivos de presença/identidade/valor continuam GREEN e cinco testes
end-to-end ficam RED somente nos spans; mensagens coincidem:

| Família | Candidato | Esperado |
|---|---:|---:|
| static missing | `12..14` | `0..14` |
| static extra | `12..22` | `18..21` |
| bound named | `19..32` | `20..31` |
| static cast | `12..17` | `13..16` |
| bound sem chamada | `0..19` | `13..19` |

Medição causal adicional, sem abrir o patch `float.rs`: o consumer baseline
`call_dispatch.rs:899-909,964-985,1213-1240` possui a AST individual antes de
`eval_args` e o precedente `CollectionCallSpans`; o consumer baseline
`field_access.rs:106,441-446` ancora o erro no acesso total, enquanto
`field_access.rs:99-101` já demonstra `access.field().span()`. Os headers
medidos mantêm ownership 1:1:

- `compiler/eval/call_dispatch.md` →
  `01_core/src/compiler/eval/call_dispatch.rs`;
- `compiler/eval/bindings/field_access.md` →
  `01_core/src/compiler/eval/bindings/field_access.rs`;
- `compiler/stdlib/foundations/float.md` →
  `01_core/src/compiler/stdlib/foundations/float.rs`.

### Decisão e limites

Spans de erro são observáveis da linguagem sob ADR-0107. Como a obrigação já
estava selada, a correção é glue interno de paridade em fluxo contínuo
ADR-0127, não novo contrato público/default/fase. Três owners foram atualizados
antes de código:

| Owner L0 | SHA-256 desta reabertura | Autoridade exclusiva |
|---|---|---|
| `compiler/eval/call_dispatch.md` | `12a5ead390651556e64353e636e2e5d8a64cc8466a789a14cd97e7770dd5e58b` | recolher call/positional/named de `is-nan` antes de `Args` e selecionar as quatro âncoras |
| `compiler/eval/bindings/field_access.md` | `5c406bb155dee43a1dafa247952b12bb56121eccf9f49f8724175f45a8ce7b2b` | usar `access.field().span()` somente no acesso `Value::Float.is-nan` sem chamada |
| `compiler/stdlib/foundations/float.md` | `0cb585699c80ab7d8f8c9bdab0bb0a00d1f3919e895d5db4aa8896e2afb4d6bd` | conservar fórmula/mensagens e consumir a âncora interna já escolhida |

É proibido mudar `entities::Args`, campo/entidade/trait/assinatura Rust
pública, default, fase eval/layout, valores, nomes ou mensagens; nenhum owner
1:N foi criado. O coordenador deve agora fazer somente o resselo mecânico dos
três consumers, repetir V5/V15/V26, executar os cinco REDs até GREEN e pedir ao
testador/verificador independente novo gate do mesmo oráculo. Somente depois
um novo selo pode substituir o histórico invalidado.

## Reabertura serial — P1293 lote B / owner gap de morfologia

### Estado causal

O selo serial do lote B, SHA-256
`3f436a01616b580b7412e35b33071e73305446d7e48f10dc2a00cd2d80501c5c`,
fica **formalmente invalidado** pela descoberta de um owner produtivo omitido
da sua allowlist. Ele permanece histórico, assim como o checkpoint A
`APPROVED`; nenhum dos dois autoriza escrita B no novo lineage. O recibo de
implementação B, SHA-256
`ae74fa392fcb1b3b93d209347ba211bc207da72fb0ed6a0f1d14c1584937d8f1`,
registra `BLOCKED_OWNER_GAP`: nenhum código produtivo B foi escrito.

Esta reabertura não altera o contrato canônico SHA-256
`2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`,
o oráculo protegido SHA-256
`6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`,
os 26 mutantes, lots/comparison ou a política `Unknown`. `B-P02..B-P06` e
`MB1..MB5` já exigem a morfologia exata, `none` presente, lowers em ordem,
vírgula singleton e convergência sintaxe/qualificada. Alterar o contrato ou o
oráculo para acomodar o owner gap inverteria a causalidade.

### Medição anterior à decisão

No estado não commitado de
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507`, às
`2026-09-01T15:51:21-03:00`, os cinco REDs próprios do lote B mediram
`0 passed / 5 failed / 5375 filtered`, exit `101`, somente por bindings
ausentes. O consumer `structural/math.rs` com esses REDs tinha SHA-256
`07b26199e1fe03f4fcab51591c1ded7da45f462156fcbb8c08bc983dbb446a88`;
`repr.rs` permanecia baseline SHA-256
`6582446811b7469bb5e632d1c4f130417d27ad30388138a22027ee3add083eb4`.

Em `repr.rs:585-619`, a projeção vigente ignorava `MathFrac.line`, escrevia
`MathAttach` como scripts e tratava `MathDelimited` genericamente. O baseline
`HEAD:eval/math.rs:963-1002`, porém, conserva forma estrutural suficiente:
lowers em posições pares de `MathSequence`, separadores exatos
`MathText(", ")` nas ímpares, `MathFrac(line:false)` e parênteses; attach
distingue `None` de `Some(Content::Empty)` nos sete campos existentes.

### Decisão e limites

O owner 1:1 `00_nucleo/prompts/compiler/eval/repr.md` foi atualizado primeiro,
raw SHA-256
`60a57b2c6a76b7a326576387e076551e37032b6aeecd904f23ea93a1f2c22cb5`,
para legitimar exclusivamente:

- `attach(base: ...)` com slots presentes em ordem
  `t,b,tl,bl,tr,br`, `Some(Content::Empty) -> none` e `None` omitido;
- `binom(upper: ..., lower: (...,))` somente para o envelope estrutural
  fechado de parênteses + fração sem linha + sequência lower alternada;
- convergência de sintaxe e chamada qualificada pelo mesmo payload, sem
  reconhecimento do nome da função ou das testemunhas.

O dry-run de lineage encontrou exatamente um drift esperado no consumer
`01_core/src/compiler/eval/repr.rs`: `old=643e33d3`,
`hash-a=7c714164`, `hash-b=c024746c`. V15 e V26 permanecem verdes antes do
resselo. Nenhum outro owner P1293 muda; campo/entidade/payload/API pública,
`Args`, casts, default, fase eval/layout, render e compatibilidade ficam
proibidos. A classificação é morfologia de linguagem, ADR-0107, e correção
interna contínua, ADR-0127.

Próxima ação: o coordenador aplica somente o resselo mecânico desse consumer,
revalida V5/V15/V26 e solicita novo gate/selo serial. Só esse selo substituto
pode reautorizar o lote B; lote C continua bloqueado.

## Segunda reabertura serial B — owner gap de spans math

### Estado causal

O selo serial B substituto SHA-256
`6370e740cd0e53e5e56c9de87afff32730cdddc64287b907e4830642522fc182`
fica **formalmente invalidado**. Ele autorizava os owners de semântica,
morfologia e layout, mas não os dois consumers que ainda possuem/consomem as
âncoras diagnósticas. O selo e seus predecessores permanecem históricos e
não autorizam nova escrita até lineage/gate/selo substitutos.

O contrato canônico SHA-256
`2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`,
o oráculo protegido SHA-256
`6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`,
os 26 mutantes, lots/comparison e `Unknown` não mudam. A política canônica
de erros já compara severidade, mensagem, source e intervalo do span, e os
negativos B já exigem missing/extra/cast/named. Alterar contrato ou oráculo
para aceitar span agregado/detached inverteria a causalidade.

### Medição anterior à decisão

O recibo B SHA-256
`7ebff1e223ceebe789224ed18f2c0518990aae787d7f3e48c3f96bbfb3d792dd`,
medido em `2026-09-01T16:38:33-03:00`, registra `10 passed / 0 failed /
5375 filtered` para semântica, morfologia e layout. A aprovação foi
corretamente bloqueada pelos negativos black-box:

- `math.attach(1)`: mensagem igual, candidato `1:11`, vanilla valor `1:12`;
- `math.attach([x], t: 1)`: mensagem igual, candidato agregado `1:11`,
  vanilla valor `1:20`;
- `math.mono(1)`: mensagem igual, candidato sem source/range por
  `Span::detached()`, vanilla `1:10`;
- `math.script`: mesma causa no helper compartilhado.

Em medição direta de `2026-09-01T16:44:26-03:00`, o consumer aprovado
`call_dispatch.rs` SHA-256
`ceb021c1f9ec0edc5480005e61dc0cb24dcfd8f2ed0efc6e4c8c1499afeb54d5`
já preserva AST individual no precedente P1293-A em `:60-113,953-960,
1274-1303`. O consumer baseline `math_style.rs` SHA-256
`439729b339dadd12cce1c7d7223c7b7aa6306ff2273cac4c645f21c378d651a4`
usa `Span::detached()` nos cinco ramos diagnósticos em `:32-101`, enquanto
`native_mono`/`native_script` já passam identidades fechadas em `:181-217`.

### Decisão L0 e limites

Dois owners 1:1 foram atualizados antes de qualquer escrita nos seus
consumers:

| Owner L0 | raw SHA-256 | Autoridade exclusiva |
|---|---|---|
| `compiler/eval/call_dispatch.md` | `bc923d906d7c11c88e1fa420962a772b299d9d4ef92bfa7ce559cf8c65aa6340` | capturar call/positional/named completo/named valor e escolher `args.span` somente para as identidades nativas resolvidas attach/binom/mono/script |
| `compiler/stdlib/math_style.md` | `9cdfeaf9160fde11ce470c68344b4fad31dc404cbe2c6e468633ab7259223180` | consumir o `args.span` recebido somente nos diagnósticos mono/script; as outras 12 funções mantêm detached |

Mensagens, precedência, ordem/quantidade de avaliações, constructors,
valores, morfologia, layout e defaults permanecem intactos. `entities::Args`,
API pública, campos, entidades, traits, compatibilidade e fase eval/layout
não mudam. A classificação é observável de linguagem ADR-0107 e correção
interna contínua ADR-0127; não há novo gate humano.

V15 e V26 passam. O dry-run encontra exatamente dois drifts esperados:

```text
call_dispatch.rs old=548a37bb hash-a=6cdb3f22 hash-b=eaca4b89
math_style.rs    old=58661c6c hash-a=a7d71b3a hash-b=258e9e0e
```

O coordenador deve ressellar mecanicamente somente esses dois consumers,
revalidar V5/V15/V26/diff e obter gate/selo serial substitutos antes de o
implementador retomar os spans B. O lote C continua proibido. Se surgir outro
owner ou mudança de contrato público, a cadeia para.

## Terceira reabertura serial B — regressão P1105 obsoleta

### Estado causal

O selo serial B SHA-256
`476513cc1d324c4319609fc70f9237d12e0e74af08ce920980a356a820e1df5c`
fica **formalmente invalidado** por um owner test-only omitido da sua lineage.
O produto do lote B não é revertido: o recibo de implementação B SHA-256
`3772126a3880a174c588dd62897e77e89bbf955fdd917dcd6218215132044eef`
registra 20/20 spans negativos coincidentes, suites math/repr/call verdes e
uma única falha em expectativa regressiva histórica.

O contrato canônico SHA-256
`2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`,
o oráculo protegido SHA-256
`6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`,
o recibo RED SHA-256
`91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e`,
os 26 mutantes, lots/comparison e a política `Unknown` permanecem
byte-conceitualmente inalterados. As mensagens exatas já pertenciam aos
negativos B; alterar produto, contrato ou oráculo para conservar as frases
portuguesas obsoletas inverteria a causalidade.

### Medição anterior à decisão

No estado não commitado de
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507`, medido em
`2026-09-01T17:14:34-03:00`, o filtro `p1105` terminou
`5 passed / 1 failed / 5379 filtered`, exit `101`; a única falha foi
`p1105_attach_zero_ou_multiplos_args_posicionais_erro`. Para zero argumentos,
o teste exige `attach espera exactamente 1 argumento, recebeu 0` em vez de
`missing argument: base`; para excesso, exige a frase histórica com contagem
em vez de `unexpected argument`. Os SHA-256 de proveniência são
`e7ce2e33d6683603012059deaf3e8254dbedee2a93c9b25663cf779b097df55a`
para `git status --short` e
`26d25cac81995d45a9c667fa181ba17259eb9f65327117d1fbaeffa0d1435b7e`
para `git diff HEAD --stat`.

O owner vigente `compiler/eval/tests.md:10-18,58-62` era genérico e não
continha P1293; o header medido de `01_core/src/compiler/eval/tests.rs:2-3`
aponta exclusivamente para ele (`@prompt-hash 2c87f3cf`). O consumer test-only
tem SHA-256
`7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`
e não foi aberto além do header nem editado por esta autoria.

### Decisão L0 e limites

Somente o owner 1:1 `00_nucleo/prompts/compiler/eval/tests.md` foi atualizado
primeiro, raw SHA-256
`fe2a82590d703db2c6ea4e984e3b3ea7dc594199415627822b5b2836da18046c`.
Ele exige substituir apenas as duas expectativas P1105 por
`missing argument: base` e `unexpected argument`, mantendo obrigatórios os
outros cinco controles P1105 e todas as demais asserções. Não aceita as duas
mensagens antigas como alternativa nem autoriza fallback no produto.

Mensagens diagnósticas são observáveis da linguagem (ADR-0107); retificar o
teste obsoleto é fluxo contínuo interno test-only (ADR-0127), sem novo gate
humano. API/campo/entidade/trait/assinatura pública, defaults, compatibilidade,
fase eval/layout, ordem de validação, spans, semântica, morfologia e layout
não mudam.

V15 e V26 passam. O dry-run encontra exatamente o drift esperado, sem escrita:

```text
eval/tests.rs old=2c87f3cf hash-a=97d4926d hash-b=5c55b251
```

O coordenador deve aplicar somente esse resselo mecânico; depois o papel
autorizado para testes retifica apenas as duas expectativas, confirma
`6/6` P1105 sem regressão dos controles, repete V5/V15/V26/diff e solicita
revalidação/selo serial substituto. Produto e lote C permanecem bloqueados.

## Quarta reabertura serial B — julgamento independente RED

### Estado causal

O selo serial test-only SHA-256
`a1eba37c94e70b7f8e9576c18f83a9f354db402932a627a32202a2d5eeeb229f`
fica **formalmente invalidado** pelo julgamento independente posterior. Ele
explicitamente não aprovava o candidato; portanto a rejeição não contradiz o
selo, mas impede usá-lo como autoridade de nova escrita.

O contrato canônico SHA-256
`2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`,
o oráculo protegido SHA-256
`6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`,
o RED SHA-256
`91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e`,
os 26 mutantes, lots/comparison e a política `Unknown` não mudam. O julgamento
exerceu obrigações já congeladas em B-P02/B-P04/B-P05/B-P06/B-P07 e nos
negativos B; adaptar contrato/oráculo ao candidato inverteria a causalidade.

### Medição anterior à decisão

O recibo final do implementador, SHA-256
`d677c0b1e6d8796c6680787d27b3409c100ff13653ab8ff89d7154813866720c`,
registra em `:5-10` evidência local sem aprovação, em `:90-94` a alegação
de convergência e em `:169-174` a obrigação de julgamento independente. O
julgamento independente RED recebido pelo coordenador refutou a conclusão
local nos seguintes observáveis públicos:

1. `MathStyled` de mono/script projetava-se como `[x]`, não como
   `styled(child: [x], ..)`;
2. folhas da sintaxe attach/binom/mono/script projetavam `x`/`T`/equivalentes,
   enquanto a forma qualificada projetava `[x]`/`[T]`/equivalentes;
3. `binom(upper: ...)` perdia o hint `try removing upper:`; `body:` de
   mono/script emitia `unexpected argument: body` em vez de
   `the argument body is positional` mais `try removing body:`;
4. quatro dos oito vetores B-P07 permaneciam fora da tolerância de layout.

O recibo vanilla público SHA-256
`39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`
mede a morfologia/convergência em `:164-204` e enumera os oito viewBoxes
B-P07 em `:206-217`. O julgamento não localizou causalmente quais owners e
fórmulas produzem os quatro REDs de layout. Logo o número quatro é evidência
de bloqueio, não autoridade para editar um L0 de geometria.

### Decisão L0 e limites

Três owners 1:1 foram atualizados primeiro, sem abrir patch candidato ou
oráculo fonte:

| Owner L0 | raw SHA-256 | Autoridade exclusiva |
|---|---|---|
| `compiler/eval/repr.md` | `2c3b10e788d85fea2e455fc217f00d47d09d7d3c92d9eb34265bae84a230c0f8` | conservar wrapper `MathStyled` mono/script e projetar folhas diretas somente nos campos P1293 de attach/binom/styled pelo formatter canônico |
| `compiler/stdlib/structural/math.md` | `9a44293c7ee7a44795f0864c1b6a9f14f98074995caea2ec932cab328d0e258d` | mensagens/hints exatos para campos posicionais reconhecidos de attach/binom |
| `compiler/stdlib/math_style.md` | `1d7236f411376dc3515214e4ea51d2df40af0895199e55d410bbd8a1e0318831` | `body:` de mono/script como posicional com hint, mantendo as outras doze funções fora do escopo |

As seleções usam somente variantes/campos/identidades já existentes. É
proibido novo payload/provenance bit, heurística por texto/testemunha/origem,
mudança de API/`Args`/entidade/default/compatibilidade/fase, ou alteração de
precedência/spans fora do explicitamente medido.

Morfologia e transcript diagnóstico são observáveis da linguagem (ADR-0107).
As três correções são paridade interna em fluxo contínuo (ADR-0127), sem
novo gate humano. V15 e V26 passam. O dry-run encontra exatamente:

```text
repr.rs            old=7c714164 hash-a=4c04b417 hash-b=545f0ea2
math_style.rs      old=a7d71b3a hash-a=a36c31b4 hash-b=56b9e300
structural/math.rs old=73443b4c hash-a=ff23cbe8 hash-b=ce079105
```

Nenhum L0 de layout muda nesta reabertura. Depois de resselo e novo selo
serial, o implementador deve primeiro reproduzir bilateralmente cada um dos
oito B-P07, identificar nominalmente os quatro REDs e medir `file:line` da
causa antes de propor qualquer owner/fórmula de layout. Se os L0s vigentes não
legitimarem a correção causal, deve parar e reabrir novamente. Lote C, novo
selo, produto, testes, ataques e veredito permanecem proibidos nesta autoria.
