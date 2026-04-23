# Passo 96.6 — Nota de visibilidade na ADR-0037 e abertura de DEBT-47

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR`. Regra 3 actual trata hierarquia de submódulos e
  paths entre submódulos, mas não aborda visibilidade (quando
  usar `pub(super)`, quando preferir métodos sobre campos, etc.).
- `00_nucleo/DEBT.md` — DEBT-46 em aberto com 9 checkboxes.
  Este passo **renumera** os checkboxes (um novo passo entra
  entre 96.5 e o que era 96.6 anterior). Abre também nova
  entrada DEBT-47.
- Reportes dos Passos 96.1, 96.2, 96.4, 96.5 — forneceram
  evidência empírica sobre como a visibilidade foi escolhida
  (ou não escolhida) durante a reestruturação.

Pré-condição: `cargo test` — 748 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.5 concluído.

---

## Natureza deste passo

Sub-passo de governança. Não altera código. Três tarefas:

1. **Tarefa A** — Adicionar nota de visibilidade à Regra 3 da
   ADR-0037. Torna explícita a preferência por métodos sobre
   campos públicos, e documenta as opções de visibilidade
   (`pub(super)`, `pub(in path)`, `pub(crate)`).

2. **Tarefa B** — Abrir DEBT-47 documentando a auditoria de
   visibilidade necessária nos ficheiros reestruturados pelos
   Passos 96.1–96.5. A auditoria é trabalho futuro (depois de
   DEBT-46 encerrar), não parte deste passo.

3. **Tarefa C** — Renumerar os checkboxes do DEBT-46 em
   consonância com a inserção deste passo. A série antiga
   96.6–96.9 passa a 96.7–96.10.

Justificação: a nota precisa de estar activa antes do próximo
passo de reestruturação (`layout/mod.rs`, agora Passo 96.7)
porque o `Layouter<M, S>` é candidato natural a ter muitos
campos partilhados. Aplicar a regra na reestruturação mais
complexa é preferível a aplicá-la retroactivamente.

Regra absoluta: altera apenas:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`
- `00_nucleo/DEBT.md`

---

## Decisões formalizadas neste passo

- Adendo à Regra 3 da ADR-0037 (não é ajuste maior — é
  clarificação que complementa os 4 ajustes do Passo 96.3).
- DEBT-47 aberto, a pagar após DEBT-46 encerrar.
- Série de sub-passos do DEBT-46 prolongada de 96.9 para 96.10
  (novo passo inserido entre 96.5 e o antigo 96.6).

---

## Tarefa A — Nota de visibilidade na Regra 3

### A.1 — Localizar secção actual da Regra 3

A Regra 3 tem bloco sobre hierarquia de submódulos e, após o
Passo 96.3, um parágrafo sobre paths entre submódulos
(`super::X::func()`). Adicionar **novo parágrafo** no fim da
Regra 3.

### A.2 — Texto da nota

Adicionar ao fim da Regra 3:

```markdown
**Visibilidade preferida**: ao extrair código para submódulos,
a preferência é a seguinte ordem:

1. **Manter privado**. Se nenhum submódulo precisa de acesso
   directo, manter sem modificador de visibilidade. Acesso
   indirecto via métodos públicos já existentes.

2. **Métodos `pub(super)` em vez de campos `pub(super)`**. Se
   submódulos precisam de operar sobre uma struct, preferir
   expor **comportamento** (métodos) em vez de **estado**
   (campos). Métodos `pub(super) fn advance(&mut self)` são
   preferíveis a campo `pub(super) pos: usize`. Isto preserva
   invariantes da struct.

3. **`pub(in path)` para escopo explícito**. Quando o escopo
   exacto é conhecido, declará-lo directamente:

   ```rust
   pub(in crate::rules::parse) fn helper(...) { ... }
   ```

   É equivalente a `pub(super)` em certos casos mas auto-documenta
   a intenção.

4. **`pub(super)` em campos apenas quando necessário**. Se
   métodos não resolvem (ex: campo que múltiplos submódulos
   precisam de ler **e** escrever, sem semântica que justifique
   método específico), usar `pub(super)`. Registar em comentário
   no código a razão.

5. **`pub(crate)` apenas quando consumido fora do módulo
   actual**. Se o item é consumido por outro módulo da crate
   (ex: `eval/closures.rs` precisa de função de `stdlib::calc`),
   `pub(crate)` é apropriado. Se só submódulos do mesmo módulo
   consomem, `pub(super)` ou `pub(in path)` são mais estritos.

6. **`pub` (público) apenas para API verdadeiramente exposta ao
   exterior**. Funções consumidas por outras crates do workspace
   ou pelo wiring.

**Anti-padrão**: `pub(super)` aplicado a todos os campos e
métodos de uma struct por conveniência (ex: bulk replace
durante reestruturação). Isto destrói invariantes e aumenta
superfície de refactor. Se uma reestruturação encontra-se
nesta situação, abrir DEBT dedicado a auditar e restringir.
```

### A.3 — Verificação da Tarefa A

```bash
grep -A 5 "Visibilidade preferida" 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md
wc -l 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md
```

Esperado: ADR cresce de 363 para aproximadamente 400–420 linhas.

---

## Tarefa B — Abrir DEBT-47

### B.1 — Localização

Inserir na Secção 1 (DEBTs em aberto), após DEBT-46 (última
entrada actual da Secção 1).

### B.2 — Texto proposto

```markdown
## DEBT-47 — Auditoria de visibilidade dos `pub(super)` aplicados nos Passos 96.1–96.5 — EM ABERTO (Passo 96.6)

Os Passos 96.1, 96.2, 96.4 e 96.5 reestruturaram quatro
ficheiros grandes (`eval.rs`, `parse.rs`, `stdlib.rs`) em
submódulos conforme ADR-0037. Durante a extracção, visibilidade
de fields e métodos foi elevada para `pub(super)` em muitos
casos — alguns por bulk replace Python (reportado no Passo
96.4 para o `Parser` struct), outros por replace manual.

A ADR-0037 Regra 3 (clarificada no Passo 96.6) estabelece
preferência por:
- Manter privado quando possível.
- Métodos `pub(super)` sobre campos `pub(super)`.
- `pub(in path)` para escopo explícito quando aplicável.

Este DEBT documenta a necessidade de auditoria retroactiva:
verificar cada `pub(super)` introduzido nos Passos 96.1–96.5 e
restringir onde possível sem perder funcionalidade.

### Escopo

Submódulos a auditar:

- `01_core/src/rules/eval/` (Passos 96.1 e 96.2):
  - `mod.rs`, `markup.rs`, `math.rs`, `modules.rs`, `rules.rs`,
    `closures.rs`, `control_flow.rs`, `bindings.rs`,
    `operators.rs`, `tests.rs`.
- `01_core/src/rules/parse/` (Passo 96.4):
  - `mod.rs`, `parser.rs`, `math.rs`, `markup.rs`, `code.rs`,
    `rules.rs`, `patterns.rs`.
- `01_core/src/rules/stdlib/` (Passo 96.5):
  - `mod.rs` e 9 submódulos.

### Critério de conclusão

- [ ] Inventário de todos os `pub(super)` nos submódulos
      listados (provavelmente via grep + análise).
- [ ] Classificação de cada ocorrência:
  - Necessária (submódulo realmente precisa).
  - Elevável (podia ser `pub(in path)` mais estrito).
  - Removível (submódulo não usa — restos de bulk replace).
  - Convertível a método (campo `pub(super)` que devia ser
    método).
- [ ] Aplicação das mudanças sem regressão:
  - Campos `pub(super)` convertíveis → métodos.
  - Campos/métodos elevados → restritos.
  - Residuais de bulk replace → removidos.
- [ ] `cargo test` preservado.
- [ ] `crystalline-lint` → zero violations.

### Dependências

Este DEBT **não é atacado** até DEBT-46 encerrar. Razão:
fazer auditoria de visibilidade em ficheiros que ainda estão
a ser reestruturados é sobreposição de trabalho. Depois do
DEBT-46 encerrar (Passos 96.7, 96.8, 96.9, 96.10), todos os
submódulos estarão na sua forma final e a auditoria é estável.

Os Passos 96.7 em diante **seguem** a Regra 3 actualizada
(não introduzem `pub(super)` desnecessário). O DEBT-47 cobre
apenas o trabalho retroactivo dos passos anteriores.

### Nota sobre escopo do Passo 96.5

O Passo 96.5 já aplicou visibilidade mais criteriosa que os
Passos 96.1–96.4 (o reporte não menciona bulk replace). Na
auditoria, pode revelar-se que o 96.5 não tem dívida
significativa de visibilidade. Nesse caso, o DEBT-47 é pago
mais rápido nos submódulos relevantes.

---
```

### B.3 — Verificação da Tarefa B

```bash
grep -n "^## DEBT-47" 00_nucleo/DEBT.md
grep -c "^## DEBT-" 00_nucleo/DEBT.md
```

---

## Tarefa C — Renumerar checkboxes do DEBT-46

### C.1 — Contexto

O DEBT-46 tinha 9 checkboxes após o Passo 96.3:
- 96.1, 96.2, 96.3 marcados `[x]`.
- 96.4, 96.5 entretanto marcados `[x]` (Passos concluídos).
- 96.6, 96.7, 96.8, 96.9 pendentes `[ ]`.

Com a inserção deste passo como 96.6, os checkboxes pendentes
precisam de renumeração.

### C.2 — Renumeração final

```markdown
### Critério de conclusão

- [x] `eval.rs` reestruturado em submódulos por domínio.
      Passo 96.1. **Concluído**.
- [x] Delegação completa dos armos longos do dispatcher
      `eval_expr`. Passo 96.2. **Concluído**.
- [x] ADR-0037 promovida de `PROPOSTO` para `EM VIGOR` com
      ajustes validados nos Passos 96.1–96.2. Passo 96.3.
      **Concluído**.
- [x] `parse.rs` reestruturado por tipo de nó. Passo 96.4.
      **Concluído**.
- [x] `stdlib.rs` reestruturado por área da stdlib. Passo 96.5.
      **Concluído**.
- [x] Nota de visibilidade na Regra 3 da ADR-0037 e abertura
      do DEBT-47. Passo 96.6. **Concluído neste passo.**
- [ ] `layout/mod.rs` reestruturado (orquestração, medição,
      emissão, sub-frames). Passo 96.7.
- [ ] `math/layout.rs` reestruturado ou marcado como
      excepção Regra 6. Passo 96.8.
- [ ] `lexer/mod.rs` reestruturado ou marcado como excepção
      Regra 6. Passo 96.9.
- [ ] Verificação final: nenhum ficheiro em `01_core/src/`
      acima de 800 linhas sem justificativa Regra 6
      documentada no topo. Passo 96.10.
```

### C.3 — Verificação da Tarefa C

- 10 checkboxes no total.
- 6 primeiros marcados `[x]`.
- 4 últimos pendentes `[ ]`, com referências a 96.7, 96.8,
  96.9, 96.10.

---

## Critérios de conclusão

- [ ] Nota "Visibilidade preferida" adicionada ao fim da Regra 3
      da ADR-0037, com 6 pontos de preferência e anti-padrão.
- [ ] Tamanho da ADR-0037 entre 390 e 430 linhas.
- [ ] DEBT-47 adicionado em `DEBT.md` Secção 1, após DEBT-46.
- [ ] DEBT-46 com 10 checkboxes renumerados (6 `[x]`, 4 `[ ]`).
- [ ] Nenhum outro ADR alterado.
- [ ] Nenhum ficheiro de código alterado.
- [ ] `cargo test` passa com os mesmos 748 L1 + 174 L3 + 6
      ignorados. `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

Tarefa A:
- Posição exacta onde a nota foi inserida na Regra 3.
- Tamanho final da ADR-0037.

Tarefa B:
- Linhas exactas onde DEBT-47 começa e termina.
- Contagem total de DEBTs na Secção 1 após o passo.

Tarefa C:
- Confirmação de que DEBT-46 tem 10 checkboxes.
- Confirmação de que os 6 primeiros estão `[x]` e os 4 últimos
  estão `[ ]` com referências actualizadas.

Verificação:
- Contagem de testes inalterada.
- Zero violations.

Go/No-Go para Passo 96.7:
- **Go incondicional** — nota activa, DEBT-47 aberto para
  auditoria futura. Passo 96.7 aplica ADR-0037 a `layout/mod.rs`
  (2848 linhas) com a nota de visibilidade já a ser seguida.
  Reportes devem justificar escolhas de visibilidade (não bulk
  replace) quando for necessário elevar visibilidade.
- **No-Go** se algum dos parágrafos adicionados à ADR criar
  contradição com os 4 ajustes anteriores. Reportar a
  contradição antes de prosseguir para 96.7.
