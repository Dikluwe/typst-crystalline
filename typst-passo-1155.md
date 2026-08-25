# P1155 — alinhar a expectativa L3 de UTF-8 ao path canónico

**Data:** 2026-08-25
**Estado:** `EXECUTADO — workspace integral GREEN`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** diagnóstico P1154 concluído

## 1. Objetivo

Eliminar a única falha restante de `cargo test --workspace`, alinhando a
assertion test-only de `read_binario_pipeline` à forma canónica P1141 do
caminho virtual:

```text
logo.png -> /logo.png
```

O passo deve alterar somente a expectativa em
`03_infra/src/integration_tests.rs`. Não muda produção, mensagens públicas,
resolução, L0, contrato `World` ou representação de `RootedPath`.

## 2. Proveniência de entrada

- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree não commitada P1149–P1154, a preservar.
- `typst-core`: 5.223 aprovados, zero falhas, zero ignorados.
- `typst-infra`: 836 aprovados e 1 falha entre 837 testes.
- Falha nominal:
  `integration_tests::integration::read_binario_pipeline`.
- Mensagem real:
  `failed to convert to string (file is not valid UTF-8 in /logo.png:1:1)`.
- Expectativa antiga em `03_infra/src/integration_tests.rs:1755-1757`:
  a mesma mensagem sem `/` antes de `logo.png`.

Antes de executar, registrar novamente:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
cargo test -p typst-infra read_binario_pipeline -- --exact --nocapture
```

Não atribuir a P1155 alterações anteriores da working tree.

## 3. Medição anterior à decisão

P1141 mediu no vanilla ratificado que `repr(path("logo.png"))` usa a vpath
absoluta portátil `path("/logo.png")`. O L0 vigente de path exige essa forma.

P1152 já atualizou as expectativas L1 equivalentes:

- `stdlib/loading.rs:970` espera `/logo.png:1:1`;
- `/latin1.txt` e `/multilinha.txt` seguem a mesma regra;
- todos esses testes estão GREEN.

P1154 executou o workspace e mostrou que o único RED L3 recebe exatamente a
mensagem canónica, mas compara contra o texto pré-P1141. A implementação em
`stdlib/loading.rs:501` apenas interpola o `RootedPath` já resolvido; remover a
barra na produção reintroduziria divergência entre path, repr e diagnóstico.

Classificação: expectativa test-only obsoleta. O que refutaria a decisão seria
uma sonda vanilla ratificada mostrar `logo.png` sem barra nesse mesmo
diagnóstico ou o L0 vigente decidir outra representação. Nesse caso, parar e
registrar a contradição; não alterar produção por conveniência do teste.

## 4. Alteração autorizada

Em `03_infra/src/integration_tests.rs`, no teste
`read_binario_pipeline`, substituir somente:

```text
failed to convert to string (file is not valid UTF-8 in logo.png:1:1)
```

por:

```text
failed to convert to string (file is not valid UTF-8 in /logo.png:1:1)
```

Atualizar o comentário local se necessário para declarar que a mensagem usa
a vpath canónica. Não relaxar para `contains("logo.png")`, não remover linha e
coluna, não usar regex permissiva e não aceitar simultaneamente as duas formas.

## 5. Nucleação e gate ADR-0127

L0s vigentes suficientes:

- `00_nucleo/prompts/entities/path.md`;
- `00_nucleo/prompts/contracts/world.md`;
- `00_nucleo/prompts/compiler/stdlib/loading.md`.

A alteração esperada é test-only e segue fluxo contínuo RED→GREEN. Não exige
resselo de hash porque nenhum L0 ou ficheiro com header de linhagem muda.

Parar no gate ADR-0127 antes de:

- alterar `RootedPath`, `VirtualPath`, `VirtualRoot` ou `World`;
- mudar a mensagem produzida em L1;
- retirar a barra canónica de paths;
- introduzir compatibilidade pública com duas representações;
- mudar defaults ou fases do pipeline.

## 6. RED→GREEN

1. Reproduzir o RED isolado e registrar a mensagem real completa.
2. Confirmar a decisão contra a medição P1141/L0 e, se necessário, uma sonda
   vanilla equivalente.
3. Alterar a única assertion L3 autorizada.
4. Executar o teste isolado e exigir 1 aprovado, zero falhas.
5. Executar todos os testes de `typst-infra` e exigir 837 aprovados.
6. Executar `typst-core` e confirmar 5.223 aprovados.
7. Executar o workspace integral e confirmar zero falhas em todos os crates.
8. Atualizar P1154 com o novo estado workspace GREEN e a proveniência.

## 7. Critérios de aceitação

- somente a assertion/comentário de `read_binario_pipeline` muda em código;
- teste isolado: 1 aprovado, zero falhas;
- `typst-infra`: 837 aprovados, zero falhas, se o total não mudar;
- `typst-core`: 5.223 aprovados, zero falhas, se o total não mudar;
- `cargo test --workspace` termina com exit status zero;
- nenhuma assertion é relaxada ou ignorada;
- nenhum código de produção, L0, contrato ou default muda;
- `crystalline-lint .` termina sem violations ou V5;
- diagnóstico P1154 deixa de declarar o workspace aberto e registra o GREEN;
- nenhum commit é criado.

## 8. Validação

```text
cargo test -p typst-infra read_binario_pipeline -- --exact --nocapture
cargo test -p typst-infra --lib
cargo test -p typst-core --lib
cargo test --workspace
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Warnings históricos permanecem fora do scope e não devem ser descritos como
resolvidos.

## 9. Handoff

Se o workspace ficar GREEN, atualizar o estado de P1155 para executado e o
diagnóstico P1154 para fechamento integral. O próximo ato será revisão humana
da working tree e decisão explícita de empacotamento; não há nova
implementação automática nem autorização implícita para commit.

## 10. Execução e resultado

### Proveniência e RED

- Hora: `2026-08-25T07:25:58-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree não commitada P1149–P1154 preservada.
- RED isolado: 0 aprovados, 1 falha; mensagem real continha
  `/logo.png:1:1` e a assertion esperava `logo.png:1:1`.

### Alteração

Em `03_infra/src/integration_tests.rs`, somente o comentário e a string exata
da assertion de `read_binario_pipeline` foram atualizados para a vpath
canónica `/logo.png`. A comparação não foi relaxada. Nenhum código de
produção, L0, contrato, default ou fase mudou; ADR-0127 não foi acionado.

### GREEN

- teste isolado: 1 aprovado, zero falhas;
- `typst-infra`: 837 aprovados, zero falhas, zero ignorados;
- `typst-core`: 5.223 aprovados, zero falhas, zero ignorados;
- `cargo test --workspace`: exit zero; todos os crates sem falhas;
- check, build, fmt e `git diff --check`: passaram;
- `crystalline-lint .`: exit zero, sem violations ou V5.

O diagnóstico P1154 foi atualizado para fechamento integral GREEN. Nenhum
commit foi criado; resta a revisão humana e decisão explícita de
empacotamento.
