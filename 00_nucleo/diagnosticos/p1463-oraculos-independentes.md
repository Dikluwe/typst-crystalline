# P1463 — oráculos independentes pré-candidato de `format.png`/`ppi`

**Estado:** ORÁCULOS CONGELADOS E CALIBRADOS; P1357 PRÉ-SELO CONCLUSIVO;
BASELINE RED; SEM SELO; SEM MUTATION SCORE; SEM CANDIDATO

**Papel:** autor dos oráculos, distinto do implementador, adversário e
verificador final

**Regime:** protocolo completo de materialização segregada

**Alegação proporcional:** **executado sem atestação de isolamento**

**Baseline causal:** `bf0a29cad5423f038cb51c8c8a079465a1d4ed65`

**Tree causal:** `3fa973a4461c438769b22c0971eccc041dade422`

**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

**Manifesto R1:** SHA-256
`4f087f97ea10ba298a97d9b1b0b32901d2538a06eb7cb311f8fe44ff6b20b739`

**Contrato R1:** SHA-256
`4a8baaf1aa23b6610205e1da1868d0f5577f25020379c26354537651337185da`

Nenhum patch candidato existia ou foi lido durante esta autoria. As leituras
de código cristalino foram pinadas ao baseline por `git show
bf0a29ca…:caminho`; os executáveis foram reconstruídos antes das sondas. Esta
autoridade escreveu persistentemente somente este documento e o JSON irmão.
Runner, fontes e outputs ficaram em `/tmp/p1463-oracles.7Xdbft`.

O workspace é compartilhado. Hashes e ordem causal provam identidade das
entradas usadas, não isolamento técnico. Durante a redação final apareceram
concorrentemente `p1463-mutantes.{md,json}` e
`04_wiring/tests/p1463_format_png.rs`; esta autoridade não os leu nem os usou.
O HEAD/tree e todos os pins de código/L0 permaneceram os do baseline.

## 1. Entradas e instrumentos pinados

Todos os hashes raw de Passo, oito L0s, Núcleo, cinco fontes vanilla e oito
consumers baseline coincidiram com o manifesto. Os pins mais diretamente
decisórios foram:

| Entrada | SHA-256 |
|---|---|
| Passo P1463 | `919fa5eb6a169acad0f304512eb62146be89ddaba91289980a1a6616972a8f8d` |
| `compiler/stdlib/format.md` | `892e1d56d20189a4eb0c95d13383d5517156533a886f6db263be20fc7aeafc37` |
| `compiler/eval/rules.md` | `94444882ec98bf6ad932513745b9c0e1834aabde2f5f6cb5410da3fbd755fed4` |
| `compiler/layout/format_options.md` | `9a4f4a41618dfa396aca8efe9301a718660055a0659cbce362f20a10e0dc0d79` |
| `entities/document_format_options.md` | `52dbeeec49bba1992fb6b86f936cb7943c0af10fd6148d78967b1a12f2df709f` |
| `shell/cli.md` | `4f0cf474c2baa41102867dbf06a8589047803b6a6a791b872d1266640d83c4b9` |
| `infra/pipeline.md` | `ea9c658e42c68ec807cc159bddb75b8efbbabca021cd210cebdf8626cca95829` |
| `wiring.md` | `7f9adfb551b0cc69dc619f76a141138080a1f70448b7c2a89c9c85aab829d70e` |
| L0 do teste futuro | `19a9dbdadc35949890ce45f691ee1ab4da2dbbc8a8747f496ac6dc8fe84b94b2` |
| Núcleo raw | `488b0699748cd2a662c0ac47b8dc3c1b801e3df7afc5e842c74f66be5b1d59e8` |
| Núcleo efetivo V26 | `68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11` |
| binário vanilla | `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` |
| binário baseline release | `003b527c908b5f9499ad5acf621504efa56940a92728eaa650d14b88a6959db4` |
| runner final em `/tmp` | `0bcb84648c8d83ab65cf54d836f435637a1c06ab7e1dc01004196651edb82bdb` |
| recibo final em `/tmp` | `8a6fd5b93f23d4b90cb5151aa109836bd9447ee4a17de9e89f3fab0e4f991f87` |

O baseline release foi construído com:

```console
cargo build --release -p typst-wiring --bin typst
```

Exit `0`, 46,34 s; o binário foi escrito em
`2026-09-19T21:18:55.973269326-03:00` e reportou, apenas como evidência
auxiliar, `typst 0.15.1 (bf0a29ca)`.

O enumerador cristalino ligado a esse mesmo core foi construído com:

```console
cargo build --release --manifest-path lab/surface-inventory/Cargo.toml
```

Exit `0`, 48,19 s; SHA-256
`c072bf3e8eaacf8160fc920291a59b0df631e51c28059f75ab7cf4598501ad39`.
O enumerador vanilla pinado tem SHA-256
`11f3eab4ccfadc8239cdfc7fecdc87822238e625ff5bf4f755f3eb3b4ef8da13`.

## 2. Instrumento, fixtures e custo

O corpus final foi executado com:

```console
python3 /tmp/p1463_oracle_probe.py
```

O recibo final cobre `149` eventos entre
`2026-09-20T00:24:46.727139+00:00` e
`2026-09-20T00:25:08.146395+00:00`. Houve zero timeout ou `Unknown`
acidental. Três controles opacos deliberados terminaram `Unknown` como
esperado. Cada processo registra argv, timestamps, exit, stdout/stderr e seus
hashes, fonte e artefato; PNG válido registra assinatura e IHDR.

Todas as fixtures raster usam **1 pt × 1 pt**, margem zero; nenhuma fixture
72 pt foi usada. A gramática exata do gerador efêmero foi:

```typ
// PAGE
#set page(width: 1pt, height: 1pt, margin: 0pt, fill: white)
// RECT — ramo sem fontes
#rect(width: 1pt, height: 1pt, fill: black)
// FONT — ramo com fonte resolvida, em lugar de RECT
#text(font: "DejaVu Sans", size: 0.5pt)[A]
```

`doc-V` prefixa `#set format.png(ppi: V)`; os casos de cascata concatenam as
regras na ordem nomeada. `nested-block` e `nested-box` põem a mesma regra
dentro desses containers. `multipage` põe `#pagebreak()` entre uma página de
1 pt e outra de 2 pt. O JSON irmão registra o SHA-256 de cada uma das 24
fontes efêmeras; os hashes mais usados são:

| Fixture | SHA-256 |
|---|---|
| `base-no-fonts` | `774165f90a5ce3fd77cb3f78da771cb3a430c0f9144dca011e147059eef056b5` |
| `base-with-fonts` | `81f03d1d5fda18f86eca9856103e844a222b4caa13fd90cc599344dabe9f637b` |
| `doc-72` | `c84115788a5991aef109925ea4cb070cfe65cca87341b849f25e165a6bc174eb` |
| `doc-72-with-fonts` | `f006d396c1cd8cc87134b21b8c5cad6e45c0f2231bc9b1247c65d62a3bf32934` |
| `empty-after-72` | `a7e828e46c9e79ebf7aac017a38d3289c4eaabb93c6a5852baec9aeee853ee39` |
| `cascade-72-288` | `1fc5358ec2aacac071c7817e1e088e45d491526a9651d1b956563cc1e03e1c9a` |
| `cascade-288-72` | `bdccad89d046a477de4075d94400f3a733b1f0fdc9bf07d0bbedd744b3ca5065` |
| `nested-block` | `35d90b481ff169768f8ee95b54c3dd6e075d2d9cd373308d134d22ab9766ff02` |
| `nested-box` | `aff0fe5978af7507c47a0ede5ddfe8c6a2aefddc1553bded9f5cd34f04981700` |
| `multipage` | `bcf5675c02ff3607baa5480871e48e88fc95ff60020b8b4388fcabf5e67cb601` |
| `html-base` | `5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03` |
| `html-doc72` | `332988ec4007a1f06e95d7fc0f8e65e19cd48525dc026c6f80e9b7c8fdc5ab1a` |

Houve duas revisões focalizadas dentro do teto contratual. R0 identificou que
a fixture paginada usada como controle HTML exercitava uma incompatibilidade
histórica alheia; não recebeu crédito. R1 trocou somente esse recorte por texto
HTML nativo e acrescentou `pdfinfo`/`pdftotext`, isolando a neutralidade sem
igualdade cega de PDF. R2 acrescentou os três opacos. Expected de PNG,
diagnósticos e P1357 não foi alterado em nenhuma revisão. Só R2 recebe crédito.

## 3. Medição antes da classificação

### 3.1 Superfície e identidade — O01–O03

Nos quatro perfis, o vanilla produziu exatamente:

```text
repr((type(format.png), format.png, format.png.ppi))
=> "(function, png, 144.0)"\n       [JSON]

alias + std + identidade
=> "(function, png, 144.0, true, true)"\n
```

`format.png()` terminou com exit `1`, mensagem
`cannot be constructed manually`, âncora impressa
`<input-expression>:1:0` e 12 carets. O baseline terminou com
`module format does not contain png`; logo é RED pela causa correta, não por
um erro lateral. `format.pdf` e `format.html` permaneceram ausentes no
cristalino nos quatro perfis.

### 3.2 Set rule, cascata e fronteira root — O04–O08

Os expected diagnósticos congelados são:

| Caso | Exit | Mensagem | âncora impressa | Artefato |
|---|---:|---|---|---|
| named desconhecido | 1 | `unexpected argument: nope` | `1:16`, 8 carets | ausente |
| string | 1 | `expected float, found string` | `1:21`, 4 carets | ausente |
| boolean | 1 | `expected float, found boolean` | `1:21`, 4 carets | ausente |
| positional | 1 | `unexpected argument` | `1:16`, 2 carets | ausente |
| duplicado | 1 | `duplicate argument: ppi` | `1:25`, 3 carets | ausente |
| `block` | 1 | `format set rules are not allowed inside of containers` | `2:8` | ausente |
| `box` | 1 | mesma | `2:6` | ausente |

Inteiro, float, zero e negativo finito foram aceitos pelo vanilla. A regra
vazia preservou `72`; `72→288` terminou `288`; `288→72` terminou `72`. O
baseline aceitou falsamente quase todos os casos pelo fallback, publicou PNG
2×2 e emitiu `set: target '' ainda não suportado`; isso é a testemunha RED de
O04–O08. O duplicado já falha no parser baseline, mas não salva a matriz como
um todo.

### 3.3 Carrier, CLI e resolução — O09–O13

O carrier público fica congelado por probe unitário independente, sem
inspecionar layout Rust:

```text
DocumentFormatOptions::default().svg.pretty == false
DocumentFormatOptions::default().png.ppi == 144.0
PngFormatOptions.ppi é f64 cru, não Option e não é dividido por 72
PngFormatOptions/DocumentFormatOptions: PartialEq; SvgFormatOptions: Eq permitido
```

O CLI vanilla anunciou `--ppi <PPI>` em `compile` e `watch`, com texto
`[default: 144]`; `eval`, `fonts` e `completions` não o anunciaram e o
rejeitaram com exit `2`. O futuro teste L2 deve estender a rejeição a `query`,
`info` e `init`, e provar `None` versus `Some(f64)` sem default nem `/72`.
`--ppi text` congelou exit `2` e:

```text
invalid value 'text' for '--ppi <PPI>': invalid float literal
```

O negativo válido usa `--ppi=-72`. O baseline não anuncia nem aceita a flag,
logo O10/O11 estão RED.

A assimetria foi medida pela dimensão pública, não inferida de tipos Rust:

```text
Some(cli_ppi) -> cli_ppi / 72.0
None          -> document.format_options.png.ppi
```

Documento 72 sem flag deu 72×72; `--ppi 72` sem regra deu 1×1; documento 288
com CLI 72 deu 1×1; documento 72 com CLI 288 deu 4×4. Isso mata tanto “dividir
o documento” como “passar CLI cru” e “documento vence caller”.

### 3.4 IHDR, limites, fontes e primeira página — O14–O17/O22

| Documento | CLI | IHDR esperado em página 1 pt |
|---|---|---:|
| ausente | ausente | `144 × 144` |
| `72` | ausente | `72 × 72` |
| `2` | ausente | `2 × 2` |
| `1.5` | ausente | `2 × 2` |
| `0` | ausente | `1 × 1` |
| `-72` | ausente | `1 × 1` |
| ausente | `72` | `1 × 1` |
| ausente | `288` | `4 × 4` |
| `288` | `72` | `1 × 1` |
| `72` | `288` | `4 × 4` |

Default e documento 72 deram dimensões idênticas com e sem fonte; CLI 72 no
ramo com fonte deu 1×1. O baseline usa 2×2 em ambos os ramos e, por isso,
preserva a simetria mecânica mas viola os valores P1463.

O caso multipágina singular mostra uma fronteira deliberadamente não-paritária:
o vanilla rejeita output sem template de página; o baseline cristalino publica
um único PNG da primeira página. O oráculo candidato preserva **um único PNG da
primeira página**, agora 144×144 pelo novo default. O17 não transforma isso em
alegação de paridade multipágina.

Todo sucesso PNG exige assinatura e IHDR válidos. Todo erro acima exige
artefato ausente. Compressão, chunks não-IHDR, metadata, tamanho e hash do PNG
não são expected.

### 3.5 Neutralidade e preservação P1462 — O18/O19

Os controles são relacionais dentro de cada produto:

- SVG: base, `--ppi 72`, documento 72 e ambos devem ter a mesma serialização;
  no vanilla os quatro SHA-256 foram
  `8c1f223efb6a0742ec1ab628c30adf9e617ccfaf6929125b4f30f0d3291ac4f0`;
- PDF: `pdfinfo` deve conservar páginas, tamanho da página, rotação e metadata
  selecionada; `pdftotext` deve conservar texto. Timestamps, creator, tamanho
  do arquivo e bytes não são expected;
- HTML: fonte target-native simples produziu os mesmos bytes nos quatro casos,
  SHA-256
  `ddfd4c85db1da3e2cc2e34836b0c8dc49960785d0c3af8e967636f6e84249d9e`;
  somente o warning experimental preexistente permanece;
- o baseline prova RED específico: rejeita `--ppi` e, sem flag, ainda vaza o
  warning legado da set rule. Não se atribui essa falha a PDF/SVG/HTML em si.

O controle P1462 baseline permaneceu discriminatório:

| Caso SVG | SHA-256 baseline |
|---|---|
| documento pretty `true` | `bc459259363f3331a6c52c63bb9d8939792951c80ee6cd0a5bc194ebe08489b1` |
| documento pretty `false` | `4371ca3b15d5a7061cd405f0985b85b90382eaba7d16fc51bd2ffbcb45423a98` |
| documento `false` + CLI `--pretty` | `bc459259363f3331a6c52c63bb9d8939792951c80ee6cd0a5bc194ebe08489b1` |

Depois do candidato, `pretty:true + --ppi 72` deve ser idêntico ao primeiro
caso. A matriz P1462 integral continua gate separado obrigatório; estes quatro
recortes não a substituem.

## 4. Resolução conclusiva da condição P1357 — O21

Esta condição ficou **conclusiva sem candidato**; não é `Unknown`.

### 4.1 Representação vanilla exata

Nos quatro perfis, o catálogo vanilla contém `entries["format.png"]` como
`kind: "function"`. A representação de `ppi` é exatamente o primeiro e único
item de `entries["format.png"].params`:

```json
{
  "name": "ppi",
  "default": "144.0",
  "positional": false,
  "named": true,
  "variadic": false,
  "required": false,
  "settable": true
}
```

O parent traz `source:
"crates/typst-render/src/format.rs:PngFormat"`, `owner_path: "format"` e
`owner_kind: "module"`. **Não existe** chave separada
`entries["format.png.ppi"]` no vanilla.

### 4.2 Rota cristalina por regra já observável

Não foi lido nem simulado um candidato. A conclusão usa três entradas
anteriores e independentes:

1. o L0 congelado exige `format.png` como `Func` com namespace contendo
   somente `ppi == Value::Float(144.0)`;
2. o enumerador cristalino pinado grava `params: null` para qualquer binding e,
   no braço genérico `Value::Func`, desce por `Func::namespace()` e concatena o
   nome do membro ao path do owner;
3. o baseline observado aplica essa mesma regra a `format.svg.pretty` nos
   quatro perfis, com owner `format.svg`, owner kind `function`.

Logo a entrada cristalina futura está congelada exatamente como:

```json
{
  "path": "format.png.ppi",
  "present": true,
  "availability": "active",
  "kind": "float",
  "params": null,
  "source": "runtime:std",
  "owner_path": "format.png",
  "owner_kind": "function",
  "slot_kind": "member",
  "access_form": "module_member",
  "structural_verified": true,
  "symbol": null
}
```

Essa conclusão seria refutada por `format.png` não ser `Func`, `ppi` não estar
em `Func::namespace`, tipo público não ser float, mudança do enumerador ou
divergência de perfil. Qualquer refutador produz `Unknown` bloqueante; não
autoriza adaptar o overlay.

### 4.3 Repetição, hashes e overlay

Os catálogos foram executados primeiro em ordem direta e depois em ordem
reversa (`html+a11y`, `html`, `a11y-extras`, `default`) entre
`2026-09-19T21:25:20.125524908-03:00` e
`2026-09-19T21:25:36.067420904-03:00`. `cmp` confirmou igualdade byte a byte:

| Perfil | catálogo vanilla SHA-256 | catálogo cristalino baseline SHA-256 |
|---|---|---|
| default | `c149c41ad694977539de77d82959d47a31e2b9907f37c3cc7b728925b6d0b8b3` | `a5ad6c744d4778e6eaba941ab317b654e6108187d34fbeb1447d87948e224beb` |
| a11y-extras | `3b759f97c883053c212d417474ccab9cc260188e4f26cef1258f805ad61ec323` | `678ebac5a173cd6610f46c2b86bdff641c72361616b8041c15c99a239ec91e36` |
| html | `4d2d8fcea084fb75563cb0ce408d5fe221139a0cbb1c99583d75e90cb830f109` | `ecdb852b2e2b30e34f5ac10e03711f95e8e24b697ff45d48352808fe4b66a6e8` |
| html+a11y | `13969ae4b5220961792a3c5ba2bf2fc43d6fe9b31256c8b443610689c87bd57c` | `ce302ddb739fccfb23802385aab6bb2ea97e398463c5d324633f0ced4fffa101` |

O baseline bruto reproduziu `2000/21/43`, `2003/24/43`, `2115/136/43` e
`2118/139/43`. Depois do candidato, o raw obrigatório é respectivamente
`2001/20/44`, `2004/23/44`, `2116/135/44` e `2119/138/44`.

O overlay coalesce somente:

```text
format.svg.pretty <-> entries["format.svg"].params[name=pretty]
format.png.ppi    <-> entries["format.png"].params[name=ppi]
```

Assim, o normalizado obrigatório é `2001/20/42`, `2004/23/42`,
`2116/135/42`, `2119/138/42`. Raw nunca é apagado e sibling nenhum é
normalizado por semelhança. A representação pré-selo está `Preserved`; as
contagens pós-candidato permanecem `Pending`, não `Preserved` antecipadamente.

## 5. Cobertura congelada O01–O23

| ID | Oráculo independente | Vanilla | Baseline causal |
|---|---|---|---|
| O01 | superfície, alias/std, quatro perfis, siblings | Preserved | Violated |
| O02 | `ppi` float 144.0 | Preserved | Violated |
| O03 | construção manual proibida | Preserved | Violated |
| O04 | integer/float/zero/negativo | Preserved | Violated |
| O05 | cinco diagnósticos, sem warning, sem artefato | Preserved | Violated |
| O06 | `72→empty` permanece 72 | Preserved | Violated |
| O07 | duas ordens last-write-wins | Preserved | Violated |
| O08 | root positivo + `block`/`box` negativos | Preserved | Violated |
| O09 | carrier/default/traits/valor cru | N/A upstream Rust | Violated; superfície ausente |
| O10 | help, `None`/`Some`, comandos alheios | Preserved | Violated |
| O11 | texto exit 2; negativo com `=` | Preserved | Violated |
| O12 | precedência caller/documento | Preserved | Violated |
| O13 | documento cru versus CLI `/72` | Preserved | Violated |
| O14 | sete dimensões centrais | Preserved | Violated |
| O15 | zero/negativo → 1×1 | Preserved | Violated |
| O16 | mesmos valores nos dois ramos de fonte | Preserved | simétrico, mas valor errado |
| O17 | primeira página única cristalina | N/A multipágina vanilla | Preserved no limite anterior |
| O18 | SVG/PDF/HTML neutros | Preserved | Violated pela flag ausente/warning legado |
| O19 | recortes P1462 + suíte integral futura | Preserved | Preserved antes de `--ppi` |
| O20 | 8 owners, pins, V15/V26 | N/A | Preserved; pós-candidato Pending |
| O21 | P1357 quatro perfis e overlay literal | Preserved pré-selo | Preserved pré-selo |
| O22 | exit/canais/Unicode/span/artefato/IHDR | Preserved | Violated nas obrigações novas |
| O23 | mecânica livre explicitamente excluída | Preserved pelo desenho | Preserved pelo desenho |

Às `2026-09-19T21:25:50.652981215-03:00` foi executado:

```console
crystalline-lint --checks v15,v26 --fail-on warning .
```

Exit `0`, `No violations found`, binário do linter SHA-256
`e47974fe903b225e96fec580040604d80380edda054a7b2187bbf5b18da9a30b`.
Isso prova o baseline de O20; o gate precisa ser repetido depois do candidato.

## 6. Opacos, `Unknown` e `Interrupted`

Três controles deliberados provaram fail-closed:

| Controle | Resultado esperado/observado | Reason code |
|---|---|---|
| PNG só com assinatura, sem IHDR | Unknown | `PNG_INVALID_OR_TRUNCATED` |
| SHA esperado artificialmente divergente | Unknown | `PRODUCT_IDENTITY_MISMATCH` |
| subprocesso com timeout de caso de 20 ms | Unknown | `CASE_TIMEOUT` |

O PNG truncado teve SHA-256
`4c4b6a3be1314ab86138bef4314dde022e600960d8689a2c8f8631802d20dab6`.
Nenhum opaco recebeu crédito positivo.

`Unknown` é reservado a unidade concluída mas semanticamente opaca: pin
ambíguo, parser/output insuficiente, crash/OOM, PNG inválido, não finito ou
timeout do próprio caso. `Interrupted` é unidade não concluída por encerramento
externo do agente, sessão ou host; não é classificação semântica, não recebe
crédito e não apaga checkpoints válidos. `Pending` nunca conta. Somente
`Preserved` credita positivo e somente `Violated` credita rejeição de mutante.

## 7. Resultado e limites

Os oráculos estão conclusivos para entrega ao adversário e ao verificador:
vanilla calibrou O01–O19/O22, o baseline ficou RED pelas causas P1463, controles
P1462/primeira página não regrediram, V15/V26 estão verdes e a forma P1357 foi
resolvida sem depender de candidato. Não há ambiguidade bloqueante nesta
autoria e não houve `Unknown` acidental.

Este artefato **não** emite selo, não executa M01–M18, não calcula score, não
autoriza por si só candidato e não antecipa contagens pós-candidato. Uma
autoridade distinta ainda precisa obter P00 `Preserved`, M01–M18 `Violated`,
score `18/18 == 1.0`, zero `Unknown` semântico, P1357 pós-candidato exato e
V15/V26 verdes. O veredito continua limitado ao fragmento `format.png`/`ppi`;
não prova paridade PNG geral, multipágina, encoder, bytes PNG, não finitos,
`format.pdf`, `format.html` nem isolamento técnico.
