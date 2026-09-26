# Passo 1504 — separar as responsabilidades remanescentes do Cursor

**Estado:** IMPLEMENTADO em 2026-09-25. Este é um passo de execução, não um
Prompt L0; não legitima código por si só. A execução deve ser fatiada e
validada após cada fronteira, sem reunir todas as extrações num único
refactor indivisível.

## Medição antes da decisão

Às 2026-09-25T21:27:48Z, HEAD
`0de5edabed197d23b647cfc68ebefda2f5c2a8d5`, **working tree não
commitada** com o P1503: `git diff HEAD --stat` mostrou
`compiler/layout.md` (16 linhas alteradas), `compiler/layout/cursor.md` (26),
`layout/cursor.rs` (698) e `layout/mod.rs` (8), total rastreado de 59
inserções/689 remoções. Não rastreados nesse instante:
`00_nucleo/diagnosticos/typst-p1503-responsabilidades-cursor.md`,
`00_nucleo/materialization/typst-passo-1503.md`,
`00_nucleo/prompts/compiler/layout/line_commit.md` e
`01_core/src/compiler/layout/line_commit.rs`. Esta base **não** pode ser
reconstituída apenas pelo HEAD; antes de executar P1504, consolidar P1503
ou registrar novamente o estado exato e os hashes. Não alterar nem commitar
P1503 como efeito colateral deste documento.

`01_core/src/compiler/layout/cursor.rs` tem 1.324 linhas, SHA-256
`f73f45955445d3159a0952cd7d0e3a26bb86001d0989446817788ec86e22d14d`.
Os intervalos atuais são mapa de investigação, não autorização automática
para mover blocos inteiros:

| Região | Responsabilidade observada | Fronteira candidata |
|---|---|---|
| `:74-359` | palavra/chunk, wrap, baseline inicial e RTL | núcleo do Cursor; só a segmentação pode sair |
| `:361-618` | transição e materialização de página | montagem de página separada; `new_page` coordena |
| `:624-737` | fechar, fundir e iniciar colunas de página | owner existente `compiler/columns.md` |
| `:743-856` | fitting, marcador de float, transação/replay e drain | auditar por função, não mover como bloco |
| `:862-1064` e `:35-65` | translação e origem física de float diferido | owner existente `compiler/layout/place.md`, se o mapa confirmar |
| `:1087-1189` | emissão de caudas diferidas de células | unidade de commit de cell tails |
| `:1192-1303` | ICU, scripts sem espaços e tailoring de aspas CJK | unidade de segmentação de palavra |

O L0 vigente `compiler/layout/cursor.md` (SHA-256 bruto
`1ee102499de08fff97862b805e2ccb5c5a8bc180eead265723c26840b1331db8`)
atribui explicitamente página/colunas, floats/replay, cell tails e
segmentação ao Cursor após P1503. `compiler/layout.md`
(`89ae2a2162f929e9129dafb8e03950efefe1bbaa2481e72df6f974f58596591c`)
possui o estado/checkpoint do `Layouter`; `compiler/columns.md`
(`61f52b9a0ae09d41b253ec7fd3dc87064ad4e8b18c85f3bd5e6d59ebfb516fc0`)
possui o layout da feature Columns, inclusive modo de página;
`compiler/layout/place.md`
(`54d51225cfbdf93148198b65eab6f1a8e47315da2105da9bb1443589b741cf59`)
possui fitting/posicionamento de Place. `compiler/layout/page_running.md`
e `compiler/layout/page_canvas.md` já possuem, respectivamente, marginais
e layers decorativos. Qualquer transferência exige primeiro delimitar esses
contratos, sem duplicar ownership.

O diagnóstico P1503 mediu dois documentos antes/depois: sem mudança de
texto/geometria e sem ganho demonstrável de tempo ou RAM. O layout inteiro
de `repeat` levou cerca de 0,6 ms numa sonda, enquanto a carga externa de
fontes do cristalino levou cerca de 270 ms contra 123 ms no vanilla no mesmo
host. Essas medições **não** tornam o Cursor um gargalo de performance.
O objetivo de P1504 é localidade e precisão de mudança (ADR-0109), não
otimização nem imitação da mecânica do vanilla (ADR-0107).

## Hipótese e decisão provisória de ordem

Hipótese: é possível deixar o Cursor como coordenador de posição, baseline,
wrap e avanço de região, delegando materialização de página, operações de
coluna, caudas de célula e segmentação a owners legíveis isoladamente. A
coordenação de floats só sai onde houver uma fronteira causal limpa. A ordem
abaixo é uma **fila de auditoria**, não uma decisão L0 antecipada:

1. **Commit da página.** Em `new_page`, mapear `:396-572` — dimensões auto,
   fixups, marginais, canvas e construção de `Page`. Candidato novo:
   `compiler/layout/page_commit.rs` + L0 proprietário
   `compiler/layout/page_commit.md`. `page_running` continua dono de
   header/footer/numeração marginal; `page_canvas`, dos layers. O Cursor
   conserva a decisão de avançar, flush de inserções/footnotes, reset da
   região e chamada de commit. Rejeitar se o módulo novo precisar decidir
   quando paginar ou duplicar a política dos owners existentes.
2. **Colunas de página.** Avaliar mover `close_current_column`,
   `merge_column_items`, `finish_columns`, `start_next_column` e
   `start_column` para o consumer existente `layout/columns.rs`, atualizando
   seu L0 `compiler/columns.md` e o do Cursor. Manter a chamada de avanço
   no `new_page`; não misturar colunas contínuas com segmentos explícitos
   sem preservar os respectivos contextos e numeração P1415–P1419.
3. **Caudas de célula.** Candidato novo `compiler/layout/cell_tail_commit.rs`
   + L0 proprietário `compiler/layout/cell_tail_commit.md` para a emissão,
   z-order, rebase de items/align órfão e avanço causado pelos tails. O
   Cursor apenas invoca o hook no topo da nova página; `grid_cell.rs`,
   `table_cell.rs` e `slicing.rs` não ganham a política de transição.
4. **Segmentação de palavra.** Candidato novo
   `compiler/layout/word_segment.rs` + L0 proprietário
   `compiler/layout/word_segment.md` para seleção de scripts, ICU,
   breakpoints e tailoring CJK. `layout_word` e `layout_chunk` permanecem
   no Cursor; a unidade nova devolve/itera fragmentos ou usa um hook interno
   sem inventar API pública, estado global mutável ou I/O em L1.
5. **Float e replay, somente após mapa causal.** Primeiro, verificar se
   `deferred_frame_origin_y` e `translate_deferred_float` são materialização
   própria de `layout/place.rs`; se sim, transferir ao L0
   `compiler/layout/place.md`. Separadamente, estudar
   `layout_flow_suffix_transaction` e o checkpoint de `layout/mod.rs` como
   possível unidade de replay com consumer/L0 novos. Os hooks
   `finish_float_prefix_at_marker`, `flush_pending_floats` e
   `flush_fitting_pending_floats` podem permanecer no Cursor se sua
   responsabilidade real for coordenar transições de região. **Não** criar
   um módulo genérico de floats só para reduzir linhas nem mover o estado
   transacional de `layout/mod.rs` por conveniência.

Para cada candidata, refutam a extração: dependência circular, aumento de
visibilidade (`pub(crate)`), cópia de estado/checkpoint, wrapper vazio que
obriga o leitor a saltar de arquivo, ownership L0 concorrente ou mudança
observável sem decisão própria. Se refutada, registrar a medição em
`diagnosticos/` e manter a função no owner atual; não forçar as cinco fatias.

## Protocolo por fatia

1. Registrar HEAD, estado não commitado, horário, hashes de fontes e
   binários, pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`
   e mapa `file:line` de chamadas/campos lidos-escritos. Comparar o L0
   completo de todos os owners afetados antes de fechar a fronteira.
2. Construir controles de linguagem antes do movimento e guardá-los para
   depois: página auto/numeração/canvas/footnotes; colunas e patterns;
   grid/table com tail e align; CJK/Thai/RTL/hifenização; float top/bottom,
   marker, sufixo rejeitado, replay, sub-layout e ausência de float.
   Comparar texto, morfologia, páginas, posições e diagnósticos — não bytes
   PDF nem igualdade interna de `FrameItem`. Incluir controles sem a feature
   para detectar efeitos laterais.
3. Redigir/atualizar **primeiro** os Prompts L0 proprietários, com caminho
   canônico (ADR-0130), e só então escrever teste arquitetural RED e mover
   código. Se criar novo consumer, validar V15/V26 antes de ressellar hashes;
   um L0 novo sem consumer é apenas estado transitório, não GREEN.
   Preservar `Layouter` em `compiler/layout.md`, despacho estático/exaustivo,
   imports e pureza L1. Não criar Núcleo Tekt sem invariantes realmente
   compartilhados por dois ou mais prompts (ADR-0129).
4. Executar uma fatia por vez: RED estrutural explícito, transferência
   mecânica, GREEN funcional e estrutural, `cargo build`, testes focais e
   `crystalline-lint .` sem violações. Revalidar também V5/V6/V15/V26,
   dry-run de hashes e `git diff --check`. Corrigir regressões na própria
   fatia antes de iniciar a seguinte; não esconder uma falha com refactor
   simultâneo de outro owner.
5. Repetir medições de tempo/CPU/RAM com o mesmo corpus, build release,
   ambiente e dispersão; relatar apenas efeitos sustentados por números
   reproduzíveis. Registrar o raio de mudança de uma correção focal antes
   e depois de cada fronteira. Performance não é condição presumida de
   aceitação da atomização.

## Gate e conclusão

As transferências internas, sem novo contrato público, default, fase ou
quebra de compatibilidade, seguem fluxo contínuo: L0 primeiro, testes
RED→GREEN e revalidação. Se qualquer fatia exigir uma dessas quatro
mudanças — ou a classe for duvidosa — **parar após o L0 e pedir confirmação
humana antes de código** (ADR-0127). Não misturar correção de paridade,
otimização de fontes ou mudança de algoritmo com esta separação.

O passo só estará concluído quando cada candidata tiver decisão registrada
(extraída ou refutada), owner único verificável, controles de linguagem
estáveis e relatório final com proveniência. Não usar uma meta arbitrária
de linhas: o Cursor pode continuar com hooks curtos de coordenação se esse
for seu papel correto.

## Execução e decisão final P1504

No HEAD `0de5edabed197d23b647cfc68ebefda2f5c2a8d5`, working tree
não commitada com P1503 e P1504, a auditoria individualizou quatro owners
novos: `page_commit`, `cell_tail_commit`, `word_segment` e `flow_suffix`.
As operações de colunas físicas foram para o owner existente `columns`;
a geometria final de floats, para o owner existente `place`. Os hooks
`finish_float_prefix_at_marker`, `flush_pending_floats` e admissão por
região ficaram no Cursor por coordenarem avanço, não posicionamento.
`new_page` e `layout_word` permanecem como coordenadores. A montagem
terminal em `finish()` fica para outro passo, explicitamente fora do L0
de `page_commit`.

Cada fatia recebeu L0 primeiro, teste estrutural RED antes do movimento
e GREEN depois. O conjunto P1504 passou 11/11; a suíte L1 completa passou
6.207 testes, com os mesmos dois testes históricos de stack overflow
excluídos. P1407 L3 passou 5/5, P1493 6/6 e P1502 4/4. Builds debug e
release offline, lint completo, V5/V6/V15/V26 estritos e dry-run de hashes
passaram. O cursor passou de 1.324 para 562 linhas.

Dois PDFs de controle conservaram texto e bbox de palavras idênticos ao
P1503. Benchmarks com warmup e dez execuções não mostraram ganho
sustentado de tempo ou RAM. O relatório completo, hashes, horas, corpus,
raio de mudança e limites estão em
`00_nucleo/diagnosticos/typst-p1504-responsabilidades-cursor.md`.
Não houve alteração de contrato público, default, fase ou compatibilidade;
aplicou-se fluxo contínuo ADR-0127. Nenhum commit foi criado neste passo.
