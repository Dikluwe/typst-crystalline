# P1288 — recibo independente de contrato observável

## Regime, fase e alcance

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Fase: autoria independente do contrato, posterior ao baseline vanilla e ao
  refinamento final de Núcleo + L0, anterior a candidato, oráculos, campanha
  adversarial, selo e veredito.
- Autoridade: agente `/root/p1288_contract2`, papel **autor de contrato**.
- Escrita autorizada e realizada somente em
  `00_nucleo/diagnosticos/p1288-manifest.json` e neste recibo.
- Não foram lidos nem executados candidato, código P1288, harness ou oráculos;
  nenhum ataque foi executado e nenhum veredito funcional foi emitido.
- O checkout é compartilhado e a sandbox tinha capacidade física de leitura
  mais ampla que a allowlist do papel. Portanto a alegação proporcional é:
  **segregação por papel, capacidade protocolar, ordem e hashes, sem atestação
  de isolamento ambiental forte**.

## Proveniência congelada

- Instante de captura: `2026-08-31T11:06:12-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Árvore: working tree não commitada e compartilhada. O estado global não é
  convertido em entrada do contrato; a reprodução usa a allowlist e os hashes
  no manifesto.
- Vanilla ratificado: upstream/main `a51e02804`.
- Binário vanilla: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Baseline canônico:
  `lab/parity/matrix/p1288-vanilla-baseline.json`, SHA-256
  `057bab7534c4855b0b47cc7cd05e243058d8b39407e30c2b3ee478f04d24c377`.
- Receipt vanilla final: SHA-256
  `06a32ad3a77cc710a4ba1c54330325d0cc5a788ba2378cf0e6fda32269c5df80`.
- Receipt final de Núcleo + L0: SHA-256
  `b259334c262683a2e62979b67cbf3b09f57f65ac9e07c4e5c504b6dda5436c6b`.
- Passo autorizado: SHA-256
  `3858860e2d3e9916b051dfbbea0ff66009382bb5daa3fcec8025e9a2492a3e28`.
- Núcleo `compiler-feature-gates`: SHA-256 raw
  `9d7509d01a00589f48d7599d39921f017cc141cf45a8786de8fa73b65abe8de9`;
  hash efetivo Tekt
  `59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9`.

O manifesto registra individualmente os 15 L0s finais, seus hashes, seus
papéis causais e as capacidades de baseline, L0, contrato, oráculos,
adversário, implementador e verificador.

## Lattice de decisão

Cada observável recebe exatamente um estado:

- `Preserved`: as duas identidades válidas executam e o observável registrado
  é equivalente no nível de linguagem, CLI pública, morfologia ou estrutura
  PDF indicado pelo contrato;
- `Violated`: uma execução válida ou mutação fornece testemunha decisiva de
  divergência, ativação proibida, estrutura ausente/incorreta ou diagnóstico
  público incompatível;
- `Unknown`: identidade, proveniência, parser, fixture, construção, ferramenta
  ou execução opaca impede decisão sólida.

`Unknown` nunca vira `Preserved`. Caso desligado pelo perfil não recebe crédito
de `MATCH` nem `Preserved`. Incapacidade do candidato no perfil ativo é
`Violated` quando existe testemunha funcional decisiva e `Unknown` quando a
execução é genuinamente opaca; nunca é “desligado pelo perfil”. Somente casos
deliberadamente opacos podem satisfazer um resultado esperado `Unknown`.

## Perfis congelados

| Perfil | Flags | Obrigação |
|---|---|---|
| `default` | nenhuma | `html` e `a11y-extras` desligados; bindings gated ausentes; zero crédito de paridade pelos casos desligados |
| `html` | `--features html` | `html` é módulo; o trio PDF permanece ausente |
| `a11y-extras` | `--features a11y-extras` | o trio PDF existe integralmente; HTML permanece desligado |

Repetição, as duas ordens, flags separadas e forma com vírgula devem produzir o
mesmo set e payload determinístico. Feature desconhecida é erro de argumentos,
nunca disabled. `bundle` permanece `Unknown`/scope-out e não pode ativar outra
feature.

## Contrato observável congelado

Aceitação futura exige `Preserved` em todos os observáveis obrigatórios:

1. default vazio, gates ausentes e ortogonalidade completa entre HTML,
   `a11y-extras`, target, formato, `PdfTags` e `StreamMode`;
2. trio indivisível `pdf.table-summary`/`pdf.header-cell`/`pdf.data-cell`;
3. composição de flags idempotente e diagnóstico público de feature inválida;
4. `pdf.table-summary` substitui somente o summary; string vazia permanece
   presente; omissão válida remove `/Summary`;
5. **`summary: none` explícito é erro**
   `expected string, found none`; somente omissão produz ausência interna;
6. os restantes casts e diagnósticos de summary, header e data correspondem
   ao baseline final, incluindo level positivo, scopes fechados e `cell`
   posicional;
7. conteúdo cru normaliza para célula default; `table.cell(...)` preserva
   campos e spans; classificação explícita Header/Data vence a derivação da
   linha;
8. tabela simples produz `Table > TR > TD`; header automático uniforme produz
   um `THead` e um `TBody`; linha explicitamente mista conserva `TR` direta;
9. Data explícita dentro de header permanece `TD`;
10. scope Row/Column/Both, rowspan/colspan e relações por level são preservados;
    level não aparece como atributo numérico;
11. tags enabled produzem uma única `StructTreeRoot`,
    `/MarkInfo << /Marked true /Suspects false >>`, marcado balanceado, MCIDs
    determinísticos e ParentTree sem órfãos;
12. tags disabled omitem toda estrutura de tabela e marcação sem mudar texto,
    páginas, boxes ou geometria;
13. MCIDs reiniciam por página; `StructParents` é próprio por página; arrays do
    ParentTree seguem a ordem local dos MCIDs;
14. multipágina conserva uma única identidade lógica, sem duplicar THead, TH,
    IDs ou relações por causa do header visual repetido;
15. texto, boxes, raster e outros targets permanecem separados da estrutura
    acessível; igualdade HTML/SVG/PNG vale somente para o par de fixtures
    congelado;
16. repetição e ordem forward/reverse produzem o mesmo resultado e todos os
    totais fecham exatamente por classificação.

Tecnologia assistiva real, navegação por screen reader, reflow, PDF/UA,
certificação e acessibilidade geral têm resultado exigido `Unknown` neste
contrato. Objetos/relações que as ferramentas registradas não decodifiquem
também permanecem `Unknown`. A warning vanilla `BorderColor` é preservada e
não é normalizada para sucesso.

## Obrigações adversariais, não execução

Este papel não criou mutantes nem executou campanha. O manifesto congela como
obrigações mínimas os 22 ataques do Passo 1288: defaults indevidos, ativação
cruzada/implícita, flag ignorada, trio parcial, bindings sem gate, stubs que
descartam metadata, perdas ou trocas de summary/classe/scope/level, MCID
duplicado, ParentTree/StructElem órfão, tags quando disabled, alteração visual,
crédito indevido de perfil desligado, feature inválida escondida, divergência
forward/reverse e drift protegido ignorado.

Cada mutante válido deve receber `Violated` com testemunha. O gate exige
`mutation_score = 1.0`; qualquer mutante válido sobrevivente impede o selo.
Casos opacos deliberados devem receber `Unknown`; mutantes inválidos ficam fora
do denominador.

## Invalidação e estado causal

Qualquer mudança no manifesto, passo, baseline, receipt vanilla, receipt de L0,
Núcleo, L0s, fixtures ou identidade vanilla invalida o contrato a partir da
primeira fase afetada. Também invalidam a cadeia uma capacidade usada fora da
allowlist, edição de artefacto protegido pelo papel que o julga, ou promoção de
`Unknown`/disabled a sucesso.

Este receipt é contrato candidato congelado para a próxima autoria independente
de oráculos e campanha discriminatória. Não é selo, certificado, aprovação do
gate humano, implementação nem veredito de refinamento.
