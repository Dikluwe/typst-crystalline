# P1312 — recibo independente A/B

Regime: **executado sem atestação de isolamento técnico**. Testador
`/root/p1312_tests`, contexto novo limitado ao papel. Nenhum código candidato,
diff funcional P1312 ou teste do implementador foi lido. Os documentos baseline
e measurement autorizados contêm diffs históricos P1310/P1311. Capacidade real
de filesystem compartilhado excede a allowlist declarada; os hashes identificam
entradas e ordem, não provam isolamento.

## Entradas e ordem prépatch

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
Lista e stat exatos no campo `start` de `p1312-ab-baseline-sealed.json`.
Medição de 2026-09-08T01:37:38.979420+00:00 até
2026-09-08T01:38:06.384857+00:00. Baseline executável
`/dev/shm/p1311-target.JE8Cyy/release/typst`, SHA-256
`4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb`.
Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Freeze `p1312-ab-freeze.json`, SHA-256
`440d7e744067dffab23371700bf421bf6dd2830c7d8a60cf26ccc3b6ea54f725`,
gravado em 2026-09-08T01:39:22.046026+00:00 antes do patch funcional.
Revisor distinto emitiu GO em `p1312-review-prepatch.md`, verificando os pins
e a ausência de patch funcional contra a identidade exata do baseline.
L0 normativo SHA-256
`1abef2ea2a2a18076534a43d8a1e74197b14fab40afd60e8649acd0d04965807`:
exclui somente uma linha canônica `Hash do Código`; todos os demais bytes ficam
protegidos. Corpus, runner, freezer, fixtures, medição e baselines são pinados.

## Cobertura congelada

São 70 casos, nos perfis default/html/a11y/html+a11y: 29 targets `read`,
3 expectativas normativas Symbol e 38 controles. A medição bilateral contém
560 execuções. Das 280 expectativas candidatas literais (exit/stdout/stderr),
o baseline viola 128 — 116 targets e 12 Symbol — e preserva os 152 controles.
Os números derivam exclusivamente da medição e freeze acima identificados.

Targets cobrem tipos longos, Bytes vazio/não vazio, alias, With aninhado,
spreads Array/Args, sink, named prefix/suffix, multilinha, primeiro positional,
função guardada e arguments.map com origem detached. Symbol usa mensagem L0
literal e origem/trace públicos previamente medidos; não conta como paridade
com a coerção Symbol→Str do vanilla.

Controles incluem leitura Str/Path, UTF-8/binário, missing, nominal path,
excesso, named desconhecido, encoding e precedência, erro de leitura,
CSV int/Bytes/Symbol/delimiter e positivo, cinco decoders P1310,
encoders e campos de funções P1311. `read(42, strange: 2)` mantém o erro named
prévio do cristalino; `read(42, encoding: 5)` exige o novo cast com origem.
O caso de Path importado expõe erro sandbox prévio no cristalino e é controle
de preservação dessa dívida; positivos Str e Path construído diretamente passam.

## Ajustes de preparação e limites

A primeira tentativa usou `--root`, incompatível com `eval` cristalino; foi
descartada como evidência RED. O runner foi corrigido para cwd da fixture e
caminhos relativos. Um caso dictionary-call foi parentetizado após a medição
indicar que não atingia `read`; a correção foi verificada focalmente antes da
medição final. A persistência herdada `write_text` das medições preliminares
foi substituída por `apply_patch` antes do freeze. Registros preliminares foram
preservados e não são usados para decidir sucesso.

Executar Python com `-B`, sem bytecode. Nenhuma pasta materialization/context
foi acessada. Arquivos escritos somente em `diagnosticos/p1312-ab-*` e fixtures
dedicadas `/tmp/p1312-ab-fixtures/`. Os executáveis RAM foram acessados no host
por `require_escalated`, sob autorização prévia do usuário.

Args sintético Rust sem ocorrências e variantes sem construção CLI exigem
evidência unitária do implementador, sem atribuí-la ao A/B independente.
Este corpus não atesta paridade geral, nem quita CSV/DataSource, coerção
Symbol, erros sandbox ou demais divergências preservadas. Observação obrigatória
ausente, ambígua ou não verificável bloqueia; Unknown nunca vira sucesso.

## Candidato

PASS: 840/840 comparações literais nas ordens normal/repeat/reverse, com zero
falhas e 0 Unknown. São 348 observações targets, 36 Symbol normativas e 456
controles preservados. A repetição e a inversão da ordem reproduzem os mesmos
observáveis congelados; nenhuma expectativa foi adaptada ao candidato.

Candidato `/dev/shm/p1312-target.B8uLa9/release/typst`, SHA-256
`446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0`.
Registro `p1312-ab-candidate-runs.json`, SHA-256
`9d5232a10803f2137642b960b67acf7ce38865c7309434d70cb822d67212c701`, contém
UTC, argv, cwd, HEAD, diff/stat e observações integrais de cada execução.
Comparação `p1312-ab-comparison.json`, SHA-256
`9849b0b27574deb7dcfb7546c3b91f2699e4c7f0ad907dcef7b014933374e44e`, concluída
em 2026-09-08T01:49:13.283353+00:00. Todos os inputs congelados foram
revalidados, incluindo fixtures e L0 normativo. Só a linha de metadado
`Hash do Código` mudou, conforme exceção autorizada antes do freeze.

Comandos reproduzíveis, a partir da raiz do repositório:

```sh
python3 -B 00_nucleo/diagnosticos/p1312-ab-runner.py --candidate /dev/shm/p1312-target.B8uLa9/release/typst --candidate-sha256 446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0 --orders normal,repeat,reverse --output /tmp/p1312-ab-replay.json
python3 -B 00_nucleo/diagnosticos/p1312-ab-freeze.py --measurement 00_nucleo/diagnosticos/p1312-ab-freeze.json --candidate-runs /tmp/p1312-ab-replay.json --output /tmp/p1312-ab-replay-comparison.json
```

Os caminhos de saída de replay devem estar ausentes, pois os scripts recusam
sobrescrever evidência existente. Uma tentativa prematura de comparação antes
da conclusão do runner falhou por ausência do arquivo de resultados; não
produziu recibo nem foi classificada como sucesso. O PASS acima usa somente
o registro completo após conclusão e SHA verificado do candidato.
