# P1339 — revisão independente das minutas de L0, R1

Executor: `/root/p1339_where_l0_review`, somente leitura de L0/código e
escrita deste adendo. Não altera o parecer anterior nem os artefatos
avaliados. Revisão de suficiência para apresentar a extensão pública ao
dono; não é PASS de implementação, contrato selado ou autorização ADR-0127.
Regime: revisão executada sem atestação de isolamento técnico.

## Entradas e proveniência

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado
com somente as seguintes alterações rastreadas na captura focal:

```text
compiler/eval/bindings/value_methods.md       | 84 +
compiler/eval/operators/equality.md           | 43 +
compiler/eval/selector_matching.md            | 55 +
entities/selector.md                         | 80 +
entities/show.md                             | 46 +
5 files changed, 308 insertions(+)
```

Todos os paths acima são relativos a `00_nucleo/prompts/`.
Artefatos de investigação P1339 não rastreados coexistem e não são código
candidato. Hashes SHA-256 calculados antes da leitura das minutas:

| Minuta | Hash |
|---|---|
| `entities/selector.md` | `46f1263e0dd9ff6dd5c5906f951b82ee43de1a071f38acc0d1281cfa5abd7f57` |
| `entities/show.md` | `3752c4a2bfcd9b4fed436108aa83c87dc202aabe7d036c0abda479b0debf2e9a` |
| `compiler/eval/bindings/value_methods.md` | `51132b56930eef6f3aa988f8485bdeb89d78122e0bf5ffdd58ccde0a2b7480b7` |
| `compiler/eval/selector_matching.md` | `22c71be24fa0e47896b0ad4dc9ca03357a24789085a753dfd8d4efe7cf416c24` |
| `compiler/eval/operators/equality.md` | `b304c33c9ff064da2a2bd24ffee9946a2980729c8a353275bfb784e8ca3b5730` |

Fontes antecedentes adicionais: `operators/equality.rs` SHA-256
`be3460acb40e12dccf904e921d478e287794ad1348c6af261d697d7728b1ed77`,
`selector_matching.rs` SHA-256
`09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9`.

## Achados concretos submetidos ao autor

### R1-A — fronteira da igualdade no matcher ambígua

Medição: `selector_matching.rs:159–184` usa `values_eq_semantic`, que
coage apenas Int↔Float direto e delega os demais casos a Value::PartialEq.
`operators/equality.rs:101–133` tem semântica diferente: compara coleções
recursivamente e Content por morph_canon. Portanto trocar indiscriminadamente
o helper usado em todo Where pode mudar comportamento do Where antecedente
mesmo quando não contém a nova variante Element.

A minuta `selector_matching.md` P1339 ordena tanto usar a igualdade de
linguagem comum quanto preservar comportamentos das variantes antigas.
São instruções insuficientemente delimitadas quando aplicadas ao braço Where
existente. A minuta de equality preserva explicitamente pares de variantes
anteriores sem Element, reforçando a necessidade dessa fronteira.

Correção recomendada: declarar que a nova comparação se aplica aos filtros
cuja base nativa vem de QuerySelector::Element/NativeElement, preservando
o caminho legado sem essa base; alternativamente classificar e especificar
explicitamente uma ampliação deliberada de semântica, com seus testes.
Este achado deve ser corrigido antes de usar a minuta para implementação;
não refuta a suficiência representacional dos carriers públicos.

### R1-B — exceção ao limite AST do owner não está explícita

`value_methods.md`, cláusula P1284, restringe o owner à orquestração que
realmente precisa de AST. A cláusula P1339 escolhe um helper semântico puro
sobre receiver Func e Args já avaliados. A escolha não viola a topologia
L1 nem exige nova entidade, mas deve declarar que P1339 excepciona o limite
anterior somente para a semântica comum de where. Isso impede o implementador
de escolher unilateralmente qual instrução prevalece e evita ampliar o owner
para outros métodos por analogia.

## Parecer delimitado

Os dados propostos são suficientes para uma pergunta concreta de aprovação:
QuerySelector::Element com Func e grupo EcoVec ordenado, e
ShowSelector::NativeElement(Func) como base dos filtros Where de show.
As minutas distinguem filtro vazio, ordem, aliases, reconhecimento de
função nativa versus Func::eq, igualdade pública versus derives, carrier de
show versus regex e restrições dos owners downstream. A existência de
Content e snapshots em introspecção foi reconhecida; não se usa o stub
histórico como prova de impossibilidade geral.

As pendências explícitas de repr/query/introspector/counter e dos dispatches
não impedem pedir aprovação da fronteira pública aditiva, mas impedem
materializar consumers sem os seus L0 próprios. O conjunto concreto de
builtins/campos e a aceitação completa ainda exigem contrato posterior;
esta revisão não transforma os exemplos de sonda em inventário completo.
Após resolver R1-A e tornar R1-B explícito, não há objeção adicional encontrada
à apresentação dessa fronteira pública ao dono. Aprovação humana, contrato
discriminatório, selo, código e gates finais continuam pendentes.
