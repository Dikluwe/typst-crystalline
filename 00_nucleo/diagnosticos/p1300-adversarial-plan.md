# P1300 — plano e recibo adversarial P3

## Identidade e regime

- Passo: `P1300`.
- Papel: `P3_adversary`.
- Executor: `/root/p1300_adversary`.
- Regime: protocolo completo de materialização segregada.
- Nível de atestação: **executado sem atestação de isolamento técnico**.
- Motivo da limitação: os agentes partilham o filesystem; a independência foi
  aplicada por contexto fresco, allowlists explícitas e autoridade de escrita,
  mas não pode ser provada por isolamento do sistema de ficheiros.
- Contexto: fresh agent context recebido depois de congelados manifesto e
  contrato; não recebeu conclusões privadas de P2 nem patch candidato.
- Início registado da autoria do artefato: `2026-09-03T19:22:21-03:00`.
- Fecho do corpus e validação JSON: `2026-09-03T19:25:35-03:00`.

## Entradas congeladas e hashes

| Entrada legível | SHA-256 verificado |
|---|---|
| `00_nucleo/diagnosticos/p1300-manifest.json` | `c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8` |
| `00_nucleo/diagnosticos/p1300-contract.json` | `c5eace73e5ff07ab49057ec1741283543408accf132ca2f4431a1ad3827a3e37` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `8115021062602c8a3663797b3fcfe0f17eb423f54a8ae7b437b260804ad58462` |
| `00_nucleo/prompts/compiler/eval.md` | `3bf911a0882e35f713cf8742f70ea921086a9a95ede6b02b8b938ab92becfc96` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `d4d6382d351e084869445fe58e1cb484e762d5b5dfc843b7974cd21149289104` |

Os únicos dados sobre consumers usados foram os hashes já declarados dentro do
manifesto: `eval/mod.rs` =
`cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4`,
`stdlib/color.rs` =
`d40197461efdee472734ef850e7a681821eb490624856bf15510d1b8a9336fed` e
`eval/tests.rs` =
`b6b87d6786874a39b3071c667476d062a4fc41a9298c6519d9434e0ae637df20`.
Nenhum desses três ficheiros `.rs` foi aberto.

Artefato produzido e congelável por P4:

| Saída | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1300-mutants.json` | `cefa2fb00d6ad5adb81783b86b94f2450c9b0366591e42637d926adc7e598501` |

O hash deste próprio recibo deve ser calculado externamente depois desta
escrita; não é autorreferenciado.

## Capacidades e proibições

Read allowlist efetiva no repositório:

- manifesto P1300;
- contrato P1300;
- os três L0 confirmados;
- hashes baseline dos consumers tal como transcritos no manifesto.

Write allowlist efetiva:

- `00_nucleo/diagnosticos/p1300-adversarial-plan.md`;
- `00_nucleo/diagnosticos/p1300-mutants.json`.

Foram respeitadas as proibições de ler ou alterar suite-oráculo P2,
implementação candidata, consumers Rust, testes permanentes, selo, receipts ou
outputs P4–P8. Também não foram lidos `materialization/` nem `context/`, não se
corrigiu candidato, não se editou contrato/oráculo e não se escreveu código de
produção. Não houve staging, commit ou push.

Fora do repositório, foram lidas apenas as instruções obrigatórias da skill
`tekt-materializacao-segregada` e as duas referências operacionais diretamente
exigidas por ela. Isso não acrescentou dados do produto, candidato ou oráculo.

## Estratégia binding-free e interface para P4

O corpus não descreve patches Rust. Cada mutante é uma transformação
declarativa sobre uma cópia de observações públicas completas e congeladas,
endereçada por família, `case_id`, perfil, subtipo de probe, namespace e entry.
P4 pode aplicá-lo a qualquer produto/candidato que satisfaça a interface CLI do
contrato, sem localizar símbolos ou adaptar seletores ao patch.

As operações do pequeno DSL são fechadas:

- `identity`: cópia byte a byte, usada somente pelo controle positivo;
- `copy_public_observables_from`: projeta todos os observáveis públicos
  completos de uma coordenada pública congelada sobre o clone-alvo;
- `replace_public_observables`: troca valores literais de observáveis públicos
  nomeados, preservando todos os demais;
- `replace_capture_fields`: troca campos públicos de uma captura agregada,
  preservando os demais observáveis de processo e de captura.

O runner deve expandir `all` para os quatro perfis pinados, exigir que fontes e
alvos semânticos estejam completos, aplicar a transformação somente ao clone e
submeter o resultado ao classificador do contrato. O campo
`expected_classification` é asserção de gate, nunca entrada do classificador.

## Mutantes e testemunhas esperadas

| ID | Transformação pública negativa | Witnesses obrigatórios |
|---|---|---|
| M01 | publica `hsl` bare e sob `std` | `negative-bare-hsl`, `negative-std-hsl`, todos os perfis |
| M02 | publica `hsv` bare e sob `std` | `negative-bare-hsv`, `negative-std-hsv`, todos os perfis |
| M03 | publica `linear_rgb` bare e sob `std` | `negative-bare-linear_rgb`, `negative-std-linear_rgb`, todos os perfis |
| M04 | remove bare, mas mantém os três sob `std` | os três negativos `negative-std-*`, todos os perfis |
| M05 | mantém default correto e vaza `hsl` em `html`, `a11y` e `html+a11y` | `negative-bare-hsl` nos três perfis divergentes e invariância de perfil |
| M06 | quebra tipo/repr/call/space de `color.hsl` | `qualified-color-hsl` e `space-hsl` |
| M07 | quebra tipo/repr/call/space de `color.hsv` | `qualified-color-hsv` e `space-hsv` |
| M08 | apaga/renomeia a rota `color.linear-rgb` | `qualified-color-linear-rgb` e `space-linear-rgb` |
| M09 | remove `rgb` apenas da projeção bare | agregado bare e repr/call bare de `rgb` na família dos globals ratificados |
| M10 | inventa `linear-rgb` bare e sob `std` | os dois negativos de spelling alternativo; oráculo congelado é precondição |
| M11 | muda o repr público de `color.hsl` para `hsl-mutant` | repr probe de `qualified-color-hsl` |
| M12 | desabilita o constructor HSL em vez de só sua publicação | call probe qualificado e `space-hsl` |
| M13 | faz `color.space` para HSV depender do alias global removido | `space-hsv` e sua identidade pública |
| M14 | altera `red`, a captura `viridis` de `color.map` e a chamada de `color.lighten` | controles de cor predefinida, mapa e operador, todos os perfis |

Todos os 14 mutantes têm `expected_classification = Violated`. Sua validade é
definida exclusivamente na semântica, sintaxe, morfologia, diagnóstico, bytes
de processo e invariância de perfil expostos pela linguagem. Nenhum depende de
igualdade Rust, endereço de função, ordem de inserção, símbolo ou estrutura de
dados interna.

## Controles e ordem

O corpus contém exatamente um controle positivo não mutado, cobrindo todos os
casos semânticos não opacos; cada coordenada expandida deve ser `Preserved`.
Contém também os quatro controles opacos do contrato, cada qual exigindo
`Unknown` com o reason exato:

| Controle | Classificação | Reason exato |
|---|---|---|
| `opaque-timeout` | `Unknown` | `timeout` |
| `opaque-missing-product` | `Unknown` | `missing_product` |
| `opaque-ambiguous-identity` | `Unknown` | `ambiguous_product_identity` |
| `opaque-unsupported-parser` | `Unknown` | `unsupported_parser_construction` |

Cada expansão deve correr no mínimo duas vezes em ordem `canonical`, duas em
`reverse` e duas em Fisher–Yates `seeded_shuffle` com seed inteiro `1300`.
Identidade é a tupla `case_id + profile + probe`; duplicatas não podem ser
sobrescritas ou deduplicadas. Observáveis completos e classificações devem ser
idênticos em todas as repetições e ordens.

## Travas contra apagamento do oráculo e fuga por Unknown

Antes da aplicação, P4 deve hash-checkar contrato, manifesto e suite-oráculo.
Operações atuam somente em clones de observações; não podem alterar expectativa,
expressão, identidade, perfil ou probe. Campo obrigatório não pode ser apagado,
anulado ou omitido. Fonte ou alvo semântico incompleto, selector inexistente,
transformação sem diferença pública e falha de witness são `harness_error` e
bloqueiam o selo.

Em particular, nenhum mutante semântico pode terminar como `Unknown`: isso
conta como sobrevivente e bloqueia o selo. `Unknown` só é aceitável nos quatro
opacos listados, com reason idêntico. O runner não pode usar opaco como fonte de
mutação e não pode injetar a classificação esperada como resultado observado.

## Budget, custo e delta discriminatório

- Revisão adversarial: `1`.
- Budget de revisões completas do manifesto: máximo `2`; esta autoria consome a
  revisão `1`, deixando no máximo uma revisão corretiva caso P4 demonstre uma
  falha focal com hipótese nova.
- Full discrimination runs executados por P3: `0` de até `2` por revisão;
  execução pertence a P4.
- Processos bilaterais executados por P3: `0` de `512`.
- Custo material desta revisão: `14` especificações negativas, `1` controle
  positivo, `4` opacos, `3` ordens, `2` repetições mínimas por ordem e `2`
  artefatos escritos. Intervalo registado da autoria do corpus: `194 s`.
- Delta discriminatório esperado, ainda não medido: `+14` mutações válidas
  corretamente `Violated`, `0` sobreviventes e mutation score de não medido
  para `1.0`; o positivo permanece `Preserved` e os quatro opacos permanecem
  `Unknown` com reason exato.
- Se P4 não obtiver esse vetor, não há selo. A correção deve começar pelo
  recorte focal falho; `Unknown` ou erro de aplicação não contam como ganho.

## Validação reproduzível

O JSON foi validado por parser padrão, sem normalização ou reescrita:

```text
python3 -m json.tool 00_nucleo/diagnosticos/p1300-mutants.json >/dev/null
```

O gate de whitespace/diff aplicável à entrega é:

```text
git diff --check -- 00_nucleo/diagnosticos/p1300-adversarial-plan.md 00_nucleo/diagnosticos/p1300-mutants.json
```

Este recibo não reivindica execução discriminatória nem mutation score medido;
essas decisões pertencem ao P4 com oráculos congelados.
