# P1142 — superfície estática de `array.all` e `str.clusters`

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — correção de paridade em fluxo contínuo pelo ADR-0127`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Owner:** `01_core/src/compiler/stdlib/collections.rs`  
**Dependência:** P1141 concluído; preservar integralmente o lote `path` ainda não commitado.

## 1. Objetivo

Auditar e materializar as formas estáticas `array.all(array, pred)` e
`str.clusters(str)` que o vanilla expõe na superfície dos valores-tipo, sem
duplicar os algoritmos já usados por `(array).all(pred)` e
`(str).clusters()`.

As duas lacunas formam uma unidade porque pertencem ao mesmo owner e porque a
mudança pretendida é de superfície, não de algoritmo. O passo não autoriza
abrir outros membros de `array`, `str` ou `dict` encontrados durante o
inventário.

## 2. Proveniência inicial

- HEAD cristalino ao redigir este passo:
  `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, branch `Tekt`.
- Estado em `2026-08-24T18:48:21-03:00`: working tree não commitado, contendo
  o lote P1141 em 36 ficheiros rastreados (496 inserções, 86 remoções), quatro
  novos ficheiros do P1141 e o handoff untracked do dono. Esta fotografia é a
  base que P1142 deve preservar; qualquer número posterior deve repetir HEAD,
  hora e `git diff HEAD --stat`.
- Fonte vanilla inicialmente localizada em
  `lab/typst-original/crates/typst-library/src/foundations/array.rs:698-714`
  e `lab/typst-original/crates/typst-library/src/foundations/str.rs:275-280`.
  Antes de decidir, conferir essas regiões contra o objeto Git
  `a51e02804`; a cópia de trabalho do lab, isoladamente, não prova o baseline.
- L0 vigente:
  `00_nucleo/prompts/compiler/stdlib/collections.md`. Ele já especifica os
  métodos de instância, mas não legitima ainda a superfície estática.

## 3. Fase A — medir o vanilla antes de decidir

Executar sondas reproduzíveis nos dois binários vanilla ratificados e registar
fonte, comando, saída, HEAD, estado da árvore e hora. No mínimo medir:

1. existência e tipo de `array.all` e `str.clusters`;
2. equivalência observável entre forma estática e forma de instância;
3. assinatura posicional, aridade insuficiente/excedente e argumentos nomeados;
4. `array.all` em array vazio, curto-circuito, predicado falso e retorno do
   predicado que não seja booleano;
5. preservação da ordem e número de chamadas do predicado apenas quando isso
   for observável na língua — não transformar passos internos em contrato;
6. `str.clusters` para ASCII, string vazia, acento combinante, emoji com
   modificador, ZWJ e flag regional;
7. `repr`/tipo dos resultados e mensagens de erro quando forem observáveis.

Sondas mínimas sugeridas:

```typ
#metadata((
  array-all-type: type(array.all),
  array-all-static: array.all((2, 4), x => calc.even(x)),
  array-all-method: (2, 4).all(x => calc.even(x)),
  array-all-empty: array.all((), x => false),
  clusters-type: type(str.clusters),
  clusters-static: str.clusters("á👩‍💻🇵🇹"),
  clusters-method: "á👩‍💻🇵🇹".clusters(),
)) <probe>
```

As sondas negativas devem ser separadas para que um erro não esconda outro.
Não presumir que a implementação cristalina atual de `str.clusters` por
`char` satisfaz grapheme clusters: isso deve ser decidido somente depois da
medição.

## 4. Fase B — auditar o estado cristalino

Confirmar e documentar, com `file:line` atualizados:

- `collections.rs` já contém consumers reais `array_all` e `str_clusters` no
  dispatcher de métodos de instância;
- `field_access.rs` expõe em `Value::Type` apenas membros estáticos parciais e
  ainda não encaminha `Type::Array`/`Type::Str` para estes dois membros;
- o L0 de collections descreve `str.clusters` como clusters simplificados por
  `char`, enquanto o vanilla ratificado usa grapheme clusters;
- se existe dependência L1 permitida ou utilitário já nuclearizado para
  segmentação Unicode; não adicionar crate externa antes de verificar a
  whitelist e os L0s afetados;
- todos os testes existentes das formas de instância que podem ser
  reaproveitados como prova de não regressão.

Classificar cada divergência segundo ADR-0107/0108. A presença dos membros,
assinatura, resultado, curto-circuito observável e morfologia dos clusters são
língua. A estrutura de dispatcher, função Rust compartilhada e algoritmo são
mecânica livre.

## 5. Decisão e L0 antes do código

Só depois das fases A e B, atualizar primeiro
`00_nucleo/prompts/compiler/stdlib/collections.md` para declarar:

1. as assinaturas estáticas ratificadas e sua relação com as formas de
   instância;
2. um único owner sem duplicação semântica;
3. a morfologia exata de `str.clusters` medida no vanilla;
4. erros e critérios de aceitação no nível da língua;
5. qualquer scope-out descoberto, nomeando o passo que o completará.

Se a morfologia correta exigir nova dependência externa, alteração de tipo
público, mudança de assinatura pública, comportamento por defeito ou fase do
pipeline, atualizar também os L0s realmente afetados e **parar no gate
ADR-0127**. Em dúvida sobre a classe, parar.

Se a auditoria concluir que se trata apenas de paridade na tabela/dispatch e
correção interna da segmentação, seguir em fluxo contínuo: L0 primeiro,
resselo, RED→GREEN e revalidação, conforme ADR-0127. O passo deve registrar
explicitamente a classificação que autorizou seguir.

## 6. Plano RED→GREEN

Após L0 válido e eventual gate:

1. escrever testes RED para acesso e chamada de `array.all` e `str.clusters`
   pelo valor-tipo;
2. escrever RED morfológico para todos os clusters Unicode medidos;
3. provar que os testes de instância existentes continuam válidos;
4. extrair/adaptar funções internas apenas na medida necessária para que ambas
   as superfícies reutilizem o mesmo owner;
5. ligar `Type::Array` e `Type::Str` no field access sem criar hub genérico;
6. ressellar todos os headers afetados;
7. executar testes focados, regressões de collections,
   `cargo check --workspace`, `cargo build --workspace`,
   `crystalline-lint .` e `git diff --check`.

O GREEN deve provar, no mínimo:

```typ
array.all((2, 4), x => calc.even(x)) == (2, 4).all(x => calc.even(x))
str.clusters("á👩‍💻🇵🇹") == "á👩‍💻🇵🇹".clusters()
```

Além da igualdade entre superfícies, o conteúdo esperado deve ser comparado
com o vanilla; duas formas igualmente erradas não constituem paridade.

## 7. Limites e critério de encerramento

Ficam fora de P1142:

- outros membros estáticos de `array`, `str` ou `dict`;
- namespace de cores, `gradient.kind`, accessors de `datetime`, bitwise de
  `int`, `counter.get`, `content.fields`, numbering, emoji e HTML;
- remoção de extensões cristalinas ou limpeza de warnings históricos;
- alterações ao lote P1141 sem regressão diretamente demonstrada.

P1142 encerra somente quando as duas formas estáticas e as formas de instância
forem semanticamente equivalentes ao vanilla ratificado, a morfologia Unicode
estiver comprovada, os L0s e hashes estiverem atuais e a validação final tiver
zero violations. Se surgir mudança enquadrada pelo ADR-0127, o estado correto
é `AGUARDA GATE`, não implementação parcial.

## 8. Execução

O passo foi executado em 2026-08-24. A fonte do objeto Git `a51e02804` e os
dois binários vanilla ratificados concordaram:

- `array.all`/`str.clusters` são funções não ligadas com `repr` respectivamente
  `all`/`clusters`, além das formas de instância;
- `array.all((), pred)` devolve `true`, faz curto-circuito no primeiro `false`
  e rejeita retorno não booleano com `expected boolean, found integer`;
- `str.clusters("á👍🏽👩‍💻🇵🇹")` devolve exatamente
  `("á", "👍🏽", "👩‍💻", "🇵🇹")` nas duas superfícies;
- `self` e `test` são posicionais; missing, extra, named e tipo incorreto foram
  medidos em sondas negativas separadas.

A classificação produzida pela medição foi correção de paridade em owner e
assinaturas Rust já existentes. `unicode-segmentation` já era dependência L1 e
estava autorizada pela ADR-0013. Não houve campo de entidade, método de trait,
assinatura pública, default, compatibilidade ou mudança de fase; portanto o
ADR-0127 determinou fluxo contínuo, sem nova paragem.

RED confirmado por `cargo test -p typst-core p1142_ --lib`: 3 testes falharam
pela ausência de `array.all`/`str.clusters` no valor-tipo; o teste de clusters
também cobria a morfologia que a implementação por `chars()` não preservava.

GREEN implementado no owner `collections.rs`:

- field access de `Type::Array` e `Type::Str` delega a uma tabela estreita do
  owner;
- a forma estática de `array.all` usa `NativeWithEngine` e o mesmo helper da
  forma de instância;
- ambas as formas exigem `Value::Bool` e curto-circuitam;
- ambas as formas de `str.clusters` usam extended grapheme clusters;
- nenhum outro membro estático foi aberto.

Validações focadas:

- `cargo test -p typst-core p1142_ --lib`: 4/4 passam;
- `cargo test -p typst-core p466_array_all --lib`: 1/1 passa;
- `cargo check --workspace`: passa;
- `crystalline-lint --fix-hashes .`: `Nothing to fix` — o owner mantém o
  formato histórico de linhagem sem `@prompt-hash`.

As sondas temporárias em `temp/p1142/` foram removidas após transcrever as
saídas. Fotografia antes da validação final em
`2026-08-24T19:00:28-03:00`: HEAD
`a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, working tree não commitado; o
diff acumulado P1141+P1142 contém 39 ficheiros rastreados, 751 inserções e 100
remoções, além dos untracked já enumerados pelo status. P1142 alterou cinco
ficheiros rastreados e este passo untracked; preservou o handoff do dono e o
lote P1141.

Validação final às `2026-08-24T19:01:39-03:00`, no mesmo HEAD e working tree:
`cargo fmt --all --check && cargo build --workspace && crystalline-lint . &&
git diff --check` terminou com exit 0. O linter reportou zero violations;
warnings e infos históricos permaneceram fora deste lote.
