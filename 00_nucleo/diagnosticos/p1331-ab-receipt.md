# P1331 — recibo independente A/B, C2/R2

Veredito: PASS no fragmento CLI congelado, com integridade e estabilidade.
Auditoria em 2026-09-09T14:13:27Z pelo autor B /root/p1331_tests.
Regime executado sem atestação de isolamento; sem selo de refinamento.

Foram lidos somente recibos CLI públicos normal/repeat/reverse R2, manifesto,
oráculos/freeze e pins permitidos. Não li runtime calc.rs, patch, baseline
privada ou recibos root de unit/build/workspace. Nenhum oráculo ou produto foi
alterado. Este recibo não atesta os gates compilados ou arquiteturais do root.

## Proveniência

HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093; working tree não commitado.
Cada recibo público contém seu git diff HEAD --stat integral, argv e hashes.
As contagens abaixo derivam exatamente dos recibos pinados nesta tabela.

| Entrada | SHA-256 |
|---|---|
| manifesto R2 | 6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9 |
| L0 calc.md, hash normativo | ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a |
| baseline pública | 621362a26b8d4d2553bbde444dc9bdfea6a0da98496b9e2a0dc9d60faafefe6c |
| p1331-ab-tests-r2.rs | de3d3797484eb721bc3af3e7f9b259db43feca596d00dc9c8e8af2fc0cb91277 |
| p1331-ab-cli-r2.py | c15a1694866804ba471760e308ce323da7128a3590931e12c0aac060fad884c4 |
| p1331-ab-cli-expected-r2.json | 50f024e8506cbc5bbe3809cd71702d7b68e8cce61fdfd2d557d8dc928d7b8a1c |
| p1331-ab-freeze-r2.md | d70f809014a912e337006b5751bc058eb1c184879e37b125070456afefc127a7 |
| p1331-ab-cli-normal-r2.json | 02134e5dca4d4a36b102af0966eeb613ed52436c19f024eedb7bd25b6f7470f3 |
| p1331-ab-cli-repeat-r2.json | 6c3acbef28d159068c8414b1a5c800ddfbc33fbb0f21d8f8dede61f19f6e0981 |
| p1331-ab-cli-reverse-r2.json | 2bb25466d3bfa4ba31c7cc6c9f942e32c6512ab2d07b53cb8a9d188600aade5b |

Execuções começaram respectivamente em 2026-09-09T14:10:13.681570+00:00,
14:10:18.495878+00:00 e 14:10:24.610381+00:00. Nos três recibos:

- BASE: /tmp/p1330-target.f0lmDu/release/typst,
  SHA256 6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0.
- VANILLA ratificado a51e02804: /usr/local/bin/typst,
  SHA256 7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8.
- C2: /tmp/p1331-target.rtY0la/release/typst,
  SHA256 a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47.

Os hashes atuais dos três executáveis foram recalculados e coincidem com os
recibos. O hash normativo atual de L0 também coincide. Nenhuma string
--version foi usada como identidade.

## Verificações e resultado

Os três recibos possuem exatamente as mesmas 504 chaves caso/perfil, sem
duplicação ou omissão. Em cada uma das 1512 observações candidatas, exit,
stdout e stderr completos coincidem literalmente com expected R2, sem
normalização. Recalculei a igualdade a partir das saídas, sem confiar apenas
no booleano candidate_matches_frozen_policy registrado.

A ordem normal repetida é idêntica; reverse efetivamente inverte os casos
preservando os quatro perfis por caso. Para cada chave, todas as saídas
BASE/C2/VANILLA são idênticas entre os três recibos. Os pins de runner,
manifesto, baseline pública, expected, L0 e binários são consistentes.

Todos os seis hashes R1 registrados em frozen_r1 do manifesto permanecem
iguais. O sufixo textual expectations de R1 e R2 é byte a byte idêntico:
SHA256 37aa9569226f9a3153dda5180a329d44b87163f323f2038d943af80d3083fe09.
Assim, a correção da fixture não reescreveu as expectativas observáveis.
Não foi necessário repetir captura global de baseline ou recalibrar valores.

| Relação literal por execução | Células |
|---|---:|
| C2 coincide com frozen | 504 |
| BASE coincide com vanilla | 252 |
| C2 coincide com vanilla | 404 |
| C2 preserva BASE | 340 |
| C2 difere de BASE | 164 |
| Divergência BASE→vanilla eliminada integralmente | 152 |
| Modificadas, ainda com dívida externa congelada | 12 |
| Dívidas preservadas integralmente desde BASE | 88 |
| Paridade anterior perdida | 0 |

As 12 células modificadas ainda divergentes são as três rotas de fallback
with-bound/with-nested/arguments-spread em quatro perfis. Seu diff completo
C2 versus vanilla limita-se ao nome externo de trace calc.abs versus abs,
já congelado antes C. Mensagem, âncora e demais saídas seguem o oráculo.

As 100 células ainda divergentes de vanilla não são Unknown oculto:
48 pertencem a rotas com nome externo de trace preservado (content, mixed,
overflow e fallback), 36 a guards named/aridade, quatro a sqrt fora do
recorte, quatro à operação float×fraction anterior a abs, quatro ao literal
mínimo inteiro que diverge no parser e quatro à construção path() sem
argumento. Todas são dívidas explicitamente congeladas e mantidas. As três
ordens reproduzem o mesmo vetor, sem nova causa ou regressão.

## Limites e reprodução

PASS cobre os observáveis desta sentinela CLI e a integridade auditada, não
equivalência funcional geral de abs/calc nem bytes de render. Location
nativa e sentinela de construção Path pertencem aos testes compilados; não
foram inferidas como aprovadas a partir da CLI. O fechamento integral ainda
depende dos gates de unit/RED→GREEN/build/workspace/fmt/lint/linhagem cuja
autoridade e recibos pertencem ao integrador/revisor.

Comandos das execuções públicas estão no freeze R2 e em argv dos recibos.
Para reproduzir a auditoria, indexar expected e cada recibo por (case,profile),
exigir as mesmas 504 chaves, comparar os três campos literais CANDIDATE
(exit/stdout/stderr), depois comparar BASE/CANDIDATE/VANILLA entre ordens.
Comparar também o sufixo textual a partir da chave expectations em R1/R2 e
recalcular SHA256 dos pins acima. Nenhuma transformação de saída é necessária.

Budget desta auditoria: uma passagem local pelos três recibos, inspeção focal
dos diffs de dívidas e verificação dos hashes; zero execuções adicionais de
produto. Não houve revisão de oráculo após C2. Mandatory Unknown bloquearia
o fechamento; nenhum Unknown foi encontrado no fragmento auditado.
