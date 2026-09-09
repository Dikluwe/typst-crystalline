# P1321-R2 — GO anterior à ampliação privada de identidade

GO para adicionar somente native_csv ao transporte agregado existente por
function pointer. As duas normas foram revistas, o RED é genuíno e os
oráculos R1 permanecem imutáveis. Não é aceite final nem selo.

Li todo o delta dos dois L0 frente aos textos anteriores já lidos integralmente;
o diff integral prova que as demais seções não mudaram. Loading declara a
única exceção de segundo owner, mantendo sua implementação sem necessidade
de AST/Source. Call_dispatch legitima apenas CSV na allowlist fnptr/With,
preservando ocorrências, preargs, synthetic sem AST e outras identidades.

Baseline R2 SHA-256
`8ca670bb4ab656a29667abd67fc06bc4d227a3dcc0afa9330a5e9ad49537f065`.
RED R2 SHA-256
`13b0dc933bc5778fdd9c4d93c7eb8327296549b3b0169fec1483ed63e43723d5`,
HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093, árvore não commitada com
fontes/diff/stat, executado entre 2026-09-08T20:41:20.713145+00:00 e
2026-09-08T20:43:35.304634+00:00. `cargo test -p typst-core --release
p1321_csv_transport -- --nocapture` compilou e terminou exit 101: um teste
passou e um falhou no span CSV não transportado. Prefixo produtivo é idêntico
ao baseline R2 e estados antes/depois coincidem.

Li os dois testes novos completos: origem agregada versus individual,
empty/populated, With encadeado e preargs preservados; identidade fake chamada
csv/read/json não transportada; encoders/panic ainda transportados. Essa
combinação discrimina captura nominal e expansão indiscriminada.

Freeze sucessor `p1321-ab2-r2-freeze.json` SHA-256
`67c6b92d62018b1cc8673fe28e75d8c55fd28f1c4e15d15ab6a4bb535ecfc8dd`,
todos os pins de inputs/binários conferidos. Normas sem Hash do Código:

- loading: aae08307fdd8602d6a4bd2700d282b89aa35ea5d4e28e189f64ae3bbb110558b;
- call_dispatch: 95c34064e36713e44f239d168029153d3c8010d2af5c9a5984369d61ffc9bfa7.

Casos/runner/expectativas/falhas R1 estão pinados sem alteração. R2 acrescenta
19 controles aceitos em quatro perfis, 76 expectativas sem Unknown; total
129 casos após integração. Li o runner sucessor e as medições math: aliases
reader e reader With alcançam CSV vanilla e exigem origem inteira. A tentativa
exploratória `$csv()$` falha em namespace vanilla antes da chamada, portanto
sua exclusão declarada é legítima e não substitui os controles bilaterais.

Rodar focal de 35 casos em quatro perfis (falhas originais + fronteiras +
novos controles) primeiro. O runner só permite full após focal GREEN no
mesmo binário. R2 deve continuar exato quanto a diagnósticos integrais;
não normalizar falha. Limitação: A/B executado sem atestação de isolamento.
