# P1307-R3 — revisão independente dos contratos L0

Estado: `DOCUMENTARY_REVIEW_CONDITIONAL_ADR0127_PENDING`.

Os dezoito L0 abaixo formam uma proposta documental coerente para o gate
humano. Não encontrei contradição substancial remanescente que impeça sua
apresentação ao dono. Isto **não** é aprovação de implementação, prontidão
RED, resultado de mutantes ou certificado de paridade. As condições anteriores
ao código continuam abertas e estão enumeradas abaixo.

## Papel, entradas e limites

Revisor: `/root/p1307_oracle`, não autor destes L0. O mesmo agente é autor da
medição R2 reutilizada; portanto há segregação entre autoria e revisão do L0,
mas não se alega uma segunda autoria independente daquela medição.
Aplicada a skill `tekt-materializacao-segregada`: **executado sem atestação de
isolamento técnico**. Fontes, árvore e evidências são compartilhadas. Não há
candidato de implementação nesta rodada nem verificador final de candidato.

Entradas congeladas:

- Preflight R3: `p1307-r3-preflight.json`, SHA-256
  `d333d8b298067d9e104efdad45c1241eaae0a821a7e49988dcf3c6658c1fa4d2`.
- Baseline documental R3: `p1307-r3-baseline.json`, SHA-256
  `b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
  capturado em `2026-09-07T17:03:42.154788+00:00`, HEAD
  `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306;
  fontes, textos L0 anteriores e diff/stat exatos estão nesse snapshot.
- Adendo sucessor de escopo: `p1307-r3-scope-amendment.json`, SHA-256
  `c8a55d6a9946cc878384c20fd6b0ced2b06ddea15f56e6e5043f727b83dc215a`.
  Baseline adicional `p1307-r3-scope-baseline.json`, SHA-256
  `592497d1e786241b9ba121479c0c55dbad370a70732e130b4f7ac3bd709d4405`,
  em `2026-09-07T17:15:47.609275+00:00`, anterior à edição dos quatro owners.
  O preflight original não foi retroativamente alterado.
- Medição de contrato P1307: `p1307-contract-measurement.json`, SHA-256
  `f1191d3de029dabf6f66f87872bee8146aa43106ae54e84facc22952afd238d0`.
- Medição R2: `p1307-r2-measurement.json`, SHA-256
  `847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`;
  auditoria de spans `p1307-r2-span-audit.md`, SHA-256
  `adb27e4be810f841c06fb70cf611f63bb6205443e2d7d47c8485de6a6f962910`.
- Nota do autor dos seis L0 de Args/transporte:
  `p1307-r3-contract-note.md`, SHA-256
  `66098523a2025b7754493a67cafcbb6d02f3516a1fe8f75581b09419db0d7600`.

Os nomes de artefatos desta seção são relativos a `00_nucleo/diagnosticos/`.
A referência continua upstream ratificado `a51e02804`, executável vanilla
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Não executei novas sondas binárias nesta revisão documental, nem modifiquei
L0, Rust, headers, Cargo, testes ou evidências predecessoras.

## Confronto substantivo com fonte e evidência

1. **Ocorrências são dados, não apenas spans.**
   `lab/typst-original/crates/typst-library/src/foundations/args.rs:218-235`
   converte cada named encontrado; a medição R2 de `pretty:"bad"` anterior a
   `pretty:true` rejeita o primeiro valor. `entities/args.md` e
   `compiler/eval/call_dispatch.md` agora retêm valores anteriores e sequência
   conjunta, em vez de confiar no último valor do IndexMap. Loading é o owner
   da validação; não surgiu tabela de assinatura ou seleção por mensagem no
   dispatcher. As hipóteses antigas de Func sobre fusão suficiente do mapa e
   ownership duplicado de Args foram expressamente substituídas.
2. **Origem acompanha o valor transportado.**
   A fonte vanilla `lab/typst-original/crates/typst-eval/src/call.rs:412-460`
   distingue spans de argumento/valor e conserva itens de spread Args. Os
   casos f/g R2 conservam origens distintas após factory e sink, embora os
   valores coincidam. O contrato prevê FileId, dois spans individuais e
   agregado independente, preservados por With/spread/clone/sink. Não permite
   busca de AST, igualdade de Value, hash ou Source da chamada final como
   recuperação de origem. A síntese None é assumidamente sem origem.
3. **Transformações não foram confundidas.**
   Em `foundations/args.rs:414-450,485-491`, filter conserva a ocorrência;
   map conserva nome e arg-span, mas destaca value-span; os resultados têm
   agregado detached. Os contratos de collections refletem isso e declaram
   ordem/quantidade de callbacks potencialmente novas. `:468-482` remove
   named LHS presentes no RHS em Add; o contrato de join não usa a concatenação
   integral de With de `foundations/func.rs:372-374`. Remoções em closures
   regeneram views e não reinserem parâmetros/defaults no sink.
4. **Mutabilidade legada tem uma obrigação concreta.**
   Some é autoritativo somente com views coerentes. O contrato proíbe deixá-lo
   stale, exige invalidação antes de mutação direta de Some e proíbe essa
   invalidação quando ela apaga origem necessária. Não exige uma comparação
   impossível de valores como prova da coerência. PartialEq conserva a dívida
   legada items/named/span e ignora o campo novo; Debug não passa a expô-lo.
   A garantia não é imposta pelo sistema de tipos: a auditoria dos writers e
   seus testes continuam sendo gate real, não propriedade já demonstrada.
5. **Identidade e representação estão separadas.**
   R2 mede membro original `encode` e parcial `(..) => ..`; a fonte ratificada
   `foundations/func.rs:460-470` sustenta a regra geral de With. Func mantém
   name/namespace e parent With não liga seus argumentos ao membro. O owner
   repr assume explicitamente State/Counter/Location e ordem causal de Args,
   sem alterar constructors ou alegar correção geral de filhos/Content.
6. **CBOR não ficou sob promessas incompatíveis.**
   `01_core/src/compiler/stdlib/loading.rs:263` já chama repr no fallback.
   Logo corrigir repr pode alterar CBOR mesmo sem tocar seu braço. Os L0 de
   loading/repr/testes agora declaram a exceção State/Counter/Location/With/Args
   e exigem deltas sucessores congelados. Os controles R2 de payload integral
   continuam úteis, mas não provam preservação geral; Symbol/Content continuam
   dívida específica. É proibido reaproveitar uma exigência antiga de
   preservação indiscriminada e simultaneamente anunciar essas correções.
7. **Migrações auxiliares não se apresentam como paridade geral.**
   Math conserva ocorrências disponíveis dos builders AST sem ocultar a dívida
   de Spread; numbering reconhece valores calculados como sintéticos. Int,
   float e gradients antepõem receiver sintético sem apagar spans dos demais.
   Method_dispatch/field_access consomem coerentemente sem mudar a política
   legada de diagnósticos, lookup ou lugar mutável. A fachada stdlib só liga
   nativas; eval só registra namespaces; testes não autorizam código de produto.

Os paths `foundations/...` nos itens acima referem-se à árvore ratificada
`lab/typst-original/crates/typst-library/src/`, não aos módulos cristalinos.
As conclusões são limitadas às obrigações R3; não auditam toda dívida histórica
dos dezoito prompts como se a rodada fosse uma reescrita global.

## Achado durante a revisão e seu refinamento

O primeiro conjunto de quatorze owners omitia writers produtivos que receberão
Some: `compiler/stdlib/gradients.rs:97`,
`compiler/stdlib/foundations/float.rs:120,124`,
`compiler/eval/bindings/method_dispatch.rs:46,336,388` e
`compiler/eval/bindings/field_access.rs:767`, sob `01_core/src/`.
Esses pontos contrariariam a obrigação universal de coerência se fossem
deixados intactos. O coordenador declarou o adendo de escopo e capturou os
quatro textos anteriores antes de redigir suas migrações. Li integralmente
os quatro L0 e confrontei os novos trechos com os writers: o achado conhecido
fica atendido **documentalmente**, não materializado.

A nota congelada do autor dos seis L0 registra corretamente o estado anterior
em que os quatro owners ainda careciam de reabertura. Este diagnóstico refina
aquele estado sem alterar a nota. Não converter o fechamento desses pontos
conhecidos em alegação de que nenhum outro writer pode existir.

## Condições ainda bloqueantes antes de implementação

- Confirmação humana ADR-0127 do contrato concreto: ArgOccurrence, novo campo
  de Args, construtores/métodos públicos e compatibilidade de literais; os
  encoders/defaults e os efeitos públicos enumerados de repr, callbacks,
  contagem, join e fallback CBOR. Autorização de redação não satisfaz isso.
- Auditoria final de todos os produtores, mutações diretas, projeções,
  transportes e consumidores de Args. Cada writer precisa de owner atualizado
  e classificação de síntese/transporte/consumo. Descoberta adicional exige
  nova declaração documental antes de editar código; não permite descartar
  Some para contornar a allowlist.
- Expectativas independentes sucessoras antes de RED/candidato para as
  extensões apoiadas por fonte mas ainda não medidas: ordem/repetições de
  Args, filter/map/join, parcial de outras nativas e deltas integrais CBOR
  das classes repr corrigidas. Ausência dessa medição não invalida a redação
  baseada em fonte, mas não pode ser chamada de PASS de implementação.
- Completar a cadeia de ataques independente sem alterar o plano/oracle
  predecessor: named anterior apagado; origem f/g fundida; stale carrier;
  map com retorno igual mas value-span errado; filter que apaga origens;
  join tratado como With; sink que conserva consumidos; callback reordenado;
  CBOR alterado fora da exceção. Mutante inválido não é morto; Unknown de caso
  obrigatório não é sucesso. Literal público integral não se reduz a
  round-trip, tamanho, sorting ou substring.
- Gates de ownership/núcleos e linhagem reavaliados pelo coordenador; eventual
  V5 neste estado L0-only deve permanecer explícito, não resolvido editando
  headers sem materialização. Depois de código autorizado, será necessária
  nova verificação final com autoridade distinta, RED/GREEN e regressões.

## Pins dos L0 efetivamente revisados

Caminhos relativos a `00_nucleo/prompts/`; SHA-256 dos bytes completos:

```text
entities/args.md 75de6ac49d69331fc604984c92874a706442f823cc4706bd480b434c55c83d94
entities/func.md 523f25d1e5926d52c1402c911c66444df7c61d6bf21e720ee692e77b68b1c0db
compiler/eval/call_dispatch.md 6f1e02969301600a8b78782133736f956f63c3f91e5273867c116bf17d730588
compiler/eval/closures.md dbf7bccc186ad25035db5b7ee1d581d001d805dfc42dc1b5f8944fc1264a9309
compiler/stdlib/collections.md f5eadc7eb3ec75c73c1ced46d87670f47705c956e1f41acb4a5310bc1ad6b781
compiler/eval/operators/join.md 155d40ba7397b5ae28a23456f57509b4f5f0ae822d56c3e28901637d6521cf66
compiler/eval/repr.md 855c657c79141e608b47f1bf942e16e1a579a292c92f13337d449df058b8cd93
compiler/stdlib/loading.md 783fa3d7c087767b13f4bd2cc3e5a3aa6a1e34632bcf2a6bd3c39c766cc61976
compiler/stdlib/_comum.md 87c18ca349ee8669dd242d85be09f95ab792336e6e35941f7663d36f23932211
compiler/eval.md 05b4eac0f2e814c67c75ae1cdcaa08083665da9878a9f67eda0d773e73eefdbb
compiler/eval/tests.md 606e41df1d5e55ad9fa51662695ed9faf937b35d1dea443067488e3c14dfc3b6
compiler/eval/math.md 409c81da6419b359e46dd970811100367edfdadcc7e59a2cd14799d1ef12335c
compiler/stdlib/numbering.md 3ef25a8ed94b9d41b088e6c515859d5e011f321d21b9d4cf22b992991bb5ebfe
compiler/stdlib/foundations/int.md 2037bf60d00edd067d84f7814bbb3229f26e61b2f6e3412c458fda316bb76e3c
compiler/stdlib/gradients.md 25ab680bf8fb4f6f6efef8a2d3f66c9cf5e05589977484848a843e6b744807e8
compiler/stdlib/foundations/float.md 68e877cfdaf7feaf7fee344c268244d04df13707f3aa2beeba83bf8513f49794
compiler/eval/bindings/method_dispatch.md 7360a5335f0adcb1ce45deea487c93347bbf9cefcfb1e8cbfceb9164d336e521
compiler/eval/bindings/field_access.md 645dcc4e7c16d48b1b36b9a0d12dc385f13d6a6f940fb5fab8e8381e24d20dda
```

Conferência de hashes feita em `2026-09-07T17:18:40.062247+00:00`, comparando
as entradas inventariadas no baseline R3 aos arquivos correntes: nenhuma
mudança nas fontes/configurações não L0 inventariadas, nenhum núcleo alterado
e nenhum artefato predecessor alterado. Esta conferência não executa linter,
não prova ausência de novos paths não inventariados e não é certificado de
implementação. O único write deste revisor nesta rodada é este diagnóstico.
