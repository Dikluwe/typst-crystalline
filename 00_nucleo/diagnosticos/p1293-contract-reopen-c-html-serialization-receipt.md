# P1293 — reabertura contratual do Lote C: serialização HTML explícita

## Estado e autoridade

```text
status: AWAITING_HUMAN_GATE_ADR_0127
phase: L0_AUTHORED_BEFORE_CODE
lineage_reseal: NOT_RUN
product_write: NONE
oracle_write: NONE
seal_write: NONE
lot_d: PROHIBITED
```

- Papel: `autor_contrato_p1293`, sequência causal de autoria L0/contrato.
- Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
  isolamento técnico de leitura.
- Instante da autoria: `2026-09-02T12:33:40-03:00`.
- `HEAD`: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`; working tree não commitada.
- Manifesto recebido: SHA-256
  `73fbab030e2eebf14b275914d851106582402b3c33f58e6a745c86b0c5e67e6d`.
- Selo C encontrado e agora causalmente invalidado, sem editar seus bytes:
  SHA-256 `13abcda7c4b955b83b6694dc26840f9362fa0cae112f52f0696d95a96b289af7`,
  bloco canônico
  `223b1ddea355939115ab8e7acb57280dfad960911eea46e561b647de7f936ec0`.
- Medição residual independente:
  `p1293-lot-c-residual-measurement-receipt.md`, SHA-256
  `4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`.
- Recibo candidato C recebido somente como evidência pública: SHA-256
  `7584b5611592998d7253c35b68a8a1a13f98d976831309027d028a817b75a23f`.
- Oráculo protegido preservado e verificado somente por hash:
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`.

## Medição antes da decisão

O recibo independente mede `44/46` divergências no teste de superfície,
uma divergência nominal de escaping no DOM, `37/37` spans agregados, oito
mensagens divergentes, quatro polos de `video.preload` não discriminados pelo
oráculo vigente, `feature/target 1/1` verde e `Unknown=0`.

As linhas causais registradas são:

- `repr.rs:580-597` usa joins lineares para `HtmlElem`, enquanto
  `repr.rs:1087-1140` já contém a disciplina canônica curta/multiline;
- `html.rs:819-843,859-862` recebe `args.span` agregado, mas
  `call_dispatch.rs:139-189,1450-1489` ainda possui as âncoras sintáticas;
- `html.rs:864-910,969-999,1044-1052` produz os oito textos divergentes;
- `data.rs:1445-1449,1846` e `typed.rs:176-206,466-475` fecham preload como
  valores `none|auto` ou string `"metadata"`;
- `export/html.rs:190-194,211-218,506-511` usa um escape único conservador;
  `encode.rs:99-133` e `charsets.rs:21-49` separam atributo e texto vanilla;
- `cli.rs:144,344`, `main.rs:384,390`, `pipeline.rs:160,171` provam os pontos
  atuais de transporte e a ausência de qualquer modo explícito.

A validação WHATWG confirmada pelo coordenador classifica ambas as
serializações como HTML válido. A forma cristalina é conservadora e permanece
o default; a forma vanilla é uma alternativa explícita de paridade. Nenhuma é
chamada de “não padrão”.

## Decisão do dono normatizada

1. CLI expõe `--html-serialization crystalline|vanilla` somente em `compile`;
   ausência resolve para `crystalline`. A flag é aceita em compile não-HTML,
   mas só produz efeito em `OutputFormat::Html`; outros comandos a rejeitam.
2. L2 valida e transporta enum/dado cru; não conhece L3 nem serializa HTML.
3. L4 faz o mapping exaustivo do enum L2 para o enum L3; não contém escaping.
4. L3 define `HtmlSerializationMode::{Crystalline,Vanilla}`. A pipeline cria
   uma entry point com features+modo e mantém as APIs antigas delegando com
   `Crystalline`.
5. O exporter mantém `export_html` com default cristalino e oferece
   `export_html_with_serialization`. `Crystalline` preserva os bytes atuais;
   `Vanilla` escapa `&`/`"` em atributo normal e `&`/`<` em texto normal.
6. Ambos permanecem no pipeline semântico HTML da ADR-0128. Não passam por
   layout paginado e não alteram feature/target.
7. O contrato C corrige `HtmlElem` curta/multiline, spans das sete identidades,
   mensagens/casts locais e `video.preload`; vetores DOM usam modo vanilla
   explícito e novos casos discriminam os dois modos e os quatro polos
   typed/string de `none`/`auto`, com `"metadata"` como controle positivo.

## Owners 1:1 e hashes

| Owner L0 → consumer | L0 antes | L0 depois | consumer preservado |
|---|---|---|---|
| `prompts/shell/cli.md` → `02_shell/src/cli.rs` | `13529356ff9b3bbeaed487b0a3d5c1860b763c6c7852d3f4749491bbc74be134` | `34b2165e21d41b24bd31f1b0f0a641b0aaa8b8017445a76ae9fb7062bcc0d1f2` | `b0cbc485dd56b8f0ecb72938ef42482f576fecf2ef904ded9d4c7744acda501a` |
| `prompts/wiring.md` → `04_wiring/src/main.rs` | `884c7418ab72d17bdee36f76b0699955d15730947b3928908f1bd8a3dcfe0d8e` | `539815d1e892117baec6e5e8e51e37a155dadcaa682e3349c11cc66dd52f623c` | `27ec540df13bff14b04b215762ea77f791cbcbb423e6f43db73ca24e6f31bec8` |
| `prompts/infra/pipeline.md` → `03_infra/src/pipeline.rs` | `28fb5f18d4f6d454de32cb6120c0ac2919d802a5d1aa85261b466872d632df3b` | `011ec99349fe18ab2fd30fa1437e9b2f9b2b520a3baf2e7aeba96e2e8c1c267c` | `ef8a69218c2d5a522580caf95425bff39ed44cb453a151fa97849e96a0cccdb4` |
| `prompts/infra/export/html.md` → `03_infra/src/export/html.rs` | `b5560e10cc7161551a8c1bba38f8747bde059367550162834dbffd2b3c9715cd` | `f436e4e1b85198064c9db391f3ae6ab7461fa4692accfec0ec16ff9d2fa30d21` | `49324678c2e37464fa813ce87e2d78156a19cc17eaf83bb88b2173f27de0bf12` |
| `prompts/compiler/eval/repr.md` → `01_core/src/compiler/eval/repr.rs` | `d9499bbbc889eb42d485c700823cd428950bedefda4496065488a678c6e8b500` | `81a2ad840822e5ab536f06f6a019059aa5732738cc3f9726f0900bbaf37a4aef` | `9c5316ecaa9332379bad5afa1c1e8de1e504e77d085c91b475480591b30a3d1b` |
| `prompts/compiler/eval/call_dispatch.md` → `01_core/src/compiler/eval/call_dispatch.rs` | `f683a20d0171fe82983ac20886b79d09710ce1c38b036a43e8ac56e58e8959f7` | `9d275e92a8e07bbbbe0dd63590ea6466c4db260ed8a9c168e58f1af684f6ed4d` | `4d7002a7475fb8999025fbb28a2381647557ad0617697c52113c34ed9cd1664b` |
| `prompts/compiler/stdlib/html.md` → `01_core/src/compiler/stdlib/html.rs` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` | `d55d75144fc7c3ed48f598ceeef041fe23b7e286cde5cfc6f50fc4395778c43c` | `7f88674ef792b04994f6c5cdc98f43830a68f6ca37537071bce8e196cee3c7e3` |
| `prompts/wiring/tests/p1293_contract.md` → `04_wiring/tests/p1293_contract.rs` | `6e3355f87a69366fab64bd0160c8e4f49f6c4b5fb95e82e82991fbef199b779f` | `adf47e09192bf792ede1cf227696e6314f1b4aa059c510141d05118ebb799989` | `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5` |

V15 e V26 passam sem violações; não há owner 1:N, Núcleo novo ou pin a
alterar. `entities::Args`, `Content`, `HtmlElem` e exporters não-HTML ficam
fora.

## Classificação ADR-0107/0108/0127/0128/0129

- ADR-0107: `repr`, aceitação, mensagens/spans e sintaxe HTML no modo escolhido
  são observáveis de linguagem; enum Rust e helpers são mecânica livre.
- ADR-0108: as medições e contraprovas precedem cada decisão. Refutariam a
  decomposição perda anterior de payload/ordem, ausência de spans na AST,
  reescrita posterior dos erros, DOM já divergente antes do encode ou
  necessidade de entidade/fase; nenhum refutador ocorreu.
- ADR-0127: repr/spans/casts/preload são correções internas, mas flag, campos,
  enums/entry point públicos e default deliberado são categorias 1 e 2.
  O conjunto inteiro para antes de código e requer novo gate humano.
- ADR-0128: os dois modos pertencem ao mesmo target semântico HTML; modo não é
  feature nem target e nunca ativa `Feature::Html`.
- ADR-0129: oito owners permanecem 1:1; não há obrigação compartilhada nova que
  justifique Núcleo.

## Dry-run e STOP

`crystalline-lint --checks v15,v26 .` terminou `PASS`. O dry-run, sem escrita,
encontrou exatamente os oito drifts esperados:

```text
call_dispatch.rs old=6cdb3f22 hash-a=63d16887 hash-b=b4d881f1
repr.rs          old=a281e850 hash-a=a7bc8b15 hash-b=d91654f3
html.rs          old=c7e51513 hash-a=4eb0acc7 hash-b=33e187a1
cli.rs           old=3b5b8ab6 hash-a=6ec6acf5 hash-b=26707ade
export/html.rs   old=ba8a0c7e hash-a=041cf55e hash-b=5b4f56d5
pipeline.rs      old=7091a54c hash-a=6d75c61f hash-b=c3a60a77
main.rs          old=1d9e10c1 hash-a=934a8298 hash-b=32ad6bca
p1293_contract.rs old=fc7f79aa hash-a=b1f580e2 hash-b=3a8a3c3e
```

`--fix-hashes` não foi executado. `git diff --check` passou. Produto, headers,
testes e oráculo permaneceram byte-idênticos. O selo permanece byte-idêntico,
mas não autoriza novas escritas depois desta reabertura. O contrato exige gate
novo, resselo dos oito headers, atualização segregada do oráculo e gate
discriminatório fresco `29/29`, score `1.0`, survivors `0`, `Unknown=0`.

## Texto exato da confirmação humana requerida

> Confirmado. Autorizo P1293/C a adicionar `--html-serialization
> crystalline|vanilla` somente a `compile`, com default `crystalline` e efeito
> apenas em `OutputFormat::Html`; L2 transporta dado cru, L4 mapeia e L3
> introduz `HtmlSerializationMode::{Crystalline,Vanilla}` e novas entry points,
> preservando as APIs antigas em `Crystalline`. Autorizo também os refinamentos
> L0/contrato de `HtmlElem` multiline, spans das sete funções, mensagens/casts,
> `video.preload` e os vetores protegidos com modo vanilla explícito, dois modos
> e quatro polos preload. Não autorizo Lote D, mudança de feature/target/fase,
> entidades `Content`/`HtmlElem`, outros defaults nem exporters fora de HTML.

