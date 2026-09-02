# P1293 — recibo independente dos testes protegidos e RED

Data da medição final revalidada: `2026-09-01T14:45:50-03:00`  
HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
Estado: **lineage ressellada e mesmo RED reproduzido, abrangente e causal aos lotes A–D**.

Este documento é o recibo do papel `testador_p1293`, sequência causal 3. Ele
não é selo, execução de mutantes, implementação, ataque nem veredito de
produto. Em particular, não declara `mutation_score = 1.0`: congela os 26
discriminadores que o adversário deverá executar sobre mutantes válidos.
O recibo imediatamente anterior a esta revalidação tinha SHA-256
`84a3c31ae1a12df159015b1c243cfe16011d9534a8e76c5c1f6b14dac2866917`.

## Segregação, autoridade e capacidade

- regime: completo, sob `tekt-materializacao-segregada`;
- autoria: independente do contrato e do futuro candidato;
- segregação efetiva: por capacidades e artefatos, sem isolamento técnico de
  leitura do filesystem compartilhado;
- leitura humana do patch candidato: não realizada;
- inspeção de conteúdo produtivo ou de diff produtivo: não realizada;
- escrita autorizada e realizada: somente
  `04_wiring/tests/p1293_contract.rs`, este recibo e fixtures efêmeras em
  `/tmp`;
- escrita não realizada: produto, L0, contrato, manifesto, selo, ataques e
  veredito;
- compilação e execução naturalmente fizeram o toolchain consumir o baseline,
  mas isso não concedeu ao autor acesso de inspeção ao futuro patch candidato.

A capacidade foi ampliada durante a autoria, por autorização explícita do
coordenador, somente para ler `/tmp/p1293-measure.py` e recuperar as oito
fontes B-P07 produzidas pelo medidor independente. O artefato tinha SHA-256
`3f4a4e96ddd1fff309de1aa6bbd02e8355193ab9ad0a4e2673c3aff97c861ab9`.
Não foram lidos o JSON bruto nem outros `/tmp/p1293-*` como fonte contratual.

## Manifesto e causalidade da emenda de capacidade

O manifesto originalmente recebido tinha SHA-256
`22967607a89e070d57e41f25a0468f698cace6ac18c260940d8353d4e4ffa474`.
Durante a autoria ele passou a
`6abe7ab6902b2dc8cbaa61520c84f911b97da61f3caf47ae3e20d0ab1a1de9ba`.
O trabalho foi interrompido até a reautorização do coordenador. A revisão
confirmou que o único delta é o bloco JSON `capability_amendments`, linhas
100–108, que registra exatamente a autorização de leitura acima; baseline,
L0s, contrato, observáveis, resultados esperados, política de `Unknown` e
capacidade de escrita não mudaram.

Prova de reconstrução executada:

```text
$ sed '100,108d' 00_nucleo/diagnosticos/p1293-manifest.json | sha256sum
22967607a89e070d57e41f25a0468f698cace6ac18c260940d8353d4e4ffa474  -
```

O teste permaneceu congelado quanto às expectativas durante essa emenda.

## Entradas pinadas

| Entrada | SHA-256 |
|---|---|
| passo P1293 | `d577a03c27731d11cdcc161a3e93646e90bdb62f0a7f9cfb8d1b4f5980a9dc3a` |
| manifesto original | `22967607a89e070d57e41f25a0468f698cace6ac18c260940d8353d4e4ffa474` |
| manifesto reautorizado na primeira autoria | `6abe7ab6902b2dc8cbaa61520c84f911b97da61f3caf47ae3e20d0ab1a1de9ba` |
| manifesto atual pré-selo | `290c183c16e451fff9bfb16ba2a51b4e69ad5d7885f1f4c952062f997c48703b` |
| owner L0 originalmente consumido | `567ed112b99f897b09f3ed1a4bd7da89391d57f21bff4c8d858ed98b43c5d455` |
| owner L0 materialized-ready | raw `6e3355f87a69366fab64bd0160c8e4f49f6c4b5fb95e82e82991fbef199b779f`; canônico `fc7f79aa` |
| recibo original do contrato | `d3fc0651169e77bb1fab09ed0365bdd2e866b8b1cb08c22f7c0092e7f871a73a` |
| recibo do contrato materialized-ready | `033df282812396f691aab941a2e3a2cb05980e54011cf737f9a2ad4b6dccd1ad` |
| recibo da medição vanilla | `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7` |
| medidor B-P07, capacidade ampliada | `3f4a4e96ddd1fff309de1aa6bbd02e8355193ab9ad0a4e2673c3aff97c861ab9` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

L0s finais consumidos pelo contrato:

| L0 | SHA-256 |
|---|---|
| `compiler/stdlib/foundations/float.md` | `b07ba6be955e28f01e9e12f3f41d3977e5728c13942b381c13428375aff3b57e` |
| `compiler/stdlib/structural/math.md` | `d12e6f40931aa60ccf0972f88d441aa439b115a649452839f5d78167147fce72` |
| `compiler/eval/math.md` | `90ae89477cc55ea25c4f90e2ceac24900a57ecc59e54c7672e637c5cfd7a1ad5` |
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` |
| `compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| `compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |

## Consumer protegido e linhagem

Artefato: `04_wiring/tests/p1293_contract.rs`  
SHA-256 após resselo de lineage: `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`  
Tamanho: `40904` bytes.

O cabeçalho materializa a linhagem 1:1:

```text
@prompt 00_nucleo/prompts/wiring/tests/p1293_contract.md
@prompt-hash fc7f79aa
@layer L4
```

### Prova de corpo semântico inalterado

Antes do resselo, o arquivo bruto tinha SHA-256
`9369fce1e216e7dd5b2845a79bfb3cddc2505a3da196efde43fc01543892606a`.
O SHA-256 calculado removendo somente a linha `@prompt-hash` era
`3a8a3c3e7bfc18c640661ef3adc922c6f6985701ce6f8b7359c978b4221a7d8e`.
Depois de trocar `567ed112` por `fc7f79aa`, o mesmo cálculo produziu exatamente
`3a8a3c3e7bfc18c640661ef3adc922c6f6985701ce6f8b7359c978b4221a7d8e`.

Como prova inversa, substituir apenas `fc7f79aa` por `567ed112` no fluxo do
arquivo novo reconstruiu exatamente o SHA bruto anterior `9369fce1...`. Nenhuma
expectativa, fixture, comparação, lot, mutação ou corpo Rust mudou. A revisão
do owner também declara o hash canônico de comparação/lots/mutações
`df0f4f6ab5e50495c95b2d0531a397913e58026583f63aaa1f54e74c33ade41a`
inalterado; o coordenador comunicou igualmente o prefixo de core do recibo
`3fb28b3b...`. A prova independente deste papel é o hash integral do corpo do
oráculo acima.

O harness verifica antes dos observáveis o hash do vanilla pinado e converte
timeout em `Unknown` explícito e falha bloqueante. Não compara `PartialEq` Rust,
bytes internos, IDs SVG ou path data.

## Política de comparação e repetição

- eval A/B/C e superfícies D: tipo de valor na língua, `repr`, sucesso/erro e
  diagnóstico integral com spans; somente CR/LF terminais são normalizados;
- morfologia matemática: conteúdo e slots preservados, `none`, ordem,
  pontuação, ausência de barra e delimitadores, estilos e `cramped`;
- layout B-P07: `viewBox` semântico com tolerância congelada de `0.01`, além da
  ausência de regra de fração, sem igualdade byte a byte;
- HTML: parser DOM próprio preserva tag, ordem dos atributos, presença versus
  omissão, nesting, texto, escaping e proíbe end tag para void;
- cada grupo parametrizado é executado na ordem normal e reversa; os mapas de
  transcripts, resumos SVG e árvores DOM devem permanecer idênticos. D também
  verifica explicitamente estabilidade direta/reversa;
- as oito fontes B-P07 são as do medidor independente pinado, sem adaptação ao
  candidato.

## Execução RED do baseline

Compilação isolada do consumer:

```text
$ cargo test -p typst-wiring --test p1293_contract --no-run -q
exit 0
```

Execução registrada:

```text
$ script -q -e -c 'cargo test -p typst-wiring --test p1293_contract -- --test-threads=1' /tmp/p1293-red-lineage-reseal.log
exit 101
running 9 tests
test result: FAILED. 2 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.29s
```

Saída integral: `/tmp/p1293-red-lineage-reseal.log`  
SHA-256 da saída: `e5c29129e0459af3297fb0c76b1c8bc136fa72b902c130d14be5807783fb4420`  
Tamanho: `91077` bytes.

Os dois testes verdes são controles do contrato: congelamento do comportamento
pré-existente D e fechamento do registro de opacidade/mutações. Os sete REDs
são os sete testes materiais A–D:

| Lote | Teste RED | Causa semântica observada |
|---|---|---|
| A | `p1293_a_static_bound_values_and_closed_errors` | `float.is-nan` estático e ligado ausentes; valores, coerção e seis diagnósticos divergem consequentemente |
| B | `p1293_b_identity_morphology_convergence_and_errors` | quatro funções qualificadas ausentes, logo identidade, payload, convergência e erros não são preservados |
| B | `p1293_b_semantic_layout_vectors_none_and_no_fraction_rule` | vetores semânticos congelados, `none`, barra/delimitadores e estilos não convergem para o vanilla |
| C | `p1293_c_surface_all_specific_casts_and_closed_errors` | sete constructors e matriz de 48 atributos/casts ainda não satisfazem presença, repr e negativos fechados |
| C | `p1293_c_dom_order_void_escaping_and_nesting` | DOM das sete tags, ordem/presença, void, body/nesting e escaping não são preservados |
| C | `p1293_c_feature_and_target_are_independent_axes` | `Feature::Html` e target paged/HTML não satisfazem os quatro quadrantes congelados |
| D | `p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved` | os dez nomes permanecem `grid_*`/`table_*` em vez de nomes curtos; calls, `.with` e aliases são discriminados |

As advertências de compilação do workspace não impediram compilação nem
execução. Não houve crash, timeout, hash vanilla divergente, erro do parser do
harness ou falha de build alheia. Portanto elas não recebem crédito como RED;
as falhas acima são exclusivamente observáveis semânticos P1293.

## Cobertura A–D do §7

- A: field estático, chamada ligada, NaN/finitos/`±inf`, coerção inteira,
  missing/extra/named/tipo inválido e acesso ligado sem chamada;
- B: presença/tipo/nome das quatro funções, repr/default/payload completo dos
  seis slots, `none`, aridade e named, binom 2+, lower/ordem/vírgulas,
  barra/delimitadores, mono/script, `cramped` e convergência
  sintática/qualificada;
- C: sete constructors, chamadas vazias, body/nesting das cinco tags normais,
  body rejeitado nas duas void, classes `AttrKind`, 48 atributos específicos,
  feature/target, DOM e escaping;
- D: dez nomes curtos, calls e tipos de conteúdo, aridade/diagnósticos, seis
  aliases flat e ausências históricas, `.with` e não contaminação.

## Matriz mutante → discriminador congelado

Esta matriz prova a capacidade discriminatória estática do oráculo; a morte
física de cada mutante pertence ao papel adversário.

| Mutante | Teste/testemunha no consumer |
|---|---|
| `MA1` | `A-P01-P02`: NaN não pode virar sempre falso |
| `MA2` | `A-P01-P02`: `+inf` e `-inf` permanecem não-NaN |
| `MA3` | `A-P03`: forma ligada converge à estática |
| `MA4` | `A-N03`: named desconhecido preserva o erro |
| `MA5` | `A-P01-P02`: identidade/repr é `is-nan`, não qualificada |
| `MB1` | `B-P02-P03-P04-P05`: todos os seis slots e a ordem são observados |
| `MB2` | `B-P02-P03-P04-P05` + `B-P03-layout`: `none` não colapsa nem vira texto |
| `MB3` | `B-P04/B-P07-binom`: barra de fração é proibida |
| `MB4` | `B-P02-P03-P04-P05`: lower conserva ordem e vírgulas |
| `MB5` | `B-P06`: sintaxe e função qualificada convergem |
| `MB6` | `B-P05/B-P06`: mono/script não ganham wrapper divergente |
| `MB7` | `B-P05/B-P07-cramped`: `cramped` explícito altera o observável correto |
| `MC1` | `C-N-*-unknown/data`: named arbitrário não vira string livre |
| `MC2` | `C-P05/C-P06-order`: `Presence(false)` é omitida |
| `MC3` | `C-N-col-body/C-N-wbr-body`: void rejeita body |
| `MC4` | `C-P01-P02`: body normal omitido conserva o default contratado |
| `MC5` | `C-N-*-cross`: atributo específico não cruza tag |
| `MC6` | `C-N-*-enum`: enum rejeita token alheio |
| `MC7` | `C-P07`: target não liga feature |
| `MC8` | `C-P06-void-only`: void não recebe end tag |
| `MC9` | `C-P05/C-P06-escaping`: ordem e escaping são estruturais |
| `MD1` | `D-P01-ten`: os dez siblings, não somente três amostras |
| `MD2` | `D-P03-flat`: aliases flat conservam nomes históricos |
| `MD3` | `D-P01-table`: table não recebe nome grid |
| `MD4` | `D-P02/D-P04`: renomear não pode mudar call/`.with`/diagnósticos |
| `MD5` | `D-P01-ten`: nenhum sibling conserva underscore |

Denominador congelado: `26`. IDs distintos: `26`. Discriminadores ausentes:
`0`. Execuções de mutantes neste papel: `0` por segregação. Score reivindicado:
**nenhum**.

## Unknown e opacidade

Os únicos opacos previstos são exatamente:

- `A-O01`: payload binário, sinal e quiet/signaling bit do NaN;
- `B-O01`: bytes SVG, IDs e path data exatos;
- `C-O01`: comportamento de browser/CSS/mídia/rede/acessibilidade além do DOM;
- `D-O01`: endereço interno do function pointer.

Eles não recebem crédito. `Unknown` exigido ou em mutação: `0`. Timeout, crash,
harness ambíguo, vanilla não pinado, mutante sobrevivente ou divergência de
ordem são programados como `Unknown` bloqueante, nunca como sucesso.

## Preservação de P1292 e gates locais

`04_wiring/tests/p1292_contract.rs` não foi editado (`git diff --quiet` exit
`0`) e conserva SHA-256
`fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`.

```text
$ cargo test -p typst-wiring --test p1292_contract -- --test-threads=1
exit 0
test result: ok. 11 passed; 0 failed; 0 ignored; finished in 18.75s

$ cargo fmt --all -- --check
exit 0
$ crystalline-lint --checks v5,v15,v26 --fail-on warning .
✓ No violations found
$ git diff --check
exit 0
```

Na medição RED, a working tree era não commitada. `git status --short` mostrava
os sete L0s finais, os sete consumers produtivos já presentes no baseline e os
artefatos diagnósticos P1293; `git diff HEAD --stat` tinha 14 arquivos
modificados, `338 insertions(+), 30 deletions(-)`. Essas alterações já existiam
fora da capacidade de escrita deste papel. Os únicos artefatos autorados por
este papel são o consumer protegido e este recibo.

## Conclusão limitada ao papel

O consumer com `@prompt-hash fc7f79aa` compila, P1292 permanece verde e A–D ficam RED por causas
semânticas P1293, com comparação no nível da língua/DOM/erros, repetição em
ordem normal e reversa, quatro opacos fechados, `Unknown` requerido igual a
zero e matriz completa de 26 discriminadores. A implementação continua fora
do escopo; o próximo passo causal é do implementador e, depois, do adversário.
