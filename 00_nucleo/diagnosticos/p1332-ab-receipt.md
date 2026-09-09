# P1332 — recibo independente A/B: PASS

Auditoria em **2026-09-09T14:52:22.552640+00:00**, restrita aos artefatos públicos autorizados. Regime executado sem atestação de isolamento e sem selo de refinamento. Não foram lidos runtime, patch, baseline privado ou recibos privados; build e testes compilados não constituem prova deste recibo.

Entrada autoritativa: [freeze](p1332-ab-freeze.json), SHA-256 `7e2e2350cefbf5316131cd8ff9ccc0baf53be0efb36d3a0a4ffe059990c21719`. L0 normalizado: `ac26a9a0492f250e4a59175256b8b1b7a4a173546dc483131a704f01262b84ad`; manifesto: `b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`. Todos os hashes de inputs, snippets, runner, baseline e expectativas enumerados no freeze continuam iguais.

Foram recomparadas literalmente **616 células × 3 = 1.848 observações candidatas**, usando exclusivamente `exit`, `stdout`, `stderr` completos das [expectativas](p1332-ab-cli-expected.json), sem normalização e sem confiar no booleano de aprovação do runner. Resultado: zero divergências e zero Unknown. Chaves únicas, cobertura integral, expressão, perfil, argv, classificação e expectativas incorporadas foram conferidos. Normal/repetição seguem a ordem congelada; inversa reverte os casos preservando a ordem dos quatro perfis. BASE, CANDIDATE e VANILLA são determinísticos entre as três execuções; BASE/VANILLA também coincidem integralmente com a medição pré-C congelada.

Proveniência: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado; cada recibo contém UTC, diff/stat exato e argv. Execuções iniciadas às 14:48:26.841466, 14:48:34.825241 e 14:48:40.473767 UTC. Hashes dos binários presentes no disco foram recalculados: CANDIDATE `dfe7c3ab89eaa145d79e5b56ac72c657e49c6c994f9a88a4338915808ec0e8ea`; BASE/VANILLA conferidos contra os pins completos nos recibos e manifesto público; vanilla ratificado a51e02804.

Por execução, BASE coincide com VANILLA em 444 células; CANDIDATE em 520. Há **76 convergências novas**, nas rotas de trace content/misto/overflow/fallback, import/arguments, warning e UTF-8. CANDIDATE preserva BASE integralmente em 472 células e altera somente os campos autorizados em 144: 128 nomes de trace e 16 interpolações primárias de gradient/show. Dos 144 ajustes, 84 pertencem ao corpus histórico e 60 aos casos novos.

Restam **96 células de dívida**: guards/aridade/precedência (52), sqrt (8), multiplicação float×fraction anterior a abs (4), literal mínimo/parsing (4), construção path (4), import em math (4), gradientes (12), show (4), where (4). Em `{import calc: abs; $abs(-1)$}`, BASE/CANDIDATE produzem equation/math.delimited; VANILLA rejeita content. Isso não é ganho nem justificativa para mudar fase. Gradientes/show apenas interpolam `abs`; seus diagnósticos completos continuam diferentes do vanilla. `.where()` permanece literalmente igual ao BASE. Preservação observada de identidade, repr e resultados não atesta paridade geral.

Recibos públicos e respectivos SHA-256:

- [normal](p1332-ab-cli-normal.json): `9d7b8bac08f04d498bfaf72d22b467858daae91dec5e80f8dba24e08f4a95d0e`.
- [repetição](p1332-ab-cli-repeat.json): `f9eb8befa9a9ba6fa6ae30fbe8f2d964b3e70cf6a7dc94c4e744bccc06cd9dec`.
- [inversa](p1332-ab-cli-reverse.json): `5bf55fb972768c8ad5d09bca078fa243e0957fcf99bbf4f8ddad808579077ba3`.
