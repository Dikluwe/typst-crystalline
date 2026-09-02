# P1293 — recibo de reabertura B após julgamento independente RED

## Estado

```text
status: ready-for-coordinator-fix-hashes
classification: independent-RED-on-frozen-B-observables
canonical-contract: unchanged
protected-oracle: unchanged-and-not-read
seal-a1eba37c: invalidated
layout: blocked-pending-bilateral-causal-diagnosis-after-new-seal
```

Este recibo pertence ao papel segregado `autor_contrato_p1293`. A autoria leu
somente L0s e recibos públicos/canônicos autorizados; não abriu patch candidato
nem fonte do oráculo protegido e não editou produto, testes, oráculo, selo,
ataques ou veredito.

## Inputs e proveniência

| Input | SHA-256 / identidade |
|---|---|
| manifesto recebido | `dd73a216a323fa7636fc55b01333b32c7714fe6a79f50fb08765cfa59cc16470` |
| recibo final B rejeitado | `d677c0b1e6d8796c6680787d27b3409c100ff13653ab8ff89d7154813866720c` |
| selo serial test-only invalidado | `a1eba37c94e70b7f8e9576c18f83a9f354db402932a627a32202a2d5eeeb229f` |
| contrato canônico | `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072` |
| recibo contratual atualizado | `2da9111c107cc8f1f2395c2d37674454e71c69dd4500d6b3859b7cdf44d43634` |
| medição vanilla pública | `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7` |
| oráculo protegido, hash-only | `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5` |
| RED congelado, hash-only | `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e` |
| HEAD / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| instante desta autoria | `2026-09-01T17:48:52-03:00` |

O recibo B causal registra medição em working tree não commitada às
`2026-09-01T17:38:04-03:00`, com `git status --short` SHA-256
`726e1ba1d9dc0e428293570360209cda99f770f35bc55bf4bc3aa46bc6eab78a`
e `git diff HEAD --stat` SHA-256
`5199b677eaa82adf35bfb594af4198e0be1a28d34b6e82ee59d39252560adce0`.
O próprio recibo limita a conclusão a evidência local e requer aprovação
independente.

## Medição anterior à decisão

### Fonte pública

- `p1293-implementation-receipt-b.md:5-10`: evidência local completa, mas
  explicitamente sem aprovação;
- `p1293-implementation-receipt-b.md:90-94`: alega wrapper/convergência sob os
  testes próprios;
- `p1293-implementation-receipt-b.md:169-174`: remete a decisão ao checkpoint
  independente;
- `p1293-vanilla-measurement-receipt.md:164-204`: assinaturas, morfologia
  `styled(child: [x], ..)` e convergência sintaxe/qualificada;
- `p1293-vanilla-measurement-receipt.md:206-217`: oito viewBoxes B-P07
  públicos e exatos;
- `compiler/eval/repr.md:248-272` antes desta edição: forma fechada de
  attach/binom sem projeção explícita da folha direta;
- `compiler/eval/repr.md:84-97`: precedente interno do formatter textual
  canônico para folha direta de `MathVec`;
- `compiler/stdlib/structural/math.md:162-180` antes desta edição: assinatura
  fechada sem hints posicionais;
- `compiler/stdlib/math_style.md:107-126,183-199` antes desta edição: named
  genérico e identidade fechada suficiente para a exceção mono/script.

### Julgamento independente recebido

O coordenador comunicou julgamento RED nos observáveis congelados:

1. `MathStyled` mono/script foi reduzido a `[x]`, em vez de preservar
   `styled(child: [x], ..)`;
2. folhas de attach/binom/mono/script na sintaxe apareceram como
   `x`/`T`/equivalentes, contra `[x]`/`[T]`/equivalentes na forma qualificada;
3. `binom(upper: ...)` perdeu `try removing upper:`; `body:` de mono/script
   produziu `unexpected argument: body` em vez de
   `the argument body is positional` com `try removing body:`;
4. quatro dos oito B-P07 continuaram divergentes.

Os oito vetores públicos, na ordem congelada, são:

```text
23.2705x19.4843
19.7681x9.6041
46.797666667x24.057
30.3325x7.8815
25.029888889x8.921
14.0987x6.2447
8.3578x5.9708
8.3578x6.5406
```

O julgamento não localizou causalmente o subconjunto de quatro nem um
`file:line`/owner de geometria. Assim, layout permanece RED/bloqueado; o número
quatro não é usado para decidir fórmula ou L0 de layout.

## Inferências e refutadores

Medido: wrapper, projeção da folha, mensagem/hint e layout são observáveis da
linguagem. Inferido: os três primeiros eixos cabem nos owners existentes por
variantes/campos/identidades já presentes, sem payload ou fase nova.

Refutam essa inferência: necessidade de provenance bit/entidade/API/`Args`;
projeção recursiva ou nominal por testemunha/origem; mudança de
precedência/span; ou divergência restante nos mesmos observáveis após usar os
formatters canônicos. Para layout, qualquer proposta sem reprodução bilateral
dos oito vetores, identificação nominal dos quatro REDs e causa `file:line`
é refutada por insuficiência de medição.

## Decisão L0 e ownership

Foram atualizados primeiro somente três owners 1:1:

| Owner | raw SHA-256 antes | raw SHA-256 agora | Consumer / drift esperado |
|---|---|---|---|
| `compiler/eval/repr.md` | `5ef385638f3d828025f18735cb180b57288c170a44ac131752ae92d79ebc4737` | `2c3b10e788d85fea2e455fc217f00d47d09d7d3c92d9eb34265bae84a230c0f8` | `repr.rs old=7c714164 hash-a=4c04b417 hash-b=545f0ea2` |
| `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` | `9a44293c7ee7a44795f0864c1b6a9f14f98074995caea2ec932cab328d0e258d` | `structural/math.rs old=73443b4c hash-a=ff23cbe8 hash-b=ce079105` |
| `compiler/stdlib/math_style.md` | `4ef71193711435b3b732354ccfdb77973ae81275aae628c61ab272ed87a68b97` | `1d7236f411376dc3515214e4ea51d2df40af0895199e55d410bbd8a1e0318831` | `math_style.rs old=a7d71b3a hash-a=a36c31b4 hash-b=56b9e300` |

Autoridade exclusiva:

- `repr.md`: wrapper `MathStyled` de mono/script e formatter textual canônico
  de folhas diretas somente nos campos P1293 de attach/binom/styled;
- `structural/math.md`: mensagens/hints exatos dos campos posicionais
  reconhecidos de attach/binom;
- `math_style.md`: `body:` posicional mais hint somente em mono/script; as
  outras doze funções permanecem fora do escopo.

Nenhum owner 1:N foi criado. L0s B não alterados permanecem byte-idênticos:

| L0 | SHA-256 |
|---|---|
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| `compiler/eval/call_dispatch.md` | `f683a20d0171fe82983ac20886b79d09710ce1c38b036a43e8ac56e58e8959f7` |
| `compiler/eval/tests.md` | `4e5963393178ccec06850ca16e84db530675c6760e977d68d68839e1cf891848` |

Morfologia, mensagens e hints são observáveis da linguagem (ADR-0107). As
três correções são paridade interna em fluxo contínuo (ADR-0127): sem API
pública, campo, entidade, trait, assinatura, default, compatibilidade ou fase
nova; não há gate humano novo.

## Gates antes do resselo

- V15: PASS;
- V26: PASS;
- dry-run: exatamente os três drifts enumerados acima;
- nenhum hash foi escrito;
- contrato canônico/oráculo/RED/produto/teste/selo permanecem inalterados.

Próxima ação: o coordenador aplica `--fix-hashes` somente nos três consumers,
revalida V5/V15/V26/diff e solicita novo gate/selo serial. Depois desse selo,
o implementador pode corrigir os REDs morfológicos/diagnósticos dentro desses
owners e apenas diagnosticar bilateralmente layout. Se o L0 vigente não
legitimar a correção causal de layout, deve parar para nova reabertura. Lote C
continua proibido.
