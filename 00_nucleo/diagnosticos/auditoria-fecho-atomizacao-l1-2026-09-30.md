# Auditoria de encerramento da atomização L1 — 2026-09-30

## Proveniência e critério

Medição da árvore limpa no commit `f59e0b9a9d8097740ed4e93e24d0638633753d92`, em 2026-10-01 01:53 UTC (2026-09-30 22:53 America/Sao_Paulo). Inventário: `rg --files 01_core/src -g '*.rs' | xargs wc -l | sort -nr`; inspeção de declarações e trechos indicados abaixo. As contagens são linhas físicas, não linhas produtivas nem estimativas de desempenho.

Um corte entra na lista **agora** somente se: (1) lógica de produção de uma responsabilidade identificável ainda reside com outra no mesmo arquivo; (2) o novo arquivo permite examinar ou trocar uma política sem percorrer o módulo inteiro; (3) o owner fica na mesma camada, preservando contratos públicos, despacho estático e semântica; (4) há fronteira concreta para o L0 1:1. Tamanho isolado, volume de testes, uma tabela exaustiva única e a mera possibilidade de criar outro arquivo não bastam. Esta é uma decisão de pesquisa e manutenção, sem previsão de ganho de tempo de execução.

## Decisão: quatro cortes finais com utilidade atual

1. **Leituras de contador filtrado e replay contextual.** Em `01_core/src/compiler/stdlib/counter.rs:317-632`, `is_filtered_key`, resolução de localização, `filtered_history`, métodos `*_owned` e `replay_context_read` formam a política de leitura/reexecução filtrada. Separar em `compiler/stdlib/counter/filtered_reads.rs`, com o mesmo contrato de observação e ordem causal. O construtor e o despacho público continuam na fachada. Esta unidade concentra a pergunta algorítmica: como obter o histórico filtrado e reproduzir as leituras sem alterar os observáveis.
2. **Exibição de contador.** Em `01_core/src/compiler/stdlib/counter.rs:666-845`, `counter_display` reúne seleção de localização, histórico, opções, padrão/callback e emissão de conteúdo; o wrapper nativo está em `:943`. Separar essa política em `compiler/stdlib/counter/display.rs`, mantendo as assinaturas e o ordenamento de erros. A questão independente é a construção do resultado visível de `.display()`; não dividir cada validação em um módulo.
3. **Convergência das tentativas de layout.** Em `01_core/src/compiler/layout/mod.rs:1859-2035`, a entrada `layout_with_introspector_and_metrics_and_callbacks_from` prepara cache/callbacks, cria tentativas, transporta mapas de páginas e decide `Complete`/`Pending` até o limite de cinco. Separar a coordenação em `compiler/layout/convergence.rs`, mantendo as entradas públicas na fachada e o `match Content` em `mod.rs`. Esta é uma unidade algoritmicamente pesquisável para critérios de convergência e custo de novas tentativas.
4. **Formatação bibliográfica de fallback.** Em `01_core/src/compiler/layout/mod.rs:1631-1698`, `format_bib_entry_body` e a composição condicional dos campos pertencem à feature bibliografia já presente em `compiler/layout/bibliography.rs:25`, que chama o primeiro helper em `:52`. Mover os helpers usados para essa unidade, sem alterar texto ou ordem dos campos. `format_bib_entry` em `mod.rs:1637` não possui chamada produtiva na árvore L1 atual; verificar sua remoção em vez de transportá-lo automaticamente. O L0 atual de `compiler/layout.md` declara expressamente o formatter no hub e `compiler/layout/bibliography.md` manda usá-lo dali; atualizar **ambos antes do código**, preservando ownership 1:1. Não confundir este fallback com a interpretação CSL de `bib_csl.rs`.

Os quatro itens são *unidades de corte*, não autorização para novos algoritmos ou para alterar comportamento. Cada um exige L0 atualizado primeiro, prova de comportamento preservado, build e lint. Nenhum dos quatro exige, pelo escopo descrito, novo contrato público, default ou fase de pipeline; reclassificar se a implementação revelar isso (ADR-0127).

## Fora da rodada

- `entities/layout_types.rs` tem 1.976 linhas, mas a produção até `:1345` é principalmente declarações de tipos públicos (geometria, estilo, frame, página e medidas). Subdividir agora criaria muitas reexportações e novos owners L0 sem uma política concorrente identificada. Não contar como corte atual.
- `compiler/eval/observation/value.rs` tem 1.823 linhas; `SyntaxKind` ocupa `:840-1381`, mas é uma única tabela exaustiva de relação por variante. Movê-la isoladamente troca o endereço de um bloco, sem separar algoritmos. Os demais `impl ObservationEq` já estão agrupados pelo contrato da relação; rever apenas se uma mudança concreta desse contrato exigir nova família.
- `entities/style_chain.rs:293-825` reúne cadeia, resolução e ponte `TextStyle`, porém os métodos de leitura operam sobre a mesma representação; a ponte começa em `:757` e é pequena. O custo de separar agora supera a redução de busca.
- `compiler/introspect.rs:139`, `compiler/stdlib/calc.rs:224`, `compiler/stdlib/loading.rs:83` e `compiler/eval/bindings/field_access.rs:33` iniciam blocos de testes depois das fachadas ou de módulos filhos; o tamanho total desses arquivos não representa monólitos produtivos comparáveis.
- `compiler/layout/mod.rs:1135-1610` mantém o dispatcher estático e exaustivo conforme ADR-0109. Não quebrar o `match` por contagem de linhas ou de comentários.
- Os grandes `compiler/layout/tests.rs`, `compiler/eval/tests.rs` e `compiler/math/layout/tests.rs` são suites de testes; sua organização é assunto separado da atomização produtiva.

## Regra de parada

Depois destes quatro cortes, encerrar a atomização geral do L1. Um corte adicional só entra por uma nova evidência concreta: investigação de algoritmo ou defeito que mostre duas políticas independentes ainda misturadas e um ganho de leitura verificável. Não reabrir a lista apenas porque resta um arquivo grande.
