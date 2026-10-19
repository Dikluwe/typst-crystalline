# Passo 1375 — `block(height: fr)` sob estilo transparente

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Fechar o primeiro scope-out do P1374: reconhecer um único Block fracionário
através do invólucro estrutural que `#text(...)[...]` produz, sem ampliar a
distribuição para colunas ou múltiplas regiões.

## Medição anterior à decisão

Sonda `p1375-wrapper.typ`, página 200pt × 240pt, margem 10pt, um Block 1fr sob
`#text(fill: red)`, um Block 2fr direto e `After`:

- vanila ratificado: 1 página; `One=7,404`, `Two=82,724670`,
  `After=220,166`;
- cristalino P1374: 2 páginas; `One=7,404`, `Two=42,230` e `After=7,404`
  na página 2.

Sonda identificador administrativo `a963fbc8…43e8`; vanila `eb60986b…1468f`; cristalino
`3f95f3b1…3148`; `pdftotext` `0fb98ea1…565e7`. Medição em
2026-09-13T15:16:19-03:00, HEAD `8ceea20f…5248`, working tree não commitado
com 45 ficheiros alterados no diff e 238 entradas de status.

## Decisão

1. O owner `sequence.rs` reconhece um caminho estrutural unitário composto por
   `Styled` e `Sequence`; `Space` e `Empty` são transparentes.
2. O caminho só é elegível quando termina em exatamente um Block com altura
   fracionária. Zero ou dois filhos substantivos falham fechado.
3. O mesmo reconhecimento alimenta a soma, o desconto dos cosméticos e a
   atribuição da parcela; wrappers não medem o body como cauda fixa.
4. Quando uma Sequence transparente interna não cria distribuição própria,
   conserva a parcela herdada até o Block terminal; depois restaura o estado.
5. Nenhuma API pública muda. Colunas, multi-região, Strong/Emph e frações
   aninhadas permanecem `Unknown` e formam a fila dos próximos passos.

## RED → GREEN e ataques

- RED estrutural: Styled unitário não resolve para o Block; Styled com dois
  filhos não pode resolver.
- GREEN focal: helper e integração de Sequence passam.
- matriz bilateral: páginas e posições congeladas coincidem no nível da
  linguagem; diferenças de bytes não recebem crédito.
- mutações: tratar Styled como fixo, contar o body duas vezes, aceitar dois
  filhos, perder a parcela na Sequence interna ou vazá-la ao sibling devem ser
  discriminadas.

## Memória

Execução estritamente serial. `cargo check` limitado a 3 GiB; link focal com
debug desligado sob 4 GiB e arquivo de 512 MiB. Matrizes usam um subprocesso
por vez, 2 GiB, arquivo 16 MiB e timeout 45 s. Nenhuma suíte ampla nem retry
do linker com símbolos completos.

## Gate ADR-0127

Correção interna de paridade, sem contrato público, comportamento padrão novo
ou mudança de fase. Fluxo contínuo autorizado: L0 atualizado antes do código,
RED comprovado e resselo ao final.
