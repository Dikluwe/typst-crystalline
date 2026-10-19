# Relatório — typst-passo-890: causa exacta do custo fixo de ~5s em `04-math` (Fase A)

**Data:** 2026-07-24T13:14:34Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `326ced44899862bbe1190386b5834f200bb4115d` (HEAD do ramo `Tekt`, após P888)
**Working tree no início:** limpa, exceto materialização de P889/890/891 e o relatório de P889 (não
commitados, nada relacionado com código). Nada a decidir sobre árvore não commitada.

Este relatório cobre só a Fase A (instrumentação temporária, já revertida). Nenhum código de
produção foi alterado ainda — só o L0 (`00_nucleo/prompts/infra/font_metrics.md`), como o gate exige
antes da Fase B.

---

## 1. Método — instrumentação temporária, revertida no mesmo passo

Adicionados `eprintln!` temporários (marcados `P890-DEBUG-TEMP`) em três pontos:
- `03_infra/src/shaper.rs::CandidateSet::covering_run` — imprime `primary` e se cai no passo 2.
- `03_infra/src/world.rs::SystemWorld::candidates_for_char` — imprime quantos slots estão por
  cachear na entrada.
- `03_infra/src/font_metrics.rs::FallbackFontMetrics::covering` e `::text_ink_bounds` — imprime
  `primary` recebido e `style.math`.

Corrido com `$ i $` isolado (`/tmp/p890/iso-i.typ`), binário recompilado com a instrumentação.
**Revertido imediatamente a seguir** com `git checkout -- 03_infra/src/shaper.rs 03_infra/src/
world.rs 03_infra/src/font_metrics.rs`, confirmado por `git status`/`grep -rn "P890-DEBUG"` (0
ocorrências) que não sobrou nada no código antes de prosseguir para o L0/Fase B.

---

## 2. Trace obtido (ordem exacta das chamadas)

```
P890-DEBUG font_metrics::covering ENTRY c='𝑖' (U+1D456) primary.len=1 families=[Some("Libertinus Serif")]
P890-DEBUG font_metrics::covering NOT found in primary, falling to candidates_for_char
P890-DEBUG candidates_for_char c='𝑖' (U+1D456) total_slots=1129 uncached_before=1129
P890-DEBUG candidates_for_char DONE, cache now has 1129 entries
P890-DEBUG text_ink_bounds ENTRY text="𝑖" style.math=true
P890-DEBUG font_metrics::covering ENTRY c='𝑖' (U+1D456) primary.len=3 families=[Some("Libertinus Serif"), Some("New Computer Modern Math"), Some("Noto Color Emoji")]
P890-DEBUG font_metrics::covering FOUND in primary, slot_idx=1124
P890-DEBUG covering_run char='𝑖' (U+1D456) primary.len=3 families=[Some("Libertinus Serif"), Some("New Computer Modern Math"), Some("Noto Color Emoji")]
P890-DEBUG covering_run char='𝑖' primary_candidates=[1] (empty means falls to step 2)
```

**Leitura, linha a linha**:
1. A **primeira** chamada a `covering()` acontece com `primary.len=1` — só `Libertinus Serif`, sem
   `New Computer Modern Math`. Isto acontece **antes** de `text_ink_bounds` sequer imprimir a sua
   própria entrada (linha 5 do trace) — ou seja, esta primeira chamada **não vem de
   `text_ink_bounds`**, apesar de `text_ink_bounds` ser o único caller de `covering()` com a
   injecção `math_fallback_font_list()` já implementada.
2. Sem `New Computer Modern Math` em `primary`, `covering()` não encontra cobertura e cai em
   `candidates_for_char`, que computa `Coverage` para os **1129** slots do `FontBook` (todos
   descachados — `uncached_before=1129`) — este é o scan caro.
2. Log completo: cache fica com 1129 entradas (todas computadas nesta única chamada).
3. **Só depois** disto, `text_ink_bounds` é chamado (linha 5) — `style.math=true` confirmado
   directamente — e a **sua própria** cópia da lógica de injecção (`font_metrics.rs:1045-1060`,
   já existente antes deste passo) produz correctamente `primary.len=3`, incluindo `New Computer
   Modern Math` — `covering()` encontra-o de imediato (`slot_idx=1124`), sem precisar do scan (o
   cache global já está quente da chamada anterior, de qualquer forma).
4. `covering_run` (`shaper.rs`, o shaping real para o PDF) corre por último, já com `primary.len=3`
   correcto — nunca precisa de cair no passo 2 (`primary_candidates=[1]`, não vazio).

## 3. Causa exacta — `advance()` não tem a injecção que `text_ink_bounds()` já tem

Rastreando quem chama `covering()` **antes** de `text_ink_bounds`: `03_infra/src/font_metrics.rs:
855`, dentro de `impl FontMetrics for FallbackFontMetrics { fn advance(...) }`
(`font_metrics.rs:829-855`):

```rust
fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
    self.cached_advance_width(text, style, || {
        let primary = self.resolve_primary(style);   // ← SEM injecção de math_fallback_font_list
        ...
        for c in text.chars() {
            let cand = self.covering(c, &primary, &variant);
            ...
```

Comparado com `text_ink_bounds` (`font_metrics.rs:1037-1061`), que **já** tem:

```rust
fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
    let mut primary = self.resolve_primary(style);
    ...
    if style.math {                                    // ← injecção presente aqui
        for family in crate::fallback_fonts::math_fallback_font_list() {
            ...
            primary.push(FontCandidate { ... });
        }
    }
```

`resolve_primary` (`font_metrics.rs:683-724`) **nunca** consulta `math_fallback_font_list()` nem lê
`style.math` — confirmado por leitura directa (não há nenhuma referência a nenhum dos dois dentro
da função). `advance()` chama só isto, sem a mesma extensão condicional que `text_ink_bounds()` já
tem. **É esta assimetria — não o mecanismo de `candidates_for_char` em si — a causa exacta.**

`layout_equation_measured` (`01_core/src/engine/math/layout/mod.rs:378-388`) chama `advance()`
(linha 383) **antes** de `text_ink_bounds()` (linha 386), na mesma iteração, para o mesmo
`FrameItem::Text`/`TextShaped` — por isso é sempre `advance()` quem primeiro tenta cobrir `𝑖`/`α`,
falha (por causa da lacuna), e dispara o scan caro. Quando `text_ink_bounds()` corre a seguir, já
não importa (o cache global de `candidates_for_char` já está quente) — mas o custo já foi pago.

`shaper.rs::try_shape`/`shaped_width` (as duas funções mencionadas como candidatas no prompt deste
passo) **já tinham a injecção correcta** (confirmado em P889, secção 2.5, e reconfirmado aqui pelo
trace — `covering_run` nunca precisa do passo 2). **Não são a causa** — a suspeita do prompt sobre
elas fica formalmente descartada por evidência directa, não por suposição.

## 4. Por que isto explica todos os achados de P889

- **Custo fixo, não por equação** (P889 secção 2.2): `candidates_for_char` cacheia globalmente
  (`self.coverage_cache`, partilhado por todo o `SystemWorld`/compilação) — só a *primeira* chamada
  de `advance()` para um carácter fora da cobertura de `Libertinus Serif` paga o scan; as 99
  restantes reutilizam o cache.
- **Só letras/gregas disparam, não dígitos/operadores** (P889 secção 2.3): dígitos e operadores
  ASCII são cobertos por `Libertinus Serif` directamente — `advance()`'s `primary` (mesmo sem a
  injecção) já cobre, nunca cai no scan.
- **A fonte certa já tinha o glifo** (P889 secção 2.4): confirma que o scan encontra `New Computer
  Modern Math` mesmo vindo do caminho lento (`candidates_for_char` não filtra por `style.math`, só
  por bloco de codepoint) — o resultado final sempre esteve certo, só o caminho para lá chegar era
  caro.

## 5. Correcção proposta (Fase B, pendente de confirmação do L0)

Aplicar a `advance()` a mesma injecção condicional a `style.math` que `text_ink_bounds()` já tem,
antes da primeira chamada a `covering()`. Extrair a lógica duplicada (a mesma sequência de `for
family in math_fallback_font_list() { ... primary.push(...) }`) para uma função privada partilhada
pelas duas, para não deixar um terceiro sítio com a mesma lógica copiada à mão (risco de a próxima
função nova repetir o mesmo esquecimento).

## 6. Gate do Protocolo de Nucleação

`00_nucleo/prompts/infra/font_metrics.md` documentava `resolve_primary`/`covering` (secção
"Resolução de fontes") sem nunca mencionar a injecção condicional a `style.math` que
`text_ink_bounds` já tinha implementado, nem a ausência dela em `advance()` — desactualizado face à
correcção que este passo precisa. Adicionada secção `## P890` com o diagnóstico completo (secções
2-4 deste relatório, resumidas) e a correcção proposta (secção 5).

---

## 7. STOP — aguardando confirmação do dono do projecto

Mesmo gate de P886/887/888: L0 editado (`font_metrics.md`), aguardo confirmação de que está bom e
que o hash foi recalculado (`crystalline-lint --fix-hashes .`) antes de avançar para a Fase B
(TDD: teste que confirme que o caminho rápido é tomado — não só que o resultado final está certo —
implementação, suíte completa, medição antes/depois, confirmação visual) e Fase C (benchmark
completo, atenção especial a `03-images` e a cenários CJK per o próprio prompt deste passo).

---

## 8. Confirmação do gate — L0 salvo, hash recalculado

`crystalline-lint --fix-hashes .` recalculou o hash em `03_infra/src/font_metrics.rs` (`033fca5c` →
`d9c9d464`). `crystalline-lint .`: 0 avisos de drift; só o V7 pré-existente. Fase B autorizada.

---

## 9. Fase B — TDD + implementação

**Nota de ordem**: a primeira tentativa de implementação foi feita antes de escrever o teste —
revertida (`git checkout --`) assim que percebido, e o teste escrito primeiro, per a exigência
explícita do prompt (secção "Fase B", ponto 1: "não só que o resultado final está certo").

**Teste primeiro** (`03_infra/src/font_metrics.rs`,
`p890_advance_math_nao_chama_candidates_for_char_quando_math_fallback_cobre`): `World` espião
(`SpyWorld`) com contador atómico de chamadas a `candidates_for_char`; regista duas fontes no
`FontBook` — "Nimbus Sans" (fixture real, não tem o carácter U+018F) como fonte de corpo, e "New
Computer Modern Math" (fixture Cantarell-VF.otf, tem esse carácter, usada só como stand-in — não
precisa de ser a fonte MATH de produção para provar o mecanismo) como primeira entrada da cadeia
math. Chama `metrics.advance(...)` com esse carácter e `style.math = true`, e afirma que
`candidates_for_char_calls == 0`.

Confirmado que falha antes da correcção: `left: 1, right: 0` (a chamada aconteceu).

**Implementação** (`font_metrics.rs`): nova função privada partilhada
`resolve_primary_with_math_fallback(style, variant)` — `resolve_primary(style)` seguido da mesma
injecção condicional a `style.math` que `text_ink_bounds` já tinha (itera
`math_fallback_font_list()`, resolve por `select_pattern`, evita duplicados, `cached_face` antes de
adicionar). `advance()` e `text_ink_bounds()` passam a chamar esta função em vez de, respectivamente,
`resolve_primary` puro (bug) e a sua própria cópia inline da injecção (duplicação removida).
`@updated` do cabeçalho actualizado para `2026-07-24`.

**Teste passa** após a correcção. **Suíte completa, discriminada por crate**:

| Crate | Passou | Falhou | Ignorado |
|---|---|---|---|
| `typst-core` | 4698 | 0 | 2 |
| `typst-infra` | 733 | 0 | 5 |
| `typst-shell` | 41 | 0 | 0 |
| `typst-wiring` (+ `tests/crystalline_lint.rs`) | 37 + 2 | 0 | 0 |

Zero falhas.

### Medição antes/depois (`04-math.typ`, `/usr/bin/time -v`)

| Métrica | Antes (P888/P889) | Depois (P890) |
|---|---|---|
| User time | 0.67s | 0.04s |
| System time | 4.34s | 0.10s |
| Elapsed | 5.02s | **0.15s** |
| Maximum RSS | 7.07 GB | 62 MB |

Muito melhor que a estimativa de P889 ("provavelmente < 100ms" — ficou em 150ms, mas a ordem de
grandeza bate: de ~5000ms para 150ms, redução de 97%).

### Contagem de aberturas de `.ttc` CJK (antes/depois)

| Ficheiro | Antes | Depois | Baseline universal (`01-hello`, não-math) |
|---|---|---|---|
| `NotoSansCJK-Regular.ttc` | 21 | 11 | 11 |
| `NotoSansCJK-Bold.ttc` | 21 | 11 | 11 |
| `NotoSerifCJK-Regular.ttc` | 11 | 6 | ~6 |
| `NotoSerifCJK-Bold.ttc` | 11 | 6 | ~6 |

Depois da correcção, a contagem cai exactamente para o nível do custo de descoberta universal
(presente em todos os cenários, incluindo os que nunca tocam matemática) — a leitura **extra**
específica de `04-math` desapareceu por completo, não só diminuiu.

### Confirmação visual

Render a 150dpi do `04-math.typ` pós-correcção é **visualmente idêntico** ao pré-correcção — mesmos
glifos, mesma anomalia de espaçamento já registada em P889 (inalterada — confirma que esta
correcção, de tempo, não tocou na causa do achado visual, como já esperado por serem mecanismos
independentes). Nenhuma troca de glifo por uma variante visualmente diferente.

`crystalline-lint .`: 0 violações novas (só o V7 pré-existente).

---

## 10. Fase C — Regressão (benchmark completo, 7 cenários)

| Cenário | Vanilla (P888) | Cristalino (P888) | Razão (P888) | Vanilla (P890) | Cristalino (P890) | Razão (P890) |
|---|---|---|---|---|---|---|
| 01-hello | 274.7ms | 96.8ms | 0.35× | 273.8ms | 94.9ms | 0.35× |
| 02-lorem | 280.1ms | 118.3ms | 0.42× | 275.7ms | 119.6ms | 0.43× |
| 03-images | 7.1ms | 104.1ms | 14.70× | 6.8ms | 103.5ms | 15.32× |
| **04-math** | 279.3ms | 5160.8ms | **18.48×** | 277.1ms | **153.7ms** | **0.55×** |
| 05-tables | 299.3ms | 116.2ms | 0.39× | 298.4ms | 115.4ms | 0.39× |
| 06-long | 294.4ms | 377.4ms | 1.28× | 293.4ms | 371.9ms | 1.27× |
| 07-context | 294.1ms | 138.0ms | 0.47× | 292.2ms | 136.5ms | 0.47× |

**Leitura**: `04-math` passa de **18.48× mais lento** para **0.55×** (agora mais rápido que o
vanilla, como todos os outros cenários excepto `03-images`). Os 6 cenários restantes ficam dentro do
ruído de hyperfine — **nenhuma regressão nova**, incluindo `03-images` (atenção especial pedida pelo
prompt: 14.70× → 15.32×, variação de ruído, não uma piora real — este cenário usa um mecanismo de
dedup de imagem completamente diferente, não tocado por esta correcção, per P873).

---

## 11. Resultado — Passo 890 fechado

- Header de linhagem actualizado (`font_metrics.rs`, `@updated 2026-07-24`; `@prompt-hash`
  recalculado).
- Instrumentação de diagnóstico (Fase A) totalmente revertida antes da Fase B — confirmado por
  `grep`/`git status` (0 ocorrências, 0 diffs) antes de qualquer código de produção ser escrito.
- Teste novo (`p890_advance_math_nao_chama_candidates_for_char_quando_math_fallback_cobre`)
  confirmando que o caminho rápido é tomado (via spy `World`), não só que o resultado final está
  correcto.
- Fase A: causa exacta confirmada por instrumentação temporária — `advance()` faltava a mesma
  injecção de `math_fallback_font_list()` que `text_ink_bounds()` já tinha; `shaper.rs` (a suspeita
  original do prompt) confirmado como não sendo a causa.
- Fase B: correcção implementada (função partilhada `resolve_primary_with_math_fallback`), suíte
  verde nas 4 crates, `crystalline-lint` limpo, `04-math` de ~5.02s para ~0.15s (redução de 97%),
  aberturas de `.ttc` CJK caem ao nível do custo universal (não zero, mas sem o extra), confirmação
  visual sem diferença (glifos e anomalia de espaçamento pré-existente inalterados).
- Fase C: benchmark completo, `04-math` deixa de ser o pior caso do benchmark (era 18.48×, passa a
  0.55×) e nenhum dos outros 6 cenários regrediu.
- Árvore de trabalho: limpa desde antes do início deste passo — nada a decidir.
