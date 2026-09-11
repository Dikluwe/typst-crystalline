# P1342 — auditoria independente da topologia candidate-free (r1)

## Estatuto, regime e veredito

- Papel: auditor independente de topologia; não autor de obrigação, contrato,
  candidato, ataque ou veredito final.
- Regime: **executado sem atestacao de isolamento**.
- Estado medido: `HEAD 2f42d64253547734564513a1159ee6b584c1c4b4`, working
  tree não commitado, em `2026-09-10T20:26:32Z`.
- Escopo: somente o fragmento vinculante P1342 e fontes candidate-free autorizadas.
  Nenhum candidato futuro P1342 foi lido; nenhum artefato P1341 foi reintroduzido.
- Veredito de topologia: **MAPEADA, MAS NÃO SELÁVEL NO ESTADO ATUAL**. Há uma
  rota causal real para ContextBlock → CounterUpdate → walk/Location → replay do
  callback → Dict. Porém, a ocorrência de `counter.update` perde o seu span antes
  de nascer `CounterUpdateElem`, e o papel sintético `func.body` não corresponde a
  um `Func` real. O próximo passo legítimo é autoria L0 test-only nos owners abaixo,
  seguida de contrato; não é implementação direta.

Este veredito não aprova candidato nem substitui o ataque/verificador segregado.

## Entradas e proveniência

Foram lidos integralmente `tekt-materializacao-segregada/SKILL.md` e as referências
`papeis-e-capacidades.md` e `artefatos-e-gates.md`. A aplicação da skill limita este
artefato a um ensaio/auditoria sem alegação de independência forte, porque não houve
atestação de isolamento.

Pins SHA-256 das entradas autorizadas:

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1342.md` | `5b7322e7223b2320d961c164e0b603c10d5f390030cd671d962592af2ff92d48` |
| `00_nucleo/diagnosticos/p1342-authority-manifest.json` | `d247e6714faf3249109beb3cfb2ef718a4d2049bb56c95aaabd270eb6eed87a4` |
| `00_nucleo/diagnosticos/p1342-candidate-free-inventory-r1.md` | `06fe7dce0bf1130dca222f3148af7680f16b754ec0ac09a7956d11713042cb73` |
| `00_nucleo/diagnosticos/p1341-verifier-candidate-r1.json` | `cf63928e5bc2360b58f6f828a3e5bb5ff8c82034e7ec513e3f098186588121f2` |

O inventário e o manifesto pinam, e esta auditoria confirmou, os seis fragmentos
candidate-free centrais: `context_stabilization.rs` (`1e42852f...`), observador
P1340 (`669af694...`), `eval/mod.rs` (`6f885d48...`), `call_dispatch.rs`
(`cde7b67a...`), `stdlib/counter.rs` (`cb2a831b...`) e `introspect.rs`
(`5e019be3...`). Pins completos constam do recibo JSON.

Limite explícito: o manifesto de papéis R4 de P1341 não integra a allowlist desta
auditoria e não foi lido. Portanto, não se afirma cobertura de nomes ocultos nesse
manifesto. A correspondência 1:1 abaixo cobre os papéis expostos pelo verificador
P1341 autorizado (`callback.with`, `func.callback`, `func.body`) e os papéis reais
medidos no fragmento. Um contrato sucessor que tenha autoridade para ler R4 deve
comparar todas as suas linhas com estas regras e falhar fechado para qualquer nome
adicional.

## Medição 1 — nascimento, clones e despacho do ContextBlock

1. `01_core/src/compiler/eval/mod.rs:7561-7584` mede `body_span`, clona o
   `SyntaxNode` do corpo, captura o scope em `Arc`, cria um `Func::closure` com
   `Capturer::Context`, conserva o span diagnóstico, aloca o id e cria
   `Content::ContextBlock(Arc<ContextBlockElem { id, closure }>)`.
   `eval/mod.rs:6383-6388` é o alocador monotônico desse id na avaliação.
2. `01_core/src/entities/elements/context_block.rs:22-37` define os campos reais
   `id` e `closure`; o `Clone` manual conserva o id e clona o `Func`.
   `context_block.rs:56-68` repete essa conservação nos mapeamentos de conteúdo e
   texto. `context_block.rs:74-75` mostra que o payload introspectivo leva somente o
   id, não a closure.
3. `01_core/src/entities/func.rs:21-24` mede `Func` como `Arc<FuncRepr>` mais
   `Span`. `func.rs:52-83` mede `ClosureRepr.body` como `SyntaxNode`, e não como
   outro `Func`; o captured scope é um `Arc`. `func.rs:143-145` cria a closure.
   `func.rs:250-268` cria `FuncRepr::With(Arc<(Func, Args)>)`, copia o span do
   alvo e preserva a primeira origem diagnóstica válida.
4. `03_infra/src/pipeline/context_stabilization.rs:74-172` percorre o conteúdo em
   ordem, clona e guarda o mesmo `Arc<ContextBlockElem>` em célula/sessão.
   `context_stabilization.rs:174-240` abre um `EvalContext` de execução e despacha
   `node.elem.closure.clone()` por `apply_func`; `:258-278` substitui o conteúdo e
   `:353-371` descobre ContextBlocks nascidos do resultado real.
5. `01_core/src/compiler/eval/call_dispatch.rs:515-568` faz o despacho real.
   A closure entra em `:528-531`; `With` desempacota o par real `(Func, Args)`,
   combina argumentos e recorre para o alvo em `:541-547`.
6. `01_core/src/compiler/eval/closures.rs:52-60` reabre o scope capturado e
   `closures.rs:160-174` converte `ClosureRepr.body: SyntaxNode` em `Expr` e chama
   `eval_expr`. Esse é o início causal do corpo.

Classificação após a medição:

- `ContextBlock.id` identifica a ocorrência do elemento; `Arc<ContextBlockElem>`
  conserva a mesma entidade entre eval e L3. O `to_payload` não substitui essa
  identidade e não contém a closure.
- `FuncRepr::With` é uma entidade real e distinta, mas o seu alvo `.0` é o mesmo
  `Func` interno preservado por clone. O braço recursivo é o elo causal de despacho.
- **Refutação:** `func.body` não pode ser tipado/identificado como `Func`; o objeto
  real é `SyntaxNode`, e o evento correto é entrada/execução do corpo dessa closure.

## Medição 2 — CounterUpdate e perda do span

1. A chamada de instância chega de `call_dispatch.rs:1287-1310` ao adapter.
   `01_core/src/compiler/eval/bindings/value_methods.rs:343-362` captura
   `args.span()` em `:354`, avalia os argumentos e chama `counter_update(key,
   value)` em `:358-361` sem transportar o span.
2. `01_core/src/compiler/stdlib/counter.rs:120-149` classifica `Int`, `Array` ou
   `Func` em `CounterUpdate`; a assinatura não recebe span. A forma estática em
   `counter.rs:237-253` também usa o span apenas para diagnóstico e chama o mesmo
   helper sem o transportar.
3. `01_core/src/entities/content.rs:2278-2283` cria o
   `Arc<CounterUpdateElem>`. `01_core/src/entities/elements/counter_update.rs:20-26`
   mede somente `key` e `action`; `:51-55` clona ambos no payload. Não há campo de
   span nem identidade de ocorrência explícita.

Classificação após a medição:

- **Lacuna provada:** o span da ocorrência é conhecido no adapter AST e é perdido
  antes da entidade que atravessa Content, walk e replay. Ele não pode ser
  reconstruído depois de forma autorizada.
- O span diagnóstico dentro do callback `Func` pode sobreviver, mas representa a
  definição/primeira origem da função. **Refutação:** ele não é o span da chamada
  `counter.update` nem da ocorrência `CounterUpdateElem`.
- O carrier test-only mínimo deve nascer em `value_methods.rs:354`, acompanhar a
  própria ocorrência até `CounterUpdateElem`/walk e ser copiado pelas rotas reais.
  Endereço de objeto, pesquisa por nome final, side table global e inferência pelo
  output são transportes rejeitados pelo Passo 1342.

## Medição 3 — walk, Location, conteúdo e snapshots

1. `01_core/src/compiler/introspect.rs:710-736` normaliza/clona o conteúdo, cria
   `Locator` e `TagIntrospector` e inicia o walk. A rota de runtime em `:741-771`
   executa walk e, depois, as funções de estado/counter.
2. `01_core/src/entities/locator.rs:24-46` deriva Locations determinísticas;
   `01_core/src/entities/location.rs:14-35` mede `Location` como valor `u128`
   copiável, comparável e hashável.
3. No walk, `introspect.rs:1648-1747` aloca a Location, extrai/popula o elemento,
   guarda `IntrospectedContent::new(content.clone(), fields)` pela mesma Location e
   emite `Start`. `introspect.rs:2331-2335` emite o `End` correspondente.
   `compiler/introspect/extract_payload.rs:74-90` extrai payloads de CounterUpdate
   e ContextBlock. `introspect.rs:1142-1159` registra a ação real do CounterUpdate.
4. `01_core/src/entities/counter_registry.rs:19-25,47-58` guarda a tripla real
   Location/key/action. `:143-160` consulta o valor anterior e `:182-191` cria o
   snapshot posterior à ação.
5. Para o selector filtrado focal, `01_core/src/compiler/introspect/from_tags.rs:35-96`
   filtra e ordena ações por Location, conserva um `state` local, abre novo
   `EvalContext` em `:69-81`, chama o callback real e grava o snapshot em `:94-96`.
   A rota global, distinta, está em `from_tags.rs:146-205`.
6. `stdlib/counter.rs:325-362` seleciona a consulta filtrada; os marcadores da
   leitura contextual estão em `eval/mod.rs:5900-5955`.
   `context_stabilization.rs:443-524` executa a introspecção candidata e repete as
   leituras antes de reter o histórico; `:565-568` entrega o conteúdo estabilizado.

Classificação após a medição:

- `Location` nasce no walk; não é coordenada de fonte. Snapshot é o estado do
  counter antes/depois de uma ação; não é alias de Location. O conteúdo observado é
  a clone da entidade real indexada naquela Location.
- **Lacuna de transporte:** o resolver filtrado abre um `EvalContext` novo. Um
  ledger preso apenas ao contexto do produtor/L3 não alcança automaticamente o
  despacho/corpo do callback. A cláusula test-only deve passar explicitamente o
  carrier append-only para esse contexto filho, ou os eventos deixam de formar uma
  cadeia causal única.
- **Inferência:** observar no próprio walk a tupla carrier da ocorrência + Content +
  Location + ação é suficiente para ligá-la ao replay, desde que o mesmo carrier
  chegue ao contexto filho. Refutação: qualquer clone/mapping entre criação e walk
  que descarte o carrier, ou qualquer replay que selecione uma ação sem esse elo.

## Medição 4 — produção de Dict

`01_core/src/compiler/eval/mod.rs:7478-7523` é o produtor real de Dict: itera
campos na ordem de fonte, avalia named/keyed/spread e retorna `Value::Dict(map)`.
`01_core/src/entities/value.rs:72-77,617-620` guarda o Dict em `IndexMap`, portanto
a ordem é parte da representação real. `eval/mod.rs:5753-5766`, no observador
candidate-free herdado, projeta Dict em objeto JSON; essa projeção não prova uma
lista ordenada tipada e não deve originar identidade post-hoc.

Classificação após a medição:

- O evento de Dict deve nascer na conclusão real de `Expr::Dict`, ou ser anexado ao
  resultado real do corpo sem reavaliar/reconstruir o valor. A serialização ocorre
  depois e deve manter pares tipados e ordenados.
- **Refutação:** um objeto JSON produzido somente no observador final não demonstra
  ordem/duplicidade nem liga causalmente o Dict ao corpo executado.

## Correspondência 1:1 dos papéis sintéticos expostos

| papel P1341 exposto | entidade real | cardinalidade/veredito | prova e refutação |
|---|---|---|---|
| `callback.with` | `Func { repr: FuncRepr::With(Arc<(inner, args)>), span }` | **1:1 preservável** | nasce em `func.rs:250-256`, alcança `CounterUpdate::Func` e despacha em `call_dispatch.rs:541-547`; refuta se o evento for fabricado após o replay ou não apontar ao par real |
| `func.callback` | o `inner: Func` armazenado em `With.0` | **1:1 preservável com definição precisa** | é o mesmo alvo clonado, não uma segunda função; a aresta real é o desempacote/recursão em `call_dispatch.rs:541-547`; refuta se receber id novo sem elo ao `With.0` |
| `func.body` | `ClosureRepr.body: SyntaxNode` e o evento de sua execução | **não corresponde a Func** | `func.rs:63-64` e `closures.rs:160-174`; o papel deve ser rebatizado/retipado, nunca receber identidade `Func` |

Papéis reais adicionais que o contrato precisa distinguir:

| papel real | entidade/identidade |
|---|---|
| closure contextual | o `Func::closure` dentro do `ContextBlockElem`; distinta do callback `step` |
| ocorrência CounterUpdate | o `Arc<CounterUpdateElem>` criado por `Content::counter_update`; requer carrier test-only porque hoje não tem id/span |
| conteúdo introspectado | clone do `Content` real indexado por Location; não é uma nova ocorrência semântica |
| Location | valor criado pelo `Locator` no walk; não é span |
| snapshot | estado anterior/posterior na ordem de ações; não é Location |
| Dict | cada `Value::Dict` produzido por `Expr::Dict`; a fixture abaixo produz dois Dicts reais e o contrato deve nomear qual observa |

## Fixture real mínima do fragmento

Fixture proposta, a congelar em bytes LF antes do contrato:

```typst
#let c = counter(heading.where(level: 1))
#let step(outer, n) = {
  let witness = (outer: outer,)
  n + witness.outer.a
}
#context [
  #c.update(step.with((a: 1,)))
  #c.get()
]
```

Ela contém um ContextBlock real, uma ocorrência `CounterUpdate::Func`, um wrapper
`With`, o alvo closure `step`, execução de corpo, consulta filtrada e produção de
Dict no corpo (`witness`). O argumento pré-ligado `(a: 1,)` é um segundo Dict real;
essa multiplicidade impede uma prova baseada apenas em “vi um Dict”. A ligação deve
ser causal ao evento de body/resultado pretendido.

Sonda candidate-free equivalente em expressão, executada no estado acima:

```text
target/release/typst eval '<expressão equivalente de uma linha>' --pretty
exit 0; stdout = { "func": "context" }
```

Isto prova alcance sintático/eval até o ContextBlock; a rota completa de walk/replay
é provada estruturalmente pelas linhas anteriores e deve ser exercitada por fixture
de pipeline no ataque futuro. **Não** reutilizar offsets/comprimentos sintéticos de
P1341. Antes do selo do contrato, medir no parser os spans exatos dos bytes
congelados; o código atual demonstra onde capturá-los, não quais números inventar.

## Owners L0 e consumers que precisam de cláusula test-only

### Obrigatórios para a fixture e o carrier mínimo

| Prompt L0 owner (SHA-256 atual) | consumer 1:1 | obrigação test-only estritamente necessária |
|---|---|---|
| `prompts/infra/pipeline/context_stabilization.md` (`dfdc29af...`) | `03_infra/src/pipeline/context_stabilization.rs` | criar/reter por attempt o ledger append-only; registrar nascimento/execução/retirement/decisão; zero efeito normal |
| `prompts/compiler/eval.md` (`bc2f7de6...`) | `01_core/src/compiler/eval/mod.rs` | ligar nascimento do ContextBlock e produção real de cada Dict ao carrier; disponibilizar o ledger test-only no EvalContext |
| `prompts/compiler/eval/bindings/value_methods.md` (`a66a71d0...`) | `01_core/src/compiler/eval/bindings/value_methods.rs` | capturar o span real em `args.span()` antes da perda e anexá-lo à ocorrência criada |
| `prompts/compiler/stdlib/counter.md` (`2ba12786...`) | `01_core/src/compiler/stdlib/counter.rs` | conservar carrier/span ao classificar a ação e construir o Content, sem alterar semântica normal |
| `prompts/entities/elements/counter_update.md` (`cf16a016...`) | `01_core/src/entities/elements/counter_update.rs` | definir o carrier de ocorrência test-only e sua preservação por Clone/mapping; não expor API produtiva |
| `prompts/compiler/introspect.md` (`e2884968...`) | `01_core/src/compiler/introspect.rs` | no walk, anexar ocorrência/Content/Location/ação e snapshots à mesma cadeia |
| `prompts/compiler/introspect/from_tags.md` (`739b43d5...`) | `01_core/src/compiler/introspect/from_tags.rs` | propagar o ledger ao EvalContext filho e registrar pre/post snapshot, dispatch e resultado real |
| `prompts/compiler/eval/call_dispatch.md` (`0843ec72...`) | `01_core/src/compiler/eval/call_dispatch.rs` | registrar despacho do With real, aresta ao inner real e despacho recursivo, sem fabricar Func |
| `prompts/compiler/eval/closures.md` (`e278e1b6...`) | `01_core/src/compiler/eval/closures.rs` | registrar entrada no `SyntaxNode` real do corpo e saída/erro causalmente ligados |

As cláusulas P1341 hoje presentes em pipeline/eval/call_dispatch/counter/introspect
não bastam como autoridade P1342: foram escritas para papéis/transportes rejeitados,
e `closures.md` e `value_methods.md` não cobrem os dois pontos causais decisivos.
Devem ser substituídas/refinadas pelo dono; não copiadas como evidência.

### Condicionais — somente se a implementação escolher mudar essas entidades

- `prompts/entities/func.md` → `entities/func.rs`: necessário apenas se for criado
  id/campo test-only novo em `Func`. A topologia existente de `Arc<FuncRepr>` já
  permite conservar os Funcs reais; a auditoria prefere não ampliar a entidade.
- `prompts/entities/elements/context_block.md` → `context_block.rs`: necessário
  somente se o carrier for posto no elemento; id + Arc existentes bastam para a
  fixture.
- `prompts/entities/counter_registry.md` → `counter_registry.rs`: necessário apenas
  se Location/key/action/snapshot ganharem campo; observar os eventos existentes no
  owner introspectivo evita essa ampliação.
- `prompts/entities/value.md` → `value.rs`: necessário apenas se `Value::Dict` mudar;
  observar o `IndexMap` existente no produtor dispensa isso.
- `prompts/entities/element_payload.md` → `element_payload.rs`: não é necessário se
  o walk usa o `Content` real que já possui. Não duplicar o carrier no payload sem
  prova de necessidade.

## Forma mínima do contrato sucessor

Sem prescrever código, o contrato precisa exigir uma cadeia tipada e append-only:

```text
ContextBlock(id, closure-real)
  -> CounterUpdate(occurrence-carrier, source-span, Func::With real)
  -> walk(Content real, Location, key/action)
  -> snapshot-pre
  -> dispatch With -> inner Closure
  -> body-enter(SyntaxNode real)
  -> Dict-produced(Value::Dict ordenado real)
  -> body-exit/result
  -> snapshot-post
  -> attempt/decision
```

Cada evento deve declarar producer, consumer, dado/id observado, transporte por
clones/mappings reais, ponto de append, ausência de efeito no build normal e escopo
da fixture. Falhar fechado se faltar carrier/span/aresta. `Unknown` só é admissível
para payload opaco depois de a estrutura acima estar provada — nunca para identidade,
span, Location, snapshot ou cardinalidade.

## Conclusão auditável

A topologia real existe e a fixture mínima a percorre, mas dois fatos impedem
qualquer selo imediato: (1) o span da ocorrência é descartado em
`value_methods.rs:358-361`; (2) `func.body` é um tipo sintético incorreto diante de
`ClosureRepr.body: SyntaxNode`. As nove cláusulas L0 obrigatórias acima são o menor
conjunto medido para uma observação test-only causal de ponta a ponta no caminho
filtrado. Transportes post-hoc/globais ou reconstruções por output/nome/endereço não
refutam essas lacunas e permanecem proibidos.
