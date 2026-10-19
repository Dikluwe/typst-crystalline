# Prompt — typst-passo-813: equação em bloco não centrada + espaçamento vertical apertado (achado #16, de P808)

**Origem**: achado novo registado por P808 (varredura da regra revogada do Passo 48), nunca corrigido até agora — o caminho de bloco nunca foi tocado por P800 (que só cobriu inline)
**Estado**: aguardando execução

---

## Achado (medição literal de P808)

Documento `Antes $ x^2 $ Depois` (equação em modo bloco), `mutool trace`:

| Elemento | Cristalino | Vanilla |
|---|---|---|
| "Antes" baseline | 78.105 | 78.104 ✓ |
| math `x` baseline | 92.493 (+14.4) | 100.410 (+22.3) |
| "Depois" baseline | 104.736 (+12.2) | 120.969 (+20.6) |
| math `x` horizontal | 70.867 (margem esq.) | 291.993 (**centrado**) |

Dois problemas confirmados: (1) a equação em bloco não é centrada horizontalmente na página/coluna; (2) o espaçamento vertical antes/depois da equação é mais apertado do que o vanilla.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Reproduzir exactamente o documento de P808 (`Antes $ x^2 $ Depois`) com os dois binários, `mutool trace`, confirmar os números acima antes de mexer em código (a medição pode ter mudado depois de P809/P811/P812 terem tocado layout matemático — reconfirmar, não assumir que continua igual).
2. Localizar no vanilla (`lab/typst-original/`) o layout de equação em modo bloco (`typst-layout/src/math/...` ou equivalente) — como calcula centragem horizontal (provavelmente centra em relação à largura da região/coluna disponível) e o espaçamento vertical (provavelmente usa `above`/`below` de `#set math.equation` ou constantes de `ParElem`/block spacing).
3. Localizar no cristalino o caminho equivalente e identificar por que a centragem não acontece (ex.: o bloco é layout como um `Frame` alinhado à esquerda por defeito, sem `Content::Align` a envolver) e por que o espaçamento é mais apertado (comparar os valores exactos de espaçamento usados).
4. Registar os dois pontos antes de tocar em código.

## Passo 2 — Implementação

Corrigir a centragem horizontal (envolver o layout da equação em bloco com o mecanismo de alinhamento já usado no resto do projecto, `Content::Align`, mesma família mencionada no handoff antigo P763–P771) e ajustar as constantes/lógica de espaçamento vertical para bater com o vanilla.

## Passo 3 — Validação

1. Recompilar. Repetir o comando do Passo 1, mostrar `mutool trace` com os quatro números da tabela acima, agora batendo com o vanilla.
2. Testar também um caso com equação mais larga que a coluna (se aplicável) e um caso com `#set math.equation(numbering: ...)` para confirmar que a centragem não quebra a numeração de equação, se essa feature já existir no cristalino.
3. Testes novos cobrindo centragem e espaçamento.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-813-relatorio.md` com: medição antes, código vanilla/cristalino identificados, diff, medição depois, contagem de testes.
