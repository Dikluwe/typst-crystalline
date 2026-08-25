# P1160 — fechar a matriz de paridade e auditar o lote de page numbering

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1157–P1159 materializados e GREEN na working tree

## 1. Objetivo

Fechar, com sondas reproduzíveis, os critérios secundários de P1157/P1159 que
ainda não têm guarda E2E explícita e preparar o lote para commit. Este passo
não abre nova feature nem novo contrato: mede, acrescenta testes de paridade,
corrige apenas defeitos encontrados dentro do contrato já aprovado e produz a
auditoria pré-commit.

Não iniciar `Symbol`, `emoji.heart` ou `html` neste passo.

## 2. Proveniência de entrada

Antes de qualquer edição, registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git branch --show-current
git status --short
git diff HEAD --stat
```

HEAD esperado: `34e3ffb2e06b105835939d0b4fd62e31c250211f`.
A working tree contém o lote P1157–P1159 e respectivos L0s/resselos. Não
separar nem descartar esses hunks.

Baseline ratificada no fechamento do P1159:

```text
typst-core: 5.225 passed; 0 failed
typst-infra: 841 passed; 0 failed
workspace: zero failures
crystalline-lint: exit 0; zero V5
```

## 3. Reproduzir primeiro no vanilla

Usar exclusivamente:

```text
lab/typst-original/target/release/typst
commit pinado: a51e02804
```

Para cada sonda, guardar fonte, comando, stdout/stderr, exit code, hash do
artefacto quando aplicável e hora da medição. Não usar a string `--version`
como prova isolada de proveniência.

### 3.1 Aridade visível e de referência

Medir separadamente:

1. callback unário usado como numbering visível;
2. callback ternário usado como numbering visível;
3. callback binário usado por `ref(form: "page")`;
4. callback variádico nos dois consumers.

Confirmar as mensagens observáveis, incluindo:

```text
unexpected argument
missing argument: c
missing argument: total
```

Não normalizar mensagens antes de comparar.

### 3.2 Morfologia do retorno

Medir callbacks que devolvem:

```text
42
none
"texto"
[markup forte]
```

Comparar semântica e morfologia do conteúdo, não frames ou bytes PDF. Retorno
`none` deve produzir conteúdo vazio, distinto de slot de realização ausente.

### 3.3 Escopo lexical de `page(...)`

Medir um page-run lexical com numbering funcional interno e numbering externo
distinto. A medição ratificada refutou a hipótese inicial de total local ao
page-run; o observável correto é:

```text
interno: X1/3, X2/3
externo restaurado: III
```

Confirmar que o callback, alinhamento, header/footer e counter lógico não
vazam através da fronteira do page-run.

### 3.4 Controles

Reexecutar:

- pattern simples e composto;
- `numbering: none`;
- counter lógico 7 → `N7/9`, `N8/9`, `N9/9`;
- referência unária `R7` com footer explícito;
- `repr(here().page-numbering())` para Pattern, Func e ausência.

## 4. Materializar testes RED no cristalino

Adicionar guards E2E em `03_infra/src/pipeline.rs` ou no owner de integração já
vigente. Cada teste deve nomear o observável de linguagem que protege.

Executar primeiro apenas os testes novos e registrar o RED real. Um teste que
nasce GREEN continua válido como guarda, mas deve ser classificado como
"cobertura ausente", não como defeito corrigido.

Se qualquer diferença exigir campo, assinatura pública, default ou mudança de
fase além do gate P1157, atualizar L0 e parar no ADR-0127. Correções internas
ao realizador/ciclo seguem em fluxo contínuo: L0 primeiro, resselo, RED→GREEN.

## 5. Auditoria de substância

Revisar o delta P1157–P1159, com atenção a:

1. `PageStore::from_realized` mantém todos os vetores alinhados 1:1;
2. o número lógico não é regressado ao índice físico;
3. callbacks só são executados pelo realizador com Engine em L3;
4. layouter e referências consomem apenas `Content` selado;
5. o store realizado não é sobrescrito pelo fixpoint interno de labels;
6. reexpansão contextual parte sempre do conteúdo original e não acumula
   marcadores;
7. diagnósticos e warnings do callback não são duplicados por iterações
   convergentes;
8. BibStore, posições, headings e stores runtime sobrevivem à reinjeção;
9. patterns antigos continuam no mesmo caminho observável;
10. nenhum hunk alheio ao lote foi incorporado.

Se a auditoria encontrar repetição de warning/callback causada por uma
passagem extra, medir o número de invocações no vanilla antes de decidir a
correção. Não transformar número de iterações internas em critério de língua.

## 6. Reconciliação documental

Depois das sondas GREEN:

- atualizar P1157–P1160 com resultados e proveniência;
- confirmar que `entities/numbering.md` não conserva marca de incompletude;
- confirmar que os L0s de PageStore, numbering, counter, layout, referências,
  call dispatch e pipeline descrevem o código real;
- executar `crystalline-lint --fix-hashes .` apenas depois das edições L0;
- verificar zero V5.

Não alterar ADRs neste passo.

## 7. Validação final

Executar:

```text
cargo test -p typst-core --lib
cargo test -p typst-infra --lib
cargo test --workspace
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Registrar contagens, exit codes, HEAD, estado não commitado e
`git diff HEAD --stat` usados como prova.

## 8. Gate de saída

Parar antes de `git add` ou `git commit` e entregar:

1. matriz vanilla ↔ cristalino com todas as sondas;
2. lista de ficheiros e hunks do lote P1157–P1160;
3. confirmação de zero alterações alheias;
4. riscos residuais explicitamente classificados;
5. mensagem de commit proposta;
6. pedido de autorização explícita para commitar.

Mensagem candidata:

```text
feat(page): realize functional numbering across layout and references
```

Somente após aprovação do dono o lote pode ser staged e commitado. O passo
seguinte ao commit volta à fila do handoff: evolução multi-codepoint de
`Symbol` e auditoria de `emoji.heart`, com novo gate ADR-0127.

## 9. Resultado executado

Proveniência da medição: `2026-08-25T09:33:43-03:00`, HEAD
`34e3ffb2e06b105835939d0b4fd62e31c250211f`, branch `Tekt`, working tree não
commitado contendo P1157–P1160. Vanilla usado:
`lab/typst-original/target/release/typst`, pin `a51e02804`.

### Matriz ratificada

| Sonda | Vanilla | Cristalino final |
|---|---|---|
| callback unário visível | `unexpected argument` | igual |
| callback ternário visível | `missing argument: c` | igual |
| callback binário em ref | `missing argument: total` | igual |
| retorno `42` | `42` na primeira margem | igual |
| retorno `none` | segunda margem vazia | igual |
| page-run lexical | `X1/3`, `X2/3`, `III` | igual |
| markup forte | estilo forte preservado | igual no PagedDocument |

A hipótese escrita inicialmente (`X1/2`, `X2/2`, `II`) foi refutada pela
medição e corrigida antes do código.

### REDs e correções

1. parâmetro posicional obrigatório ausente recebia `none` → `closures.rs`
   agora emite `missing argument: {nome}`;
2. a vista unária era realizada mesmo sem `ref(form: "page")` → realização
   passa a ser lazy e limitada às páginas alvo desse consumer;
3. `pagebreak()` adjacente ao fim de page-run materializava uma quarta página
   vazia → consome a boundary já criada;
4. callback marginal era reduzido a plain text → layout normal em sub-frame;
5. background/foreground ficavam fora de shaping, seleção e subset de fontes
   → todos os walkers atravessam as três camadas.

Artefactos PDF finais reproduziram `42` e `X1/3`, `X2/3`, `III` por
`pdftotext -layout`. Os cinco testes P1160 ficaram GREEN.

### Validação focada

```text
typst-core: 5.225 passed; 0 failed
typst-infra: 846 passed; 0 failed
```

### Fechamento integral

Proveniência final: `2026-08-25T09:48:29-03:00`, mesmo HEAD
`34e3ffb2e06b105835939d0b4fd62e31c250211f`, working tree não commitado com
74 paths (`git diff HEAD --stat`: 68 paths rastreados, 1.091 inserções e 137
remoções; seis ficheiros novos não entram nesse stat).

```text
P1160 focado:       5 passed; 0 failed
typst-core:     5.225 passed; 0 failed
typst-infra:      846 passed; 0 failed
typst-shell:       53 passed; 0 failed
wiring/CLI:        55 passed; 0 failed
demais suites:      6 passed; 0 failed; 3 ignored
cargo check --workspace: exit 0
cargo build --workspace: exit 0
cargo fmt --all -- --check: exit 0
git diff --check: exit 0
crystalline-lint .: exit 0; zero V5
```

O lint conserva advisories históricos V16–V20 fora do gate de drift; nenhum
foi introduzido como contrato do lote. A inspeção do inventário não encontrou
alteração alheia a P1157–P1160: o hunk em `foundations/path.rs`, por exemplo,
é somente o resselo automático do owner `compiler/eval.md`.

Risco residual classificado: o callback funcional pode alterar o número de
páginas e exigir o fixpoint limitado a cinco passagens; convergência além desse
limite continua fora do contrato. A vista unária ficou limitada às páginas
realmente referenciadas, removendo a execução lateral em páginas sem consumer.

Nenhum staging ou commit foi realizado. Mensagem proposta:
`feat(page): realize functional numbering across layout and references`.
