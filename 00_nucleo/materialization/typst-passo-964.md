# Passo 964 — variantes gregas (`ϵ`/`ϑ`/`ϱ`/`ϕ`) podem estar usando codepoint diferente do vanilla

**Precede este passo**: achado novo, não quantificado com rigor ainda — auditoria externa, terceira
rodada (2026-08-04), seção 2 — `ϵ`/`ϑ`/`ϱ`/`ϕ` no cristalino usam o bloco Grego (U+03xx), mas a
auditoria suspeita que o vanilla usa uma família de codepoint diferente para o mesmo glifo
visualmente. **Não confirmado ainda** — a própria auditoria disse explicitamente que precisa
confirmar os codepoints exatos do lado vanilla antes de registar como divergência real.

**Pré-condição de árvore**: `git status`. Confirmar P962/963 (se já executados) presentes.

---

## Fase A — confirmar os codepoints dos dois lados, sem presumir

1. Extrair, via `pikepdf`/`mutool trace`, os codepoints reais (ToUnicode) usados pelo vanilla para
   `ϵ` (epsilon variante), `ϑ` (theta variante), `ϱ` (rho variante), `ϕ` (phi variante) na seção 2
   do documento de 30 seções.
2. Confirmar os codepoints que o cristalino usa para os mesmos quatro símbolos (`symbols.rs`).
3. Comparar — se forem o mesmo bloco/codepoint, não há divergência real (a auditoria estava a ver
   uma diferença de renderização de fonte, não de codepoint) — fechar sem código, mesmo padrão de
   P960.
4. Se forem codepoints diferentes: confirmar contra o codex (mesma fonte de verdade já usada em
   P958) qual é o codepoint canónico esperado para cada símbolo, e se o vanilla ou o cristalino é
   quem diverge do padrão — não presumir que "diferente do vanilla" significa "cristalino errado"
   sem essa confirmação (mesmo cuidado que P946 já teve com `partial`/`dot` — variantes podem ter
   mais de uma forma válida dependendo do contexto).

## Fase B — Implementação (só se a Fase A confirmar divergência real e a direção da correção)

TDD directo, mapeamento de tabela — mesma classe de P895/902/958.

## Resultado esperado

- Codepoints dos dois lados confirmados com número/evidência, não suposição.
- Se não houver divergência real: passo fecha na Fase A, sem código (mesmo padrão de P960).
- Se houver: corrigido, com confirmação contra o codex de qual lado estava certo.
