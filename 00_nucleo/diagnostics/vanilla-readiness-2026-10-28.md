# Tekt × vanilla: prontidão para usuários — 2026-10-28

**Já pode ser mostrado como protótipo em uma demonstração técnica guiada. Ainda não recomendo uma prévia aberta para uso geral nem apresentar o Tekt como substituto compatível do Typst.** A comparação nova encontrou problemas em documentos básicos, além dos limites especializados já conhecidos.

Referência: vanilla fixado `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, árvore `483bd6f1d8ac8810db47b65336d402647955fd17`, executável SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Tekt release `d726aef1e418e0aa848562c5bbd1d348bc368faae56477d9a2ad6f3b99a8e4be`.

## Comparação nova

33 fontes em PDF e SVG: 14 exemplos representativos e 19 arquivos matemáticos do corpus local derivado da documentação; seis controles mínimos adicionais. Todos têm fontes/hashes e logs preservados. Os dois pacotes foram testados em versões locais fixadas: oxifmt 1.0.0 e CeTZ 0.5.2. Não é uma amostra estatística nem porcentagem de cobertura global.

Dos 14 exemplos representativos, 11 compilam bilateralmente. Tabela com array de alinhamentos e template `#show: theme.with(...)` são rejeitados pelo Tekt, mas aceitos pelo vanilla; CeTZ atinge timeout de 60 s no Tekt em PDF e SVG. A importação isolada CeTZ também atinge o limite, enquanto vanilla conclui. A causa interna ainda precisa de investigação.

| Prioridade | Limite observado |
|---|---|
| 1 | Títulos numerados, sumário e blocos tabela/raw podem se sobrepor ao texto. Reproduzido sem JPEG e, no caso de títulos, sem sumário. |
| 2 | `#show: it => it` e templates sem seletor são rejeitados; impedem um padrão básico de uso. |
| 3 | Arrays de alinhamento em tabelas são rejeitados; figuras com legenda perdem centralização e alteram o avanço vertical, inclusive usando retângulo em vez de imagem. |
| 4 | Importação CeTZ não conclui no tempo observado. Exemplos HTML válidos no vanilla falham no Tekt. |
| 5 | Há diferenças de colunas/notas, localização/estilo bibliográfico e matemática pública. `math.class` chega a imprimir a chamada literal na página. |

[Comparação mínima dos títulos](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/png/control-heading-numbering-compare.png) · [Artigo completo](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/png/article-multipage-compare.png) · [Tabela/código](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/png/code-data-compare.png). Tekt à esquerda, vanilla à direita.

Entre os exemplos representativos bilaterais com palavras, relatório matemático e oxifmt passam texto/páginas/geometria a 0,001 pt e têm raster PDF idêntico nesta amostra; os pequenos resíduos SVG ficam registrados separadamente. O JPEG ruidoso, sem palavras, passa geometria de imagem a 0,001 pt e tem rasters PDF/SVG idênticos. Os 66 PDFs das campanhas de fontes/controles que foram produzidos passam qpdf; isso não qualifica diagramação por si só.

Dos 19 arquivos matemáticos locais, 18 são aceitos pelo oracle: cinco desses são rejeitados pelo Tekt; treze compilam bilateralmente e quatro passam geometria textual estrita. `primes`, o restante, é rejeitado pelo vanilla e aceito pelo Tekt, e não recebe crédito de paridade. Diferenças de extração de delimitadores compostos não foram chamadas de omissões sem confirmação visual.

## Recursos e validação

Na janela final, sem build/compilação concorrente, 308 processos medidos em quatro cargas, com dois warmups, ordens alternadas e IC95% pareado. JPEG com ruído: **−2,47% no tempo e −30,53% no RSS contra vanilla**; relatório matemático: **+10,25% no tempo e +4,65% no RSS**. Artigo/figuras têm layout diferente e seus recursos são apenas contexto. Não há superioridade universal de desempenho. A coleta inicial que coincidiu com análise de raster foi arquivada e excluída do aceite; somente a janela final sustenta esses números.

As **2170 entradas produtivas/L0 e o release coincidem exatamente com a validação completa imediatamente anterior**, reutilizada com prova de identidade: 12 gates, 10030 testes Rust, 7 ignorados preexistentes, 42 Python, build e crystalline-lint sem erros/avisos. As correções de notas, linhas, imagens/stack, largura/continuação matemática e ganhos JPEG continuam validadas nos corpora específicos; esses corpora finitos não substituem documentos comuns. Permanecem tracking R10, refutadores `super`, contextos de coluna/inline, baseline de rodapé e pinturas JPEG, entre os limites anteriores discriminados no relatório integral. O diagnóstico de 2026-10-27 é histórico: a antiga diferença de uma versus duas páginas de imagens já foi corrigida depois dele.

## Recomendação

Mostrar arquitetura, medições e exemplos previamente conferidos, com os casos que falham expostos. Antes de convidar usuários a trabalhar livremente: corrigir primeiro fluxo vertical de títulos/blocos, show/templates e contratos de tabelas/figuras; investigar CeTZ e repetir os mesmos casos sem retirar os negativos. Depois ampliar com documentos reais autorizados por voluntários. O help ainda expõe números internos de construção e falta um primeiro caminho de uso dirigido a usuários.

Proveniência: HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitada**, recebida às 2026-10-28T16:03:53.676317+00:00. Comparação principal 16:16:55–16:19:27 UTC; controles/benchmark têm horários próprios. `before.json` guarda a lista exata de `git diff HEAD --stat`, status e hashes; `source.json.gz` guarda os fontes. Foram preservadas as alterações existentes, sem edição produtiva/L0, commit ou push. Recursos: um job, cgroup 12 GiB/High 8 GiB, zero swap.

[Relatório integral e método](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/relatorio.md) · [Observações novas](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/observations.json) · [Controles mínimos](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/supplemental-observations.json) · [Tempo/RSS e amostras](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/benchmark.json) · [Preservação/fechamento](../../lab/results/construction/diagnostics/vanilla-readiness-2026-10-28/completion.json).
