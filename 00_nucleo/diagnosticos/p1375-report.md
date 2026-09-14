# P1375 — relatório de fração sob estilo transparente

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** Um `Block(height: fr)` sob o caminho
estrutural unitário produzido por `Content::Styled` participa agora da mesma
distribuição que um sibling direto. A sonda que antes gerava duas páginas no
cristalino passou a gerar uma, com `One=7,404`, `Two=82,724670` e
`After=220,166`, iguais ao vanila no observável de linguagem congelado.

O reconhecedor percorre referências iterativamente: não aloca por nível e não
usa recursão. Uma Sequence com dois filhos substantivos falha fechado. A
parcela atravessa apenas uma Sequence transparente que não cria distribuição
própria e é restaurada depois do wrapper.

## RED → GREEN e regressão

O RED focal terminou em 14,7 s com erro único: `fractional_block` inexistente.
Após a implementação, 2/2 testes P1375 passaram; os 4/4 testes P1374 também
passaram. `cargo check`, link do binário com debug desligado, formatação,
validação JSON, `git diff --check` e `crystalline-lint .` terminaram com código
zero. O resselo informou zero drift restante e o lint não encontrou violation
bloqueante; avisos consultivos históricos continuam fora do escopo.

## Memória e proveniência

Todos os processos foram seriais. Check limitado a 3 GiB; links a 4 GiB com
debug desligado; matriz a 2 GiB e timeout 45 s. O binário de testes tem
87.207.408 bytes, o cristalino 98.166.400 bytes e o maior PDF desta matriz
6.553 bytes. Não houve OOM do host, timeout, SIGKILL ou retry com debug
completo. O pico RSS não foi medido, portanto não é alegado.

Medição final em 2026-09-13T15:24:44-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado: 45 ficheiros no diff, 3.210 inserções, 336 remoções e 245 entradas
de status. A ausência de `jq` foi uma indisponibilidade de ferramenta, não uma
falha do candidato; os mesmos JSON foram validados com o parser Ruby.

O protocolo completo foi executado sem atestação de isolamento. Colunas,
multi-região, Strong/Emph e frações aninhadas continuam `Unknown`; o próximo
passo deve medir um desses e não herdar `MATCH` por analogia.
