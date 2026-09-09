# P1329 — recibo final A/B de CLI e pins

Veredito: PASS no escopo de CLI literal e integridade dos artefatos congelados.
Autor /root/p1329_tests, UTC 2026-09-09T12:46:49.804Z. Regime A/B executado sem
atestação técnica de isolamento e sem selo de refinamento. Esta autoridade escreveu
os oráculos antes de C e agora compara os recibos fornecidos pelo root; não leu
o runtime, o patch candidato ou recibos protegidos do root, nem reexecutou candidato.

## Identidade e proveniência

Norma L0 R2: 07f83fc24dc13837f54a25f0bec6be20ff495e1f679c4975bec3f1fb583ef5ae.
A normalização remove somente a linha Hash do Código, atualmente 483d55d0.
Todos os pins abaixo foram recalculados dos arquivos presentes e coincidem com o freeze.

- p1329-manifest-r2.json: c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1
- p1329-ab-cli-r2.py: ee0d6a5da2283d50b0591a24db58f93a8300091729b6d6f2ad48e3457b393ab2
- p1329-ab-cli-expected-r2.json: 9501ae018ada17565bfc54227c244bea9af99e3988c9a212b262e92a0ee7c239
- p1329-ab-freeze-r2.md: 1f649122c8a95b2b153db8cb88bb2711188c7a853eed5381f0cc18bb4418eba0
- p1329-ab-tests-r1.rs: 567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838
- p1329-ab-p1328-successor.rs: 1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c
- p1329-ab-cli-baseline-r1.json: b89049333f76a7f49b2828a6ced0fc0c7ba747f6b7ca9ea019fba5a27e0bd91d
- p1329-baseline-public.json: e7af5fca1897431c6060ded8a8072df03e8946aa7be95ab71aa3ebd366d40063

Binários dos três recibos, também conferidos por SHA-256 dos arquivos atuais:

- BASE: /tmp/p1328-target.T7Tg57/release/typst; 94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9
- VANILLA: /usr/local/bin/typst; 7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8
- CANDIDATE: /tmp/p1329-target.bg3p5A/release/typst; 9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec

Vanilla é o build ratificado a51e02804. Os recibos registram HEAD
d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado.
A autoridade A/B não inspecionou diff/stat produtivo; o estado de fonte completo
deve ser vinculado e validado pelo root/revisor. Para a presente comparação,
os binários possuem identidade exata pelos hashes acima, compartilhada nas três
execuções; não se usa a string --version como prova de proveniência.

## Recibos auditados

- p1329-ab-cli-normal.json: SHA-256 a97b10d1689303fe65f214b979d75e2adb443e13c25cef3b92d2eda9dbcfe410; UTC inicial 2026-09-09T12:43:41.496140+00:00; ordem normal; 252/252 saídas conforme expectativa congelada.
- p1329-ab-cli-repeat.json: SHA-256 4394d37c64fe4b878f90c3fcd152eeae55f28646f9d143d700b537371ed6fcc4; UTC inicial 2026-09-09T12:43:47.972380+00:00; ordem normal; 252/252 saídas conforme expectativa congelada.
- p1329-ab-cli-reverse.json: SHA-256 54ee045431a1e09b06460278edc4f1262f43534b4ad4d1eb4069313fa90d20aa; UTC inicial 2026-09-09T12:43:54.304415+00:00; ordem reverse; 252/252 saídas conforme expectativa congelada.

Cada execução contém 63 casos × quatro perfis = 252 chaves sem duplicatas;
as três totalizam 756 observações candidatas. O candidato coincide literalmente
com exit/stdout/stderr congelados em todas elas. As saídas de BASE/VANILLA também
coincidem com a medição pré-C. Repetição e ordem reversa conservam as mesmas saídas
por chave. Os números desta seção decorrem exclusivamente dos três recibos
identificados por hash e hora, com os binários igualmente identificados.

Ordem normal/repetida: ordem integral do catálogo congelado.
Ordem reversa: casos invertidos, conservando default/html/a11y/html+a11y dentro
de cada caso. Expressões e argv foram comparados integralmente, inclusive
--color never antes de eval e as flags exatas dos quatro perfis.
Os campos expected embutidos foram comparados ao arquivo frozen separado.
O veredito não foi inferido dos booleanos candidate_matches_frozen_policy:
exit, stdout e stderr foram comparados novamente por esta autoridade.

## Classificação e limites da alegação

Por execução há 196 observações com igualdade literal ao vanilla e 56 com
dívidas explícitas preservadas. Isso não é uma contagem de paridade semântica geral:
12 controles mantidos incluem math e falhas anteriores à chamada.
A classificação congelada tem 172 correction, 24 correction com dívida de nome
de dispatcher, 12 parity, 32 preserved-debt e 12 preserved-control.

As 24 observações com dívida de dispatcher correspondem a seis rotas em quatro
perfis: With, With encadeado e arguments spread, para Content e Length misto.
A mensagem e as âncoras seguem as transcrições pré-C; o nome externo calc.abs
é preservado. Não houve substituição, filtragem ou normalização do candidato.

As 32 observações de outras dívidas preservam guards named/aridade, rejeições
str/symbol, saturação i64::MIN, calc.sqrt e Float×Fraction. Em
prior-float-fraction, cannot multiply float with fraction ocorre antes de abs.
Em prior-zero-division, cannot divide by zero também antecede a chamada.
Math -2pt falha antes por variável pt desconhecida; math alias -2deg e os casos
históricos exercitam conteúdo sem coerção. Essas falhas anteriores não provam
resultado dimensional de abs.

NaN dimensional nativo verifica somente a obrigação local explicitada no L0 R2.
O domínio vanilla não possui a mesma entrada já construída porque Scalar
normaliza NaN para zero. A dívida anterior também é alcançável pela linguagem
em (calc.inf - calc.inf) * 1deg e * 1%, conforme medição recebida pelo L0;
não é exclusivamente sintética. Nenhuma igualdade geral com vanilla é alegada
para esses estados. Inf é representável, mas paridade exige entradas equivalentes
já construídas. Os recibos CLI aqui auditados não substituem os testes nativos
de tipo/magnitude nem provam o GREEN desses testes.

A auditoria histórica reconfirmou 96 expectativas P1328 literalmente preservadas
e 16 alteradas, exclusivamente length/angle/ratio/fraction × quatro perfis.
Os dois snippets conservam os hashes pré-C. Não há Unknown na comparação literal
deste catálogo; obrigações fora dessa observação continuam sob os gates do root.

## Método reproduzível e delimitação

A auditoria usou Node para ler os arquivos JSON autorizados, mapear por
case/profile, exigir a ordem congelada, confrontar expression/classification/argv,
e comparar triplas {exit, stdout, stderr} por igualdade exata. SHA-256 foi
recalculado com crypto.createHash('sha256') para todos os pins e binários.
O núcleo da comparação independente é:

```javascript
const observable = r => ({ exit: r.exit, stdout: r.stdout, stderr: r.stderr });
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const key = r => r.case + "/" + r.profile;
const frozen = new Map(expected.expectations.map(r => [key(r), r]));
for (const row of receipt.cases) {
  const wanted = frozen.get(key(row));
  if (!wanted || !same(observable(row.results.CANDIDATE), wanted.expected))
    throw new Error("candidate mismatch: " + key(row));
  if (!same(row.expected, wanted.expected))
    throw new Error("embedded expectation mismatch: " + key(row));
}
```

Além desse núcleo foram verificados todos os predicados descritos acima.
Não houve edição de oráculos nem nova execução funcional após recebimento
dos recibos. Somente este recibo foi escrito nesta fase. O veredito é CLI/pins:
build, workspace, RED/GREEN nativo, fmt integrado, linhagem e V5/V15/V26 ficam
sob auditoria separada do root/revisor. Não é certificado de equivalência
geral de abs/calc ou de isolamento técnico.
