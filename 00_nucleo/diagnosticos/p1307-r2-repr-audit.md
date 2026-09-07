# P1307-R2 — auditoria do formatter canônico

## Medição e fontes

O registro `p1307-r2-gates.json`, entrada `repr_audit_measurement`, contém 17
expressões × quatro perfis × dois binários, 136 invocações entre
`2026-09-07T16:38:58.034515+00:00` e `16:39:04.786635+00:00`. Não é teste
de candidato. HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais o
working tree P1306 não commitado; status, diff integral, diff/stat, sources e
binários foram reconferidos em `p1307-r2-baseline.json`, SHA-256
`0c0e92fa4c84bb23a1e2adefc95e93ad75ffaf0f12f3edce58ab8e9393b2f611`.
O snapshot identifica exatamente os quatro arquivos previamente alterados.
Cada invocação guarda argv, timestamps, env e stdout/stderr integrais.

O L0 `compiler/eval/repr.md` foi lido integralmente, SHA-256
`c5169f71ee6a0ece8cc2fc56ca9cecd07443a339c460b6ae63f15638c436eccd`;
o consumer tem SHA-256
`0dde543205c04eeedc0896961d505ba315f1573d55fd17498e8b1b5544f9af8c`
e header `d9ff5309`. Os L0 de `entities/state` e `entities/counter` foram
também lidos integralmente: conservam os dados necessários, não pedem mudança
de entidade para corrigir a representação.

Na fonte ratificada `a51e02804`, sob
`lab/typst-original/crates/typst-library/src/introspection/`:

- `state.rs:370-374`: a repr usa a repr da chave e do valor inicial;
- `counter.rs:519-523`: a repr envolve a representação da chave;
- `counter.rs:557-565`: Page é `page`, Selector usa sua repr e Str usa a repr
  de string. São formas de linguagem, não o Debug nem a estrutura Rust.

No cristalino, `entities/state.rs:16-19` já guarda `key` e `init`;
`entities/counter.rs:22-26,53-55` já distingue Page/Selector/Str e guarda a
chave. `compiler/eval/repr.rs:122-123` ignora esses dados. A fachada de
serialização em `compiler/eval/mod.rs:83-85` somente delega a esse formatter.

## Resultado observado

`state("probe", 0)`, estado com string escapada, array longo e estado
aninhado produzem no vanilla os dados completos, mas no cristalino sempre
`state(...)`. O estado com array longo mantém `state("long", (` na primeira
linha e a quebra interna da repr do array: não se deve aplicar arbitrariamente
o helper de layout de campos ao constructor State inteiro.

`counter(page)`, `counter("page")`, `counter(heading)`, `counter("heading")`,
chave escapada e `counter(heading.where(level: 2))` distinguem as categorias
no vanilla, mas no cristalino retornam `counter(...)`. As chaves já são
distintas no carrier: o defeito aqui é de projeção, não colisão de identidade.

Os seguintes limites foram medidos para não atribuir toda diferença ao
formatter:

- `state("probe")`: vanilla constrói com init `none`, cristalino rejeita a
  aridade. É dívida anterior do constructor, não dado perdido no formatter.
- `counter(<a>)`: vanilla aceita, cristalino rejeita Label. Também é entrada
  de constructor, não motivo para mudar `Counter` nesta correção.
- `counter(heading.or(figure))`: vanilla rejeita a chamada `.or` sobre
  function; cristalino aceita. Não é caso positivo comum.
- `selector(heading).or(figure)`: vanilla publica
  `selector.or(figure, heading)` e cristalino `(heading | figure)`.
  `repr_selector` em `repr.rs:1001` é do mesmo owner, mas essa divergência
  precisa de contrato próprio se fizer parte dos payloads exigidos. Não se
  corrige o wrapper Counter para esconder a repr divergente do seu filho.

As 12 saídas não zero do recorte são as três fronteiras de construção acima
em quatro perfis. Elas não são Unknown de execução nem RED de implementação.

### Complemento da medição independente R2

`p1307-r2-measurement.json`, matriz encerrada em
`2026-09-07T16:44:45.243137+00:00`, usa o mesmo baseline pinado. Sua rota
contextual pública confirmou `location(..)` no vanilla e `location(...)`
no cristalino (`construct-Location`), sem depender de `eval --in`.

Também refutou uma frase do contrato documental P1307 anterior: os casos
`with-json-encoder-with-identity`, `with-toml-encoder-with-identity` e
`with-yaml-encoder-with-identity` medem type `function` e repr `(..) => ..`
para a função parcialmente aplicada. O membro original continua `encode`;
o resultado de sua aplicação parcial não tem a mesma repr. O artefato
anterior foi mantido intacto como histórico, não como expectativa vigente
para essa rota.

Os casos `*-encoder-with-identity` do recibo original P1307 já continham
`["function", "(..) => .."]`. Portanto foi erro de transcrição/interpretação
no contrato anterior, não mudança de comportamento entre as duas medições.

Fonte ratificada: `foundations/func.rs:460-470`, braço `FuncInner::With`,
retorna a forma anônima; `Func::name()` é outro observável e pode continuar
delegando ao nome interno. O cristalino em `repr.rs:60-78,1033-1040`
percorre With para decidir se o interno é closure, depois usa o nome da
função nativa. Não é necessário mudar `Func::name()` nem o namespace para
corrigir esse formatter; isso regrediria observáveis distintos.

## Desenho mínimo demonstrado, ainda não materializado

Para State e Counter já construídos, **não é necessário alterar entidades**:

1. No owner `compiler/eval/repr.md` → `compiler/eval/repr.rs`, State deve
   projetar a chave pelo formatter canônico de string e o init pela repr
   recursiva, preservando escaping e multiline do filho.
2. Counter deve despachar exaustivamente pela chave existente: Page literal,
   Selector pelo formatter do selector, Str pelo formatter de string.
   `"page"` nunca pode ser confundido com Page, nem `"heading"` com Heading.
3. Location deve ter sua forma opaca corrigida neste owner, agora sustentada
   pela medição contextual bilateral, sem expor identidade interna nem
   inspecionar documentos.
4. A repr de aplicação parcial deve distinguir `FuncRepr::With` do membro
   original e produzir a forma medida, sem alterar `Func::name()`, aplicação
   dos argumentos ou herança do namespace. Medir controles de outras funções
   parciais antes de ampliar o teste além dos encoders.

É inferência de viabilidade, apoiada em dados já presentes e nos formatters
existentes; não é prova de implementação GREEN. Refutam-na um payload exigido
sem dados no carrier, um filho cuja repr canônica não satisfaça o contrato,
ou necessidade de reconhecer fixtures/origem. Casos de Selector compostos
mostram por que a correção de dois wrappers não prova paridade geral.

Essa correção isolada é paridade interna em fluxo contínuo ADR-0127, não
mudança pública. O P1307 completo continua sujeito ao gate dos encoders e
ao redesenho independente de argumentos. Este diagnóstico não legitima
código: antes de implementar, o L0 deste owner deve receber a decisão medida
e o novo hash, com RED/GREEN e verificação separados.

Nenhum L0, Rust, header ou artefato predecessor foi alterado nesta auditoria.
