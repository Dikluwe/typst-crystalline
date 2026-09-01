# Passo 1288 — perfis de feature e `a11y-extras` PDF sem contaminar o perfil padrão

## Natureza

Documento tático de execução. Não é Prompt L0, não legitima código e não
autoriza antecipar a implementação antes do gate indicado abaixo.

## Objetivo

Separar corretamente feature desligada de divergência de produto nos probes
bilaterais e materializar a feature `a11y-extras` do vanilla ratificado como
perfil explícito, mantendo:

1. o perfil padrão com conjunto de features vazio;
2. `html` desligado por padrão e testado no perfil HTML já existente;
3. `a11y-extras` desligado por padrão e testado num perfil bilateral próprio;
4. `Unknown` e incapacidade do candidato sem promoção a sucesso;
5. semântica real de tabela acessível no PDF, sem stub criado apenas para
   tornar `type(pdf.data-cell)` verde.

O passo não procura `111/111` artificiais. O resultado esperado no perfil
padrão distingue `MATCH`, `DIFFERENCE` e `DISABLED_BY_PROFILE`; uma feature
desligada deixa de poluir a contagem de diferenças, mas também não conta como
igualdade exercitada.

## Estado medido que motiva o passo

Medição local em `2026-08-31T09:45:59-03:00`, HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitida e
compartilhada:

- vanilla `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- candidato `target/release/typst`, SHA-256
  `acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a`;
- relatório P1287, SHA-256
  `7ca2da4a24bcc04e588e28090010062d68c7b32290dda8a02da966da647e9d76`;
- receipt de verificação P1287, SHA-256
  `8a970f4bf25aa0f47317ab85a9270f2268bc65e74cd533ee8e025659dc805b83`.

Comandos e observações:

```text
/usr/local/bin/typst eval 'repr(type(html))' --format json
target/release/typst eval 'repr(type(html))' --format json
```

Ambos rejeitam o binding com o mesmo diagnóstico porque `html` está desligado.
Com `--features html`, ambos devolvem `"module"`. Logo HTML já respeita a
ADR-0128: feature e target são eixos separados, default off. Ativar HTML no
perfil padrão seria regressão, não correção.

```text
/usr/local/bin/typst eval \
  'repr((type(pdf.table-summary), type(pdf.header-cell), type(pdf.data-cell)))' \
  --format json --features a11y-extras
```

O vanilla devolve `"(function, function, function)"`. O candidato rejeita
`a11y-extras` no parser de `--features` e anuncia somente `html`. Portanto
`pdf.data-cell` não é uma lacuna isolada: pertence ao trio coerente
`pdf.table-summary`/`pdf.header-cell`/`pdf.data-cell`, gated pela mesma
feature.

O receipt P1287 mediu 111 probes padrão: 72 iguais e 39 agrupados como
“different or disabled”. A auditoria desse conjunto separou:

- 2 casos desligados por perfil: `html` e `pdf.data-cell`;
- 10 extensões cristalinas deliberadas;
- 27 diferenças acionáveis não pertencentes a este passo.

Essas contagens identificam o corpus executado e não são percentagem da
linguagem.

## Fontes normativas e L0 vigentes

Antes de redigir qualquer L0, reler e piná-los por SHA-256:

- ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0128 e ADR-0129;
- `prompts/entities/html.md`;
- `prompts/shell/cli.md`;
- `prompts/wiring.md`;
- `prompts/infra/pipeline.md`;
- `prompts/compiler/eval.md`;
- `prompts/compiler/stdlib/pdf.md`;
- `prompts/entities/elements/table.md`;
- `prompts/entities/elements/table_cell.md`;
- `prompts/compiler/eval/table.md`;
- `prompts/compiler/layout/table.md` e `table_cell.md`;
- `prompts/entities/layout_types.md`;
- `prompts/infra/export/stream.md` e `builder.md`.

Referência vanilla de comportamento: upstream/main `a51e02804`, em especial
`typst-library/src/lib.rs`, `typst-library/src/pdf/mod.rs`,
`typst-library/src/pdf/accessibility.rs`, `typst-library/src/model/table.rs` e
o exportador PDF/tagging correspondente. A paridade é de linguagem e
morfologia pública; tipos, storage e passos internos podem divergir.

## Regime de execução

Usar o protocolo completo da skill `tekt-materializacao-segregada`:

1. autor de baseline mede somente o vanilla e congela feature sets, corpus e
   artefatos PDF;
2. autor de contrato recebe baseline e L0, sem ler o patch candidato;
3. autor de oráculos congela positivos, negativos e opacos antes do candidato;
4. adversário produz mutações discriminatórias e não corrige a solução;
5. implementador recebe apenas L0 confirmado, contrato e selo;
6. verificador final não pode editar contrato, oráculos, implementação ou
   relatório que julga.

O checkout compartilhado permite segregação por papel, capacidade, ordem e
hashes, mas não autoriza alegar isolamento ambiental forte.

## Fase A — corrigir a classificação do harness

Esta fase é laboratório e não altera comportamento do compilador.

### A1. Lattice explícito

O runner de probes deve emitir exatamente uma classificação por caso:

- `MATCH`: execução bilateral válida e observável equivalente;
- `DIFFERENCE`: execução bilateral válida com observável diferente;
- `DISABLED_BY_PROFILE`: o manifesto declara a feature ausente neste perfil e
  existe perfil ativo separado que a exercita;
- `UNKNOWN`: flag, binário, fixture, parser, proveniência ou execução impede a
  decisão;
- `BASELINE_ONLY`/`EXTRA_BINDING`: extensão ou assimetria declarada, sem crédito
  de paridade.

`DISABLED_BY_PROFILE` nunca soma em `MATCH`. Falha do candidato em aceitar a
feature dentro do perfil ativo é `DIFFERENCE` ou `UNKNOWN`, nunca disabled.

### A2. Manifesto de perfis

Congelar três perfis bilaterais:

| Perfil | Flags em ambos os lados | Estado esperado |
|---|---|---|
| `default` | nenhuma | `html` e `a11y-extras` desligados |
| `html` | `--features html` | módulo/target HTML exercitados |
| `a11y-extras` | `--features a11y-extras` | trio PDF acessível exercitado |

Combinação `html,a11y-extras`, repetição e ordem das flags são casos de
controle. `bundle` fica fora deste passo e deve permanecer explicitamente
`Unknown`/scope-out, nunca inferido da infraestrutura de HTML.

### A3. RED obrigatório do harness

Antes da correção, testes devem falhar demonstrando que:

1. falha bilateral por feature desligada entra hoje no balde agregado;
2. `html` desligado é confundido com diferença apesar do diagnóstico bilateral;
3. `pdf.data-cell` no default não possui perfil ativo correspondente;
4. uma flag desconhecida no perfil ativo poderia ser escondida como disabled;
5. os totais não fecham separadamente por classificação.

O GREEN exige totais independentes e soma exata igual ao número de probes,
forward/reverse idênticos e payload determinístico por bytes.

## Fase B — medir e congelar `a11y-extras`

O baseline independente deve medir antes de decidir:

1. parsing de `--features a11y-extras` em compile e eval, repetição, ordem e
   combinação com `html`;
2. disponibilidade do trio com e sem feature;
3. assinatura, defaults, casts e diagnósticos de cada função;
4. `pdf.table-summary(summary:, table)` com summary string e `none`;
5. `pdf.header-cell(level:, scope:, cell)` para `row`, `column` e `both`,
   incluindo level zero/tipo inválido;
6. `pdf.data-cell(cell)` com conteúdo cru e `table.cell(...)`;
7. tabelas simples, header automático, header explícito, data dentro de header,
   rowspan/colspan e repetição multipágina;
8. PDF com tags enabled e disabled;
9. texto, páginas, boxes e geometria visual separados da estrutura acessível;
10. objetos `/Table`, `/TR`, `/TH`, `/TD`, summary, scope, níveis, MCIDs,
    ParentTree e ordem estrutural, conforme realmente observados;
11. comportamento em HTML/SVG/PNG sem inferir que metadata PDF possui efeito
    nesses targets;
12. saída opaca ou não interpretável como `Unknown`.

Tecnologia assistiva real, PDF/UA, reflow, navegação por screen reader e
certificação de standard continuam `Unknown` salvo teste externo específico.
Presença de tags não prova acessibilidade efetiva.

## Fase C — nucleação e gate humano obrigatório

### C1. Núcleo compartilhado

Auditar se já existe Núcleo Tekt para gates de feature. Se não existir, criar
`00_nucleo/prompts/_nuclei/compiler-feature-gates.toml` com claims mínimas:

- conjunto de features vazio por default;
- feature só é ativada por pedido explícito;
- target/formato não ativa feature;
- binding gated não existe quando a feature está desligada;
- perfil desligado não recebe crédito de paridade;
- incapacidade no perfil ativo é `Unknown` ou `Violated`.

Cada Prompt L0 consumidor deve pinar o SHA-256 completo do Núcleo. O Núcleo
não possui owner produtivo, `Hash do Código` nem referência direta em código.

### C2. Owners L0

Redigir/atualizar, antes do código:

1. novo `prompts/entities/compiler_features.md` para um consumer puro dedicado
   `01_core/src/entities/compiler_features.rs`;
2. `entities/html.md`, removendo de `html.rs` a propriedade canônica do set de
   features; HTML continua dono apenas de `HtmlElem`/`HtmlBody`/attrs;
3. `shell/cli.md` para aceitar `a11y-extras`, default vazio e composição;
4. `wiring.md` e `infra/pipeline.md` para transportar o set sem inventar
   ativação;
5. `compiler/eval.md` para filtrar bindings pelo contexto de features;
6. `compiler/stdlib/pdf.md` para o trio e seus diagnósticos;
7. `entities/elements/table.md` para summary semântico;
8. `entities/elements/table_cell.md` para kind/level/scope explícitos;
9. `compiler/eval/table.md` para conversão content-or-cell e preservação dos
   wrappers;
10. `compiler/layout/table.md`/`table_cell.md` e, se a medição exigir,
    `entities/layout_types.md`, para transportar a árvore de tabela sem
    alterar o desenho visual;
11. `infra/export/stream.md`/`builder.md` para tagging PDF validado.

Manter ownership Prompt L0 ↔ consumer produtivo estritamente 1:1. Claims
compartilhadas pertencem ao Núcleo, não são copiadas entre prompts.

### C3. Engenharia da refatoração

`Feature`/`Features` não pode continuar conceptualmente pertencendo ao módulo
`entities::html` após ganhar `A11yExtras`. Mover o dado puro para o owner
`compiler_features`; migrar consumidores para o caminho canônico. Se o path
Rust anterior for público, preservar compatibilidade por re-export explícito
medido e documentado, em vez de quebra silenciosa.

O layout mantém match fechado, estático e exaustivo. A lógica da feature fica
nos módulos `compiler/layout/table*.rs` por descendência, conforme ADR-0109;
não criar vtable, `dyn`, dispatcher por strings nem import reverso
`entities → compiler`.

### C4. Paragem

Este passo adiciona variante/capacidade pública de feature, nova flag e
metadata pública de tabela/célula. É PARAGEM OBRIGATÓRIA pela ADR-0127.

Fluxo obrigatório:

```text
medição vanilla → contrato/oráculos → Núcleo + L0s → hashes/ownership válidos
→ PARAR → confirmação humana → testes RED → implementação → GREEN
```

Não criar consumer produtivo, campo, enum, flag ou stub antes da confirmação.

## Fase D — testes RED independentes

Após confirmação humana, congelar REDs que provem:

1. `--features a11y-extras` é inicialmente rejeitado pelo candidato;
2. sem feature, os três bindings permanecem indisponíveis com semântica de
   gate equivalente ao vanilla;
3. com feature, os três bindings estão ausentes antes da implementação;
4. ativar HTML não ativa `a11y-extras` e vice-versa;
5. summary/scope/level/data não sobrevivem hoje até o PDF;
6. tags disabled preservam visual/texto mas omitem estrutura;
7. um documento multipágina mantém ordem/IDs válidos;
8. os 72 `MATCH` padrão do P1287 não regridem;
9. o perfil HTML existente não regride;
10. uma implementação stub que apenas registra as funções é rejeitada.

Os testes A/B são derivados do contrato congelado sem ler a candidata.

## Fase E — implementação

Implementar somente depois do gate:

1. set puro de features com `Html` e `A11yExtras`, ambos false por default;
2. parsing/propagação bilateral de `--features a11y-extras`;
3. gating do namespace `pdf` pelo contexto de eval;
4. trio completo com casts/defaults/diagnósticos medidos;
5. carriers semânticos de table/cell, preservados por `map_*`, hash, repr,
   show/eval e layout;
6. estrutura de tabela no PDF tagged com texto/geometria inalterados;
7. ausência da estrutura quando tags estão disabled;
8. perfil `a11y-extras` no inventário e nos probes.

Não implementar pela igualdade estrutural do Rust nem copiar a mecânica
interna vanilla sem necessidade. O gate é a semântica, sintaxe e morfologia
pública observável.

## Ataques adversariais mínimos

O gate discriminatório deve matar, no mínimo:

1. HTML ligado por default;
2. target HTML ativando implicitamente a feature;
3. `a11y-extras` ligado por default;
4. `html` ativando `a11y-extras`;
5. `a11y-extras` ativando `html`;
6. flag aceita mas ignorada;
7. somente `pdf.data-cell` registrado;
8. bindings presentes sem feature;
9. stub que devolve o body e descarta metadata;
10. summary perdido antes do builder;
11. header convertido em data;
12. data dentro de header promovido automaticamente a header;
13. scope `row`/`column` trocado;
14. level ignorado ou zero aceito;
15. MCID duplicado;
16. ParentTree/StructElem órfão;
17. tags emitidas quando disabled;
18. mudança visual/textual causada apenas pela metadata;
19. `DISABLED_BY_PROFILE` contado como `MATCH`;
20. flag desconhecida no perfil ativo classificada como disabled;
21. ordem forward/reverse divergente;
22. drift de manifesto, baseline, fixture, Núcleo ou L0 ignorado.

`mutation_score = 1.0` para todos os mutantes válidos. Caso opaco deliberado
pode resultar em `Unknown`; mutante inválido não entra no denominador.

## Critérios de aceitação

O passo só fecha se:

- o perfil padrão separa os dois casos disabled sem aumentar falsamente
  `MATCH`;
- o perfil HTML continua bilateral e default off;
- o perfil `a11y-extras` executa bilateralmente o trio completo;
- todos os observáveis obrigatórios do contrato ficam `Preserved`, sem
  `Unknown` ou `Violated`;
- PDF visual/textual permanece equivalente quando só metadata acessível muda;
- PDF tagged preserva a estrutura medida e PDF sem tags a omite;
- inventários/probes são determinísticos e forward/reverse concordam;
- mutações válidas são todas mortas;
- hashes protegidos, ownership 1:1 e Núcleos permanecem íntegros;
- nenhuma contagem é apresentada como percentagem global de linguagem.

## Não objetivos

- ativar qualquer feature por default;
- implementar `bundle`;
- remover as 45 extensões cristalinas;
- fechar os outros 27 probes padrão acionáveis;
- declarar `111/111` alterando a classificação;
- corrigir as divergências globais C05/C07/C08/E11 do P1287;
- prometer PDF/UA, AT real ou acessibilidade geral;
- converter HTML a partir de `PagedDocument`;
- despacho dinâmico ou violação da Forma B.

## Validação final

Executar e registrar commit, árvore, hora, comandos e hashes dos binários:

```text
cargo test --workspace
python3 -m unittest discover -s lab/surface-inventory -p 'test_*.py'
python3 -m unittest lab/parity/matrix/test_runner.py
<runner P1288 de oráculos, forward e reverse>
<inventário/probes default>
<inventário/probes html>
<inventário/probes a11y-extras>
python3 lab/parity/matrix/p1287_global.py \
  --binary lab/parity/matrix/p1287_candidate_adapter.py
python3 lab/parity/matrix/runner.py \
  --vanilla /usr/local/bin/typst \
  --crystalline target/release/typst \
  --output /tmp/p1288-matrix-final.json
cargo build --workspace --bin typst
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --checks v5,v15,v26 --fail-on warning .
git diff --check
```

Uma divergência global P1287 fora deste recorte não invalida automaticamente
o refinamento P1288, mas deve permanecer visível e não pode ser renomeada como
sucesso.

## Artefatos finais esperados

- `00_nucleo/diagnosticos/p1288-vanilla-measurement-receipt.md`;
- `00_nucleo/diagnosticos/p1288-contract-receipt.md`;
- manifesto canônico e baseline de oráculo P1288;
- testes/runner de perfis e `a11y-extras`;
- plano e receipt adversarial com log persistido da campanha;
- `00_nucleo/diagnosticos/typst-passo-1288-relatorio.md`;
- `00_nucleo/diagnosticos/p1288-verification-receipt.md`.

Persistir o driver e o log da campanha adversarial; não repetir a limitação
P1287 de depender somente de executor efêmero em `/tmp`.

## Veredito permitido

O relatório termina com exatamente um:

- **REFINED** — harness separa estados corretamente e o perfil
  `a11y-extras` preserva o contrato completo;
- **NOT REFINED** — qualquer gate, mutação ou observável obrigatório falha;
- **INCONCLUSIVE** — identidade, harness, parser ou proveniência impede a
  decisão sem ser legítimo classificar como falha funcional.
