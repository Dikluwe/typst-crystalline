# Passo 83.6 — Recuperação do relatório do Passo 83.5

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — estado actual pós-Passo 84.6.
- `00_nucleo/relatorios/` — directório onde o relatório do 84.7 já
  vive; destino deste passo.
- Mensagens da conversa do Passo 83.5 — contêm o sumário executivo
  original mas não o ficheiro completo.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.7 concluído.

---

## Natureza deste passo

**Passo de recuperação documental.** Não altera código, não altera
`DEBT.md`, não altera ADRs. Tem **uma única tarefa**: regenerar o
relatório que o Passo 83.5 deveria ter produzido como ficheiro e
que só existe fragmentado na conversa.

Nota histórica: o enunciado original do Passo 83.5 definiu o formato
do relatório em "Tarefa 4 — Formato do relatório" mas não especificou
o caminho de ficheiro onde deveria ser persistido. O Claude Code da
altura produziu o conteúdo como resposta na conversa; nenhum ficheiro
foi criado em `00_nucleo/relatorios/`. O relatório do Passo 84.7 (já
persistido correctamente) estabeleceu o padrão que este passo aplica
retroactivamente.

---

## Abordagem: regeneração, não reconstituição

Duas abordagens possíveis foram consideradas:

- **Reconstituir** a partir do histórico da conversa do 83.5. Rejeitada
  — o histórico tem fragmentos mas não os outputs completos dos
  comandos `grep` executados. Reconstituir produziria documento
  incompleto e fraco.

- **Regenerar** executando os mesmos comandos do 83.5 agora. Adoptada,
  com salvaguarda: o estado do código mudou entre o 83.5 e agora
  (DEBT-21, 22, 36, 37, 38 encerraram; DEBT-39 abriu). Se a regeneração
  corresse contra o estado actual, o relatório identificaria DEBTs como
  "em aberto" quando já foram encerrados — informação falsa.

**Solução**: a regeneração corre contra o **estado do código em
`00_nucleo/DEBT.md` na data do Passo 83.5**. Como o DEBT.md foi
reorganizado pelo próprio 83.5 e depois modificado pelos 84.1–84.6,
precisamos de reconstituir o estado pós-83.5 **via git**.

---

## Tarefa 1 — Localizar o commit do Passo 83.5

```bash
# Localizar o commit que corresponde ao fim do Passo 83.5.
# Procurar por mensagens de commit que mencionem 83.5 ou passo-83-5
# ou DEBT.md movido para 00_nucleo/.
git log --oneline --all | grep -iE "83\.5|passo-83-5|passo 83\.5|mover DEBT|00_nucleo/DEBT" | head -10
```

Se múltiplos candidatos, identificar o último antes de qualquer
commit de 84.x. O estado pretendido é o final do 83.5, antes de
84.1 começar a mover DEBTs entre secções.

**Reportar o commit hash identificado antes de prosseguir.** Se não
houver commit identificável (o projecto pode não ter commits
granulares por passo), reportar e passar ao fallback (Tarefa 1b).

### Tarefa 1b — Fallback se commit não identificável

Se os commits não têm granularidade suficiente, reconstituir o DEBT.md
na data do 83.5 manualmente:

- Começar do DEBT.md actual.
- Reverter as alterações dos Passos 84.1, 84.4, 84.5, 84.6:
  - Desfazer consolidação DEBT-23 (84.1): voltar a ter as duas
    entradas.
  - Desfazer riscos em DEBT-1, DEBT-8 (84.1): restaurar pendências.
  - Desfazer actualização de cabeçalho em DEBT-21 (84.1): voltar a
    "MITIGADO (Passo 70)" sem "desbloqueado".
  - Mover DEBT-22, DEBT-36, DEBT-37, DEBT-38 de Secção 2 para
    Secção 1 (revertendo 84.4, 84.5, 84.6 — DEBT-38 foi aberto no
    83 e fechado no 84.2; estava em Secção 1 no momento do 83.5).
  - Remover DEBT-39 (aberto no 84.4) — não existia no 83.5.

Tudo isto apenas **numa cópia temporária** (`/tmp/debt-snapshot-83.5.md`)
para referência da regeneração. Não alterar o `00_nucleo/DEBT.md`
actual.

---

## Tarefa 2 — Executar a auditoria retrospectiva

Com o estado do DEBT.md reconstituído (do git ou manual), executar
os comandos da Tarefa 3 do enunciado original do Passo 83.5 contra
o código na altura. Se o código não mudou substancialmente desde o
83.5 (muitos DEBTs são sobre código que continua igual — DEBT-33,
DEBT-34d, etc.), os comandos podem correr directamente contra o
estado actual e produzir o mesmo output que teriam produzido em
83.5.

Para cada DEBT que estava aberto no 83.5 (conforme sumário que tu
partilhaste na altura):

**DEBTs abertos no momento do 83.5**: 1, 2, 8, 9, 21, 22, 33, 34d,
34e, 35b, 36, 37, 38.

Executar os comandos de verificação listados no enunciado original
do 83.5 (Secção "Tarefa 3 — Auditoria de cada DEBT aberto") para
cada um. Ajustar para correr contra o commit/estado da altura se
necessário.

### Comandos a executar

Copiar literalmente do enunciado original do Passo 83.5:

```bash
# DEBT-1
grep -n "styles" 01_core/src/rules/eval.rs | grep -iE "save|restore|push|pop"
grep -rn "StyleChain" 01_core/src/

# DEBT-2
grep -rn "TrackedWorld" 03_infra/src/ 01_core/src/
grep -rn "shadow\|capture" lab/parity/tests/

# DEBT-8
grep -rn "MathPrimes\|MathAlignPoint" 01_core/src/

# DEBT-9
find lab/parity/tests -name "*.rs" -exec wc -l {} \;
grep -c "#\[test\]" lab/parity/tests/parse_parity.rs

# DEBT-21
rustc --version
grep -rn "fn_addr_eq" 01_core/src/

# DEBT-22
grep -n "show_rules" 01_core/src/rules/eval.rs

# DEBT-33
grep -rn "CubicTo\|bounding\|aabb" 01_core/src/rules/layout/

# DEBT-34d
grep -A 25 "TrackSizing::Auto" 01_core/src/rules/layout/mod.rs

# DEBT-34e
grep -A 7 "Grid {" 01_core/src/entities/content.rs

# DEBT-35b
grep -n "available_width" 01_core/src/rules/layout/mod.rs

# DEBT-36
grep -rn "Value::Align\|Align2D::from_string" 01_core/src/

# DEBT-37
grep -A 10 "Content::Place" 01_core/src/rules/layout/mod.rs

# DEBT-38
grep -B 1 -A 10 "TrackSizing::Auto =>" 01_core/src/rules/layout/mod.rs \
  | grep -A 10 "row_idx"
```

Onde o código actual diverge do código do 83.5 (ex: DEBT-21, 22, 36,
37, 38 já foram resolvidos — os ficheiros mudaram), usar
`git show <commit-83.5>:<path>` para ler o conteúdo histórico:

```bash
# Exemplo — estado de eval.rs no commit do 83.5
git show <commit>:01_core/src/rules/eval.rs | grep -n "show_rules"
```

---

## Tarefa 3 — Produzir o relatório

Criar `00_nucleo/relatorios/relatorio-auditoria-debts-passo-83.5.md`
com a estrutura definida no enunciado original do 83.5 (Tarefa 4):

```markdown
# Relatório de auditoria de DEBTs — Passo 83.5 (regenerado no Passo 83.6)

**Data original**: 2026-04-XX (data do 83.5)
**Data de regeneração**: 2026-04-XX (data actual deste passo 83.6)
**Aviso retrospectivo**: este relatório foi produzido no Passo 83.6
como recuperação documental. Reflecte o estado do código **no
momento do Passo 83.5**, não o estado actual. DEBTs identificados
aqui como "em aberto" podem já ter sido encerrados em passos
subsequentes — consultar o `00_nucleo/DEBT.md` actual para o estado
vivo.

## 1. Movimentação e reorganização

[secção original: DEBT.md movido de 01_core/ para 00_nucleo/, 36
ficheiros tocados, três secções aplicadas]

## 2. DEBTs confirmadamente em aberto

[para cada DEBT em 1, 2, 8, 9, 21, 22, 33, 34d, 34e, 35b, 36, 37,
38 — confirmar estado, verificar código, dificuldade estimada]

## 3. DEBTs com divergência entre ficheiro e código

[DEBT-1 com pendências já resolvidas, DEBT-8 com MathAlignPoint
implementado, DEBT-21 com Rust 1.92 desbloqueando, DEBT-23 duplicado]

## 4. Sumário para o Passo 84

[candidatos FÁCEIS, MÉDIOS, DIFÍCEIS, FORA DE ESCOPO]

## 5. Anexo — comandos de verificação executados

[lista dos comandos grep/find/cat executados durante a auditoria
original]
```

O conteúdo concreto de cada secção é o que tu partilhaste na conversa
como sumário do Passo 83.5, expandido com os outputs reais dos
comandos (regenerados na Tarefa 2). Se algum output não for
recuperável (ex: o comportamento de um grep específico mudou entre
83.5 e agora), anotar `[output não recuperável — estado actual diverge]`
em vez de inventar.

---

## Tarefa 4 — Verificação

```bash
# Confirmar que o ficheiro existe
ls -la 00_nucleo/relatorios/relatorio-auditoria-debts-passo-83.5.md

# Confirmar que tem as 5 secções
grep -c "^## " 00_nucleo/relatorios/relatorio-auditoria-debts-passo-83.5.md
# Esperado: 5 (secções numeradas 1-5)

# Confirmar que o aviso retrospectivo está presente
grep "retrospectiv" 00_nucleo/relatorios/relatorio-auditoria-debts-passo-83.5.md

# Confirmar que nada mais foi alterado
git status | grep -v "relatorios/relatorio-auditoria-debts-passo-83.5.md"
# Esperado: zero ficheiros adicionais modificados

# Testes continuam a passar (nada de código tocado)
cargo test

# Linter continua limpo
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] Commit do Passo 83.5 localizado (Tarefa 1) ou fallback manual
  aplicado (Tarefa 1b).
- [ ] Comandos de verificação da Tarefa 3 do 83.5 original
  executados contra o estado da altura.
- [ ] Ficheiro `00_nucleo/relatorios/relatorio-auditoria-debts-passo-83.5.md`
  criado.
- [ ] Ficheiro começa com aviso retrospectivo claro.
- [ ] Ficheiro tem 5 secções conforme formato original do 83.5.
- [ ] Nenhum outro ficheiro alterado (nem código, nem DEBT.md, nem
  ADRs).
- [ ] `cargo test` mantém 911 testes a passar.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Hash do commit do 83.5 identificado, ou confirmação de que fallback
  manual foi usado.
- Número total de DEBTs auditados retrospectivamente (esperado: 13 —
  os que estavam em aberto no 83.5).
- Confirmação do caminho do ficheiro.
- Observações sobre outputs não recuperáveis, se houver.

Este passo não tem Go/No-Go — o produto é o ficheiro persistido. O
próximo passo (84.8a) parte daqui com a garantia de que o histórico
documental está completo.

---

## Nota de processo — para passos futuros

A partir do Passo 84.8a em diante, todos os passos que produzem
relatório (auditorias, diagnósticos extensos, investigações)
incluirão **explicitamente** no enunciado:

- Caminho de ficheiro obrigatório: `00_nucleo/relatorios/relatorio-<tipo>-passo-<N>.md`.
- Tarefa final de verificação que faz `ls` ao ficheiro.
- Lembrete explícito: "o sumário executivo na resposta da conversa é
  adicional, não substitui o ficheiro".

Esta regra nasceu precisamente da descoberta de que o relatório do
83.5 não tinha sido persistido — o Passo 83.6 é a recuperação; a
regra impede recorrência.
