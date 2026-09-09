# P1329 — reabertura pré-C do domínio comparável

**Veredito: manter HOLD até sucessão explícita do L0, manifesto e freeze.**
O parecer `p1329-review-l0.md` fica superado quanto à suficiência da ressalva
IEEE. Sua revisão parou cedo demais nos métodos abs e não seguiu os
construtores das grandezas. Não é necessário mudar entidades nem copiar Scalar.

Revisor `/root/p1329_review`; A/B sem atestação de isolamento/refinement seal;
somente evidência e parecer, sem edição do material verificado.

## Fonte e medição anteriores à recomendação

Vanilla `typst-utils/src/scalar.rs:29-31` converte NaN em zero na construção;
`scalar.rs:195-215` reaplica a conversão na multiplicação. Os construtores
`layout/angle.rs:34-40`, `ratio.rs:78-79`, `fr.rs:39-40`, `abs.rs:26-32` e
`em.rs:30-31` usam esse Scalar. Seus campos internos privados impedem construir
legalmente essas grandezas com NaN através dessas APIs. Inf continua admitido.

Hashes SHA-256 das fontes locais ratificadas lidas:

```text
scalar.rs e3b7bcf23af0ae7fedd312c74fa21b0a65cc14f1cfd62bec556155f75c902891
angle.rs 76edf90e5c2e38c61189644d77ef78655486230723ad1fdce6b624f8259182d5
ratio.rs 522409f31d4973a3fd8438d1dcd6ba53d2923d45f1ecaadfcc8aed8cd66db06a
fr.rs 884667cce6653a4a24912ad7626c37ea0f381fe20c6b67da261761b981f592d6
abs.rs 13829790d0386138ae6215e0c52448954095bad726817ea2c9962aff88268fd7
em.rs 4e875e95e96cb4ff6cb999da768ddcfdbf73e7b635636a9b8ff341c96f1b5327
```

Sondas públicas em 2026-09-09T12:30:30.628Z, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
`git diff HEAD --stat` produziu exatamente o snapshot integral transcrito em
`p1329-review-l0.md` (inclusive todos os caminhos e suas contagens).
Os binários foram `/usr/local/bin/typst` (vanilla SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`) e
`/tmp/p1328-target.T7Tg57/release/typst` (baseline SHA
`94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9`).
Argv por expressão: `[binário, "--color", "never", "eval", expressão]`.

| Expressão | stdout vanilla | stdout baseline |
|---|---|---|
| `(calc.inf - calc.inf) * 1deg` | `"0deg"` | `"float.nan * 1deg"` |
| `(calc.inf - calc.inf) * 1%` | `"0%"` | `"float.nan * 1%"` |
| `(calc.inf - calc.inf) * 1pt` | `"0pt"` | `"0pt"` |
| `(calc.inf - calc.inf) * 1em` | `"0pt"` | `"0pt"` |
| `(-calc.inf * 1pt)` | `"-float.inf * 1pt"` | `"-float.inf * 1pt"` |
| `(-calc.inf * 1em)` | `"-float.inf * 1em"` | `"-float.inf * 1em"` |

Todas as células da tabela têm exit 0, stderr vazio e newline final no stdout.
Aplicar `calc.abs(...)` às quatro primeiras expressões devolve os mesmos zeros
dimensionais no vanilla; no baseline ainda há a rejeição dimensional histórica.
Essa rejeição baseline não informa qual seria o módulo do candidato.
O primeiro ensaio via Node sem escalada falhou por `spawnSync EPERM`; um ensaio
shell sem parênteses nos argumentos negativos encontrou o parser da CLI. Nenhum
desses resultados instrumentais fundamenta a tabela, que veio da execução
com argv estruturado e parênteses acima.

## Recomendação

Manter a operação local especificada para valores já construídos no cristalino,
mas explicitar que NaN dimensional **não é entrada equivalente no vanilla**.
O resultado nativo com NaN verifica uma obrigação local sobre a representação
cristalina; não prova paridade de calc.abs para uma mesma entrada dimensional.
Não se deve generalizar isso a todo não finito: Inf tem representante vanilla.

Também não basta chamar esses estados de "somente sintéticos": as sondas de
ângulo e ratio demonstram construção pública de NaN no cristalino. Os testes
podem ser sintéticos, mas a dívida anterior à operação pode ser observável na
linguagem. A impressão de `0pt` não prova que os componentes de Length são zero;
essa inferência seria inadequada sem observação semântica dos componentes.

O domínio comparável exige entradas dimensionais equivalentes já constituídas
nos dois sistemas. O comportamento de construção divergente deve ficar fora
da alegação de paridade deste passo, explicitamente como dívida preservada;
isso não autoriza editar operadores/entidades/dispatcher. A aceitação P1329
continua a exigir a operação local decidida e todas as obrigações finitas/mistas.

A norma e o manifesto congelados são entradas protegidas. Registrar sucessores
e fazer o autor A/B reconhecer o novo domínio antes de integrar/rodar RED/C;
conservar r1 imutável. Se a obrigação de módulo permanecer igual, pode haver
reuso dos mesmos bytes de testes com nova declaração de alcance, sem repetir
todo o corpus apenas por mudança de hash. Novas sentinelas de construção, se
necessárias, pertencem ao autor A/B e devem ser congeladas antes de C.
