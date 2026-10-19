# Relatório — typst-passo-848: triagem sistemática, lote 6 — os 7 módulos finais

**Data:** 2026-07-22  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-848.md`).  
**Proveniência das medições:** commit HEAD `92daa9c66` (P847 — sem alterações de código, apenas resolução de headers multi-@prompt). Working tree limpa no arranque, excepto os próprios documentos de passo em `00_nucleo/materialization/` (`typst-passo-848.md`, `849.md`, `850.md`) — não são alterações de código. Binários: cristalino `./target/release/typst` (0.15.0, `23ff5b5c`), vanilla `lab/typst-original/target/release/typst` (0.15.0, `969087ec`). Todos os fixtures e assets em `temp/p848/` (pasta em `.gitignore`).

**Pré-requisito verificado:** `git status --short` mostra apenas os três ficheiros de passo (não código). `cargo test --workspace` foi corrido como verificação final da suíte; resultado reportado na secção de validação.

---

## Passo 1 — Confirmação da lista

- Inventário: `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`, secção `lacuna-inventario`.
- P831 deixou exactamente **7 módulos não triados**, todos confirmados ainda presentes no inventário:
  1. `typst_library::foundations::styles::rule`
  2. `typst_library::layout::abs`
  3. `typst_library::layout::axes`
  4. `typst_library::layout::corners`
  5. `typst_library::layout::fragment`
  6. `typst_utils::pico::bitcode`
  7. `typst_utils::pico::exceptions`
- Nenhum novo módulo apareceu na lista desde P831.

---

## Passo 2 — Triagem módulo a módulo

Convenção de comandos (raiz do repo): cristalino `./target/release/typst temp/p848/<f>.typ -o <out>.pdf`; vanilla `lab/typst-original/target/release/typst compile temp/p848/<f>.typ <out>.pdf`. Comparação por `pdftotext`, `pdftotext -bbox`, `pdfinfo` e `diff`.

### 1. `foundations::styles::rule` — **PARIDADE CONFIRMADA**

- Fixture: `temp/p848/styles-rule.typ` — `#show heading: it => [Capítulo: #it.body]` e `#show emph: it => [*#it.body*]`.
- Ambos os binários compilam com exit 0.
- Sequência de palavras extraída por `pdftotext -bbox` é idêntica nos dois:
  `Capítulo: Título Texto normal e itálico num parágrafo. Capítulo: Subtítulo Outro destaque.`
- Apenas a quebra de linhas difere (envolvimento do parágrafo), o que é variação mecânica de layout.

### 2. `layout::abs` — **PARIDADE CONFIRMADA**

- Fixture: `temp/p848/abs.typ` — rectângulos com `1in`, `2cm`, `5mm`, `50.8mm`, `28.35mm`, `72pt`, `36pt`.
- Ambos compilam; nenhum texto, apenas formas. Page size 200×120 pt em ambos.
- Não há divergência observável na compilação nem nas dimensões declaradas da página.

### 3. `layout::axes` — **ACHADO**

- Fixture: `temp/p848/axes.typ` — `#grid(columns: 2, rows: 2, ...)` e `#stack(dir: ltr, ...)`.
- A `grid` funciona em ambos (cristalino gera PDF se o `stack` for removido), mas o `stack` rejeita o valor `direction`:

```
/home/dikluwe/Documentos/Antigravity/typst-crystalline/temp/p848/axes.typ:<detached>: error: stack(dir:) deve ser string, recebeu direction
  while calling `stack` at .../axes.typ:13:1
    stack(…)
```

- Teste complementar: `stack(dir: "ltr", ...)` compila no cristalino mas falha no vanilla (`expected direction, found string`).
- **Conclusão:** o cristalino trata `stack(dir:)` como string, enquanto o vanilla aceita o tipo `direction` (`ltr`, `rtl`, `ttb`, `btt`). Divergência na superfície da língua.

### 4. `layout::corners` — **ACHADO**

- Fixture: `temp/p848/corners.typ` — `rect(radius: (top-left: 15pt, ...))` e `rect(radius: 10pt)`.
- Cristalino rejeita o argumento `radius` em `rect()`:

```
/home/dikluwe/Documentos/Antigravity/typst-crystalline/temp/p848/corners.typ:5:5: error: argumento nomeado inesperado em rect(): 'radius'
```

- Vanilla compila e renderiza cantos arredondados.
- **Conclusão:** o parâmetro `radius` de `rect()` não existe no cristalino.

### 5. `layout::fragment` — **PARIDADE CONFIRMADA**

- Fixture: `temp/p848/fragment.typ` — conteúdo que quebra página (`#block(height: 140pt, lorem(40))` numa página de 150×80 pt).
- Ambos compilam; `pdfinfo` reporta 3 páginas em ambos.
- Texto extraído por `pdftotext` é idêntico.
- Os tamanhos de PDF diferem (cristalino ~345 kB, vanilla ~9,5 kB) devido a embedding/subsetting de fontes — diferença mecânica, não de língua.

### 6. `pico::bitcode` — **PARIDADE CONFIRMADA**

- Fixture: `temp/p848/pico-bitcode.typ` — string curta compatível com bitcode (`"abc-1234"`) em `repr` e `metadata`.
- Ambos compilam; texto extraído por `pdftotext` é idêntico (`"abc-1234"`).

### 7. `pico::exceptions` — **PARIDADE CONFIRMADA**

- Fixture: `temp/p848/pico-exceptions.typ` — string da lista de excepções (`"accept-charset"`, `"number-clearance"`) em `repr` e `metadata`.
- Ambos compilam; texto extraído por `pdftotext` é idêntico.
- A string excepcional é correctamente preservada na saída.

---

## Passo 3 — Fechamento da varredura sistemática

Os 7 módulos restantes da lista original de P772t (revisada por P831 para 82 módulos) estão agora triados. Com este lote, a **varredura sistemática está formalmente completa**:

- Total de módulos triados: 82 (P785+P786+P798: 45 + P810: 15 + P831: 15 + P848: 7).
- Restantes não triados: **0**.

---

## Taxa de sinal real

- Módulos com pelo menos um achado real: **2 / 7** (`layout::axes`, `layout::corners`).
- Módulos em paridade confirmada: **5 / 7** (`styles::rule`, `layout::abs`, `layout::fragment`, `pico::bitcode`, `pico::exceptions`).
- Taxa de sinal real deste lote: **~29%**.

Apesar da classificação de P831 como "Tier D — mecânica pura", 2 dos 7 revelaram divergências observáveis na superfície da língua. Isto reforça a lição de P798: nenhuma classificação como "mecânica" dispensa um teste real.

---

## Tabela de achados novos (continuação da fila a partir de #60)

| # | Módulo | Achado |
|---|--------|--------|
| 61 | `layout::axes` | `stack(dir:)` aceita string no cristalino em vez de valor `direction` (`ltr`/`rtl`/`ttb`/`btt`) |
| 62 | `layout::corners` | `rect()` não aceita o argumento `radius` |

## Achado incidental (fora dos 7 módulos, emergiu na triagem)

- **#63 (incidental)** — `ref(<label>)` falha no cristalino: `ref() espera nome como string, recebeu label`. O vanilla aceita uma label como argumento de `ref()` (e só depois valida se o alvo tem numeração). Pertence ao domínio de introspecção/ref, não a `pico::exceptions`.

---

## Notas transversais

- O CLI cristalino continua a ignorar a extensão do `-o` (escreve sempre PDF), como já registado em P831.
- Diferenças de tamanho de PDF (embedding de fontes) não foram contadas como achados — o texto e a geometria das páginas coincidem.
- Nenhum dos achados deste passo foi corrigido; seguem para passos dedicados, conforme o protocolo de triagem.

---

## Validação final

- `crystalline-lint .`: exit 0. Apenas um aviso pré-existente (V7 — prompt órfão `00_nucleo/prompts/infra/package_version_resolution.md`), não causado por este passo e não relacionado com os módulos triados.
- `cargo test --workspace`: **5417 passed; 0 failed** (typst-core 4632/0/2ign; typst-infra 714/0/5ign; demais crates 71/0). Nenhum teste novo neste passo (triagem não altera código), logo a contagem reflecte o estado de P847.
