# Passo 1300 — fechar a contradição dos constructors globais de cor

**Estado inicial:** autorizado para iniciar, mas não para ultrapassar o gate L0 sem nova
confirmação humana.
**Coorte selecionada por P1299:** `color-global-constructors`.
**Paths removidos da superfície pública:** `hsl`, `hsv`, `linear_rgb`.
**Rotas canônicas preservadas:** `color.hsl`, `color.hsv`, `color.linear-rgb`.
**Regime:** protocolo completo de materialização segregada Tekt.
**Baseline produtivo:** `1f082370e59939de7b57992e137a9f74bfb6758f`.
**Vanilla ratificado:** `a51e02804`; `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
**ADRs:** ADR-0107, ADR-0108, ADR-0127 e ADR-0129.

---

## 1. Resultado pretendido

Alinhar o scope público cristalino ao vanilla ratificado no único fragmento selecionado
por P1299:

- os nomes bare `hsl`, `hsv` e `linear_rgb` deixam de existir;
- os mesmos três nomes também deixam de existir como `std.hsl`, `std.hsv` e
  `std.linear_rgb`, porque `std` é a cópia não sombreada da mesma stdlib;
- `color.hsl`, `color.hsv` e `color.linear-rgb` continuam funções chamáveis, com os mesmos
  nomes públicos, argumentos, valores, repr e semântica vigentes;
- os constructors globais ratificados `rgb`, `luma`, `oklab`, `oklch` e `cmyk` continuam
  presentes, tanto bare quanto sob `std`;
- nenhuma nativa de conversão de cor é apagada ou reimplementada;
- nenhuma cor predefinida, operador de cor, `color.map`, feature, target, parser, entidade,
  default ou fase do pipeline muda.

A alegação máxima permitida ao final é:

> A contradição pública P1299 dos três constructors globais de cor foi removida no
> baseline e contrato pinados, preservando as rotas qualificadas e os cinco constructors
> globais ratificados; não se alega paridade geral da stdlib ou do tipo `color`.

---

## 2. Medição anterior à decisão

P1299 terminou com `P1299_PASS_P1300_COHORT_SELECTED` e selecionou uma única coorte:

| Path cristalino extra | Perfil default | HTML | a11y | HTML+a11y | Rota canônica |
|---|---|---|---|---|---|
| `hsl` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `color.hsl` = `MATCH_VALUE` |
| `hsv` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `color.hsv` = `MATCH_VALUE` |
| `linear_rgb` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `CRYSTALLINE_ONLY` | `color.linear-rgb` = `MATCH_VALUE` |

Fontes medidas:

- `01_core/src/compiler/eval/mod.rs:1650-1654` registra os três extras;
- `lab/typst-original/crates/typst-library/src/lib.rs:397-401` registra globalmente apenas
  `luma`, `oklab`, `oklch`, `rgb` e `cmyk`;
- `00_nucleo/prompts/compiler/stdlib/color.md:22-24` afirma incorretamente que os oito
  constructors de `color` são também globais;
- `01_core/src/compiler/stdlib/color.rs:38-43` repete essa afirmação no comentário do
  consumer, embora seu `color_type_field` exponha corretamente os oito fields;
- `00_nucleo/prompts/compiler/eval.md` ainda não contém a regra fechada do conjunto global
  de constructors de cor.

P1299 classificou os três casos como `L0_CONTRADICTION`, não como extensão deliberada.
A inferência é refutada se o vanilla ratificado aceitar qualquer path bare, ou se qualquer
rota canônica deixar de ser `MATCH_VALUE` no mesmo perfil. Nenhuma dessas refutações foi
observada nos quatro perfis.

### 2.1 Inputs P1299 protegidos

Antes de agir, verificar no mínimo:

| Artefato | SHA-256 esperado |
|---|---|
| `00_nucleo/materialization/typst-passo-1299.md` | `574c0f77207a86bbe656bf191120c9603924d0c555e5b411760756a6b303b158` |
| `00_nucleo/diagnosticos/p1299-certificate.json` | `c9d18805b081e55b34caaf02e39ddfc15fd76c5d48849dd82ca112552cdac754` |
| `00_nucleo/diagnosticos/p1299-decision-report.md` | `332425ce95df73ce8ea8e54e84aec31d37286a6dec8b07ce9b6b5f77e33eeb2e` |
| `00_nucleo/diagnosticos/p1299-owner-ledger.tsv` | `79261fe7f8b377b2fb28781b15e88828c8b87843e5b1d0d99cdf1e7a7355fa69` |
| `00_nucleo/diagnosticos/p1299-feature-matrix.json` | `58f08430c479ba5dea093945301785a89e77f20e9af0809ae05c6641a7ecdc11` |
| `00_nucleo/prompts/compiler/stdlib/color.md` | `fecc619d1b336a6f8ca8c39c22a302babdec4daf22d3387ce97ce28e36d87dc3` |
| `00_nucleo/prompts/compiler/eval.md` | `98d8255070dd4f23626d174ef3eef021299d62bd0a039ff521a7b5d363848f79` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `4e5963393178ccec06850ca16e84db530675c6760e977d68d68839e1cf891848` |
| `01_core/src/compiler/stdlib/color.rs` | `d40197461efdee472734ef850e7a681821eb490624856bf15510d1b8a9336fed` |
| `01_core/src/compiler/eval/mod.rs` | `cafdcbf690f5ad8020bbe3da4457a759397c29ac091dc1624f33e14f5127c5b4` |
| `01_core/src/compiler/eval/tests.rs` | `b6b87d6786874a39b3071c667476d062a4fc41a9298c6519d9434e0ae637df20` |
| fonte vanilla `typst-library/src/lib.rs` | `0f43c11dd22fc2da64e6f526d20f4257f8e324b087c70b7986d13694f5abb5ee` |

Validar também todas as identidades declaradas no certificado P1299. Alteração de algum
input protegido termina em `P1300_BLOCKED_INPUT_DRIFT`; não regenerar o P1299 dentro deste
passo.

---

## 3. Escopo fechado

### 3.1 Escritas produtivas permitidas após o gate humano

Somente:

1. `01_core/src/compiler/eval/mod.rs` — remoção dos três registros e imports tornados
   mortos; atualização do comentário local;
2. `01_core/src/compiler/stdlib/color.rs` — correção documental do comentário que chama
   os oito constructors de globais; nenhuma lógica de `color_type_field` muda;
3. `01_core/src/compiler/eval/tests.rs` — regressões permanentes do fragmento.

### 3.2 L0s que devem mudar antes do código

1. `00_nucleo/prompts/compiler/stdlib/color.md` — owner do tipo e de seus fields;
2. `00_nucleo/prompts/compiler/eval.md` — owner formal de `eval/mod.rs` e do scope base;
3. `00_nucleo/prompts/compiler/eval/tests.md` — owner do consumer de testes.

O primeiro prompt deixa de afirmar que os oito constructors são globais e delega a
política do scope raiz ao owner `compiler/eval`. O segundo passa a definir o conjunto
global fechado. O terceiro legitima as regressões do fragmento. Essa divisão respeita a
cardinalidade 1:1: `color.md` não passa a ser owner de `eval/mod.rs`, e `eval.md` não
reivindica a implementação interna de `color_type_field`.

### 3.3 Fora do escopo

- os outros 42 extras intencionais do ledger P1299;
- os três spans `pdf.*`;
- os 106 membros ausentes;
- as 11 diferenças de `color.map`;
- aliases globais de matemática, counter, state, table, grid, `lof` ou `lot`;
- `cyan`, `magenta` e `none`, explicitamente mantidos pelo L0 de `color`;
- `calc.deg`, `calc.rad`, `calc.log10`;
- mudar o spelling público para um novo global `linear-rgb`;
- alterar `Func::name`, repr, igualdade de função ou `color.space()`;
- editar o catálogo, runner ou resultados P1299;
- staging, commit, push ou limpeza destrutiva.

Encontrar defeito fora dessa lista gera diagnóstico para passo futuro; não amplia P1300.

---

## 4. Gate L0 obrigatório — executar e parar

Esta é remoção de contrato público e quebra de compatibilidade dos três aliases. ADR-0127
obriga parada humana.

### 4.1 Repetir a medição focal

Com um build cristalino fresco do baseline e o vanilla pinado, medir nos quatro perfis:

```typst
repr(type(hsl))
repr(type(hsv))
repr(type(linear_rgb))
repr(type(color.hsl))
repr(type(color.hsv))
repr(type(color.linear-rgb))
repr(type(std.hsl))
repr(type(std.hsv))
repr(type(std.linear_rgb))
repr((type(rgb), type(luma), type(oklab), type(oklch), type(cmyk)))
repr((type(std.rgb), type(std.luma), type(std.oklab), type(std.oklch), type(std.cmyk)))
```

Guardar argv, features, exit, stdout, stderr e spans integrais em
`00_nucleo/diagnosticos/p1300-pre-gate-measurement.json`. Antes do candidato, os três
bare e os três `std.*` devem reproduzir a assimetria; as três rotas canônicas e os dez
controles positivos devem permanecer bilaterais. Divergência diferente bloqueia o gate.

### 4.2 Redigir os três L0s

O diff normativo deve conter, sem antecipar código:

#### `stdlib/color.md`

- `color` conserva oito constructors no próprio tipo:
  `rgb`, `linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`;
- somente `rgb`, `luma`, `cmyk`, `oklab` e `oklch` também são globais;
- `linear-rgb`, `hsl` e `hsv` são exclusivamente qualificados por `color.*`;
- `color_type_field`, as nativas e `color.space()` permanecem inalterados;
- a composição do scope global pertence a `compiler/eval.md`.

#### `compiler/eval.md`

- `make_stdlib_with_features` registra exatamente os cinco constructors globais
  ratificados `rgb`, `luma`, `cmyk`, `oklab` e `oklch`;
- não registra `hsl`, `hsv`, `linear_rgb` nem inventa `linear-rgb` global;
- como `std` é construído a partir da mesma stdlib, os quatro nomes também não existem
  sob `std`;
- o conjunto é independente das features `html` e `a11y-extras`;
- a regra não afeta fields de `color`, nativas ou cores predefinidas.

#### `compiler/eval/tests.md`

- testes negativos cobrem os três nomes bare e os três `std.*` nos quatro perfis;
- testes positivos cobrem as três rotas `color.*` e os cinco globals ratificados, bare e
  sob `std`;
- erros negativos comparam classe, mensagem e span público com o vanilla;
- testes impedem que a correção apague a nativa ou esconda apenas uma das duas projeções
  root/`std`.

### 4.3 Receipt e parada

Criar `00_nucleo/diagnosticos/p1300-l0-gate-receipt.md` com:

- baseline e working tree exatos;
- hashes dos inputs P1299;
- medição focal;
- diff integral dos três L0s;
- SHA-256 novo de cada L0;
- classificação explícita `ADR0127_PUBLIC_CONTRACT_AND_COMPAT_BREAK`;
- lista de arquivos ainda proibidos de alterar.

Então terminar a execução com:

```text
P1300_WAITING_HUMAN_L0_CONFIRMATION
```

**PARAR.** Antes da confirmação humana é proibido:

- editar qualquer `.rs`;
- escrever teste RED permanente;
- executar `crystalline-lint --fix-hashes`;
- criar contrato selado contra o L0 ainda não confirmado;
- iniciar implementação, staging ou commit.

---

## 5. Continuação somente após confirmação humana explícita

A confirmação deve referir-se ao diff L0 P1300. Uma autorização anterior ou genérica não
substitui este gate. Após a confirmação, congelar os três novos hashes L0 e criar
`00_nucleo/diagnosticos/p1300-manifest.json`.

O manifesto registra protocolo, baseline, L0s confirmados, vanilla, P1299, observáveis,
política de `Unknown`, budget e capacidades dos papéis. Se qualquer entrada protegida
mudar depois do selo, invalidar o selo e reiniciar na primeira fase afetada.

---

## 6. Papéis e capacidades segregados

| Papel | Pode ler | Pode escrever | Proibição decisiva |
|---|---|---|---|
| P1 — autor do contrato | L0 confirmado, baseline, observáveis públicos | contrato e receipt | não lê patch candidato |
| P2 — autor dos oráculos | L0, contrato candidato, vanilla pinado | casos positivos/negativos/opacos | não adapta oracle ao patch |
| P3 — adversário | L0, contrato, baseline | plano e mutantes | não corrige solução |
| P4 — selador discriminatório | contrato, oráculos, mutantes e recibos | selo | não escreve código/testes produtivos |
| P5 — testador A/B | L0 confirmado e selo | apenas testes permanentes autorizados | não lê implementação candidata antes de congelar o teste |
| P6 — implementador | L0 confirmado e contrato selado | `eval/mod.rs` e comentário de `color.rs` | não edita contrato, oráculos ou testes P5 |
| P7 — integrador | selo e outputs P5/P6 | integração mecânica e resselo de hashes | não relaxa expectativa |
| P8 — verificador | todos os artefatos congelados e candidato | receipts e certificado | não corrige o que julga |

Preferir ambientes/worktrees separados. Se o filesystem compartilhado permitir leitura
cruzada, registrar expressamente `executado sem atestação de isolamento técnico`; nomes
de agentes diferentes não provam independência.

---

## 7. Contrato observável antes da implementação

P1 cria `00_nucleo/diagnosticos/p1300-contract.json`, contendo pelo menos:

### 7.1 Negativos obrigatórios

Nos quatro perfis `default`, `html`, `a11y` e `html+a11y`:

- `hsl`, `hsv`, `linear_rgb` devem falhar como variável desconhecida;
- `std.hsl`, `std.hsv`, `std.linear_rgb` devem falhar como field ausente;
- exit, mensagem, hints e span devem coincidir com o vanilla ratificado.

### 7.2 Positivos obrigatórios

Nos quatro perfis:

- `color.hsl`, `color.hsv`, `color.linear-rgb` existem, são funções e permanecem
  chamáveis;
- chamadas representativas preservam o mesmo `repr` bilateral;
- `rgb`, `luma`, `oklab`, `oklch`, `cmyk` e seus pares `std.*` continuam funções;
- `color.space(color.hsl(...))`, `color.space(color.hsv(...))` e
  `color.space(color.linear-rgb(...))` continuam devolvendo a identidade pública medida;
- `color.map`, as 18 cores ratificadas e os extras `cyan`, `magenta`, `none` não mudam.

### 7.3 Política fechada

- `Preserved`: todos os observáveis do caso coincidem;
- `Violated`: pelo menos um observável público contradiz o contrato, com testemunha;
- `Unknown`: timeout, falta de produto, identidade ambígua ou observação incompleta;
- `Unknown` nunca conta como sucesso, exceto em caso opaco criado explicitamente para
  validar opacidade.

O contrato não compara estrutura Rust, endereço de function pointer ou ordem interna de
inserção, salvo quando a ordem se tornar observável na linguagem.

---

## 8. Oráculos e campanha adversarial

P2 mede o contrato no vanilla pinado e produz:

- `p1300-vanilla-measurement-receipt.md`;
- `p1300-oracle-suite.json`;
- casos de fronteira e pelo menos um caso opaco deliberado.

P3 produz `p1300-adversarial-plan.md` e mutantes válidos que simulam, no mínimo:

1. manter `hsl` global;
2. manter `hsv` global;
3. manter `linear_rgb` global;
4. ocultar o bare, mas deixar o mesmo path sob `std`;
5. ocultar somente no perfil default;
6. apagar ou quebrar `color.hsl`;
7. apagar ou quebrar `color.hsv`;
8. apagar ou renomear `color.linear-rgb`;
9. remover um dos cinco globals ratificados;
10. criar incorretamente um global `linear-rgb`;
11. mudar `Func::name`/repr das rotas qualificadas;
12. remover a nativa em vez de apenas sua publicação global;
13. alterar `color.space()` para depender da presença do alias;
14. atingir cores predefinidas, `color.map` ou operadores alheios à coorte.

P4 executa positivos, negativos, opacos, repetição e ordem invertida. Exigir:

```text
mutation_score = mutantes válidos rejeitados / mutantes válidos = 1.0
```

Mutante sobrevivente, positivo rejeitado ou `Unknown` inesperado impede o selo. Produzir
`p1300-discrimination-receipt.json` e `p1300-contract-seal.json`.

Budget de calibração: no máximo duas revisões completas. Cada revisão registra hipótese,
delta discriminatório, regressões e custo. Duas revisões consecutivas sem ganho na mesma
causa terminam em `P1300_BLOCKED_CONTRACT_DESIGN`; não afrouxar o contrato.

---

## 9. RED independente

Somente após o selo, P5 escreve testes no consumer autorizado
`01_core/src/compiler/eval/tests.rs`, a partir do L0 confirmado e sem ler o patch P6.

O RED deve demonstrar separadamente:

1. os três nomes bare ainda são aceitos no baseline;
2. os três paths `std.*` ainda são aceitos no baseline;
3. as três rotas qualificadas e os dez controles dos globals ratificados já são GREEN;
4. os resultados são idênticos nos quatro perfis quanto à disponibilidade;
5. o contrato rejeita esconder a rota qualificada ou remover uma nativa.

Guardar comandos, filtros, contagens e falhas exatas em
`p1300-red-tests-receipt.md`. RED que falhe por compilação, import, fixture ou erro alheio
não conta.

---

## 10. Implementação mínima

P6 recebe somente L0 confirmado e contrato selado. A implementação permitida é:

1. em `make_stdlib_with_features`, remover as definições das chaves `linear_rgb`, `hsl` e
   `hsv`;
2. remover apenas os imports locais que ficarem sem uso;
3. atualizar o comentário do conjunto global em `eval/mod.rs`;
4. em `color.rs`, corrigir apenas o comentário que chama os oito constructors de globais;
5. não alterar `color_type_field`, `native_linear_rgb`, `native_hsl`, `native_hsv`,
   `native_color_space` ou seus function pointers.

Não criar blacklist no lookup, filtro dependente de feature, wrapper, alias alternativo ou
ramo especial em field access. A causa está na publicação indevida das três chaves; a
correção deve atuar nesse ponto.

P6 produz `p1300-implementation-receipt.md` com diff, hashes, testes próprios e declaração
de que contrato/oráculos/testes P5 não foram editados.

---

## 11. Integração, linhagem e superfícies

P7 integra os outputs sem mudar expectativas. Depois:

1. atualizar `@prompt-hash` dos três consumers afetados;
2. atualizar `Hash do Código` dos três L0s;
3. executar `crystalline-lint --fix-hashes .` apenas sobre o estado pós-gate confirmado;
4. exigir V15/V26 verdes e `--fix-hashes --dry-run` igual a `Nothing to fix`;
5. gerar uma matriz P1300 dos quatro perfis com o mesmo classificador P1299;
6. executar novamente as probes padrão default/HTML para medir o delta.

Delta mínimo esperado nas probes padrão, se o catálogo amostrado permanecer idêntico:

- default: três diferenças extras observadas (`hsl` e as que o sampler selecionar) podem
  cair, mas nenhuma contagem fixa é alegada antes da rerun;
- HTML: idem;
- na matriz focal, `CRYSTALLINE_ONLY` deve cair exatamente 12 observações
  (`3 paths × 4 perfis`), sem aumento em `VANILLA_ONLY`, `DIFFERENT_VALUE`,
  `DIFFERENT_DIAGNOSTIC` ou `EXECUTION_UNKNOWN`.

A contagem do sampler padrão não é gate absoluto porque default e HTML amostram subconjuntos
diferentes. O gate absoluto é nominal: os três extras deixam de existir nos quatro perfis,
as três rotas canônicas continuam bilaterais e nenhum controle regride.

---

## 12. Verificação final

P8 revalida todos os hashes selados e executa, com target isolado e proveniência:

```bash
cargo test -p typst-core p1300
cargo test --workspace
cargo build
cargo fmt --all -- --check
python3 -m unittest \
  lab/surface-inventory/test_merge.py \
  lab/surface-inventory/test_run_probes.py \
  00_nucleo/diagnosticos/test_p1299_run_matrix.py
crystalline-lint .
crystalline-lint --fail-on warning --check V3 .
crystalline-lint --fail-on warning --check V4 .
crystalline-lint --fail-on warning --check V5 .
crystalline-lint --fail-on warning --check V13 .
crystalline-lint --fail-on warning --check V14 .
crystalline-lint --fail-on warning --check V15 .
crystalline-lint --fail-on warning --check V26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
git diff --cached --check
```

Executar o contrato selado duas vezes, uma em ordem normal e outra invertida. Verificar que
nenhum input protegido mudou, que o índice não recebeu paths e que o diff está limitado ao
allowlist P1300 e aos artefatos P1299 preexistentes.

Produzir:

- `p1300-verification-receipt.json`;
- `p1300-surface-default.json`;
- `p1300-surface-html.json`;
- `p1300-feature-matrix.json`;
- `p1300-final-report.md`;
- `p1300-certificate.json`.

O certificado pina manifesto, L0s confirmados, contrato, oráculos, ataques, selo, testes,
implementação, superfícies, receipts e limitações de isolamento. Não contém seu próprio
SHA recursivamente; o hash detached é reportado depois.

---

## 13. Vereditos terminais

Antes da confirmação humana, o único estado de sucesso parcial é:

```text
P1300_WAITING_HUMAN_L0_CONFIRMATION
```

Após a confirmação:

- `P1300_CERTIFIED_COLOR_GLOBAL_CONSTRUCTORS_ALIGNED`;
- `P1300_BLOCKED_INPUT_DRIFT`;
- `P1300_BLOCKED_MEASUREMENT_REFUTED`;
- `P1300_BLOCKED_CONTRACT_DESIGN`;
- `P1300_BLOCKED_MUTATION_SURVIVOR`;
- `P1300_BLOCKED_RED_INVALID`;
- `P1300_BLOCKED_IMPLEMENTATION`;
- `P1300_BLOCKED_VERIFICATION`;
- `P1300_BLOCKED_UNKNOWN`.

É proibido declarar certificado se algum dos três extras continuar acessível bare ou sob
`std`, se alguma rota `color.*` ou global ratificado regredir, se o mutation score for
menor que `1.0`, se houver `Unknown` inesperado, ou se a confirmação humana do diff L0 não
estiver registrada.
