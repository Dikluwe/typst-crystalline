# Passo 1464 — materializar `format.pdf` e projetar os cinco membros de `pdf`

**Estado:** L0 CAUSAL ESCRITO — AGUARDA CONFIRMAÇÃO HUMANA PÓS-COMMIT
(ADR-0127)
**Data da medição:** 2026-09-19T22:06:57-03:00
**Baseline cristalino:** `a1286f958f15a539dca17a3696ef83ef9e5b1b78`
**Tree do baseline:** `15bba96f8a7d4ada71345df6fe67d4b3e1bfcd04`, limpa
(`git status --short`, `git diff` e `git diff HEAD --stat` sem saída)
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**SHA-256 dos dois binários vanilla medidos:**
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`

## 1. Objetivo e fronteira

P1464 materializa a superfície pública já reservada por P1461:

```text
format.pdf                         sempre
├── attach                         sempre; mesma função que pdf.attach
├── artifact                       sempre; mesma função que pdf.artifact
├── table-summary                  somente com a11y-extras
├── header-cell                    somente com a11y-extras
└── data-cell                      somente com a11y-extras
```

`format.pdf` é uma **função/elemento**, não um módulo. Seu namespace público
também expõe os defaults medidos `pages: none`, `standard: ("1.7",)`,
`tagged: auto` e `pretty: false`. A chamada direta permanece proibida com
`cannot be constructed manually`.

O recorte reutiliza as cinco funções produtivas do módulo global `pdf`; não
cria wrappers, cópias das implementações, stubs nem funções que apenas imitam
o nome. Nos perfis sem `a11y-extras`, o trio gated não existe e o acesso
produz o diagnóstico público de feature desativada. Nos perfis com a feature,
o trio aparece em conjunto.

Este passo fecha somente descoberta, identidade, defaults de namespace e
gates dos seis caminhos reservados. Não afirma paridade geral de
`format.pdf`, nem implementa as quatro opções documentais até o exportador.
Essa incompletude é explícita na seção 5.

## 2. Medição anterior à decisão

### 2.1 Vanilla ratificado

Os dois binários vanilla têm o mesmo SHA-256 acima. Em
`/usr/local/bin/typst`, os probes produziram:

```text
repr((type(format.pdf), format.pdf, format.pdf.pages,
      format.pdf.standard, format.pdf.tagged, format.pdf.pretty))
=> (function, pdf, none, ("1.7",), auto, false)

repr((type(format.pdf.attach), format.pdf.attach == pdf.attach,
      type(format.pdf.artifact), format.pdf.artifact == pdf.artifact))
=> (function, true, function, true)

{ let f = format.pdf;
  repr((f, f.attach == pdf.attach, std.format.pdf == format.pdf)) }
=> (pdf, true, true)

repr((format.pdf.attach, format.pdf.artifact))
=> (attach, artifact)

format.pdf()
=> erro: cannot be constructed manually
```

Sem `a11y-extras`, `format.pdf.table-summary` terminou com exit 1 e:

```text
cannot access field `table-summary` because the `a11y-extras` feature is not enabled
hint: try enabling the `a11y-extras` feature
hint: see https://typst.app/help/compiler-features for more details
```

Com `--features a11y-extras`:

```text
(type(format.pdf.table-summary), format.pdf.table-summary == pdf.table-summary,
 type(format.pdf.header-cell), format.pdf.header-cell == pdf.header-cell,
 type(format.pdf.data-cell), format.pdf.data-cell == pdf.data-cell)
=> (function, true, function, true, function, true)

repr((format.pdf.table-summary, format.pdf.header-cell,
      format.pdf.data-cell))
=> (table-summary, header-cell, data-cell)
```

Com `pretty: true` pré-ligado por `.with`, `attach` e `artifact` devolveram
`(true, true)` contra os globais; no perfil `a11y-extras`, o trio devolveu
`(true, true, true)`. P1464 exige que os cinco membros funcionais sobrevivam a
`.with`; não dá crédito à projeção dos quatro campos de opção depois de
`.with`, que é um observável distinto e não foi incluído no recorte.

A fonte ratificada confirma a medição:

- `typst-library/src/format.rs:90-100` instala cada elemento de formato e o
  gate da própria feature;
- `typst-pdf/src/format.rs:102-348` declara `pages`, `standard`, `tagged` e
  `pretty`;
- `typst-pdf/src/format.rs:351-360` torna o elemento não construtível;
- o scope de `PdfFormat` projeta `AttachElem`, `ArtifactElem` e o trio de
  acessibilidade.

Hashes das fontes usadas para decidir:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/format.rs` | `37ce18defae73642aa9e6f74cb8a78dffba575b004d9a8ddfbf52cb272898a1f` |
| `lab/typst-original/crates/typst-pdf/src/format.rs` | `a7270293467eb6b918df5dd238650b8bdbe0ab5463dc03dda979a6baba04c432` |

### 2.2 Estado cristalino

No baseline medido:

- `compiler/stdlib/format.rs:29-56` cria somente `svg` e `png`; o teste do
  próprio owner exige que `pdf` e `html` estejam ausentes;
- `compiler/stdlib/pdf.rs:36-58` já constrói as cinco funções corretas e aplica
  o gate `A11yExtras` ao trio;
- `compiler/eval/mod.rs:7773` registra `format`, enquanto `:8357` constrói e
  registra `pdf` separadamente;
- `compiler/eval/bindings/field_access.rs:505-526` reconhece o diagnóstico de
  feature somente quando o alvo é o módulo global chamado `pdf`; ainda não
  cobre a função `format.pdf`.

Hashes do código lido:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/format.rs` | `69acc937a3464ae12f364aec023eb4b75792da867e9bca7647d796e0ba15b4f3` |
| `01_core/src/compiler/stdlib/pdf.rs` | `b1d7b325eace8f6d819a16c6ff7c135f1ca42781bacaec71888045dbd977ce4d` |
| `01_core/src/compiler/eval/mod.rs` | `8d879e3a3e005834793c543f202edd2aa80bda1bd5be698db3d24b1be1b9d824` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `596d67d2d70cd856a2739a9ad8bdb739676b5857121cd9ea7f3e2bce47a158d4` |

Os binários cristalinos presentes em `target/debug` e `target/release`
reportam `bf0a29ca`, anterior à implementação P1463, e por isso foram
deliberadamente excluídos da medição. A execução de P1464 deve produzir o
binário descartável a partir do baseline correto em diretório exclusivo de
`/dev/shm`.

## 3. Classificação e decisão

São linguagem pública: tipo e nome de `format.pdf`, quatro defaults visíveis,
construção proibida, igualdade dos cinco membros com `pdf.*`, presença por
feature, diagnóstico e hints do gate, aliases e sobrevivência dos membros a
`.with`.

São mecânica livre: ordem de inserção em `Scope`, endereço de `Arc`, forma da
factory, clone da `Value`, local onde o valor PDF é temporariamente guardado e
uso ou não de helper privado. P1464 compara identidade linguística, não
identidade de ponteiro.

Decisão: construir o módulo global `pdf` uma única vez na composição do scope
base, projetar seus bindings já existentes no namespace da função
`format.pdf` e depois registrar o mesmo valor como binding global `pdf`.
Assim os dois caminhos não mantêm catálogos paralelos e qualquer gate é
derivado da mesma instância efetiva.

O namespace de `format.pdf` contém:

```text
pages    = none
standard = ("1.7",)
tagged   = auto
pretty   = false
attach/artifact = clones dos valores do módulo pdf
trio a11y        = clones somente quando presente no módulo pdf efetivo
```

O nome da função é `pdf`; o constructor usa o mesmo erro não construtível já
usado por `format.svg` e `format.png`. `format.pdf` não pode ser representado
como `Value::Module`.

O ramo de diagnóstico `a11y-extras` em field access deve aceitar também a
função nativa `pdf` com namespace de formato, sem alterar o diagnóstico do
módulo global e sem atingir closures de utilizador sem esse namespace.

## 4. L0 e ownership antes do código

Antes de RED, teste ou implementação:

1. atualizar `compiler/stdlib/format.md` com o contrato P1464, o pin do Núcleo
   `compiler-feature-gates` e o estado materializado de P1463;
2. atualizar `compiler/eval.md` para construir/reutilizar uma única projeção
   efetiva de `pdf` ao compor os dois bindings;
3. atualizar `compiler/eval/bindings/field_access.md` para o diagnóstico gated
   sobre `format.pdf`;
4. criar `wiring/tests/p1464-format-pdf.md` como owner 1:1 do teste de
   integração independente `04_wiring/tests/p1464_format_pdf.rs`.

O Núcleo `compiler-feature-gates` permanece semanticamente inalterado. Seu pin
efetivo vigente é
`59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9`.
Código não aponta para Núcleo.

| Prompt L0 | Consumer exclusivo |
|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` |
| `compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` |
| `compiler/eval/bindings/field_access.md` | `01_core/src/compiler/eval/bindings/field_access.rs` |
| `wiring/tests/p1464-format-pdf.md` | `04_wiring/tests/p1464_format_pdf.rs` |

`compiler/stdlib/pdf.md` e `compiler/stdlib/pdf.rs` permanecem inalterados: as
cinco funções já possuem owner e implementação produtiva. Duplicar ali uma
segunda factory para `format.pdf` violaria a fronteira desta decisão.

## 5. Incompletude explícita e passos de conclusão

P1464 não valida nem aplica `#set format.pdf(...)`. Também não adiciona
carriers, chaves de estilo, flags, seleção de páginas ou conformidade PDF. A
presença dos defaults no namespace não autoriza alegar que as opções já chegam
ao exportador.

O L0 de `compiler/stdlib/format.md` deve registrar esta divisão:

- **P1465:** mantém a reserva anterior para `format.html` e seus membros;
- **P1466:** materializa `format.pdf.tagged` e `format.pdf.pretty` de ponta a
  ponta, incluindo precedência entre CLI, documento e default;
- **P1467:** materializa `format.pdf.pages` e `--pages`, com seleção física e
  interação medida com tagging;
- **P1468:** materializa `format.pdf.standard` e `--pdf-standard`, incluindo
  validação e conformidade real; mudar apenas o cabeçalho `%PDF-*` não fecha
  PDF/A nem PDF/UA.

Até esses passos, nenhuma set rule PDF recebe crédito de paridade. O fallback
atual não pode ser citado como suporte e nenhum valor pode ser silenciosamente
aceito e descartado pela implementação P1464.

## 6. Gate ADR-0127

P1464 adiciona uma função pública, campos públicos e bindings condicionais.
Logo exige a única paragem obrigatória deste passo:

1. escrever os quatro L0s da seção 4 e ressellar hashes;
2. validar V15/V26 e commitar passo + L0, sem código produtivo e sem RED;
3. **parar para confirmação humana posterior ao commit causal**;
4. somente depois executar implementação e testes independentes.

A confirmação de P1463 não autoriza P1464.

## 7. Execução segregada após confirmação

Aplicar exatamente os três papéis da skill relida:

1. **Agente A — implementação:** recebe os L0s commitados e altera somente o
   código necessário; não escreve testes;
2. **Agente B — testes:** recebe os mesmos L0s e o mesmo baseline, sem ver
   código, diff ou raciocínio de A; escreve somente os testes;
3. **Agente C — integração:** o agente principal integra as duas saídas,
   executa primeiro os testes novos e depois a suíte proporcional.

Compilações, executáveis e caches descartáveis ficam em diretório exclusivo
de `/dev/shm`. Se ele não estiver disponível ou não tiver espaço, a execução
para em vez de redirecionar silenciosamente ao disco.

Não criar manifesto, contrato auxiliar, selo, certificado, recibo, campanha de
mutantes, adversário ou ADR nova. Os artefatos do passo são somente L0, código,
testes e resultado da execução.

## 8. Testes independentes e aceitação

O teste escrito por B deve cobrir bilateralmente os perfis `default`,
`a11y-extras`, `html` e `html+a11y`:

- `type(format.pdf) == function`, `repr(format.pdf) == "pdf"` e os quatro
  defaults exatos;
- `format.pdf()` falha com `cannot be constructed manually`;
- alias lexical e `std.format.pdf` preservam identidade;
- `attach` e `artifact` existem sempre, têm nomes curtos e comparam iguais a
  `pdf.attach` e `pdf.artifact`;
- o trio gated é totalmente ausente sem `a11y-extras`, com mensagem, dois
  hints e span no field iguais ao vanilla;
- com `a11y-extras`, as três funções existem, têm nomes curtos e comparam
  iguais às funções globais;
- os cinco membros funcionais preservam o namespace através de `.with`, como
  nos probes medidos;
- `format.svg`, `format.png`, o módulo global `pdf` e seus gates não regridem;
- `format.html` continua ausente;
- nenhum teste declara `#set format.pdf(...)` como suportado neste passo.

Contabilidade esperada, ainda não usada como prova: nos perfis sem a feature,
P1464 pode mover três rotas (`format.pdf`, `attach`, `artifact`) de
`VANILLA_ONLY` para `BOTH`; com `a11y-extras`, pode mover seis. O observador
P1357 deve medir o resultado real antes de fechar o passo e não deve fabricar
aliases para alcançar essa projeção.

## 9. Gates finais

Após confirmação, A/B e integração:

```text
cargo test -p typst-core <testes focais de format/pdf/field_access>
cargo test -p typst-wiring --test p1464_format_pdf
cargo test --workspace
cargo build
crystalline-lint .
git diff --check
```

Executar também os probes vanilla/cristalino desta especificação e o
inventário P1357 nos quatro perfis. A suíte global pode conservar somente
falhas históricas reproduzidas no baseline; qualquer falha nova bloqueia.

O veredito final deve dizer apenas: superfície `format.pdf`, defaults de
namespace, cinco membros reutilizados e gates medidos. Não declarar paridade
de opções documentais, export PDF, PDF/A, PDF/UA, seleção de páginas, tagging
ou serialização pretty.
