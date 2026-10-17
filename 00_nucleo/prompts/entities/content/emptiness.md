# L0 — entities/content/emptiness
Hash do Código: 79f57cfc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/projection-exhaustiveness.toml sha256:71273fd4ed96ebc9bb2910e90f03cde1ecdd3543d7cf02a934c945f2c159a880
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/emptiness.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the semantic `Content::is_empty` projection. The exhaustive static match
must preserve which variants are observable and delegate element-specific
semantics to their owners.

## Acceptance

Existing emptiness behavior and tests remain unchanged; no rendering, I/O, or
dynamic registry is introduced.


## Realização com avaliação única

Vazio visual de carriers internos delega ao body/visual; origem consumida não torna produto visualmente não vazio.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## V16 — projeções explícitas de Content (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tekt-v16-content-views-2026-10-17/before.json.gz registra
UTC/status/diff HEAD --stat, fontes/L0 e 2129 hashes conferidos. contexts-before.json
mapeia os seis V16 por AST/linhas; projection-plan.json guarda o domínio real
Content (120 variantes) e os complementos por função. before-probe-result.json
conserva 17280 controles simbólicos de padrões e guards: o mutante Content
futuro compila nos seis locais (zero E0004 selecionados). É mecânica de proteção
de domínio, sem nova prova de paridade ou de leitura por IA. Payloads são
opacos no modelo; ele não substitui controles reais dos algoritmos.

before-field-probe.json usa a rlib atual e o programa field-control.rs com hash,
argv, UTC e source freeze: 48 de 84 leituras diferem quando Source, Located,
ShowRecipe ou Realized estão dentro de Styled/Styled aninhado. Fora de Styled,
os quatro já preservam campos. fields.rs:10–13 reconhece só Equation/Styled;
seu guard recusa os carriers antes da delegação. O núcleo source-origin já
exige transparência; o owner fields já exige body/visual dos outros três.
É invariância de campos da língua, não exigência de estrutura Rust vanilla:
esses carriers são internos e não têm syntax própria no pin. Não inferir
intenção do defeito; refutam a correção perda de precedência, ausência/none,
campos indevidos em não-equações, mudança do visual para original ou execução
de receitas. O pin de linguagem permanece 9f2b6e8715237cb086899a42873660fe744622e8;
esta medição metamórfica não é declarada comparação bilateral nova.

### Responsabilidade exclusiva

Em Content::is_empty, preservar todos os braços atuais e substituir somente o fallback false por seu complemento nominal de 63 variantes. Essas variantes conservam a classificação estrutural não vazia; não mudar vacuidade de elementos, matemática, texto vazio ou container neste lote.

A lista nominal por função, em ordem da declaração vigente, é derivável das
variantes não cobertas incondicionalmente pelos braços anteriores; guardar
Styled no complemento se o guard puder recusar. Nos campos, variantes com
nome literal ou guard conservam também o caso de campo desconhecido; variantes
já delegadas para qualquer f ficam fora do complemento. O domínio string
continua aberto. O núcleo pinado projection-exhaustiveness partilha esta regra;
o owner continua exclusivo e source-origin conserva sua obrigação vigente.

### Aceitação e limites

L0 antes de testes/código; fluxo contínuo ADR-0127, sem campo/assinatura/trait
público, compatibilidade, default ou fase nova. RED funcional de fields antes
de código, sem usar falhas de montagem de sonda como RED. Após o reparo, os 84
controles reais de carriers devem coincidir; testar precedência de estilos e
não-equações (Realized consulta visual, não original). A sonda estrutural deve
passar de zero para seis E0004 selecionados ao adicionar uma variante somente
no modelo descartável. Seus 17280 casos simbólicos conservam a classificação,
exceto os 96 casos do helper que passam a reconhecer os quatro carriers;
cada variante de cada complemento V19 deve ser exercitada separadamente.
Essa prova não é equivalência de payloads: exige replay dos corpos anteriores,
regressão real e invariância dos algoritmos não alterados. Não adicionar
matriz produtiva que apenas espelhe o match.

Esperado: remover exatamente seis V16, acrescentar seis V19 nominais uniformes
prescritos, preservar os demais diagnósticos por fonte/regra/severidade/mensagem
e não acrescentar V20 ou supressão. Grafo V15/V26 antes de resselo dos cinco
pares. Build, testes focais e workspace, Clippy all-targets -D warnings/-D
unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem estrita/reversa,
replay, source freeze e diff --check. Pesados sequenciais MemoryMax12GiB,
High10GiB/swap0/jobs1. Sem commit/push/merge/release. As frentes de linguagem,
contratos pendentes e o objetivo global não são encerrados por este lote.
