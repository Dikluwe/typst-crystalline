# P1369 — relatório da medição bilateral S02

## Veredito

**CONTRACT_REFUTED.** A execução foi completa e segura, mas refutou a premissa
herdada do P1357/P1368: o vetor observado foi `(true,true,true)` em ambos os
produtos e nos quatro perfis, não `(true,false,true)`.

O resultado registrado pelo classificador congelado é quatro
`DIFFERENCE`, pois ele só admite `MATCH` quando ambos satisfazem o vetor
esperado. Materialmente, os dois produtos tiveram valores byte-normalizados
iguais em todas as 12 comparações correspondentes; portanto não há evidência
de gap cristalino–vanila neste recorte. Há evidência de que o oráculo de
precedência estava invertido.

## Causa atribuída depois da medição

A fonte vanila confirma a observação. Em
`lab/typst-original/crates/typst-pdf/src/lib.rs`, `PdfOptions.format` é
documentado como opções que **substituem** os defaults definidos pelo
documento, e `resolve` chama `self.format.resolve(doc)`. Em
`typst-cli/src/compile.rs`, `--pdf-tagged=true` vira
`config.tagged = Some(true)` e depois `PdfFormatOptions.tagged =
Smart::Custom(true)`. Assim, no conflito congelado, a CLI vence o setting do
documento. A hipótese “documento vence CLI” não veio do comportamento vanila
ratificado.

Nenhum expected foi alterado após observar o resultado. O contrato permanece
refutado e deve ser substituído explicitamente num sucessor, não corrigido
retroativamente neste passo.

## Execução e segurança de memória

- 24/24 compilações concluídas;
- 24/24 chamadas `pdfinfo` concluídas;
- zero timeout, sinal, reason code ou observação opaca;
- maior compilação: 316,243 ms;
- maior `pdfinfo`: 34,478 ms;
- soma dos tempos de compilação: 5.179,989 ms;
- soma dos tempos `pdfinfo`: 521,825 ms;
- maior PDF: 5.639 bytes;
- maior stdout: 441 bytes;
- maior stderr: 276 bytes.

Cada subprocesso rodou sozinho com `RLIMIT_AS=2 GiB`, `RLIMIT_FSIZE=16 MiB`,
`RLIMIT_CPU=45 s`, timeout de 45 s e leitura máxima de 1 MiB por canal. Os
canais foram arquivos temporários, não pipes acumulados em RAM. Não houve
retry, Cargo, build, watch, rede deliberada ou suíte ampla. Não foi medido o
pico efetivo de RSS; somente os tetos e a ausência de OOM/SIGKILL foram
observados.

## Regime e limitações

Protocolo completo executado sem atestação de isolamento, pois contrato,
runner e veredito compartilham workspace e autoridade. O teste unitário do
runner passou 1/1; o RED anterior foi a ausência do módulo candidato. A
medição prova somente o recorte S02 nos binários e perfis congelados, não
paridade geral de PDF.

Os gates finais `py_compile`, validação JSON pela biblioteca padrão,
`git diff --check` e `crystalline-lint .` retornaram exit 0. O lint manteve
somente warnings/infos preexistentes e teve sua apresentação limitada pela
ferramenta.

## Próximo passo indicado

P1370 deve retificar o oráculo para a precedência medida e confirmada na fonte:
CLI explícita vence o setting do documento. Depois, reclassificar o mesmo
resultado P1369 sem repetir as 48 execuções externas, desde que hashes de
binários, observador e resultado permaneçam idênticos.
