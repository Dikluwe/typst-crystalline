# Passo 893 — `FallbackFontMetrics::math_constants` usa fallback fixo, não a tabela MATH real da fonte

**Precede este passo**: `typst-passo-891-relatorio.md`, secção "Achado colateral, fora de âmbito
deste passo". Ler antes de começar.

**Por que isto é maior que os dois achados já corrigidos (`math_kern`, e a tentativa de
`italic_correction` em P892)**: `math_kern`/`italic_correction` afectam o posicionamento pontual de
um glifo específico face a outro. `math_constants` — `MathConstants::fallback()` em vez da tabela
MATH real — afecta **todas** as proporções de **toda** equação: altura do eixo matemático, espessura
da barra de fração, factor de redução de sub/sup, espaçamento de radicais, e mais. Se a fonte
activa tiver constantes diferentes do fallback hardcoded, cada equação no documento está
potencialmente com proporções levemente erradas, não só os casos pontuais já observados.

**Escopo deste passo**: só `math_constants`. `vertical_glyph_variants`/`vertical_glyph_assembly`
(delimitadores extensíveis) ficam explicitamente fora — são um problema diferente (crescimento de
glifo, não proporção contínua) e maior em complexidade própria; tratar num passo dedicado
posterior, não misturar aqui.

**Pré-condição de árvore**: `git status`. Confirmar se P891/P892 já estão commitados antes de
começar; decidir e registar se não.

---

## Fase A — Diagnóstico

1. Confirmar exactamente quais constantes `MathConstants::fallback()` define
   (`01_core/src/engine/math/` — localizar o ficheiro certo, não presumir) e comparar com a lista
   completa de constantes da tabela `MathConstants` do OpenType MATH spec (axis_height,
   fraction_rule_thickness, script_percent_scale_down, script_script_percent_scale_down,
   sub/superscript_shift_*, etc. — usar a spec como referência, não a memória).
2. Confirmar, via `fontTools`/`ttx`, os valores reais dessas constantes em
   `NewComputerModernMath-Regular.otf` (a fonte primária da cadeia math), e comparar com os valores
   do fallback hardcoded no código. Quantificar a divergência para pelo menos 3-4 constantes chave
   (`axis_height`, `fraction_rule_thickness`, `script_percent_scale_down`) — se os valores forem
   idênticos ou muito próximos, o impacto prático deste achado pode ser pequeno apesar do raciocínio
   teórico acima; medir antes de assumir que é grave.
3. Confirmar como o vanilla lê `MathConstants` da fonte real (localizar a função equivalente em
   `lab/typst-original/`) — usar como referência de implementação, não reinventar a leitura da
   tabela do zero.
4. Confirmar o trait `FontMetrics` — método `math_constants` já existe com default (per o achado de
   P891), precisa de implementação real em `FallbackFontMetrics`, mesmo padrão dos dois passos
   anteriores (resolver face via `resolve_primary_with_math_fallback` + `covering`, ler a tabela
   MATH dessa face).

**Se a Fase A (ponto 2) encontrar divergência desprezível entre fallback e valores reais**: registar
isso explicitamente e decidir com o dono do projecto se ainda vale a pena implementar a leitura real
(correcção teoricamente correcta, mas de impacto prático baixo) ou se fica documentado como
scope-out deliberado por ora. Não presumir que "está errado" implica automaticamente "vale a pena
corrigir agora" — são perguntas distintas.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

Só depois da Fase A confirmar que vale a pena prosseguir.

1. Teste(s) que falhem primeiro: valores de constantes lidos directamente via `ttf_parser` no
   próprio teste (ground-truth calculado no teste, mesmo padrão de P891/P892 — não hardcoded), para
   pelo menos as constantes identificadas como mais impactantes na Fase A.
2. Implementar a leitura real, delegando para uma função partilhada se fizer sentido (mesmo padrão
   de `math_kern_from_face` extraída em P891).
3. Suíte completa verde, discriminada por crate.
4. Recompilar `04-math.typ` e, se possível, um documento com fração (`frac(a, b)` ou equivalente,
   para exercitar `fraction_rule_thickness`, que `04-math.typ` actual não usa) — confirmar
   visualmente que as proporções mudam de forma consistente com os valores reais da fonte, e que
   nada quebra visualmente onde os valores do fallback já estavam próximos dos reais.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, comparar com a baseline mais recente (P892, se já tiver fechado;
senão P891). Esta correcção mexe em código de layout matemático puro — não se espera impacto de
tempo mensurável, mas confirmar de qualquer forma, mesma disciplina dos passos anteriores.

## Resultado esperado

- Header de linhagem actualizado, L0 actualizado (gate do Protocolo de Nucleação, STOP antes da
  Fase B se algum L0 for editado).
- Teste(s) novo(s) com ground-truth calculado nos próprios testes.
- Relatório com: divergência quantificada entre fallback e valores reais (Fase A), decisão explícita
  se valia a pena prosseguir, confirmação visual com pelo menos um documento que exercite
  `fraction_rule_thickness` além do `04-math.typ` já usado nos passos anteriores.
- Se decidido não prosseguir na Fase A: relatório de diagnóstico puro, sem Fase B/C, registando a
  divergência medida e a decisão de scope-out, mesmo padrão já usado noutros passos desta frente
  quando a investigação conclui que não vale a pena avançar.
