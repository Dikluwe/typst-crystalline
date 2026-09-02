# Passo 1293 — fecho acionável do perfil HTML e nomes públicos de namespaces

**Estado:** pronto para execução, sujeito ao gate L0/ADR-0127

**Data de autoria:** 2026-09-01

**Alvo de paridade:** vanilla ratificado `a51e02804`

**Regime:** materialização Tekt segregada completa

**Meta observável:** eliminar os 12 membros ausentes e as 3 diferenças de nome
público amostradas pelo perfil HTML, sem remover extensões cristalinas nem ligar
features fora do perfil

---

## 1. Resultado pretendido

P1292 fechou os quatro resíduos acionáveis da superfície default. A sua
certificação final registra `111 total / 99 MATCH / 12 justificadas`, sem
lacuna nominal restante naquele perfil.

O perfil HTML ainda contém 26 resultados não equivalentes:

- 1 binding corretamente oculto por feature: `pdf.data-cell`;
- 10 bindings extras cristalinos, que não devem ser removidos neste passo;
- 12 membros realmente ausentes;
- 3 membros presentes cujo `repr` revela nomes internos incorretos.

P1293 fecha as duas últimas categorias em quatro lotes independentes:

| Lote | Obrigação | Quantidade amostrada |
|---|---|---:|
| A | `float.is-nan` estático e ligado | 1 |
| B | `math.attach`, `math.binom`, `math.mono`, `math.script` | 4 |
| C | `html.button`, `html.col`, `html.iframe`, `html.select`, `html.template`, `html.video`, `html.wbr` | 7 |
| D | nomes públicos curtos de `grid.*` e `table.*` | 3 no inventário, 10 siblings no contrato |

Se os quatro lotes forem aprovados, a superfície HTML fixa de P1292 deve ir de
`89/115` para `104/115 MATCH`, restando somente 1 feature desligada e 10 extras
deliberados. A superfície default fixa deve preservar `99/111` e as mesmas 12
justificativas.

Este passo fecha um fragmento de superfície. Não declara paridade global do
compilador.

---

## 2. Proveniência da decisão

### 2.1 Estado de autoria

- instante: `2026-09-01T13:06:42-03:00`;
- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- árvore não commitada;
- estado exato:
  `00_nucleo/diagnosticos/p1293-authoring-baseline-status.txt`;
- SHA-256 do manifesto de estado:
  `a0b31236c089623b0cd757d1401a6e1a1b90a931cbe48e78eda009ee72087b8e`;
- SHA-256 de `git status --short`:
  `c230dcc0640c02d614315f290c7f19067c898995bee1973a67de0a2e4be0e2c8`;
- SHA-256 de `git diff HEAD --stat`:
  `913e2e7d9f17bc2ef6c43cf9cd23ae8af055c27c9226e58a124bb61073bcd4b3`.

Artefatos medidos:

| Artefato | SHA-256 |
|---|---|
| `target/release/typst` | `ed5f85e03e6fe473c1a7a9a42cceafe8c5fbe075de4dad56a7c88c07ad2c3b6f` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| `p1292-surface-default.json` | `a827b192fc38541f9b00baa3b666fc94aa2ff6f691ca7867bf28aa3a451e5f15` |
| `p1292-surface-html.json` | `646884f8268f74e0acab79cf19e82fc81cceebf9e4258daeeb3ec2474d0f950a` |
| `p1292-final-certificate.json` | `2450600a111a23ab9318fedd3fb5610531a4d0e35dcfcf2503d7271c233346b9` |

Esses números são baseline de autoria, não prova final. A execução deve
recapturar árvore, binários e ambas as superfícies antes de editar L0.

### 2.2 Evidência de língua já reproduzida

No vanilla pinado:

```text
float.is-nan                  -> function "is-nan"
float.is-nan(float("NaN"))   -> true
float.is-nan(1.0)            -> false
float("NaN").is-nan()        -> true

math.attach([x])              -> attach(base: [x])
math.binom([n], [k])          -> binom(upper: [n], lower: ([k],))
math.mono([x])                -> styled(child: [x], ..)
math.script([x])              -> styled(child: [x], ..)

html.col(span: 2)             -> elem(tag: "col", attrs: (span: "2"))
html.wbr()                    -> elem(tag: "wbr")
```

Os sete membros HTML são funções. `col` e `wbr` são void e não recebem body;
os outros cinco recebem body content posicional opcional. As sondas
representativas preservaram atributos específicos, ordem, body e a morfologia
genérica `HtmlElem`.

O vanilla representa os dez siblings de namespace assim:

```text
grid.cell grid.header grid.footer grid.hline grid.vline
  -> cell header footer hline vline

table.cell table.header table.footer table.hline table.vline
  -> cell header footer hline vline
```

O cristalino atual usa `grid_cell`, `grid_header`, ..., `table_vline`. Essa é
diferença pública de `repr`, não mera metadata interna.

---

## 3. Medir antes de fechar o contrato

O medidor independente deve produzir
`p1293-vanilla-measurement-receipt.md` antes de qualquer alteração normativa.
Além da presença nominal, deve congelar:

1. assinatura, defaults, aridade, casts, named desconhecidos e spans de A–D;
2. `repr` de valores default e explicitamente fornecidos;
3. igualdade semântica entre formas sintáticas math e funções qualificadas;
4. layout de `attach`, `binom`, `mono` e `script` em estilos relevantes;
5. DOM HTML produzido por cada tag, com body, atributos e nesting;
6. rejeição de body em `col`/`wbr` e de named não pertencente à tag;
7. comportamento com `Feature::Html` desligada, ligada com target paged e
   ligada com target HTML;
8. identidade e chamada das subfunções grid/table antes e depois de corrigir
   somente seu nome público.

Para os sete constructors HTML, o inventário já fixa os nomes dos atributos
específicos:

| Tag | Específicos, além dos 76 globais já materializados |
|---|---|
| `button` | `command`, `commandfor`, `disabled`, `form`, `formaction`, `formenctype`, `formmethod`, `formnovalidate`, `formtarget`, `name`, `popovertarget`, `popovertargetaction`, `type`, `value` |
| `col` | `span` |
| `iframe` | `allow`, `allowfullscreen`, `height`, `loading`, `name`, `referrerpolicy`, `sandbox`, `src`, `srcdoc`, `width` |
| `select` | `autocomplete`, `disabled`, `form`, `multiple`, `name`, `required`, `size` |
| `template` | `shadowrootclonable`, `shadowrootcustomelementregistry`, `shadowrootdelegatesfocus`, `shadowrootmode`, `shadowrootserializable` |
| `video` | `autoplay`, `controls`, `crossorigin`, `height`, `loop`, `muted`, `playsinline`, `poster`, `preload`, `src`, `width` |
| `wbr` | nenhum |

O inventário prova nomes e posição, não prova sozinho os casts. Tipos, enums,
presence e serialização devem vir da fonte pinada ou de sondas bilaterais. Não
inferir tipos da especificação web nem aceitar todo valor como string.

---

## 4. Obrigações por lote

### O-A — `float.is-nan`

Estender o owner já existente de `float.is-infinite`, preservando a mesma
arquitetura fechada:

- `float.is-nan` é função de nome público `is-nan`;
- forma estática aceita `Float` e coerção de `Int` medida;
- forma ligada `(value).is-nan()` delega à mesma nativa;
- retorna `true` somente para NaN, e `false` para finitos e `±infinito`;
- aridade e named são fechados;
- obter o método ligado como valor sem chamada continua ausente se a nova
  medição confirmar o comportamento irmão de `is-infinite`.

Não criar trait, registry ou segunda fórmula em field access/call dispatch.

### O-B — quatro membros math

#### `math.attach`

Expor o `MathAttachElem` canônico com `base` obrigatório e os seis slots
opcionais `t`, `b`, `tl`, `bl`, `tr`, `br`. O L0 antigo de
`entities/elements/math_attach.md` ainda descreve somente cinco campos e deve
ser corrigido antes de qualquer binding. O novo L0 precisa refletir o consumer
real de sete slots, traversal, igualdade, `repr` e layout vigente.

Syntactic `attach(...)` e `math.attach(...)` devem convergir para o mesmo
construtor puro. Named explícito `none`, se medido como ausência, não pode virar
conteúdo textual nem slot presente.

#### `math.binom`

Expor upper obrigatório e lower posicional variádico com pelo menos um item.
O resultado usa a fração canônica sem barra, envolvida por parênteses
esticados. A sintaxe math e a função qualificada convergem; não usar Matrix
como aproximação e não perder `MathFracElem.line = false` em transformações.

#### `math.mono` e `math.script`

Registrar no módulo as nativas já donas em `stdlib/math_style.rs`, sem wrappers
duplicados. `mono` exige body. `script` exige body e aceita somente
`cramped: bool`, default `true`. A identidade pública dentro de `math` é curta
(`mono`, `script`) mesmo que o mesmo function pointer seja reutilizado.

### O-C — sete tags HTML tipadas

Acrescentar os sete constructors ao dispatcher estático de
`compiler/stdlib/html.rs`, reutilizando `HtmlElem`, `HtmlAttrs`, `HtmlBody`, os
76 casts globais e os helpers existentes.

- `button`, `iframe`, `select`, `template` e `video`: body opcional,
  `HtmlBody::None` quando omitido;
- `col` e `wbr`: void, `HtmlBody::Unset`, qualquer posicional é erro;
- atributo omitido não é serializado;
- booleano Presence verdadeiro vira atributo vazio e falso é omitido;
- named desconhecido, `data-*` e atributo específico de outra tag são erro;
- ordem dos atributos segue a chamada após omissões;
- o exporter genérico deve emitir DOM equivalente, inclusive `<col>` e
  `<wbr>` sem end tag.

Este lote não cria nova variante de `Content`, não duplica `HtmlElem`, não
habilita a feature por target e não materializa `html.frame` ou as tags fora da
lista.

### O-D — nomes públicos grid/table

Nas instâncias anexadas aos namespaces, usar exatamente os nomes curtos:

```text
cell header footer hline vline
```

para ambos `grid` e `table`. As nativas, function pointers, argumentos,
conteúdo retornado e bindings globais extras (`grid_cell`, `table_header`,
etc.) não mudam neste passo. Não renomear símbolos Rust nem usar o nome do
binding global como `repr` do membro qualificado.

O inventário atual amostra três desses dez nomes. O contrato cobre a família
inteira porque a mesma tabela e a mesma causa atingem todos os siblings.

---

## 5. Nucleação e gate ADR-0127

### 5.1 L0s a auditar/atualizar

- `00_nucleo/prompts/compiler/stdlib/foundations/float.md`;
- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/eval/math.md`;
- `00_nucleo/prompts/entities/elements/math_attach.md`;
- `00_nucleo/prompts/entities/elements/math_frac.md`;
- `00_nucleo/prompts/compiler/math/layout/attach.md`;
- `00_nucleo/prompts/compiler/math/layout/frac.md`;
- `00_nucleo/prompts/compiler/stdlib/math_style.md`;
- `00_nucleo/prompts/compiler/stdlib/html.md`;
- `00_nucleo/prompts/entities/html.md`, somente se a medição exigir mudança de
  entidade — a hipótese inicial é que não exige;
- `00_nucleo/prompts/compiler/eval.md`;
- `00_nucleo/prompts/compiler/eval/table.md` e
  `00_nucleo/prompts/compiler/stdlib/structural/table_grid.md`, apenas para
  manter a descrição pública dos namespaces coerente com seus owners.

Antes de ressellar, executar V15/V26 e registrar a matriz 1:1. O L0 de HTML
tem um único consumer produtivo em `compiler/stdlib/html.rs`; o registro do
módulo no scope base permanece no owner `compiler/eval.md`. Não transformar a
menção histórica “owners candidatos” em ownership 1:N.

### 5.2 Condição de paragem

Depois da medição e da redação dos L0s, produzir
`p1293-pre-gate-l0-receipt.md` com hashes, fontes `file:line`, inferências e o
que as refutaria, e **PARAR**.

Os novos membros são superfície pública. Código só começa após confirmação
humana explícita do gate P1293. Confirmações de P1292 não são transferidas.

Se a medição mostrar necessidade de novo campo público, nova entidade, novo
default ou mudança de fase do pipeline, atualizar o L0 e pedir nova confirmação
em vez de ampliar o escopo silenciosamente.

---

## 6. Segregação de autoria

O protocolo é completo porque o passo altera linguagem pública, casts e HTML
exportável. Para cada lote:

| Papel | Saída | Restrição decisiva |
|---|---|---|
| Medidor | baseline e sondas vanilla | não escreve contrato/código |
| Autor do contrato | L0s, contrato e inferências | não lê patch candidato |
| Autor dos oráculos | casos positivos, negativos e opacos | não adapta ao patch |
| Implementador | código produtivo dentro da allowlist | não edita contrato/oráculos |
| Adversário | mutantes e recibo | não corrige candidato |
| Verificador | recibos reproduzíveis | não escreve o que verifica |
| Adjudicador | certificado | não acumulou contrato+solução+veredito |

Criar `p1293-manifest.json` antes das entregas, registrando executor, ambiente,
entradas legíveis, caminhos graváveis, contexto herdado, hashes e ordem causal.
O filesystem compartilhado limita a atestação técnica; o certificado deve dizer
“segregado por capacidades e artefatos, sem isolamento técnico de leitura”.

Contratos e testes dos quatro lotes podem ser preparados em paralelo depois do
baseline comum. Integração só ocorre sobre sub-selos válidos. Escrita simultânea
no mesmo arquivo é proibida; mudanças em `compiler/eval/mod.rs` ficam exclusivas
do lote D e mudanças em `structural/math.rs` exclusivas do lote B.

---

## 7. Selo e RED

Após o gate humano, criar `p1293-contract-seal.json` com hashes do manifesto,
L0s, baseline, casos e política de `Unknown`. O gate discriminatório deve
rejeitar todas as mutações válidas antes do selo.

### RED-A

- field estático ausente;
- chamada ligada ausente;
- NaN, finitos e `±inf`;
- integer coercion;
- missing, extra, named e tipo inválido;
- acesso ligado sem chamada.

### RED-B

- presença/tipo/nome das quatro funções;
- `repr` default e payload completo de attach;
- todos os seis slots, `none`, named desconhecido e aridade;
- binom de 2 e mais argumentos, lower ausente, ordem e vírgulas;
- ausência real de barra e delimitadores esticados;
- mono/script, cramped true/false e erros;
- convergência sintática/qualificada sem comparar bytes internos.

### RED-C

- presença e `repr` de cada constructor;
- chamada vazia, body e nesting nas cinco tags normais;
- rejeição de body nas duas void;
- pelo menos um caso válido e inválido por classe de `AttrKind` usada;
- todos os atributos específicos cobertos por uma matriz de cast;
- `Feature::Html` off/on e target paged/HTML;
- DOM de todas as sete tags e escaping de atributo/body.

### RED-D

- os dez nomes curtos em `repr`;
- chamadas continuam produzindo os mesmos tipos de conteúdo;
- aridade/diagnósticos permanecem;
- bindings globais extras mantêm os nomes históricos e não contaminam o
  namespace;
- `.with(...)` preserva o namespace e o nome curto quando aplicável.

O recibo RED registra comando, saída, hash e causa semântica. Falha de build
alheia à obrigação não satisfaz RED.

---

## 8. Implementação permitida após RED

### Lote A

Adicionar `native_is_nan` ao mesmo consumer de float, estender os matches
fechados e delegar a forma ligada à mesma nativa. Não tocar `float.inf`,
`float.nan`, `signum` ou bytes.

### Lote B

Adicionar as quatro entradas a `make_math_module` antes do espelho `sym →
math`. Reutilizar `native_mono`/`native_script`. Para attach/binom, extrair ou
criar helpers canônicos no owner estrutural e fazer o caminho sintático
delegar, sem novo payload nem segunda regra de layout.

### Lote C

Adicionar specs de atributos e funções tipadas ao consumer HTML. Reutilizar o
cast genérico e a tabela void já vigentes. Alterar exporter somente se uma
sonda RED provar que o caminho genérico não satisfaz o L0; nesse caso, atualizar
primeiro o owner L3 e atravessar o gate aplicável.

### Lote D

Corrigir somente o primeiro argumento textual de `Func::native` usado nas dez
instâncias de namespace. Não renomear funções Rust, bindings globais ou chaves
do scope.

Todos os consumers tocados recebem headers de linhagem ressellados. L1
permanece puro e o dispatcher continua estático.

---

## 9. Campanha adversarial

Exigir `mutation_score = 1.0` por lote e agregado. Mínimos:

### A

1. sempre falso;
2. infinito tratado como NaN;
3. forma ligada usa fórmula diferente;
4. named desconhecido é ignorado;
5. função se chama `float.is-nan` no `repr`.

### B

1. um slot de attach é trocado ou descartado;
2. `none` vira conteúdo;
3. binom desenha barra;
4. lower variádico perde ordem/vírgula;
5. syntax e função usam payloads diferentes;
6. mono/script são wrappers divergentes;
7. cramped explícito é ignorado.

### C

1. tag aceita qualquer named como string;
2. atributo Presence falso é emitido;
3. `col`/`wbr` aceitam body;
4. body omitido de tag normal vira `Unset`;
5. atributo específico é aceito na tag errada;
6. um enum aceita token fora da tabela;
7. feature é habilitada pelo target;
8. void recebe end tag;
9. ordem/escaping de atributos é alterada.

### D

1. apenas os três membros amostrados são corrigidos;
2. nome curto é aplicado ao binding global em vez da instância namespaced;
3. `table.cell` recebe nome `grid_cell`;
4. nome muda, mas function pointer/call muda junto;
5. um dos dez siblings permanece com underscore.

Mutante equivalente só sai do denominador por decisão do adversário aceita pelo
adjudicador antes do cálculo. Denominador zero, timeout, crash do harness ou
construção ambígua é `Unknown`. Qualquer `Unknown` requerido ou sobrevivente
bloqueia o lote.

---

## 10. Verificação integrada e metas

Executar no estado integrado e estável:

```text
cargo build --release
cargo test --workspace -q
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --quiet --fail-on-warning=V5 .
crystalline-lint --quiet --fail-on-warning=V7 .
crystalline-lint --quiet --fail-on-warning=V15 .
crystalline-lint --quiet --fail-on-warning=V26 .
git diff --check
```

Reexecutar:

1. superfícies default e HTML;
2. contrato P1293 e repetição em ordem reversa;
3. testes protegidos P1292;
4. regressões P1288–P1291 nos seus limites certificados;
5. fixtures DOM HTML já existentes;
6. testes math de attach, frac, styles e transformações;
7. testes grid/table de construção e namespace.

Metas para os inventários fixos de P1292:

```text
default: 111 total / 99 MATCH / 12 justificadas / 0 regressões
HTML:    115 total / 104 MATCH / 11 justificadas / 0 MISSING_MEMBER
         / 0 UNVERIFIED_METADATA / 0 Unknown
```

Se o inventário canônico for regenerado e mudar o denominador, não editar a
contagem à mão. Registrar o novo catálogo e adjudicar nominalmente os 12
membros, os dez nomes grid/table e a preservação do baseline.

---

## 11. Artefatos obrigatórios

Produzir em `00_nucleo/diagnosticos/`:

```text
p1293-manifest.json
p1293-baseline-status.txt
p1293-vanilla-measurement-receipt.md
p1293-pre-gate-l0-receipt.md
p1293-contract-receipt.md
p1293-contract-seal.json
p1293-red-tests-receipt.md
p1293-implementation-receipt-a.md
p1293-implementation-receipt-b.md
p1293-implementation-receipt-c.md
p1293-implementation-receipt-d.md
p1293-adversarial-plan.md
p1293-adversarial-receipt.json
p1293-surface-default.json
p1293-surface-html.json
p1293-preservation-receipt.md
p1293-verification-receipt.md
p1293-final-certificate.json
typst-passo-1293-relatorio.md
```

Todo número decisório registra `HEAD` ou “working tree não commitada”, hora,
lista exata de caminhos alterados, comandos, entradas e hashes.

---

## 12. Veredito

`whole_step_closed: true` exige:

1. A, B, C e D individualmente `APPROVED`;
2. confirmação humana pós-L0;
3. RED anterior ao código em cada lote;
4. mutation score 1.0, sem sobrevivente e sem `Unknown` requerido;
5. 12 membros presentes e funcionais;
6. dez nomes grid/table curtos e respectivas chamadas preservadas;
7. superfície HTML sem `MISSING_MEMBER` ou `UNVERIFIED_METADATA`;
8. zero regressões das 99 equivalências default e 89 HTML anteriores;
9. P1292 e regressões anteriores preservadas;
10. build, workspace, fmt, lint, V5, V7, V15, V26 e diff-check verdes;
11. L0s, consumers, contrato e oráculos byte-idênticos aos hashes verificados.

Falha em qualquer item mantém o passo aberto com o lote exato. Presença de
binding, DOM plausível, ausência de crash ou maioria de mutantes mortos não é
substituto.

---

## 13. Condições de paragem imediata

Parar e devolver ao dono se:

- o baseline fresco não reproduzir as 15 diferenças amostradas;
- o inventário de atributos divergir da fonte/binário pinado;
- `math.attach` não puder convergir sem mudar entidade pública além dos sete
  campos já existentes;
- `math.binom` exigir nova fase ou payload em vez da fração sem barra vigente;
- uma tag exigir comportamento de browser, CSS, mídia ou rede;
- a correção de nome grid/table alterar identidade ou chamada;
- V15/V26 revelar owner 1:N ou Núcleo inválido;
- qualquer entrada selada mudar;
- um teste só ficar verde relaxando o observável;
- houver mutante sobrevivente ou `Unknown` requerido.

P1293 termina com os resíduos acionáveis do perfil HTML fixo; features
desligadas e extensões cristalinas permanecem explicitamente fora do escopo.
