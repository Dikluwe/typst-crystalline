# Relatório — catálogo matemático nativo

Etapa concluída: as 26 rotas matemáticas omitidas agora são reconhecidas como funções de elemento em `.where()` e na admissão de `figure(kind: ...)`. O catálogo mantém os campos selecionáveis e a não locatabilidade definidos no pin. O objetivo global permanece ativo.

## Proveniência e medição

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T14:38:31.985678+00:00`, fechamento `2026-10-17T15:18:54.703941+00:00`. before.json.gz/final-state.json.gz conservam status, lista exata de alterações via diff HEAD --stat e hashes de 2122 arquivos protegidos. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A matriz anterior foi reutilizada da etapa precedente somente após provar igualdade exata dos hashes de fontes/configuração e do binário Tekt atual, em baseline-reuse.json. É a medição anterior real, com seu UTC original, não nova execução presumida. A matriz posterior foi executada novamente: 3956 casos nos oito perfis, cobrindo type/where/kind. before-batch.json.gz e after-batch.json.gz registram 227 casos adicionais de campos, exclusões, label, counter e aplicação parcial. batch-schema.json conserva fonte primária do pin, blob SHA-256, file:line, bloco literal, atributos e ABI real de cada callback.

## Implementação e L0

Cinco pares consumer/owner foram atualizados; mais um Núcleo Tekt compartilhado `selectors/math-native-bridges.toml`, pinado pelos cinco L0. O L0 e o Núcleo precederam testes/código. A tabela central recebeu 26 entradas por fn_addr_eq tipada, sem invocar o constructor ou discriminar pelo nome. Todas são não locatáveis. As três fachadas transportam 23 callbacks existentes por reexport direto restrito a crate::compiler; accent/cancel/mat já tinham pontes. O alias interno de class preserva o reexport público de teste preexistente. Apenas scripts/limits ampliam visibilidade interna de pub(super) para pub(in crate::compiler), com corpos e assinaturas preservados.

integrity-result.json prova que os hubs mudaram somente por reexports internos, os corpos de scripts/limits são preservados, a montagem runtime de math não mudou e toda a lógica/testes produtivos anteriores de element_where permanece fora das 26 novas entradas. Não há API Rust externa nova, mudança de fase/default, binding novo, commit/push/release ou supressão de lint. TS1/equality, demais consumers e os manifestos TS1/RG1 permanecem íntegros.

| Binding | Campos selecionáveis | Campos excluídos |
|---|---|---|
| `std.math.accent` | `base`, `accent`, `size`, `dotless` | nenhum |
| `std.math.attach` | `base`, `t`, `b`, `tl`, `bl`, `tr`, `br` | nenhum |
| `std.math.binom` | `upper`, `lower` | nenhum |
| `std.math.cancel` | `body`, `length`, `inverted`, `cross`, `angle`, `stroke`, `background` | nenhum |
| `std.math.cases` | `delim`, `reverse`, `gap`, `children` | nenhum |
| `std.math.class` | `class`, `body` | nenhum |
| `std.math.frac` | `num`, `denom`, `style` | `num-deparenthesized`, `denom-deparenthesized` |
| `std.math.limits` | `body`, `inline` | nenhum |
| `std.math.lr` | `size`, `body` | nenhum |
| `std.math.mat` | `delim`, `align`, `augment`, `row-gap`, `column-gap`, `rows` | `gap` |
| `std.math.mid` | `body` | nenhum |
| `std.math.overbrace` | `body`, `annotation` | nenhum |
| `std.math.overbracket` | `body`, `annotation` | nenhum |
| `std.math.overline` | `body` | nenhum |
| `std.math.overparen` | `body`, `annotation` | nenhum |
| `std.math.overshell` | `body`, `annotation` | nenhum |
| `std.math.primes` | `count` | nenhum |
| `std.math.root` | `index`, `radicand` | nenhum |
| `std.math.scripts` | `body` | nenhum |
| `std.math.stretch` | `body`, `size` | nenhum |
| `std.math.underbrace` | `body`, `annotation` | nenhum |
| `std.math.underbracket` | `body`, `annotation` | nenhum |
| `std.math.underline` | `body` | nenhum |
| `std.math.underparen` | `body`, `annotation` | nenhum |
| `std.math.undershell` | `body`, `annotation` | nenhum |
| `std.math.vec` | `delim`, `align`, `gap`, `children` | nenhum |

`mat(gap:)` continua parâmetro de construção válido sob seu contrato; gap external não é campo de seleção. Frac não expõe controles internos de deparenthesização. Math underline/overline conservam identidade própria, diferente das decorações globais. Sqrt/abs/norm/round não viram elementos por produzirem conteúdo.

## Validação

- RED funcional: 26 testes falharam por identidade ausente; um controle passou. A primeira tentativa teve erro de inferência de lifetimes na fixture de ponteiros; foi corrigida e arquivada em red-harness-*, sem ser contada como RED funcional.
- O primeiro verificador de integridade esperava chaves também em imports unitários, mas rustfmt as remove. O parser foi corrigido para as duas formas, preservando a exigência do conjunto exato de 23 callbacks; a tentativa e a retomada com fontes congeladas estão arquivadas em integrity-checker-first-attempt.
- Clippy detectou um reexport novo após o módulo inline de testes do hub estrutural. A tentativa candidate-1 conserva fontes, hashes e logs. O L0 foi atualizado antes de mover as pontes para antes dos testes; todos os gates e a matriz foram repetidos sobre as fontes finais, sem supressão do lint.
- O comparador diferencial inicialmente reutilizou o caminho SARIF de uma etapa mais antiga. Foi corrigido para a etapa imediatamente anterior, com igualdade obrigatória entre seus hashes finais de fontes e o baseline desta etapa. O comparador completo confirmou zero diferenças após mapear as linhas; tentativa preservada em differential-baseline-first-attempt. Nenhum código mudou nessa correção do verificador.
- GREEN: 27 testes focados passaram. Suíte completa: 8879 passaram, zero falhas, 7 ignorados, sem mutação das fontes durante a execução.
- Matriz completa: 448 observáveis mudaram só nas identidades autorizadas e nos aliases globais Tekt accent/cancel; 416 valores de linguagem passaram a coincidir com o pin. Os demais 19856 observáveis ficaram idênticos, incluindo todas as saídas do vanilla.
- As rotas omitidas caíram de 41 para 15; nenhuma rota matemática continua omitida nesta matriz de admissão. Isso não é certificado geral de morfologia/layout/query/show.
- Dos 227 casos detalhados, 146 têm valores de linguagem iguais ao pin; 29 erros desconhecidos/excluídos têm mensagem correta mas âncora residual; 26 controles de counter e 26 de aplicação parcial foram preservados. Counter permanece divergente e aberto.
- A entrada das sondas é vazia. O aviso Tekt `ficheiro vazio: sem conteúdo` já existia em 4976 controles anteriores e continua nos novos sucessos; o verificador exige seus bytes exatos. Não é declarado stderr igual ao pin nem são descartados outros avisos.
- Build, fmt, Clippy workspace/all-targets estrito (-D warnings e unfulfilled_lint_expectations), Rustdoc estrito com quatro índices, V1/V5/V15/V26, linhagem reversa e diff-check passaram. V15/V26 foram verificados antes do resselo limitado aos cinco owners.
- Diagnóstico global preservado após deslocamentos de linha: 117 V16 errors, 1076 V19 notes, 873 V20 notes. Nenhum novo erro/aviso. O gate global continua bloqueado pelos V16 anteriores; este lote não os reduz nem os silencia.

## RAM e evidências

Comandos pesados sequenciais, MemoryMax12GiB, MemoryHigh10GiB, swap0, Cargo jobs1. Pico do cgroup: 10267193344 bytes (9.562 GiB). Eventos agregados `{'low': 0, 'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}`; sem atingir MemoryMax, OOM ou oom_kill. Inclui cache contabilizado pelo cgroup, não apenas RSS.

[Pendências e próximos lotes](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/follow-ups.md). Fontes exatas, escopo, patch, scripts, logs, testes, hashes e medições estão neste diretório; final-state.json.gz e SHA256SUMS fixam o fechamento. O Núcleo tem um papel distinto dos cinco Prompts L0 proprietários e não é alvo direto de @prompt.
