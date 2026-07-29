# Prompt — typst-passo-930: caracterizar o custo da `coverage` e o potencial do índice invertido

**Origem:** P927 eliminou a regressão no caso comum movendo a extração de `Coverage` para o caminho lazy. P928 mostrou que voltar à extração eager durante o arranque reintroduz regressão inaceitável. P929 descartou paralelização e cache em disco. Antes de alterar novamente o algoritmo de fallback, é necessário medir se existe oportunidade real para um índice invertido (`bloco Unicode -> fontes`).

**Estado:** aguardando execução.

---

## Contexto: o que já está confirmado, não precisa responder

* O `Face` já é aberto durante `font_info_from_bytes`; nomes, peso, estilo e restantes metadados já são extraídos.
* O único trabalho evitado por P880 foi a iteração da `cmap` (`extract_coverage`).
* Ainda não sabemos se a iteração da `cmap` é barata o suficiente para voltar ao arranque nem se um índice invertido reduziria significativamente o conjunto de candidatos.

---

## Regra da linha de trabalho (obrigatória)

Comando exacto + saída literal.

Toda instrumentação é temporária e deve ser removida no final.

Nenhuma alteração funcional permanente neste passo.

---

## Passo 1 — medir o custo isolado da extração de `Coverage`

Instrumentar temporariamente `font_info_from_bytes` para medir separadamente:

* `Face::parse`
* extração de metadados
* `extract_coverage`
* tempo total por fonte
* tempo agregado de `load_system_fonts`

Produzir tabela semelhante a:

| fase             | tempo |
| ---------------- | ----: |
| Face::parse      |   ... |
| metadata         |   ... |
| extract_coverage |   ... |
| total            |   ... |

Responder objetivamente:

> Quanto do tempo de descoberta seria acrescentado pela iteração da `cmap`?

---

## Passo 2 — medir a cardinalidade real dos candidatos

Sem alterar o algoritmo actual, instrumentar temporariamente `candidates_for_char`.

Quando o scan lazy terminar, produzir estatísticas por bloco Unicode:

* bloco
* número de fontes candidatas

Exemplo:

| bloco | candidatos |
| ----- | ---------: |
| U+03  |        ... |
| U+4E  |        ... |
| U+1F6 |        ... |

Apresentar também:

* média
* mediana
* p95
* máximo

Responder objetivamente:

> O índice invertido reduziria a procura para poucas fontes ou ainda deixaria dezenas/centenas de candidatas?

---

## Passo 3 — medir a distribuição da `Coverage`

Instrumentar temporariamente a extração de cobertura para registar:

* blocos distintos por fonte

Apresentar:

* média
* mediana
* p95
* máximo

Responder:

> A maioria das fontes cobre poucos blocos ou muitos?

---

## Passo 4 — benchmark

Executar exactamente a metodologia dos P927–P929.

Medir:

* 7 cenários canônicos
* 05-utf8
* utf8-latin
* utf8-greek
* utf8-cjk
* utf8-emoji

Critério de aceitação:

A instrumentação não pode alterar significativamente os resultados após ser removida.

---

## Passo 5 — decisão

Responder apenas:

1. Quanto custa realmente `extract_coverage`.
2. O índice invertido tem potencial elevado, moderado ou baixo.
3. Vale a pena implementar um protótipo no P931.

Não implementar o índice neste passo.

---

## Relatório

`00_nucleo/diagnosticos/typst-passo-930-relatorio.md`

Deve conter:

* metodologia;
* tempos detalhados;
* distribuição de cobertura;
* distribuição de candidatos;
* benchmark;
* conclusão fundamentada sobre a viabilidade do índice invertido.
