# Passo 892 — `italic_correction` da tabela MATH nunca lido (candidato para o gap de `i²` que P891 não fechou)

**Precede este passo**: `typst-passo-891-relatorio.md`, secção final "Confirmação visual — achado
importante", último parágrafo. Ler antes de começar.

**Contexto herdado de P891**: a correção de `math_kern` estava certa (confirmada com `D_2`, efeito
real medido), mas não fechou o sintoma original de `i²` — numa comparação fresca e sincronizada,
vanilla e cristalino produziram o mesmo resultado (kern zero) para esse par específico. P891 já
confirmou, por leitura directa do código, que `attach.rs` **nunca lê** `italic_correction` — campo
per-glifo da tabela MATH, distinto de `kern_infos` (que `math_kern` já lê desde P891).

**Por que `italic_correction` é candidato plausível, não só "mais um campo em falta"**: em OpenType
MATH, o posicionamento horizontal de um sobrescrito é tipicamente `avanço_da_base +
correcção_itálica_da_base`, não só o avanço puro. Para bases sem dados de `kern_infos` relevantes
(como pode ser o caso de `𝑖`, confirmado em P891 que retorna `None`), a correcção itálica é
frequentemente o único ajuste que existe para compensar a inclinação da base — omiti-la produziria
exactamente um sobrescrito posicionado "cedo demais" ou com espaço estranho antes, dependendo de
como o resto do cálculo compensa (ou não) a ausência.

**Pré-condição de árvore**: `git status`. P891 deixou trabalho pendente por commitar
(`font_metrics.rs`, vários L0s) — confirmar se já foi commitado antes deste passo; se não, decidir e
registar, mesmo padrão dos passos anteriores.

---

## Fase A — Diagnóstico

1. Confirmar, via `fontTools`/`ttx` (mesma ferramenta usada em P889/P890/P891), se
   `NewComputerModernMath` tem `italic_correction` **não-zero** para `𝑖` (U+1D456) — sem isto
   confirmado primeiro, o resto do passo não tem fundamento. Se for zero também para este glifo,
   parar aqui, registar, e considerar a hipótese descartada (o gap teria de vir de outro lugar
   ainda não identificado).
2. Se confirmado não-zero: ler exactamente como o vanilla usa `italic_correction` no cálculo de
   posição do sobrescrito (`lab/typst-original/`, módulo de `attach`/superscript positioning — não
   presumir a fórmula "avanço + correcção itálica" do parágrafo acima sem confirmar no código-fonte
   real; pode haver condições adicionais, ex: só aplicar quando não há `kern_infos` relevante, ou
   sempre somar independentemente).
3. Confirmar se o trait `FontMetrics` (`01_core/src/engine/layout/metrics.rs`) já tem algum método
   para expor `italic_correction`, ou se precisa de um novo (mesmo padrão de `math_kern`: default
   no trait, implementação real em `FallbackFontMetrics` via `03_infra/src/font_metrics.rs`, usando
   o mesmo mecanismo de resolução de face de P890/P891 — `resolve_primary_with_math_fallback` +
   `covering`).
4. Confirmar o ponto exacto em `attach.rs` onde a posição do sobrescrito é calculada
   (`~linhas 219-227`, mesma área já usada por P889/P891 para `math_kern`) e onde a correcção
   itálica entraria na fórmula.

**Não avançar para a Fase B sem a Fase A confirmar que `italic_correction` é de facto não-zero para
o caso observado** — evitar repetir o padrão de implementar em cima de hipótese não verificada.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

1. Se for preciso método novo no trait: mudança mecânica de assinatura primeiro (suíte verde antes
   de qualquer lógica nova), mesmo padrão que P891 usou para `math_kern(c, style)`.
2. Teste que falhe primeiro: valor esperado de `italic_correction` calculado directamente via
   `ttf_parser` no próprio teste (não hardcoded — mesmo padrão do teste de `math_kern` em P891, que
   evitou ficar frágil a actualização do asset).
3. Implementar a leitura real e a aplicação em `attach.rs` conforme a fórmula confirmada na Fase A.
4. Suíte completa verde, discriminada por crate.
5. Recompilar `04-math.typ` (fonte actual, hash já confirmado em passos anteriores) nos dois
   binários, no mesmo momento, e traçar (`mutool trace` ou equivalente, mesmo método de P891) a
   posição exacta do glifo `²` antes/depois. Comparar com o vanilla para o mesmo par — desta vez
   confirmar que a comparação está de facto sincronizada (mesmo hash de `.typ`, mesmo momento),
   para não repetir a suspeita de erro metodológico que já pairou sobre este mesmo sintoma duas
   vezes (achados 1/4 de P885, e agora este).
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Mesma exigência dos passos anteriores desta frente — benchmark completo, 7 cenários, comparar com
a baseline de P891.

## Resultado esperado

- Header de linhagem actualizado, L0 actualizado se necessário (gate do Protocolo de Nucleação,
  mesmo processo de P890/P891 — STOP e aguardar confirmação antes da Fase B se algum L0 for
  editado).
- Teste novo com ground-truth calculado no próprio teste.
- Relatório com: valor de `italic_correction` confirmado na Fase A, fórmula do vanilla confirmada
  por leitura de código, posição do glifo antes/depois medida por traçado directo (não visual a
  olho), confirmação explícita de que a comparação final vanilla-vs-cristalino foi sincronizada.
- Se a Fase A refutar a hipótese (valor zero também aqui): registar isso como achado fechado por
  descarte, e o gap de `i²` volta a ficar sem causa confirmada — não inventar uma quarta hipótese
  sem evidência nova.
