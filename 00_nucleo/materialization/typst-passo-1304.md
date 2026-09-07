# Passo 1304 — rebaseline pós-P1303 e seleção causal do próximo lote

## Natureza do passo

P1304 é uma auditoria diagnóstica e de seleção. Não altera Prompt L0, código,
testes permanentes, contratos públicos, defaults nem fases do pipeline.

O regime é **A/B bilateral com classificação e veredito independentes**, sem
mutation testing de produto: não existe implementação candidata P1304 a ser
mutada. A segregação deve impedir que o autor do classificador escolha o próximo
lote adaptando o oracle ao resultado desejado. Como o filesystem é compartilhado,
o resultado será descrito como `executado sem atestação de isolamento técnico`.

## Baseline composto

O HEAD continua em:

```text
5b4a0d0438a535c54fdb5e74b28903c1313f5bc2
fix: seal color globals and module diagnostics through step 1302
```

P1303 está certificado, mas ainda não commitado. Portanto o baseline semântico
do P1304 é composto por `HEAD + candidato P1303`, nunca apenas pelo hash do HEAD.

Os quatro paths tracked do candidato P1303 devem ter exatamente estes hashes:

| Path | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `6b82a38b7c953ebe9dcbe96857dd9cb689aaaf69e95df153afa5de668ef89486` |
| `01_core/src/compiler/eval/tests.rs` | `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0` |

Artefatos finais P1303 congelados:

- `p1303-final-report.md`, SHA-256
  `c4aa69ab55dc9dcfce69c305d57a74a87cd7690cdc3cbe52300f39c7f5a47bc8`;
- `p1303-certificate.json`, SHA-256
  `77d209164fa3e2c9af5d5df69ee8ad7d72db8a5fc829e7469a827efcbc46fb90`;
- `p1303-final-measurement.json`, SHA-256
  `e56622f54973981c294233c3d1a3a12df4ef0bee13c91347fb1c343dd2d35ed7`.

O diff tracked esperado antes do P1304 contém somente os quatro paths acima,
com `562 insertions(+), 4 deletions(-)`. Os artefatos não rastreados P1303 e os
passos P1303/P1304 são permitidos. Qualquer outro delta bloqueia a auditoria.

O vanilla permanece upstream/main ratificado `a51e02804`, pelo binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Objetivo

Reexecutar a superfície P1299 sobre o candidato P1303, incorporar a divergência
separada `repr(std)`, confirmar que P1300/P1301r2/P1303 não regrediram e selecionar
exatamente uma coorte para o P1305.

P1304 deve responder, com evidência reproduzível:

1. quais dos 160 paths residuais P1299 continuam abertos;
2. se algum path mudou de classe ou foi fechado incidentalmente;
3. se surgiu regressão fora do ledger;
4. se `repr(std)` continua `DIFFERENT_VALUE`;
5. se os 11 casos `color.map*` são divergência de dados/kind ou somente de
   representação pública;
6. se `json.encode`, `toml.encode` e `yaml.encode` continuam ausentes e formam
   uma única coorte implementável;
7. qual coorte vence a seleção determinística e qual gate ADR-0127 o próximo
   passo deverá observar.

P1304 não corrige nenhuma divergência encontrada.

## Entradas congeladas

Usar, com hashes registrados no manifesto P1304:

- `00_nucleo/diagnosticos/p1299-probe-catalog.json`;
- `00_nucleo/diagnosticos/p1299-feature-matrix.json`;
- `00_nucleo/diagnosticos/p1299-owner-ledger.tsv`, SHA-256
  `79261fe7f8b377b2fb28781b15e88828c8b87843e5b1d0d99cdf1e7a7355fa69`;
- `00_nucleo/diagnosticos/p1299-decision-report.md`;
- os três artefatos finais P1303 pinados acima;
- vanilla ratificado e candidato cristalino fresco;
- Prompts L0 vigentes dos owners examinados.

O catálogo P1299 contém 626 probes. P1304 adiciona `repr(std)` como probe
explícito, com ID próprio e sem alterar o catálogo histórico. Não sobrescrever
nenhum artefato P1299/P1300/P1301/P1302/P1303.

## Owners que devem ser lidos antes da seleção

O classificador/selecionador deve ler integralmente e registrar o SHA-256 de:

- `00_nucleo/prompts/compiler/eval/repr.md`;
- `00_nucleo/prompts/entities/module.md`;
- `00_nucleo/prompts/compiler/stdlib/color.md`;
- `00_nucleo/prompts/compiler/stdlib/loading.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- o Prompt proprietário de qualquer outra coorte que alcance a fase final de
  seleção.

Leitura não autoriza edição. Se o L0 vigente contradisser a classificação
derivada da medição, registrar `L0_CONTRADICTION`; não adaptar silenciosamente
o resultado ao L0 nem propor código contra ele.

## Política de `Unknown`

- `EXECUTION_UNKNOWN` bloqueia o rebaseline;
- path sem owner 1:1 válido é `UNRESOLVED_OWNERSHIP`, nunca selecionável;
- valor truncado pelo próprio observável não prova igualdade nem diferença de
  conteúdo;
- parser sem suporte, timeout, crash, saída não decodificável ou identidade
  ambígua permanecem `Unknown` com reason code;
- somente controles previamente declarados opacos podem terminar `Unknown` sem
  bloquear o certificado;
- ausência bilateral não equivale a paridade de membro público sem confirmar a
  expectativa de feature/target.

## Fase A — congelamento e build fresco

Criar `p1304-baseline-status.txt` e `p1304-manifest.json` com:

- HEAD, timestamp com timezone e versão do protocolo;
- `git status --short`, `git diff HEAD --stat`, `git diff --cached --stat`;
- lista e SHA-256 de todo path modificado/não rastreado do P1303;
- hashes dos inputs e dos Prompts L0 lidos;
- capacidades de cada papel;
- política de `Unknown`;
- comandos planejados e allowlist de escrita.

Antes do build, confirmar:

```bash
cargo test -p typst-core p1303 -- --test-threads=1
crystalline-lint --fix-hashes --dry-run .
git diff --check
```

Os três comandos devem terminar com exit `0`, e o dry-run deve imprimir
`Nothing to fix`.

Construir o cristalino em `CARGO_TARGET_DIR` temporário e exclusivo:

```bash
cargo build --release --bin typst
```

Registrar caminho, SHA-256, tamanho, `--version`, duração e logs integrais. Não
reutilizar `target/release/typst` nem binário P1299/P1303 já removido.

## Fase B — runner A/B independente

O autor do runner recebe catálogo, perfis e envelope de saída, mas não recebe a
hipótese de qual coorte deve vencer.

Executar os 626 probes P1299 mais `repr(std)` em:

- `default`;
- `html`;
- `a11y`;
- `html+a11y`.

Para cada execução bilateral preservar:

- ID/path/expression/profile;
- argv integral;
- hashes dos binários;
- exit code;
- stdout e stderr integrais, mais SHA-256;
- duração em nanos;
- estado completo ou reason code de `Unknown`.

Classificação fechada:

- `MATCH_VALUE`;
- `MATCH_DIAGNOSTIC`;
- `CRYSTALLINE_ONLY`;
- `VANILLA_ONLY`;
- `DIFFERENT_VALUE`;
- `DIFFERENT_DIAGNOSTIC`;
- `EXECUTION_UNKNOWN`.

Não usar o runner legado que colapsa falhas bilaterais em
`DIFFERENCE_OR_DISABLED`. Diagnósticos coincidem somente quando exit code e
stderr integral são idênticos.

Executar o corpus completo uma vez. Depois repetir em ordem invertida apenas:

- todos os paths não-`MATCH`;
- `repr(std)`;
- os três aliases P1300 bare e sob `std`;
- os controles de módulo P1301r2;
- os três gates `pdf.*` P1303 nos quatro perfis;
- um sample determinístico de 5% dos `MATCH`, definido pelo menor SHA-256 do ID.

Resultados normal/invertido devem ser idênticos por ID. Essa estratégia limita
custo sem omitir repetição nas decisões.

Gerar `p1304-probe-catalog.json`, `p1304-feature-matrix.json` e
`p1304-run-matrix.py`.

## Fase C — sentinelas dos passos fechados

Classificar separadamente, sem reabrir vereditos históricos:

### P1300/P1301r2

- `hsl`, `hsv`, `linear_rgb`, `std.hsl`, `std.hsv`, `std.linear_rgb` continuam
  rejeitados como no vanilla nos quatro perfis;
- `color.hsl`, `color.hsv`, `color.linear-rgb` continuam `MATCH_VALUE`;
- `calc.nope`, `sym.nope`, `color.map.nope` continuam `MATCH_DIAGNOSTIC`;
- `std.rgb`, `calc.abs`, `sym.arrow`, `color.map.turbo` preservam sucesso.

### P1303

- sem `a11y-extras`, `pdf.data-cell`, `pdf.header-cell` e
  `pdf.table-summary` continuam `MATCH_DIAGNOSTIC`;
- com `a11y-extras`, continuam `MATCH_VALUE`;
- zero divergência por ordem/repetição.

Qualquer regressão recebe prioridade absoluta e impede selecionar dívida antiga.

## Fase D — refinamento de `repr`

O probe histórico `repr((type(path), repr(path)))` não distingue conteúdo
correto de representação truncada. Criar uma suíte de refinamento somente
observável para:

### Módulos

- `repr(std)`;
- `repr(color.map)`;
- `repr(calc)`, `repr(sym)`, `repr(pdf)`;
- módulo importado nomeado e módulo anônimo, se construíveis sem I/O opaco.

Medir forma, nome público, kind e diferença entre módulo global e módulos
ordinários. Não inferir que todos devam usar a mesma string a partir apenas de
`std`.

### Arrays grandes de `color.map`

Para os dez mapas classificados como divergentes no P1299, separar:

1. kind e cardinalidade;
2. valores nos índices `0`, `1`, `39`, `40`, penúltimo e último;
3. digest canônico integral dos canais RGBA na ordem pública;
4. `repr` integral, incluindo limite de elisão, marcador e quebra de linha.

Incluir controles `color.map.viridis`, `inferno`, `magma`, `plasma` e
`spectral`, que já coincidiam. Se kind/cardinalidade/digest coincidirem e só a
string `repr` divergir, classificar no owner `compiler/eval/repr`; caso contrário,
classificar como semântica de dados no owner `stdlib/color`.

Não usar stdout truncado do relatório como digest. O runner deve calcular o
digest a partir de valores observados ou de chamadas indexadas reproduzíveis.

Gerar `p1304-repr-refinement.json` e um recibo com a inferência e sua condição
de refutação.

## Fase E — prontidão do trio `*.encode`

Reconfirmar presença, kind, `repr`, assinatura e chamadas do vanilla para:

- `json.encode(value, pretty: true|false)`;
- `toml.encode(dict, pretty: true|false)`;
- `yaml.encode(value)`.

O cristalino deve continuar `VANILLA_ONLY` para os três antes que sejam
considerados dívida atual. Medir no vanilla, no mínimo:

- dict ordenado, array, `none`, bool, int, float e strings com escaping;
- nesting e multiline;
- diferença pretty/compact para JSON e TOML;
- TOML com argumento não-dicionário;
- named inesperado, falta de argumento e argumento extra;
- um valor opaco cujo serializer vanilla use representação pública, sem
  converter um caso não observável em igualdade.

Auditar causalmente:

- `stdlib/loading.rs` possui decoders e somente `cbor.encode`;
- `eval/mod.rs` registra `json`, `toml` e `yaml` como funcs sem namespace;
- a implementação do trio exigiria pelo menos os owners L0 `stdlib/loading` e
  `compiler/eval`, além do owner de testes;
- qualquer nova função pública Rust, tipo/variant, default ou mudança de fase
  deve ser marcada para gate ADR-0127.

Gerar `p1304-encode-readiness.json`. P1304 não escreve L0 nem código do trio.

## Fase F — novo ledger semântico

Produzir `p1304-owner-ledger.tsv` a partir do ledger P1299, sem editar o
original. Incluir os 160 paths residuais, `repr(std)` e qualquer regressão nova.

Colunas mínimas:

```text
path
profiles
runtime_class_by_profile
previous_class
current_language_class
crystalline_owner_file_line
vanilla_source_file_line
owner_prompt
owner_prompt_hash_declared
owner_prompt_sha256
l0_claim
gate_class
inference
refutation
recommended_action
```

Classes semânticas permitidas:

- `CLOSED_CONFIRMED`;
- `MISSING_LANGUAGE_MEMBER`;
- `WRONG_PUBLIC_KIND_OR_IDENTITY`;
- `WRONG_PUBLIC_REPR`;
- `DIAGNOSTIC_DIVERGENCE`;
- `L0_CONTRADICTION`;
- `INTENTIONAL_PRODUCT_EXTENSION`;
- `EXPECTED_FEATURE_DISABLED`;
- `NEW_REGRESSION`;
- `UNRESOLVED_OWNERSHIP`;
- `UNRESOLVED_UNKNOWN`.

Publicar contagens antes e depois, com soma verificável. Nenhum path pode sumir
por deduplicação silenciosa, mudança de spelling ou classificação bilateral.

## Fase G — seleção determinística do P1305

Selecionar exatamente uma coorte, aplicando esta prioridade:

1. `NEW_REGRESSION` em passo já certificado;
2. `L0_CONTRADICTION` com rota canônica funcional;
3. `DIAGNOSTIC_DIVERGENCE`;
4. `WRONG_PUBLIC_REPR`/`WRONG_PUBLIC_KIND_OR_IDENTITY` que caiba em um único
   owner produtivo e não exija entidade/API/fase nova;
5. `MISSING_LANGUAGE_MEMBER` com um único owner semântico e representação já
   disponível;
6. demais dívidas, por menor expansão arquitetural.

Nunca selecionar automaticamente:

- `INTENTIONAL_PRODUCT_EXTENSION` — requer reabrir contrato por decisão humana;
- `EXPECTED_FEATURE_DISABLED` — não é defeito;
- qualquer `Unknown`/ownership inválido;
- `color.spot`/`color.spot.tint` ou `outline.entry/*` sem o gate público já
  identificado.

Desempate dentro da mesma prioridade, nesta ordem:

1. menos owners produtivos/L0;
2. zero tipo/variant/trait/assinatura Rust pública nova;
3. zero mudança de default/fase;
4. maior quantidade de paths fechados por uma causa única;
5. menor superfície de regressão;
6. ordem lexicográfica do ID da coorte.

Coortes candidatas obrigatórias, sem preferência prévia:

- `repr-module-and-large-array`: `repr(std)`, `color.map` e somente os mapas
  cuja causa for comprovadamente o formatter público;
- `loading-encoders`: `json.encode`, `toml.encode`, `yaml.encode`;
- qualquer coorte mais prioritária descoberta pela matriz fresca.

O relatório deve mostrar a tabela de pontuação e explicar por que a vencedora
vence. “Parece fácil” ou “foi recomendada antes” não constitui critério.

Para a coorte vencedora, classificar o P1305 como:

- `ADR-0127_CONTINUOUS_PARITY_CORRECTION`, se for somente fórmula/tabela/repr
  interna já legitimada e sem default/API/fase/quebra; ou
- `ADR-0127_HUMAN_GATE_REQUIRED`, se adicionar/remover superfície pública,
  mudar default, fase ou compatibilidade.

P1304 só seleciona e escreve o gate esperado; não executa o gate e não redige a
mudança L0 da coorte.

## Fase H — verificação independente

O verificador recebe manifesto, matrizes, refinamentos e ledger, mas não corrige
nenhum deles. Deve recomputar:

- hashes e baseline composto;
- número de probes/runs por perfil;
- classificação de uma amostra determinística de cada classe;
- todos os paths das coortes finalistas;
- contagens e somas do ledger;
- ausência de `EXECUTION_UNKNOWN` nos casos obrigatórios;
- preservação P1300/P1301r2/P1303;
- aplicação literal da prioridade/desempate;
- allowlist de escrita.

Executar ainda:

```bash
cargo fmt --all -- --check
cargo test -p typst-core p1300 -- --test-threads=1
cargo test -p typst-core p1301 -- --test-threads=1
cargo test -p typst-core p1303 -- --test-threads=1
cargo build
crystalline-lint .
crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
```

Como P1304 não edita Rust/L0, `cargo test --workspace` pode reutilizar o recibo
P1303 se — e somente se — todos os hashes de produto/L0 forem idênticos aos
pinados. Qualquer drift obriga nova execução integral.

## Artefatos e allowlist de escrita

P1304 pode criar somente:

```text
00_nucleo/materialization/typst-passo-1304.md
00_nucleo/diagnosticos/p1304-baseline-status.txt
00_nucleo/diagnosticos/p1304-manifest.json
00_nucleo/diagnosticos/p1304-probe-catalog.json
00_nucleo/diagnosticos/p1304-feature-matrix.json
00_nucleo/diagnosticos/p1304-run-matrix.py
00_nucleo/diagnosticos/p1304-repr-refinement.json
00_nucleo/diagnosticos/p1304-encode-readiness.json
00_nucleo/diagnosticos/p1304-owner-ledger.tsv
00_nucleo/diagnosticos/p1304-verification-receipt.json
00_nucleo/diagnosticos/p1304-decision-report.md
00_nucleo/diagnosticos/p1304-certificate.json
```

Logs integrais grandes podem ser incorporados nos JSONs ou criados como
`p1304-*.log`, desde que declarados previamente no manifesto. Não modificar
artefatos históricos, produto, L0 ou testes.

## Vereditos

O único veredito positivo é:

```text
P1304_PASS_P1305_COHORT_SELECTED
```

Vereditos impeditivos:

- `P1304_BLOCKED_BASELINE_DRIFT`;
- `P1304_BLOCKED_P1303_REGRESSION`;
- `P1304_BLOCKED_BUILD`;
- `P1304_BLOCKED_EXECUTION_UNKNOWN`;
- `P1304_BLOCKED_OWNERSHIP`;
- `P1304_BLOCKED_CLASSIFIER_DISAGREEMENT`;
- `P1304_BLOCKED_SELECTION_TIE`;
- `P1304_BLOCKED_VERIFICATION`;
- `P1304_BLOCKED_SCOPE`.

O certificado deve formar DAG detached `manifesto → certificado → relatório`,
sem auto-hash ou ciclo. Deve limitar a alegação à superfície medida e à seleção;
não declarar paridade funcional geral nem autorização de implementação.

## Orçamento e retorno decrescente

- um build release fresco;
- uma execução completa dos 627 probes;
- uma repetição invertida somente do conjunto decisório e sample de 5%;
- até duas revisões do classificador/refinamento;
- corpus completo novamente apenas se a revisão mudar classificação fora do
  recorte focal.

Duas revisões consecutivas com o mesmo vetor e a mesma causa dominante exigem
parar e emitir diagnóstico de insuficiência. É proibido afrouxar `Unknown`,
remover paths ou mudar a prioridade para fabricar um vencedor.

## Critério de encerramento

P1304 fecha somente quando:

- o baseline composto P1303 foi revalidado por hash e sentinelas;
- 627 probes foram classificados nos quatro perfis sem execução desconhecida;
- os 160 paths residuais e `repr(std)` aparecem no ledger novo;
- os 11 `color.map*` foram separados entre dados/kind e `repr` com digest;
- o trio `*.encode` recebeu medição e auditoria de prontidão;
- contagens e ordens foram verificadas independentemente;
- uma única coorte P1305 foi selecionada pela regra congelada;
- nenhum produto/L0/teste foi editado;
- nenhum staging ou commit foi realizado.
