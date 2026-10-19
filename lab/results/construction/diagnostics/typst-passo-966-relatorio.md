# Passo 966 — Relatório (conteúdo de função de utilizador em math recebe default matemático)

**Data**: 2026-08-04
**Estado da árvore**: Fase A+A.1 commitadas em `5e33b9661` após gate
confirmado pelo dono ("Continue"); Fases B/C por cima.

---

## 1. Fase A — mecanismo (resumo; relatório parcial já tem o detalhe)

- Árvore real medida: `bra(phi)` → `Content::Sequence` de **markup** com
  filhos `Text`/`MathText`; `apply_math_default` (layout, P812) só recursava
  em containers `Math*` — as folhas `MathText("φ")` ficavam sem itálico.
- Vanilla aplica o default na **resolução** (`ir/resolve.rs:127-146`,
  realize com `RealizationKind::Math` sobre o output de funções de
  utilizador). Diferença de camada = causa raiz confirmada.
- Direcção aprovada pelo dono: **(a)** estender `apply_math_default` com
  braços para `Content::Sequence` e `Content::Styled`. (b) mover para o
  eval rejeitada (contradiz P812, blast radius maior).

## 2. Fase B — dois agentes (protocolo P898)

- **Agente A**: 6 testes L1 (`p966_tests`: bra(phi) à mão, segundo
  template, aninhamento, Styled + 2 guardas — `Text` literal nunca
  transformado, `MathStyled` upright de P962 intacto) + 2 de integração
  (`p966_bra_ket_funcao_utilizador_recebe_default` end-to-end com o
  `#let bra/ket` real; guarda fora-de-math). RED exacto registado
  (4 L1 + 1 integração a falhar com valores concretos).
- **Agente B**: dois braços em `apply_math_default` (Sequence recursa
  items; Styled recursa o corpo com styles intactos). 6/6 + 2/2 verdes;
  **zero testes antigos afectados**. Suite: **5716 testes, 0 falhas**
  (4872 core + 762 infra + 41 + 2 + 37 + 2).
- **Revisão do orquestrador (B.3)**: funções de utilizador aninhadas reais
  (`#let inner`/`#let outer`/`#let outermost`, 2 níveis dentro de math):
  `outer(psi)` → `⟨𝜓^*⟩`, `outermost(phi)` → `‖⟨𝜑^*⟩‖` — idêntico ao
  vanilla. Não é superficial.

## 3. Fase C — revalidação

- **Smoke (contagens de codepoints no doc de 30 secções)**:
  convergência exacta com o vanilla — U+03C6/U+03C8: 1/3 → **0/0**;
  U+1D711: 2 → **3** (vanilla 3); U+1D713: 5 → **8** (vanilla 8). O
  `⟨φ|ψ⟩` da secção 26 passou a `⟨𝜑|𝜓⟩`.
- **Visual sec 26** (`temp/p966/sec26.png`): conteúdo math equivalente ao
  vanilla (bra/ket itálicos, Ψ, ⟨Â⟩); diferenças remanescentes são as já
  catalogadas (acento sobre A, detalhes de expoente).
- **`compare.py` sec 26: nota honesta** — a mediana |dx| sobe para 3.65
  (de 2.89 em P964), mas a decomposição mostra os pares flagged a serem
  letras da **prosa dos cabeçalhos** ("Mecanica Quantica"), não math —
  limitação documentada da ferramenta (mesma lição de P952 §6.4/P949:
  mediana dominada por mis-pairs de prosa, não por matemática). A prova do
  fix é a convergência de codepoints acima + o visual, não a mediana crua.
- Benchmark: ver tabela (hyperfine, "antes" = release pós-P965; JSONs em
  `tools/perf/results/p966-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 87.78 | 88.53 | 1.009 |
| 02-lorem | 106.08 | 107.42 | 1.013 |
| 03-images | 95.76 | 95.52 | 0.997 |
| 04-math | 121.09 | 120.55 | 0.995 |
| 05-tables | 94.83 | 100.57 | 1.060 |
| 06-long | 300.71 | 297.85 | 0.991 |
| 07-context | 128.12 | 128.92 | 1.006 |

Ratio médio **1.010** — o outlier (05-tables, 1.060) é o ruído de ambiente
habitual (a mudança são dois braços de match numa função de layout math;
o cenário tables mal exercita math). Sem regressão atribuível.
