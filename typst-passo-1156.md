# P1156 — auditoria pré-commit e proposta de empacotamento P1149–P1155

**Data:** 2026-08-25
**Estado:** `EXECUTADO — auditoria limpa; parado antes de staging`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1155 com workspace integral GREEN

## 1. Objetivo

Auditar integralmente a working tree P1149–P1155, separar alterações do lote
de qualquer trabalho alheio, confirmar que L0/código/testes/documentos formam
uma unidade coerente e produzir uma proposta concreta de commit.

P1156 não cria commit. Deve parar com:

- inventário fechado dos ficheiros e hunks;
- revisão de substância e riscos;
- validação reproduzida no estado exato;
- mensagem e escopo de commit propostos;
- pedido de aprovação explícita ao dono.

## 2. Proveniência de entrada

- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Branch: `Tekt`.
- P1141–P1148 já pertencem ao HEAD.
- Working tree não commitada contém P1149–P1155.
- Última validação P1155:
  - `typst-core`: 5.223/5.223;
  - `typst-infra`: 837/837;
  - workspace: todos os crates sem falhas;
  - check/build/fmt/diff/lint: GREEN;
  - zero violations e zero V5.

Antes de executar, fotografar novamente:

```text
date --iso-8601=seconds
git rev-parse HEAD
git branch --show-current
git status --short
git diff HEAD --stat
git diff HEAD --numstat
git diff --check
```

Se HEAD, branch ou working tree tiverem mudado, rebaselinear. Não usar reset,
checkout destrutivo, stash, clean, amend ou rebase.

## 3. Escopo esperado

### 3.1 L0s

- bindings de `field_access`, `value_methods` e `call_dispatch`;
- introspecção `from_tags`;
- stdlib `counter` e `foundations/query`;
- entidade `Value`.

### 3.2 Produção L1

- counter contextual, callbacks/fixpoint e sua exposição estática;
- superfície estática de `content`;
- transporte `Content + Location` por `LocatedContent`;
- propagação da nova variante em matches, repr, equality, erros e dispatch.

### 3.3 Harness e testes

- mocks P1152 para `resolve_path` + `read_path`;
- expectations de vpath canónica;
- correções test-only P1153 de int/gradient;
- assertion L3 P1155 para `/logo.png`.

### 3.4 Documentação

- passos P1149–P1156;
- diagnóstico P1151 das 40 falhas;
- diagnóstico consolidado P1154.

Qualquer ficheiro ou hunk fora desse inventário deve ser classificado antes de
entrar no lote. Não assumir que “já estava sujo” significa “pertence”.

## 4. Revisão obrigatória por hunk

Ler `git diff HEAD -- <ficheiro>` para cada ficheiro rastreado e o conteúdo
integral de cada ficheiro novo. Construir matriz:

```text
ficheiro/hunk | passo owner | L0 legitimador | teste | risco |
pertence? | decisão
```

Para cada hunk, verificar:

1. existe explicação em P1149–P1155;
2. há L0 vigente quando o hunk é produção;
3. o header aponta para o L0 correto e o linter confirma o hash;
4. não há debug, fixture temporária, caminho físico ou sonda embutida;
5. não há assertion relaxada, skip ou ignore novo;
6. matches de `Value::LocatedContent` preservam tipo/repr/igualdade;
7. mocks P1152 permanecem test-only e sem filesystem real;
8. alterações de int/gradient P1153 não vazam para produção;
9. a única alteração L3 P1155 é comentário + expectativa canónica;
10. nenhuma mudança incidental de formatação esconde lógica alheia.

Em ficheiros sobrepostos, atribuir o hunk, não apenas o ficheiro. Se a autoria
continuar ambígua após os passos e o diff, parar e pedir revisão humana.

## 5. Revisão arquitetural

Confirmar novamente, com `file:line`:

- P1149 fecha counter sem duplicar lógica entre forma estática/instância;
- P1150 mantém a lógica de content no owner e o hub apenas delega;
- P1151 conserva Location por transporte explícito, nunca por igualdade;
- `LocatedContent` continua publicamente `content` em type/repr/equality;
- P1152 não reintroduz `read_bytes` no caminho de produção;
- P1153/P1155 corrigem somente expectativas confirmadas no vanilla/L0;
- nenhum default, contrato ou fase mudou depois dos gates aprovados.

Consultar ADR-0107, ADR-0108, ADR-0109 e ADR-0127. Não reabrir decisões
aprovadas, mas apontar qualquer implementação que não corresponda ao L0.

## 6. Estratégia de empacotamento

### Recomendação inicial: um commit coeso

Recomenda-se um único commit P1149–P1155 porque:

- P1149–P1151 sobrepõem os mesmos owners de counter/content/eval;
- separar L0 e implementação produziria estados intermediários com linhagem ou
  comportamento incompletos;
- P1152/P1153/P1155 são necessários para que o estado final tenha workspace
  GREEN;
- a divisão automática por ficheiro não consegue separar corretamente
  `stdlib/mod.rs`, `eval/tests.rs` e os bindings compartilhados;
- o lote inteiro tem uma narrativa única: fechar counter/content/location e
  restaurar o baseline após `RootedPath`.

Mensagem proposta:

```text
feat(p1149-p1155): fechar counter, content location e baseline de path
```

Corpo proposto:

```text
- completa a superfície contextual de counter e o glue estático de content
- preserva Location exata em conteúdos devolvidos por query
- migra mocks para RootedPath e elimina as 40 falhas do typst-core
- alinha expectativas de int, gradient e diagnóstico de path ao vanilla
- deixa typst-core, typst-infra e workspace integralmente GREEN
```

Uma divisão em vários commits só é aceitável se cada commit intermediário:

- tiver L0 + código + headers coerentes;
- compilar e passar `crystalline-lint`;
- não depender de hunks ainda não presentes;
- tiver baseline explicitamente descrito sem falsamente alegar GREEN.

Não executar `git add` ou `git commit` nesta fase. A proposta deve ser auditada
pelo dono primeiro.

## 7. Gate de aprovação para operações Git

Após a auditoria, parar e apresentar:

- lista exata de ficheiros que entrarão;
- lista exata de ficheiros excluídos, se houver;
- achados de revisão por prioridade;
- validações finais;
- mensagem/corpo propostos;
- recomendação de um ou vários commits.

Só um pedido posterior explícito como “pode commitar” autoriza staging e
commit. Essa aprovação não autoriza push, PR, tag, merge ou rebase.

Se aprovado depois, o passo de commit deve usar paths explícitos, conferir o
staged diff e repetir ao menos `git diff --cached --check` antes de criar o
commit. Nunca usar `git add .` sem comparar a lista final ao inventário.

## 8. Validação pré-aprovação

Executar no estado auditado:

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

Registrar números, exit status, HEAD, hora e `git diff HEAD --stat`. Warnings
históricos não são violations e não devem ser descritos como resolvidos.

## 9. Critérios de aceitação

- todos os ficheiros e hunks da working tree foram atribuídos;
- nenhum trabalho alheio ou temporário entra na proposta;
- L0, headers, código e testes são coerentes;
- nenhum achado crítico ou alto permanece sem decisão;
- `typst-core` e `typst-infra` reproduzem 5.223/5.223 e 837/837, se os totais
  não mudarem;
- workspace integral, check, build, fmt, diff e lint ficam GREEN;
- zero violations e zero V5;
- mensagem/corpo e lista de paths são apresentados ao dono;
- nenhum ficheiro é staged e nenhum commit é criado.

## 10. Handoff

Se a auditoria for limpa, parar para a decisão humana de empacotamento. Se
houver hunk ambíguo, regressão, V5 ou falha de workspace, escrever primeiro um
passo corretivo estreito; não mascarar o problema com staging seletivo.

## 11. Execução da auditoria

### 11.1 Proveniência

- Hora: `2026-08-25T07:36:24-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Branch: `Tekt`.
- Diff rastreado: 30 ficheiros, 857 inserções e 184 remoções.
- Ficheiros novos: 10 documentos P1149–P1156/diagnósticos.
- Nenhum ficheiro estava staged antes ou depois da auditoria.

### 11.2 Inventário exato proposto

L0s:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/bindings/value_methods.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/introspect/from_tags.md
00_nucleo/prompts/compiler/stdlib/counter.md
00_nucleo/prompts/compiler/stdlib/foundations/query.md
00_nucleo/prompts/entities/value.md
```

Código/testes rastreados:

```text
01_core/src/compiler/eval/bibliography.rs
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/bindings/method_dispatch.rs
01_core/src/compiler/eval/bindings/mod.rs
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/operators/equality.rs
01_core/src/compiler/eval/operators/error_formatting.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/eval/tests.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/introspect/fixpoint.rs
01_core/src/compiler/introspect/from_tags.rs
01_core/src/compiler/stdlib/counter.rs
01_core/src/compiler/stdlib/foundations/query.rs
01_core/src/compiler/stdlib/loading.rs
01_core/src/compiler/stdlib/mod.rs
01_core/src/compiler/stdlib/plugin.rs
01_core/src/compiler/stdlib/state.rs
01_core/src/compiler/stdlib/transforms.rs
01_core/src/entities/value.rs
03_infra/src/integration_tests.rs
```

Documentos novos:

```text
00_nucleo/diagnosticos/p1151-location-e-baseline-40-falhas.md
00_nucleo/diagnosticos/p1154-fechamento-path-location-baseline.md
typst-passo-1149.md
typst-passo-1150.md
typst-passo-1151.md
typst-passo-1152.md
typst-passo-1153.md
typst-passo-1154.md
typst-passo-1155.md
typst-passo-1156.md
```

Todos os 10 documentos pertencem ao lote. Não há path excluído da proposta e
não foi encontrado trabalho alheio.

### 11.3 Resultado da revisão por hunk

Nenhum achado crítico, alto, médio ou baixo acionável foi encontrado.

- P1149: mudanças de counter permanecem no owner; wrappers estáticos delegam
  e callbacks recebem componentes posicionais conforme as sondas.
- P1150: `content_type_field` apenas expõe os cinco wrappers; lógica de
  `Content` continua centralizada em `eval_content_method_at`.
- P1151: `native_query` transporta a Location exata; type/repr/equality e
  erros preservam morfologia `content`; não existe reconstrução por igualdade.
- P1152: cinco mocks usam `resolve_path`/`read_path` somente em `#[cfg(test)]`;
  não há filesystem real ou reintrodução de `read_bytes` em produção.
- P1153: int/gradient alteram apenas assertions/fixtures test-only.
- P1155: o único hunk L3 é comentário + mensagem esperada `/logo.png`.
- Não foram encontrados `dbg!`, prints de debug, `todo!`, `unimplemented!`,
  `#[ignore]` novo, sondas temporárias ou I/O novo em L1.
- Headers e L0s foram validados sem V5; os gates públicos P1148/P1151 estão
  documentados como aprovados.

Os ficheiros sobrepostos (`field_access`, `value_methods`, `call_dispatch`,
`eval/tests` e `stdlib/mod.rs`) não devem ser separados automaticamente. A
recomendação de um commit coeso foi confirmada.

### 11.4 Validação reproduzida

No estado acima:

```text
typst-core:  5.223 passed; 0 failed; 0 ignored
typst-infra:   837 passed; 0 failed; 0 ignored
cargo test --workspace: exit 0; todos os crates sem falhas
cargo check --workspace: exit 0
cargo build --workspace: exit 0
cargo fmt --all -- --check: exit 0
git diff --check: exit 0
crystalline-lint .: exit 0; zero violations; zero V5
```

Warnings históricos permanecem fora do scope.

## 12. Proposta final para aprovação

Um único commit com todos os paths da secção 11.2:

```text
feat(p1149-p1155): fechar counter, content location e baseline de path

- completa a superfície contextual de counter e o glue estático de content
- preserva Location exata em conteúdos devolvidos por query
- migra mocks para RootedPath e elimina as 40 falhas do typst-core
- alinha expectativas de int, gradient e diagnóstico de path ao vanilla
- deixa typst-core, typst-infra e workspace integralmente GREEN
```

P1156 para aqui. Nenhum `git add`, commit, push, tag ou PR foi executado. O
próximo passo depende de aprovação explícita do dono para criar o commit; push
continuará fora de escopo.
