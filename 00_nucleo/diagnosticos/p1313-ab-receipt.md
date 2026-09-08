# P1313 — recibo independente A/B

Veredito do fragmento congelado: **PASS**. O candidato satisfez todas as
expectativas anteriores ao patch, sem alteração do corpus ou dos oráculos
depois da sua materialização. Regime A/B executado sem atestação de isolamento
técnico; não é selo do protocolo completo nem equivalência geral da linguagem.

## Proveniência e ordem causal

Executor testador: `/root/p1313_tests`. Entradas e capacidades declaradas no
freeze. Foram lidos skill e duas referências, L0 loading completo, passo P1313
explicitamente autorizado, baseline de implementação, fontes vanilla CSV e
DataSource, infraestrutura histórica A/B P1312 e observações públicas CLI.
Não foram lidos o owner cristalino, o diff candidato ou os testes locais.
Todas as escritas do papel usaram apply_patch em `p1313-ab-*` ou nas fixtures
dedicadas; não houve stage/commit. O filesystem compartilhado não atesta a
restrição técnica das capacidades; os hashes atestam identidade das entradas.

Freeze às `2026-09-08T11:46:33.577812+00:00`, anterior ao candidato, SHA-256
`0f7ba52eb0ec820f53754274d51cb7ff4d20aab8d284d3c56161ff3d70b3e998`
(`p1313-ab-freeze.json`). L0 normativo (exclui somente a linha canônica
Hash do Código): `553294f33c131121926207fb203c1e4a4174b03e653afe39e5827b3a773fead3`.
Todos os pins protegidos foram revalidados na comparação final; a alteração
da linha Hash do Código permaneceu dentro da exceção previamente congelada.

Medições no HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree
não commitado. Lista exata de arquivos alterados e diff/stat integral nos
campos `start` e `end` dos recibos de execução; nenhum número deste veredito
é atribuído ao HEAD sozinho. Execução candidata entre
`2026-09-08T11:53:11.902307+00:00` e `2026-09-08T11:57:45.194683+00:00`;
273,04830857619527 segundos somados dos subprocessos, conforme elapsed_seconds
do recibo. Comparação às `2026-09-08T11:58:06.511259+00:00`.

| Identidade | SHA-256 |
|---|---|
| Baseline `/dev/shm/p1312-target.B8uLa9/release/typst` | `446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0` |
| Vanilla `/usr/local/bin/typst`, upstream `a51e02804` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| Candidato `/dev/shm/p1313-target.keFg93/release/typst` | `cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce` |
| `p1313-ab-cases.json` | `9708ba0d9633824fb6acb4b7d2c4a84bd186f2975697539ef951b1aedc937ee9` |
| `p1313-ab-runner.py` | `f08455a75909779090aac4ef6d8ab7658cb57926f002c644fa9b06425f26820b` |
| `p1313-ab-freeze.py` | `c693ce2377e716cdeb277587ce9ea0f758efc68198e04a19f50081acdc4477cd` |
| `p1313-ab-baseline-final.json` | `712c79126b0476e9417803ced4bfc595fed6f8b2df5f6e6a6d6019573f48e746` |
| `p1313-ab-candidate-runs.json` | `5b75c1a7346fb92519bcb1dff11cd99c34ef567a7ceb050f2cf6801569c9ad4b` |
| `p1313-ab-comparison.json` | `773f2ffa166980fe9f3267a33a3a386c4eda27cdfed993e2d8d832e5a23da12d` |

## Observáveis e resultado

Os 144 casos geraram 576 expectativas nos perfis default, html, a11y e
html+a11y. As ordens normal, repeat e reverse totalizaram 1.728 comparações:
todas passaram, sem diferenças, observações ausentes ou Unknown. São
27 casts e carriers causais; 21 sucessos Bytes; 3 rejeições Symbol normativas;
14 entradas Bytes sob política legada; 79 controles de preservação.

Valores Bytes cobrem vazio, array/dictionary, delimitador, aspas, campos
vazios, Unicode, CRLF, NUL e rotas With/Args/sink/alias/map. Casts inválidos
exigem diagnóstico completo de tipo público longo e origem em ocorrência
posicional, inclusive named anterior, With, Args, spread, multiline e origem
detached pública por arguments.map. Casos de sucesso e cast usam os
observáveis vanilla medidos; os casos Bytes/cast eram RED no baseline.

Symbol preserva rejeição cristalina com a mensagem normativa aprovada e
origem/trace medidos. Não é classificado como igualdade vanilla, que tenta
abrir o Unicode do símbolo como caminho.

Para erros de parsing/opções Bytes, as expectativas foram derivadas antes
do candidato de Path/Str com os mesmos dados. Apenas o texto da expressão
exibida foi substituído; mensagem, ausência de âncora primária e trace legado
ficaram intactos. Named desconhecido precede cast; opções precedem leitura;
excesso posicional e opção válida posterior a duplicata inválida conservam
os sucessos legados medidos. Essas divergências vanilla não foram corrigidas
nem ocultadas pelo oráculo. Os controles preservam read/P1312, cinco decoders,
encoders, campos, I/O e demais validações delimitadas.

Os quatro sentinelas históricos CSV estão presentes com expressão exata:
`csv(42)`, `csv(bytes("a,b"))`, `csv(sym.alpha)` e
`csv(delimiter: ";", false)`. Os replays históricos mais amplos são da
coordenação e não foram executados ou atestados por este papel.

## Calibração e limites

Uma medição inicial de 142 casos permaneceu preservada em
`p1313-ab-pre-measurement.json`. Antes do freeze, a revisão adicionou os dois
sentinelas históricos exatos faltantes e tornou o arquivo equivalente das
rotas Bytes de sucesso legado igual aos dados da expressão. A medição final
prépatch validou os 144 casos; não houve ajuste motivado pelo candidato nem
revisão focal posterior. A política de duas falhas da mesma causa não foi
acionada; nenhum Unknown foi convertido em sucesso.

A CLI mostra que Bytes com aparência de nome de arquivo são decodificados;
provar zero chamadas World depende dos testes locais e revisão do owner.
Args sintético Rust e carrier explicitamente detached em Rust também exigem
evidência local; arguments.map cobre uma origem detached observável da língua.
Este corpus não inclui arquivo com UTF-8 inválido; erro de campos desiguais
cobre a política legada de falha do decoder. Build, lint, workspace e replays
não integram este veredito independente A/B e permanecem gates próprios.
