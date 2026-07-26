# Passo 918 — exocitose geométrica: extrair núcleo partilhado de `math/layout/*`

**Precede este passo**: `ADR-0123` (geometria tipográfica — "próximo passo natural: extrair núcleo
geométrico partilhado"); `typst-passo-909-relatorio.md` (padrão de atomização já aplicado,
prova por PDF byte-idêntico); os relatórios P901/905/906/912-917 (cada módulo já auditado
individualmente contra o vanilla — este passo não reaudita, consolida).

**Este é um passo de refactor — content-preserving (ADR-0107), mesmo padrão de P909.** Nenhuma
mudança de comportamento é esperada. Se qualquer teste mudar de resultado, parar e investigar — a
lógica foi alterada, não só movida.

**Pré-condição de árvore**: `git status`. Confirmar que P915/916/917 estão commitados (o dono
confirmou que sempre commita depois do agente terminar — mas confirmar de qualquer forma antes de
começar um refactor deste tamanho, que toca praticamente todo o subsistema math).

---

## Por que agora, não antes

Extrair um núcleo partilhado **antes** de cada módulo estar individualmente correto arriscaria
generalizar a partir de fórmulas erradas (exatamente o erro que `root.rs`/`frac.rs`/`underover.rs`
cometeram cada um por conta própria, per `ADR-0123`). Agora que os nove módulos já passaram por
correção e auditoria contra o vanilla real (P901, P905, P906, P911, P912, P913, P914, P915, P917),
extrair o comum é consolidação, não mais uma frente de descoberta de bugs.

## Fase A — inventariar o que é genuinamente comum (medir, não presumir)

Candidatos a extrair, um por um — confirmar por leitura directa de cada módulo, não por memória
dos relatórios anteriores (podem ter divergido em detalhe durante a implementação):

1. **Posicionamento relativo à baseline própria** (`y=0=baseline`, `ADR-0123`): usado em
   `frac.rs` (offsets de numerador/denominador), `root.rs` (radicando/índice), `underover.rs`
   (acento/over/under), `attach.rs` (sup/sub). Confirmar se a fórmula de posicionamento em si
   (não só a convenção) é idêntica nos quatro, ou se cada um tem uma variação legítima (por
   exemplo, `frac.rs` usa gap fixo, `attach.rs` usa `.max()` de vários termos — podem não ser a
   mesma função, só a mesma convenção de eixo).
2. **Empilhamento com gap de `MathConstants`**: `frac.rs` (`fraction_rule_thickness`/gaps),
   `underover.rs` (P906 registou "sem constante de gap explícita" como achado em aberto — este
   passo é candidato natural para resolver isso ao mesmo tempo que extrai o padrão partilhado).
3. **`covering()`/resolução de fonte MATH por carácter**: já centralizado em P912 (raiz,
   `font_metrics.rs`) — confirmar que `stretchy.rs`/`assembly.rs`/`attach.rs` (`math_kern`) o
   consomem todos da mesma função, sem cópia paralela.
4. **`hor_advance` vs. `advance`/`full_advance`** (achado de P917): confirmar que a distinção já
   está correctamente exposta como conceito reutilizável em `GlyphVariant`/`GlyphPart`, não só
   corrigida nos dois call sites que motivaram o achado — se um décimo módulo precisar de glifo
   esticável no futuro, tem de ser óbvio qual campo usar para quê.
5. **Kerning de duas alturas** (`compute_math_kern`, P914): confirmar se é especificamente de
   `attach.rs` (sub/sobrescrito) ou se poderia servir outro caso futuro — não forçar
   generalização se só houver um consumidor real hoje.

Para cada candidato: **decidir se vale extrair agora** (usado por 2+ módulos, fórmula idêntica
confirmada) ou **deixar como está** (um só consumidor, ou variação legítima entre módulos que só
parece igual à primeira vista). Não extrair por extrair — o critério é duplicação real confirmada,
não semelhança superficial.

## Fase A.1 — desenhar o núcleo partilhado (gate, mesmo protocolo de P893/896/906/909)

1. Para cada candidato confirmado na Fase A: desenhar a função/módulo partilhado (provavelmente
   `math/layout/_geometry.rs` ou extensão de `_comum.md`, a decidir).
2. Resolver o achado em aberto de P906 (gap de `underover` sem constante explícita) como parte
   desta extração, se a Fase A confirmar que faz sentido unificá-lo com o gap de `frac.rs`.
3. Registar a decisão, editar os L0s afectados, sincronizar hashes — **parar para confirmação do
   dono antes da Fase B**, dado o tamanho do refactor (toca potencialmente 9 módulos).

## Fase B — implementação

1. Migrar módulo por módulo (não tudo de uma vez), cada um com o próprio commit/verificação —
   mesma disciplina de dividir por parte já usada em P899.
2. Cada módulo migrado: suíte verde, mesmo comportamento.
3. `crystalline-lint --fix-hashes .` a cada módulo.
4. **Prova final, mesmo padrão de P909**: PDF do `.typ` de 30 secções byte-idêntico antes/depois
   de toda a migração — não só "suíte verde", prova mais forte de content-preserving.
5. Se a Fase A.1 tiver decidido resolver o gap de `underover` (item 2 acima): esse ponto
   específico **pode** mudar o output visual (é uma correção real, não só reorganização) —
   isolar esse commit dos de puro refactor, para não misturar "mudou porque foi corrigido" com
   "mudou porque foi movido" no mesmo diff.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20` (ou mais, se o padrão de ruído de P915
continuar a aparecer). Attestation completa per `ADR-0123`/`L11` (Tekt): comando exacto, recibo
(números), comparação — não "sem regressão" em prosa.

## Resultado esperado

- Relatório da Fase A: tabela dos 5 candidatos, veredicto de cada um (extraído / mantido separado,
  com razão).
- Núcleo geométrico partilhado implementado onde confirmado, com gate de confirmação cumprido.
- PDF byte-idêntico como prova de content-preserving (separado do commit que resolve o gap de
  `underover`, se aplicável).
- Suíte completa com números reais, discriminada por crate.
- Benchmark completo, atestado (executor/recibo/comparação, não adjectivo).

## Se o escopo crescer durante a Fase A

Mesmo critério já usado em P899/906: se um dos cinco candidatos revelar mais complexidade do que
cabe num passo (por exemplo, se unificar o gap de `underover` exigir revisitar a tabela MATH
inteira outra vez), destacar para passo próprio em vez de forçar caber aqui — registar a decisão,
não empurrar silenciosamente.
