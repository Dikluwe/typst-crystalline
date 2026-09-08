# P1311 — GO independente do contrato A/B antes do patch funcional

Revisor `/root/p1311_review`, regime A/B sem atestação técnica de isolamento.
Conclusão: **GO prépatch funcional**, sujeito a RED→GREEN e gates finais.

Em `2026-09-08T00:54Z`, no HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, conferi o amendment P1311 do L0
integral anteriormente lido, o passo explicitamente autorizado
`00_nucleo/materialization/typst-passo-1311.md`, a suíte, o builder/comparador
e a medição bilateral final. O consumer tinha somente header ressellado e
testes novos, sem diff funcional. Seu SHA-256 observado era
`54768e40e2927b5d107b7eec3fe4ca6b1acc113786a3518aa160f246f0902279`.
O baseline completo e os resíduos P1310 estão em `p1311-baseline.json` de
SHA-256 `2338de6be4567c9ea03be335c5935832241d8010272b10eb29c1ca858c9af753`.

Entradas protegidas conferidas:

- Freeze `p1311-ab-freeze.json`, SHA-256
  `37db1eee5ae27d597d11df9534574008b46a31fae56bbdd77b7135c772663b3a`.
- Medição `p1311-ab-baseline-final.json`, SHA-256
  `6428031aa5309044bcead62e4fbcb02eab881600303abef7fdd17748bb73c590`.
- L0 `00_nucleo/prompts/compiler/eval/bindings/field_access.md`, SHA-256
  bruto `5d19d634405c2a414a4cb7a1b133a202d99fd51b0478e7f96da4f0766f770758`;
  normativo `c8f60e7a400ad1585008385d352f0f7b11d2e446918065e570868beaef0e1d51`.
  A única normalização permitida remove exatamente a linha `Hash do Código`,
  cuja atualização mecânica foi prevista antes do candidato.

Recalculei hashes de todas as entradas do freeze, unicidade e completude das
chaves, identidade dos argv com os casos, e origem literal de cada expectativa.
São 55 casos: 24 targets de paridade vanilla e 31 controles de preservação
baseline, nos quatro perfis; 220 expectativas vêm de 440 runs bilaterais.
Binários da medição: vanilla SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
baseline P1310 SHA-256
`7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146`.
Hora exata, HEAD e diff/stat de cada execução estão no recibo citado.

A auditoria encontrou antes do freeze que `text.nope` fora classificado como
controle embora `text` seja Native com namespace None. O autor independente
corrigiu text e seu With para target antes da implementação. `assert`,
`table` e decoders com namespace Some preservam baseline; closures nomeadas e
anônimas, inclusive With, também. O corpus verifica nome qualificado/alias,
NativeWithEngine, callee, multilinha e fronteiras Dict/Type/Content/Module,
float e PDF. `eval` interno mantém a projeção causal externa medida.

O comparador exige igualdade literal de exit/stdout/stderr, incluindo mensagem,
span renderizado, hints e traces; nenhum adapter descarta diferenças.
Também exige as chaves exatas das três ordens. Timeout/crash ausente não gera
GREEN: o executor aborta sem recibo de sucesso. Não há equivalência geral nem
selo de refinamento ou score de mutação. Element/Plugin custom e namespaces
Some vazios são cobertos por testes locais do owner, sem atribuir-lhes autoria
A/B independente.

O amendment é compatível com as expectativas congeladas e substitui
expressamente a proteção anterior de targets não Module só para a categoria
medida. Nenhum achado prépatch pendente. O revisor não editou L0, corpus,
oracles, consumer ou testes; este GO não substitui revisão final do candidato.
