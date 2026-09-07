# P1307-R5 — suplemento independente de `repr` e fallback CBOR

Medição aditiva autorizada pelo coordenador após congelar os quatro artefatos Content R5. Nenhum predecessor, L0, Rust, header, teste ou candidato foi alterado ou lido como candidato. Regime: **executado sem atestação de isolamento técnico**. A skill `tekt-materializacao-segregada` exigiu preservar a primeira tentativa inválida e só estender o recorte após a revisão focal válida.

## Entradas e medidas

Baseline R5 SHA-256 `32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`; HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado. Cada seção da medição registra `git diff HEAD --stat`, horário, hashes de fonte e dos predecessores. Binários novamente conferidos: vanilla `/usr/local/bin/typst` SHA `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`; baseline em RAM SHA `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`. O alvo continua o upstream ratificado `a51e02804`.

Fonte lida antes de decidir: vanilla `foundations/content/mod.rs:625–642` monta `repr` dos campos do elemento, sem o label dos metadados; `:709–719` serializa Content como mapa dedicado. Baseline `01_core/src/compiler/stdlib/loading.rs:240–264` serializa Content no fallback `Text(repr(v))`. Vanilla `loading/cbor.rs:74–84` define a função pública decodificadora `cbor(bytes)`, não `cbor.decode`.

Execuções entre `2026-09-07T18:33:05.710081+00:00` e `2026-09-07T18:36:33.453397+00:00`: 10 históricas na tentativa 1, 10 na revisão focal, 42 na extensão limitada e 20 controles de strings literais; total 82, soma de tempo dos processos 12,461709909 s. Houve ainda um timeout da revisão automática de permissão **antes de criar processo**; um redespatch autorizado executou a mesma revisão 2. Esse intervalo não foi contado como execução Typst.

São 17 casos de oracle e 72 observações correntes, zero `Unknown` e zero falhas de pré-condição. Default Heading foi controlado nos quatro perfis de features, sempre target PDF, nas três projeções `repr`, CBOR decodificado e bytes integrais. en/pt e numbering "1"/"I" foram medidos somente em default. Não houve matriz global, repeat/reverse nem certificação HTML.

## Mapeamento exato do predecessor

`p1307-r4-oracle.json::r2.construct-LocatedContent` é reproduzido em `default.repr` **com o mesmo documento integral**, inclusive marcador `P1307R2:`. SHA-256 da fonte: `78fd17fa61567bfd8107820e9daa9de588d9fcd08c993c24261fd2c6a3e304a9`. Os envelopes medidos coincidem com os lados vanilla/baseline históricos.

O predecessor preservava como dívida o baseline `heading(level: 1)[[Probe]]`. A proposta sucessora R5 é o `repr` realizado vanilla abaixo, sem label, com quebras e espaços integralmente pinados no JSON:

```text
heading(
  level: 1,
  depth: 1,
  offset: 0,
  numbering: none,
  supplement: [Section],
  outlined: true,
  bookmarked: auto,
  hanging-indent: auto,
  body: [Probe],
)
```

en reproduz default. pt substitui `[Section]` por `[Seção]`. Os dois padrões substituem `numbering: none` por `numbering: "1"` ou `numbering: "I"`. O baseline atual produz a mesma string curta em todos esses casos.

O registro `predecessor_mapping` é uma **proposta explícita de supersessão no manifesto sucessor**, não alteração retroativa nem autorização autônoma para implementar: a troca da antiga política baseline depende do L0 e do gate do owner R5.

## CBOR: delta do fallback, não paridade geral com vanilla

Na medição direta `cbor(cbor.encode(x))`, vanilla retorna um dictionary dedicado, com `func`, campos realizados e label. Baseline retorna uma **String** contendo seu repr curto. O contrato anterior do CBOR deliberadamente preserva esse fallback Text; este suplemento não o troca pelo mapa vanilla.

Para obter a expectativa sucessora sem candidato, cada `repr` vanilla medido foi usado como **entrada String literal** em controles públicos bilaterais. Ambos os binários produziram exatamente os mesmos bytes CBOR e a mesma String decodificada. Dessa forma:

- `repr(x)` futuro proposto: envelope vanilla integral.
- `cbor(cbor.encode(x))` futuro proposto: String contendo esse repr realizado, não Dictionary.
- Bytes futuros propostos: bytes CBOR integrais do controle String correspondente, medidos e pinados, não apenas comprimento nem roundtrip.

O oracle referencia a origem exata por `expected_measurement_ref`. Para `cbor-decoded` e `cbor-bytes`, a expectativa vem de `literal_controls.rows`, lado baseline, cujos resultados foram conferidos iguais ao vanilla. A referência vanilla direta do Content também permanece registrada e **não** é selecionada como obrigação CBOR. Os controles de String são default; sua aplicação aos três outros perfis está explicitamente declarada na referência. As três projeções diretas de Heading default foram de fato medidas nos quatro perfis e ficaram estáveis.

Como indicação auditável, não substituto dos bytes integrais: baseline usa 28 bytes em cada Heading; a expectativa Text(repr realizado) tem 172 bytes em default/en/pt e 171 bytes nos padrões "1"/"I". Os dois textos de 172 bytes não são confundidos por tamanho: Section/Seção têm payloads diferentes registrados integralmente. O controle cru `"Probe α\n"` produz nos dois binários `105,80,114,111,98,101,32,206,177,10` e decodifica exatamente a string original.

## Tentativa preservada e limitações

A primeira tentativa seguiu a expressão solicitada `cbor.decode(...)`, que é inválida nos dois binários. Vanilla reportou `function \`cbor\` does not contain field \`decode\``; baseline reportou sua mensagem anterior. São diagnósticos válidos, mas falhas da pré-condição do probe, não falta de Content nem RED da implementação. A fonte confirmou `cbor(bytes)` e a revisão 2 usou essa API existente.

A primeira projeção `repr(array de bytes)` também revelou abreviação vanilla (`.. (129 items omitted)`). Ela permanece no histórico, mas **não** é usada como oracle integral. A revisão passou a `range(len).map(i => str(bytes.at(i))).join(",")`; cada byte está presente. O transporte externo igualmente preserva todos os bytes UTF-8 da string pública. Não houve normalização de payload, ordenação de mapa ou ajuste de expectativa a candidato.

O caso decodificado usa `repr` do resultado para transporte de tipo/valor; a projeção independente dos bytes integrais impede que isso se reduza a roundtrip-only. O comparador é `p1307-r4-oracle.py::classify`, congelado: igualdade do envelope integral, incompletude → `Unknown`.

Este suplemento não cobre repr geral de Content direto, callbacks, todas as classes de Content, símbolos, todos os queries ou paridade geral de CBOR. Não declara selo, implementação, RED/GREEN ou score de mutações. Seus `future_expected` são a proposta concreta a incorporar somente pelo manifesto/L0 sucessor aprovado.

## Arquivos congelados

| Arquivo em `00_nucleo/diagnosticos/` | SHA-256 |
|---|---|
| `p1307-r5-repr-measure.py` | `f2d6fc2ab3960f5118d63f049f82b885bd3677093bb5a9e9afd5cc7fef312531` |
| `p1307-r5-repr-measurement.json` | `5c145ef6f5694948d34de6be2f35c873b526170d2752b34ca08e858216508567` |
| `p1307-r5-repr-oracle.json` | `49fafb6c13b48e990a04ec65732eea8cd1f371c85d0ed59d8a15f5b0e0aca34c` |

Os quatro Content R5 anteriores foram preservados; seus hashes estão em `provenance.protected_inputs` em cada etapa deste suplemento. A nota possui hash externo no recibo do coordenador, evitando autorreferência.
