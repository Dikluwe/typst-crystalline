---
# P655 — Terceira ronda: usar os avisos do próprio compilador, não padrões de texto

> **Passo:** 655
> **Data:** 2026-07-09
> **Foco:** P633 e P650 procuraram por padrões de texto já conhecidos — seis mais seis. Esse método só encontra o que alguém já pensou em procurar. Este passo usa uma abordagem diferente e complementar: os avisos que o próprio compilador Rust e o `clippy` já produzem, que dependem da análise real dos tipos, não de correspondência textual. `Result` é `#[must_use]` por definição na linguagem — qualquer sítio onde um valor desse tipo é criado e nunca tratado gera aviso automático, sem ninguém ter de adivinhar o padrão certo.
> **Tipo:** Sonda directa, ampla. Sem implementação neste passo.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633, P650 (rondas anteriores, método diferente).

---

## Sonda

### Parte 1 — Avisos nativos do `cargo build`

```bash
cargo clean -p typst-core -p typst-infra -p typst-shell -p typst-wiring 2>/dev/null
cargo build --workspace 2>&1 | tee /tmp/p655-build-warnings.txt
grep -c "^warning" /tmp/p655-build-warnings.txt
```

Confirmar quantos avisos existem hoje. Se o número for zero, confirmar se algum aviso foi silenciado globalmente (`#[allow(...)]` a nível de crate ou módulo, em vez de local) — o que esconderia avisos reais atrás de uma supressão ampla demais.

```bash
grep -rn "#!\[allow(" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs"
grep -rn "#\[allow(" 01_core/src/ 03_infra/src/ 02_shell/src/ 04_wiring/src/ --include="*.rs" | grep -v "test"
```

Para cada `#[allow(...)]` encontrado fora de testes, confirmar a razão — se for para silenciar um aviso genuinamente inofensivo (documentado), ou se está a esconder algo que devia ser corrigido, não silenciado.

### Parte 2 — `clippy`, com os grupos mais relevantes para falhas silenciosas

```bash
cargo clippy --workspace --all-targets -- \
  -W clippy::unwrap_used \
  -W clippy::expect_used \
  -W clippy::result_unwrap_used \
  -W clippy::option_unwrap_used \
  -W clippy::let_underscore_must_use \
  -W clippy::let_underscore_future \
  -W clippy::unused_result_ok \
  2>&1 | tee /tmp/p655-clippy-warnings.txt
```

Estes lints do `clippy` são especificamente desenhados para apanhar valores descartados ou tratados de forma pouco cuidadosa — uma segunda fonte de avisos automáticos, complementar aos nativos do compilador.

### Parte 3 — Classificar os resultados

Para cada aviso encontrado nas Partes 1 e 2, classificar:

1. **Já coberto** — o mesmo caso já foi encontrado e tratado por P633-P654 (confirmar por `file:line`, não assumir).
2. **Novo, inofensivo** — razão técnica documentada.
3. **Novo, suspeito** — precisa de teste directo.
4. **Novo, confirmado** — falha real, com teste a provar.

### Critério de fecho da sonda

- [ ] `cargo build --workspace` corrido do zero (sem cache), avisos contados.
- [ ] Todos os `#[allow(...)]` fora de testes revistos, com razão confirmada para cada um.
- [ ] `clippy` corrido com os lints listados, avisos contados.
- [ ] Cada aviso classificado nas quatro categorias.
- [ ] Casos "suspeito" testados directamente.
- [ ] Lista final de "confirmado", se houver, priorizada da mesma forma que P633/P650.

---

## Decisão

Mesma estrutura das rondas anteriores: este passo não corrige nada por si. Produz a lista. Cada "confirmado" vira o seu próprio passo.

Se esta ronda não encontrar nada de novo (tudo já coberto ou inofensivo): isso é, em si, um resultado válido — aumenta a confiança de que as duas rondas anteriores, mais este método diferente, cobrem o essencial. Registar isso com os números, não como "não vale a pena ter feito".

---

## Critério de fecho do passo

- [ ] `cargo build` limpo corrido, avisos contados e revistos.
- [ ] `#[allow(...)]` fora de testes todos revistos com razão.
- [ ] `clippy` corrido com os lints relevantes, avisos contados e revistos.
- [ ] Lista final classificada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p655.md`, com a lista completa e os números de avisos brutos, não só o resumo.
