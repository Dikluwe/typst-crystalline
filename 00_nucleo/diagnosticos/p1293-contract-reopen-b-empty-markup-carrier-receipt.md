# P1293 — recibo de reabertura B: carrier de presença de markup vazio

Estado: `L0_AND_NUCLEUS_AUTHORED_AWAITING_LINEAGE_RESEAL_REPLACEMENT_GATE_AND_SEAL`  
Papel: `autor_contrato_p1293` — autoria de obrigação/L0, segregada do produto e do oráculo  
Autoria: `2026-09-02T08:22:28-03:00` (`America/Sao_Paulo`)

## 1. Entradas congeladas e confirmação

- manifesto recebido: SHA-256
  `d85deb7b60bc69e403df84b1ae0fd6828b289cfdc8fe4c9c705b94b311f67ed4`;
- input de preconfirmação: SHA-256
  `5c615928f69084dccdbbfe02e440363ffd128e23b86c909d9e5dcc5cc8ecfe02`;
- confirmação humana ADR-0127 categoria 1: `2026-09-02T08:06:37-03:00`,
  texto exato `Confirmado`;
- recibo causal `p1293-empty-markup-carrier-measurement-receipt.md`:
  SHA-256 `80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`;
- recibo de implementação B interrompida:
  SHA-256 `8752d98afc37119b87033b454fb183eae447b8640f386bc46bc34ec976309919`;
- selo serial consumido/refutado, mantido byte-idêntico:
  SHA-256 `77dd70e005c0868988c707b0e941a00a21e4f358d7e7de8ece3d02e59486f213`.

Proveniência da auditoria: `HEAD
7dd25ff0e222b6c7c640d6bc7957b98f94227507`; working tree compartilhada e
não commitada; `git status --short` SHA-256
`fe3de296316ffb89466c9459ec56ab07f1c5696975cec4522871a53ac9b7d7e4` e
`git diff HEAD --stat` SHA-256
`684fae83ab9c406de47f7018b8eed447d0bba1d37dc82a553b992ac2ede6e54f`.
Estes hashes descrevem uma árvore com materializações anteriores P1293 de
outros papéis; não atribuem essas mudanças a esta autoria.

## 2. Medição anterior à decisão

O recibo causal mede que parser, AST, `Value` e `Args` ainda distinguem os
três casos. A primeira colisão ocorre em
`01_core/src/compiler/stdlib/structural/math.rs:637-644,696-703`: ausência do
named permanece exterior, mas `Value::None` e
`Value::Content(Content::Empty)` viram ambos `Some(Content::Empty)`.

O payload em `entities/elements/math_attach.rs:17-25` usa seis
`Option<Content>`; seus accessors e traversal estão em `:27-77`. Os
constructors públicos em `entities/content.rs:1589-1618` recebem a mesma
cardinalidade binária. `eval/math.rs:422-459` produz attachments sintáticos;
`eval/repr.rs:496-514` serializa o estado já colidido;
`math/layout/mod.rs:667-675` o projeta para layout e `:2029-2037,2242-2250`
faz as duas reconstruções recursivas; `math/layout/attach.rs` recebe apenas
conteúdo opcional. Assim, nenhum desses pontos pode recuperar legitimamente
`Present(Content::Empty)` depois da colisão.

Os probes públicos fixam:

- omitido e `none`: largura `5,8080pt` e sem `SpaceAfterScript`;
- `[]` e outros conteúdos vazios efetivamente presentes: largura `6,4240pt`;
- delta `0,6160pt`, exatamente um `SpaceAfterScript` da fonte;
- `repr`: omitido ausente, `none` explícito e `[]` distintos;
- `Unknown = 0` para a classificação causal.

ADR-0107: os três estados, a forma `repr` e a extensão são observáveis de
linguagem; o enum Rust é mecânica. ADR-0108: a decisão abaixo sucede a medição,
separa intenção de comportamento da mecânica vanilla e explicita refutadores.

## 3. Inventário de owners e call sites

### Owners produtivos que realmente mudam

| Prompt L0 1:1 | Consumer | Motivo causal |
|---|---|---|
| `entities/elements/math_attach.md` | `01_core/src/entities/elements/math_attach.rs` | define carrier, seis campos, traversal e `sup/sub` |
| `entities/content.md` | `01_core/src/entities/content.rs` | constructors públicos recebem carrier |
| `compiler/stdlib/structural/math.md` | `01_core/src/compiler/stdlib/structural/math.rs` | primeiro ponto de captura/colisão |
| `compiler/eval/math.md` | `01_core/src/compiler/eval/math.rs` | produtor sintático do mesmo payload |
| `compiler/eval/repr.md` | `01_core/src/compiler/eval/repr.rs` | projeção morfológica dos três estados |
| `compiler/math/layout/_comum.md` | `01_core/src/compiler/math/layout/mod.rs` | despacho e duas reconstruções |
| `compiler/math/layout/attach.md` | `01_core/src/compiler/math/layout/attach.rs` | box/spacing somente para `Present` |
| `shell/cli.md` | `02_shell/src/cli.rs` | call site produtivo adicional `:792-805`, serialização semântica de MathAttach |

O oitavo owner L2 não constava do mínimo causal inicial, mas a busca global
read-only provou acesso produtivo direto aos slots; omiti-lo deixaria o tipo
público novo sem consumer coordenado. Não há owner 1:N: cada prompt permanece
dono exclusivo do consumer listado e todos pinam o mesmo Núcleo Tekt.

### Call sites auditados que não exigem nova obrigação L0

- `compiler/math/layout/spacing.rs:64` lê somente `base`; a troca do tipo dos
  slots não muda seu contrato;
- `compiler/introspect.rs`, `compiler/introspect/locatable.rs` e
  `03_infra/src/query_helpers.rs` casam somente a variante `MathAttach(_)` e
  não leem/reconstroem os slots;
- `compiler/layout/equation.rs:751`, `02_shell/src/cli.rs:1800`, os blocos
  `#[cfg(test)]` dos oito consumers, `compiler/eval/tests.rs`,
  `compiler/math/layout/tests.rs` e `03_infra/src/export/tests.rs` contêm
  fixtures/calls mecânicos. Precisarão da adaptação de assinatura em etapa de
  implementação autorizada, mas não são owners produtivos adicionais nem
  autorizam edição de teste neste recibo;
- `compiler/eval/bindings/field_access.rs` não consome o carrier vigente e a
  lacuna adjacente de fields continua fora do escopo, conforme exigido.

## 4. Decisão e Núcleo Tekt

Foi criado
`00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml`:

- TOML 1.0, `tekt = 1`, `kind = "nucleus"`;
- SHA-256 raw:
  `29a7ec096d899850eb602afa0cbe0b44d46147214b06296f2e90c40776e3cb7b`;
- SHA-256 efetivo V26, sem dependências:
  `81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715`;
- DAG: nó raiz, `depends = []`, sem ciclo;
- consumidores pinados: exatamente os oito prompts da tabela acima.

Contrato confirmado: `MathAttachSlot` é enum fechado
`Omitted | ExplicitNone | Present(Content)` usado por `t,b,tl,bl,tr,br`.
Constructors, transformations e reconstruções recebem/preservam o carrier.
`sup/sub` continuam `Option<&Content>`, projetando somente `Present` com a
precedência já contratada. `repr` omite `Omitted`, imprime `none` para
`ExplicitNone` e `[]` para `Present(Content::Empty)`. Layout cria caixa e
reserva spacing somente para `Present`; `Omitted` e `ExplicitNone` são
layout-equivalentes.

Proibidos: `Settable` genérico, sentinel, heurística por conteúdo/texto/
caractere/fonte/span/width/ink, estado global, parser/AST/Args/Value,
nova variante global de `Content`, default, fase ou compatibilidade além da
assinatura pública expressamente confirmada. Refutadores: qualquer colapso dos
três estados, alteração de mensagens/spans/ordem, spacing em `ExplicitNone`,
ausência de spacing em `Present(Content::Empty)`, ou necessidade de owner/
contrato adicional.

## 5. Hashes L0 antes/depois e consumers congelados

| L0 | SHA-256 antes | SHA-256 após autoria |
|---|---|---|
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` | `8be46d4c97b9f63b0b757274610bb257c1e1f19378bcc7cf342039bd00f00ae7` |
| `entities/content.md` | `48212970feaf1e46db64e31eaeba606e0dcbc283bba2c86a28dd679c3f51e7e9` | `a3b6e823858ac71068fbe6d42bd1b3a426bfffbc0c845e94f7375cf5ff0e1cd9` |
| `compiler/stdlib/structural/math.md` | `8a6c9255bd739a9adc710a007a6d99c0feb967d818221ee986dca161e6da04e5` | `feab650dfe0ed5ea10befbed58ef9d0ea8fed212235cabb6bb4d55707e718542` |
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` | `7e5621fe39ca1d9ee314d83815b189950b08149ed18541c7e50c85ebbe2811d7` |
| `compiler/eval/repr.md` | `07bdbec8bf68e17ab1fe360559059259e3f4f2731b9a0d15dc29a8b6c511e5de` | `fcc9646ce83828eee2119747c965353a5e8772558de1dac0192d7d18822bdf47` |
| `compiler/math/layout/_comum.md` | `71c304857a76eb71fb7698be061bdd449e330410bf081e48d8ba21caa816b2b1` | `14b21d75dbc3ab49e484e5fc91654d77ff2d0217cd4f88f7f21a84870192163d` |
| `compiler/math/layout/attach.md` | `24cb3d594438709e759d01cae6b0dc0c7c8162c95f81c78ed4cd094d2d1a0c83` | `be8b854258c3e5ff1b9b49f8d88200a57813de7a2a4b28d4eb891c79150d807d` |
| `shell/cli.md` | `ed05e4ffa8dd1f94137ceda7486eae3c893acba50ae55f3f53de43864ec28bff` | `13529356ff9b3bbeaed487b0a3d5c1860b763c6c7852d3f4749491bbc74be134` |

Consumers permaneceram byte-idênticos durante esta autoria:

| Consumer | SHA-256 |
|---|---|
| `entities/elements/math_attach.rs` | `3dc6b20ee1d1e9ad420b78b0eeaabc7688d4f920e4c5923b268c81236f8d3e02` |
| `entities/content.rs` | `f31a13ee93df8039bf8727221c36fa64d50a0715f32814b3a8b74bd7a7a6f974` |
| `compiler/stdlib/structural/math.rs` | `2ddc0acf10582966dbe8bb9caded4ce273608b433db95cd04efd6440e4bc4540` |
| `compiler/eval/math.rs` | `9a24894c3341371a42afe782ad39c505aeea25d689f590cd0a6c9d60b5bbd80b` |
| `compiler/eval/repr.rs` | `8efc40fefab5836603ae757b8ee3a0e00c5d3b53f755703db811f04e39aa045a` |
| `compiler/math/layout/mod.rs` | `f612884fc3f39cdb509fa20e59d7d5f077be33dd49052b34aaf56e08a9d6c81c` |
| `compiler/math/layout/attach.rs` | `36a6699f507b3ddc4f1360c04bcc46c451dd42010f7e9051ed1536e738a7acd4` |
| `02_shell/src/cli.rs` | `cac2c72d05f59a57f46e5175639954a78f666ea68a699a0bb01adfbb5312cf30` |

## 6. Gates e dry-run

`crystalline-lint --checks v15,v26 --fail-on warning .` terminou exit `0`,
sem violações. TOML foi parseado por `tomllib`; pins usam o hash efetivo
completo e as referências estão byte-sorted. `git diff --check` nos nove
artefatos L0/nucleus terminou exit `0`.

`crystalline-lint --fix-hashes --dry-run .` terminou exit `0`, sem escrever, e
enumerou exatamente estes oito drifts:

```text
eval/math.rs old=34a81f38 hash-a=bf2c17ba hash-b=2ad201fa
eval/repr.rs old=4c04b417 hash-a=a281e850 hash-b=d4cff924
math/layout/attach.rs old=11e7a221 hash-a=af390ac9 hash-b=53188f01
math/layout/mod.rs old=289307df hash-a=c4b5f741 hash-b=60be9ddc
stdlib/structural/math.rs old=ff23cbe8 hash-a=e658276b hash-b=81441281
entities/content.rs old=25d896e3 hash-a=de6f3b46 hash-b=cd6bccec
entities/elements/math_attach.rs old=45fcff7a hash-a=a385d72b hash-b=bc010300
02_shell/src/cli.rs old=35fdd87b hash-a=3b5b8ab6 hash-b=84ab63b4
```

`--fix-hashes` não foi executado e nenhum header consumer mudou. V5 deve
permanecer RED exatamente nesses oito pares até o resselo mecânico humano.

## 7. Invalidação, segregação e próxima transição

O selo SHA-256
`77dd70e005c0868988c707b0e941a00a21e4f358d7e7de8ece3d02e59486f213`
fica histórico, consumido e invalidado pela colisão de carrier; o arquivo do
selo permanece byte-idêntico e não autoriza writes. Contrato protegido,
oracle, RED/discrimination receipts, testes, produto, headers, ataques,
veredito e lotes C/D não foram lidos/escritos por esta autoria além das
entradas públicas expressamente autorizadas.

Próximo passo permitido ao coordenador: aplicar `--fix-hashes`, confirmar que
somente os oito consumers acima mudam em header, revalidar V5/V15/V26, executar
gate discriminatório substituto e emitir selo serial separado. Uma futura
allowlist de implementação deve cobrir somente esses oito consumers e o
`p1293-implementation-receipt-b.md`; adaptações mecânicas de fixtures/testes
exigem autorização explícita no selo, sem transformar seus prompts em owners
do carrier. Até lá, produto permanece bloqueado.
