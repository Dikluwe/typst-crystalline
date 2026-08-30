# P1164 — auditar e commitar o lote público `Symbol`

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; COMMIT ÚNICO CRIADO`
**Baseline:** commit `4ed7f6a8d`; vanilla ratificado `a51e02804`
**Pré-condição:** P1161–P1163 executados; P1163 GREEN; nenhum path staged

## Objetivo

Fechar transacionalmente a auditoria e nucleação do tipo público `Symbol`:
rever o diff acumulado P1161–P1163, excluir qualquer alteração estranha ao
lote, revalidar os gates essenciais, stagear somente os paths auditados e
criar um commit único. Este passo não adiciona comportamento, não atualiza L0
e não inicia HTML nem YAML.

## Proveniência inicial obrigatória

Registar antes da revisão:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
```

Confirmar:

- HEAD ainda é `4ed7f6a8d9d9943b74191444e1e3c23f8f584785`;
- o índice está vazio;
- existem 40 paths rastreados alterados e P1161–P1164 untracked;
- não houve mudança posterior às validações finais do P1163, exceto a escrita
  deste passo.

Se qualquer condição divergir, medir novamente e explicar antes de stagear.

## Ordem obrigatória

### 1. Auditar o diff por intenção

Ler `AGENTS.md` e os relatórios P1161, P1162 e P1163. Depois revisar:

```text
git diff --check
git diff --stat HEAD
git diff --numstat HEAD
git diff HEAD -- 00_nucleo/prompts
git diff HEAD -- 01_core/src/entities/symbol.rs
git diff HEAD -- 01_core/src/entities/value.rs
git diff HEAD -- 01_core/src/compiler/eval
git diff HEAD -- 01_core/src/compiler/stdlib
git diff HEAD -- 02_shell/build.rs 02_shell/src/cli.rs
git diff HEAD -- 04_wiring/tests/cli.rs
```

Classificar todos os paths numa destas quatro famílias:

1. L0s que legitimam `Symbol`, seus consumers, `emoji.heart`, `repr` e CLI;
2. entidade/consumers alterados pela migração `char → EcoString`;
3. tabelas `sym`/`emoji` convertidas mecanicamente e a entrada `heart`;
4. testes e linhagem/hash decorrentes do lote.

Verificações específicas:

- nenhuma tabela existente mudou de grapheme além da conversão de tipo;
- `emoji.heart` conserva base U+2764 U+FE0F e as 22 variants ratificadas;
- `symbol("♥️")` conserva U+2665 U+FE0F, sem confundir os dois hearts;
- as alterações pequenas em consumers (`bibliography`, `flow`, `rules`,
  `path` etc.) são somente adaptação ao novo campo/string;
- `02_shell/build.rs` mudou apenas o hash de linhagem causado pelo L0 CLI;
- nenhuma alteração de HTML, YAML, pipeline ou API pública adicional entrou;
- não há artefactos temporários, snapshots gerados ou ficheiros fora do lote.

Se um path não couber inequivocamente nessas famílias, parar antes de staging.

### 2. Revalidar o estado auditado

Como P1163 já executou a suite integral no mesmo estado de código, reexecutar
os gates sensíveis ao fechamento:

```text
cargo test -p typst-core p1162_
cargo test -p typst-core p1163_
cargo test -p typst-shell p1163_
cargo test -p typst-wiring --test cli p1163_
cargo check --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Exigir zero falhas e zero `V5`. Se qualquer comando alterar ficheiros, voltar
à etapa 1 e auditar o delta novo antes de continuar.

Não é necessário repetir `cargo test --workspace` neste passo se HEAD e o
diff de código forem exatamente os validados no P1163; o relatório P1163 já
regista 5.232 core, 846 infra, 55 shell, 57 CLI e zero falhas. Se o estado não
for idêntico, repetir a suite integral.

### 3. Stagear lista explícita e fechada

Construir a lista a partir de `git status --short`, confrontá-la com a
classificação da etapa 1 e usar `git add` com paths explícitos. Incluir:

- os 40 paths rastreados auditados;
- `typst-passo-1161.md`;
- `typst-passo-1162.md`;
- `typst-passo-1163.md`;
- `typst-passo-1164.md`.

Não usar `git add .`, `git add -A` nem glob. Não stagear qualquer path que
apareça depois da lista ter sido fechada sem nova auditoria.

### 4. Auditar o índice antes do commit

Executar:

```text
git diff --cached --check
git diff --cached --stat
git diff --cached --name-status
git status --short
```

Confirmar 44 paths staged e nenhum path unstaged/untracked. Inspecionar ainda
o diff cached dos pontos de maior risco:

```text
git diff --cached -- 01_core/src/entities/symbol.rs
git diff --cached -- 01_core/src/compiler/stdlib/emoji.rs
git diff --cached -- 01_core/src/compiler/stdlib/sym.rs
git diff --cached -- 02_shell/src/cli.rs
```

O volume das tabelas não dispensa a verificação semântica definida na etapa
1. Se o índice divergir do working tree auditado, parar e não commitar.

### 5. Criar o commit

Com todos os gates GREEN, criar exatamente um commit:

```text
feat(symbol): preserve multi-codepoint graphemes and emoji variants
```

Depois registar:

```text
git rev-parse HEAD
git show --stat --oneline --decorate --no-renames HEAD
git status --short
```

Critério final: novo HEAD contém somente o lote auditado e a working tree fica
limpa. Não fazer push, merge, rebase, tag ou novo commit.

## Critérios de aceitação

- todos os 44 paths são justificados pelo lote P1161–P1164;
- nenhuma mudança alheia é stageada;
- testes focados, check, fmt, diff-check e lint ficam GREEN;
- zero drift `V5`;
- commit único criado com a mensagem especificada;
- working tree limpa após o commit;
- HTML e YAML permanecem inalterados.

## Próximo passo

Com o lote `Symbol` commitado, escrever P1165 para medir e auditar a
superfície HTML feature-gated contra o vanilla ratificado `a51e02804`. P1165
deve medir antes de decidir, identificar os L0s donos e parar no gate ADR-0127
antes de qualquer novo contrato público, flag, formato ou comportamento por
defeito.

## Resultado executado

Proveniência pré-commit: `2026-08-25T12:38:54-03:00`, HEAD
`4ed7f6a8d9d9943b74191444e1e3c23f8f584785`, índice vazio, 40 paths
rastreados alterados e P1161–P1164 untracked.

A auditoria classificou os 44 paths nas quatro famílias previstas. Os
consumers pequenos contêm apenas resselo ou adaptação `ch → value`; o diff de
`02_shell/build.rs` é somente o hash do L0 CLI. `sym.rs` e a tabela simples de
`emoji.rs` conservam nomes e graphemes durante a migração de tipo. A única
adição de tabela é `emoji.heart`, com base U+2764 U+FE0F e as 22 variants
medidas. Não entraram HTML, YAML, mudança de pipeline ou artefactos gerados.

Revalidação pré-commit:

```text
P1162 core focado: 6 passed; 0 failed
P1163 core focado: 1 passed; 0 failed
P1163 shell:       2 passed; 0 failed
P1163 CLI:         2 passed; 0 failed
cargo check --workspace: exit 0
cargo fmt --all -- --check: exit 0
git diff --check: exit 0
crystalline-lint .: exit 0; zero V5
```

O commit e o seu hash são a própria transação descrita por este documento;
a proveniência pós-commit fica no objeto Git e no handoff da execução. Nenhum
push, merge, rebase ou tag pertence a este passo.
