# C-P1303-v1 — contrato observável dos gates `pdf.*`

## Identidade e estado

- `contract_id`: `C-P1303-v1`
- `step`: `P1303`
- `role`: `AUTOR_DO_CONTRATO`
- `regime`: protocolo completo de materialização segregada, executado sem
  atestação de isolamento técnico
- `baseline_commit`: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`
- `vanilla_commit`: upstream/main ratificado `a51e02804`
- `vanilla_binary`: `/usr/local/bin/typst`
- `vanilla_binary_sha256`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
- `authored_at`: `2026-09-04T00:32:12,993512092-03:00`
- `timezone`: `America/Sao_Paulo (-03:00)`

Este artefato congela obrigações antes do RED e sem leitura de implementação
candidata. Ele é um contrato candidato para os gates posteriores; seu hash
externo identifica esta versão, mas não prova isolamento, suficiência dos
oráculos, GREEN, score de mutação ou equivalência funcional.

## Proveniência e capacidades

### Executor e ambiente

O executor foi uma sessão Codex no papel exclusivo de autor do contrato, no
workspace compartilhado `/repos/Antigravity/typst-crystalline`. O contexto
herdado continha a atribuição do papel, a allowlist abaixo e as instruções do
repositório. O filesystem é compartilhado com outros papéis; portanto este
artefato declara somente `executado sem atestação de isolamento`, nunca
`segregado e atestado`.

Capacidades efetivamente exercidas:

- leitura: `AGENTS.md`, a skill `tekt-materializacao-segregada`, suas duas
  referências, o passo P1303, o recibo P0 e os três owners L0 enumerados na
  tabela de inputs;
- escrita: somente
  `00_nucleo/diagnosticos/p1303-contract.md`;
- não lidos: código produtivo, testes Rust, patch/candidato de implementação,
  oráculos P1, plano adversarial, ledger de mutantes e artefatos de veredito;
- não exercidas: edição de L0, produto, testes, oracle ou baseline; staging,
  commit, selo, implementação e veredito.

### Inputs diretos congelados

| Input | SHA-256 | Bytes |
|---|---|---:|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` | 19017 |
| `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` | 4318 |
| `references/papeis-e-capacidades.md` da skill | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` | 1798 |
| `references/artefatos-e-gates.md` da skill | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` | 4314 |
| `00_nucleo/materialization/typst-passo-1303.md` | `f1db5c02b7e6af5ef461900c213e928a8d16bbed156d8eba7ff5c6b32c584fe9` | 16143 |
| `00_nucleo/diagnosticos/p1303-baseline-status.txt` | `8e06d7deba171604986f5ae3eca4d9c2c964a2aaf6d2a0f1971f7aa27574d9a8` | 2066 |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `de81ef3bf6572a1777f9057655e8433c6f2b393e8846f1fbdb66bcd1c8f81df9` | 21728 |
| `00_nucleo/prompts/compiler/eval/tests.md` | `5ca2ad1e4bcf3f6fe30909be21f42bb2a2a5939a6efcbdfc136dba5b44e7a8d1` | 9909 |
| `00_nucleo/prompts/compiler/stdlib/pdf.md` | `33cd1462491b6e94801d13d04a5b4cca9d223b008542fb835d5c4932f58d495a` | 6988 |

O P0 registra a árvore limpa exceto pelo próprio passo P1303, sem diff tracked
nem staged, capturada em `2026-09-04T00:29:29,144306984-03:00`. Os hashes de
produto presentes no P0 são identidade de baseline, não conteúdo lido por este
papel. As medições P1299 citadas pelo passo selecionam a coorte, mas não foram
reabertas nem reinterpretadas aqui. O recibo P1 futuro deve validar este
contrato; este documento não antecipa que essa reprodução passou.

## Medição recebida antes da decisão

O passo congelado informa que, nos perfis `default` e `html`, vanilla e
cristalino pré-candidato já coincidem em mensagem e nos dois hints para os três
fields, mas divergem na âncora: o cristalino cobre `pdf.<field>` e o vanilla
cobre somente `<field>`. Nos perfis `a11y` e `html+a11y`, os três casos já são
`MATCH_VALUE`.

O passo atribui à fonte vanilla o uso de `field.span()` em
`lab/typst-original/crates/typst-eval/src/code.rs:347-366` e informa que o
baseline cristalino intercepta a falha feature-gated no owner
`field_access.rs:83-98`, usando a âncora total em `:91`. Estas referências são
proveniência recebida do passo; o autor deste contrato não leu esses arquivos.

É inferência, não facto de implementação provado por este papel, que a âncora
total do ramo especial causa as seis divergências. Refutam a inferência:
mensagem, hints, severidade ou cardinalidade diferentes numa medição fresca;
diferença com `a11y-extras` ligada; outro owner causal; span do identificador
irresolvível; ou necessidade de mudar API pública, default, feature, fase ou
compatibilidade.

## Classificação ADR-0127

Classificação congelada:
`ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`.

O observável alterado é somente o span publicado por seis diagnósticos já
existentes. O contrato não acrescenta campo de entidade, método de trait,
assinatura Rust pública, modo/flag/caminho padrão, mudança de fase
`eval`/`layout` ou quebra de compatibilidade. Assim, após L0-first e resselo, o
fluxo autorizado é contínuo e tem como gate RED→GREEN mais revalidação.

Se o P1 refutar qualquer premissa acima, este contrato não autoriza código: o
resultado obrigatório é `P1303_BLOCKED_ADR0127_RECLASSIFICATION` ou, quando a
medição em si divergir, `P1303_BLOCKED_MEASUREMENT_REFUTED`.

## Universo e perfis

O universo principal é o produto cartesiano de exatamente três fields por
quatro perfis, totalizando doze casos:

| Perfil | `html` | `a11y-extras` | Classe obrigatória por field |
|---|---:|---:|---|
| `default` | desligada | desligada | diagnóstico negativo |
| `html` | ligada | desligada | diagnóstico negativo |
| `a11y` | desligada | ligada | valor positivo |
| `html+a11y` | ligada | ligada | valor positivo |

Fields, por identidade pública e grafia exata:

1. `pdf.data-cell`;
2. `pdf.header-cell`;
3. `pdf.table-summary`.

Ativar `html` não modifica o gate `a11y-extras`. Nenhum alias com underscore,
field adicional, stub ou aproximação pertence ao universo.

## Obrigações negativas — feature desligada

Para cada field nos perfis `default` e `html`, todas as condições seguintes
são cumulativas:

1. a execução falha, com exit code não zero e igual ao vanilla ratificado para
   o mesmo argv;
2. stdout é igual ao vanilla e não contém valor do probe;
3. existe exatamente um diagnóstico primário, de severidade `error`;
4. existem exatamente zero diagnósticos laterais;
5. a mensagem é byte por byte a forma aplicável da tabela abaixo;
6. existem exatamente dois hints e nenhum terceiro, na ordem indicada abaixo;
7. existe exatamente um span resolvível, half-open, igual ao range congelado;
8. o span cobre todos e somente os bytes do identificador à direita do ponto:
   exclui `pdf`, exclui o ponto e não perde nem acrescenta byte do field;
9. o stderr CLI integral é byte-idêntico ao vanilla para o mesmo argv;
10. a classificação bilateral final é `MATCH_DIAGNOSTIC`.

| Field | Mensagem exata | Range de bytes exato |
|---|---|---:|
| `data-cell` | ``cannot access field `data-cell` because the `a11y-extras` feature is not enabled`` | `14..23` |
| `header-cell` | ``cannot access field `header-cell` because the `a11y-extras` feature is not enabled`` | `14..25` |
| `table-summary` | ``cannot access field `table-summary` because the `a11y-extras` feature is not enabled`` | `14..27` |

Hints exatos, preservando texto e ordem:

```text
try enabling the `a11y-extras` feature
see https://typst.app/help/compiler-features for more details
```

Os ranges são offsets de bytes half-open dos canários congelados pelo passo e
são normativos como registrados; não devem ser recalculados a partir da
formatação Markdown deste documento. A forma conceitual do probe é
`repr((type(pdf.<field>), repr(pdf.<field>)))`.

Cardinalidade agregada obrigatória nos seis casos negativos: `6` erros
primários, `0` diagnósticos laterais, `12` hints, `6` spans resolvíveis e `0`
resultados `Unknown`/`EXECUTION_UNKNOWN`.

## Obrigações positivas — feature ligada

Para cada field nos perfis `a11y` e `html+a11y`, todas as condições seguintes
são cumulativas:

1. exit code `0`;
2. stderr integral vazio;
3. nenhum diagnóstico primário ou lateral e nenhum hint;
4. `type(pdf.<field>) == function`;
5. `repr(pdf.<field>)` preserva o nome público exato;
6. o probe bilateral termina em `MATCH_VALUE`, nunca em uma equivalência
   aproximada.

Valores textuais exatos do canário:

| Field | Resultado exato de `repr((type(...), repr(...)))` |
|---|---|
| `data-cell` | `"(function, \"data-cell\")"` |
| `header-cell` | `"(function, \"header-cell\")"` |
| `table-summary` | `"(function, \"table-summary\")"` |

Uma chamada representativa de cada função deve ainda preservar a morfologia
P1288:

- `pdf.table-summary` devolve a mesma tabela, substituindo somente o summary
  semântico; omissão de summary difere de `summary: none` explícito;
- `pdf.header-cell` aceita conteúdo cru ou `table.cell`, produz classificação
  explícita Header e preserva os demais campos; defaults continuam `level: 1`
  e `scope: "column"`;
- `pdf.data-cell` aceita conteúdo cru ou `table.cell`, produz classificação
  explícita Data e preserva os demais campos.

O valor/morfologia da chamada deve ser bilateralmente idêntico ao vanilla e ao
contrato vigente P1288. Este P1303 não redefine argumentos, defaults, erros de
chamada, render ou carriers.

Cardinalidade agregada obrigatória nos seis casos positivos: `6` exits `0`,
`6` stderrs vazios, `6` resultados `MATCH_VALUE`, `0` diagnósticos e `0`
resultados `Unknown`/`EXECUTION_UNKNOWN`.

## Sentinelas obrigatórias

As sentinelas fazem parte do gate de não regressão e devem ser executadas nos
perfis em que sua condição se aplica. Nenhuma pode resultar em `Unknown`.

### Diagnósticos `Module` já selados por P1301r2

Cada lookup ausente produz exatamente um erro, zero laterais, zero hints e
span resolvível somente sobre `nope`:

| Probe | Mensagem exata |
|---|---|
| `std.nope` | ``module `global` does not contain `nope` `` |
| `calc.nope` | ``module `calc` does not contain `nope` `` |
| `sym.nope` | ``module `sym` does not contain `nope` `` |
| `color.map.nope` | ``module `map` does not contain `nope` `` |

O whitespace exterior às mensagens na tabela não faz parte do valor. Um
lookup existente em cada módulo deve permanecer positivo e preservar valor e
kind. Os fixtures mínimos são `std.rgb`, `calc.abs`, `sym.alpha` e
`color.map.viridis`; sua expectativa de valor/kind é a do vanilla ratificado e
deve ser congelada pelo oracle antes do RED, sem adaptação ao patch.

### Target não-`Module`

- um dicionário sem a chave `nope` continua a emitir exatamente
  `dictionary does not contain key "nope"` com a expressão completa como
  span, não o field-only span;
- `float("NaN").is-nan` sem chamada continua a emitir exatamente
  `cannot access fields on type float`, com span somente em `is-nan`;
- ambos exigem um erro, zero laterais, seus hints vigentes inalterados e range
  resolvível; servem para impedir generalização da correção.

### Namespace `pdf` e gates vizinhos

- `pdf.attach` e `pdf.artifact` continuam acessíveis sem `a11y-extras`, com
  kind `function`, nome/repr e valor bilaterais preservados;
- nenhum de `pdf.data-cell`, `pdf.header-cell` ou `pdf.table-summary` fica
  acessível sem `a11y-extras`;
- todos os três ficam acessíveis com `a11y-extras` nos dois perfis positivos;
- o canário de `html` executado sem a feature HTML continua classificado
  exatamente como `EXPECTED_FEATURE_DISABLED`, não como sucesso, diferença ou
  estado opaco;
- o gate `html`, o catálogo de `pdf`, `color.map` e as extensões cristalinas
  permanecem fora de modificação.

## Repetição e ordem

A ordem normal é:

1. perfis `default`, `html`, `a11y`, `html+a11y`;
2. dentro de cada perfil, fields `data-cell`, `header-cell`, `table-summary`;
3. depois, sentinelas na ordem em que aparecem neste contrato.

A ordem invertida é a reversão integral dessa sequência. Ambas executam o
mesmo multiconjunto de argv e devem produzir, por chave estável
`(perfil, probe)`, os mesmos exit codes, stdout, stderr, hashes, cardinalidade,
mensagens, hints, ranges e classificações. Duração, diretórios temporários e
ordem física de execução não são comparados. Repetir uma ordem também deve
preservar o mesmo vetor observável.

Antes do patch, a reprodução P1 esperada é `3 DIFFERENT_DIAGNOSTIC` em
`default`, `3 DIFFERENT_DIAGNOSTIC` em `html`, `3 MATCH_VALUE` em `a11y`, `3
MATCH_VALUE` em `html+a11y` e `0 Unknown`. Depois do candidato, o vetor exigido
é `3 MATCH_DIAGNOSTIC`, `3 MATCH_DIAGNOSTIC`, `3 MATCH_VALUE`, `3 MATCH_VALUE`,
com `DIFFERENT_DIAGNOSTIC: 0` e `EXECUTION_UNKNOWN: 0`.

## Política de `Unknown`

`Unknown` nunca é convertido implicitamente em `Preserved`, `MATCH_VALUE`,
`MATCH_DIAGNOSTIC` ou PASS. Nos doze casos principais, em todas as sentinelas,
nas duas ordens e em suas repetições, `Unknown` e `EXECUTION_UNKNOWN` são
proibidos: qualquer ocorrência bloqueia o gate.

Os únicos opacos explicitamente reconhecidos pelos inputs são AT real e
fallbacks de PDF dependentes de versão, herdados do scope-out P1286. Eles não
são executados nem contados por C-P1303-v1. Se um experimento adicional os
observar, `Unknown` pode ser registrado somente como opacidade, jamais como
evidência de preservação ou como contribuição ao score. Parser sem suporte,
identidade ambígua, budget esgotado ou falha de execução nos casos cobertos não
são opacos aceitos; são bloqueio/inconclusão.

## Poder discriminatório obrigatório

Os oráculos derivados deste contrato devem rejeitar, no mínimo:

- span total `pdf.<field>`, span somente em `pdf`, inclusão do ponto, início um
  byte à esquerda ou fim um byte à direita;
- correção de apenas um field ou apenas do perfil sem `html`;
- remoção, adição ou reordenação de hints e qualquer alteração de aspas,
  hífen, field ou nome da feature na mensagem;
- exposição do trio sem `a11y-extras` ou ocultação com a feature ativa;
- generalização field-only ao erro de dicionário;
- regressão da projeção pública `std` para `global`;
- alteração da acessibilidade, kind, nome/repr ou valor de
  `pdf.attach`/`pdf.artifact`.

Cada mutação aplicável deve chegar a `Violated` com testemunha. O score para o
gate posterior é `mutantes válidos rejeitados / mutantes válidos = 1.0`;
mutante sobrevivente impede o selo.

## Língua versus mecânica

São observáveis de linguagem e, portanto, normativos sob ADR-0107:

- disponibilidade dos fields por feature;
- exit de sucesso/falha, kind `function`, nome público, `repr` e morfologia das
  chamadas;
- severidade, mensagem, hints, cardinalidade e span resolvível dos erros;
- stderr CLI byte-idêntico, porque os bytes do diagnóstico são neste caso o
  próprio observável público.

São mecânica deliberadamente não contratada: ponteiros, endereço de function
pointer, igualdade Rust, layout de enum, ordem interna de inserção, estrutura
de dados, função/helper escolhido, passos do algoritmo, posição do `match` e
qualquer estratégia de implementação. O contrato também não exige igualdade
de duração nem de caminhos temporários.

## Limites da alegação e proibições

C-P1303-v1 cobre somente os três spans feature-gated e as sentinelas
explicitamente enumeradas. Mesmo um PASS não prova equivalência funcional
geral do compilador, do módulo `pdf`, do PDF exportado ou da acessibilidade.

Ficam fora da alegação e sem autorização de mudança:

- membros ausentes, inclusive o inventário residual P1299;
- `color.map`, os dez mapas e identidade/`repr` públicos ainda residuais;
- extensões cristalinas intencionais;
- o comportamento esperado de `html` sem sua feature;
- a divergência separada de `repr(std)`;
- contratos completos de `pdf.attach`/`pdf.artifact`, bytes PDF, tagging, AT
  real e fallbacks por versão;
- argumentos/defaults/erros funcionais P1288, salvo preservação sentinela;
- API pública, entidades, traits, assinaturas, feature flags, defaults,
  compatibilidade, pipeline, CLI, exportadores, wiring e lab.

O contrato não autoriza mover o gate, duplicar catálogo, criar blacklist ou
fallback reflexivo, generalizar spans de targets não cobertos, fabricar stubs
ou corrigir divergências vizinhas. Qualquer necessidade desse tipo refuta o
escopo e exige `P1303_BLOCKED_SCOPE` ou reclassificação humana.

## Condição de handoff

O próximo papel deve receber este arquivo pelo seu SHA-256 externo e conferir
literalmente que todos os inputs acima permanecem com os hashes congelados.
Mudança de passo, baseline, owner L0, contrato ou oracle protegido invalida a
cadeia desde a primeira fase afetada. O contrato somente pode ser selado após
P1 reproduzível, oráculos positivos/negativos, gate discriminatório completo e
score `1.0`; este autor não emite o selo nem o veredito.
