# P1331 — freeze A/B R2, sucessor somente da fixture

Congelado em 2026-09-09T13:58:40Z. Regime A/B executado sem atestação de
isolamento, sem selo de refinamento. Autor /root/p1331_tests. Valem as
capacidades e restrições do freeze R1; não consultei runtime calc.rs, patch,
recibos GREEN/RED privados ou baseline integral. Nenhum artefato R1 foi editado.

## Medição pública anterior à correção

World::resolve_path em 01_core/src/contracts/world.rs:49-55 retorna Err por
omissão, independentemente do Source estar attached/detached. A construção
native_path em 01_core/src/compiler/stdlib/foundations/path.rs:54-58 delega a
esse método antes de abs. O TestWorld R1 não o implementava. Assim, a fixture
pública de Path podia falhar na construção e não sustentava o fechamento do
caso pretendido. R1 permanece registro histórico; eventual execução de C1 não
substitui a obrigação de novo RED→GREEN com fixture alcançável. O autor B não
leu saídas privadas ou fez ajuste orientado pelo patch.

A autorização pública do root/revisor reabriu somente esta fixture, sob a mesma
norma. Segundo comunicação do root, C1 foi revertido antes de elaborar R2.
Esta é uma declaração causal do integrador, não uma atestação técnica por B.
HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093; working tree não commitado,
inventários e diff/stat da captura original permanecem na baseline pinada.
Nenhuma nova medição do produto ou recaptura CLI foi realizada nesta revisão.

## Correção limitada e integridade

p1331-ab-tests-r2.rs adiciona somente World::resolve_path virtual puro e
p1331_fixture_path_constructor_succeeds_before_abs. A fixture conhece um
ficheiro virtual na raiz Project; usa VirtualPath::new/RootedPath::new,
sem I/O, relógio ou ambiente. A sentinela avalia path("p1331.typ") nos quatro
perfis, exige ausência de warnings, variante Path, raiz Project e caminho
/p1331.typ. Não chama calc.abs: verifica a pré-condição de construção.
A validação compilada independente desse braço é o primeiro gate do root.

O diff após rustfmt edição 2021 contém somente as duas adições. As sete
funções de teste anteriores, todas as expressões, valores e asserções são
idênticas. Agora há oito testes; o aumento é uma sentinela da fixture, não
expansão do corpus de comportamento. Path não foi removido nem enfraquecido.
Sucessor P1328 R1 e módulos P1329/P1330 permanecem intocados.

Runner R2 troca apenas paths de manifesto/expected e hash do manifesto.
expected-r2.json troca somente manifest_sha256; frozen_utc continua a data da
autoria original dos observáveis. O sufixo textual expectations, incluindo
todos os 504 casos/perfis, é byte a byte idêntico a R1:
SHA256 37aa9569226f9a3153dda5180a329d44b87163f323f2038d943af80d3083fe09.
A baseline CLI e os literais de valor/diagnóstico/trace seguem os mesmos.
Não execute novamente --freeze-from: os valores já estão congelados.

| Artefato | SHA-256 |
|---|---|
| L0 calc.md normativo | ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a |
| p1331-manifest-r2.json | 6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9 |
| p1331-baseline-public.json | 621362a26b8d4d2553bbde444dc9bdfea6a0da98496b9e2a0dc9d60faafefe6c |
| p1331-ab-tests-r2.rs | de3d3797484eb721bc3af3e7f9b259db43feca596d00dc9c8e8af2fc0cb91277 |
| p1331-ab-cli-r2.py | c15a1694866804ba471760e308ce323da7128a3590931e12c0aac060fad884c4 |
| p1331-ab-cli-expected-r2.json | 50f024e8506cbc5bbe3809cd71702d7b68e8cce61fdfd2d557d8dc928d7b8a1c |
| p1331-ab-cli-baseline.json, sem alteração | 6f73013c9e951ee146089eebd2267d412d742eeee051528a7427cea016d980bb |
| p1331-ab-p1328-successor.rs, sem alteração | 994f3dc7493e1230b8ceb41e53ef4c182e58120fe6f4e487637ede014edeea3a |
| freeze R1 | 207ff2040a913c2da429325b566e0923c98215d292993c30f95e51020591bd8a |

## Gates restantes

Root integra cegamente o snippet R2 no lugar de R1, preservando os demais
módulos. Primeiro executa p1331_fixture_path_constructor_succeeds_before_abs
no runtime pré-C; sucesso demonstra construção pela API pública sem abs.
Depois executa novo RED dos mesmos testes comportamentais e GREEN após C.
B não executou cargo e não reivindica que a nova sentinela já passou.

Depois dos gates compilados, executar o runner R2 com outputs novos:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli-r2.py --candidate /tmp/p1331-target.rtY0la/release/typst --output 00_nucleo/diagnosticos/p1331-ab-cli-normal-r2.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli-r2.py --candidate /tmp/p1331-target.rtY0la/release/typst --output 00_nucleo/diagnosticos/p1331-ab-cli-repeat-r2.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1331-ab-cli-r2.py --candidate /tmp/p1331-target.rtY0la/release/typst --order reverse --output 00_nucleo/diagnosticos/p1331-ab-cli-reverse-r2.json
```

Custo da revisão B: leitura focal das APIs, uma adição de fixture/sentinela,
um diff mecânico e comparação textual integral de expectations; zero
processos de produto e zero recapturas globais. Ganho esperado é distinguir
construção Path de rejeição abs; ainda pendente de gate compilado. Duas revisões
sem ganho na mesma causa exigem rever observabilidade. Mandatory Unknown
bloqueia; não converter falta de alcance em sucesso. Nenhum congelado será
alterado após este R2. Aguardar recibos CLI públicos para recibo B.
