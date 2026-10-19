# Passo de materialização — contexto bidi completo

Este documento coordena execução; não é L0. Evidências: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05.md`. Pacote público: `00_nucleo/materialization/bidi-contexto-l0-manifest-2026-10-05.json`. Estado: pacote ratificado pelo humano e materializado no subconjunto certificado. Prova congelada em bidi-contexto-aprovacao-2026-10-05.json; implementação e medições em `00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05.md`.

1. Confirmar integridade do manifesto e receber ratificação humana; não materializar novos campos/APIs antes disso.
2. Auditar os 72 owners candidatos do inventário, distinguindo constructors, patterns, testes/comentários. Atualizar L0 internos antes de cada mudança; ownership 1:1 e compartilhamento normativo por Núcleo quando aplicável. Nenhum passo ou diagnóstico legitima código.
3. Escrever testes RED da entidade (races inexistentes, UTF-8/bounds/subspan/overflow/ocorrências repetidas), transporte/None/splits/clones e fonte real. Materializar o consumer novo e as declarações/campos públicos ratificados; não ressellar grafo incompleto.
4. Preparar entrada textual integral e ranges originais em L1 no subconjunto certificado, transportando por linhas/cauda/checkpoints. Auditar cache de frames e regiões recusadas; fonte desconhecida não vira origem inventada.
5. Resolver o parágrafo completo no L3 e aplicar ranges de linha/fragmento, removendo somente controles virtuais de sua representação auxiliar. Conservar dados originais durante reordenação e shaping; evitar duplicar Unicode por palavra/linha ou self-reference/unsafe.
6. GREEN das refutações, CLI bilateral de bases opostas, override e números no wrap, restauração e controles anteriores. Separar qualquer falha restante de espaços/fontes/objetos. Medir custo e contagens com SHA/árvore/UTC antes de afirmar benefício.
7. Build e suites apropriadas, V15/V26 antes do resselo limitado, linter completo comparado por multiset. Registrar limites e pendências de Block/Align; não resync vanilla ou promover novos defaults.

## Encerramento do pacote ratificado

Input/Span e campos opcionais materializados; transporte integral em L1 e resolução compartilhada em L3, sem mudar fase. 6528 testes L1 (+5 ignorados), 1165 L3, builds/lint registrados. Matriz 54 fontes: 47 passam extração/caixas agrupadas; 11 corrigidas e nenhum controle antes positivo regrediu nesse critério. CLI sem ganho demonstrado. Fonte real no fallback, extração de controles e Block/Align são frentes distintas ainda abertas, não foram fechadas por essa materialização. Relatório/receipts conservam refutações intermediárias e working tree integral.
