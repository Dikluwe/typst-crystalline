# P1384 — relatório de paridade

## Veredito

`MATCH` no recorte exato em que um `Styled(Text)` maior, alterando somente
`text.size`, é seguido por `Text` direto no tamanho exterior antes de um Block
fracionário em segmento explícito de colunas. Vanila e cristalino produzem uma
página e as mesmas coordenadas: `Lead=5,752`, `More=12,010`, `One=32,448`,
`Two=7,404` e `After=Tail=220,166` pontos.

Não há alegação geral. Fragmento único maior, 18pt→8pt com segundo wrapper,
dois wrappers 18pt→11pt, primeiro delta size+fill e três fragmentos continuam
`Unknown` e fail-closed.

## Implementação

O reconhecedor adia a rejeição do primeiro fragmento maior apenas até conhecer
a forma completa. Aceita somente primeiro wrapper size-only maior e segundo
texto direto exterior. O top-edge maior já entra pelo máximo da linha; a outra
parcela usa o top-edge exterior previsto pelo Cursor P1382, evitando contar o
18pt duas vezes. Leading e boundary continuam exteriores como em P1383.

Não há recursão, coleção auxiliar proporcional, carrier, API pública ou
mudança de pipeline.

## Correção de proveniência

A primeira tentativa foi invalidada antes da implementação final: a fixture
histórica chamada reverse era 18pt→8pt, embora tivesse sido interpretada como
18pt→tamanho exterior. Candidata e testes foram retirados, criou-se a fixture
correta `2fd7f64d...`, repetiu-se a medição bilateral e reabriram-se L0,
contrato e oráculos. A cadeia final parte somente dessa medição corrigida.

## Evidência

- RED válido: 2/3; somente o positivo falhou. GREEN final: P1384 3/3.
- Controles: P1380–P1383 13/13 e P1374–P1379 16/16.
- Matriz bilateral: 2 `MATCH`, 5 `Unknown`; os opacos não foram absorvidos.
- `cargo check --workspace`, `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 1433 advisories preexistentes e zero
  advisory novo.
- Build e testes seriais sob 4 GiB; matriz sob 2 GiB/45 s. Pico máximo de toda
  a execução: 1 958 288 KiB; RED válido 1 753 776 KiB, GREEN final
  1 605 192 KiB, release 1 147 844 KiB e matriz 37 396 KiB. Nenhum OOM,
  timeout ou SIGKILL.

Proveniência final medida em 2026-09-13T22:08:19-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`4808` inserções, `419` remoções) e 345
entradas de status. Binários: vanila `eb60986b...`, cristalino `99940887...`,
teste `dc527685...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, testes, candidato, ataques e veredito; hashes demonstram
ordem e identidade, não independência.
