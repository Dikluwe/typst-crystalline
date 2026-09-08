# P1318 — parecer independente final

Veredito: PASS no escopo P1318. Nenhum achado bloqueante. A implementação
acrescenta posição textual somente aos erros de parsing CSV Bytes e preserva
causa, ordinal, precedência e origem. Decoder público e composição Path/Str
mantêm o comportamento anterior. O parecer não fecha paridade geral CSV.

Última auditoria independente: `2026-09-08T15:47:51.052Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
`p1318-review-final-audit.json` registra diff/stat, hashes completos, comandos,
horários e resultados; `p1318-review-audit.cjs` reproduz as verificações de
integridade e comparação. Fonte
`870f861651c35f44afd5f3f68548bb0364463b2ca3d4d2087b5460bdbc986f5a`, L0 raw
`8cfa574ea121eb053598a5a45b7d54d4c1546ed920f519a3e2341b833f3e8246` e
executável `/tmp/p1318-target.eAgQwp/release/typst`, SHA-256
`0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c`,
continuam com as identidades julgadas.

A cadeia ex-ante está preservada: medição → L0 → delta de testes → RED →
freeze/GO pré-patch → candidato. O módulo inteiro de testes do candidato é
literalmente idêntico ao RED reconstruído. L0 normativo, artefatos históricos
P1315/P1316/P1317 e inputs congelados têm zero divergências de hash.
O erro de serialização U+2028 do writer foi corrigido com oito reexecuções
focais concordantes; o bruto original permaneceu preservado e pinado.

Resultados conferidos diretamente nos recibos e recontados:

- RED: oito falhas de sufixo, 59 pass; GREEN: 67 pass, zero falhas.
- Workspace: 6659 pass, zero falhas, três ignorados; exit 0.
- Build workspace release, fmt check e diff check: exit 0.
- Linhagem dry-run: `Nothing to fix`, exit 0.
- Lint: exit 0, zero errors, 240 warnings e 1137 infos. Não é ausência
  absoluta de avisos; não há reparo global ou eliminação dessas dívidas.
- A/B: 6156 observações integrais confrontadas independentemente com as
  expectativas congeladas, nas ordens normal/repeat/reverse e quatro perfis.
  Nenhum caso ausente/duplicado, zero diferenças, zero Unknown reportado.
  As 2052 expectativas cobrem 513 casos (387 replays históricos); 788
  distinguem baseline/candidato, sendo 756 de paridade vanilla integral e
  32 efeitos normativos de excesso, separados da dívida vanilla.

Freeze `aede13464afe720b4dffe501a9cab59f22f92a6101c12e9be7878c083abcb334`,
runs `c5907504691f3d2f524804624b3db343058e2db93d4004422332e8ecba168ef0` e
comparação `9ed11dfe247403eb6fd58942c4a774693e6fdc105b4ba425b2051667b686f1a1`
identificam o corpus julgado. A execução candidata ocorreu entre
`2026-09-08T15:42:08.395881+00:00` e `15:46:09.393724+00:00`.

Regime: A/B executado sem atestação técnica de isolamento; autoridades e
capacidades declaradas nos recibos anteriores/freeze. O revisor escreveu
somente `p1318-review-*`, nunca o produto nem os oráculos. Não há mutation
score, selo de refinamento, equivalência geral ou atestação de isolamento.
Fallback sem Position e offsets impossíveis foram avaliados na fonte;
CLI não os atesta. As dívidas de Path/Str, excesso, casts/opções e demais
superfícies fora do recorte permanecem delimitadas no L0.
