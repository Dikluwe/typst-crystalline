# Passo 1303 — selar os spans dos gates `pdf.*` e rebaixar a dívida residual

## Estado de partida

Este passo parte do commit limpo:

```text
5b4a0d0438a535c54fdb5e74b28903c1313f5bc2
fix: seal color globals and module diagnostics through step 1302
```

O P1302 está fechado e commitado. Não reabrir, reescrever nem reinterpretar os
vereditos históricos P1300/P1301/P1301r2/P1302.

O baseline de linguagem continua sendo o vanilla ratificado `a51e02804`, pelo
binário `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

As evidências de seleção são:

- `00_nucleo/diagnosticos/p1299-feature-matrix.json`;
- `00_nucleo/diagnosticos/p1299-owner-ledger.tsv`, SHA-256
  `79261fe7f8b377b2fb28781b15e88828c8b87843e5b1d0d99cdf1e7a7355fa69`;
- `00_nucleo/diagnosticos/p1299-decision-report.md`;
- `00_nucleo/diagnosticos/p1302-final-report.md`, SHA-256
  `e421db23565da3fe60f1c1fed689d2bfa56e1380827b2b7e73f37d9ba9346211`.

Essas medições foram produzidas em estados anteriores. Servem para selecionar a
coorte, mas o executor deve reproduzir a medição no HEAD acima antes de decidir
ou editar L0/código.

## Objetivo único

Fechar a classe `DIAGNOSTIC_SPAN_DIVERGENCE` dos três fields feature-gated:

- `pdf.data-cell`;
- `pdf.header-cell`;
- `pdf.table-summary`.

Quando `a11y-extras` estiver desligada, os três diagnósticos devem preservar
mensagem, severidade, hints e cardinalidade atuais, mas ancorar somente no
identificador à direita do ponto. Quando a feature estiver ligada, os três
fields continuam funções e mantêm valor, kind e `repr` já coincidentes.

Não implementar membros ausentes, não alterar `color.map`, não remover extensões
cristalinas e não mudar o gate `html` neste passo.

## Medição anterior à decisão

No P1299, nos perfis `default` e `html`, ambos os produtos rejeitam os três
acessos com a mesma mensagem e os mesmos dois hints. A única diferença é o span:

```text
cristalino: começa na coluna 11 e cobre pdf.<field>
vanilla:    começa na coluna 15 e cobre somente <field>
```

Para os canários congelados, os ranges de bytes esperados são:

| Expressão | Range vanilla |
|---|---:|
| `repr((type(pdf.data-cell), repr(pdf.data-cell)))` | `14..23` |
| `repr((type(pdf.header-cell), repr(pdf.header-cell)))` | `14..25` |
| `repr((type(pdf.table-summary), repr(pdf.table-summary)))` | `14..27` |

Nos perfis `a11y` e `html+a11y`, os três já são `MATCH_VALUE`:

```text
"(function, \"data-cell\")"
"(function, \"header-cell\")"
"(function, \"table-summary\")"
```

A fonte vanilla entrega `field.span()` em
`lab/typst-original/crates/typst-eval/src/code.rs:347-366`. O cristalino
intercepta a falha feature-gated em
`01_core/src/compiler/eval/bindings/field_access.rs:83-98`, mas ainda passa
`access.span()` em `:91`. Logo abaixo, o fluxo geral de `Module` já usa
`access.field().span()` em `:106-113`, conforme P1301r2.

É inferência que a âncora total no ramo especial é causa suficiente das seis
observações `DIFFERENT_DIAGNOSTIC` (três paths em dois perfis). Refutam essa
inferência: mensagem/hints diferentes numa medição fresca; diferença com a
feature ligada; outro owner causal; ou necessidade de alterar API pública,
default, feature ou fase.

## Classificação ADR-0127

`ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`.

Trata-se de correção localizada de paridade de diagnóstico, sem campo de
entidade, método de trait, assinatura pública, default de produto, fase de
pipeline ou quebra de compatibilidade. Portanto o executor atualiza primeiro o
L0, ressela os hashes e continua para RED→GREEN sem nova paragem humana.

Se a medição fresca refutar essa classificação, parar antes de qualquer código e
devolver `P1303_BLOCKED_ADR0127_RECLASSIFICATION`.

## Contrato congelado `C-P1303-v1`

### Feature desligada: `default` e `html`

Para cada um dos três fields:

1. exatamente um diagnóstico primário de erro;
2. zero diagnósticos laterais;
3. mensagem exata:

   ```text
   cannot access field `<field>` because the `a11y-extras` feature is not enabled
   ```

4. hints exatos, na ordem:

   ```text
   try enabling the `a11y-extras` feature
   see https://typst.app/help/compiler-features for more details
   ```

5. span resolvível e igual ao range do identificador `<field>`, sem incluir
   `pdf` nem o ponto;
6. stderr CLI byte-idêntico ao vanilla ratificado para o mesmo argv;
7. mesma saída na ordem normal e invertida dos probes;
8. nenhuma observação `Unknown` ou `EXECUTION_UNKNOWN`.

### Feature ligada: `a11y` e `html+a11y`

Para cada field:

1. avaliação com exit `0` e stderr vazio;
2. `type(...) == function`;
3. `repr(...)` preserva o nome público medido;
4. uma chamada representativa continua a produzir a morfologia já contratada
   por `compiler/stdlib/pdf.md` P1288;
5. a execução bilateral permanece `MATCH_VALUE`.

### Sentinelas obrigatórias

- `std.nope`, `calc.nope`, `sym.nope` e `color.map.nope` continuam com mensagem
  e field-only span selados por P1301r2;
- um lookup existente em cada módulo continua a devolver o mesmo valor/kind;
- o dicionário ausente de P1301 continua com o span total contratado;
- `float("NaN").is-nan` sem chamada preserva o field-only span de P1293;
- `pdf.attach` e `pdf.artifact` continuam acessíveis sem `a11y-extras`;
- nenhum dos três fields acessíveis é exposto sem a feature;
- `html` sem feature HTML continua sendo `EXPECTED_FEATURE_DISABLED`.

Não observar ponteiros, ordem interna de inserção, layout do enum, igualdade Rust
ou passos do algoritmo. O observável é o diagnóstico e a superfície da linguagem
(ADR-0107).

## Owners L0 e allowlist

Antes de escrever testes ou código, ler integralmente e atualizar somente o que
for necessário em:

- `00_nucleo/prompts/compiler/eval/bindings/field_access.md` — owner da seleção
  do span e do diagnóstico especial;
- `00_nucleo/prompts/compiler/eval/tests.md` — owner das regressões permanentes.

`00_nucleo/prompts/compiler/stdlib/pdf.md` já possui o contrato do gate e das
funções; não o editar se a medição confirmar que disponibilidade, mensagem e
hints não mudam. Se for necessário alterar o contrato funcional do módulo,
parar e separar um novo passo owner-specific.

Allowlist máxima do produto/L0:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/tests.rs
```

Além desses, são permitidos este passo e artefatos novos `p1303-*` em
`00_nucleo/diagnosticos/`. Não editar `stdlib/pdf.rs`, entidades, feature flags,
CLI, exportadores, wiring, lab ou artefatos históricos.

## Protocolo segregado de execução

A obrigação, o oracle, o ataque, a implementação e o veredito devem ser
produzidos em contextos separados. Como o filesystem é compartilhado, nenhuma
alegação de isolamento técnico é permitida: registrar hashes de entrada/saída e
revisar cada handoff por diff literal.

### P0 — congelamento do owner

Registrar em `p1303-baseline-status.txt`:

- `git rev-parse HEAD`;
- `git status --short`;
- `git diff HEAD --stat`;
- `git diff --cached --stat`;
- timestamp com timezone;
- SHA-256 dos quatro arquivos da allowlist, do binário vanilla e de todos os
  inputs P1299 usados.

A árvore deve estar limpa exceto por este passo. Qualquer alteração inesperada
produz `P1303_BLOCKED_BASELINE_DIRTY`.

### P1 — oracle e reprodutibilidade

Construir um binário cristalino release fresco em `CARGO_TARGET_DIR` temporário
e imutável. Não reutilizar `target/release/typst`.

Executar os três canários nos quatro perfis, nos dois binários, em ordem normal
e invertida. Preservar para cada execução:

- argv integral;
- hash do binário;
- exit code;
- stdout e stderr integrais e seus SHA-256;
- duração em nanos;
- perfil/features;
- range de byte derivado do diagnóstico;
- estado completo/Unknown.

Gerar `p1303-pre-measurement.json` e `p1303-oracle-receipt.md`. O esperado é:

```text
default:   3 DIFFERENT_DIAGNOSTIC
html:      3 DIFFERENT_DIAGNOSTIC
a11y:      3 MATCH_VALUE
html+a11y: 3 MATCH_VALUE
Unknown:   0
```

Qualquer outro vetor bloqueia a decisão e exige nova análise causal.

### P2 — contrato independente

Materializar `C-P1303-v1` em `p1303-contract.md`, sem ler patch de
implementação. O contrato deve enumerar mensagem, hints, ranges, cardinalidade,
perfis positivos/negativos e sentinelas. Hashar o contrato antes do RED.

### P3 — adversário antes do patch

Criar `p1303-adversarial-plan.md` com, no mínimo, estes mutantes independentes:

1. manter `access.span()`;
2. ancorar somente `pdf`;
3. incluir o ponto no span;
4. deslocar o início um byte à esquerda;
5. deslocar o fim um byte à direita;
6. corrigir apenas `data-cell`;
7. corrigir apenas os perfis sem `html`;
8. remover ou reordenar os hints;
9. alterar aspas, hífen ou nome da feature na mensagem;
10. expor o trio sem `a11y-extras`;
11. esconder o trio com `a11y-extras`;
12. generalizar o span do dicionário para field-only;
13. regredir a projeção `std` → `global` de P1301r2;
14. alterar o sucesso de `pdf.attach`/`pdf.artifact`.

Para cada mutante, nomear o teste/probe que deve matá-lo. Mutante sem assassino
é lacuna de contrato e impede avançar.

### P4 — L0 primeiro e resselo

Acrescentar ao owner `field_access.md` uma seção P1303 com a medição, inferência,
condição de refutação, decisão field-only e proibições. Acrescentar ao owner
`tests.md` a obrigação das regressões negativas/positivas e controles.

Executar o resselo normal do projeto e confirmar:

```bash
crystalline-lint --fix-hashes .
crystalline-lint --fix-hashes --dry-run .
```

O segundo comando deve imprimir `Nothing to fix`. Inspecionar o diff: o resselo
não autoriza mudanças fora da allowlist efetivamente afetada.

### P5 — RED independente

Adicionar testes permanentes `p1303_*` em `eval/tests.rs`, derivados apenas do
contrato/oracle. Eles devem falhar no baseline pré-patch exclusivamente pelos
seis spans (`3 fields × 2 perfis`) e passar nos seis controles com a feature
ligada.

Executar:

```bash
cargo test -p typst-core p1303 -- --test-threads=1
```

Gerar `p1303-red-tests-receipt.md` com output integral/hash, lista exata dos
casos falhos e prova de que mensagem/hints/cardinalidade já coincidem. Se o RED
falhar por outro motivo, corrigir o teste, não o produto.

### P6 — implementação mínima

Somente depois do RED válido, alterar o owner causal. A implementação esperada
é fazer o ramo feature-gated usar o span do identificador retornado por
`access.field().span()`.

Não mover o gate para `stdlib/pdf.rs`, não duplicar o catálogo fora do owner,
não criar blacklist genérica, não alterar `Module`, `Scope`, `Features` nem a
assinatura de qualquer função. Se a correção mínima não satisfizer o contrato,
voltar à medição e produzir um redesenho novo antes de ampliar o patch.

Gerar `p1303-implementation-receipt.md` com diff, hashes, RED→GREEN e limites da
alegação.

### P7 — ataques e verificação independente

Aplicar cada mutante um por vez sobre o candidato, executar ao menos o teste
focal que o assassina e restaurar somente o hunk do mutante. Nunca usar comando
destrutivo de reset/checkout.

O score obrigatório é:

```text
mutantes mortos / mutantes aplicáveis = 1.0
```

Registar diff e resultado por mutante em `p1303-mutant-ledger.json`. Mutante
sobrevivente exige reforçar contrato/teste e reiniciar a verificação; não pode
ser perdoado como “equivalente” sem demonstração observável.

Depois executar, nesta ordem:

```bash
cargo fmt --all -- --check
cargo test -p typst-core p1303 -- --test-threads=1
cargo test -p typst-core p1301 -- --test-threads=1
cargo test -p typst-core p1300 -- --test-threads=1
cargo test -p typst-core
cargo test --workspace
cargo build
crystalline-lint .
crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
```

Reexecutar a matriz bilateral focal com os binários congelados. O vetor final
obrigatório é:

```text
default:   3 MATCH_DIAGNOSTIC
html:      3 MATCH_DIAGNOSTIC
a11y:      3 MATCH_VALUE
html+a11y: 3 MATCH_VALUE
DIFFERENT_DIAGNOSTIC: 0
EXECUTION_UNKNOWN:    0
```

O runner legado de “probes padrão” pode continuar rotulando falhas bilaterais
como `DIFFERENCE_OR_DISABLED`, pois ele não compara diagnósticos completos. Esse
contador não é gate deste passo. O gate é o classificador P1299/P1303 que exige
exit code e stderr idênticos.

Gerar `p1303-verification-receipt.json` com comandos, tempos, hashes, outputs e
estado exato da árvore.

### P8 — veredito e certificado

O verificador final lê contrato, oracle, plano adversarial, patch, ledger de
mutantes e gates, mas não modifica produto. Deve emitir um dos vereditos:

- `P1303_CERTIFIED`;
- `P1303_BLOCKED_BASELINE_DIRTY`;
- `P1303_BLOCKED_MEASUREMENT_REFUTED`;
- `P1303_BLOCKED_L0`;
- `P1303_BLOCKED_RED`;
- `P1303_BLOCKED_MUTANT_SURVIVED`;
- `P1303_BLOCKED_VERIFICATION`;
- `P1303_BLOCKED_SCOPE`;
- `P1303_BLOCKED_ADR0127_RECLASSIFICATION`.

Criar `p1303-final-report.md`, `p1303-certificate.json` e um manifesto detached,
sem ciclos de auto-hash. O certificado deve pinar todos os artefatos e declarar
explicitamente que a alegação se limita aos três spans feature-gated.

## Orçamento de redesenho e retorno decrescente

Há três tentativas causais permitidas:

1. trocar a âncora no ramo especial para `access.field().span()`;
2. se a AST não preservar o span, transportar explicitamente o `Span` do field
   dentro de `eval_field_access`, sem mudar assinatura pública;
3. se o gate especial estiver causalmente incorreto, alinhar o lookup interno ao
   fluxo vanilla no mesmo owner, preservando catálogo, mensagem e hints.

Cada tentativa deve ser precedida por uma nova hipótese refutável e seguida pelo
mesmo conjunto focal. Se três tentativas falharem pela mesma causa, parar com
evidência; não ampliar para entidades, stdlib ou CLI por conveniência.

## Inventário residual após um PASS

O ledger P1299 tinha 166 paths. P1300/P1301r2 fecharam as três
`L0_CONTRADICTION`. Se P1303 fechar os três spans sem regressão, permanecem 160
paths classificados naquele universo:

| Classe residual | Paths | Tratamento |
|---|---:|---|
| `MISSING_LANGUAGE_MEMBER` | 106 | dívida funcional real; selecionar coortes por owner |
| `WRONG_PUBLIC_KIND_OR_IDENTITY` | 11 | `color.map` e dez mapas; corrigir identidade/`repr` em passo próprio |
| `INTENTIONAL_PRODUCT_EXTENSION` | 42 | não é defeito aceito; reabrir apenas com decisão de contrato e gate humano |
| `EXPECTED_FEATURE_DISABLED` | 1 | `html` sem feature; comportamento esperado |

Além do ledger, P1301r2 deixou explicitamente fora de escopo a divergência
`repr(std)` (`<module global>` no vanilla versus `module(std)` no cristalino).
Ela deve entrar no próximo rebaseline como item separado de identidade pública,
sem ser mascarada por mensagens de field ausente.

O P1303 deve terminar com uma recomendação para rebaseline incremental do HEAD
certificado antes de escolher o próximo lote. O candidato funcional mais coeso
é o trio do mesmo owner `json.encode`/`toml.encode`/`yaml.encode`; isso é apenas
hipótese de seleção, não autorização de implementação. A matriz fresca pode
selecionar outro lote se custo, gate ou causalidade forem melhores.

## Critério de encerramento

O passo só fecha quando:

- o baseline fresco reproduz exatamente `6` divergências de span e `0` Unknown;
- L0 é atualizado antes do código e os hashes estão selados;
- o RED falha apenas nos spans previstos;
- os `14` mutantes aplicáveis são mortos (`score = 1.0`);
- os quatro perfis terminam sem `DIFFERENT_DIAGNOSTIC` para os três paths;
- P1300/P1301 e todos os gates do workspace permanecem verdes;
- o diff está contido na allowlist;
- o relatório limita a alegação e publica o inventário residual ajustado.

Não fazer staging nem commit sem pedido explícito do dono.
