# P1463 — contrato observável congelado de `format.png`/`ppi`

**Estado:** CONTRATO CONGELADO PRÉ-CANDIDATO — REVISÃO R1  
**Regime:** protocolo completo de materialização segregada  
**Alegação proporcional:** **executado sem atestação de isolamento**  
**Baseline causal imutável:** `bf0a29cad5423f038cb51c8c8a079465a1d4ed65`  
**Tree causal:** `3fa973a4461c438769b22c0971eccc041dade422`  
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`  
**Manifesto:** `00_nucleo/diagnosticos/p1463-manifesto.json`  
**SHA-256 do manifesto:** `4f087f97ea10ba298a97d9b1b0b32901d2538a06eb7cb311f8fe44ff6b20b739`

Este contrato foi escrito sem ler implementação candidata. O delta entre o
antecedente operacional `23cd47e4ade4d6deb7095cb7400478feffb1e9f9` e o
baseline causal contém somente o Passo P1463, os L0s e o Núcleo Tekt; nenhum
consumer L1–L4 mudou. O candidato estava ausente durante a autoria.

A confirmação humana posterior ao commit causal P1463 foi reportada pelo
orquestrador a esta autoridade. O autor do contrato não a verificou
independentemente; o registro satisfaz a precedência causal informada para o
gate ADR-0127, mas não transforma o workspace compartilhado em ambiente
isolado nem autoriza alegação de segregação atestada.

## 1. Autoridade, alcance e embargo

As fontes normativas e seus hashes são as entradas protegidas do manifesto.
Elas congelam somente o fragmento público `format.png`/`ppi`, o transporte da
ausência/override até o raster e as fronteiras arquiteturais diretamente
afetadas. Não se alega paridade PNG geral.

Até existir selo discriminatório válido:

- não existe candidato autorizado;
- autor de contrato, autor de oráculos, adversário, testador A/B,
  implementador e verificador obedecem às allowlists do manifesto;
- o autor do contrato não lê patch candidato, não escolhe estrutura de dados,
  não corrige solução e não emite veredito;
- manifesto, este contrato, oráculos e mutantes congelados tornam-se entradas
  protegidas e não podem ser ajustados para acomodar um candidato.

Qualquer mudança no corpo semântico de Prompt L0 ou em entrada protegida
não-Prompt invalida o selo e os agregados que dela dependam. A única exceção é
a transição de metadata de linhagem definida na próxima seção. Checkpoints
independentes com pins semânticos idênticos permanecem reutilizáveis conforme
ADR-0131.

### 1.1 Revisão R1 — hash semântico e resselo de linhagem

**Hipótese de correção:** separar o hash raw baseline do hash do corpo
semântico permite que `crystalline-lint --fix-hashes .` materialize a linhagem
obrigatória depois de RED/candidato sem falsamente invalidar um contrato cujo
texto normativo não mudou.

**Custo:** zero vetor de mutação e zero caso de oráculo executados; somente os
dois artefatos contratuais foram revistos. **Delta discriminatório:** antes de
R1, qualquer `Hash do Código`/`@prompt-hash` novo seria indistinguível de deriva;
depois de R1, apenas essas linhas, com recibo before/after e corpo semântico
idêntico, passam. O vetor classificatório e O01–O23 não mudaram.

Para cada Prompt L0, o **hash raw baseline** cobre os bytes integrais. O **hash
semântico** aplica uma e somente uma normalização: remove a linha inteira que
começa por `Hash do Código:` e seu terminador de linha. Nenhum whitespace,
Unicode, pin, cabeçalho ou outra linha é normalizado. Se a linha não existir,
como no Prompt do teste antes de seu consumer existir, os bytes permanecem
inalterados.

| Prompt | SHA-256 raw baseline | SHA-256 semântico sem `Hash do Código` |
|---|---|---|
| `compiler/stdlib/format.md` | `892e1d56d20189a4eb0c95d13383d5517156533a886f6db263be20fc7aeafc37` | `4ca2e7d77f610b2026bd82795317abf82d493c45cac944de8e8e419e63aca4d0` |
| `compiler/eval/rules.md` | `94444882ec98bf6ad932513745b9c0e1834aabde2f5f6cb5410da3fbd755fed4` | `1214db59a58a55c436c6ebdfd1e1bc77d3108d9661580e74ef594e0ff4debc3f` |
| `compiler/layout/format_options.md` | `9a4f4a41618dfa396aca8efe9301a718660055a0659cbce362f20a10e0dc0d79` | `5e2c1a4e29cd701ca6537d7577ae82ee91d0e05b888c1ef63d3f2ad148b74bf0` |
| `entities/document_format_options.md` | `52dbeeec49bba1992fb6b86f936cb7943c0af10fd6148d78967b1a12f2df709f` | `18b2b57f5e238db9cd60722442768258ff84586cdf6386ffd05aa58c13ed06bb` |
| `shell/cli.md` | `4f0cf474c2baa41102867dbf06a8589047803b6a6a791b872d1266640d83c4b9` | `86d17aa1df32b03eed11a93b431ad444b0177b7bd9c48b45b24badf176544c1f` |
| `infra/pipeline.md` | `ea9c658e42c68ec807cc159bddb75b8efbbabca021cd210cebdf8626cca95829` | `7aedac0f75ffc06be3a201e29b53a1c1c809224c9e22bc71e652cfa3c5622092` |
| `wiring.md` | `7f9adfb551b0cc69dc619f76a141138080a1f70448b7c2a89c9c85aab829d70e` | `7ff655970922a5aaeddf1c648c07e2f03020d35e130c961ae20e898755f500bf` |
| `wiring/tests/p1463-format-png.md` | `19a9dbdadc35949890ce45f691ee1ab4da2dbbc8a8747f496ac6dc8fe84b94b2` | `19a9dbdadc35949890ce45f691ee1ab4da2dbbc8a8747f496ac6dc8fe84b94b2` |

Depois de RED independente e candidato, fica autorizada exatamente uma
transição determinística de metadata:

- em Prompt L0, inserir ou substituir somente `Hash do Código: <8 hex>`;
- em consumer, inserir ou substituir somente `//! @prompt-hash <8 hex>`;
- no Prompt de teste, a ausência baseline de `Hash do Código` pode virar a
  linha calculada quando `04_wiring/tests/p1463_format_png.rs` existir;
- o corpo semântico de cada Prompt e consumer permanece byte-idêntico durante
  a execução do reparador;
- bytes raw/efetivos do Núcleo e seus pins em `pipeline.md`/`wiring.md`
  permanecem idênticos.

A comparação de consumer é feita entre o estado imediatamente anterior e o
estado imediatamente posterior ao `--fix-hashes`, ambos já contendo o mesmo
candidato. Não se compara código candidato ao baseline para alegar que o
consumer inteiro ficou idêntico.

O resselo precisa de recibo before/after com commit/tree e working tree,
comando, path e SHA-256 da ferramenta, timestamps, hashes raw de cada Prompt e
consumer tocado, hashes semânticos dos Prompts, diff de linhas, hashes/pins do
Núcleo e resultado V15/V26 posterior. O diff deve provar que **nenhuma outra
linha** mudou. Hash semântico divergente, pin divergente, linha adicional ou
recibo incompleto invalida a transição e os checkpoints dependentes; não pode
ser reclassificado como simples resselo.

## 2. Classificação língua versus mecânica

São observáveis de linguagem ou de CLI: existência, tipo, nome e namespace de
`format.png`; default público; construção proibida; casts e diagnósticos da set
rule; fronteira root/container; ordem da cascata; presença e parsing de
`--ppi`; precedência caller/documento; dimensões do raster; publicação ou não
do artefato; neutralidade dos targets; inventário público P1357.

São mecânica livre: macro/trait upstream, `Scalar`, forma Rust da coleção,
downcast, número de passes, algoritmo de travessia, conversão final `f64` para
`f32`, encoder, compressão, metadados, tamanho e hash do PNG, endereços e layout
de memória. Não se pode fechar paridade por igualdade desses itens.

Mensagens, spans, exit, stdout/stderr e IHDR são mecânica somente na origem,
mas são observáveis quando expostos pela interface pública. Bytes PNG fora de
assinatura/IHDR não são congelados.

## 3. Superfície pública congelada

Nos perfis `default`, `a11y-extras`, `html` e `html+a11y`:

1. `type(format.png)` é `function`, o nome público/repr é `png` e
   `format.png.ppi` é **float** `144.0`;
2. alias lexical e `std.format.png` preservam tipo, nome, identidade nominal e
   namespace;
3. o namespace contém somente `ppi` neste passo;
4. `format.png()` falha com `cannot be constructed manually` e não produz
   conteúdo;
5. `format.pdf` e `format.html` continuam ausentes; P1463 não cria aliases
   globais, feature gate, pages, template de nomes, multipágina nem encoder;
6. `format.svg` e toda a superfície certificada em P1462 permanecem intactos.

O valor `144.0` não pode aparecer como integer `144`, como `72`, como `2`, como
sentinela ou como resultado de conversão antecipada.

## 4. Validação de `#set format.png`

O alvo reconhecido é estruturalmente e somente `format.png`. A gramática
pública aceita zero ou um argumento named `ppi`:

| Entrada | Resultado obrigatório |
|---|---|
| `#set format.png()` | sucesso neutro; nenhuma chave/default é escrita |
| `#set format.png(ppi: 72)` | sucesso; integer é normalizado ao float `72.0` |
| `#set format.png(ppi: 1.5)` | sucesso; float preservado |
| `#set format.png(ppi: 0)` | sucesso |
| `#set format.png(ppi: -72)` | sucesso |
| `#set format.png(nope: 72)` | erro `unexpected argument: nope` |
| `#set format.png(ppi: "72")` | erro `expected float, found string` |
| `#set format.png(ppi: true)` | erro `expected float, found boolean` |
| `#set format.png(72)` | erro `unexpected argument` |
| `#set format.png(ppi: 72, ppi: 144)` | erro `duplicate argument: ppi` |

Erros ocorrem antes de alterar a cadeia. Nenhum desses casos pode cair no
warning legado `set: target '' ainda não suportado`. P1463 não faz alegação
positiva para NaN ou infinito; crash, abort, OOM, timeout ou saída insuficiente
nesses casos é `Unknown`, nunca `Preserved`.

## 5. Cascata, root e carrier documental

A chave causal é `format.png.ppi`. Omitir `ppi` significa ausência, não escrita
implícita do default. Portanto:

- `ppi: 72` seguido de regra vazia termina em `72`;
- `72 → 288` termina em `288`;
- `288 → 72` termina em `72`;
- SVG e PNG são campos ortogonais: uma regra não altera a outra;
- chave futura de PDF/HTML permanece neutra, não é inferida por prefixo.

Somente `Sequence` e `Styled` transparentes no envelope root podem transportar
a decisão documental. Qualquer ocorrência contratada de formato dentro de
container falha exatamente com:

```text
format set rules are not allowed inside of containers
```

Nesse erro nenhum `PagedDocument` parcial e nenhum artefato são publicados.
Uma variante interna diferente de float sob `format.png.ppi` é estado inválido
e falha fechado; nunca volta silenciosamente para `144.0`.

O carrier público documental tem estes observáveis:

```text
DocumentFormatOptions.svg.pretty == false
DocumentFormatOptions.png.ppi == 144.0
```

`png.ppi` guarda o `f64` documental cru; não usa `Option`, não divide por 72 e
não fabrica igualdade total. `PngFormatOptions` e `DocumentFormatOptions`
suportam `PartialEq`; `SvgFormatOptions` pode conservar `Eq`. Clone/default e
duas instâncias são independentes.

## 6. CLI e preservação de ausência

`compile`, `watch`, os aliases respectivos e a grafia legada normalizada
aceitam `--ppi <PPI>`. A representação pública transportada é `Option<f64>`:

```text
flag ausente        -> None
--ppi 72            -> Some(72.0)
--ppi 1.5           -> Some(1.5)
--ppi=0             -> Some(0.0)
--ppi=-72           -> Some(-72.0)
```

L2 não aplica `144`, não divide por 72, não conhece `RenderOptions` e não
condiciona o parsing ao formato final. `--ppi texto` termina no parser com exit
2. A grafia negativa com `=` segue o parser normal. `eval`, `query`, `info`,
`fonts`, `init` e `completions` não aceitam a flag.

A flag pode ser sintaticamente aceita quando o target final não é PNG, mas o
valor não pode produzir efeito fora do braço PNG.

## 7. Resolução e assimetria documental versus CLI

A precedência pública é total e congelada:

```text
Some(cli_ppi) -> cli_ppi / 72.0
None          -> document.format_options.png.ppi
```

Consequências obrigatórias:

- caller explícito vence o documento;
- ausência chega intacta até a consulta do documento;
- o valor documental entra **cru** como pixels por ponto;
- somente o caller CLI é dividido por `72.0`;
- o default total é o `144.0` já resolvido pelo carrier documental;
- o mesmo valor efetivo alimenta os ramos com e sem fontes;
- a conversão numérica final na fronteira do raster não pode alterar as
  dimensões observáveis.

Esta assimetria é contraintuitiva, mas foi medida no vanilla pinado. “Corrigir
unidades” dividindo o documento por 72 ou passando a CLI crua é violação, não
melhoria. O Núcleo Tekt congela a precedência genérica; a transformação `/72`
continua específica do owner da pipeline e não migra ao Núcleo.

## 8. Oráculo dimensional

A fixture canônica usa página opaca quadrada de `1pt`, margem zero. O oráculo
lê assinatura PNG e largura/altura no IHDR, sem comparar compressão ou hash.

| Documento | CLI | Dimensão esperada |
|---|---|---:|
| sem regra | ausente | `144 × 144` |
| `ppi: 72` | ausente | `72 × 72` |
| `ppi: 2` | ausente | `2 × 2` |
| `ppi: 1.5` | ausente | `2 × 2` após arredondamento vanilla |
| sem regra | `--ppi 72` | `1 × 1` |
| sem regra | `--ppi 288` | `4 × 4` |
| `ppi: 288` | `--ppi 72` | `1 × 1` |
| `ppi: 72` | `--ppi 288` | `4 × 4` |

Zero e negativos finitos produzem `1 × 1` no corpus. A mesma matriz aplicável
é executada com fonte ausente e fonte resolvida; dimensões divergentes entre os
dois ramos são `Violated`.

O export continua selecionando somente a primeira página. P1463 não transforma
essa limitação em alegação de paridade multipágina.

## 9. Neutralidade e preservação

Com entrada semântica idêntica, variar `--ppi` ou `format.png.ppi` não altera:

- PDF, SVG ou HTML;
- eval, layout em pontos ou número de páginas;
- `format.svg.pretty` e a precedência P1462;
- `pretty`, `compact`, `PdfTags`, features ou modo HTML;
- `document_id`, timings e publicação atômica de watch;
- warnings, errors ou códigos de saída alheios.

Para SVG, o controle bilateral exige a mesma saída canônica com e sem
`--ppi`; hash idêntico pode ser evidência auxiliar apenas porque os próprios
bytes SVG são o artefato público nesse controle, não um requisito geral para
PNG. Para PDF/HTML, o oráculo compara os observáveis próprios já congelados,
sem introduzir equivalência geral de bytes.

## 10. Ownership, linhagem, V15 e V26

A cardinalidade congelada é:

| Prompt L0 | Consumer exclusivo |
|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` |
| `shell/cli.md` | `02_shell/src/cli.rs` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` |
| `wiring.md` | `04_wiring/src/main.rs` |
| `wiring/tests/p1463-format-png.md` | `04_wiring/tests/p1463_format_png.rs` |

Nenhum código aponta para
`00_nucleo/prompts/_nuclei/export/format-option-precedence.toml`. O Núcleo não
tem `Hash do Código`, owner produtivo ou lista de consumers. `pipeline.md` e
`wiring.md` pinam conjuntamente o hash efetivo V26
`68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11`.

Antes do selo e na certificação final, exigir:

```text
crystalline-lint --checks v15,v26 --fail-on warning .
```

Exit diferente de zero, novo owner, owner órfão, consumer compartilhado, pin
divergente, Núcleo órfão, DAG inválido ou código apontando ao Núcleo é
`Violated` e bloqueia o selo. No baseline causal o comando terminou com exit 0
às `2026-09-19T21:08:56-03:00`, usando binário SHA-256
`e47974fe903b225e96fec580040604d80380edda054a7b2187bbf5b18da9a30b`.
Depois do candidato, uma mudança raw causada exclusivamente pelo resselo R1 não
é violação se o recibo provar igualdade dos hashes semânticos, pins do Núcleo
idênticos e zero linha fora da allowlist de metadata.

## 11. Inventário P1357 e correção do observador

O baseline canônico normalizado herdado de P1462 é:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY normalizado |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

P1463 acrescenta exatamente o owner `format.png`. Depois do candidato, o
vetor esperado normalizado é:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY normalizado |
|---|---:|---:|---:|
| default | 2001 | 20 | 42 |
| a11y-extras | 2004 | 23 | 42 |
| html | 2116 | 135 | 42 |
| html+a11y | 2119 | 138 | 42 |

O observador deve registrar separadamente o inventário bruto. Se, como medido
na representação causal, o vanilla publicar `ppi` em `format.png.params` e o
cristalino publicar `format.png.ppi`, o bruto cristalino esperado sobe de 43
para 44 e a correção canônica coalesce **somente** os dois pares bilaterais
provados:

```text
format.svg.pretty <-> parametro vanilla homologo de format.svg
format.png.ppi    <-> parametro vanilla homologo de format.png
```

Assim o overlay subtrai exatamente dois aliases do raw cristalino e conserva
42 normalizado. Não coalescer `format.pdf`, `format.html`, sibling futuro,
prefixo semelhante ou rota não observada. Se a medição bilateral produzir
representação diferente, classificação é `Unknown`, o overlay não é ajustado
por conveniência e o contrato precisa ser reaberto antes do selo.

Contagem esperada bruta sob a representação acima:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY raw |
|---|---:|---:|---:|
| default | 2001 | 20 | 44 |
| a11y-extras | 2004 | 23 | 44 |
| html | 2116 | 135 | 44 |
| html+a11y | 2119 | 138 | 44 |

## 12. Controle positivo e matriz adversarial

`P00` é o controle positivo integral: todos os observáveis O01–O23 aplicáveis,
os quatro perfis, a matriz dimensional, os dois ramos de fontes, controles de
target, P1357 e V15/V26 devem terminar `Preserved`.

O denominador mínimo contém 18 mutantes válidos. Cada linha precisa terminar
`Violated` com testemunha pública; sobrevivente ou `Unknown` bloqueia selo.

| ID | Mutação negativa | Testemunha discriminatória mínima |
|---|---|---|
| M01 | omitir `png`, nome errado ou publicar módulo em vez de função | tipo/nome/descoberta nos quatro perfis |
| M02 | default/namespace `2`, `72`, integer ou tipo errado | `repr((type(format.png), format.png, format.png.ppi))` |
| M03 | permitir construção manual | `format.png()` deve falhar |
| M04 | aceitar/ignorar named, positional, duplicado ou tipo inválido | matriz de diagnósticos da seção 4 |
| M05 | regra vazia escreve `144` | `72` seguido de regra vazia permanece `72` |
| M06 | first-write-wins ou ordem invertida | pares `72→288` e `288→72` |
| M07 | aceitar nested ou rejeitar root | root positivo + dois containers negativos sem artefato |
| M08 | colapsar `None` em L2 ou L4 | documento `ppi:72` sem flag produz `72×72` |
| M09 | dividir o documento por 72 | documento `ppi:72` não pode produzir `1×1` |
| M10 | deixar de dividir a CLI por 72 | `--ppi 72` não pode produzir `72×72` |
| M11 | documento vence CLI explícito | documento `288` + CLI `72` produz `1×1` |
| M12 | aplicar só no ramo com fontes ou só no ramo sem fontes | dimensões iguais entre ramos equivalentes |
| M13 | vazar PPI para PDF/SVG/HTML | controles bilaterais de target |
| M14 | antecipar `format.pdf`/`format.html` ou falsificar P1357 | ausência de siblings + inventário bruto/normalizado |
| M15 | corromper owner ou hash de Prompt | V15 e header canônico |
| M16 | código aponta diretamente ao Núcleo | V26 e inspeção de linhagem |
| M17 | Núcleo muda sem ambos os pins consumidores | V26 sobre `pipeline.md` e `wiring.md` |
| M18 | alterar `format.svg` ao acrescentar PNG | matriz P1462 integral |

Mutantes que não compilam por erro não relacionado, não exercitam a mutação ou
alteram escopo fora de P1463 são inválidos e não entram no denominador. A
invalidade precisa de justificativa anterior ao resultado; não pode ser usada
para remover sobrevivente.

O score é:

```text
mutation_score = mutantes validos Violated / mutantes validos
```

O pré-selo exige `18/18 == 1.0`, `P00 == Preserved`, zero `Unknown` semântico,
P1357 conclusivo, V15/V26 verdes e todos os pins intactos. Um vetor completo
nessas condições autoriza o candidato conforme ADR-0131; repetição universal
pré-candidato não é requisito.

## 13. Política de estados e checkpoints

- `Preserved`: unidade positiva concluída que satisfaz integralmente seus
  observáveis e controles.
- `Violated`: mutante válido rejeitado, ou candidato divergente, sempre com
  testemunha pública.
- `Unknown`: unidade concluída mas opaca/ambígua, timeout do caso, crash/OOM do
  produto, PNG inválido, identidade não pinada ou evidência insuficiente.
- `Interrupted`: execução não concluída por causa externa comprovada.
- `Pending`: unidade ainda não iniciada.

Somente `Preserved` dá crédito a positivo e somente `Violated` dá crédito à
rejeição de mutante. `Unknown`, `Interrupted` e `Pending` nunca são convertidos
em sucesso. `Interrupted` não apaga unidades concluídas.

Cada unidade concluída persiste recibo atômico com hashes de manifesto,
contrato, L0 semântico, L0 raw da fase, baseline, oráculo, mutante e binário;
comando, ordem, ambiente, timestamps, exit, classificação, testemunha e hashes
de outputs. Se já houve resselo, o recibo referencia também o recibo R1 de
metadata. Após
interrupção, auditar pins, descartar somente a unidade parcial e retomar na
primeira unidade incompleta em diretório temporário novo.

## 14. Budget e rendimento discriminatório

Valem os limites exatos do manifesto: 180 minutos de computação ativa
pré-selo, um vetor completo pré-candidato, até três revisões totais antes de
revisão humana e parada após duas revisões consecutivas sem ganho
discriminatório. Antes de rerodar o vetor, executar apenas casos afetados e
controles de fronteira.

Budget esgotado impede iniciar unidade nova, mas não apaga prova, não reduz o
denominador e não reclassifica `Interrupted`/`Pending` como `Unknown`. Duas
revisões sem ganho, regressão positivo/opaco ou informação pública insuficiente
reabrem o desenho; não autorizam afrouxar predicado.

Na certificação final, repetir por risco ao menos P00, assimetria documento/CLI,
precedência, ambos os ramos de fontes, nested, neutralidade de target,
preservação P1462, P1357 nos quatro perfis e V15/V26. Ordem divergente sob os
mesmos pins é `Unknown` até explicação causal.

## 15. Gates finais e limite do veredito

Depois de selo, RED independente e candidato, a autoridade verificadora deve
registrar comandos, estado Git e hashes para:

```text
cargo test -p typst-core <testes focais P1463>
cargo test -p typst-shell <testes focais de --ppi>
cargo test -p typst-infra <testes focais P1463>
cargo test -p typst-wiring --test p1463_format_png
cargo test --workspace
cargo build
crystalline-lint .
git diff --check
```

Executam-se ainda os probes bilaterais, P1357 nos quatro perfis e a repetição
dirigida por risco. Falhas históricas alheias precisam ser identificadas por
baseline e não recebem crédito verde nem podem mascarar regressão P1463.

O certificado, se todos os gates passarem, pode alegar somente:

> o fragmento `format.png`/`ppi` registrado, entre o baseline causal e o
> candidato pinado, preservou os observáveis congelados e rejeitou todos os
> mutantes válidos sob as condições registradas.

Ele não prova paridade PNG geral, multipágina, identidade de bytes, encoder,
`format.pdf`, `format.html`, não finitos ou segregação atestada.

## 16. Ambiguidades e condições de reabertura

Não há ambiguidade bloqueante para este congelamento. Há três limites
explícitos que bloqueiam fases posteriores se não forem satisfeitos:

1. a confirmação ADR-0127 foi reportada pelo orquestrador, não verificada
   independentemente por esta autoridade;
2. a forma bilateral exata do alias P1357 de `format.png.ppi` precisa ser
   registrada pelo autor independente de oráculos antes do selo; divergência
   da forma esperada produz `Unknown` e reabre o overlay;
3. nenhuma alegação de isolamento pode ser feita neste workspace compartilhado.

Também reabrem o contrato: necessidade real de novo owner/consumer, alteração
de fase, novo target, mudança de primeira página para multipágina, suporte a
não finitos, alteração do pin vanilla, ou evidência de que a assimetria
documento cru versus CLI `/72` mudou no upstream ratificado.
