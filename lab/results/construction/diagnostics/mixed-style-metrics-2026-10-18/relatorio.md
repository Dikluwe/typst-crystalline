# Métricas de faces mistas e admissão de linhas

Etapa concluída; o objetivo **Faça todas as correções** continua aberto.
As pendências estão em [proximos.md](proximos.md). A autorização R2-S já foi
registrada e executada no lote Sequence; esta etapa posterior corrige geometria
privada, sem novo campo/assinatura pública, modo do produto ou fase de callbacks.
Não houve commit, push, merge ou release.

## Resultado e causa

O fechamento de linha escolhia a face pelo maior tamanho nominal e conservava
a primeira em empates. Isso perde extremos de fontes diferentes: na Libertinus
12pt, o topo romano mede 7.896pt e o itálico 7.740pt. A correção negativa entre
linhas também era descartada. O efeito de 0.156pt estava nas posições SVG, não
apenas nas caixas extraídas do PDF.

O fechamento agora calcula top máximo e bottom mínimo independentemente nos
itens textuais realmente emitidos. Envelopes Link/Semantic/TextReplacement são
percorridos uma vez; Group conserva a caixa própria, sem recontar seus filhos.
Marcadores de quebra permanecem no frame e deixam de oferecer uma face fictícia;
Text vazio autoral conserva sua métrica. Os caminhos sem texto e os fallbacks
legados de colunas preservam seus contratos.

A admissão ordinária distingue previsão genérica, texto com face conhecida e
top certificado. Nova página registra o ascent que efetivamente instalou,
incluindo zero. A entrada após bloco usa esse top antes de decidir avançar.
A proteção de viúvas/órfãs admite faces diferentes com mesmo size/leading,
usando alturas reais das próximas linhas. Profile empresta ranges/Prepared;
foi removido o vetor temporário de referências da emissão hifenizada.
Não se refaz seleção de linhas nem se executam callbacks para medir conteúdo.

## Medição de paridade

Foram repetidas **297 sondas bilaterais**: 276 comparações PDF, 12 controles
de decoração e nove controles SVG de corpo vazio. Trinta das sondas PDF também
têm comparação SVG. Há fontes e controles repetidos; não são 297 bugs distintos
nem uma porcentagem de cobertura da linguagem.

Aceitação da tabela: mesmas palavras, páginas, atribuição das palavras às páginas,
dimensões e caixas com tolerância de **0.001pt**. O audit impede regressão em
qualquer caso que já passava. Este certificado geométrico não certifica conteúdo
semântico não medido, acessibilidade, bytes idênticos ou toda a linguagem.

| Matriz | Casos | Antes aceitos | Depois aceitos |
| --- | ---: | ---: | ---: |
| Texto misto / três fontes | 24 | 5 | 8 |
| Bibliografia / seis sondas | 6 | 0 | 6 |
| Páginas curtas | 20 | 0 | 20 |
| Recuos / matriz anterior | 132 | 83 | 114 |
| Margens / controles anteriores | 94 | 84 | 94 |

As 20 páginas curtas passaram, incluindo alturas 39.44/55.14/70pt, que antes
antecipavam quebra ou distribuíam 3+1 linhas em vez de 2+2. Os 31 resíduos de
faces mistas da matriz de recuos e os dez de margens foram eliminados.
Continuam abertos sete casos Plain+optimized em CSL custom, dez containers e
um alias de Content na matriz de recuos; nenhum foi reclassificado como aceito.

No SVG, **30/30** sondas coincidem em Y e **27/30** em X/Y dos glyphs,
com transformações ancestrais compostas e IDs ignorados. As três restantes
Liberation Serif conservam o deslocamento horizontal anterior de 0.216796875pt.
Dos 16 casos genéricos que não passam nas caixas PDF, 13 têm posições SVG
coincidentes; é preciso separar metadados/precisão/extração de um defeito da
língua antes de escolher o reparo. Não se alterou largura para maquiar métricas.
Os 12 SVGs de decoração Tekt ficaram byte a byte iguais ao predecessor, inclusive
as origens LTR já corrigidas; os nove corpos vazios continuam sem traço fantasma.
Isso não fecha Y, espessura, cauda ou RTL completo de decorações.

## Testes, lint e recursos

**9041 testes passaram, 0 falhas, 7 ignorados; 5 doctests passaram.**
Os 12 testes novos cobrem extremos independentes, ordem de faces, tamanhos
distintos, correção negativa, envelopes, Group, separador fraco, marcadores,
entrada regional, nova página, custos de viúvas/órfãs e entrada após bloco.
Os REDs funcionais estão em red-functional, pagination-red e block-entry-red.
A primeira tentativa red não compilou por import da fixture; não conta como RED.
A primeira fixture regional de paginação conservava cursor_x do cenário direto;
foi corrigida e seu RED funcional isolado foi preservado antes da implementação.
As falhas e os candidatos intermediários permanecem arquivados.

A primeira suíte ampla registrou três falhas em initial-workspace: um teste
geométrico exigia desigualdade binária estrita entre 41.449999999999996pt e
41.45pt; os dois testes de IDs aleatórios recebiam a data fixa do harness.
O L0 do observador foi atualizado antes de seu código: somente essa desigualdade
passa a exigir coordenadas finitas e admite um epsilon relativo de f64
(aproximadamente 9.2e-15pt nessa magnitude). Fixture e código produtivo não
mudaram para acomodar esse RED; a comparação de paridade continua em 0.001pt.
Os testes de IDs conservaram todas as asserções: o runner deixou de lhes impor
o relógio fixo. A rodada completa seguinte repetiu os dois conjuntos e passou.

Build workspace, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations,
fmt, Rustdoc -D warnings, V1/V5/V15/V26, linhagem reversa e diff-check passaram.
Clippy e todos os testes finais foram executados sobre o freeze refinado.
Snapshots PDF usaram o contrato existente CRYSTALLINE_PDF_FIXED_EPOCH=1792281600,
com TMPDIR em /dev/shm e o linter/oráculo explicitamente pinados. O workspace
sem wiring usa essa data fixa; wiring é testado separadamente, sem impô-la aos
testes que verificam IDs aleatórios. suite-summary soma os dois conjuntos.
A primeira invocação isolada de wiring incluía --lib, inválido para esse pacote
de binário; Cargo parou sem executar testes. O log foi preservado e finish-final.py
retomou com --bins/--tests, reutilizando build/Clippy/workspace já aprovados no
mesmo freeze. Essa falha do runner não foi contada como teste funcional RED.

**O gate Tekt global continua bloqueado pelos 105 V16 herdados.** A comparação
por regra/severidade/arquivo/mensagem não trouxe erro ou warning novo nem apagou
os antigos. V19: 1123→1128 notas; V20: 879→878. Os agrupamentos nominais novos
estão prescritos nos L0s e permanecem visíveis, sem supressões; executar um arm
agrupado não prova cobertura de todas as alternativas. O delta está em lint-delta.

MemoryMax **12GiB**, High **8GiB**, swap máximo zero, Cargo jobs1,
debug0/incremental0 e trabalho pesado sequencial. Pico da etapa:
**4.450GiB (4778545152bytes)**; eventos OOM/OOM-kill zero.
O pico é de cgroup da janela, incluindo compilação/cache. Não é benchmark de
RSS do produto nem prova de aproximação de desempenho ao vanilla.

## Proveniência e reprodução

HEAD **b1b486f3d42431f8ea360bc5712637ee1e3d4f6c**, branch Tekt, working tree **não commitada**.
Freeze final: **2026-10-18T11:27:25.018026+00:00**. Os 2102 arquivos produtivos/L0 ficaram
congelados durante build e testes finais; hash de cada um está no freeze.
measure-freeze preserva o estado anterior das medições. Entre os dois estados,
somente o harness cfg(test) de integração e seu L0 mudaram pelo ajuste numérico;
todos os demais hashes ficaram iguais. O build final foi repetido e comprovou
a mesma identidade do CLI usado nas 297 sondas, sem repetir medições inalteradas.
before-sources/after-sources guardam os textos completos; stage.patch separa as
alterações deste lote do trabalho herdado. Os oito owners L0 foram atualizados
antes do respectivo código e os selos revalidados. Ownership continua 1:1;
nenhum consumer/núcleo produtivo novo foi criado neste lote.

Vanilla pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore
**cac836c07e4dec2a89219370428292393a3216b5**; SHA do oráculo
**9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9**. As quatro fontes normativas upstream foram
comparadas com os objetos Git pinados (oracle-sources-final).
CLI anterior ce9981c27e72e389cb208884416cb10b1bbdd4c17803247d9570ecf7e1389a80;
CLI final **3525e1a570386bf3274cc83ddd5dfb2500d2b9ce9ffcda7202df44806d4cf24e**. Linter CI commit
12797073972e375d9ab078cf6750c8b30b71e2b9, SHA **e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2**.

Cada coleta conserva UTC, HEAD/status/diffstat, fontes, argv e SHA dos binários.
run.py registra MemoryMax/High/swap/jobs/pico/eventos. validate-final.py mostra
os comandos sequenciais. audit-final compara metadados preservados antes/depois
e glyphs SVG; artifacts-final.tar.gz guarda as saídas finais, e
artifacts-candidate1.tar.gz as saídas intermediárias. Os artefatos brutos genéricos
iniciais foram sobrescritos pelo primeiro candidato; seus bboxes, coordenadas SVG,
fontes exatas e binário anterior estão preservados. Não alegamos guardar aqueles
PDFs brutos iniciais. A tentativa regional SVG sem padrão de páginas foi recusada
pelo vanilla e não serve como prova; as 20 comparações regionais usam PDF válido.

artifact-verification confere 654 observações de artefatos em 642 arquivos.
Os seis PDFs vanilla bibliográficos foram recapturados mais tarde pelo corpus
no mesmo caminho: fonte, argv, binário e geometria extraída são exatamente iguais,
mas o relógio do oracle muda seus bytes. O mapa preserva ambos os SHA e vincula
os bytes sobreviventes à coleta posterior; não alega conservar os seis PDFs brutos
da primeira dessas duas observações. Nenhuma comparação geométrica foi alterada.

O manifesto anterior de bibliography-hanging foi verificado antes das mudanças
(1452 entradas, SHA 800c53750031041b4d61a26673d563e842b104c061cacc450173e6f29760d534).
Seus relatórios e evidências continuam históricos e intactos. manifest.json e
manifest.sha256 deste lote selam o relatório, anexos e os arquivos alterados.

Arquivos produtivos/L0 alterados somente nesta etapa:

- `01_core/src/compiler/layout/cursor.rs`
- `01_core/src/compiler/layout/line_commit.rs`
- `01_core/src/compiler/layout/paragraph_emit.rs`
- `01_core/src/compiler/layout/paragraph_pagination.rs`
- `01_core/src/compiler/layout/text_baseline.rs`
- `01_core/src/compiler/layout/region_progress.rs`
- `01_core/src/compiler/layout/word_layout.rs`
- `03_infra/src/integration_tests.rs`
- `00_nucleo/prompts/infra/integration_tests.md`
- `00_nucleo/prompts/compiler/layout/cursor.md`
- `00_nucleo/prompts/compiler/layout/line_commit.md`
- `00_nucleo/prompts/compiler/layout/paragraph_emit.md`
- `00_nucleo/prompts/compiler/layout/paragraph_pagination.md`
- `00_nucleo/prompts/compiler/layout/text_baseline.md`
- `00_nucleo/prompts/compiler/layout/region_progress.md`
- `00_nucleo/prompts/compiler/layout/word_layout.md`
