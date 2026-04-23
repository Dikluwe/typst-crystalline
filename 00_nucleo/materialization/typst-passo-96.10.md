# Passo 96.10 — Verificação final e encerramento do DEBT-46

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR`.
- `00_nucleo/DEBT.md` — DEBT-46 com 9 checkboxes marcados
  `[x]` (Passos 96.1 a 96.9). Último checkbox (96.10 —
  verificação final) pendente.

Pré-condição: `cargo test` — 764 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.9 concluído (lexer reestruturado).

---

## Natureza deste passo

Passo único de governança. Verifica empiricamente que o
DEBT-46 está resolvido e encerra.

Três tarefas:

1. **Tarefa A** — Inventário final: listar todos os ficheiros
   em `01_core/src/` ainda acima de 800 linhas e confirmar que
   cada um tem justificativa Regra 6 documentada no topo, ou é
   excepção natural aceite (ex: `tests.rs`).

2. **Tarefa B** — Consolidar resultados do DEBT-46: marcar
   último checkbox, adicionar secção de resultados finais com
   métricas totais (antes vs depois).

3. **Tarefa C** — Mover DEBT-46 da Secção 1 (em aberto) para
   Secção 2 (encerrado).

**Não altera código**. Só `DEBT.md`.

---

## Decisões formalizadas neste passo

- DEBT-46 encerrado.
- Métricas finais registadas como evidência do trabalho.

---

## Tarefa A — Inventário final

### A.1 — Listar ficheiros acima de 800 linhas

```bash
# Todos os ficheiros .rs em 01_core/src/ ordenados por tamanho:
find 01_core/src -name "*.rs" -exec wc -l {} + | sort -rn | head -25

# Apenas os acima de 800:
find 01_core/src -name "*.rs" -exec wc -l {} + | awk '$1 > 800'
```

### A.2 — Verificar justificativas Regra 6

Para cada ficheiro acima de 800 linhas, verificar se tem
comentário de excepção Regra 6 no topo:

```bash
# Para cada ficheiro identificado:
head -10 <caminho_do_ficheiro>
```

Esperado encontrar coisas como:
- Ficheiros `tests.rs` grandes (ex: `eval/tests.rs`, `layout/tests.rs`,
  `math/layout/tests.rs`) — excepção natural da Regra 5 +
  Regra 6 combinadas. `#[cfg(test)]` gated.
- `parse/parser.rs` (700 linhas) — abaixo de 800, não é
  excepção.
- Possivelmente `entities/content.rs`, `entities/ast/expr.rs`
  — enums fundamentais com muitas variantes (Regra 6).

### A.3 — Classificar cada ficheiro acima de 800

Para cada ficheiro identificado em A.1, reportar:

| Ficheiro | Linhas | Tipo | Justificativa |
|----------|--------|------|---------------|
| eval/tests.rs | 2100 | Testes E2E | Regra 5 + Regra 6 |
| layout/tests.rs | 1399 | Testes E2E | Regra 5 + Regra 6 |
| math/layout/tests.rs | 761 | Testes E2E | Abaixo de 800 - não é excepção |
| entities/content.rs | X | Enum fundamental | Regra 6 a verificar |
| ... | ... | ... | ... |

**Se encontrar ficheiro acima de 800 SEM justificativa Regra 6
no topo**: reportar. Duas opções:
- Adicionar justificativa (se o ficheiro é legitimamente denso).
- Programar reestruturação futura (novo DEBT).

Neste passo, não fazemos reestruturação nova. Só identificamos
e decidimos.

---

## Tarefa B — Actualizar DEBT-46 com resultados

### B.1 — Marcar último checkbox

```markdown
- [x] Verificação final: nenhum ficheiro em `01_core/src/`
      acima de 800 linhas sem justificativa Regra 6
      documentada no topo. Passo 96.10. **Concluído neste
      passo.**
```

### B.2 — Adicionar secção "Resultados finais"

Ao fim da entrada DEBT-46 (antes do separador `---`), adicionar:

```markdown
### Resultados finais (após Passo 96.10)

Trabalho completo da ADR-0037 Regra 2 aplicada a ficheiros
grandes de `01_core/src/rules/`.

**Antes** (inventário do Passo 96):

| Ficheiro | Linhas |
|----------|--------|
| eval.rs | 3780 |
| layout/mod.rs | 2848 |
| parse.rs | 2255 |
| math/layout.rs | 1806 |
| stdlib.rs | 1711 |
| lexer/mod.rs | 1250 |

Total: 13.650 linhas em 6 ficheiros acima de 1000.

**Depois** (após Passos 96.1–96.9):

| Estrutura original | Resultado |
|--------------------|-----------|
| eval.rs | eval/mod.rs (520) + 7 submódulos + tests (2100) |
| layout/mod.rs | layout/mod.rs (756) + 7 submódulos + tests (1399) |
| parse.rs | parse/mod.rs (156) + 6 submódulos |
| math/layout.rs | math/layout/mod.rs (484) + 9 submódulos + tests (761) |
| stdlib.rs | stdlib/mod.rs (617) + 9 submódulos |
| lexer/mod.rs | lexer/mod.rs (468) + 3 submódulos |

**Ficheiros ainda acima de 800 linhas em `01_core/src/`**:

- [listar da Tarefa A.3]
- Cada um com justificativa Regra 6 no topo ou sendo excepção
  de testes cross-cutting (Regra 5).

**Contagem de testes**:
- Antes (Passo 96): 746 L1.
- Depois (Passo 96.10): 764 L1 (+18 smoke tests V2 obrigatórios
  dos submódulos novos).

**ADR-0037 validada empiricamente**. A Regra 2 (limite
orientativo de 800 linhas) e a Regra 6 (excepções documentadas)
funcionaram como esperado em 7 aplicações consecutivas.

**Dívida residual registada**:
- DEBT-47 (auditoria de visibilidade dos `pub(super)`
  introduzidos) aberto no Passo 96.6.
- Smoke tests V2 (+18 no total) — obrigatórios pelo linter,
  não trazem valor funcional. Podem ser revistos no futuro
  se a política do linter mudar.
```

Ajustar os números conforme realidade da Tarefa A.3.

---

## Tarefa C — Mover DEBT-46 para Secção 2

### C.1 — Mover a entrada

Cortar toda a entrada DEBT-46 (do cabeçalho `## DEBT-46` até
ao separador `---` final) e colar na Secção 2 (DEBTs
encerrados), após a última entrada encerrada.

Adicionar marca de encerramento no cabeçalho:

```markdown
## DEBT-46 — Ficheiros de L1 com coesão baixa por tamanho excessivo — ENCERRADO (Passo 96.10) ✓
```

### C.2 — Verificação

```bash
# DEBT-46 não deve estar na Secção 1:
grep -n "^## DEBT-46" 00_nucleo/DEBT.md

# Deve aparecer na Secção 2:
# (inspecção visual — deve estar após o "## Secção 2" header)
```

Contagem de DEBTs em aberto diminui em 1. Contagem de DEBTs
encerrados aumenta em 1.

---

## Critérios de conclusão

- [ ] Tabela de ficheiros acima de 800 linhas reportada,
      todos com justificativa.
- [ ] DEBT-46 com secção "Resultados finais" completa.
- [ ] Último checkbox do DEBT-46 marcado `[x]`.
- [ ] DEBT-46 movido da Secção 1 para Secção 2 com marca
      `ENCERRADO (Passo 96.10) ✓`.
- [ ] Nenhum ficheiro de código alterado.
- [ ] Nenhum ADR alterado.
- [ ] `cargo test` preservado (764 L1 + 174 L3 + 6 ignorados).
- [ ] `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

Tarefa A:
- Tabela de ficheiros acima de 800 linhas com classificação.
- Se algum ficheiro não tinha justificativa Regra 6 e precisou
  de ser adicionada ou registada como trabalho futuro.

Tarefa B:
- Confirmação da secção "Resultados finais" com números
  exactos.

Tarefa C:
- Confirmação DEBT-46 em Secção 2.
- Contagem final de DEBTs por secção.

Observações finais sobre a série 96.x:
- Lições aprendidas (se houver insights novos não registados
  antes).
- Sugestões para futura revisão da ADR-0037 (se alguma regra
  se revelou ambígua).

Go/No-Go para próximo trabalho:
- **Go** — DEBT-46 encerrado. Próximos trabalhos candidatos:
  - **DEBT-47** (auditoria de visibilidade) — trabalho natural
    após reestruturações. Começa agora que submódulos estão
    estáveis.
  - **Materialização de `Engine<'a>`** — agora que `EvalContext`
    está atomizado e `eval/` está reestruturado, materializar
    o agregador vanilla faz sentido.
  - **Materializar dependências folha** (`Style`, `LazyHash`,
    `Introspection`) — desbloqueia `Styles` e `Sink`.
  - **Continuar extracção do `EvalContext`** — `figure_numbering`
    e `current_file` restantes como candidatos da ADR-0036.

A decisão fica para conversa após o reporte deste passo.
