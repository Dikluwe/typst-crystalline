# Passo 1050 — `table`/`grid`: inset de célula ausente + altura de linha divergente

**Tipo**: Investigar → gate (`ADR-0127`, categoria 2/3 — muda output visual em qualquer
tabela sem `inset` explícito, caso mais comum) → corrigir.
**Achado**: durante a verificação empírica do Passo 1049 (caso #4, `layout/grid.rs:663`),
descoberto por acidente ao pedir coordenadas reais em vez de aceitar "paridade textual"
como prova suficiente.

**Medição** (`#table(columns: (1fr, 1fr, 1fr), stroke: 0.5pt)`, 3×3, página 250pt,
margem 10pt):

| | Vanilla | Cristalino | Delta |
|---|---|---|---|
| `Δx` de todas as 9 células (posição horizontal do texto vs borda da coluna) | +5.00pt (inset aplicado) | +0.00pt (flush na borda) | **-5.00pt constante** |
| Altura de linha (linha a linha) | 17.24pt constante | 14.38-14.39pt constante | **-2.86pt por linha** |
| `Δy` acumulado (linha 1→2→3) | — | — | -11.07 / -13.91 / -16.77pt (cresce por acumulação da diferença de altura de linha) |
| `Δw` do texto dentro da célula | — | — | 0.0000pt (só a largura do glifo bate, não a posição) |

**Pré-condição**: `git status` limpo.

---

## Fase A — Confirmar o inset horizontal em falta

1. Ler `layout/grid.rs` — confirmar se `inset` (default `5pt` per o vanilla) é lido do
   elemento `table`/`grid` e aplicado ao conteúdo da célula, ou se é ignorado/tratado como
   `0pt` por defeito.
2. Confirmar o default real do vanilla por `file:line` (`typst_library::layout::grid` ou
   equivalente) — não presumir "5pt" só pela medição, confirmar na fonte.
3. Testar `#table(inset: 5pt, ...)` **explícito** — se, com o valor definido
   explicitamente, o cristalino aplica correctamente, o problema é só o **default**
   (mesma classe do P1034 — bibliography/figure/língua). Se mesmo explícito não aplicar,
   o problema é mais fundo (o campo nunca chega ao layout).

## Fase B — Investigar a diferença de altura de linha (2.86pt/linha)

1. Localizar onde a altura de linha é calculada em `layout/grid.rs` — é derivada do
   conteúdo da célula (altura de texto + inset vertical), ou de outro mecanismo?
2. Hipótese a confirmar, não presumir: se o inset vertical também estiver em falta (mesmo
   defeito do Mecanismo A, agora no eixo Y), isso explicaria parte da diferença — inset
   vertical de 5pt "em cima" + 5pt "em baixo" por célula seria 10pt por linha, mas a
   diferença medida é só 2.86pt. **Não presumir que é só o inset vertical** — pode haver
   uma segunda causa (altura de linha do texto em si, `row-gap`, ou `stroke` a contar
   para a altura de forma diferente). Medir isoladamente antes de decidir.
3. Testar uma tabela de 1 linha só (sem acumulação) para isolar a altura de UMA linha
   sozinha, sem o efeito de soma ao longo de várias — mais fácil de decompor a causa.

## Fase C — Critérios de verificação

```
Dado #table(columns: (1fr, 1fr, 1fr), stroke: 0.5pt) sem inset explícito
Quando renderizado
Então cada célula tem inset horizontal e vertical batendo com o default vanilla
  (confirmar valor exacto na Fase A, não presumir 5pt/5pt)

Dado a mesma tabela, com múltiplas linhas
Quando renderizado
Então altura de linha bate com vanilla, sem acumulação de erro — Δy de qualquer linha
  N deve ser igual ao Δy da linha 1, não crescente

Dado #table(inset: 5pt, ...) explícito
Quando renderizado
Então continua a bater (guarda de não-regressão — não presumir que só o caminho default
  estava errado)
```

Não-regressão: todos os testes de `table`/`grid` existentes.

## Fase D — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque visual contra o corpus canónico — qualquer documento com tabela muda de
aparência (correctamente). Dado que `table`/`grid` são extremamente comuns em documentos
reais, este é um dos achados de maior alcance desta frente — tratar com prioridade alta.

---

## Resultado esperado

Inset de célula (horizontal e vertical) aplicado por defeito, batendo com vanilla. Altura
de linha sem acumulação de erro. Corrigido para qualquer tabela, não só o caso de teste
específico.
