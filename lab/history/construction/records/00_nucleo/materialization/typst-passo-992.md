# Passo 992 — `attach()`/`limits()`/`scripts()` não implementadas: argumentos nomeados desaparecem sem virar texto nem posição

**Precede este passo**: achado externo (2026-08-07, seção 32 do documento estendido) — três
funções nativas de posicionamento explícito do Typst não são reconhecidas pelo cristalino:

| construção | cristalino (hoje) | vanilla |
|---|---|---|
| `attach(A, t:α, b:β, tl:n, tr:m, bl:p, br:q)` | texto literal `"attach(A)"`, os 6 argumentos nomeados descartados silenciosamente | "A" com os 6 cantos posicionados |
| `attach(sum, t:n, b:(k=1))` | texto literal `"attach(∑)"`, limites descartados | ∑ com limites no formato padrão |
| `limits(A)^alpha_beta` | texto literal `"limits(A)"`; o `^_` externo ainda tenta anexar ao texto literal | "A" com α empilhado acima, β abaixo |
| `scripts(A)^alpha_beta` | texto literal `"scripts(A)"`, mesmo padrão | "A" com α/β ao lado, não empilhados |

**Diferença crítica em relação ao bug de `lr(` (P981)**: lá, o texto vazava mas a geometria por
baixo ficava certa. Aqui, os **argumentos nomeados somem por completo** — nem viram texto visível,
nem produzem posicionamento nenhum. É perda de informação, não só vazamento.

**Pré-condição de árvore**: `git status`. Confirmar P991 presente.

---

## Fase A — confirmar quanto do mecanismo já existe

1. Confirmar se o mecanismo interno de `attach.rs` já suporta os 6 cantos (`t`/`b`/`tl`/`tr`/`bl`/
   `br`) internamente, mesmo que só acessível hoje via `^`/`_`/subscrito-sobrescrito simples — já
   há referências nesta frente (P944 em diante) a `Content::MathAttach` com um campo `limits` —
   confirmar se essa flag já existe e já é respeitada pelo layout, faltando só o dispatch de eval
   para `limits()`/`scripts()` a activarem (mesma classe de bug de P958/962/981: mecanismo pronto,
   função não cadastrada).
2. Se o mecanismo de 6 cantos **não** existir ainda internamente (só topo/base via `attach`
   implícito de sub/sobrescrito, sem os 4 cantos diagonais `tl`/`tr`/`bl`/`br`): isto é escopo
   maior — confirmar antes de decidir se cabe num só passo ou se `attach()` (6 cantos) precisa de
   passo próprio, separado de `limits()`/`scripts()` (mais simples, só uma flag).
3. Ler o mecanismo real do vanilla para os três (`typst-library/src/math/`, funções `attach`,
   `limits`, `scripts`) — confirmar a semântica exacta de cada uma antes de implementar.

## Fase A.1 — gate se necessário

Se `attach()` (6 cantos) exigir estrutura nova em `Content::MathAttach` (campos para os 4 cantos
diagonais, hoje talvez só topo/base): editar L0s, sincronizar hashes, **parar para confirmação do
dono antes da Fase B** — mudança de contrato, per `ADR-0127`. Se `limits()`/`scripts()` forem só
dispatch de uma flag já suportada: fluxo contínuo.

## Fase B — Implementação (protocolo de dois agentes de P898 se `attach()` exigir estrutura nova;
TDD directo para `limits()`/`scripts()` se forem só dispatch)

1. Testes cobrindo os quatro casos da tabela do achado, mais casos de guarda (uso normal de
   sub/sobrescrito sem estas funções continua a funcionar).
2. Implementar (podem ser sub-partes: `limits`/`scripts` primeiro se forem mais simples,
   `attach()` de 6 cantos depois, ou juntos se o desenho permitir).
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recompilar os quatro casos do achado — confirmar ausência de texto literal e posicionamento
   correto (topo/base para `attach`/`limits`; lateral para `scripts`; os 4 cantos diagonais para
   `attach()` completo).
2. Confirmação visual a alta resolução, comparando com o vanilla.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- `attach()`, `limits()`, `scripts()` implementadas, sem vazamento de texto nem perda de
  argumentos.
- Se `attach()` de 6 cantos precisar de mais escopo que `limits()`/`scripts()`: separado
  explicitamente, registado, não forçado no mesmo passo se maior que o esperado.
- Benchmark sem regressão.
