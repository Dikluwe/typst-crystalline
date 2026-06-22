# Passo 391 — Materialização: `lorem(n)` (XS)

**Tipo**: Materialização (L1 — stdlib helper puro; zero tipo novo; zero I/O; zero layout).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 cumprida); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `lorem`, XS, zero deps.
**Passo anterior**: P390 (`square`) — valida que a fila limpa flui.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 confirmou `lorem` como dívida genuína acidental (balde D), XS, zero dependências. O vanilla expõe `lorem(n)` — gera `n` palavras de texto dummy (Lorem Ipsum). É helper puro: entrada `Int`, saída `Str`, sem I/O, sem layout, sem tipo novo.

Este passo continua o **piso da fila limpa** após `square` (P390). XS, zero deps, zero graded, zero scope-out. Serve para manter momentum antes de subir para S/M.

---

## 2. Decisão de engenharia

`lorem` é **helper puro de texto**. No vanilla:

```typst
#lorem(5)  // "Lorem ipsum dolor sit amet."
#lorem(0)  // ""
#lorem(100) // 100 palavras de texto dummy
```

A paridade (ADR-0107) é semântica: dado `n`, retorna `n` palavras de texto dummy. A forma do texto (as palavras exatas) é **implementação** — o vanilla usa um gerador específico; o cristalino pode usar outro, desde que:
- O número de palavras bate com `n`.
- O texto é legível como dummy (não precisa ser byte-identical ao vanilla).
- `n = 0` retorna string vazia.

No cristalino:

- `native_lorem` recebe `Int` (n), retorna `Value::Str`.
- Implementação: gerador de texto dummy com vocabulário fixo (Lorem Ipsum clássico), repetindo/cortando até `n` palavras.
- Zero tipo novo; zero I/O; zero layout.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `lorem.md`

Novo em `00_nucleo/prompts/rules/stdlib/lorem.md` (confirmar path; seguir padrão de `square.md`):

- **Paridade**: `lorem(n)` devolve `Str` com `n` palavras de texto dummy.
- **Substrato**: helper stdlib puro; recebe `Int`, devolve `Value::Str`.
- **Sem tipo novo**: reutiliza `Value::Str`.
- **Parâmetros**: `n` (obrigatório, `Int` ≥ 0). Erro se negativo.
- **Implementação**: gerador de texto dummy com vocabulário Lorem Ipsum fixo; repete/corta até `n` palavras. Não precisa ser byte-identical ao vanilla.
- **Teste**: `lorem(5)` → 5 palavras; `lorem(0)` → ""; `lorem(100)` → 100 palavras; erro para `n < 0`.
- **Nota**: independente de shaping (DEBT-53). Texto dummy não exige OpenType features.

### A.2 — CHECKPOINT

Parar. Apresentar `lorem.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Implementar `native_lorem`** em módulo stdlib apropriado (ex.: `text.rs` ou `foundations.rs`, confirmar padrão do repo):
   ```rust
   // pseudo: native_lorem(n: i64) -> Value::Str
   // vocab: ["Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", ...]
   // repete vocab até ter >= n palavras; corta exatamente n; junta com " "
   ```
2. **Registar em `make_stdlib`** (ou equivalente).
3. **Testes**:
   - `lorem(5)` → string com 5 palavras.
   - `lorem(0)` → string vazia.
   - `lorem(100)` → string com 100 palavras.
   - `lorem(-1)` → erro (argumento inválido).
   - `lorem(1)` → uma palavra (sem espaço trailing).
4. **Linhagem**: `@prompt` aponta para `lorem.md`; `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace` — verde (ou `cargo test -p typst-core lorem`).
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas `native_lorem` + registro + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar tipo `Value` ou `Content` novo — reutiliza `Value::Str`.
- **Não** tocar em layout/render — é helper puro de string.
- **Não** exigir byte-identical ao vanilla — paridade é semântica (n palavras), não mecânica (texto exato).
- **Não** abrir reservas.
- **Não** tocar em outro ausente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `lorem(5)` devolve `Str` com 5 palavras; `lorem(0)` devolve `""`; `lorem(-1)` erro.
2. Zero tipo novo; zero variant novo; zero I/O.
3. Testes verdes; lint zero; hashes propagados.
4. Inventário 148: `lorem` transita `ausente` → `implementado`.
5. L0 salvo e hashado antes do código (protocolo de nucleação).

---

## 7. O que pode sair errado

- **Vocabulário Lorem Ipsum não disponível.** Mitigação: embeddar um vocab mínimo (20-30 palavras) no código; é dummy text, não precisa ser completo.
- **Paridade medida por texto exato.** Mitigação: ADR-0107 — a paridade é "n palavras de dummy text", não o mesmo texto byte-a-byte.
- **Tentação de já fazer `panic` junto.** Mitigação: um passo de cada vez; ritmo não é desculpa para violar escopo.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `lorem` como XS, zero deps.
- ADR-0107 — paridade semântica (n palavras, não texto exato).
- ADR-0033 — paridade vanilla.
- P390 (`square`) — precedente de XS na fila limpa.

---

## 9. Nota sobre o Tekt

Dois XS seguidos (`square`, `lorem`) validam o pipeline: L0 → hash → código → teste → lint → reclassificação do inventário. Se o tempo de ciclo estiver estável (< X minutos), a fila limpa flui. Se estiver crescendo, o problema é infraestrutura (prompt grosso, hash lento, lint pesado), não a fila. Registar o tempo como baseline.
