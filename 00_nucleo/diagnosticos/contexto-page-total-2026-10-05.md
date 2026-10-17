# Total/final de Page — auditoria e proposta L0, 2026-10-05

O problema não é apenas preencher um total físico. A adaptação atual combina página física com o último update causal de modo que updates posteriores alteram leituras anteriores. O final global também atravessa documentos em bundle. A proposta corrige o snapshot completo de Page por ocorrência e documento; nenhuma implementação produtiva nova foi escrita nesta etapa.

## Medição antes da decisão

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. `contexto-page-total-2026-10-05/measurement.json.gz` contém timestamps, lista exata de alterações (`git diff HEAD --stat`), status, fontes produtivas/L0, programa de reprodução, comandos, stderr, saídas completas e hashes de cada PDF/HTML. CLI Tekt é a da correção anterior, SHA-256 `8b228c47f5a6294fc057c238e9b87ea712fd9e91c104497188efbf4238baeb01`; vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

| Controle | Tekt | Vanilla |
|---|---|---|
| Uma página, both/final | B1/0 F0 | B1/1 F1 |
| Duas páginas, both/final | 1/0 e 0 | 1/2 e 2 |
| Início lógico 7, duas páginas | 7/7 e final 7 | 7/8 e final 8 |
| Update 7 na segunda página | 7/7 e 8/7 | 1/7 e 7/7 |
| Step após a leitura na única página | B1/1 F1 | B1/2 F2 |
| Bundle, primeiro PDF de uma página | 1/7 | 1/1 |
| Bundle, segundo PDF iniciado em 7, duas páginas | final 7 | final 8 |
| Bundle misto, HTML com update 7 | 1 | 7 |
| Bundle misto, último PDF de duas páginas | 1/1 | 1/2 |

Não há números de desempenho neste estudo. São observáveis da linguagem extraídos dos resultados das CLIs. O corpo “Second” nas páginas finais é preservado nos recibos.

## Causa e proposta

`03_infra/src/pipeline/paged.rs:360–365` toma CounterRegistry::final como início lógico de todas as páginas; `:840–913` soma o índice físico a esse início. `context_stabilization.rs:969–1010` usa o último update por body de maneira equivalente. O registry causal não inclui os avanços físicos, e sua leitura final global não delimita o documento pertinente.

No vanilla, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254` restringe Page ao documento; `:730–773` dobra updates efetivos com o contador manual; `:934–963` combina estado inicial, updates e avanços físicos. Essa medição refuta a solução de usar apenas a última página lógica armazenada: ela não representa a leitura atual antes de updates tardios nem final após um step tardio.

Proposta L0: duas vistas completas no TagIntrospector, current e final por Location, produzidas conjuntamente pelo fold puro Page a partir da contribuição, ordem causal, posição física e partição por documento. HTML não herda páginas de PDFs vizinhos. Readers get/at/display/final/both e replay consultam a mesma geração. Callbacks Page recebem o estado que inclui avanços físicos, sem repetir callbacks das outras chaves ou duplicar sinks. Ciclos e budget de estabilização permanecem.

A correção é mais ampla que total/final porque a medição demonstrou que o current existente também precisa desse fold. Somar pages.len ao registro global ou acrescentar mais uma passagem não é uma correção suficiente.

## Especificação pronta para revisão

O conjunto L0 salvo está enumerado em `contexto-page-total-2026-10-05/l0-manifest.json`, SHA-256 **`1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`**. O manifesto contém o SHA completo de cada owner e do núcleo; `proposal-state.json.gz` preserva conteúdos e estado exato da proposta.

Contratos públicos propostos:

- `00_nucleo/prompts/entities/introspector.md`, SHA-256 `d7eaa643ebde11a5c085950134fa9e42544d299cd6062d516cca7bcd3e7648b8`: campos `page_counter_values` e `page_counter_final_values`, ambos `HashMap<Location, Vec<usize>>`. Ausência difere de estado vazio. Não se modifica o trait Introspector.
- `00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md`, SHA-256 `676132b09a787cdf4b222c8297972cffd59f8c2322936646ad7a0b59d0389930`: fachada pura realize_page_counter_snapshot, com tags, ocorrências por documento e total físico opcional (HTML), introspector mutável e Engine/observer injetados. Esse owner executa o fold e conserva os erros/callbacks originais.

Os demais owners de leitura, composição e testes consomem o Núcleo Tekt `00_nucleo/prompts/_nuclei/page-counter-snapshot.toml` com pin completo. Cada prompt continua proprietário do seu único consumer. Não se ressellaram headers nem Hash do Código para fingir implementação dessa proposta. O código e binário produtivos continuam iguais aos da etapa anterior.

## Gate e aceitação após ratificação

A ADR-0127 exige “PARAGEM OBRIGATÓRIA antes de escrever código (confirmar com o dono)” para contrato público e mudança de fase/ordem. A proposta acrescenta campos públicos e uma fachada; o callback Page passa a considerar o snapshot físico realizado. Portanto requer confirmação específica deste L0, não apenas autorização geral para continuar.

Após ratificação: testes RED reproduzindo os controles acima, callbacks Func, vetores vazios/multiníveis, páginas sem ocorrências finais, standalone HTML e erros/recovery; fold puro e integração com posições/documentos sob o budget existente; atualização dos L0 de qualquer outro owner antes de seu código; GREEN, build e lint; comparação CLI com pin ratificado. Medir retenção das vistas e garantir invalidação conjunta. Não declarar a frente encerrada enquanto current/final/replay ou contexto HTML/bundle divergirem. Não inserir cache global, unsafe ou reavaliação do arquivo inteiro.

## Materialização após confirmação

O humano confirmou o conjunto e a implementação foi realizada. A proposta histórica acima permanece como registro do estado submetido; resultado, medição, retenção e limites estão em [contexto-page-snapshot-2026-10-05.md](contexto-page-snapshot-2026-10-05.md).
