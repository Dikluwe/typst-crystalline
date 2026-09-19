# P1462/P1357 — correção canônica do observador de `format.svg.pretty`

**Estado:** CORREÇÃO OBSERVACIONAL CANÔNICA; não é selo nem veredito final

**Papel:** autor independente da correção observacional; não implementador e
não verificador final

**Regime:** protocolo Tekt completo no recorte de observabilidade. A execução
ocorreu em workspace compartilhado e, portanto, a atestação proporcional é
**executado sem atestação de isolamento**.

## 1. Entradas e proveniência

A medição foi feita em 2026-09-19, entre
`2026-09-19T20:11:17.067Z` e `2026-09-19T20:11:20.656Z`, sobre:

- HEAD `68542c0c0d912ea33909209ed7131c83e3418a59`;
- tree `590f85207e4252ba5e543da81fbc1ca5e020c049`;
- `git diff HEAD --stat` vazio;
- quatro outputs de recuperação preexistentes e ainda não rastreados:
  `p1462-recovery-checkpoint.json`, `p1462-recovery-preseal.json`,
  `p1462-recovery2-checkpoint.json` e `p1462-recovery2-preseal.json`.

Pins relevantes:

| Entrada | SHA-256 |
|---|---|
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0121 | `00be2575c57efb572b7935ab0ae9da80d62e66c8739bffbb17994b2f7a50d5ef` |
| ADR-0131 | `2ba422df0147a6f6469e869d91aa07e3ddac102432554c800da9d03e65fad3cc` |
| Passo P1462 | `197d04ab12f6c01476b32d3efb842bf714b7912319a660f1278cf8d3214735e8` |
| contrato R2 | `69cfdc0de39564cf542e2ca7fc99bcb7b80752d10bc8cdce3c809c1d9cf02fa2` |
| recovery checkpoint | `012989ef882940929dfbbce55da758632af79440f19066f12a33985862b8914b` |
| recovery preseal | `2e8703f3af68812a62b9b0cf97f275eb0d768472a4c11f5c7b7751cfcd7169bc` |
| recovery2 checkpoint | `714362c77a60b51b3ad5db2f54fae1c7b6fd54fdef6b03b8047a5e47573b8c98` |
| recovery2 preseal | `4a8f1ee43a1b83dc7d44945c7cf472356f51fc1c3ecfd72a06c5e30f1ce07217` |
| harness P1357 | `f59cc94163c5be064f0bd9eea950e9c51d405e7cac0803fc8682b6532e9dec1b` |
| output recovery | `5a36db97712c371e348e8d94981149cd68e4f70005860ff50be5655826be5132` |
| output recovery2 | `deefe2e09f6b2276185c6d95c7892e2947deb8ee0b787baf042e1577ae77df05` |
| enumerador vanilla | `11f3eab4ccfadc8239cdfc7fecdc87822238e625ff5bf4f755f3eb3b4ef8da13` |
| fonte do enumerador vanilla | `ff7702fb6ebbb7aafb422e94e7f3061d363e50367905fdee00486978c9185c01` |
| enumerador cristalino ligado ao P00 | `d2a7f39be5d6d355d4c155c95d1cceb0b62008e03d4586cd4be6bcbee7e978a6` |
| fonte do enumerador cristalino | `409a9d59381f03f47febf8fd587b1a87fde0472735e9ed77d896cc43c4f76176` |
| binário vanilla ratificado | `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` |
| binário P00 | `2435eb151ffcad3b2b5ec57222ae00d5666f46535b2fd3c0bb12745b158ef3e4` |

O recovery inicial fica preservado como controle histórico: seu enumerador
estava ligado ao core do repositório, não ao P00. A correção abaixo usa o
recovery2, cujo vínculo ao source tree P00 foi provado por manifesto, metadata
e dep-info. Nenhum output anterior foi editado.

## 2. Medição antes da decisão

### 2.1 Resultado bruto do recovery2

O recibo `/tmp/p1462-recovery2-p1357.json` executou 32 subprocessos, todos
com exit `0`, em duas ordens canônicas e duas reversas. Os quatro hashes de
determinismo foram idênticos:
`08e7308c23e39f8093dd0fd01f1c75535bc58925cf93148b13fc6957dea797b0`.
O conjunto de 32 inventários brutos tem índice
`736fcb613358d7cbf9d786ca6b96a4e5110801eb607416f023002b051060a674`.

| Perfil | BOTH bruto | VANILLA_ONLY bruto | CRYSTALLINE_ONLY bruto |
|---|---:|---:|---:|
| default | 2000 | 21 | 43 |
| a11y-extras | 2003 | 24 | 43 |
| html | 2115 | 136 | 43 |
| html+a11y | 2118 | 139 | 43 |

Em todos os perfis, `format.svg` é `BOTH_PRESENT`. A única linha que explica
o excesso em `CRYSTALLINE_ONLY` é `format.svg.pretty`: o catálogo bruto
cristalino a registra como `bool`, membro estrutural de função, enquanto o
catálogo bruto vanilla não cria uma rota separada com esse nome.

### 2.2 Assimetria dos enumeradores

Isto é uma diferença de representação do observador:

- o enumerador vanilla lê parâmetros de função em
  `lab/typst-original/crates/p1140-inventory/src/main.rs:133-146`; no raw,
  `format.svg.params` contém `pretty`, default `false`, named e settable;
- o mesmo enumerador só desce por `Func::scope()` em
  `lab/typst-original/crates/p1140-inventory/src/main.rs:213-223`; esse scope
  não publica `pretty` como rota filha;
- o enumerador cristalino deixa `params: None` em
  `lab/surface-inventory/src/main.rs:171-193` e desce por
  `Func::namespace()` em `lab/surface-inventory/src/main.rs:199-204`; como
  `pretty` está nesse namespace, ele nasce como a rota
  `format.svg.pretty`.

O harness atual conta presença literal dos nomes, sem normalização semântica
(`p1357-harness.py:233-253`). Portanto o `43` bruto é correto para esse
instrumento e deve continuar registrado como bruto.

### 2.3 Sonda direta bilateral pinada

Foi executada a expressão `format.svg.pretty` com `--format json` nos quatro
perfis, contra os dois binários pinados acima. Para cada perfil, a flag
`--features` foi respectivamente ausente, `a11y-extras`, `html` e
`html,a11y-extras`.

| Perfil | vanilla | P00 |
|---|---|---|
| default | exit 0, `false` | exit 0, `false` |
| a11y-extras | exit 0, `false` | exit 0, `false` |
| html | exit 0, `false` | exit 0, `false` |
| html+a11y | exit 0, `false` | exit 0, `false` |

O output capturado foi exatamente `false\n` em todas as oito execuções
(SHA-256 `2ed27c1421e6928dbe13dbfdb5c59e1045b30341fe7ebe05700006bc5ac572c0`).
A sonda demonstra acesso público bilateral ao mesmo valor; não infere isso da
contagem.

## 3. Classificação ADR-0107/0108

**Medido como língua:** `format.svg.pretty` é uma expressão pública válida
nos dois produtos e avalia para o booleano `false` nos mesmos quatro perfis.
O vanilla ainda declara no próprio raw que `pretty` é parâmetro named,
settable, não requerido e com default `false`.

**Medido como mecânica:** um enumerador codifica esse membro dentro de
`format.svg.params`; o outro o codifica como entrada do namespace da função.
A diferença entre “campo aninhado na entrada dona” e “rota filha” é forma do
catálogo, não semântica, sintaxe ou morfologia da linguagem.

**Inferência marcada:** as duas representações descrevem o mesmo slot público
`pretty` de `format.svg`. Refutam a inferência: valor/tipo divergente na sonda,
perfil divergente, ausência do parâmetro vanilla, owner cristalino diferente,
pin de binário divergente ou qualquer sibling que dependa desta exceção. Sob
qualquer refutador, não há normalização e o gate permanece falho/indeterminado.

## 4. Regra canônica do overlay normalizado

1. Os catálogos, crosswalk e contagens brutos permanecem imutáveis. O raw
   continua a registrar `CRYSTALLINE_ONLY == 43`.
2. O overlay só considera o caminho literal `format.svg.pretty` e somente
   quando, em todos os quatro perfis:
   - o raw cristalino o contém como `bool`, owner `format.svg`, owner kind
     `function`;
   - `format.svg` existe dos dois lados;
   - o raw vanilla contém o parâmetro `pretty` named/settable com default
     `false` dentro de `format.svg.params`;
   - a sonda direta bilateral usa exatamente os binários pinados, termina com
     exit `0` e devolve o mesmo booleano `false`.
3. Satisfeitas essas condições, o overlay marca
   `semantic_vanilla_present = true` para `format.svg.pretty` e classifica a
   linha como **alias representacional coalescido** no parâmetro já contado
   dentro do owner `format.svg`.
4. Coalescência é necessária para a aritmética ser honesta: apenas trocar
   `vanilla_present: false` por `true` numa crosswalk de presença ingênua
   produziria `BOTH +1`. A regra canônica não conta o mesmo slot uma segunda
   vez. Ela remove uma falsa discrepância mecânica, sem criar uma rota pública
   nova.
5. Nenhum outro caminho é normalizado. Em particular, nenhum sibling
   `format.png`, `format.pdf`, `format.html` ou descendente herda a regra.
6. Falta de qualquer pin, perfil, metadado ou resultado direto é fail-closed:
   o overlay não se aplica; não se converte a ocorrência por semelhança de
   nome.

O resultado normalizado e coalescido é:

| Perfil | BOTH normalizado | VANILLA_ONLY normalizado | CRYSTALLINE_ONLY normalizado |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

Esses são exatamente os totais contratuais de O9. A linha crua não é apagada:
o relatório normalizado deve carregar a referência ao raw e a disposição
`SEMANTIC_ALIAS_COALESCED`.

## 5. Por que isto não afrouxa o gate

A correção é mais estreita que ignorar diferenças:

- exige identidade criptográfica dos produtos e instrumentos;
- exige quatro perfis bilaterais, não só um nome parecido;
- exige valor e tipo públicos iguais;
- exige duas evidências estruturais independentes: parâmetro vanilla e owner
  cristalino;
- preserva a contagem bruta `43` para auditoria;
- aplica-se a um único caminho literal e falha fechado;
- não normaliza siblings nem concede crédito a rotas não sondadas.

Pela ADR-0107, bloquear o candidato porque dois enumeradores modelam o mesmo
membro público de modos diferentes seria medir paridade pela mecânica. Pela
ADR-0108, a decisão acima vem depois da medição, explicita língua versus
mecânica e nomeia seus refutadores. Pela ADR-0121, os números têm commit,
tree, estado da árvore, tempo e hashes. Pela ADR-0131, os receipts anteriores
permanecem válidos e imutáveis; esta correção não reescreve seus vereditos.

Este artefato define somente a interpretação canônica do observador P1357.
Não sela P1462, não autoriza implementação por conta própria e não substitui
o verificador final.

