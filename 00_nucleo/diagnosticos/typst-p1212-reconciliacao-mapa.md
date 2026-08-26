# P1212 — reconciliação de paridade assistida por mapa

**Resultado:** `MAP PILOT CLOSED — RESIDUAL CLASSIFIED`  
**Data:** 2026-08-26  
**Regime:** protocolo completo executado numa única sessão, **sem atestação de
isolamento**.  
**Vanilla ratificado:** `a51e02804`.  
**Cristalino:** `dc47c9c32b8b6769a58622c98b885cb094337508` + working tree
documental não commitida.  
**Lente:** `98d8f9e7a3882beb3fd0a294a95df9332058f76c` + working tree da
materialização P0079; binário SHA-256
`9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`.

## 1. Conclusão executiva

O mapa tornou comparáveis 24 módulos e, pela primeira vez neste baseline, 18
arestas entre módulos vanilla e cristalinos. As cinco regras de prefixo foram
aceitas apenas como correspondência mecânica; nenhuma recebeu alegação funcional
coletiva.

A correspondência piloto `typst_eval::ops::apply_binary` foi corrigida de `1:3`
para `1:5`: além dos três domínios puros, a responsabilidade exige o dispatcher
`eval_binary_op` e o braço de avaliação/short-circuit em `eval_expr`. Vinte e uma
sondas diferenciais preservaram todos os valores e classes de erro; 19 também
preservaram o núcleo textual, mas duas mensagens divergem. Como a matriz completa
dos L0s não foi exaurida, a alegação final é `PARTIAL`, não
`DECLARED-CLOSED`.

O passo não corrige os gaps P1210 e não declara percentual de paridade. O resíduo
continua misturando renomes, mecânica interna, produtos fora do alvo e ausência
real; agora está agregado e possui dez candidatos nominais para os próximos
lotes.

## 2. Proveniência

Congelamento em `2026-08-26T14:04:01-03:00`:

- cristalino HEAD `dc47c9c32b8b6769a58622c98b885cb094337508`;
- working tree inicial do P1212: mapa e passo P1212 não versionados;
- mapa inicial SHA-256
  `b121b356c4234ea890dd3a9a9c04834b5ec933d977462d476ef8fceef84d5d68`;
- mapa adjudicado SHA-256
  `8b45aaf1e6b84c706ffb02d4399c0aa546ffb030a029995b2bceff9cc0822877`;
- probes SHA-256
  `8832da254c189e8bf301197953603326aec6f2c37a4209d971b81915e9ee8d74`;
- JSON sem mapa SHA-256
  `0881042f0e98038a3d19b946302291a2a35712cd0d5fab4ffa8c3ed6bda7688c`;
- JSON com mapa adjudicado SHA-256
  `2000580683d0a7a050f24a167e590c630d1ae001bc216aca993e5e63154f91a9`.

As repetições A/B produziram os mesmos hashes byte a byte.

## 3. Baseline A/B

| Medida | sem mapa | mapa adjudicado | delta |
|---|---:|---:|---:|
| módulos pareados | 0 | 24 | +24 |
| módulos sem par — vanilla | 386 | 362 | −24 |
| módulos sem par — cristalino | 438 | 414 | −24 |
| arestas comuns | 0 | 18 | +18 |
| arestas só vanilla | 0 | 33 | +33 observáveis |
| arestas só cristalino | 0 | 14 | +14 observáveis |
| itens K4 pareados | 1.906 | 1.906 | 0 |
| ambiguidades | 148 | 147 | −1 |
| itens sem par — vanilla | 11.266 | 11.266 | 0 |
| itens sem par — cristalino | 3.711 | 3.709 | −2 |

As arestas “só” não são regressões automáticas: tornaram-se visíveis porque as
duas pontas passaram a ter correspondência. O humano ainda precisa julgar a
responsabilidade.

## 4. Auditoria dos prefixos

As cinco regras produziram exatamente 24 pares:

| Regra | pares aplicados | estado funcional |
|---|---:|---|
| `typst_eval` → `typst_core::compiler::eval` | 5 | `UNKNOWN` |
| `typst_syntax::ast` → `typst_core::entities::ast` | 1 | `UNKNOWN` |
| `typst_layout` → `typst_core::compiler::layout` | 6 | `UNKNOWN` |
| `typst_layout::math` → `typst_core::compiler::math::layout` | 3 | `UNKNOWN` |
| `typst_library::foundations` → `typst_core::compiler::stdlib::foundations` | 9 | `UNKNOWN` |

O aceite é mecânico: igualdade literal do sufixo e paths existentes. Uma regra
de prefixo não diz que todo o módulo fecha paridade; para isso, responsabilidades
específicas precisam de `[[correspondencia]]`, sonda e veredito.

## 5. Piloto `eval-apply-binary`

### 5.1 Medição antes da decisão

Vanilla `typst-eval/src/ops.rs:59-78`:

- avalia lhs;
- faz short-circuit de `and`/`or`;
- avalia rhs;
- chama a operação semântica.

Cristalino:

- `compiler/eval/mod.rs:884+` — `eval_expr`, incluindo avaliação e
  short-circuit em `1007-1028`;
- `compiler/eval/operators/mod.rs:38+` — `eval_binary_op`, dispatcher puro;
- `arithmetic::apply_binary`;
- `equality::apply_binary`;
- `ordering::apply_binary`.

Portanto, a relação `1:3` inicial omitia duas responsabilidades observáveis e
foi refutada antes do veredito.

### 5.2 Sondas

Foram executados 21 casos:

- aritmética: 5;
- igualdade/pertença: 5;
- ordenação: 5;
- short-circuit: 2;
- erros: 4.

Resultado:

- 17 valores idênticos;
- 2 erros com núcleo textual idêntico (`divide by zero`, `bad_add`);
- 2 erros com mesma classe e mensagens diferentes:
  - vanilla `cannot apply 'in' to integer and string` versus cristalino
    `cannot apply In to integer and string`;
  - vanilla `cannot compare 1pt with 1em` versus cristalino
    `cannot compare length and length`.

O primeiro ensaio foi descartado integralmente porque usou `--color never`
somente no cristalino; o vanilla rejeitou a flag. A segunda rodada removeu a
assimetria.

### 5.3 Veredito

```text
mechanical_state = APPLIED
functional_state = PARTIAL
relation = divisao (1:5)
```

Refutadores ainda abertos:

- braço L0 não coberto pela matriz focal;
- diferença de mensagem onde mensagem é observável;
- responsabilidade vanilla adicional fora dos cinco destinos;
- coerções/scope-outs nominais ainda não sondados.

## 6. Ataques ao mapa

| Ataque | Esperado | Obtido |
|---|---|---|
| destino inexistente marcado fechado | `unknown`, não consumir | `unknown` + diagnóstico do path |
| mesmo path vanilla em duas relações | ambas `unknown` | ambas `unknown` + dois diagnósticos |
| reordenar tabelas e prefixos | mesmo resultado semântico | igualdade integral após remover caminho/hash |
| relação inicial `1:3` | adversário encontra responsabilidade omitida | refutada; atualizada para `1:5` |
| promover após apenas outputs favoráveis | promoção bloqueada por mensagens/matriz incompleta | permaneceu `PARTIAL` |

Os ataques cobrem os mutantes declarados neste fragmento, mas não constituem
campanha externa de mutation testing. Não há alegação de mutation score
atestado.

## 7. Resíduo

O maior resíduo vanilla permanece em:

1. `typst_library`: 7.938;
2. `typst_layout`: 770;
3. `typst_pdf`: 507;
4. `typst_html`: 496;
5. `typst_syntax`: 408.

No cristalino: `typst_core` 2.860, `typst_infra` 680 e `typst_shell` 169.
Funções dominam ambos os lados. Essas contagens não são percentual de features:
por exemplo, structs `*Elem`, variantes `Content` e helpers internos possuem
morfologias mecânicas deliberadamente diferentes.

Os dez candidatos seguintes estão congelados em `p1212-candidatos.tsv`.
Prioridade 1–5 vem de gaps P1210; 6–10 são ambiguidades com alto risco de falso
pareamento.

## 8. Separação de papéis executada

Os artefatos foram produzidos em ordem causal:

1. censo A/B e lista de paths;
2. leitura L0/vanilla e contrato das sondas;
3. execução dos dois binários;
4. ataques;
5. adjudicação e atualização do mapa.

Contudo, uma única sessão teve acesso a todas as fases. Classificação correta:
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. O laudo não usa “independente”.

## 9. Alterações e limites

- nenhum `.rs` do `typst-crystalline` foi alterado;
- nenhum Prompt L0 foi alterado;
- nenhuma ausência funcional foi corrigida;
- o mapa e diagnósticos são as únicas mudanças do P1212;
- o working tree da lente contém a materialização P0079 anterior e não pertence
  ao diff funcional do Typst;
- falhas de extração da lente permanecem declaradas; não viraram ausência.

## 10. Estado terminal

Gates finais no working tree não commitado:

- `crystalline-lint .`: exit 0, com avisos informativos/preexistentes;
- `git diff --check`: exit 0;
- validação de cardinalidade das quatro tabelas TSV: exit 0;
- `cargo test --workspace --quiet`: 5.250 + 858 + 55 + 2 testes
  passaram antes de uma falha por timeout em
  `p1137_watch_dependencias_recuperacao_e_filtro` (69/70 no binário);
- repetição isolada do teste que expirou: 1 passou, 0 falhou, em 1,91 s.

Portanto, não se mascara o exit 101 da primeira suíte global: a evidência aponta
para flutuação temporal do teste de watch, e não para regressão dos artefatos
documentais deste passo.

```text
MAP PILOT CLOSED — RESIDUAL CLASSIFIED
```

O próximo passo deve abrir um único cluster da fila congelada. HTML é o maior
gap de capacidade já confirmado, mas SVG/PDF podem oferecer sondas menores; a
escolha deve ser explícita e não alterar este veredito retroativamente.
