# P1285 — receipt dos testes RED independentes

**Estado:** `ADDENDUM_RED_38_42_FINAL_CARRIER`  
**Regime:** testes A/B dentro do protocolo completo de materialização segregada.  
**Papel:** Testador A/B; autoria independente da implementação candidata.  
**Instante da medição final pós-restart:** `2026-08-30T14:46:55-03:00`.  
**Janela da reexecução na candidata:** `2026-08-30T14:56:12-03:00` a
`2026-08-30T14:57:34-03:00`.  
**Janela RED do segundo restart v3:** `2026-08-30T15:06:33-03:00` a
`2026-08-30T15:10:09-03:00`.  
**Janela RED do restart final v4:** `2026-08-30T15:33:22-03:00` a
`2026-08-30T15:34:24-03:00`.  
**Janela RED do addendum de precisão v4:** `2026-08-30T15:47:46-03:00` a
`2026-08-30T15:49:55-03:00`.  
**Correção do fixture gradient e reexecução:** `2026-08-30T15:54:42-03:00` a
`2026-08-30T15:55:15-03:00`.  
**Janela RED do carrier final:** `2026-08-30T15:58:43-03:00` a
`2026-08-30T15:59:15-03:00`.  
**Refinamento de compatibilidade pós-verificador:**
`2026-08-30T16:42:24-03:00` a `2026-08-30T16:42:54-03:00`.  
**Reexecução final independente do refinamento:**
`2026-08-30T16:45:57-03:00` a `2026-08-30T16:46:32-03:00`.  
**Refinamento verificador v2 — Color RGB nominal:**
`2026-08-30T17:09:58-03:00` a `2026-08-30T17:12:20-03:00`.  
**HEAD hospedeiro:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este receipt é diagnóstico. Não é Prompt L0, implementação, gate de mutações,
certificado nem veredito de refinamento.

## 1. Capacidades e isolamento

Na autoria inicial, as entradas permitidas foram AGENTS, o skill
`tekt-materializacao-segregada` e suas duas referências diretas, somente o
Passo 1285 explicitamente autorizado, os seis L0 então finais, os receipts de
contrato/oráculo, o runner e os módulos de teste existentes. No segundo
restart, a derivação usou somente o contrato v3, os três L0 produtivos
novos/repinados, o L0 test-only do harness E2E e os módulos proprietários de
teste. O oracle receipt v3, o runner e o baseline não foram abertos nem usados
como entradas da autoria v3. No restart final, somente contrato v4, L0
`compiler/eval/math.md`, L0 test-only do harness e assinaturas do módulo de
teste foram entradas novas; oracle v4, runner e baseline não foram abertos.
No addendum de precisão, foram entradas novas somente o contrato v4+precision
ressellado e os três canónicos C-JSON comunicados pela autoridade verificadora;
oracle receipt, runner e baseline não foram abertos nem usados.
Na correção posterior do fixture, a única entrada nova foi a identificação
autoritativa de que os nomes Typst `red`/`blue` da sonda correspondem a
`#ff4136`/`#0074d9`, não aos primários RGB puros. Nenhum artefato de oráculo,
runner ou baseline foi aberto.
Na correção final do carrier, a única entrada nova foi a constatação
autoritativa de que `rect(width: 10pt, height: 20pt)` chega ao serializer E2E
como `Value::Length`, enquanto a testemunha anterior construía diretamente
`Value::Relative`. Nenhum artefato de oráculo, runner ou baseline foi aberto.
Escritas permitidas ao longo das três fases:

- blocos `#[cfg(test)]` de `02_shell/src/cli.rs`;
- bloco `#[cfg(test)]` de `01_core/src/compiler/eval/selector_matching.rs`;
- bloco `#[cfg(test)]` de `03_infra/src/query_helpers.rs`;
- suíte proprietária `04_wiring/tests/cli.rs`;
- bloco `#[cfg(test)]` de
  `01_core/src/compiler/stdlib/foundations/selector.rs`;
- suíte test-only proprietária `01_core/src/compiler/eval/tests.rs`;
- este receipt.

Não foi escrita nem corrigida implementação produtiva. Não foram lidos outros
ficheiros de `materialization/` nem ficheiros de `context/`. A árvore global já
estava extensamente não commitada; nos três consumers P1285, antes destes testes,
`git diff HEAD --stat` mostrava somente duas linhas alteradas por ficheiro,
correspondentes ao resselo de lineage comunicado pelo coordenador. Não se alega
isolamento ambiental forte: o resultado é **segregado por capacidade e
executado sem atestação de isolamento**.

### Primeiro restart causal anterior à candidata

Depois do primeiro receipt RED, mas ainda antes de qualquer candidata P1285, o
coordenador identificou que o transporte de `QueryFormat` e a source transitória
de `query -` pertencem ao owner produtivo L4. `00_nucleo/prompts/wiring.md` foi
atualizado e ressellado. Esta alteração de entrada protegida invalidou o pin
anterior de cinco owners; os testes foram derivados novamente sob seis L0
produtivos. O receipt anterior, hash
`382b27b10838a79cde955edf560e3980d7e3ab0c665f2eb730d2fdca8ef1a046`,
é predecessor histórico, não evidência pós-restart.

Naquele primeiro restart, o contrato autoritativo passou a ser
`P1285-CONTRACT-v2`, SHA-256
`94bb0f0115e5179b3c744834402058343a53792df8eae1c69102cad403595aa9`.
Ele invalida expressamente o contrato v1, SHA-256
`9897a6f893734b66b2d1c93ee653bc1155ab0c51f705c89f22011ee73a3c77ec`,
antes da primeira candidata, e incorpora o sexto owner L4. O pin v1 que
permaneceu na versão anterior deste receipt era erro documental: não muda a
cronologia das execuções, feitas pós-restart contra os seis L0, nem o conteúdo
dos dez testes. Este receipt corrige causalmente o pin sem fabricar uma nova
fase RED depois de existir implementação candidata.

### Segundo restart causal após a candidata v1

O contrato v3, SHA-256
`1ea58e14a01f6cb55fe38695133f4c279164ca2943134d2dcf5cab81c3df681e`,
invalida o contrato v2 e a candidata v1 como evidência final. O resultado
parcial sobre constructor/show revelou obrigações antes não congeladas. Foram
adicionados os owners produtivos `compiler/eval/rules.md` e
`compiler/stdlib/foundations/selector.md`, e
`compiler/eval/selector_matching.md` foi repinado. A autoria RED reiniciou na
primeira fase afetada, sem corrigir a candidata v1 e sem consultar as saídas
privadas dos oráculos.

### Restart final causal antes da candidata math

O contrato v4+precision, SHA-256
`647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`,
invalida o contrato v3 e as candidatas v1/v2 como evidência final. O nono
owner `compiler/eval/math.md` congela a distinção morfológica
`MathTextKind::Grapheme → Content::MathIdent` e
`MathTextKind::Number → Content::MathText`. O teste foi escrito e executado
antes de qualquer edição candidata em `math.rs`. O prefixo `1f4e98…` do oracle
receipt v4 foi apenas comunicado pelo coordenador; o artefato não foi aberto
nem usado por esta autoridade.

O predecessor v4, SHA-256
`8f6c94e6046574f39f8c913076c5b455914fbce3b6e74a5af05157e82b95f588`,
foi substituído pelo adendo de precisão antes do fechamento desta evidência.
O adendo explicita três canónicos de testemunhas já pertencentes a C-JSON e
não altera versão lógica, L0, ownership ou obrigação semântica; portanto não
houve restart causal. Os testes L2 foram fortalecidos e congelados contra a
candidata v2, sem qualquer edição produtiva por esta autoridade.

## 2. Entradas congeladas verificadas

### 2.1 Restart final v4 — nove owners e política seal-only

| Owner L0 | Pin normativo sem `Hash do Código` | Full SHA-256 de proveniência |
|---|---|---|
| `prompts/shell/cli.md` | `8fe484152e1371d7ef4a79659d6388bf5e63ff63ff84dc5698db7147a33543a7` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `7b2dc94f28457d84022f59a80b858c12428f671e67e3a5e90e6c0865d773e6c3` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `3f1bcd114c5f880e10fedd3f5fbaee868d456b9c31b244244cb7a8a15d23860b` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` | `557abd0a6fb52f8b198606e928ba23fd65234e3b75c2a14703b07a9975ba1b11` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `prompts/compiler/eval/rules.md` | `3655f2922835fd0bfcf0a1857a6d12d70ac9c30eae23f9f7b9eea1f69d9fc7c0` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` |
| `prompts/compiler/stdlib/foundations/selector.md` | `9bea0284d242754ca1103baf45c53b7614817cfff1e5358fd708980f971f7c22` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` |
| `prompts/infra/query-helpers.md` | `352604527883503c72e349f9171cdb92cf730af3772cde28820275f11005713d` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` | `2142e48948482f1f86a8390b4dbd40cdc84f67f1783d5c9f3f370e9ed1401a44` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `prompts/compiler/eval/math.md` | `c2ba2c5f028cd7424d15f92486e1d281a6efca1b74696f75249c6b9f1a018a9c` | `7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2` |

Contrato autoritativo: `P1285-CONTRACT-v4`, SHA-256
`647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`.
Owner do harness E2E test-only: `prompts/compiler/eval/tests.md`, full SHA-256
`92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871`.

Política normativa: o pin sem a linha `Hash do Código` identifica a obrigação
semântica. Mudança exclusivamente nessa linha é drift seal-only permitido:
regista novo full hash de proveniência, mas não reinicia a semântica se o pin
normativo permanecer idêntico. Qualquer alteração no pin normativo reinicia a
cadeia na primeira fase afetada.

### 2.2 Segundo restart v3 — entradas históricas

| Entrada | SHA-256 |
|---|---|
| `diagnosticos/p1285-contract-receipt.md` — `P1285-CONTRACT-v3` | `1ea58e14a01f6cb55fe38695133f4c279164ca2943134d2dcf5cab81c3df681e` |
| `prompts/shell/cli.md` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` — repin v3 | `fd7a0b50308e119932d9fee90f9aa972d4a6ebab55ad31b15aa9decead59cae6` |
| `prompts/compiler/eval/rules.md` — owner produtivo novo | `c97c2f6fc0275e37667b1d183ada5437e523ab3248b2549536da7b038bba875f` |
| `prompts/compiler/stdlib/foundations/selector.md` — owner produtivo novo | `6849e281548fd239321dab6fdf31a9a6473ac8bd7a9bc6d7fd8a78c4be63d7d8` |
| `prompts/infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `prompts/compiler/eval/tests.md` — owner test-only do harness E2E | `92ac9be1bcaeb65d0cda76603ba7260ffaccda2f6ff7049bf57bff24d6d83871` |

O prefixo `e7bc3b…` do oracle receipt v3 foi comunicado pelo coordenador, mas o
artefato não foi aberto nem hasheado por esta autoridade. Ele não é input
destes testes.

### 2.3 Entradas históricas v2

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `tekt-materializacao-segregada/SKILL.md` | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `materialization/typst-passo-1285.md` | `5a69234ba5340c92e7cfe523212b102869e8a7fcefafb4647e4acf1dc3690a6c` |
| `prompts/shell/cli.md` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` | `1de6b662299ba6dc1492d3230cd4e12c732dcc5648a288bbd0ea26b9dbfd7c4d` |
| `prompts/infra/query-helpers.md` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` — sexto owner produtivo | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `prompts/wiring/tests/cli.md` — owner da suíte binária | `ccd47ca2c66c6830b3e5cde505987c7731aa7a78be4b0e58e7b1c29e261413d4` |
| `diagnosticos/p1285-contract-receipt.md` — `P1285-CONTRACT-v2` | `94bb0f0115e5179b3c744834402058343a53792df8eae1c69102cad403595aa9` |
| `diagnosticos/p1285-oracle-receipt.md` | `4ee6d07c38aff7a216d00ea317d5759aaa26669fbdfc2eb0f179201476e2e1b9` |
| `lab/surface-inventory/run_p1285_oracles.py` | `08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16` |

## 3. Matriz acrescentada

No segundo restart foram acrescentados seis testes, elevando a matriz própria
de P1285 de dez para dezasseis testes. O restart final acrescentou um teste,
totalizando dezassete.

### L1 test-only — morfologia math grapheme/number (v4)

Um teste no harness já proprietário `compiler/eval/tests.rs` avalia primeiro
`$ 2 $` e exige folha `Content::MathText("2")` como controle; depois avalia
`$ x $` e exige folha `Content::MathIdent("x")`. A asserção é sobre variants
morfológicos existentes, sem observar implementação do parser ou layout.

### L1 — constructor público de selector (v3)

Três testes no owner `stdlib/foundations/selector.rs` cobrem separadamente:

- preservação estrutural de um `Value::Selector::Where` recebido;
- conversão de `Value::Regex("f.o")` em `Value::Selector(Regex)`;
- diagnósticos byte-exatos e distintos para string vazia, regex vazia e regex
  não vazia capaz de casar texto vazio.

### L1 — splice regex puro (v3)

Dois testes em `selector_matching.rs` exigem todas as ocorrências `foo`, `fxo`
na ordem, com prefixo/interstício preservado, e `Ok(None)` sem invocar a
replacement tanto para no-match quanto para regex cujo único match é vazio.

### L1 test-only — show por ocorrência (v3)

Um teste E2E em `compiler/eval/tests.rs` usa o harness já proprietário. O
controle literal transforma `foo bar foo` em `L bar L`; a testemunha regex
transforma `foo fxo` em `R R`. Replacement constante elimina recursão e
discrimina uma chamada sobre o nó inteiro de uma chamada por ocorrência.

### L2 — CLI e serialização

Cinco testes `p1285_*` cobrem:

- parsing explícito de `EvalFormat::Yaml` e `QueryFormat::Yaml`;
- `Version` exatamente como `version(0, 15, 1)` e fallbacks `auto`/length
  dentro de array estrutural; no addendum, a mesma testemunha exige também
  Func `calc.gcd` como `gcd`, gradient exatamente como
  `gradient.linear((oklab(65.95%, 0.2, 0.108), 0%), (oklab(56.22%, -0.05, -0.17), 100%))`
  e tiling exatamente como `tiling((10pt, 10pt), ..)`;
- `Content::Sequence` com `Strong`, espaço e texto como árvore `func + fields`;
- query de metadata e figure rotulada sem expor o wrapper como `func: label`;
  no addendum, a mesma testemunha compara integralmente rect, figure, caption
  e equation: carrier `Value::Length` para width/height, serializado nos
  relativos canónicos, fill sem `stroke:null`,
  `alt:null`, `placement:null`, `scope:column`, `caption` sem position e com
  seus defaults públicos, counters canónicos, supplement `Equation`,
  number-align e attach com symbol `x` e expoente text `2`;
- `--field`/`--one`: extração estrutural, filtro de campo ausente no modo lista,
  erro nominal de campo no modo one e cardinalidade zero.

### L1 — selector matching

Dois testes cobrem literal e regex sobre `Content::Text`, positivos, negativos,
nó não textual e regex cujo único resultado seria vazio. Ambos afirmam também
que `is_node_rule(Text|Regex)` permanece `false`.

### L3 — recuperação de query

Dois testes cobrem recuperação de metadata/figure e o transporte de label sobre
a figure original, com cardinalidade unitária e sem match extra.

### L4 — composição observável do binário

Um teste de integração usa o harness já proprietário `wiring/tests/cli.md` e
envia por stdin `#metadata((name: "x", n: 2))` para
`query - metadata --field value --one --format yaml`. Ele exige exit 0, YAML em
stdout e warning deprecated em stderr. Assim discrimina conjuntamente o
transporte de `QueryFormat` e a criação da `Source` markup transitória, sem
importar funções privadas de `main.rs` nem criar consumer novo.

## 4. Execuções focais e cronologia causal

### RED histórico pós-restart, anterior à candidata

As quatro medições desta subseção ocorreram no instante final pós-restart acima,
antes da implementação candidata. Os dez corpos de teste usados nessas medições
continuam materialmente inalterados. Como os mesmos ficheiros passaram depois a
conter edição produtiva concorrente, os hashes integrais históricos da seção 5
não são apresentados como hashes da candidata atual.

### Matcher L1

```sh
cargo test -p typst-core p1285_selector_ -- --nocapture
```

Resultado: exit `101`; `0 passed`, `2 failed`, `5309 filtered out`.

- `p1285_selector_literal_casa_ocorrencia_local_sem_virar_node_rule` falhou
  porque `selector_matches(Content::Text("abc"), Text("b"))` devolveu `false`;
- `p1285_selector_regex_casa_match_nao_vazio_e_rejeita_negativos` falhou porque
  `selector_matches(Content::Text("abc123"), Regex("[0-9]+"))` devolveu `false`.

### Query L3

```sh
cargo test -p typst-infra p1285_query_elements_ -- --nocapture
```

Resultado: exit `101`; `1 passed`, `1 failed`, `909 filtered out`.

- recuperação de metadata e figure passou como controlo;
- `p1285_query_elements_transporta_label_sem_criar_match_extra` falhou: query
  por `<fig-one>` devolveu `Content::Figure` nu, não `Content::Label` envolvendo
  a mesma figure.

### CLI L2

```sh
cargo test -p typst-shell p1285_ -- --nocapture
```

Resultado: exit `101`, RED de compilação isolado em dois `E0599`:

- `EvalFormat::Yaml` ausente em `02_shell/src/cli.rs:1315`;
- `QueryFormat::Yaml` ausente em `02_shell/src/cli.rs:1323`.

Uma primeira execução também revelou uma anotação de tipo insuficiente no
helper do próprio teste; ela foi corrigida antes da medição final. A repetição
acima contém somente as duas ausências produtivas esperadas. Como o crate não
compila enquanto as variants faltam, os outros quatro testes L2 ainda não
receberam verdict de execução; isto não é convertido em `Preserved`.

### Wiring L4 pós-restart

```sh
cargo test -p typst-wiring --test cli \
  p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `70 filtered out`. O processo
testado terminou com exit `2`, antes de consumir a fonte, e stderr informou
`invalid value 'yaml' for '--format <FORMAT>'` com possible values somente
`json`; o teste exigia exit `0`. Este é RED observável de processo, não
`Unknown`. Depois de L2 aceitar YAML, o mesmo teste continuará a discriminar o
segundo eixo: tratar `-` como source transitória em vez de path físico.

Após o restart, os três comandos focais L1/L2/L3 acima também foram repetidos e
reproduziram os mesmos exits, contagens e testemunhas já registados.

### Reexecução na candidata v1 sob contrato v2 — não é RED v3

Os mesmos quatro comandos foram reexecutados na candidata entre
`2026-08-30T14:56:12-03:00` e `2026-08-30T14:57:34-03:00`. Todos terminaram
com exit `0`:

| Focal | Resultado na candidata atual |
|---|---|
| `typst-core p1285_selector_` | `2 passed`, `0 failed`, `5309 filtered out` |
| `typst-infra p1285_query_elements_` | `2 passed`, `0 failed`, `909 filtered out` |
| `typst-shell p1285_` | `5 passed`, `0 failed`, `56 filtered out` |
| `typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout` | `1 passed`, `0 failed`, `70 filtered out` |

Classificação exata dessas quatro execuções: `GREEN_CURRENT_CANDIDATE`. Elas não
são alegadas como RED e não substituem a evidência RED histórica. O snapshot foi
estável: os sete hashes integrais verificados imediatamente antes e depois dos
comandos foram idênticos.

Com o segundo restart, essa candidata v1 e esse GREEN permanecem apenas
evidência histórica v2. Não satisfazem nem refutam o contrato v3 completo.

### Segundo restart v3 — RED real contra a candidata v1 invalidada

#### Constructor de selector

```sh
cargo test -p typst-core p1285_native_selector_ -- --nocapture
```

Resultado: exit `101`; `0 passed`, `3 failed`, `5312 filtered out`.

- identidade falhou com `selector(): argumento inválido (selector)`;
- regex válida falhou com `selector(): argumento inválido (regex)`;
- o primeiro diagnóstico de vazio observado foi
  `selector(): kind '' não reconhecido`, em vez de
  `text selector is empty`.

Os testes fixam também os dois diagnósticos regex exatos; a primeira asserção
daquele caso agregado já basta para RED, sem converter os demais em sucesso.

#### Show regex por ocorrência

```sh
cargo test -p typst-core \
  p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `5314 filtered out`. O controle
literal `L bar L` passou dentro do teste; a testemunha regex devolveu `R` em vez
de `R R`, provando uma única chamada da recipe para o nó textual inteiro.

Uma primeira formulação exploratória com `strong(it)` encontrou
`maximum show rule depth exceeded`; ela não integra a medição final porque
misturava ocorrência com revogação/recursão. A replacement constante acima
removeu essa ambiguidade antes do RED final.

#### Splice regex puro

```sh
RUSTFLAGS=-Awarnings cargo test -p typst-core \
  p1285_splice_regex_ -- --nocapture
```

Resultado: exit `101`, RED de compilação isolado em dois `E0425`: a função
`splice_regex_rule_matches` exigida pelo L0 não existe. Não houve teste
executado, e isso não é convertido em `Unknown` ou sucesso. O primeiro caso
fixa matches `foo`, `fxo` e morfologia do splice; o segundo fixa no-match e
empty-only sem chamada da replacement.

### Restart final v4 — RED real antes da candidata math

```sh
cargo test -p typst-core \
  p1285_math_grapheme_e_numero_preservam_variantes_distintas -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `5317 filtered out`. O controle
numérico `$ 2 $ → Content::MathText("2")` passou antes da falha. Para `$ x $`,
a candidata v2 produziu
`equation(EquationElem { body: math.text("x"), block: true })`, enquanto o
contrato exige `Content::MathIdent("x")`. É RED morfológico discriminatório de
M-M1, não `Unknown`; M-M2 permanece protegido pelo controle numérico verde
dentro do mesmo teste.

### Addendum v4+precision — RED real L2 contra a candidata v2

Os dois corpos L2 existentes foram fortalecidos sem aumentar a contagem de
dezessete testes. Os três canónicos de fallback vieram da autoridade
verificadora e foram depois confirmados no contrato v4+precision; não foram
extraídos de oracle receipt, runner, baseline ou output candidato.

```sh
cargo test -p typst-shell \
  p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `60 filtered out`. A primeira
violação foi Func: a candidata serializou `"calc.gcd"\n`, enquanto C-JSON
exige `"gcd"\n`. A execução terminou antes das asserções de gradient e tiling;
portanto não produz verdict para elas. Além disso, o fixture gradient desta
execução ainda usava RGBs puros incorretos e o hash integral dessa suíte foi
posteriormente invalidado. A falha de Func antecede o fixture defeituoso e
permanece evidência histórica somente dessa testemunha.

```sh
cargo test -p typst-shell \
  p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `60 filtered out`. A árvore
observada preservou width `0% + 10pt`, height `0% + 20pt`, fill, number-align
e attach com base symbol `x`/expoente text `2`, mas violou a igualdade exata:

- rect acrescentou `stroke:null`;
- figure emitiu `placement:"auto"` em vez de `null`; `alt:null` e
  `scope:"column"` estavam corretos e fazem parte do esperado;
- caption usou `func:"figure.caption"`, acrescentou `position:"bottom"` e
  omitiu `kind`, supplement, numbering e counter, em vez da forma pública
  `func:"caption"` sem position;
- os counters de caption/figure devem ser
  `counter(figure.where(kind: image))`; o figure atual emitiu aspas indevidas
  em torno de `image` e o caption nem sequer transportou o campo;
- equation emitiu supplement `"auto"` em vez de content text `Equation`.

Esta é evidência `RED_CANDIDATE_V2_AT_MEASUREMENT` discriminatória das seis violações
de apresentação comunicadas como `36/42`; não é score independente nem
veredito sobre os demais 36 casos.

#### Correção posterior do fixture gradient

O fixture foi alinhado à sonda Typst `gradient.linear(red, blue)`:
`red = #ff4136 = rgb(255, 65, 54)` e
`blue = #0074d9 = rgb(0, 116, 217)`. O canónico esperado permaneceu idêntico.

```sh
cargo test -p typst-shell \
  p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture
```

Na candidata compartilhada já avançada, o resultado foi exit `0`;
`1 passed`, `0 failed`, `60 filtered out`. Classificação exata:
`GREEN_CURRENT_CANDIDATE`. Esta reexecução valida conjuntamente Func, gradient
e tiling com o fixture correto, mas não é alegada como RED nem reescreve a
cronologia da falha histórica de Func.

#### Carrier final de rect — RED real contra a candidata v3

A testemunha rect foi corrigida para reproduzir o transporte E2E:
`width` e `height` agora são construídos como
`Value::Length(Length::pt(10.0))` e `Value::Length(Length::pt(20.0))`; o esperado público continua
`0% + 10pt`/`0% + 20pt`.

```sh
cargo test -p typst-shell \
  p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture
```

Resultado: exit `101`; `0 passed`, `1 failed`, `60 filtered out`. A candidata
v3 emitiu width `"10pt"` e height `"20pt"`, em vez de
`"0% + 10pt"`/`"0% + 20pt"`. Todo o restante da árvore rect, figure,
caption e equation coincidiu com o esperado integral dentro da mesma asserção.
Classificação exata: `RED_CURRENT_CANDIDATE_V3`, discriminando a causa única
das quatro violações remanescentes comunicadas como `38/42`; isto não é score
independente nem veredito dos outros 38 casos.

### Refinamento pós-verificador — expectativa legada de gradient

A autoridade verificadora comunicou uma execução integral com `5317 passed` e
`1 failed`: `compiler::eval::repr::tests::repr_value_complex_types` ainda
exigia `gradient(...)` para um linear vazio, enquanto o contrato congelado
SHA-256
`647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`
fixa `gradient.linear()`. Essa falha integral é classificada somente como RED
de compatibilidade da suíte legada contra contrato já congelado; não muda L0,
contrato, produção ou a obrigação.

Foi alterada exclusivamente essa expectativa no bloco `#[cfg(test)]` de
`compiler/eval/repr.rs`. Hash integral anterior do ficheiro:
`aa47dd1f62f8ccfb25e54166b32c9c64b796f743968c28708d87d42ee2ff853b`;
hash após a edição:
`5a8163b8f5df91f6b9fb7ff65fd70daca0359eed1aacc2abc9acb44a8704b6ee`.
O hash deste receipt antes do adendo era
`3f093f748769ccbb128183ca73cfab9052f2a7d6b097503ab5e5ccff2532f25d`;
o novo hash externo é reportado junto da entrega, sem autoembuti-lo.

```sh
cargo test -p typst-core repr_value_complex_types -- --nocapture
```

O focal não confirmou GREEN: exit `101`; `0 passed`, `1 failed`,
`5317 filtered out`. A asserção gradient corrigida passou e a execução avançou
até uma segunda expectativa legada, não autorizada neste refinamento:
produção `tiling(..)` versus teste `tiling(...)` em `repr.rs:1080`. Essa
asserção não foi alterada. O resultado é registrado como bloqueio factual do
GREEN focal, sem ampliar escopo e sem emitir veredito.

#### Reexecução final independente após preservação produtiva de Tiling

A candidata preservou posteriormente o ramo Tiling sem tamanho como
`tiling(...)`, sem mudar a expectativa legada. Antes de executar, esta
autoridade comparou todo o sufixo `#[cfg(test)]` de `repr.rs` contra `HEAD`:
a única diferença encontrada foi
`gradient(...)` → `gradient.linear()`; a expectativa Tiling permaneceu
byte-exatamente `tiling(...)`. O hash integral observado de `repr.rs` foi
`cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b`,
igual ao comunicado pela candidata.

Cronologia dos hashes de `repr.rs`:

- `aa47dd1f62f8ccfb25e54166b32c9c64b796f743968c28708d87d42ee2ff853b`:
  expectativa gradient legada; RED integral comunicado, `5317 passed` e
  `1 failed`;
- `5a8163b8f5df91f6b9fb7ff65fd70daca0359eed1aacc2abc9acb44a8704b6ee`:
  somente expectativa gradient corrigida; RED secundário focal em Tiling,
  exit `101`, `0 passed`, `1 failed`, `5317 filtered out`;
- `cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b`:
  candidata preserva produtivamente o ramo Tiling legado; nenhum teste foi
  alterado nesta fase final.

Comandos e resultados finais:

```sh
cargo test -p typst-core repr_value_complex_types -- --nocapture
```

Exit `0`; `1 passed`, `0 failed`, `5317 filtered out`.

```sh
cargo test -p typst-core p1285 -- --nocapture
```

Exit `0`; `9 passed`, `0 failed`, `5309 filtered out`. O segundo comando cobre
os nove testes P1285 L1 encontrados pelo filtro; não é execução integral do
crate. O hash deste receipt antes deste adendo final era
`8e9aa9d10de5c60b86eab9c5edee9a1d4f38f769c18e2e4f721e1aae5d4dd955`;
o novo hash externo é reportado na entrega, sem autoembuti-lo. Estes resultados
são registros de gates focais GREEN e não constituem veredito.

### Refinamento pós-verificador v2 — Color RGB nominal

A rodada do verificador v2 comunicou RED no teste legado
`p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido`: a cláusula
C-JSON do contrato congelado aceita o fallback nominal de `Color`, enquanto a
asserção anterior ainda exigia erro. Somente essa asserção no bloco
`#[cfg(test)]` de `02_shell/src/cli.rs` foi substituída; as asserções de
`Stroke` e a proibição de `Raw` foram preservadas, e o teste não foi
renomeado.

A primeira instrução de autoridade usou a medição de `black`, variante
`Luma`, e produziu o esperado intermediário `"luma(0%)"`. Esse esperado foi
invalidado como fixture incorreto: o teste constrói
`Color::rgb(0, 0, 0)`, não a variante `Luma`. Os comandos equivalentes ao
construtor, medidos e comunicados pela autoridade,

```sh
target/debug/typst eval 'rgb(0, 0, 0)' --format json
/usr/local/bin/typst eval 'rgb(0, 0, 0)' --format json
```

produziram ambos exatamente `"rgb(\"#000000\")"\n`. A expectativa final em
bytes Rust passou a ser `b"\"rgb(\\\"#000000\\\")\"\n"`.

Cronologia integral dos hashes de `02_shell/src/cli.rs`:

- `a5c7110577501564004ad2f6d84a29ed1a282c94a412364b73e3b493e75d329f`:
  antes deste refinamento;
- `2326bd802820584b7d1390cf4946e993512e7a2da21b424c0b5cc1cff1d3c950`:
  esperado Luma intermediário, invalidado por não representar o construtor do
  fixture;
- `87f7983ec6ebc261603af3d9147faa936c79bb20582c7c1359fe29dd9ff11e85`:
  esperado final derivado da medição RGB equivalente.

Com o esperado Luma intermediário, a medição independente registrou:

```sh
cargo test -p typst-shell p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido -- --nocapture
# exit 101; 0 passed; 1 failed; 60 filtered out

cargo test -p typst-shell p1285 -- --nocapture
# exit 0; 5 passed; 0 failed; 56 filtered out

cargo test -p typst-shell
# exit 101; 60 passed; 1 failed
```

A única falha integral foi o mesmo teste, com resultado RGB real contra o
esperado Luma inválido. Após substituir exclusivamente essa expectativa pela
forma RGB medida, os três comandos foram repetidos:

```sh
cargo test -p typst-shell p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido -- --nocapture
# exit 0; 1 passed; 0 failed; 60 filtered out

cargo test -p typst-shell p1285 -- --nocapture
# exit 0; 5 passed; 0 failed; 56 filtered out

cargo test -p typst-shell
# exit 0; 61 passed; 0 failed
# doc-tests: 0 passed; 0 failed
```

`cargo fmt --all -- --check` e `git diff --check` passaram no estado final.
O contrato permaneceu pinado em
`647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`.
O hash deste receipt antes do adendo era
`5f6d145a6489d23df306b9d9f216ed2668acb63b5bf8e4b04ccde0f7ea6e9391`;
o novo hash externo é reportado na entrega, sem autoembuti-lo. Este adendo
registra a cronologia e os gates; não emite veredito.

## 5. Artefatos produzidos

Hashes históricos no instante RED final, antes de criar este receipt:

| Artefato | SHA-256 |
|---|---|
| `02_shell/src/cli.rs` | `45399c49617d5624927c086008f122dbda243c56ec0ab6162efffd31b36266ed` |
| `01_core/src/compiler/eval/selector_matching.rs` | `8768d2842b7b91404721fe9a85578829625fe6c13ed24f0ad3c2b254c75c08af` |
| `03_infra/src/query_helpers.rs` | `c2f3f7c2283232e9ea68ee5348915422171fe676c1386a5db29f888091236660` |
| `04_wiring/tests/cli.rs` | `2267e80473a7398dfa65b619c2eaa248663dd36793283b3f0a08bf328ca51bc0` |

`rustfmt --edition 2021 --check` passou nos quatro ficheiros de teste e
`git diff --check` não encontrou erro de whitespace.

Hashes integrais do snapshot candidato estável usado na reexecução atual:

| Entrada da execução | SHA-256 antes e depois |
|---|---|
| `02_shell/src/cli.rs` | `d577f1d9cf94aa986d7e789d5cd1dfc58ea0e7a4a9722ca7284764968b6ff06a` |
| `01_core/src/compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `01_core/src/compiler/eval/repr.rs` | `1e0c7b8cecfd86e2e1395be07de8d4ffbcc09a1bcff40d3de15bdbf312a5372d` |
| `01_core/src/compiler/eval/selector_matching.rs` | `e0c6b754dd3ee4c0d79b077426611b39967ab1b95bbf67028245b36ed86cf2e3` |
| `03_infra/src/query_helpers.rs` | `f060ebce29a30271e18e21b0dccb528de7a0a0a395560a6df1d7c2a0e260ed40` |
| `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |
| `04_wiring/tests/cli.rs` | `2267e80473a7398dfa65b619c2eaa248663dd36793283b3f0a08bf328ca51bc0` |

Os quatro hashes históricos dos ficheiros com testes identificam a árvore no
instante RED; os três consumers também receberam edição produtiva depois disso.
Por isso, a diferença dos hashes integrais não implica alteração dos corpos de
teste. A suíte L4, que não contém produção, preservou exatamente o mesmo hash.

Artefatos do segundo restart v3 no instante RED final:

| Artefato | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/foundations/selector.rs` | `b3eea776281f6e78fb1f70b02b9198493d8466a08f048322a6eacf71c9a9fb10` |
| `01_core/src/compiler/eval/selector_matching.rs` | `001b178fd1af340de5773c4f4966a887dffbabd86bdb9df1fa48d0e035a6efe2` |
| `01_core/src/compiler/eval/tests.rs` | `8e927dd1b981c58c6cdf2e9f033f0ba900ca90de2a6957bfedecd40b423c259a` |
| `01_core/src/compiler/eval/rules.rs` — controle produtivo não editado | `0fb23fe5e6e0a086a9d0b8c88bf93439402ca7b0e319f09e0e9378c398c82b5e` |

`rustfmt --edition 2021 --check` passou nos três ficheiros de teste v3.

Artefatos do restart final v4 no instante RED congelado:

| Artefato | SHA-256 |
|---|---|
| `01_core/src/compiler/eval/tests.rs` | `e0e59d6e7f7af88f2b6a65752716207d043334969f165e77739cfeb478d40b15` |
| `01_core/src/compiler/eval/math.rs` — controle produtivo não editado | `192a6deb62222e911a11510f10fe53b65dc77b5c942d0e4a4cf6d8b6b0ee7ece` |
| `prompts/compiler/eval/math.md` — full hash congelado | `7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2` |
| `diagnosticos/p1285-contract-receipt.md` — contrato v4 | `8f6c94e6046574f39f8c913076c5b455914fbce3b6e74a5af05157e82b95f588` |

`rustfmt --edition 2021 --check 01_core/src/compiler/eval/tests.rs` passou
antes da medição RED v4.

Artefato L2 congelado no addendum v4+precision:

| Artefato | SHA-256 |
|---|---|
| `02_shell/src/cli.rs` | `c19f09578ab3392d23ed64300286a0e9890358fab8f6da11012e692b33242185` |
| `diagnosticos/p1285-contract-receipt.md` — v4+precision | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |

`cargo fmt --all -- --check` e
`git diff --check -- 02_shell/src/cli.rs` passaram às
`2026-08-30T15:59:15-03:00`. O hash RED foi congelado e comunicado
imediatamente depois. Após a liberação da candidata, o hash integral de
`cli.rs` mudou para
`e00dbc1f445f6c3ab1976740daeebb4c4e2a6668fb409d8c25d14ae0e206857b`
às `2026-08-30T16:00:04-03:00`; esse estado posterior não substitui nem é
alegado como snapshot da execução RED.

O primeiro congelamento L2 deste addendum, SHA-256
`c84f67c49503d758e75a052b649ab2a68cbd2e80b65f5ea69f90e84ecb975210`,
foi invalidado antes do fechamento porque o esperado figure confundia campos
canónicos preservados com violações: omitia `alt`/`scope`, incluía
`caption.position` e não transportava os defaults/counters completos. A
autoridade coordenadora forneceu a correção; somente o bloco de teste foi
ajustado, os dois focais foram repetidos e o novo hash acima substitui aquele
predecessor malformado.

O congelamento seguinte, SHA-256
`551fabdd5b1e6d077579a63747a932c41fea439703720b146648bff057256228`,
também foi invalidado como artefato integral de teste: os dois
`GradientStop` usavam `(255, 0, 0)`/`(0, 0, 255)`, enquanto o canónico
congelado provém das cores nomeadas Typst `red`/`blue`, isto é,
`(255, 65, 54)`/`(0, 116, 217)`. Somente esses dois RGBs foram corrigidos; o
esperado canónico não mudou. O hash `651c7e…fd7b9` substituiu naquele instante
o predecessor com RGBs incorretos.

Esse hash `651c7e…fd7b9` foi depois invalidado como fixture E2E integral: embora
as cores gradient estejam corretas, o rect construía width/height como
`Value::Relative`, carrier que já continha a forma esperada e não exercitava a
normalização pública exigida para `Value::Length`. O hash
`c19f09…42185` acima substitui-o e reproduz o carrier real sem mudar o esperado.

## 6. Veredito limitado

Os dez testes iniciais e os seis testes v3 permanecem proveniência histórica,
mas contratos v2/v3 e candidatas v1/v2 não são evidência final. O teste v4
deriva diretamente de C-MATH 1–3 e discrimina M-M1, mantendo M-M2 como
controle positivo interno.

Classificação causal final: `ADDENDUM_RED_38_42_FINAL_CARRIER`. O RED
morfológico v4 e as apresentações L2 anteriores permanecem como proveniência
causal; a reexecução eval com fixture gradient válido foi GREEN, e o carrier
rect corrigido é RED atual contra a candidata v3 somente em width/height.
Nenhum `Unknown` foi convertido em sucesso. Esta autoridade corrigiu apenas os
testes e não a candidata.

Este receipt não é veredito de equivalência geral. O runner black-box congelado,
o gate de mutações e o refinamento permanecem sob as autoridades verificadoras
correspondentes.
