# P1331 — gate RED compilado

Revisor `/root/p1331_review`, A/B sem atestação técnica de isolamento;
sem refinamento. Este parecer segue `p1331-review-pre-c.md`. Não escrevi
testes, implementação ou expectativas; nenhuma saída privada foi entregue
ao autor A/B.

Recibo avaliado: `p1331-unit-red.json`, SHA-256
`ea4f7d53c78727266b2252d1b7e3109dcda8baf110f6367a870160ee312e0be7`.
Execução de `2026-09-09T13:48:55.244507+00:00` a
`2026-09-09T13:50:48.669042+00:00`, sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada
com before/after e diff/stat integrais no recibo. Manifesto
`837bb832ce13f0686a438f672c6fe001224738a15b1a369efcc6f7533b7db141`.

Comando: `cargo test --release --locked -p typst-core compiler::stdlib::calc::p13 -- --nocapture`,
com target `/tmp/p1331-target.rtY0la`. O compilador terminou o perfil release
e iniciou o executável de testes: não se trata de falha de compilação.
Resultado: 30 testes executados, 24 passados e seis falhados, exit 101.

As falhas são assertions das obrigações novas de diagnóstico: cinco testes
P1331 de fallback e o teste P1328 cujas duas expectativas foram migradas.
Os witnesses compilados divergem no texto/nome longo exigido pelo L0.
Não há falha de construtor ou API inexistente entre essas falhas. Os testes
P1329/P1330 e os controles novos de guards/espécies numéricas passam.
Como testes densos interrompem na primeira assertion, o RED não prova que
cada subcaso falhou; os mesmos testes completos ainda precisam ficar GREEN.

Conferi programaticamente igualdade do inventário produtivo antes/depois
do RED e com a saída da integração congelada. HEAD e index permanecem
iguais ao baseline; arquivos produtivos fora do owner/L0 preservam seus
hashes. Os hashes de todos os artefatos congelados continuam iguais à
integração. A revisão pré-C já comprovou runtime baseline exato e snippets
P1329/P1330 intactos, e não houve mutação entre os estados. Fmt integrado
também terminou exit 0.

Veredito: RED_ACCEPTED. C pode começar exclusivamente no fallback e com
as entradas congeladas vigentes. Este gate não aprova o resultado futuro,
não atesta isolamento e não fecha P1331.
