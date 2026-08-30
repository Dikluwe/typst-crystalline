# P1179 — auditar migração global de hashes do linter atualizado

**Data:** 2026-08-25
**Estado:** `EXECUTADO — DECISÃO C; MIGRAÇÃO BLOQUEADA`
**Baseline:** P1178.1 GREEN, working tree acumulada e índice vazio
**Objeto:** `crystalline-lint` instalado em 2026-08-25T16:18:16-03:00
**Classe ADR-0127:** manutenção interna de linhagem; sem contrato público

## Objetivo

Auditar, explicar e preparar separadamente a migração global de hashes
acionada pela versão atualizada do `crystalline-lint`. Determinar por que
`--fix-hashes` tenta alterar centenas de consumers, por que alguns paths
reportam dois hashes candidatos (`hash-a`/`hash-b`) e por que metadata
canônica duplicada produz `Partial write`.

Não incorporar a migração ao P1178.1 nem alterar código funcional. Aplicação
massiva só ocorre depois de dry-run reproduzível, classificação completa e
prova de que nenhuma mudança da working tree será sobrescrita.

## 1. Proveniência obrigatória

Registrar antes de qualquer comando mutante:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
git diff --cached --quiet
which crystalline-lint
stat /home/dikluwe/.cargo/bin/crystalline-lint
sha256sum /home/dikluwe/.cargo/bin/crystalline-lint
crystalline-lint --help
```

Baseline já observado, a reproduzir:

- binário modificado em `2026-08-25 16:18:16.689710851 -0300`;
- SHA-256
  `1265a0d534274c07b9d44fb152507f1c1a9052528236876394b260fecbbfb466`;
- a primeira execução mutante tentou tocar 738 tracked paths no total;
- 732 paths externos ao escopo HTML foram restaurados ao HEAD;
- o lint posterior terminou com exit 0 e reportou 421 avisos V5 globais;
- os seis tracked paths acumulados anteriores permaneceram preservados;
- índice permaneceu vazio.

Todo número novo deve registrar hora, HEAD e `git diff HEAD --stat`.

## 2. Restrições de segurança

- não acessar/listar `00_nucleo/context/` ou `00_nucleo/materialization/`;
- não executar `--fix-hashes` sem `--dry-run` na fase de auditoria;
- não usar `git restore`, `checkout`, `reset` ou limpeza global;
- não modificar prompts, código funcional, testes ou contratos;
- não stagear nem commitar;
- preservar integralmente a working tree P1173.1–P1178.1;
- qualquer aplicação futura deve usar manifest explícito de paths e comparar
  o estado imediatamente anterior e posterior.

## 3. Identificar a mudança do linter

Localizar a proveniência da instalação sem rede:

```text
cargo install --list
readlink -f /home/dikluwe/.cargo/bin/crystalline-lint
```

Se fonte/checkout local da versão instalada existir, medir:

- commit/tag e estado da fonte;
- implementação de V5 e `--fix-hashes`;
- definição de hash L0 e de `Hash do Código`;
- parser de metadata canônica;
- condição que produz `hash-a`, `hash-b`, `Applied` e `Partial write`;
- mudança face à versão anterior, se a fonte anterior estiver disponível.

Se não houver fonte anterior local, não inventar diff histórico: classificar
somente comportamento observado e declarar o refutador.

## 4. Dry-run selado

Executar e guardar fora do repositório:

```text
crystalline-lint --fix-hashes --dry-run .
```

Capturar stdout, stderr e exit code em `/tmp`, acompanhados do SHA-256 dos
logs. Não usar redirecionamento para paths tracked.

Produzir manifest determinístico:

```text
path | prompt | hash atual | hash-a | hash-b | ação |
metadata canônica | classe | decisão
```

Contar e separar:

1. `Applied` simples, um consumer/um prompt;
2. `Partial write` por metadata canônica não única;
3. múltiplos consumers do mesmo prompt com hashes de código distintos;
4. paths da working tree acumulada;
5. paths limpos no HEAD;
6. warnings V5 sem ação mutante proposta;
7. qualquer alteração fora de headers de linhagem.

As contagens devem fechar exatamente com o total do dry-run. Duplicatas e
paths sem classificação impedem aplicação.

## 5. Triangulação mínima

Escolher pelo menos um representante de cada classe, incluindo:

- `01_core/src/compiler/eval/bibtex.rs` — `Applied` simples observado;
- `01_core/src/compiler/eval/bibliography.rs` — `Partial write` observado;
- um prompt compartilhado por múltiplos consumers;
- `01_core/src/compiler/stdlib/html.rs` — consumer modificado legitimamente;
- `03_infra/src/export/html.rs` — consumer modificado legitimamente.

Para cada representante, calcular independentemente os hashes conforme o
algoritmo medido na fonte do linter e responder:

- qual hash representa conteúdo L0;
- qual representa código/consumer;
- por que `hash-a` e `hash-b` divergem;
- se `Hash do Código` participa do próprio hash e cria ciclo;
- se metadata duplicada é erro de dados, ambiguidade legítima ou bug do
  migrador;
- qual alteração mínima produz idempotência.

Rodar o dry-run duas vezes sem mutação e provar logs/manifest idênticos.

## 6. Teste de idempotência em cópia isolada

Não testar a migração diretamente na working tree compartilhada. Criar uma
cópia temporária apenas dos prompts/consumers representantes e da configuração
necessária, ou usar mecanismo de fixture oficialmente suportado pelo linter.

Na cópia:

1. executar `--fix-hashes` uma vez;
2. executar novamente em dry-run;
3. exigir zero alterações na segunda passagem;
4. executar lint com `--fail-on warning`;
5. provar que somente metadata de linhagem mudou.

Se o linter exigir o repositório completo ou Git real e não houver isolamento
seguro, parar e registrar o bloqueio; não usar a árvore compartilhada como
fixture destrutiva.

## 7. Decisão após evidência

Classificar uma das saídas:

### A — migração global válida e idempotente

O novo formato é inequívoco, o dry-run fecha, a segunda passagem é vazia e
todos os diffs são exclusivamente metadata. Escrever um passo de aplicação
separado, preferencialmente sobre árvore limpa ou commit-base explícito.

### B — migração válida com conflitos reais

Os `Applied` simples são corretos, mas prompts compartilhados/metadata
duplicada exigem correção do linter ou decisão de formato. Nuclear primeiro a
correção do linter; não aplicar subconjunto silencioso.

### C — regressão do linter

Há ciclo de hash, não idempotência, sobrescrita parcial, alteração funcional
ou escolha arbitrária entre candidatos. Produzir reprodução mínima e parar;
não migrar o repositório.

Inferências devem declarar o que as refutaria. O fato de o comando terminar
com exit 0 não prova correção da migração.

## 8. Entregáveis

Criar somente após medir:

```text
00_nucleo/diagnosticos/typst-p1179-auditoria-migracao-hashes-linter.md
00_nucleo/diagnosticos/typst-p1179-manifest-hashes-linter.tsv
```

O diagnóstico deve conter:

- proveniência do binário e da árvore;
- contagens fechadas por classe;
- representantes triangulados;
- resultado de idempotência;
- decisão A/B/C posterior à evidência;
- lista exata de paths que uma aplicação futura poderia tocar;
- explicação dos 421 V5 e dos 732 paths restaurados.

O TSV é diagnóstico, não L0. Não atualizar prompts L0 neste passo, salvo se a
auditoria provar que o próprio contrato de linhagem mudou; nesse caso redigir
o L0 correspondente e parar antes de código/aplicação.

## Critérios de aceitação

- dry-run reproduzido duas vezes sem mutação;
- SHA-256 do binário e logs registrados;
- totais fecham sem paths desconhecidos;
- `hash-a`/`hash-b` explicados pela fonte ou marcados como bloqueio;
- casos `Applied` e `Partial write` triangulados;
- idempotência comprovada em isolamento ou bloqueio seguro registrado;
- nenhuma alteração funcional, staging ou commit;
- working tree acumulada preservada;
- `git diff --check` limpo e índice vazio.

## Próximo passo condicionado

- decisão A: escrever P1179.1 para aplicar a migração em lote controlado;
- decisão B: escrever passo de correção do linter e repetir P1179;
- decisão C: abrir reprodução mínima e aguardar correção/rollback do linter.

## Resultado da execução

Executado em 2026-08-25 sobre HEAD
`ce49041de76eb64c011e990e9774a0339f979211`. Dois dry-runs byte-idênticos
classificaram 421 consumers. A fixture isolada provou escrita parcial sem
rollback e escolha do último consumer para prompt compartilhado; a segunda
passagem e o lint estrito fecharam falsamente em zero. Decisão **C**. Evidência
completa em
`00_nucleo/diagnosticos/typst-p1179-auditoria-migracao-hashes-linter.md` e
manifesto em
`00_nucleo/diagnosticos/typst-p1179-manifest-hashes-linter.tsv`. Nenhum hash foi
aplicado à árvore do produto.
