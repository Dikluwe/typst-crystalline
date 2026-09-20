# P1463 — catálogo adversarial pré-candidato de `format.png`/`ppi`

**Estado:** CATÁLOGO R1 PRÉ-CANDIDATO; P00 não fornecido; mutantes não
materializados nem executados; nenhum selo ou veredito emitido

**Papel:** adversário `/root/p1463_adversario`

**Regime:** protocolo completo de materialização Tekt segregada

**Alegação proporcional:** **executado sem atestação de isolamento**

**Baseline causal:** `bf0a29cad5423f038cb51c8c8a079465a1d4ed65`

**Tree causal:** `3fa973a4461c438769b22c0971eccc041dade422`

**Manifesto R1:** `00_nucleo/diagnosticos/p1463-manifesto.json`, SHA-256
`4f087f97ea10ba298a97d9b1b0b32901d2538a06eb7cb311f8fe44ff6b20b739`

**Contrato R1:** `00_nucleo/diagnosticos/p1463-contrato-congelado.md`, SHA-256
`4a8baaf1aa23b6610205e1da1868d0f5577f25020379c26354537651337185da`

**Catálogo canônico legível por máquina:**
`00_nucleo/diagnosticos/p1463-mutantes.json`, SHA-256
`25b8b6dafca2d360f8828d46511c5e92ba48ab4c25070195e753e280821776da`

Este artefato foi escrito sem candidato P1463 e não escolhe nem corrige uma
implementação. As receitas são operadores construtivamente compiláveis sobre
uma futura base positiva P00 que satisfaça os L0s; compilação observada só pode
ser alegada depois da materialização em cópia temporária. Nesta fase foram
executadas apenas leituras, validação de identidade e validação do JSON/diff.

## 1. Autoridade, proveniência e embargo

A autoridade adversarial leu o Passo 1463, os oito Prompts L0 P1463, o Núcleo
pinado, manifesto/contrato R1 e os pontos produtivos do baseline listados no
manifesto. Pode escrever somente este Markdown e o JSON companheiro. Não pode
editar código, teste, L0, Núcleo, contrato, manifesto ou oráculo; não pode
escolher estrutura privada, corrigir solução, emitir selo ou invalidar um
mutante sobrevivente por conveniência.

Medição de entrada em `2026-09-19T21:19:01-03:00`:

- checkout compartilhado `/repos/Antigravity/typst-crystalline`;
- HEAD e tree iguais ao baseline causal acima;
- `git diff --name-only <baseline> -- 01_core 02_shell 03_infra 04_wiring`
  sem saída;
- working tree antes destes dois outputs:
  `?? p1463-contrato-congelado.md`, `?? p1463-manifesto.json` e
  `?? 04_wiring/tests/p1463_format_png.rs` com caminhos completos sob o repo;
- candidato produtivo: ausente;
- mutações executadas: zero;
- oráculos `p1463-oraculos-*`: ausentes.

O teste A/B apareceu no workspace durante esta autoria por ação de outra
autoridade. O adversário registrou somente seu caminho pelo `git status`; não
leu nem calculou hash do conteúdo. Antes de qualquer vetor, o verificador deve
congelar autoria, hash e comando desse teste. A ausência atual de oráculos e de
uma base positiva P00 é `Pending` operacional, não `Unknown` semântico.

Hashes raw dos L0s recebidos e conferidos:

| Prompt | SHA-256 raw |
|---|---|
| `compiler/stdlib/format.md` | `892e1d56d20189a4eb0c95d13383d5517156533a886f6db263be20fc7aeafc37` |
| `compiler/eval/rules.md` | `94444882ec98bf6ad932513745b9c0e1834aabde2f5f6cb5410da3fbd755fed4` |
| `compiler/layout/format_options.md` | `9a4f4a41618dfa396aca8efe9301a718660055a0659cbce362f20a10e0dc0d79` |
| `entities/document_format_options.md` | `52dbeeec49bba1992fb6b86f936cb7943c0af10fd6148d78967b1a12f2df709f` |
| `shell/cli.md` | `4f0cf474c2baa41102867dbf06a8589047803b6a6a791b872d1266640d83c4b9` |
| `infra/pipeline.md` | `ea9c658e42c68ec807cc159bddb75b8efbbabca021cd210cebdf8626cca95829` |
| `wiring.md` | `7f9adfb551b0cc69dc619f76a141138080a1f70448b7c2a89c9c85aab829d70e` |
| `wiring/tests/p1463-format-png.md` | `19a9dbdadc35949890ce45f691ee1ab4da2dbbc8a8747f496ac6dc8fe84b94b2` |

O Núcleo `format-option-precedence.toml` tem SHA-256 raw
`488b0699748cd2a662c0ac47b8dc3c1b801e3df7afc5e842c74f66be5b1d59e8`
e pin efetivo V26
`68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11`.
Os oito hashes do código baseline também coincidem byte a byte com
`baseline_code_inputs` do manifesto.

## 2. Validade, equivalência e classificação

Cada M01–M18 é uma classe congelada com uma instância mínima planejada. Uma
instância futura só é **válida** se:

1. partir de uma cópia nova do mesmo P00 pinado, sem outro mutante;
2. aplicar apenas o desvio catalogado e registrar diff/hash;
3. compilar; para M15–M17, cujo delta é comentário/metadata, o Rust deve
   continuar compilável e o gate estático deve ser alcançável;
4. alcançar a testemunha indicada, sem falha anterior não atribuível;
5. mudar uma obrigação congelada e conservar os controles de fronteira.

Estados de execução:

- `Preserved`: resultado esperado somente de P00;
- `Violated`: resultado esperado de todo mutante válido, com witness pública;
- `Survived`: mutante válido não rejeitado; entra no denominador, bloqueia
  score `1.0` e selo;
- `Unknown`: caso concluído sem evidência pública suficiente, timeout/crash do
  produto, parser/artefato opaco ou identidade ambígua; não recebe crédito;
- `Interrupted`: unidade não concluída por causa externa comprovada; não apaga
  checkpoints válidos;
- `Excluded-Equivalent`: delta compila, mas não muda obrigação observável;
- `Excluded-NonCompilable`: construção do mutante não compila ou não alcança
  a witness; exige nova instância da mesma classe;
- `Pending`: ainda não iniciado.

O score é `mutantes válidos Violated / mutantes válidos`. Além de score `1.0`,
cada classe precisa de uma instância válida: mínimo `18/18`. Exclusão não
satisfaz cobertura; `Unknown` e `Interrupted` não matam mutante.

## 3. Catálogo M01–M18

Todos os estados atuais são `Pending` e a classificação esperada após execução
válida é `Violated`.

| ID | Mutação compilável mínima em cópia temporária | Witness pública mínima | Ficheiros temporários | Timeout focal | Validade/equivalência |
|---|---|---|---|---:|---|
| M01 | Trocar somente a chave pública do binding `png` por `pnq`, mantendo a `Func` nominal `png` e seu namespace. | O01: tipo/nome/descoberta direta, `std` e alias nos quatro perfis. | `01_core/src/compiler/stdlib/format.rs` | 300 s | Válido se apenas `format.png` some e `pnq` aparece. Renome de variável privada é equivalente; factory quebrada é não compilável. |
| M02 | No namespace público PNG, trocar `Value::Float(144.0)` por `Value::Float(72.0)`. | O02: `repr((type(format.png), format.png, format.png.ppi)) == (function, png, 144.0)`. | `01_core/src/compiler/stdlib/format.rs` | 300 s | Precisa isolar o namespace se P00 usar constante compartilhada. Literal morto é equivalente. |
| M03 | No native constructor PNG, trocar o erro por `Ok(Value::None)` sem mudar assinatura. | O03: `format.png()` deve falhar com `cannot be constructed manually`. | `01_core/src/compiler/stdlib/format.rs` | 300 s | Válido se a chamada passa e identidade/default continuam acessíveis. Outra construção do mesmo erro é equivalente. |
| M04 | Trocar o erro de named desconhecido por `continue`, ignorando apenas `nope`. | O05: `#set format.png(nope: 72)` falha pelo canal normal, sem warning legado/artefato. | `01_core/src/compiler/eval/rules.rs` | 300 s | Válido com integer/float positivos ainda alcançáveis. Reordenação que preserva erro é equivalente. |
| M05 | Quando nenhum `ppi` foi visto, empurrar `format.png.ppi = 144.0` na regra vazia. | O06: `ppi:72` seguido de `#set format.png()` permanece `72×72`. | `01_core/src/compiler/eval/rules.rs` | 300 s | Calibrar à forma final de ausência (`bool`/`Option`). Criar literal não inserido é equivalente. |
| M06 | Validar/contar todas as ocorrências root PNG, mas atribuir `options.png.ppi` somente na primeira. | O07: `72→288` termina 288 e `288→72` termina 72. | `01_core/src/compiler/layout/format_options.rs` | 300 s | Calibrar sentinela separada; `144.0` explícito é válido e não pode ser sentinela. Reverter também SVG seria mutação composta. |
| M07 | Tornar falsa somente a condição que emite erro quando `all_occurrences > root_occurrences`. | O08: root positivo e dois containers negativos, sem artefato. | `01_core/src/compiler/layout/format_options.rs` | 300 s | Válido se root passa e nested passa indevidamente. Mudança de contador sem efeito é equivalente. |
| M08 | Na construção de `CompileIntent`, trocar `ppi: args.ppi` por `Some(args.ppi.unwrap_or(144.0))`. | O10/O12/O14: ausência é `None`; doc `ppi:72` sem flag produz `72×72`. | `02_shell/src/cli.rs` | 300 s | Válido se ausência vira override 144 e vence o documento. Default após encontro documental é equivalente a este contrato. |
| M09 | No braço `None` do resolver L3, dividir `document.format_options.png.ppi` por 72. | O13/O14: doc `ppi:72` produz `72×72`, não `1×1`. | `03_infra/src/pipeline.rs` | 300 s | CLI permanece inalterada. Dividir e multiplicar de volta antes do raster é equivalente. |
| M10 | No braço `Some`, passar `cli_ppi` cru em vez de `cli_ppi / 72.0`. | O13/O14: `--ppi 72` produz `1×1`, não `72×72`. | `03_infra/src/pipeline.rs` | 300 s | Documento permanece cru. Mover a mesma divisão para helper chamado é equivalente. |
| M11 | No braço `Some`, ignorar CLI e devolver o `ppi` documental. | O12/O14: doc 288 + CLI 72 produz `1×1`. | `03_infra/src/pipeline.rs` | 300 s | Válido se o conflito passa a seguir o documento. `match` equivalente a `unwrap_or` não é mutante. |
| M12 | No ramo sem fontes, trocar somente `&render_options` por `&RenderOptions::default()`; ramo com fontes conserva o resolvido. | O16: dimensões iguais entre ramos equivalentes com `ppi` não default. | `03_infra/src/pipeline.rs` | 600 s | Calibrar fixtures que comprovem ambos os ramos. Duas instâncias com mesmo `pixel_per_pt` são equivalentes. |
| M13 | No braço SVG L4, passar `pretty_override.or(ppi.map(|_| true))`, fazendo `--ppi` alterar serialização SVG. | O18: mesma fixture SVG com/sem `--ppi 72` tem saída pública idêntica. | `04_wiring/src/main.rs` | 600 s | Calibrar observador para igualdade pública, não só morfologia. SVG inválido é `Unknown`, não kill válido. |
| M14 | Acrescentar `scope.define("pdf", Value::None)` na factory `format`. | O14/O21: `format.pdf` ausente e P1357 bruto/normalizado exato nos quatro perfis. | `01_core/src/compiler/stdlib/format.rs` | 900 s | Binding público extra é não equivalente. Variável privada `pdf` sem binding é equivalente. |
| M15 | Trocar o `@prompt` de `document_format_options.rs` para `compiler/layout/format_options.md`, sem mudar Rust. | O20: `crystalline-lint --checks v15 --fail-on warning .` rejeita com `PromptOwnership`. | `01_core/src/entities/document_format_options.rs` | 180 s | Rust deve continuar verde. Comentário fora do header reconhecido é equivalente. |
| M16 | Trocar o `@prompt` de `pipeline.rs` diretamente para o TOML do Núcleo. | O20: `crystalline-lint --checks v26 --fail-on warning .` rejeita `NucleusIntegrity`. | `03_infra/src/pipeline.rs` | 180 s | Rust continua verde. Mencionar o caminho em comentário comum é equivalente. |
| M17 | Trocar um nibble somente do pin efetivo em `pipeline.md` (`68c5…` → `78c5…`), mantendo `wiring.md` e Núcleo raw. | O20: V26 rejeita pin unilateral divergente. | `00_nucleo/prompts/infra/pipeline.md` | 180 s | Calibrar o preflight para permitir apenas o delta deliberado na cópia; P00 original continua pinado. Texto fora da linha lida é equivalente. |
| M18 | Trocar apenas o default público SVG `Value::Bool(false)` por `true`. | O19: matriz P1462 integral, começando pelo repr/default nos quatro perfis. | `01_core/src/compiler/stdlib/format.rs` | 900 s | PNG fica intacto. Bool privado não exposto é equivalente; alterar PNG junto é mutação composta. |

M15–M17 são mutantes arquiteturais: o witness é o gate canônico do repositório,
não bytes. A falha esperada específica do gate classifica `Violated`; falha de
build, preflight ou identidade não atribuível não recebe esse crédito.

## 4. Plano executável P00 + M01–M18

### 4.1 Entradas que ainda precisam chegar

Antes da primeira unidade, uma autoridade verificadora diferente do
implementador deve pinçar:

- a identidade da **base positiva P00**. O baseline causal é deliberadamente
  RED e não pode ser chamado P00; este catálogo não decide se P00 será uma
  materialização de referência independente ou outra base autorizada;
- hash e causalidade do teste A/B `p1463_format_png.rs`;
- hashes, comando e IDs públicos dos oráculos P1463;
- SHA-256 do binário P00, toolchain, `crystalline-lint`, manifesto, contrato,
  L0s raw/semânticos e Núcleo raw/efetivo.

Sem esses pins, a campanha permanece `Pending`; não se improvisa expected no
catálogo nem se executa contra o baseline que já viola P1463.

### 4.2 Ordem primária e comandos

Executar serialmente uma única ordem pré-candidato:

```text
P00, M01, M02, M03, M04, M05, M06, M07, M08, M09,
M10, M11, M12, M13, M14, M15, M16, M17, M18
```

Para cada unidade:

1. verificar todos os pins e restaurar uma cópia temporária nova do mesmo P00;
2. em Mxx, aplicar exclusivamente a receita e registrar diff/hash; P00 não
   recebe delta;
3. compilar com
   `cargo build --locked -p typst-wiring --bin typst` (timeout 900 s);
4. executar primeiro o witness focal mapeado e seus controles; processos
   públicos têm timeout 90 s e conjunto focal 300 s, salvo os valores maiores
   da tabela;
5. reutilizar, sem copiar expected para este catálogo:
   `cargo test --locked -p typst-wiring --test p1463_format_png` e o comando
   canônico publicado pelo autor dos oráculos apontado ao binário da unidade;
6. quando aplicável, executar P1357 nos quatro perfis e
   `crystalline-lint --checks v15,v26 --fail-on warning .`;
7. executar `git diff --check` e persistir o recibo atômico antes de avançar.

O verificador deve congelar, antes de P00, o mapa entre cada `public_witness`
do JSON e os IDs efetivos do A/B/oráculos. Assim as mesmas sondas independentes
são reutilizadas e o adversário não ganha autoridade sobre os expected.

P00 só é `Preserved` se O01–O23 aplicáveis, A/B, oráculos, P1357 e V15/V26
passarem. Mxx só é `Violated` quando sua witness pública o rejeitar e os
controles provarem que a rejeição é atribuível ao desvio. Um vetor completo
`P00 Preserved + M01–M18 Violated`, zero `Unknown`, cobertura `18/18`, score
`18/18 == 1.0`, P1357 conclusivo e gates verdes no P00 satisfaz o gate
discriminatório de ADR-0131.

### 4.3 Checkpoints atômicos e retomada ADR-0131

Cada unidade terminada escreve futuramente:

```text
00_nucleo/diagnosticos/p1463-checkpoints/<vector-id>/<sequência>-<id>.json
```

O recibo é criado em arquivo temporário no mesmo diretório, recebe
`flush`/`fsync`, é renomeado atomicamente e o diretório é sincronizado. Campos
mínimos:

- schema/instrument version, `vector_id`, unidade, ordem lógica e hash do
  recibo predecessor;
- hashes de baseline, P00 source/tree/binário, manifesto, contrato, L0
  semântico/raw, Núcleo, A/B, oráculos, mutante/diff e ferramentas;
- comando, cwd, ambiente relevante, timestamps, custo ativo e timeout;
- exit, classificação, `reason_code`, witness e hashes de stdout/stderr,
  artefato e JSON do oráculo;
- estado Git antes/depois e prova de cópia temporária exclusiva.

Após causa externa, marcar a unidade parcial `Interrupted`, auditar pins dos
recibos completos, descartá-la e retomar essa unidade do início em diretório
novo. `Unknown` produzido pelo caso não pode ser repetido oportunisticamente.
Budget de 180 minutos limita unidades futuras; não apaga checkpoints.

Uma repetição universal pré-candidato não é exigida. Na certificação final,
repetir por risco ao menos P00 e as classes M06–M13, M17 e M18, além de nested,
ramos de fontes, targets neutros, P1357 e V15/V26. Divergência sob pins
idênticos é `Unknown` até causa pública.

## 5. Calibração obrigatória antes da execução

Seis classes exigem calibração material, sem mudar a classe:

| ID | Motivo |
|---|---|
| M02 | Isolar o namespace se o P00 compartilhar constante com o carrier; evitar mutação composta. |
| M05 | A forma final de representar ausência em eval ainda não existe. |
| M06 | Implementar first-write-wins com sentinela própria; `144.0` explícito não serve de sentinela. |
| M12 | Provar por observação que os fixtures selecionam realmente os dois ramos de fontes e usam PPI não default. |
| M13 | Confirmar que o observador SVG compara o output público exigido, não apenas morfologia normalizada. |
| M17 | Permitir o delta deliberado só na cópia mutante sem afrouxar os pins protegidos do P00. |

Para qualquer classe, se a âncora sintática não casar com a mecânica livre do
P00, isso é `Unapplicable` provisório, não prova de invalidade da classe. O
adversário pode adaptar somente a forma de aplicação, preservando desvio,
witness e ficheiros autorizados. Duas tentativas sem ganho na mesma causa
reabrem o desenho antes da terceira.

## 6. Riscos e limite da alegação

- A/B chegou durante a autoria, mas segue sem hash e não lido por esta
  autoridade; causalidade precisa de auditoria independente.
- Os oráculos ainda não chegaram e o mapa witness→case não está selado.
- P00 positivo não foi identificado; usar o baseline RED como P00 falsificaria
  o gate.
- A forma privada futura é livre; calibração não pode virar redesign orientado
  ao candidato.
- M15–M17 precisam separar claramente kill arquitetural de erro de preflight.
- Workspace compartilhado e contexto herdado impedem alegar isolamento
  atestado; hashes provam identidade e ordem, não capacidades técnicas.

Este catálogo atesta somente que M01–M18 foram materializados como operadores
adversariais planejados, com witness, exclusões e execução reproduzível. Não
atesta compilação de mutante, poder discriminatório observado, paridade PNG
geral, selo, candidato ou veredito.
