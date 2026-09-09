# P1325 — classificação A/B anterior à candidata

Regime: ensaio A/B executado sem atestação técnica de isolamento; sem selo de
refinamento. Autor de testes recebeu somente L0, núcleo pinado, manifesto,
contratos públicos e mocks auxiliares; não leu o owner nem o recibo contendo
seu código. Ambiente compartilhado não impõe tecnicamente esta restrição.

A medição `p1325-ab-baseline-cli.json`, SHA-256
`e5251512b942c1df1e02e9fb66695e03aa1c92291c8ccb789341d338781d2b10`,
registra horários por processo, HEAD, diffstat integral, comandos, stdout,
stderr, exit e hashes dos dois binários. Produziu 224 processos de baseline,
28 casos nos quatro perfis, dois produtos. Nenhum exit fora de 0/1.

Classificação das 112 comparações antes de C, individualizada integralmente
em `p1325-ab-expectations.json`:

- 44 diferenças de âncora esperadas RED, com mensagens iguais: Dict, raw
  Content e Float diretos, aliases, multilinha, campos longos e Unicode.
- 4 resíduos `[x].text`: baseline cristalino erro, vanilla valor `"x"`.
  Somente a âncora do erro cristalino deve mudar; não contar como paridade.
- 52 controles já coincidentes: valores, morfologia, float/is-nan, chamadas,
  métodos, módulo, nativa, ordem causal e gates PDF/hints nos quatro perfis.
- 12 resíduos em tipos fora de escopo: integer, string e closure mantêm
  exatamente suas saídas baseline, inclusive spans divergentes do vanilla.

Todas as expectativas candidatas comparam exit/stdout/stderr integralmente.
Nenhum caso obrigatório Unknown é convertido em sucesso. O comparador exige
um registro para cada produto, caso, perfil e ordem normal/repetida/inversa.

A tentativa `typst --color never eval --format json --in
00_nucleo/diagnosticos/p1325-ab-located.typ
'query(heading).first().nope'` no cristalino terminou exit 2,
stderr `error: unexpected argument '--in' found` seguido de ajuda de uso.
É falha de instrumentação, não evidência de preservação. A fronteira
LocatedContent foi transferida ao teste Rust com AST real, Scopes e Engine,
snapshot Some e None. O teste usa contratos existentes e não lê o owner.

Revisão R1: a primeira compilação do snippet R0 não executou assertions,
pois `Route` foi importado do módulo errado. Causa pública comunicada pelo
revisor: E0432; Route pertence a `entities::world_types`. R1 corrige apenas
essa instrumentação no conjunto original e acrescenta a fronteira Located.
R0 permanece preservado. Uma revisão sem ganho na causa de compilação;
orçamento máximo de duas sem ganho na mesma causa antes de reabrir desenho.
Rustfmt aplicado antes de cada freeze. O root integra os bytes cegamente;
o implementador não recebe assertions privadas.

Foco executado antes do corpus integral: `p1325-ab-focal.json`, SHA-256
`4a90f5f7eeaa25056f4a8e511618d48ea8f40152fc21c435ef9fa80c02c643d8`.
O baseline esperado exige RED real nas três categorias; erro de compilação
não satisfaz esse requisito. C somente após esse gate e freeze final.
