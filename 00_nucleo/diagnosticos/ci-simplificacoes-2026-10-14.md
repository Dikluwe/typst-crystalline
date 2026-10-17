# CI: expressões equivalentes sem relaxar lints

O inventário Clippy caiu de 227 para 196 avisos distintos.
Foram removidos 31 avisos em dez consumers, com atualização prévia dos dez
prompts proprietários e resselo restrito. Os seis grupos selecionados passam
como erros no workspace/all-targets. O CI geral com -D warnings continua
pendente dos demais avisos; nenhum lint foi suprimido ou reconfigurado.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, snapshot às 2026-10-14T07:19:34.089089+00:00.
before.json.gz conserva status, git diff HEAD --stat, fontes/owners com SHA-256,
spans e seleção do inventário anterior. O inventário original vem da validação
content-serialization-2026-10-14, executada no mesmo estado produtivo antes deste
lote; o comando RED deste lote revalida as classes escolhidas.

RED entre 2026-10-14T07:19:34.682837+00:00 e 2026-10-14T07:20:29.975353+00:00: exit 101, 29 erros únicos
na infraestrutura. O Cargo interrompeu esse caminho antes de verificar wiring;
os dois avisos adicionais de wiring estão no inventário all-targets anterior,
não são apresentados como erros emitidos pelo RED. Mensagens duplicadas por
lib/test são deduplicadas por código, mensagem e spans exatos. Não confundir
número de mensagens ou avisos com número de defeitos funcionais.

### Evidência e classificação por unidade

- font_metrics/resolution e shaper/font_selection: units_per_em já é u16;
  os casts u16→u16 não convertem valores. max(1), cadeias, identidade e ordem
  de seleção permanecem. Os predicados Option conservam ausência false e a
  mesma consulta de MATH/cobertura apenas quando há Some.
- gradients/adaptive: next_t já é f64; retirar f64::from não muda precisão,
  offsets, interpolação, thresholds ou subdivisão. export/fonts tinha Into
  de EcoString para o mesmo tipo em fixture; texto e asserções permanecem.
- query_helpers: header/footer/caption/title usam is_some_and, com os mesmos
  predicados, valores ausentes e curto-circuito. O walker continua sendo a
  aproximação legada; não foi promovido a índice semântico de query.
- world/font_session: closures que só chamavam verify são substituídas pelo
  callable. Os primeiros usos emprestam &verify; o último consome o callable
  privado, sem uso posterior. Seus callers produtivos capturam paths por
  referência; os testes usam função ou closure sem estado próprio destrutível.
  Ordem/condicionalidade das verificações, lock, postcheck e fallback ficam
  intactos. Não houve memoização nem aquisição nova de recursos.
- fontdb/generation e shaper/tests: fixtures usam slice::from_ref no lugar de
  clones feitos só para uma slice unitária. Arrays AB/BA/repetidos conservam
  seus valores e clones necessários. A mesma simplificação foi aplicada aos
  dois estilos dos controles pareados, inclusive o quarto clone que não gerava
  diagnóstico próprio. Todas as expectativas e fixtures continuam iguais.
- integration_tests passa os mesmos quatro arrays de bytes JPEG diretamente
  ao AsRef<[u8]> de write. O fixture mínimo não passa a provar decodificação
  JPEG geral. wiring/compile passa o &PathBuf já existente aos dois writes
  de timings; path, JSON, ordem e tratamento de erros permanecem.

Essas são simplificações Rust internas, não alterações de linguagem nem uma
medição nova de paridade com vanilla. Não se alega ganho de tempo por remover
clones. Nenhuma API pública, fase, default, dependência ou campo foi alterado.
As regras de ownership continuam 1:1; Núcleos e seus pins não mudaram.

## Testes e gates

GREEN do mesmo comando entre 2026-10-14T07:21:50.930445+00:00 e 2026-10-14T07:22:01.032312+00:00, exit 0:
unnecessary_cast, unnecessary_map_or, redundant_closure, useless_conversion,
needless_borrows_for_generic_args e cloned_ref_to_slice_refs tratados como
-D no workspace/all-targets. Os outros avisos permanecem visíveis, e o
inventário completo posterior confirma a redução líquida sem categoria nova.
Não foram criados testes que apenas espelham as substituições mecânicas;
os controles funcionais existentes foram executados integralmente, sem
relaxar suas expectativas ou mudar seus nomes.

Suíte completa de 2026-10-14T07:22:18.372423+00:00 a 2026-10-14T07:28:03.719706+00:00: 8455 passados,
0 falhas e 7 ignorados. workspace-pre/result conservam
HEAD/árvore/UTC, argv, fontes e ambiente com /dev/shm. O oráculo usado pelos
fixtures é o pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Fontes permaneceram estáveis durante cada execução.

Build, check all-targets, rustdoc -D warnings, fmt, diffcheck, linhagem estrita
V1/V5/V15/V26 e reversa passaram. Arquitetura: 0
erros, 382 warnings e
1631 notas, sem aumento. O inventário Clippy usa
cap-lints warn para enumerar todos os avisos; isso não aprova -D warnings geral.

## O que permanece aberto

Restam 196 avisos Clippy, além dos avisos arquiteturais e das frentes
funcionais já registradas nos diagnósticos de produção. Tipos complexos e
muitos argumentos não são, por si, defeitos de linguagem; precisam de revisão
por owner antes de reorganizar contratos. Não remover código só porque aparece
como dead_code: alguns helpers são exclusivos de testes e outros representam
contratos ou lacunas ainda não resolvidas.

A proposta pública de presença lexical em Raw continua aguardando confirmação
ADR-0127 e não foi implementada por este lote. Matriz remota Windows/macOS/i686,
Docker e distribuição continuam sem certificação nesta máquina. O objetivo de
corrigir todas as frentes segue ativo; não há declaração de produção concluída.

Nenhum commit, push, PR, publicação ou alteração do pin do linter foi feito.
evidence.json.gz conserva scripts, fontes/owners, logs completos, resultados e
diff do lote com base64 e SHA-256 verificado após releitura.
