# P1143 — namespace completo das cores predefinidas

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — correção tabelada de paridade em fluxo contínuo pelo ADR-0127`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Owner:** `01_core/src/compiler/stdlib/color.rs`  
**Dependências:** P1141 e P1142 concluídos; preservar os dois lotes ainda não commitados.

## 1. Objetivo

Auditar e completar o namespace público `color` com todas as cores
predefinidas que o vanilla ratificado expõe simultaneamente como bindings
globais e fields do valor-tipo, usando uma única tabela canônica no owner.

`color.black` é apenas a sentinela que revelou a lacuna. P1143 não deve
adicionar somente `black`, nem copiar os valores RGB para um segundo `match`.
O conjunto inteiro medido forma uma unidade tabelada.

## 2. Proveniência inicial

- HEAD cristalino em `2026-08-24T19:07:12-03:00`:
  `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, branch `Tekt`.
- Working tree não commitado: diff acumulado P1141+P1142 com 39 ficheiros
  rastreados, 751 inserções e 100 remoções, além dos novos ficheiros desses
  lotes e do handoff untracked do dono. P1143 deve preservar esta fotografia.
- L0 vigente:
  `00_nucleo/prompts/compiler/stdlib/color.md`.
- Código vigente: `predefined_color_bindings()` já materializa uma lista
  global; `color_type_field()` contém constructors e operadores, mas nenhuma
  cor predefinida.
- Fonte vanilla candidata:
  `lab/typst-original/crates/typst-library/src/lib.rs` para registro no scope e
  `lab/typst-original/crates/typst-library/src/visualize/color.rs` para valores.
  Conferir ambas no objeto Git `a51e02804`, não apenas na cópia do lab.

Todo número usado para fechar o passo deve registrar novamente HEAD, hora e
`git diff HEAD --stat`.

## 3. Fase A — medir o vanilla antes de decidir

Construir o inventário do namespace pela fonte ratificada e confirmá-lo nos
dois binários vanilla. Para cada nome candidato, medir:

1. existência global (`black`) e estática (`color.black`);
2. `type`, igualdade entre as duas superfícies e `repr`;
3. valor/canais e espaço de cor observáveis pela linguagem, usando
   `components()`, `space()` e `to-hex()` quando aplicáveis;
4. ordem do scope somente se houver uma porta de linguagem que a torne
   observável;
5. comportamento de nome inexistente e de aliases/extras encontrados no
   cristalino (`cyan`, `magenta`, `none`, `pink`, `ostrich`), sem presumir que
   devam ser adicionados ou removidos.

O inventário histórico do L0 contém 18 candidatos:

```text
black gray silver white navy blue aqua teal eastern
purple fuchsia maroon red orange yellow olive green lime
```

Essa lista é hipótese a reconfirmar contra `a51e02804`; não é prova por estar
documentada. As sondas devem testar a família inteira, não apenas
`color.black`.

Sonda mínima sugerida, gerada com identifiers literais para cada nome (exemplo
de `black`):

```typ
#metadata((
  global-type: type(black),
  static-type: type(color.black),
  equal: black == color.black,
  global-repr: repr(black),
  static-repr: repr(color.black),
  space: color.black.space(),
  components: color.black.components(),
  hex: color.black.to-hex(),
)) <probe>
```

Não usar lookup dinâmico como substituto da superfície literal: gerar uma
entrada por identifier torna ausências individuais visíveis. Sondas negativas
devem ficar separadas para que um erro não esconda outro.

## 4. Fase B — auditar o cristalino

Documentar com `file:line` atuais:

- todos os consumers de `predefined_color_bindings()` e se dependem da ordem;
- todas as entradas de `color_type_field()`;
- testes que já fixam o conjunto global e os canais das cores;
- qualquer segunda tabela de cores no owner ou fora dele, distinguindo a
  superfície de linguagem dos parsers CSS/string de outros domínios;
- a representação interna de `Color` necessária para preservar espaço,
  componentes, igualdade e `repr` observáveis;
- a situação dos extras cristalinos, tratada como compatibilidade separada e
  não como autorização para inseri-los em `color.*`.

Classificar explicitamente segundo ADR-0107/0108:

- nomes, disponibilidade nas duas superfícies, igualdade de linguagem,
  `space()`, `components()` e `repr` são semântica/morfologia observável;
- disposição da tabela, função Rust que a consulta e bytes internos são
  mecânica livre;
- inferência inicial: global e namespace devem consultar a mesma tabela
  canônica. Seria refutada por fonte ratificada que exponha conjuntos ou
  valores deliberadamente diferentes.

## 5. Decisão e L0 antes do código

Somente depois das fases A e B, atualizar primeiro
`00_nucleo/prompts/compiler/stdlib/color.md` para registrar:

1. conjunto ratificado completo e valores observáveis;
2. quais nomes pertencem ao global, ao namespace `color` ou a ambos;
3. política explícita para extras cristalinos preexistentes;
4. tabela canônica única e consumidores permitidos;
5. critérios de erro, igualdade, `repr`, espaço e componentes no nível da
   língua;
6. qualquer scope-out descoberto, com passo futuro nomeado.

Se a medição exigir alterar representação/variante pública de `Color`,
assinatura pública, comportamento deliberado por defeito ou compatibilidade,
atualizar os L0s afetados e **parar no gate ADR-0127**. Em caso de dúvida,
parar.

Se o lote for somente adição/correção de entradas em tabela para paridade,
seguir em fluxo contínuo pelo ADR-0127: L0 primeiro, resselo, RED→GREEN e
revalidação. Registrar a classificação antes de escrever código.

## 6. Plano RED→GREEN

Depois do L0 válido e de eventual gate:

1. escrever RED tabelado que percorra o conjunto ratificado e exija cada
   `color.<nome>`;
2. provar igualdade de linguagem entre `<nome>` e `color.<nome>`;
3. comparar `space`, `components`, `to-hex` e `repr` com as medições vanilla,
   sem usar apenas igualdade Rust ou bytes de render;
4. escrever negativos para campo inexistente e para extras que não pertençam
   ao namespace ratificado;
5. extrair uma tabela canônica no owner, reutilizada por bindings globais e
   `color_type_field`, sem duplicar literais RGB;
6. preservar constructors e operadores já existentes no namespace;
7. ressellar headers afetados;
8. executar testes focados, regressões de cor e field access,
   `cargo check --workspace`, `cargo build --workspace`,
   `crystalline-lint .` e `git diff --check`.

O teste tabelado deve falhar informando o nome individual ausente ou divergente;
um único booleano agregado sem diagnóstico não é suficiente para manter a
família auditável.

## 7. Limites e critério de encerramento

Ficam fora de P1143:

- novos constructors ou operadores de cor;
- alteração dos parsers CSS/string em `shapes.rs` ou outros owners;
- `gradient.kind`, accessors de `datetime`, bitwise de `int`, `counter.get`,
  `content.fields`, numbering, emoji e HTML;
- remoção automática de extras globais cristalinos;
- limpeza de warnings históricos.

P1143 encerra quando o conjunto completo medido estiver disponível nas
superfícies corretas a partir de uma fonte tabelada única, seus observáveis de
linguagem coincidirem com o vanilla ratificado, constructors/operadores não
regredirem e a validação final tiver zero violations. Se a auditoria revelar
mudança enquadrada pelo ADR-0127, o estado correto é `AGUARDA GATE`, não uma
implementação parcial.

## 8. Execução

P1143 foi executado em 2026-08-24. A fonte do objeto Git `a51e02804` e os dois
binários ratificados confirmaram, para os 18 nomes:

- presença global e em `color.<nome>`;
- `type(color.<nome>) == color` e igualdade com o binding global;
- quatro cores Luma: `black`, `gray`, `silver`, `white`;
- 14 cores sRGB com os hexadecimais documentados na tabela do L0;
- `repr`, `space()`, `components()` e `to-hex()` coincidentes nas duas
  instalações vanilla;
- `cyan` e `magenta` inexistentes tanto globalmente quanto no namespace;
  `color.none`, `color.pink` e `color.ostrich` igualmente inexistentes.

A medição refutou a classificação antiga do L0 que tratava Luma versus sRGB
como mera mecânica: espaço, componentes e `repr` tornam a diferença observável
na língua. O domínio cristalino já possuía `Color::Luma` e todos os consumers;
logo não foi necessário alterar representação pública, assinatura, trait,
default ou fase. O lote foi classificado como correção de tabela/paridade e
seguiu em fluxo contínuo pelo ADR-0127.

RED confirmado por `cargo test -p typst-core p1143_ --lib`: dois testes
falharam porque os 18 fields ainda não existiam; o teste de Luma também
capturava a representação global incorreta das quatro escalas de cinza. O
negativo dos extras já passava.

GREEN:

- `PREDEFINED_COLORS` é a única tabela canônica das 18 cores ratificadas;
- `predefined_color_bindings()` projeta essa tabela no global e acrescenta,
  separadamente, os extras cristalinos compatíveis `cyan`, `magenta` e `none`;
- `color_type_field()` consulta somente a tabela ratificada, depois dos 20
  constructors/operadores existentes;
- `black`, `gray`, `silver` e `white` passaram a preservar `Color::Luma`;
- nenhum extra cristalino foi promovido ao namespace.

Validações focadas:

- `cargo test -p typst-core p1143_ --lib`: 3/3 passam;
- `cargo test -p typst-core p687_ --lib`: 3/3 passam;
- `cargo test -p typst-core p736_color --lib`: 4/4 passam;
- `cargo check --workspace`: passa;
- `crystalline-lint --fix-hashes .`: ressellou `color.rs` para `8053e59b` e
  terminou com zero drift warnings.

As sondas temporárias em `temp/p1143/` foram removidas após transcrição. Entre
a fotografia inicial e a final, o dono commitou os lotes anteriores; por isso
a proveniência final não usa o HEAD inicial. Às `2026-08-24T19:15:47-03:00`,
HEAD `59e5cc97c3475f359986e8a8dcf2dac8d3548ad3`, o working tree não commitado
continha somente três ficheiros rastreados de P1143, com 238 inserções e 50
remoções, além deste passo untracked.

Validação final às `2026-08-24T19:16:38-03:00`, no HEAD
`59e5cc97c3475f359986e8a8dcf2dac8d3548ad3`:
`cargo fmt --all --check && cargo build --workspace && crystalline-lint . &&
git diff --check` terminou com exit 0. O linter reportou zero violations;
warnings e infos históricos permaneceram fora do lote.
