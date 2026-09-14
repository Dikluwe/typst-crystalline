# P1371 — relatório final da reclassificação S11

## Veredito

**MATCH_WITH_LIMITED_ATTESTATION.** O adaptador cristalino executou o vetor
exato de duração: 1 teste aprovado, zero falhas e S11 reclassificado de
`Unknown(PARSER_UNSUPPORTED)` para `MATCH`.

A alegação combina a fonte/teste autoritativos vanila e o contrato P1361,
que rejeitou 22/22 mutantes válidos, com uma execução atual do binário de teste
cristalino pinado. Não executa o relógio, scheduler ou ciclo watch.

## Calibração e correção de segurança

O desenho inicial tentou executar Cargo com um limite de arquivo de 8 MiB.
Duas tentativas terminaram antes do teste: o `rustc` recebeu sinal 25
(`SIGXFSZ`) ao criar seu próprio artefato, com stderr de 24.314 bytes. Isso não
foi OOM nem falha do formatter. A primeira tentativa não conservou a cauda do
canal; a segunda acrescentou 8 KiB finais e revelou a causa.

Em vez de elevar limites e tentar uma terceira compilação, a revisão R2
inventariou o executável focal preexistente. Ele tem 154.550.664 bytes, foi
gerado depois das fontes atuais, lista nominalmente o teste exigido e possui
SHA-256 `06c91f06…e1be`. A execução direta eliminou Cargo/rustc e a escrita do
artefato grande.

## Resultado final e memória

- duração: 5,476 ms;
- exit: 0;
- testes: 1 aprovado, 0 falhas;
- stdout: 187 bytes;
- stderr: 0 bytes;
- subprocessos simultâneos: 1;
- teto de endereço: 3 GiB;
- teto de arquivo: 8 MiB;
- leitura máxima por canal: 2 MiB;
- timeout/CPU: 120 s.

Não houve OOM, SIGKILL, timeout, Cargo, rustc, Typst, `pdfinfo`, watch, rede ou
suíte ampla na execução aceita. O resultado JSON possui 1.116 bytes. O pico
real de RSS não foi medido; os limites e a ausência de terminação por memória
são a evidência disponível.

## Regime e limitações

Protocolo completo executado sem atestação de isolamento. O contrato passou
por revisão durante a calibração e testador, implementador e verificador
compartilham workspace/autoridade. O binário pinado é posterior às fontes por
timestamp, mas não contém uma ligação criptográfica interna ao hash delas.
Por isso o veredito permanece limitado e cobre apenas S11.

Os gates finais `py_compile`, validação JSON, `git diff --check` e
`crystalline-lint .` retornaram exit 0. O lint apresentou warnings/infos
preexistentes, sem violação bloqueante, e teve sua visualização truncada.
