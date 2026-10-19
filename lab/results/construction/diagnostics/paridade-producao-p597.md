# Paridade de Produção — P597

**Data do relatório:** 2026-07-07
**Passo:** 597
**Foco:** Cristalino força duas colunas com conteúdo curto sob `height: 200pt`; vanilla não.

---

## Resumo executivo

A diferença observada em P596 (`#set page(columns: 2, height: 200pt)` + `#lorem(30)`) não é causada pelo algoritmo de colunas desenhar colunas vazias. O algoritmo de colunas do cristalino só utiliza a segunda coluna quando o conteúdo transborda da primeira. A causa real é que a **margem padrão do cristalino é fixa** (`70.87 pt`), enquanto o vanilla ajusta as margens automaticamente em função do tamanho da página. Numha página de `200 pt` de altura, o vanilla usa uma margem muito menor (`≈ 23.81 pt`), o que deixa as colunas mais largas e o texto curto cabe numa só coluna. No cristalino, as colunas são mais estreitas e o mesmo texto transborda mais cedo.

**Decisão:** não corrigir código neste passo. Alterar o cálculo de margens padrão é uma mudança arquitetural que requer um novo Prompt L0 para `PageConfig`/`layout_types`. O problema original de P595 (overflow de notas de rodapé) já foi resolvido em P595. P597 fica registado como disparidade consciente.

---

## Proveniência das medições

- **Hash do commit:** `0aeada0869603119d2a0fe0bfc5416210495016d`
- **Estado da working tree:** apenas ficheiros `00_nucleo/materialization/typst-passo-*.md` e `00_nucleo/diagnosticos/registo-fecho-p583-p584-p585.md` como untracked — nenhum deles afeta o binário compilado.
- **Data/hora da corrida:** 2026-07-07T19:54:25-03:00 (e medições subsequentes no mesmo dia)
- **Binários usados:**
  - Cristalino: `./target/release/typst`
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool draw`, `pdftotext -layout`, `pdfinfo`

---

## Sonda — alcance da diferença

Documento de teste:

```typst
#set page(columns: 2, height: 200pt)
#lorem($n)
```

Valores de `$n` testados: `5, 15, 30, 50, 80, 84, 85, 86, 87, 88, 89, 90, 95, 100, 120, 150, 200, 300, 500`.

### Resultados (número de páginas)

| `$n` | Páginas cristalino | Páginas vanilla |
|------|-------------------:|----------------:|
| 5    | 1                  | 1               |
| 15   | 1                  | 1               |
| 30   | 1                  | 1               |
| 50   | 2                  | 1               |
| 80   | 2                  | 1               |
| 100  | 3                  | 1               |
| 120  | 3                  | 1               |
| 150  | 4                  | 1               |
| 200  | 5                  | 2               |
| 300  | 7                  | 2               |
| 500  | 12                 | 3               |

### Visualização das colunas no vanilla

Extração de texto com `pdftotext -layout` mostra quando o vanilla começa a utilizar a segunda coluna visível:

| `$n` | Colunas visíveis no vanilla |
|------|----------------------------:|
| 5–87 | 1                           |
| 88+  | 2                           |

Exemplo de transição:

- `n=87` — uma coluna:
  ```text
  Lorem ipsum dolor sit amet, consectetur adipiscing elit,
  sed do eiusmod tempor incididunt ut labore et dolore
  ...
  ```

- `n=88` — duas colunas (primeira linha já contém texto na coluna direita):
  ```text
  Lorem ipsum dolor sit amet, consectetur adipiscing elit,     defensa et collaudata est, cum id, quod maxime placeat,
  sed do eiusmod tempor incididunt ut labore et dolore         facere.
  ...
  ```

Conclusão: **o vanilla começa a usar duas colunas visíveis a partir de `n ≈ 88`**; o cristalino já gera 2 páginas (i.e. usa a segunda coluna) a partir de `n = 50`.

### Dimensões medidas no cristalino

Saída de debug do layout de colunas (presente no binário de release usado):

```text
columns layout: page_width=595.28, usable_width=453.53999999999996,
                gutter=23.8112, column_width=214.8644, page_columns=true
```

Valores:

- Largura da página: `595.28 pt`
- Margem fixa: `70.87 pt` (≈ 2.5 cm)
- Largura útil: `595.28 − 2 × 70.87 = 453.54 pt`
- Gutter: `23.8112 pt`
- Largura de cada coluna: `214.8644 pt`

### Dimensões medidas no vanilla (n=100)

Extração estruturada via `mutool draw -F stext`:

- Largura da página: `595.2756 pt`
- Altura da página: `200 pt`
- Margem esquerda medida: `23.809525 pt`
- Bloco da coluna esquerda: `bbox="23.809525 18.672516 280.61558 177.74352"`

Valores:

- Margem estimada: `≈ 23.81 pt`
- Largura útil estimada: `595.28 − 2 × 23.81 ≈ 547.66 pt`
- Largura de cada coluna estimada (com gutter igual ao cristalino): `≈ 261.92 pt`

A margem vanilla (`≈ 23.81 pt`) corresponde aproximadamente a **11.9 % da menor dimensão da página** (`200 pt × 0.119 ≈ 23.8 pt`). Para A4 (`595.28 × 841.89 pt`), a mesma proporção dá `≈ 70.87 pt`, que é exatamente a margem fixa do cristalino. Em páginas pequenas, o vanilla reduz a margem; o cristalino mantém a margem de A4.

---

## Causa localizada no código

`01_core/src/entities/layout_types.rs:446-455`:

```rust
impl Default for PageConfig {
    fn default() -> Self {
        Self {
            width:     595.28, // A4 portrait
            height:    841.89, // A4 portrait
            margin:     70.87, // ≈ 2.5 cm
            numbering: None,
            columns:   None,
        }
    }
}
```

A `margin` é um valor fixo. Quando o utilizador faz `#set page(height: 200pt)`, a altura muda, mas a margem permanece `70.87 pt`. Isso reduz a área útil horizontal e, consequentemente, a largura de cada coluna, forçando o texto a transbordar mais cedo.

Não foi encontrada nenhuma lógica de margem automática proporcional ao tamanho da página nas camadas cristalinas consultadas.

---

## Decisão e justificação

**Não se corrige o código neste passo.**

Razões:

1. **Escopo:** P597 é uma sonda para confirmar o alcance da diferença, não para alterar a semântica de margens padrão.
2. **Arquitetura:** a alteração de `PageConfig::default` para margens automáticas é uma mudança arquitetural na configuração de página. Segundo o Protocolo de Nucleação, requer um Prompt L0 atualizado em `00_nucleo/prompts/entities/layout_types.md` (ou novo prompt dedicado) antes de qualquer modificação em L1.
3. **Paridade intencional vs. não intencional:** a margem fixa do cristalino reproduz corretamente o vanilla em A4 padrão. A divergência só aparece em páginas com dimensões não padrão. Decidir se o vanilla deve ser seguido neste ponto requer análise de L0, não apenas uma medição.
4. **Problema original resolvido:** P595 corrigiu o overflow de notas de rodapé. O comportamento de colunas duplas com texto curto é uma observação separada, não um bloqueio.

**Próximo passo recomendado (fora do escopo de P597):** atualizar o Prompt L0 de `layout_types` para especificar o cálculo de margens automáticas do vanilla (11.9 % da menor dimensão ou equivalente), depois implementar e testar.

---

## Critérios de fecho do passo

- [x] Alcance da diferença confirmado com vários tamanhos de conteúdo.
- [x] Ponto de transição do vanilla para duas colunas visíveis localizado (`n ≈ 88`).
- [x] Causa localizada com `file:line` (`01_core/src/entities/layout_types.rs:451`).
- [x] Decisão registada: não corrigir; requer novo Prompt L0.
- [x] Relatório escrito com hash do commit e proveniência das medições.

---

## Ligações

- `00_nucleo/materialization/typst-passo-597.md` — passo que originou esta sonda.
- `01_core/src/entities/layout_types.rs:446-455` — definição de `PageConfig::default`.
- ADR-0108 — *Disciplina anti-deriva: medir antes de decidir*.
- ADR-0107 — *Paridade é com a linguagem (semântica/sintaxe/morfologia), não com a mecânica/igualdade do Rust*.
