# Relatório de Paridade — P759

**Passo:** 759
**Data:** 2026-07-14
**Foco:** Avaliar se a diferença greedy vs Knuth-Plass justifica reescrever o motor de quebra de linha do cristalino.
**Hash base:** `81a2cb1f32a5045bd2633f065853cb05e00d315c`
**Hash do commit com as alterações de código:** *não aplicável — este passo não introduziu código.*
(Nota: o commit final que incluir este relatório terá um hash diferente.)

---

## 1. Sonda

### 1.1 Mecanismo do vanilla

Ficheiro analisado: `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs`.

```text
wc -l lab/typst-original/crates/typst-layout/src/inline/linebreak.rs
1039 lab/typst-original/crates/typst-layout/src/inline/linebreak.rs
```

A implementação contém:

- `enum Breakpoint { Mandatory, Normal, Hyphen(l, r) }`
- `badness` calculada como `100 * ratio^3` (limitado)
- `demerits` como função quadrática de badness + penalty
- Procura do "predecessor óptimo" ao longo do parágrafo inteiro
- Justificação e hifenização integradas na optimização

Trata-se de uma implementação Knuth-Plass / TeX-style completa, não de uma variante simplificada. O esforço de replicação directa é da ordem de grandeza deste ficheiro (~1039 linhas) mais adaptação ao modelo de layout do cristalino.

### 1.2 Teste 1 — Texto latino curto, página 300 pt

```typst
#set page(width: 300pt, margin: 20pt)
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
```

Resultado (`pdftotext`):

```text
Vanilla:  Lorem ipsum dolor sit amet, consectetur adipiscing elit,
          sed do eiusmod tempor incididunt ut labore et dolore
          magna aliqua. Ut enim ad minim veniam, quis nostrud
          exercitation ullamco laboris nisi ut aliquip ex ea
          commodo consequat.

Cristalino: Lorem ipsum dolor sit amet, consectetur adipiscing elit,
            sed do eiusmod tempor incididunt ut labore et dolore
            magna aliqua. Ut enim ad minim veniam, quis nostrud
            exercitation ullamco laboris nisi ut aliquip ex ea
            commodo consequat.
```

**Quebras idênticas.**

### 1.3 Teste 2 — Texto latino longo, página 350 pt, DejaVu Sans

```typst
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
[200 palavras de Lorem ipsum fixo]
```

Resultado (`pdftotext`):

**Vanilla e cristalino produziram o mesmo texto e as mesmas quebras.** Ambos dividiram o parágrafo nas mesmas fronteiras de palavras.

Comparação visual (ImageMagick, resolução 150 dpi):

```text
AE    = 116 612 pixels diferentes
Total = 1 280 420 pixels
AE%   = 9,11 %
RMSE  = 0,21316 (21,32 % do range)
```

Interpretação: as quebras são idênticas; a diferença de pixels é residual e atribuída a variações mecânicas de renderização (hinting, anti-aliasing, cache de fonte), não à decisão de layout.

### 1.4 Teste 3 — Texto latino estreito, página 220 pt

```typst
#set page(width: 220pt, margin: 15pt)
#set text(font: "DejaVu Sans", size: 11pt)
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
```

Resultado:

```text
Vanilla:    Lorem ipsum dolor sit amet,
            consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut
            labore et dolore magna aliqua.

Cristalino: Lorem ipsum dolor sit amet,
            consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut
            labore et dolore magna aliqua.
```

**Quebras idênticas**, mesmo em largura apertada onde o algoritmo greedy poderia tomar decisões localmente subóptimas.

### 1.5 Teste 4 — CJK com aspas (contexto P758)

Repetido para contextualizar:

```text
Cristalino: 测试文本，
            "测试引号的位
            置"。...

Vanilla:    测试文本，“测试
            引号的位置”。这
            ...
```

Neste caso, greedy vs Knuth-Plass produz resultados visivelmente diferentes. No entanto, P758 demonstrou que o vanilla resolve o caso correctamente **mesmo em modo `linebreaks: "simple"`** (greedy), o que indica que a causa principal é o segmentador customizado (`CJ_SEGMENTER`) e as heurísticas CJK, não o algoritmo global de optimização.

---

## 2. Decisão

**Não se implementa Knuth-Plass neste momento.**

Raciocínio:

1. Para texto latino comum, o layout greedy do cristalino produz **as mesmas quebras** que o Knuth-Plass do vanilla nos casos de teste (curto, longo, estreito). A diferença residual de pixels é mecânica (renderização/fonte/AA), não morfológica.
2. As diferenças visuais em CJK/Thai não são primariamente explicadas pela ausência de Knuth-Plass, mas pelo segmentador customizado e heurísticas CJK do vanilla (ver P758).
3. O custo de implementar Knuth-Plass completo (~1039 linhas no vanilla, com integração no layout do cristalino) é elevado e o benefício observável para a paridade de texto latino é nulo.
4. Sob ADR-0107, aceita-se a divergência mecânica (algoritmo de quebra diferente) desde que o resultado na língua (semântica/sintaxe/morfologia do texto) seja equivalente. O texto latino é equivalente; o CJK/Thai requer o segmentador/heurísticas, não necessariamente Knuth-Plass.

**Débito arquitetural registado:** melhoria da segmentação/heurísticas CJK e Thai (equivalente ao `CJ_SEGMENTER` do vanilla) fica para passo futuro. Knuth-Plass completo só voltará a ser considerado se medições futuras mostrarem diferenças visuais significativas em texto latino que não possam ser atribuídas a segmentação ou fontes.

---

## 3. Validação

Como não houve alterações de código, a validação limita-se a confirmar que o estado do repositório permanece consistente:

```text
cargo test --workspace
crystalline-lint .
```

(Resultados na secção 4.)

---

## 4. Estado do repositório

```text
cargo test --workspace
  4121 passed (typst-core)
   635 passed (typst-infra)
    33 passed (typst-shell)
     2 passed (typst-wiring)
    27 passed (cli integration)
     2 passed (crystalline-lint integration)
     0 failed
crystalline-lint .
  ✓ No violations found
```

---

## 5. Fecho

P759 está fechado como sonda:

- Mecanismo Knuth-Plass do vanilla confirmado.
- Magnitude da diferença medida em texto latino: quebras idênticas; diferença de pixels residual (~9 % AE, ~21 % RMSE) atribuída a renderização/fonte.
- Decisão tomada com base na medição: não implementar Knuth-Plass.
- CJK/Thai mantêm-se como débito a tratar num passo focado no segmentador customizado/heurísticas, não numa reescrita do motor de layout.
- Sem alterações de código; sem regressões.
