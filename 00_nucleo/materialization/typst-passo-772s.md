---
# P772s — Span por argumento em chamadas de funções nativas

> **Passo:** 772s
> **Data:** 2026-07-17
> **Foco:** P772p identificou, como efeito colateral (não o objetivo original do passo), que `Args` (`01_core/src/entities/args.rs`) não transporta o span de cada argumento posicional — por isso todo erro de `native_image` (e, por extensão, de qualquer função nativa que valide argumentos após recebê-los) usa `Span::detached()` em vez de apontar para o argumento problemático no documento. Isto é maior que `image()`: afeta a precisão de qualquer mensagem de erro de validação de argumento em toda a stdlib nativa. Este passo mede o alcance real antes de decidir se é uma mudança de ABI justificável.
> **Tipo:** Sonda de alcance + Decisão registada (regra 1) + Implementação se o alcance justificar.
> **Tamanho:** M/L — mudança na ABI de chamada de funções nativas é estrutural, não pontual.
> **ADR-0108 EM VIGOR** — medir quantos erros reais perdem precisão de span antes de decidir corrigir.
> **Dependências:** P772p (achado original, commit `56bbaa7a4f2ef35ee0aa970f72bde8021223f552`), P772b/P772d (disciplina de span já estabelecida nessa linha de trabalho, para reutilizar o mesmo padrão de correção, não inventar um novo).

---

## Sonda — alcance real do problema

```bash
grep -rn "Span::detached()" 01_core/src/rules/stdlib/*.rs | wc -l
grep -rn "Span::detached()" 01_core/src/rules/stdlib/*.rs
```

Contar quantos pontos de erro em funções nativas usam `Span::detached()` hoje — isto é a medida directa do "alcance" antes de decidir se vale a mudança estrutural.

```bash
grep -n "struct Args\|pub fn.*positional\|pub fn.*named" 01_core/src/entities/args.rs
```

Confirmar a estrutura actual de `Args` e como ela é construída no caminho de chamada (`eval_call`/`apply_closure`/despacho para `native_*`) — onde o span de cada argumento existe no AST antes de "achatar" para `Args`, e onde essa informação se perde.

### Como o vanilla faz

```bash
grep -n "struct Args\|struct Arg\b" lab/typst-original/crates/typst-library/src/foundations/args.rs 2>/dev/null
```

Confirmar se o vanilla guarda span por argumento na própria struct `Arg`/`Args`, ou se resolve de outra forma (ex: reconstituindo a partir do `Span` da chamada inteira mais offset).

---

## Decisão (regra 1)

| Critério | Decisão |
|---|---|
| Se o número de pontos afetados (grep acima) for pequeno (<10) e concentrado | Corrigir directamente neste passo |
| Se for grande e disperso por toda a stdlib | Avaliar custo/benefício: mudança de ABI em `Args` para incluir span por argumento é justificável a longo prazo, mas pode exigir revisão de todos os call sites — decidir se faz agora ou fica registado como débito técnico priorizado |
| Se `Span::detached()` só aparecer em casos de erro raros/pouco usados | Pode não justificar a mudança estrutural agora — registar como conhecido, não urgente |

Registar a decisão com o número real, não estimativa.

---

## Implementação (se a decisão for corrigir)

1. Adicionar span por argumento posicional em `Args` (`01_core/src/entities/args.rs`), replicando a estrutura do vanilla confirmada pela sonda.
2. Confirmar todos os pontos de construção de `Args` no caminho de avaliação de chamada — propagar o span de cada argumento do AST, não perder a informação ao "achatar".
3. Atualizar as funções nativas identificadas pelo grep para usar o span do argumento em vez de `Span::detached()`.

---

## Validação

```bash
cat > /tmp/p772s-test.typ <<'EOF'
#image("bogus.png")
EOF
./target/release/typst compile /tmp/p772s-test.typ 2>&1
```

Confirmar que o erro agora aponta para `"bogus.png"` no documento, não `<detached>`.

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar que nenhum teste dependia da ausência de span (mensagens formatadas assumindo `<detached>`).

---

## Critério de fecho do passo

- [ ] Alcance real medido (número de pontos com `Span::detached()` em funções nativas).
- [ ] Mecanismo do vanilla confirmado.
- [ ] Decisão registada com base no número real, não estimativa.
- [ ] Se corrigido: span por argumento implementado, pontos afetados atualizados, erro de `image()` (caso original de P772p) já aponta para o span correto.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772s.md`.

---

## Próximo passo

Reconfirmar `lacuna-inventario` (estilo P772e, segunda rodada) — decidir se a varredura sistemática da stdlib continua ou encerra, com o volume já coberto até aqui (P765a a P772s).
