# Passo 1307 — fechar a família pública de encoders de dados

**Estado:** pronto para execução até ao gate humano ADR-0127.  
**Vanilla ratificado:** `a51e02804`.  
**Objetivo:** materializar `json.encode`, `toml.encode` e `yaml.encode`, sem
regredir os decoders chamáveis nem `cbor.encode`, com contrato, implementação,
ataques e veredito segregados.

Este passo usa o regime de materialização segregada. O repositório é partilhado
entre os papéis; portanto o fecho deve declarar **“executado sem atestação de
isolamento técnico”**. Não alegar independência forte por processo, worktree ou
filesystem. A independência exigida aqui é de autoria dos artefatos e da ordem
causal congelada.

## 1. Medição anterior à decisão — universo fechado de `encode`

Auditoria feita em `2026-09-07`, no HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, sobre o vanilla pinado presente em
`lab/typst-original`. O working tree continha o candidato P1306 ainda não
commitado nos quatro ficheiros tracked seguintes:

- `00_nucleo/prompts/compiler/eval/bindings/field_access.md`;
- `00_nucleo/prompts/compiler/eval/tests.md`;
- `01_core/src/compiler/eval/bindings/field_access.rs`;
- `01_core/src/compiler/eval/tests.rs`.

O certificado P1306 é
`00_nucleo/diagnosticos/p1306-certificate.json`, SHA-256
`9c7008973799dcf02cb3f02b08a76707c5e52a0a63b7738260fad2c06664733d`.
Antes de qualquer alteração P1307, P0 deve voltar a registrar HEAD, status,
diff/stat, hashes dos quatro tracked acima e todos os untracked que compõem o
fecho P1306. Se o estado já estiver commitado, o novo commit torna-se o baseline;
se não estiver, o baseline é composto e deve ser pinado integralmente — não se
mistura silenciosamente P1306 com P1307.

A busca de fonte:

```text
rg -n --glob '*.rs' '^\s*pub fn encode\b|^\s*fn encode\b' \
  lab/typst-original/crates
```

encontrou exatamente quatro funções públicas da linguagem:

| path público | fonte vanilla | cristalino antes de P1307 |
|---|---|---|
| `cbor.encode` | `crates/typst-library/src/loading/cbor.rs:104` | presente; controle positivo |
| `json.encode` | `crates/typst-library/src/loading/json.rs:134` | ausente nos quatro perfis |
| `toml.encode` | `crates/typst-library/src/loading/toml.rs:107` | ausente nos quatro perfis |
| `yaml.encode` | `crates/typst-library/src/loading/yaml.rs:111` | ausente nos quatro perfis |

O inventário P1299 confirma os mesmos quatro e nenhum quinto path
`*.encode`. A prontidão P1304
`00_nucleo/diagnosticos/p1304-encode-readiness.json`, SHA-256
`156f56b9f65ad5fd4310aa7785d4667c1f2c20e13f3f5c8be43b280700e7b950`,
mediu os três ausentes como `VANILLA_ONLY` em `default`, `html`, `a11y` e
`html+a11y`.

**Decisão do universo:** P1307 inclui todos os `encode` públicos ausentes — e
somente eles. `csv.encode`, `xml.encode`, `read.encode`, encoders de
PDF/SVG/HTML/imagem, percent-encoding e helpers internos não são membros
públicos ratificados. `csv.encode` e `xml.encode` entram como controles
negativos para impedir invenção de superfície.

Esta conclusão é sobre nomes públicos, não sobre todo o domínio de valores que
cada serializer aceita. A medição P1304 cobriu primitivos, arrays, dicionários,
ordem, escaping, multiline, nesting, argumentos e `Color` opaca, mas advertiu
corretamente que `Symbol` e `Content` possuem serialização dedicada no vanilla.
P1307 deve ampliar o oracle antes de alegar paridade total dos encoders.

## 2. Classificação ADR-0107 / ADR-0127

Presença, assinatura Typst, valor textual, ordem, escaping, quebras de linha,
indentação, defaults, nomes/repr das funções, mensagens, hints e spans são
observáveis da linguagem. O tipo intermediário Rust, uso de `serde`, árvore do
parser, algoritmo do emitter e igualdade byte a byte de estruturas internas
não são critérios; os bytes da `Str` devolvida são, porque constituem o valor
público.

Adicionar estes três fields expande a superfície pública; `pretty: true` em
JSON/TOML também introduz comportamento por defeito. O passo é portanto
`ADR-0127_HUMAN_GATE_REQUIRED`.

> **TRAVA:** até à confirmação explícita do dono, é permitido medir, congelar
> contrato/oracles documentais e editar/resselar os Prompts L0. Não escrever
> teste RED produtivo nem código L1 antes da confirmação.

## 3. Owners e fronteiras autorizadas

Antes do gate, ler integralmente e auditar os quatro owners vigentes:

| owner L0 | consumer exclusivo | obrigação P1307 |
|---|---|---|
| `00_nucleo/prompts/compiler/stdlib/loading.md` | `01_core/src/compiler/stdlib/loading.rs` | serialização pura, nativas e testes unitários do formato |
| `00_nucleo/prompts/compiler/stdlib/_comum.md` | `01_core/src/compiler/stdlib/mod.rs` | reexports internos explícitos das três nativas |
| `00_nucleo/prompts/compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` | namespaces e registo no scope global |
| `00_nucleo/prompts/compiler/eval/tests.md` | `01_core/src/compiler/eval/tests.rs` | regressões públicas e spans resolvíveis |

Não criar segundo consumer para qualquer prompt, não apontar código para um
Núcleo Tekt e não deslocar lógica de serialização para o hub `stdlib/mod.rs`.
Se surgir necessidade de mudar `Value`, `Symbol`, `Content`, `Func`, `Args`,
um contrato externo, Cargo/dependências ou outro consumer, **parar e reabrir o
escopo/L0**. P1307 não autoriza nova variant, trait público nem crate nova.

O helper público já autorizado
`compiler::eval::repr_value_for_serialization(&Value)` pode ser consumido para
o fallback textual; não duplicar `Debug` e não ampliar novamente sua assinatura.

## 4. Contrato público a escrever nos L0

Após a medição completa de P1 e antes de código, os owners devem fixar:

```typst
json.encode(value, pretty: true) -> str
toml.encode(dictionary, pretty: true) -> str
yaml.encode(value) -> str
```

Obrigações cumulativas:

1. `json`, `toml` e `yaml` continuam funções chamáveis de decode e ganham um
   namespace fechado com exatamente `encode`.
2. Cada membro tem `type == function` e `repr == "encode"`; o nome interno
   observável não pode ficar `json.encode`, `native_json_encode` etc.
3. `json.encode` aceita um `Value` posicional e somente o named booleano
   `pretty`; seu default é `true`.
4. `toml.encode` aceita um `Dict` posicional e somente o named booleano
   `pretty`; seu default é `true`.
5. `yaml.encode` aceita um `Value` posicional e nenhum named.
6. A ordem de inserção de `Dict` é preservada quando observável.
7. O resultado é exatamente a `Str` pública do vanilla para o mesmo valor,
   inclusive trailing newline, delimitadores, escaping, indentação e estilo
   multiline.
8. `None`, booleanos, inteiros, floats finitos/não finitos, strings Unicode e
   de controle, `Bytes`, `Symbol`, `Content`, arrays, dicts e o fallback de
   cada variante opaca atualmente construtível seguem a classificação
   `Serialize for Value` da fonte vanilla
   `foundations/value.rs:343-364`; não generalizar o fallback de `Color` para
   `Symbol` ou `Content`.
9. JSON/TOML/YAML são formatos human-readable: `Bytes` segue a representação
   textual pública medida, enquanto CBOR continua byte-string.
10. Falhas de cast, argumento, valor não representável e serialização preservam
    mensagem, cardinalidade, hints e span público do vanilla. O span do erro
    emitido pelo serializer deve ancorar no valor posicional, quando assim
    medido; detached ou chamada inteira não contam como equivalentes.
11. Os quatro perfis expõem a mesma família; os encoders não dependem de target
    HTML nem de `a11y-extras`.
12. `cbor.encode` e os sete decoders `read/csv/json/yaml/toml/cbor/xml`
    preservam valores, chamadas, namespace e diagnósticos preexistentes.

Se `.with` no parent ou no próprio encoder produzir comportamento público
distinto, P1 mede-o e o L0 congela o resultado. Não inferir automaticamente do
precedente de `place.flush`; `Unknown` bloqueia essa alegação.

## 5. P0 — congelar baseline e proveniência

O coordenador registra em
`00_nucleo/diagnosticos/p1307-baseline.json`:

- HEAD, `git status --short`, `git diff HEAD --stat`, diff tracked integral e
  SHA-256 de cada um;
- hashes dos quatro L0 e quatro consumers acima;
- binário vanilla `/usr/local/bin/typst` com SHA-256 esperado
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- hash/argv do binário cristalino fresco; não reutilizar `target/release/typst`
  sem provar que foi construído do baseline pinado;
- versão/hash dos scripts, fixtures, cwd, env relevante, perfis, timestamps e
  stdout/stderr integrais.

Mudança de estado entre medição, RED, GREEN ou mutação cria nova revisão
explicitamente ligada; não sobrescrever evidência.

## 6. P1 — autor do contrato mede antes de decidir

Um autor de contrato, sem editar implementação, cria
`p1307-contract-measurement.json` e `p1307-contract.md`. Medir bilateralmente,
nas ordens normal, repetida e integralmente invertida:

- presença/kind/repr dos quatro encoders nos quatro perfis;
- assinatura válida, default e valores `pretty: true/false`;
- ausência, excesso, named inesperado e tipo errado;
- parent decoder antes/depois do namespace;
- acesso direto e todas as rotas `.with` efetivamente aceitas pelo vanilla;
- escalares, strings vazias/Unicode/quotes/backslash/control/multiline;
- arrays vazios/aninhados e dicts vazios/aninhados com ordens não alfabéticas;
- `none`, `Bytes`, `Symbol`, `Content` simples e cada classe opaca atualmente
  construtível (`Color` é sentinela mínima);
- floats `NaN`, `inf`, `-inf`, `-0.0` e limites representativos;
- TOML com `none` em dict, arrays heterogêneos e não-dict no topo;
- erros completos com source numerizada e ranges half-open resolvidos;
- negativos `csv.encode`, `xml.encode` e `read.encode`;
- round-trips semanticamente válidos `json(json.encode(...))`,
  `toml(toml.encode(...))`, `yaml(yaml.encode(...))` e controle CBOR.

Não exigir round-trip quando o formato ou o serializer ratificado rejeitar o
valor. Classificar cada célula como valor, diagnóstico, ausência esperada ou
`Unknown`; `Unknown` obrigatório nunca vira PASS. Toda inferência deve declarar
o que a refutaria.

Orçamento de refinamento do adaptador: no máximo duas revisões focais. Cada
revisão preserva a tentativa anterior, hipótese, delta, custo e razão de
ganho. Se a segunda ainda não tornar observável um caso obrigatório, parar e
reabrir o contrato; não reduzir a barra silenciosamente.

## 7. P2 — autor do oracle congela expectativas antes do candidato

Um autor diferente produz, sem ler eventual patch de implementação:

- `p1307-oracle.json`: entradas literais e observáveis integrais esperados;
- `p1307-oracle-receipt.json`: hashes do contrato, baseline, fontes ratificadas
  e fixtures;
- `p1307-mutant-plan.json`: mutantes e testemunha que deve matar cada um;
- script bilateral reproduzível que não calcula expectativa chamando código
  candidato.

O oracle pode usar resultados vanilla literais congelados. Não normalizar
whitespace do valor serializado, não ordenar mapas no comparador, não aceitar
“qualquer erro” e não comparar apenas round-trip: dois encoders com morfologia
divergente podem decodificar ao mesmo valor.

## 8. P3 — L0-first e gate humano obrigatório

O autor do contrato edita primeiro os quatro Prompts L0 da §3, acrescentando a
medição e o contrato final da §4. Em seguida:

1. validar ownership/núcleos antes do resselo;
2. executar `crystalline-lint --fix-hashes .` somente se V15/V26 estiverem
   verdes;
3. registrar hashes L0 pós-edição e headers esperados;
4. executar o linter em modo leitura e `git diff --check`;
5. publicar `p1307-pre-gate-seal.json`.

### PARAR AQUI

Pedir confirmação explícita do dono para a nova superfície e os defaults
`pretty: true`. Sem confirmação, o passo termina como
`P1307_WAITING_FOR_ADR0127_OWNER_CONFIRMATION`. Não escrever testes RED nem
código L1, não editar Cargo e não antecipar headers de código.

## 9. P4 — depois da confirmação: teste RED independente

Somente após a confirmação, o autor do oracle materializa os testes:

- testes puros de serializer em `stdlib/loading.rs` com valores em memória;
- testes públicos de eval em `eval/tests.rs` para namespaces, chamadas,
  defaults, diagnóstico e span;
- matriz CLI bilateral nos quatro perfis para valores textuais integrais.

O RED deve falhar pela ausência exata dos três membros. Erro de compilação do
harness, fixture inválida, binário errado, timeout ou falha P1306 não conta.
Guardar `p1307-red.json` com comandos e outputs integrais.

## 10. P5 — implementação L1 e sequência de redesenhos

O implementador recebe contrato e selo, mas não altera expectativas. Deve
tentar em ordem e registrar cada tentativa; passar à seguinte somente quando
um observável congelado refutar a anterior.

### Desenho A — adapters privados por formato (preferido)

- Em `loading.rs`, criar nativas `native_json_encode`,
  `native_toml_encode`, `native_yaml_encode` e helpers privados puros.
- Usar um adapter privado por referência ao `Value`; não implementar
  `Serialize` global/public para `Value` neste passo.
- JSON/TOML podem delegar aos serializers já dependentes, desde que o adapter
  reproduza as branches vanilla, inclusive `Symbol`/`Content` dedicados e
  fallback por `repr_value_for_serialization`.
- Para YAML, converter para a árvore `saphyr::Yaml` e usar `YamlEmitter` apenas
  se os bytes públicos coincidirem integralmente com o oracle vanilla.

### Desenho B — árvore semântica privada + emitters finos

Se A divergir por capacidade/ordem/forma do emitter, criar em `loading.rs` uma
árvore **privada** comum que preserve a classificação vanilla e adaptadores
finais separados para `serde_json`, `toml` e `saphyr`. Ela não entra em API,
entidades ou outro owner. Nenhum formato pode herdar acidentalmente as
restrições do TOML.

### Desenho C — writer YAML privado compatível

Se somente o emitter `saphyr` divergir na morfologia pública, manter os
adapters validados de JSON/TOML e escrever um writer YAML privado e mínimo,
dirigido pelo contrato congelado, cobrindo integralmente o domínio aceito pelo
vanilla para os `Value` cristalinos. Não adicionar `serde_yaml`/`serde_yml`,
pois a decisão vigente ADR-0111 as excluiu. Não special-case fixtures: regras
de quoting, multiline, sequência, mapa e escalares precisam de classes de
equivalência e ataques.

Se C ainda falhar num observável obrigatório, parar como
`P1307_REDESIGN_REQUIRED`; não introduzir dependência nova, não dividir o
contrato já confirmado e não marcar paridade parcial como fecho.

No registo de eval, os três parents usam `Func::native_with_namespace`; cada
namespace define somente `encode` com nome público curto `encode`. O hub
`stdlib/mod.rs` faz reexports explícitos, sem wildcard e sem lógica.

Após implementação, atualizar os quatro headers `@prompt-hash` e nunca usar
`--fix-hashes` como substituto da auditoria 1:1.

## 11. P6/P7 — GREEN e ataques adversariais

O GREEN repete exatamente o corpus RED e produz `p1307-green.json`. Um
verificador distinto valida contrato, outputs, lineage e ausência de mudanças
fora do allowlist.

Mutantes mínimos, reais e aplicáveis:

1. instalar somente um ou dois encoders;
2. criar aliases flat em vez de namespaces;
3. substituir o decoder pelo módulo/não preservar chamabilidade;
4. nome público `json.encode` em vez de `encode`;
5. perder namespace numa rota `.with` comprovada;
6. default JSON `pretty: false`;
7. default TOML `pretty: false`;
8. YAML aceitar/ignorar `pretty`;
9. TOML aceitar não-dict no topo;
10. ordenar dicionário ou trocar duas chaves;
11. remover/duplicar trailing newline;
12. alterar escaping Unicode/controle ou multiline;
13. usar `Debug` no fallback opaco;
14. tratar `Symbol` ou `Content` como fallback genérico;
15. tratar `Bytes` human-readable como byte array;
16. alterar `none`, float não finito ou `-0.0`;
17. erro detached ou span de chamada inteira;
18. regredir `cbor.encode` ou um decoder;
19. inventar `csv.encode` ou `xml.encode`;
20. expor os membros somente em um perfil.

Mutante que não aplica/compila não é morte. Corrigir a instrumentação,
preservar tentativa e restauração, e exigir testemunha observável. Fecho:
`20/20` famílias aplicáveis mortas, `mutation_score = 1.0`, zero sobrevivente e
zero `Unknown` obrigatório.

## 12. P8 — gates integrais e rebaseline

Executar, com comandos/saídas/tempo/hash registrados:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
crystalline-lint .
git diff --check
```

Além disso:

- repetir a matriz de presença P1299/P1304 para os quatro `encode`;
- repetir o corpus P1307 em ordem normal, repetida e invertida;
- rodar as sentinelas P1300–P1306 pertinentes (módulos, repr, imports, spans e
  feature gates);
- atualizar a contagem de paridade somente a partir de matriz bilateral fresca;
- provar que os únicos paths que mudaram de `VANILLA_ONLY` para igualdade são
  `json.encode`, `toml.encode` e `yaml.encode`, salvo diferença já prevista e
  explicada pelo corpus de chamadas;
- bloquear `EXECUTION_UNKNOWN`, timeout, crash, saída não parseável ou binário
  sem proveniência.

Não usar apenas `cargo test`; a aceitação primária continua
`crystalline-lint .` com zero violations e a matriz pública bilateral.

## 13. P9 — artefatos e veredito

O coordenador publica manifesto acíclico e um verificador distinto emite
`p1307-certificate.json`. Artefatos mínimos:

```text
p1307-baseline.json
p1307-contract-measurement.json
p1307-contract.md
p1307-oracle.json
p1307-oracle-receipt.json
p1307-mutant-plan.json
p1307-pre-gate-seal.json
p1307-red.json
p1307-green.json
p1307-mutant-ledger.json
p1307-gates.json
p1307-final-measurement.json
p1307-verification.json
p1307-certificate.json
p1307-final-report.md
```

Veredito PASS exige cumulativamente:

- confirmação humana registrada após o selo L0 e antes do RED/código;
- três encoders presentes e iguais no contrato público nos quatro perfis;
- `cbor.encode`, decoders e sentinelas preservados;
- todos os outputs/diagnósticos obrigatórios observáveis, sem Unknown;
- mutation score `1.0`;
- owners 1:1, pins/núcleos válidos, headers corretos;
- gates integrais verdes;
- relatório que delimite exatamente o fragmento certificado e dívidas
  restantes, sem alegar paridade total da linguagem.

Não fazer commit, stage, push ou apagar artefatos sem pedido explícito do dono.
