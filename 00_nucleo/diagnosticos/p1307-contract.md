# P1307 — contrato observado e reabertura necessária

Estado: **P1307_REDESIGN_REQUIRED**. Este é um diagnóstico/contrato candidato,
não um Prompt L0, selo de implementação ou autorização para escrever código.
O gate ADR-0127 não está pronto: há perda de informação fora dos quatro owners
autorizados e observáveis contextuais obrigatórios sem medição bilateral válida.
Regime: executado sem atestação de isolamento técnico; autoria do contrato
separada da autoria do oracle, sem leitura de candidato P1307.

## Medição anterior à classificação

Em `2026-09-07T16:07:54.426421+00:00`–`16:10:04.623831+00:00`, sobre HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` mais o working tree P1306
certificado, o script de observação executou 350 casos em quatro perfis,
três ordens e dois binários: 8.400 execuções. O diff tracked integral antes e
depois é idêntico. A lista exata de ficheiros alterados, `git diff HEAD --stat`,
diff, status, timestamps, argv/cwd, ambiente e fixtures estão nos recibos abaixo.
Os únicos tracked previamente alterados são os dois owners e dois consumers
de `compiler/eval/bindings/field_access` e `compiler/eval/tests` do P1306.

| Entrada/recibo | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1307-preflight.json` | `f928436a6ce7e37e16c9a3b9f160480593efa8f9fa05d72dc2cdec836716e938` |
| `00_nucleo/diagnosticos/p1307-baseline.json` | `418443cb00f8733ae034cbf098662869835a1235f2112ef423f900cf9b56e48e` |
| `00_nucleo/diagnosticos/p1307-contract-measure.py` | `70ee55e1fc2e5eaae8392182cf0b3dae0110a02d0810d536847528003d1e22a7` |
| `00_nucleo/diagnosticos/p1307-contract-measurement.json` | `f1191d3de029dabf6f66f87872bee8146aa43106ae54e84facc22952afd238d0` |
| Baseline fresco `/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst` | `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3` |
| Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

As ordens são normal, repetição integral e inversão integral da lista de
submissões, com quatro processos concorrentes limitados. O recibo retém ordem
de submissão, início/fim individual e outputs integrais; não alega execução
estritamente serial. Os quatro perfis usam explicitamente ausência de features,
`html`, `a11y-extras` e `html,a11y-extras`.

| Lado | Valor observado | Diagnóstico de linguagem | Unknown |
|---|---:|---:|---:|
| Vanilla | 3.480 | 720 | 0 |
| Baseline | 768 | 3.336 | 96 |

Não houve diferença por repetição/ordem em `(exit, stdout, stderr)`. Nas
execuções válidas não houve diferença por perfil. As 72 diferenças textuais
entre perfis pertencem exclusivamente ao texto de usage da opção inválida
`--in`; não são divergência de disponibilidade dos encoders. Estes números
descrevem observações, não contagem de paridade ou veredito de implementação.

O recibo preserva a tentativa focal vanilla anterior (328 casos), inclusive
o erro inicial do adaptador que subtraía um à coluna da CLI `eval`.
A coluna publicada é zero-based. A revisão focal corrigiu os derivados para
`json.encode()` → `0..13`, `(x: (none,))` → `12..24` e o positional TOML
multilinha → `34..48`; stdout/stderr da tentativa anterior permanecem intactos.
Preserva também a tentativa de execução no sandbox cujo `/dev/shm` privado
não via o binário do host. A execução válida usou acesso revisto ao tmpfs e
verificou os hashes antes do corpus. Falha de infraestrutura não é RED.

Cada caso contém expressão literal, SHA-256 e source numerizada. A lista
`diagnostic.messages` é somente uma projeção da primeira linha de cada
mensagem; para mensagens multilinha de decoders, o observável integral é
`stderr`, sempre preservado. Ausência de anchors num diagnóstico detached
preexistente não é prova de span resolvível nem erro do encoder ausente.

## Partição observacional da fonte

A fonte ratificada `foundations/value.rs:343-364` seleciona primitivas,
Bytes, Symbol, Content, Array e Dict; os demais valores usam a sua `repr`
pública. `foundations/symbol.rs:394-401` serializa o texto do símbolo;
`foundations/content/mod.rs:709-719` serializa mapa com `func` primeiro e
os fields do elemento em seguida. `foundations/bytes.rs:362-374` distingue
formato human-readable de byte-string. Os caminhos completos e seus hashes
estão em `measurement.sources`; são fontes do vanilla em
`lab/typst-original/crates/typst-library/src/`.

O inventário cristalino foi lido em `01_core/src/entities/value.rs:31-177`.
As classes atuais, com constructor literal individualizado no recibo, são:

- estruturadas: None, Bool, Int, Float, Str, Array, Dict, Bytes, Symbol,
  Content; LocatedContent é também conteúdo da linguagem, embora tenha
  carrier cristalino distinto;
- fallback textual: Module, Datetime, Func, Auto, Length, Relative, Ratio,
  Angle, Color, Stroke, Fraction, Align, Location, Gradient, Regex, Tiling,
  Decimal, Duration, Version, Selector, Args, State, Counter, Label, Dir,
  Path e Type.

Os 36 constructors sem contexto executaram com sucesso bilateralmente.
Location e LocatedContent foram construídos no vanilla por
`query(<probe>).first().location()` e `query(<probe>).first()`, no documento
literal congelado `context.typ`. A fonte cristalina
`compiler/stdlib/foundations/query.rs:47-63,79-80` confirma que os carriers
existem e são produzidos pela introspecção; isso não substitui uma medição
bilateral válida. No baseline, `eval --in context.typ` é opção inexistente,
exit 2. Os oito casos contextuais × quatro perfis × três ordens são os
96 Unknown obrigatórios. Não chamar isto ausência do tipo na linguagem.

Esta partição é semântica/morfologia de linguagem (ADR-0107). Os bytes da
`Str` retornada são seu valor público; estrutura do serializer, árvore Rust,
ponteiros e algoritmo interno não são critério. Os comentários `#[func]` de
`loading/json.rs:131-151`, `loading/toml.rs:104-126` e
`loading/yaml.rs:108-123` declaram a finalidade e assinaturas. A emissão
específica foi medida; não se infere intenção histórica do autor a partir
de um detalhe do emitter.

## Contrato público candidato derivado

As três assinaturas vanilla são:

```typst
json.encode(value, pretty: true) -> str
toml.encode(value, pretty: true) -> str // value deve ser dictionary
yaml.encode(value) -> str
```

`json`, `toml` e `yaml` continuam funções de decode. Cada namespace contém
o membro público `encode`, kind function, repr `encode`; os três faltam no
baseline nos quatro perfis. `cbor.encode` já mede kind function e repr
`encode` bilateralmente, apesar de nomes internos históricos na fonte.
`csv.encode`, `xml.encode`, `read.encode` e suas rotas parent `.with()`
continuam ausentes; não há autorização para criá-los.

Os resultados de cada caso são strings integrais congeladas no recibo.
As seguintes classes explicam as diferenças que o oracle deve discriminar:

| Classe | JSON | TOML dentro de dict | YAML |
|---|---|---|---|
| `none` | `null` | campo omitido; dict só com none retorna string vazia | `null` + newline |
| `none` em array | `null` no array | erro `unsupported None value` | `null` no array |
| NaN / inf / -inf | `null` | `nan` / `inf` / `-inf` | `.nan` / `.inf` / `-.inf` |
| `-0.0` | conserva sinal e `.0` | conserva sinal e `.0` | conserva sinal e `.0` |
| Bytes | string `bytes(N)` | string `bytes(N)` | scalar textual `bytes(N)` |
| Symbol | sequência Unicode completa | sequência Unicode completa | sequência Unicode completa |
| Content `[hi]` | mapa `func: text`, `text: hi` | table com esses fields | mapa com esses fields |
| opaque | string da repr pública | string da repr pública | scalar textual da repr pública |

JSON default e `pretty:true` usam indentação de dois espaços, sem newline
terminal; `pretty:false` é compacto. TOML default e `pretty:true` expandem
arrays não vazios com quatro espaços, vírgulas e newline; `pretty:false`
tem a forma compacta medida. TOML retorna newline terminal para documentos
não vazios e string vazia para dict vazio. YAML retorna newline terminal e
rejeita qualquer named, inclusive `pretty:false`.

A ordem de dict deve seguir a ordem publicamente observada do formato:
JSON/YAML preservam inserção. TOML emite os campos escalares antes das tables,
mantendo a ordem relativa dentro dessas classes. A testemunha
`(z: (q: 1), a: 2, y: (v: 3), b: 4)` emite `a`, `b`, `[z]`, `[y]`;
exigir ordem global z,a,y,b seria um oracle incorreto.

Strings cobrem vazio, Unicode, aspas, backslash, tab, CR, NUL, backspace,
form-feed, multiline e quebras iniciais/finais. YAML distingue strings
parecidas com null/bool/número, quoting simples e block scalar com indicadores
de indentação/chomping; por exemplo `"a\nb\n"` produz `|\n  a\n  b\n`.
Content inclui vazio, strong, sequence e conteúdo de introspecção; Symbol
inclui modificador. Arrays/dicts vazios, nesting, arrays heterogêneos e limites
numéricos também estão individualizados. Nenhum formato pode herdar
acidentalmente a restrição de dict no topo do TOML.

Parent `.with()` mantém acesso ao membro sem transmitir seus argumentos ao
encoder: até `json.with(nope:9).encode(v)` é válido. No próprio encoder,
`.with()` vazio, positional pré-ligado e named JSON/TOML `pretty:false`
funcionam; o named explícito da chamada final sobrepõe o pré-ligado.
`.with()` retorna função com repr `encode`. A validação é adiada até chamar:
named inválido/YAML pretty falha apontando ao argumento da ligação original.
Missing e excesso continuam erros. As rotas foram medidas separadamente;
não foram inferidas do precedente de `place.flush`.

O diagnóstico de aceitação deve conservar stdout vazio, exit, severidade,
mensagem completa, quantidade, hints ordenados, laterais e range UTF-8 half-open.
Exemplos congelados:

- `json.encode()` → `missing argument: value`, `0..13`;
- `json.encode(value: v)` → `the argument \`value\` is positional`,
  hint único `try removing \`value:\``, named completo;
- `pretty: "no"` → `expected boolean, found string`, somente o valor;
- excesso positional → `unexpected argument`, somente o positional extra;
- named desconhecido → `unexpected argument: <nome>`, named completo;
- TOML não-dict → `expected dictionary, found <tipo>`, valor positional;
- TOML array com none →
  `failed to encode value as TOML (unsupported None value)`, valor positional;
- `yaml.encode.with(pretty:false)(v)` → `unexpected argument: pretty`,
  argumento pré-ligado `17..30` na fixture literal medida.

Round-trip é controle adicional somente para dados representáveis. Não
substitui comparação da Str: emissores diferentes podem decodificar igual.

## Fronteiras refutadas pela medição

### R1 — fallback canônico não é paritário para todas as classes exigidas

Nos quatro perfis e três ordens, `construct-State` mede
`state("probe", 0)` no vanilla e `state(...)` no baseline;
`construct-Counter` mede `counter(heading)` e `counter(...)`.
A causa de fonte está em `01_core/src/compiler/eval/repr.rs:124-125`;
o helper público de serialização apenas delega a esse formatter.
Também `repr.rs:87` fixa `location(...)`, contra `location(..)` observado
no vanilla; a falta de rota contextual bilateral impede chamar a última
testemunha de paridade medida dos dois executáveis.

A inferência refutável é que reutilizar inalterado esse helper nos encoders
propagaria as diferenças de State/Counter. Uma execução bilateral do mesmo
Value pelo helper que devolvesse a repr vanilla refutaria essa inferência;
os controles atuais mostram o contrário. Duplicar formatter ou reconhecer
fixtures em loading não é uma correção autorizada de ownership.

Reabertura mínima a auditar: owner `compiler/eval/repr.md` e consumer
`01_core/src/compiler/eval/repr.rs`, para a morfologia pública dos tipos
efetivamente divergentes. Seu L0 vigente, lido integralmente e SHA-256
`c5169f71ee6a0ece8cc2fc56ca9cecd07443a339c460b6ae63f15638c436eccd`,
já exige formatter canônico e proíbe `Debug`. Não foi editado.

### R2 — argumento pré-ligado perde sua origem antes de loading

`01_core/src/entities/args.rs:20-32` guarda valores e um único span agregado.
`compiler/eval/call_dispatch.rs:432-473` aplica `With` por merge recursivo;
`:1016-1023` escolhe `new.span` e descarta `pre.span`. A nativa recebe os
valores fundidos e nenhuma origem individual do argumento pré-ligado.
As capturas atuais por argumento em `:1290-1310,1540-1598` só atendem
collections e identidades específicas existentes, não a nova família.

O erro vanilla na ligação original de `yaml.encode.with(pretty:false)`
demonstra um observável que não está no Args final entregue ao serializer.
Não é legítimo inventar span detached, selecionar toda a chamada ou inferir
origem por igualdade dos valores. O problema antecede escolha de emitter.

Reabertura mínima a auditar: owner `compiler/eval/call_dispatch.md` e
consumer `01_core/src/compiler/eval/call_dispatch.rs`, incluindo retenção
da origem antes da fusão e passagem da âncora correta. Seu L0 foi lido
integralmente, SHA-256
`e09f8d04c91c009af2bacadd2d6a06b3b314e07b42322b0e84370e7c1767f9b1`.
Os precedentes privados existentes proíbem ampliar silenciosamente Args
e limitam a captura às identidades enumeradas.

Não está provado que seja inevitável mudar `entities::Args` ou `Func`.
Os owners `entities/args.md` (SHA-256
`3a33f1f2628348e3484b8346ab55aa82474b789e6836c62303b24b43297f5d56`)
e `entities/func.md` (SHA-256
`52b6f6e7ca7714027d4e3a883f6e5ae888a48e4a5c67e45651b63b86d9811908`)
foram lidos integralmente para verificar a fronteira vigente. Um redesenho
que exija carrier persistente, campo ou assinatura pública terá de medir
todos os consumidores e obter aprovação explícita adicional. Não se propõe
aqui uma nova estrutura sem essa prova.

### R3 — Location/LocatedContent continuam Unknown bilateral

O corpus preserva os 96 exit 2 por `--in` inexistente no baseline.
Uma possível rota documental por `query` ainda requer desenho e controle
focal das quatro features; somente o help foi inspecionado. Não há prova
de equivalência dessa rota e não se autoriza ampliar a CLI. Por instrução
do coordenador, a execução obtida foi preservada sem nova repetição integral
após o bloqueio. É necessário resolver a observabilidade antes do selo.

## Controles preexistentes e limites da prova

Os sete decoders foram medidos por chamada direta, `.with`, identidade,
missing e um erro de leitura/parsing/argumento por formato. CBOR tem identidade,
encode, decode de retorno, Symbol, Content e negativos próprios.
O baseline tem diferenças anteriores que a futura mudança deve preservar
individualmente: XML omite `namespace:null`; missing/parse de vários
decoders usam mensagens próprias e spans detached; CBOR Symbol/Content usam
fallback textual (scope-out já declarado em `compiler/stdlib/loading.md`).
Nenhuma dessas diferenças pode ser convertida em obrigação de correção
silenciosa ou apagada genericamente do comparador.

O transporte CLI JSON de Bytes publica `bytes(N)`. Assim, os controles
CBOR atuais observam comprimentos, estrutura decodificada e diagnósticos,
mas não fixam cada byte do buffer. Um oracle futuro de preservação integral
deve congelar também o array de bytes ou outra projeção pública completa;
não alegar que a representação `bytes(N)` prova igualdade do payload.

Os quatro L0 originais continuam byte-idênticos ao baseline P0; nenhum
consumer, header, teste RED, Cargo ou artefato P1306 foi editado por este
autor. Não foi emitido selo pronto nem alegada mutação morta. O contrato
candidato conserva as obrigações medidas; a reabertura não reduz a barra
nem transforma Unknown em Preserved. Antes de P3, resolver a autorização
dos owners adicionais e a observabilidade; só então revisar L0, congelar
oracle e apresentar ao dono um gate ADR-0127 materializável.
