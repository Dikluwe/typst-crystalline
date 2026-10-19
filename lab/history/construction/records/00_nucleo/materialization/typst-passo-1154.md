# P1154 — fechamento consolidado da sequência `path` → `location` → baseline GREEN

**Data:** 2026-08-25
**Estado:** `EXECUTADO — diagnóstico concluído; workspace com 1 resíduo test-only`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1141–P1153 materializados; P1149–P1153 ainda na working tree

## 1. Objetivo

Produzir um diagnóstico final, reproduzível e sem nova implementação, que
feche a sequência iniciada pela auditoria do tipo público `path` e prove quatro
resultados independentes:

1. a superfície de linguagem de `path` está medida e nuclearizada;
2. a resolução de ficheiros usa `RootedPath` sem re-resolução indevida;
3. `content.location()` preserva a Location exata de resultados de query sem
   mudar a morfologia pública de `content`;
4. a dívida de 40 falhas foi explicada e eliminada, terminando em 5.223 testes
   aprovados e zero falhas.

P1154 é um passo de auditoria, documentação e validação. Não acrescenta
features, não altera contrato público e não deve “aproveitar o fechamento”
para refactors ou limpezas adjacentes.

## 2. Proveniência inicial

- Hora da escrita: `2026-08-25T07:16:53-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree não commitada: 29 ficheiros rastreados alterados, 855 inserções
  e 182 remoções, além dos passos/diagnósticos não rastreados.
- Última suite medida em P1153: **5.223 aprovados, zero falhas, zero
  ignorados**.

Antes de executar, registrar novamente:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --check
```

Preservar todas as alterações do dono. Não usar reset, checkout destrutivo,
stash, amend ou reordenação automática da working tree.

## 3. Artefato de saída

Criar:

```text
00_nucleo/diagnosticos/p1154-fechamento-path-location-baseline.md
```

O documento deve descrever estado e evidência; não é Prompt L0. Deve apontar
para os L0s owners, ADRs, passos e linhas de código, mas não copiar grandes
trechos nem converter histórico tático em especificação perene.

Estrutura obrigatória:

1. proveniência e escopo;
2. linha temporal P1141–P1153;
3. contrato final de `path`/`World`;
4. contrato final de `content.location()`;
5. explicação e fechamento das 40 falhas;
6. matriz L0 → código → testes;
7. gates ADR-0127 acionados e não acionados;
8. validação final e resíduos reais;
9. recomendação de empacotamento/commit, sem criar commit.

## 4. Auditoria de substância por eixo

### 4.1 `path` e resolução

Reabrir somente as fontes autorizadas e vigentes:

- `AGENTS.md`;
- `00_nucleo/diagnosticos/typst-auditoria-handoff-pos-p1140.md`;
- passos P1141 até o último passo diretamente ligado a `path`;
- L0s de `entities/path`, `contracts/world` e consumers alterados;
- ADR-0107, ADR-0108 e ADR-0127.

Não listar nem varrer `00_nucleo/context/` ou
`00_nucleo/materialization/`.

Confirmar com `file:line`:

- superfície pública de linguagem e diferença entre `Path` e `Str`;
- preservação de `VirtualRoot` e vpath portátil;
- `Value::Path` segue enraizado e `Value::Str` resolve uma única vez;
- `resolve_path` não faz I/O e `read_path` recebe o caminho já resolvido;
- mocks L1 usam armazenamento em memória, não filesystem real;
- a barra inicial canónica é coerente em repr/diagnósticos/fixtures conforme o
  L0 vigente.

Qualquer afirmação sobre linguagem deve apontar para sonda ou fonte vanilla
`a51e02804`, nunca para igualdade de structs Rust.

### 4.2 `content` e `location`

Confirmar a cadeia completa:

```text
query/introspecção
  -> Content + Location exata
  -> Value::LocatedContent
  -> field/método location
  -> Value::Location
```

Verificar que:

- dois conteúdos semanticamente iguais podem carregar Locations distintas;
- igualdade, `repr`, `type` e métodos não relacionados continuam com
  morfologia pública de `content`;
- conteúdo inline sem provenance devolve `none` em `location()`;
- nenhum lookup por igualdade tenta reconstruir Location;
- todos os matches relevantes de `Value` tratam `LocatedContent` sem perder
  semântica.

Registrar que a introdução da variante pública Rust acionou ADR-0127 e que o
dono aprovou o gate antes do código. Não reabrir essa decisão no fechamento.

### 4.3 Das 40 falhas ao GREEN

Reconciliar nominalmente as contagens, sempre com a proveniência já registrada:

| Marco | Aprovados | Falhas | Explicação |
|---|---:|---:|---|
| pós-P1151 | 5.183 | 40 | 30 harness/I/O + 10 expectativas semânticas |
| pós-P1152 | 5.213 | 10 | mocks migrados para `resolve_path`/`read_path` |
| pós-P1153 | 5.223 | 0 | testes obsoletos revalidados no vanilla |

Não inferir que uma diferença numérica prova a identidade dos testes. Conferir
as listas nominais registradas em P1151–P1153 e declarar explicitamente que
nenhuma falha nova substituiu uma antiga.

Separar:

- P1152: dívida de harness, sem mudança de produção;
- P1153: expectativas test-only de `int(float)`, nomes focais e mínimo de
  stops, confirmadas nos dois vanilla;
- warnings preexistentes: não são falhas nem violations, mas também não devem
  ser descritos como resolvidos.

## 5. Matriz de linhagem

Construir no diagnóstico uma tabela mínima:

```text
eixo | Prompt L0 owner | hash declarado | ficheiro(s) de código |
testes sentinela | passo/gate | estado
```

Auditar pelo menos:

- entidade `path` e contrato `World`;
- loading/image/plugin/bibliography consumers de path;
- entidade `Value`;
- query/introspecção;
- field access e method dispatch de `content`;
- counter, porque P1149 partilha a mesma working tree;
- testes test-only P1152/P1153.

Executar `crystalline-lint .` antes de declarar hashes íntegros. Não correr
`--fix-hashes` automaticamente: se houver V5, localizar a origem e classificar
se pertence à sequência ou a alteração concorrente. Qualquer resselo deve ser
explicado e limitado ao owner correto.

## 6. Auditoria de escopo da working tree

Como vários passos estão materializados sem commit, produzir uma atribuição
por lote baseada nos documentos e no diff, sem fingir precisão quando hunks do
mesmo ficheiro se sobrepõem:

```text
ficheiro | P1149 | P1150 | P1151 | P1152 | P1153 | sobreposição/observação
```

Classificar cada ficheiro rastreado e não rastreado como:

1. pertencente ao lote;
2. preexistente/preservado;
3. ambíguo — requer revisão humana antes de commit;
4. temporário — não deve entrar em commit.

Não apagar sondas ou artefatos fora do repositório como parte dessa
classificação. Não criar commit, branch, tag ou PR sem pedido explícito.

## 7. Gate ADR-0127

O diagnóstico e as verificações read-only/test-only não acionam gate. Parar
antes de qualquer descoberta que exija:

- nova assinatura, variante, campo ou tipo público;
- mudança de default ou compatibilidade;
- mudança de fase eval/layout/render;
- revisão da representação aprovada `LocatedContent`;
- alteração do contrato público de `path` ou `World`.

Se surgir apenas erro documental interno ou hash desatualizado, atualizar L0
primeiro e seguir o fluxo aplicável; se houver dúvida de classificação, parar.

## 8. Validação final

Executar no estado exato que será descrito:

```text
cargo test -p typst-core --lib
cargo test --workspace
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Se `cargo test --workspace` revelar falhas fora de `typst-core`, não mascarar
nem converter automaticamente em scope-out: listar nominalmente, localizar a
primeira causa e decidir se é regressão da sequência ou baseline externo. O
diagnóstico só pode usar “workspace GREEN” se o comando integral terminar com
exit status zero.

## 9. Critérios de aceitação

- diagnóstico criado na pasta correta, com números reproduzíveis;
- cada decisão é precedida pela medição que a sustenta;
- matriz L0/código/testes/gates completa para os owners afetados;
- gate aprovado de `LocatedContent` registrado; demais mudanças corretamente
  classificadas como fluxo contínuo ou test-only;
- 40 → 10 → 0 reconciliado nominalmente e não apenas aritmeticamente;
- `typst-core` reproduz 5.223/5.223, se não houver mudança concorrente;
- resultado de `cargo test --workspace` registrado sem eufemismo;
- `crystalline-lint .` termina com zero violations;
- nenhum código, contrato público, default ou pipeline é alterado;
- working tree permanece preservada e nenhum commit é criado.

## 10. Handoff

Se todos os critérios forem satisfeitos, o próximo ato não é nova
implementação: é revisão humana do diagnóstico e decisão explícita sobre como
empacotar a working tree em commits. Se houver V5, falha de workspace ou
ficheiro ambíguo, escrever um passo corretivo estreito a partir dessa evidência
antes de propor commit.

## 11. Resultado da execução

O diagnóstico foi criado em
`00_nucleo/diagnosticos/p1154-fechamento-path-location-baseline.md`.

Resultados finais no HEAD `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`,
working tree não commitada, até `2026-08-25T07:21:44-03:00`:

- `typst-core`: 5.223 aprovados, zero falhas, zero ignorados;
- check, build, fmt e `git diff --check`: passaram;
- `crystalline-lint .`: exit zero, sem violations ou V5;
- `cargo test --workspace`: uma falha em 837 testes de `typst-infra`;
- falha nominal: `integration_tests::integration::read_binario_pipeline`;
- causa: assertion L3 espera `logo.png`, mas a mensagem canónica P1141 contém
  `/logo.png`; o teste L1 equivalente já foi migrado em P1152.

Nenhum código ou L0 foi alterado. P1154 encerra a auditoria, mas não autoriza
chamar o workspace de GREEN nem empacotar commits. O sucessor deve corrigir e
revalidar exclusivamente essa expectativa test-only L3.
