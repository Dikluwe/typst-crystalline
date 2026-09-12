# Passo 1353 — sanitização produtiva pós-P1339 sem mudança semântica

## 1. Objetivo

Sanitizar o produto depois da poda documental P1352:

1. retirar exclusivamente `cfg(p1339_observation)` e a telemetria que só ele
   alimentava;
2. preservar integralmente a estabilização contextual produtiva de P1339;
3. aplicar o `rustfmt` canônico;
4. resselar os consumers alterados;
5. provar por matriz A/B que nenhuma observação pública mudou.

Não há implementação HTML, rota nova, crédito de paridade ou refatoração da
estabilização neste passo.

## 2. Estado medido antes da decisão

Medição inicial `2026-09-12T09:39:46-03:00`:

- branch `Tekt`, HEAD original
  `e77fbd27308d5dc0eb4325e67b3f3f009f56ec2d`;
- árvore inicialmente limpa;
- `cargo fmt --all -- --check`: RED, exit 1;
- sete consumers continham `p1339_observation`;
- referência pública: 4.718 probes × quatro perfis = 18.872 células,
  18.260 matches, 175 paths divergentes e zero Unknown.

Uma reescrita Git externa ocorrida durante P1353 mudou hashes de commits sem
mudar a árvore `0b822176943f36f69b1a1d0576ea572c09de9530`. A proveniência efetiva
e os resets externos devem ser registrados no relatório.

### 2.1 Arquivos inicialmente fora de formato

```text
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/bindings/mod.rs
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/introspect/from_tags.rs
01_core/src/compiler/introspect/locatable.rs
01_core/src/compiler/stdlib/counter.rs
01_core/src/compiler/stdlib/foundations/query.rs
01_core/src/compiler/stdlib/state.rs
01_core/src/entities/introspector.rs
```

Formatar é transformação mecânica. Não aproveitar o diff para alterar
algoritmos, mensagens, literais ou warnings alheios.

### 2.2 Consumers da instrumentação

```text
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/introspect/from_tags.rs
01_core/src/compiler/stdlib/primitives_constructors/array.rs
01_core/src/compiler/stdlib/state.rs
01_core/src/entities/style_chain.rs
03_infra/src/pipeline.rs
03_infra/src/pipeline/context_stabilization.rs
```

Devem permanecer `ContextRead`, `ContextReads`, a comparação
`Same/Different/Unproven`, seleção causal, replay, estabilização seletiva,
cinco tentativas e diagnósticos de não convergência. Permanecem os métodos:

```rust
pub fn has_filtered_counter_reads(&self) -> bool;
pub fn context_reads_valid_for(
    &self,
    candidate: &TagIntrospector,
    engine: &mut Engine<'_>,
) -> SourceResult<bool>;
pub fn context_nonconvergence_diagnostics(
    &self,
    history: &[TagIntrospector],
    engine: &mut Engine<'_>,
) -> SourceResult<Vec<SourceDiagnostic>>;
```

## 3. Classificação

`rustfmt` é mecânico. A remoção não afeta o build normal, mas elimina uma
superfície Rust condicional e sucede cláusulas P1341/P1342. Por prudência,
aplica-se o gate ADR-0127. Não aplicar a skill de materialização segregada nem
criar cadeia candidato/adversário/verificador para esta limpeza.

## 4. Fase A — L0 e gate humano

Atualizar os sete L0 proprietários, declarando aposentadoria da telemetria,
valor somente histórico das medições, preservação do contrato P1339, proibição
de callback/evento substituto e exigência de nova medição/L0/check-cfg para
instrumentação futura.

Antes do código, mostrar o diff L0 integral, confirmar os três métodos e a
estabilização e aguardar. O dono autorizou expressamente a continuação.

## 5. Fase B — baseline A compacta

Compilar o produto anterior à remoção em diretório temporário de RAM e repetir
o catálogo P1335:

- 4.718 IDs distintos;
- perfis `default`, `html`, `a11y`, `html+a11y`;
- vanilla `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- exit code, stdout e stderr exatos, sem normalização;
- sucesso JSON parseável;
- timeout/crash/saída impossível = `EXECUTION_UNKNOWN`.

Esperado, mas obrigatoriamente revalidado:

```text
MATCH_VALUE             18000
MATCH_DIAGNOSTIC          260
VANILLA_ONLY              284
CRYSTALLINE_ONLY          168
DIFFERENT_VALUE            40
DIFFERENT_DIAGNOSTIC      120
EXECUTION_UNKNOWN           0
TOTAL                   18872
paths integralmente iguais 4543
paths divergentes           175
```

Guardar a matriz integral apenas em RAM. O recibo compacto traz proveniência,
hashes, contagens, paths divergentes e SHA-256 canônico das 18.872 células.
Qualquer desvio bloqueia a remoção.

## 6. Fase C — RED/GREEN e remoção estreita

Congelar o RED com:

```bash
rg -n 'p1339_observation' 01_core/src 03_infra/src
```

Remover somente itens condicionados ou exclusivos do modo. Não simplificar a
lógica produtiva adjacente, ocultar warnings com `allow`, renomear o cfg,
reorganizar APIs ou corrigir warning alheio.

GREEN:

```bash
if rg -n 'p1339_observation' 01_core/src 03_infra/src; then exit 1; fi
```

Não pode restar `include!` para diagnóstico removido.

## 7. Fase D — formatação e resselagem

Executar uma vez `cargo fmt --all`. Cada diff é classificado apenas como
`RUSTFMT_ONLY` ou `P1339_OBSERVATION_REMOVAL`; uma terceira classe bloqueia.
Validar V15/V26 e usar `crystalline-lint --fix-hashes .`; nunca editar hashes
à mão.

## 8. Fase E — equivalência e gates finais

Recompilar e repetir a matriz. O hash canônico, contagens e os 175 paths devem
ser idênticos; Unknown deve ser zero; as dez rotas P1339 devem continuar match
nos quatro perfis; não conceder crédito novo.

Executar:

```bash
cargo fmt --all -- --check
cargo build --workspace --release --locked
cargo test --workspace --release --locked --no-fail-fast
crystalline-lint .
git diff --check
```

Confirmar ausência de erro, V5/V15/V26, ocorrência ou warning
`p1339_observation`, registrar a árvore final e contar honestamente warnings
preexistentes.

## 9. Artefatos permitidos

Além de L0, consumers, hashes e deste passo:

```text
00_nucleo/diagnosticos/p1353-baseline-compact.json
00_nucleo/diagnosticos/p1353-final-compact.json
00_nucleo/diagnosticos/p1353-relatorio.md
```

Cada JSON é compacto e menor que 1 MiB. Binários, fixtures e resultados por
célula permanecem somente no temporário.

## 10. Encerramento

Fechar apenas com gate registrado, L0 inequívoco, diff restrito, gates finais,
A/B byte-equivalente no nível observável e proveniência reproduzível. Se uma
probe mudar, declarar `BLOCKED_SEMANTIC_CHANGE`, reverter somente a remoção
causal e abrir passo semântico próprio.

## 11. Fora de escopo

- as 175 divergências e constructors HTML restantes;
- `html.frame` e extensões autorizadas;
- `calc.deg`, `calc.rad`, `calc.log10`;
- redução geral de warnings V16–V20;
- refatoração contextual;
- mudança de API, default, feature ou fase do pipeline.
