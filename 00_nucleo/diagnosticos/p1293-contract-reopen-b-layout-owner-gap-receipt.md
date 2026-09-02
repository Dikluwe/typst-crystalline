# P1293 — recibo de reabertura do lote B por owner gap de layout

## Estado e autoridade

```text
status: READY_FOR_COORDINATOR_FIX_HASHES_AND_REPLACEMENT_GATE_SEAL
lot: B-only
contract: unchanged
oracle/RED/gate: unchanged and protected
active-seal: none; predecessor invalidated historically
lot-C/D: forbidden
```

- papel: `autor_contrato_p1293`, autor L0/contratual segregado;
- regime: protocolo completo da skill `tekt-materializacao-segregada`, por
  capacidades e artefatos, sem isolamento técnico de leitura no filesystem
  compartilhado;
- instante da decisão: `2026-09-01T18:45:35-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: não commitada; SHA-256 de `git status --short`
  `1363caab43eb163c174d288cd656966ab169e21877b8698aab64c78f29224b27`;
  SHA-256 de `git diff HEAD --stat`
  `d8ff11ec72970c210e24c462ba68cf9d035ef0de30a7402b0895f490c1c0c9dc`.

Entradas públicas congeladas:

- manifesto predecessor: SHA-256
  `9891106d0d915f7aa9cd0ba65566e82bf5bf7d450b70f419c3a18b1f6e689499`;
- recibo parcial de implementação B: SHA-256
  `d41baadf3bc57a712d6906e27ce00b49797be4ebe5fb3fe0c65f9950714fbf3d`;
- medição vanilla P1293: SHA-256
  `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`;
- selo predecessor: SHA-256
  `8f6ac395dbcb337911f320c5f7ecce72fd4518a0f4c4234dfc338d82f8f1acf6`;
- contrato canônico: SHA-256
  `2bb10fa308558a82b75acabb1a188c0832ad6708e5ff0b8246af3ced52707072`;
- oráculo/RED protegidos, verificados somente por hash:
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5e` /
  `91f3e53f1f4b01b10522821018a704aec658f7c8c37b9dae3eb92c2ca7114b5e`.

Não foram lidos ou editados código produtivo candidato, oráculo protegido,
RED/gate privado, harness, testes, ataques ou veredito. A única fonte de código
lida foi o baseline vanilla ratificado em quarentena, para fechar as fórmulas
já localizadas pela medição pública. Não se executou o oráculo.

## Medição antes da decisão

### `binom`: owner comum de sequência

O recibo parcial (`p1293-implementation-receipt-b.md:85-104`) mede:

| Vetor | Vanilla | Candidato | Resultado |
|---|---:|---:|---|
| display, três lowers | `46.797666667 × 24.057` | `54.101666667 × 24.057` | RED somente largura |
| inline qualificado | `30.3325 × 7.8815` | `39.6187 × 7.8738` | RED largura |

O repr já converge e o envelope permanece `MathFrac(line=false)` com
denominator `MathSequence` e separadores `MathText(", ")`
(`p1293-implementation-receipt-b.md:123-148`). O primeiro ponto causal é
`compiler/math/layout/mod.rs:648-660,1396-1445`: o espaço terminal é medido
como texto e o mesmo separador participa dos gaps. `frac.rs:51-64` apenas
consome a largura pronta.

A fonte vanilla ratificada, SHA-256
`115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112`,
insere somente o símbolo vírgula entre lowers
(`typst-library/src/math/ir/resolve.rs:738-747`); o spacing pertence ao run.
Logo o owner mínimo é `_comum.md → mod.rs`, sem mudar payload, repr ou
`spacing.rs`.

### `attach`: owner fino de composição pre/post

O mesmo recibo (`:106-153`) mede altura equivalente com excesso horizontal:

| Sonda pública própria | Vanilla | Candidato |
|---|---:|---:|
| display, sintaxe, seis slots | `22.1925 × 19.4843` | `28.2216 × 19.4843` |
| inline, qualificado, seis slots | `20.7614 × 9.7735` | `26.3912 × 9.7658` |

Os dois fingerprints congelados continuam os do receipt vanilla
(`p1293-vanilla-measurement-receipt.md:206-217`). A medição localiza a causa
em `compiler/math/layout/attach.rs:381-420`. A fonte vanilla ratificada,
SHA-256
`d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b`,
fixa em `scripts.rs:149-187,220-265`: `space_after_script` entra uma vez por
pre/post-script presente; top/bottom de cada lado competem por máximo; largura
final é `pre + base + post`. A redação P1293 anterior, que remetia apenas às
fórmulas vigentes, estava desatualizada face a essa divergência medida.

### Classificação e refutadores

- ADR-0107: morfologia, ordem da pontuação e geometria horizontal são língua;
  hashes SVG, helpers e estrutura Rust são mecânica usada somente para medir;
- ADR-0108: as duas decisões acima seguem as medições `file:line`, não nomes
  de função ou coordenadas de fixture;
- inferência `binom`: o excesso é dupla materialização do espaço terminal;
  refuta-a falta de convergência dos dois vetores ou regressão de texto/markup
  não-separador;
- inferência `attach`: o excesso está na composição pre/post; refuta-a
  divergência após a fórmula ratificada ou necessidade de outro consumer;
- qualquer refutador aciona parada antes de ampliar heurística, fórmula ou
  owner.

## L0 atualizado primeiro

| Owner 1:1 | SHA antes | SHA após autoria | Consumer atual, somente hash |
|---|---|---|---|
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `903788030729e81faa7cf08622a673454f29301cfb99b8fa7c2adf317db31746` | `b5b95df2f8260c2f8eaf7c98e4d149fb1ae43193395799540d14abc7b9c05bfe` | `01_core/src/compiler/math/layout/mod.rs` `8724834cafa14bbfac12ca8db2d1fcabcbbc0b067657c2abc5c08f4daa6928c6` |
| `00_nucleo/prompts/compiler/math/layout/attach.md` | `c9b6e3b3eb5724226ffe65597d0e66d7ab487f41907f2f9cf58cbad54381e9aa` | `e19d4cb4866cc199fe534161d221145d0dae70cde3dcb1b1fa3d355a36a62120` | `01_core/src/compiler/math/layout/attach.rs` `5e3ed835337affe7aa8399d6ad8dc895ad1fe9f34344d23f7d367682f395241f` |

`_comum.md` agora contrata uma vista transitória de layout: separador
pontuacional `MathText` seguido somente por whitespace e por irmão material
preserva a pontuação, mas não mede o whitespace e o gap de classe como duas
contribuições. Não há regra nominal `binom`, constante de fixture ou mudança
de payload.

`attach.md` agora contrata as larguras pre/post pela fórmula vanilla: spacing
uma vez por script material, máximos top/bottom por lado, correção itálica no
post-subscript e `total = pre + base + post`. Slots ausentes/vazios não ganham
spacing; vertical, cramped, limits e fase ficam intactos.

Ownership permanece 1:1. Os Núcleos Tekt pinados por `_comum.md` não mudaram;
V26 valida os pins e o DAG. Não foi criado owner 1:N nem novo Núcleo.

## ADR-0127 e contrato protegido

A mudança é correção interna de paridade em funções e owners existentes. Não
adiciona campo, entidade, trait, assinatura pública, default, compatibilidade
ou mudança de fase. Classificação: **fluxo contínuo**, sem novo gate humano;
coordenador deve ressellar a linhagem e repetir gate discriminatório antes de
um selo substituto.

O contrato canônico já exige os oito B-P07 e não muda. Oráculo, RED e política
`Unknown` permanecem congelados; `Unknown` nunca conta como sucesso. Se a
implementação exigir `spacing.rs`, `frac.rs` ou qualquer outro owner, deve
parar e reabrir novamente.

## Invalidação histórica do selo

O selo ativo predecessor
`8f6ac395dbcb337911f320c5f7ecce72fd4518a0f4c4234dfc338d82f8f1acf6`
foi marcado inativo em `p1293-contract-seal.json`; o arquivo após essa
invalidação tem SHA-256
`c5c98761417f0c3b80ba2014cdd26d2500c9560dd8e884d5e6028b0d191cc912`.
Motivo: `mod.rs`/`_comum.md` não constavam da autoridade e `attach.md` precisava
fechar a composição horizontal antes de nova escrita. Nenhum selo substituto
foi emitido. B continua não aprovado; C/D continuam proibidos.

## Gates e drift esperado

Executados sem escrever código ou headers:

| Gate | Resultado |
|---|---|
| `crystalline-lint --checks V5 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --checks V15,V26 --fail-on warning .` | PASS, zero violações |
| `crystalline-lint --fix-hashes --dry-run .` | PASS, exatamente dois drifts |

Dry-run exato:

```text
attach.rs old=d0e91016 hash-a=050c25a2 hash-b=d3ac5492
mod.rs    old=27d922bd hash-a=4a1a2d60 hash-b=ecc6209c
```

Embora o check V5 isolado reporte zero violações, o dry-run canônico ainda
planeia exatamente os dois ressellos de linhagem acima. Qualquer outro drift
ou falha V5/V15/V26 é blocker.

## Allowlist proposta para o próximo selo serial B

Escrita mínima:

1. `01_core/src/compiler/math/layout/mod.rs` — somente a normalização
   estrutural/spacing contratada;
2. `01_core/src/compiler/math/layout/attach.rs` — somente a composição
   horizontal pre/post contratada;
3. `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md` — evidência,
   hashes e capabilities; não é aprovação.

Todos os demais consumers B já materializados ficam congelados para regressão,
sem nova autoridade de escrita. `spacing.rs`, `frac.rs`, testes, prompts,
manifesto, selo, contrato, oracle, RED/gate, C/D, ataques e veredito ficam fora
da allowlist do implementador. O próximo selo deve exigir os oito B-P07
bilaterais, regressões de morfologia/diagnósticos/spans, V5/V15/V26, dry-run e
parada para julgamento independente.
