# Passo 393 — Materialização: `#show regex(...)` (S)

**Tipo**: Materialização (L1 — wiring de show-rules; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 cumprida); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0077 (`Selector::Regex` em L1), ADR-0017 (não aplica — tipo já existe).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `#show regex`, S, `Selector::Regex` pronto; falta wiring.
**Passo anterior**: P392 (`panic`) — fecha o bloco de três XS.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 confirmou `#show regex(...)` como dívida genuína acidental (balde D), S, com substrato claro: o tipo `Selector::Regex` já existe em L1 (`selector.rs:48`, P209D/ADR-0077), mas o **wiring** — a aplicação de show-rules quando o seletor é `Regex` — não está ligado.

Este passo sobe o escopo de XS para S: não é helper puro, é **wiring de pipeline** (show-rules). Mas continua zero tipo novo e zero I/O.

> **Nuance:** o `.where(field:)` (predicado por campo) é **fora deste passo** — precisa de `Selector::Where`, que ainda não existe. Este passo é só o wiring do `Regex` que já existe.

---

## 2. Decisão de engenharia

`#show regex("pattern"): it => body` aplica uma show-rule a elementos cujo conteúdo textual bate com o padrão regex. No vanilla:

```typst
#show regex("\d+"): it => strong(it)
// todo texto que contenha dígitos consecutivos fica strong
```

A paridade (ADR-0107) é semântica: dado um padrão regex, a show-rule aplica-se a elementos cujo conteúdo textual bate com o padrão.

No cristalino:

- `Selector::Regex` já existe em L1 (`selector.rs:48`) — tipo pronto.
- O wiring falta em `apply_show_rules` (ou equivalente) — o loop/match que decide se uma show-rule se aplica a um elemento.
- A lógica: quando o seletor é `Selector::Regex(pattern)`, testar se o conteúdo textual do elemento bate com o padrão (via regex engine existente em L1 ou crate autorizado).
- Zero tipo novo; zero I/O.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `#show-regex.md`

Novo em `00_nucleo/prompts/rules/show-regex.md` (ou integrado no prompt de show-rules existente, seguindo padrão do P391):

- **Paridade**: `#show regex(pattern): it => body` aplica show-rule a elementos cujo conteúdo textual bate com `pattern`.
- **Substrato**: wiring em `apply_show_rules` (ou equivalente); reusa `Selector::Regex` existente.
- **Sem tipo novo**: `Selector::Regex` já existe (P209D/ADR-0077).
- **Parâmetros**: `pattern` (obrigatório, `Str`, regex válido). Erro se regex inválido.
- **Implementação**: no loop de aplicação de show-rules, quando o seletor é `Regex(pattern)`, testar match do padrão contra o conteúdo textual do elemento. Se bater, aplicar a transformação.
- **Teste**: `#show regex("\d+"): it => strong(it)` → texto com dígitos fica strong; texto sem dígitos não muda. Erro para regex inválido.
- **Scope-out**: `.where(field:)` (predicado por campo) — precisa `Selector::Where`, ainda não existe.

### A.2 — CHECKPOINT

Parar. Apresentar `#show-regex.md` (ou secção integrada) ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Implementar wiring em `apply_show_rules`** (ou módulo de show-rules apropriado):
   ```rust
   // pseudo: no match de seletor contra elemento:
   // Selector::Regex(pattern) => testar se conteúdo textual do elemento bate com pattern
   // se sim, aplicar a show-rule
   ```
2. **Confirmar que `Selector::Regex` está acessível** em L1 (já existe; verificar import).
3. **Testes**:
   - `#show regex("\d+"): it => strong(it)` aplicado a texto com dígitos → strong.
   - Mesmo show-rule aplicado a texto sem dígitos → sem alteração.
   - Regex inválido → erro de eval (mensagem clara).
   - Múltiplos show-rules (regex + outro seletor) → ordem correta.
4. **Linhagem**: `@prompt` aponta para `#show-regex.md`; `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace` — verde (ou `cargo test -p typst-core show_regex`).
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas wiring de show-rules + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar `Selector::Where` — fora deste passo; é feature separada.
- **Não** criar tipo `Value` ou `Content` novo — reutiliza `Selector::Regex` existente.
- **Não** tocar em layout/render — o wiring é em eval/show-rules, antes do layout.
- **Não** abrir reservas.
- **Não** tocar em outro ausente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `#show regex("\d+"): it => strong(it)` aplica strong a texto com dígitos.
2. Zero tipo novo; zero variant novo; zero I/O.
3. Testes verdes; lint zero; hashes propagados.
4. Inventário 148: `#show regex` transita `ausente` → `implementado` (ou `parcial` se `.where()` ainda faltar; documentar).
5. L0 salvo e hashado antes do código (protocolo de nucleação).

---

## 7. O que pode sair errado

- **`Selector::Regex` não expõe o padrão ou a API de match.** Mitigação: verificar `selector.rs:48` e a estrutura de `Regex` em L1. Se necessário, adicionar método `matches(&self, text: &str) -> bool` ao tipo.
- **Wiring de show-rules é pipeline complexo.** Mitigação: reusa o mesmo padrão de match que outros seletores (`Elem`, `Label`, etc.) já usam. O regex é mais um braço no `match`.
- **Paridade medida por ordem de aplicação.** Mitigação: ADR-0107 — a paridade é "aplica a elementos que batem", não a ordem exata de resolução quando múltiplas show-rules competem.
- **Tentação de já fazer `.where()` junto.** Mitigação: um passo de cada vez; `Selector::Where` é tipo novo (ADR-0017), vira passo separado.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `#show regex` como S, `Selector::Regex` pronto.
- `selector.rs:48` (`Selector::Regex`) — tipo existente.
- ADR-0077 — `Selector::Regex` em L1.
- ADR-0107 — paridade semântica (aplica a elementos que batem, não ordem exata).
- ADR-0033 — paridade vanilla.
- P390–P392 — precedentes de XS na fila limpa.

---

## 9. Nota sobre o Tekt

Este passo é o **primeiro S após três XS**. O salto de escopo é controlado: zero tipo novo, zero I/O, só wiring. Se o tempo de ciclo crescer significativamente em relação aos XS, o problema é o pipeline de show-rules (complexidade de match/ordenação), não a ferramenta. Registar o tempo como ponto de inflexão: XS → S.
