# Passo 392 — Materialização: `panic(msg)` (XS)

**Tipo**: Materialização (L1 — stdlib helper puro; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 cumprida); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `panic`, XS, zero deps.
**Passo anterior**: P391 (`lorem`) — terceiro XS da fila limpa.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 confirmou `panic` como dívida genuína acidental (balde D), XS, zero dependências. O vanilla expõe `panic(msg)` — aborta a avaliação com uma mensagem de erro. É helper puro: entrada `Str`, efeito `abort eval`, sem I/O, sem layout, sem tipo novo.

Este passo fecha o **bloco de três XS** (`square` → `lorem` → `panic`) da fila limpa. Depois dele, a fila sobe para S/M (`#show regex`, `eval`, Model).

---

## 2. Decisão de engenharia

`panic` é **abort de avaliação com mensagem**. No vanilla:

```typst
#panic("something went wrong")  // aborta eval; mensagem no diagnóstico
```

A paridade (ADR-0107) é semântica: dada uma mensagem, aborta a avaliação e reporta a mensagem. No cristalino:

- `native_panic` recebe `Str` (msg), constrói `SourceDiagnostic` com a mensagem, e aborta via mecanismo de erro existente (reusa `bail!` ou equivalente do eval).
- Zero tipo novo; zero I/O; zero layout.
- A mensagem é **user-facing** — considerar se passa pelo catálogo i18n futuro (não neste passo; o catálogo ainda não existe).

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `panic.md`

Novo em `00_nucleo/prompts/engine/stdlib/panic.md` (ou integrado no prompt existente do domínio, seguindo o padrão do P391 — confirmar se `text.md` ou outro já cobre):

- **Paridade**: `panic(msg)` aborta eval com mensagem `msg`.
- **Substrato**: helper stdlib puro; recebe `Str`, constrói erro de eval.
- **Sem tipo novo**: reutiliza mecanismo de erro existente (`SourceDiagnostic`, `bail!`).
- **Parâmetros**: `msg` (obrigatório, `Str`).
- **Implementação**: chamar o mecanismo de abort de eval com a mensagem. Reusar o mesmo caminho que `assert` (se existir) ou o construtor de `SourceDiagnostic`.
- **Teste**: `panic("fail")` → aborta eval; mensagem "fail" no diagnóstico. Teste unitário captura o abort (não o panic do Rust — usar `should_panic` ou capturar o erro de eval).
- **Nota**: mensagem user-facing; quando o catálogo i18n existir, esta string será chave de catálogo. Por agora, literal hardcoded (H) aceitável.

### A.2 — CHECKPOINT

Parar. Apresentar `panic.md` (ou secção integrada) ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Implementar `native_panic`** em módulo stdlib apropriado (ex.: `foundations.rs` ou `text.rs`, confirmar padrão do repo):
   ```rust
   // pseudo: native_panic(msg: EcoString) -> !
   // constrói SourceDiagnostic com msg e aborta eval
   // reusa o mesmo caminho que assert_eq ou outros aborts de eval
   ```
2. **Registar em `make_stdlib`** (ou equivalente).
3. **Testes**:
   - `panic("fail")` → eval aborta; mensagem "fail" presente.
   - `panic("")` → eval aborta; mensagem vazia.
   - Erro: tipo errado de argumento (não `Str`).
4. **Linhagem**: `@prompt` aponta para `panic.md` (ou secção integrada); `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace` — verde (ou `cargo test -p typst-core panic`).
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas `native_panic` + registro + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar tipo `Value` ou `Content` novo — reutiliza mecanismo de erro existente.
- **Não** tocar em layout/render — é abort de eval.
- **Não** centralizar a mensagem no catálogo i18n — o catálogo ainda não existe (eixo 2 do P386 adiado).
- **Não** abrir reservas.
- **Não** tocar em outro ausente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `panic("fail")` aborta eval; mensagem "fail" no diagnóstico.
2. Zero tipo novo; zero variant novo; zero I/O.
3. Testes verdes; lint zero; hashes propagados.
4. Inventário 148: `panic` transita `ausente` → `implementado`.
5. L0 salvo e hashado antes do código (protocolo de nucleação).

---

## 7. O que pode sair errado

- **Mecanismo de abort de eval não existe ou é diferente.** Mitigação: reusa o mesmo caminho que `assert` (se existir) ou o construtor de `SourceDiagnostic`. Se o mecanismo for `Result`-based, `native_panic` retorna `Err` que sobe até o topo do eval.
- **Paridade medida por tipo de erro exato.** Mitigação: ADR-0107 — a paridade é "aborta com mensagem", não o tipo Rust exato do erro.
- **Tentação de já fazer `#show regex` junto.** Mitigação: um passo de cada vez; ritmo não é desculpa para violar escopo.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `panic` como XS, zero deps.
- ADR-0107 — paridade semântica (abort com mensagem, não tipo de erro exato).
- ADR-0033 — paridade vanilla.
- P390 (`square`), P391 (`lorem`) — precedentes de XS na fila limpa.

---

## 9. Nota sobre o Tekt

Três XS seguidos (`square`, `lorem`, `panic`) fecham o bloco de momentum. O próximo passo sobe para S (`#show regex`) ou M (`eval` / Model). O tempo de ciclo dos três XS é a baseline de saúde do pipeline no Kimi Code. Se estiver estável, a ferramenta está calibrada. Se cresceu, investigar antes de subir de escopo.
