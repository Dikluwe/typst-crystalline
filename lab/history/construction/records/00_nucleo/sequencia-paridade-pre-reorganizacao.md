# Sequência de paridade — antes da reorganização

| Campo | Valor |
|-------|-------|
| Data | 2026-07-02 |
| Base | Relatório de P531 (sondagem-vanilla-p531.md) |
| Objectivo | Ordenar os itens de alto impacto encontrados em P531, antes de começar a reorganização do projecto |

---

## Nota sobre uma afirmação a corrigir

O handoff regista P515 como "fontdb + font fallback: FECHADO — font fallback por carácter". O relatório de P531, Grupo 3.3, mostra que isto não está correcto: um documento com latim, CJK, emoji, e árabe na mesma linha perde os caracteres não-latinos (`pdftotext` mostra `Hello ???? ? ?????`). O fallback existe, mas troca sempre para uma única fonte default, não escolhe fonte por carácter como o vanilla faz.

Isto entra na mesma categoria de casos já registados no projecto: uma linha do handoff escrita como fechada que, sondada de novo, não está. Corrijo isto no primeiro passo da sequência abaixo, antes de decidir a dimensão real do trabalho.

---

## Os seis itens de alto impacto, por ordem

A ordem segue três critérios, por esta prioridade: (1) resultado errado é pior que resultado ausente — um documento que mostra `?` no lugar de um nome próprio é pior que um documento sem numeração de página; (2) passos pequenos e isolados primeiro, para confirmar que o método continua a funcionar sem regressão antes de tocar em código maior; (3) passos que tocam o shaper (área mais sensível do projecto, trabalhada em P515–P530) ficam depois dos que não tocam.

| Ordem | Passo | Item | Porquê nesta posição |
|-------|-------|------|----------------------|
| 1 | P532 | Numeração de página customizada | Isolado, pequeno, propriedade já reconhecida, só falta aplicar no render. Bom primeiro passo — confiança antes do resto. |
| 2 | P533 | Citações bibliográficas (`@key1` não resolve, formatação com erros de pontuação) | Produz texto errado no documento ("See ." em vez de "See [1]."), não só ausência. Prioridade por gravidade do resultado. |
| 3 | P534 | Fallback de fonte por carácter (multi-script/emoji) | O maior dos seis. Toca o mesmo código do shaper mexido em P515–P530. Precisa de sonda própria antes de qualquer especificação, incluindo a correcção da linha do handoff. |
| 4 | P535 | Bookmarks PDF (`/Outlines`) | Isolado, não toca shaper nem layout de texto. Só exportação. |
| 5 | P536 | Metadados XMP (`#set document(title:, author:)`) | Isolado, pequeno, só exportação. |
| 6 | P537 | Notas de rodapé em layout de duas colunas | Bug de posicionamento, contido ao caso de multi-coluna. Não afecta o caso comum (uma coluna), por isso fica por último entre os seis. |

Depois de P537, um passo curto (P538) repete a sondagem de P531 só para os seis itens, confirma que ficaram fechados, e só aí a reorganização (P539+) começa.

---

## O que fica para depois da reorganização, com débito registado

Do relatório de P531, itens classificados como baixo impacto para a maioria dos documentos:

- HTML/SVG/PNG export
- IDE/LSP
- PDF Tagged/PDF-UA (acessibilidade)
- Compressão por object streams
- Fontes Type1/PostScript
- Escrita vertical CJK
- Pacotes (`@preview`)

Estes não entram na sequência agora. Ficam registados como trabalho pendente, visível, não escondido pela reorganização.

---

## Critério de fecho deste documento

- [ ] Seis passos (P532–P537) especificados um de cada vez, cada um com sonda própria antes de qualquer código, seguindo o método já usado no projecto.
- [ ] P538 confirma o fecho dos seis.
- [ ] Só então a reorganização (P539+) começa.

O ficheiro seguinte (`typst-passo-532.md`) já está pronto para o primeiro item.
