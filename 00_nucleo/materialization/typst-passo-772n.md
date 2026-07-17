---
# P772n — L0 + Implementação: `cannot_mutate_constant` (mutação silenciosa de nomes da stdlib)

> **Passo:** 772n
> **Data:** 2026-07-16
> **Foco:** P772l confirmou que `#{ calc = 5 }` reatribui `calc` silenciosamente (exit 0, sem erro), corrompendo o uso subsequente de `calc.pi`/`calc.max`/etc. pelo resto do âmbito, sem qualquer diagnóstico. O vanilla dá `error: cannot mutate a constant: calc`. Causa: `Binding` no cristalino (`01_core/src/entities/scope.rs`) só tem `value` — falta o campo que distingue bindings imutáveis (stdlib, seedados uma vez) de bindings normais. P772l também identificou que a mesma mudança de schema resolveria, de graça, §2.2 (mensagem errada ao mutar variável capturada — "unknown variable" em vez de "read-only") e §2.6 (avisos de depreciação de símbolos) — decidir explicitamente se entram no âmbito deste passo ou ficam para depois, não por default.
> **Tipo:** L0 + Implementação (schema change em `Binding`).
> **Tamanho:** M/L — mudança de schema num tipo central (`Binding`), usado em todo o avaliador.
> **ADR-0108 EM VIGOR.** **Regra de ouro do CLAUDE.md** — L0 antes de código, dado ser mudança de schema, não patch pontual.
> **Dependências:** P772l (achado, análise da causa raiz, commit `61b7edee78fdae9b020e458f5989f638cbf04096`).

---

## Sonda — confirmar o mecanismo exacto do vanilla

```bash
grep -n "cannot_mutate_constant\|BindingKind\|fn bind\b" lab/typst-original/crates/typst-library/src/foundations/scope.rs 2>/dev/null
```

Confirmar:
1. Como o vanilla marca um binding como constante no momento do seeding da stdlib (`Library::global()` ou equivalente) — presumivelmente `Scope::bind` com um `BindingKind::Const` ou similar.
2. A mensagem de erro exacta e se há variação por contexto (a mesma família que P772l encontrou para `Capturer::Function` vs `Capturer::Context` — confirmar se `cannot_mutate_constant` também varia, ou é sempre a mesma mensagem independentemente de onde a mutação é tentada).
3. Se bindings de utilizador podem alguma vez ser marcados como constantes (ex: `#let` normal nunca é const, só a stdlib), para não introduzir uma noção mais ampla do que o vanilla tem.

```bash
cat > /tmp/p772n-mutate-test.typ <<'EOF'
#{ calc = 5 }
EOF
lab/typst-original/target/release/typst compile /tmp/p772n-mutate-test.typ 2>&1
```

Confirmar a mensagem exacta e o exit code, palavra por palavra.

---

## Decisão de âmbito (registar antes de implementar)

| Achado | Incluir neste passo? |
|---|---|
| `cannot_mutate_constant` (prioridade escolhida) | Sim |
| §2.2 — mensagem correcta para mutação de variável capturada (`Capturer`) | Decidir: mesma mudança de schema, custo incremental baixo se incluído agora |
| §2.6 — avisos de depreciação (`Deprecation`) | Decidir: idem, mas é warning (severidade menor), pode ficar para depois sem risco |

Registar a decisão explicitamente no relatório, não implementar por default além do item prioritário sem essa decisão.

---

## Implementação

### 1. L0

Escrever/actualizar `00_nucleo/prompts/entities/scope.md` e `00_nucleo/prompts/rules/scopes.md` com o novo campo em `Binding` (nome e tipo a confirmar pela sonda — `kind: BindingKind` com pelo menos `Normal`/`Const`, expansível para `Capturer`/`Deprecation` se decidido incluir).

### 2. Schema

Em `01_core/src/entities/scope.rs`: adicionar o campo a `Binding`. Confirmar todos os pontos de construção de `Binding` no código (não só os óbvios) para decidir o valor por defeito (`Normal`) e onde marcar `Const` (seeding da stdlib, `Library::global()`/equivalente).

### 3. Verificação na mutação

Em `01_core/src/rules/eval/bindings.rs` (ponto de `access()`/atribuição já identificado por P772l): antes de mutar, verificar `BindingKind`; se `Const`, emitir o erro exacto confirmado pela sonda, com o span correcto.

### 4. Se decidido incluir §2.2/§2.6

Implementar na mesma mudança de schema, com testes próprios para cada mensagem.

---

## Validação

```bash
cat > /tmp/p772n-test1.typ <<'EOF'
#{ calc = 5 }
EOF
./target/release/typst compile /tmp/p772n-test1.typ 2>&1
```

Confirmar mensagem idêntica ao vanilla, exit code 1.

```bash
# Confirmar que bindings normais de utilizador continuam mutáveis
cat > /tmp/p772n-test2.typ <<'EOF'
#let x = 1
#{ x = 2 }
#x
EOF
./target/release/typst compile /tmp/p772n-test2.typ 2>&1
```

Deve continuar a funcionar sem erro (`x` não é constante).

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar que nenhum teste existente dependia do comportamento antigo (mutação silenciosa de stdlib) — se algum teste quebrar, avaliar se é regressão real ou correcção de um teste que validava o bug.

---

## Critério de fecho do passo

- [ ] Mensagem de erro do vanilla confirmada palavra por palavra, incluindo variações por contexto.
- [ ] Decisão de âmbito registada (§2.2/§2.6 incluídos ou não).
- [ ] L0 escrito antes do código.
- [ ] `Binding` com novo campo; stdlib seedada como constante.
- [ ] `#{ calc = 5 }` (e `image`, `table`, etc.) dá erro idêntico ao vanilla.
- [ ] Bindings normais de utilizador continuam mutáveis, sem regressão.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772n.md`.

---

## Próximo passo

Se §2.2/§2.6 não incluídos aqui: considerar passo dedicado curto para fechá-los, dado a infra-estrutura de schema já estar pronta depois deste passo.
Retomar a lista de prioridades: imagem com formato desconhecido omitida silenciosamente (P772k/P650), colapso de espaço em fontes variáveis (P772m), ou reconfirmação da varredura (P772e-style).
