# Passo 1461 — nuclear o módulo público `format` sem placeholders

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-19T08:00:49-03:00
**Commit medido:** `7392f490397c56876fcf6134e86c44b528b396bb`
**Árvore medida:** `ebda99b6b71a8d46cfc021a94866b4c4cded4ed2`, limpa
(`git status --short` e `git diff HEAD --stat` sem saída)
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**Regime:** protocolo completo de materialização segregada, a executar como
`executed_without_isolation_attestation` no workspace compartilhado

## 1. Unidade e resultado pretendido

O inventário fresco deixou 141 caminhos públicos `VANILLA_ONLY`; 124 pertencem
à família `format.*`. Este passo inicia o fecho pela única unidade que é comum
a todos eles: a raiz pública `format`.

P1461 fecha **somente** o caminho `format`, como módulo nomeado e sempre
presente. Não cria `format.svg`, `format.png`, `format.pdf`, `format.html`,
opções de exportação, set rules, aliases, stubs nem valores marcadores. A
ausência dos filhos continua explícita até os passos verticais que possuem
contrato próprio.

Resultado estrutural esperado do recorte: 141 → 140 caminhos únicos ausentes e
328 → 324 células `VANILLA_ONLY`, sem crédito aos 123 descendentes restantes.

## 2. Medição anterior à decisão

### 2.1 Inventário público atual

O harness P1357 executado no HEAD medido, contra o vanilla ratificado, produziu
`PASS`, quatro hashes de determinismo idênticos e o artefato
`/tmp/p1461-current-inventory.json`, SHA-256
`067ce14b28f0b1bafa88f8fe0198b0a36f76248ec2ffd180e8e1e454845a4d80`.

| Perfil | BOTH_PRESENT | VANILLA_ONLY | CRYSTALLINE_ONLY |
|---|---:|---:|---:|
| default | 1998 | 23 | 42 |
| a11y-extras | 2001 | 26 | 42 |
| html | 2113 | 138 | 42 |
| html+a11y | 2116 | 141 | 42 |

Os 141 caminhos únicos dividem-se em 124 `format.*`, nove `math.*` e oito
membros isolados. Dentro de `format.*`, o denominador é: raiz `format` (1),
`format.html` e 114 membros (116), `format.pdf` e cinco membros (6),
`format.png` (1) e `format.svg` (1).

### 2.2 Fonte vanilla ratificada

Em `lab/typst-original/crates/typst-library/src/format.rs:89-102`, a função
`module` cria um módulo chamado `format` e registra nele somente os formatos
recebidos. Em `lab/typst-original/crates/typst-library/src/lib.rs:368-381`, a
raiz global registra esse módulo independentemente do target. O feature gate é
propriedade do filho, não da raiz (`format.rs:94-98`).

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/format.rs` | `37ce18defae73642aa9e6f74cb8a78dffba575b004d9a8ddfbf52cb272898a1f` |
| `lab/typst-original/crates/typst-library/src/lib.rs` | `a6ac296ad0029e8ee45c102f68195f8c439b2ad088c0d9317a77913946daf130` |

O binário vanilla medido tem SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
Os probes bilaterais registraram:

| Probe | Vanilla | Cristalino |
|---|---|---|
| `repr(type(format))` | `"module"` | erro `unknown variable format` |
| `repr(format)` | `"<module format>"` | erro `unknown variable format` |
| `repr(std.format)` | `"<module format>"` | ausente com a raiz |
| alias `let f = format; repr(f)` | `"<module format>"` | ausente com a raiz |
| `repr(format.absent)` | erro `module format does not contain absent` | erro anterior na raiz |

Nos quatro perfis `default`, `html`, `a11y-extras` e `html+a11y`, o vanilla
devolveu exatamente `(module, "<module format>")`. Selecionar target ou
feature não cria nem remove a raiz.

### 2.3 Estado cristalino e L0 vigente

`01_core/src/compiler/eval/mod.rs:7594-7773` constrói o scope base e registra
diretamente `html`, mas não registra `format`. Não existe módulo produtivo ou
Prompt L0 proprietário para a raiz. O `Module` e o `Scope` necessários já
existem; nenhum novo carrier de entidade é necessário neste recorte.

Identidades do baseline:

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval.md` | `85a407c1c1c877ea60f82de255a5faa94d862c057c9ef0ccd3ccfd80780e98d9` |
| `00_nucleo/prompts/compiler/stdlib/_comum.md` | `61255791dbf880217985fb2a8b42cdb2bdea9a84850be3998e7e7e81302f8e3f` |
| `00_nucleo/prompts/entities/module.md` | `17d75e8c05e28546b7212ae232fa5d9e0474eda2b5bd7b67740e54ec3c1659a9` |
| `01_core/src/compiler/eval/mod.rs` | `a014d409e6a0e53d05d11365be28d77bda89b03f6da2a03807767e69ceeb5fa1` |
| `01_core/src/compiler/stdlib/mod.rs` | `d0640072de23c2811fc6e373c9c80c77b3628bc8c7ca267b11f9235aaedb031d` |
| `01_core/src/entities/module.rs` | `da6380bef91ebaa1166ba8f76cba953671e9bab210fbcd2a8b22e667802ab49f` |
| `target/release/typst` | `e634f2e4c54f6211837b65412787b231f9d738caf88b647b1729365c4cbcd633` |

## 3. Classificação anti-deriva

- Existência da raiz, tipo `module`, identidade nominal `format`, `repr`,
  estabilidade por alias/`std` e disponibilidade nos quatro perfis são
  semântica e morfologia públicas da linguagem (ADR-0107).
- `Scope::deduplicating`, o vetor dinâmico `Format`, traits, downcast, ordem de
  alocação e a categoria interna do vanilla são mecânica; não são contrato.
- Os filhos são elementos configuráveis, não módulos nem aliases dos bindings
  globais `html`/`pdf`. Criar módulos filhos ou funções passthrough para ganhar
  contagem seria falsa paridade.
- Inferência: o `Module`/`Scope` cristalino vigente basta para fechar a raiz sem
  novo tipo. Refutam a inferência a necessidade de mudar `entities/module.rs`,
  lookup genérico, `Value`, feature gates ou representação pública de módulos.
  Se alguma refutação ocorrer, parar e redesenhar antes do código.

## 4. Decisão L0 proposta

Antes de qualquer teste ou código produtivo:

1. criar `00_nucleo/prompts/compiler/stdlib/format.md`, proprietário 1:1 do
   futuro `01_core/src/compiler/stdlib/format.rs`;
2. atualizar `compiler/stdlib/_comum.md` somente para declarar o submódulo e o
   reexport interno explícito da factory;
3. atualizar `compiler/eval.md` somente para legitimar o registro da raiz no
   scope global.

O novo owner define uma factory interna equivalente a:

```rust
pub(crate) fn make_format_module() -> Module
```

P1461 exige que a factory devolva `Module::new("format", Scope::new())`, sem
filhos. A fachada reexporta a factory sem lógica; `make_stdlib_with_features`
registra `format` como `Value::Module` em toda construção real do global. O
binding `std` alcança o mesmo valor pelo scope global já existente; não se cria
binding global `global`, `svg`, `png`, `pdf` ou novo `html`.

`entities/module.md` foi lido e permanece inalterado: nome, clone `Arc`, scope
e `repr` existentes já satisfazem a unidade. Alterá-lo neste passo é fuga de
escopo.

## 5. Incompletude explícita e passos de conclusão

O Prompt L0 inicial de `compiler/stdlib/format.md` deve declarar, no cabeçalho
e no contrato, que o módulo está **deliberadamente incompleto**. Esta divisão é
obrigatória pela regra de constructor fracionado entre passos.

- **P1462:** materializar `format.svg` verticalmente, incluindo elemento não
  construtível, `pretty: bool = false`, set rule top-level, opções do documento
  e precedência da flag CLI explícita;
- **P1463:** materializar `format.png`, incluindo `ppi = 144`, validação e
  transporte até o rasterizador;
- **P1464:** materializar `format.pdf` e seus cinco membros/gates, sem confundir
  o elemento de formato com um módulo;
- **P1465:** materializar `format.html`, seu `pretty`, o gate `html` e o scope
  de 114 membros, reutilizando identidades sem duplicar implementações.

Esses números reservam a sequência lógica, não autorizam seu código. Se uma
medição posterior mostrar pré-requisitos independentes, o passo correspondente
deve registrar nova subdivisão no L0 antes de materializar.

Até P1462–P1465, `format.svg`, `format.png`, `format.pdf` e `format.html`
continuam ausentes no cristalino. P1461 não recebe crédito por eles e não pode
introduzir placeholders para modificar a contagem.

## 6. Gate ADR-0127

Adicionar o binding público `format` é mudança de contrato público. O fluxo é:

1. escrever e ressellar os três L0s da seção 4;
2. validar V15/V26 e commitar passo + L0;
3. **PARAR sem criar `format.rs`, sem editar L1 e sem alterar headers de
   linhagem**;
4. prosseguir somente após confirmação humana explícita posterior ao commit;
5. depois da confirmação, congelar manifesto, contrato, oráculos, ataques e
   selo antes da implementação.

Esta proposta não constitui a confirmação e não autoriza antecipar RED ou
candidato.

## 7. Cadeia segregada após confirmação

Artefatos mínimos:

- `p1461-manifest.json`: prompts, baseline, binários, perfis, comandos,
  capacidades e política de `Unknown`;
- `p1461-contract.json`: observáveis da raiz e limites dos filhos;
- `p1461-oracles.json`: positivos vanilla, controles cristalinos RED e casos
  opacos, sem leitura do candidato;
- `p1461-mutants.json`: mutações negativas e equivalentes mecânicos;
- `p1461-preseal.json`: hashes protegidos e mutation score `1.0`;
- `p1461-red.json`: falha atribuível à raiz ausente;
- `p1461-results.json` e `p1461-verifier-certificate.json`: campanha final e
  veredito limitado ao fragmento.

Papéis e autoridades:

1. **autor da intenção/L0:** escreve somente passo e prompts; não aprova o
   próprio candidato;
2. **autor do contrato:** lê L0, baseline e fonte ratificada; não lê nem
   executa implementação candidata;
3. **autor dos oráculos:** mede vanilla e baseline pré-candidato; não adapta
   expected após observar o patch;
4. **adversário:** recebe prompt, contrato e baseline; escreve apenas mutantes
   e não corrige a solução;
5. **implementador:** recebe somente L0 e contrato selado; não edita manifesto,
   contrato, oráculos, mutantes ou preseal;
6. **verificador:** lê artefatos selados e saídas; não escreve produto nem
   corrige entradas protegidas.

O workspace compartilhado impede atestar isolamento físico. Registrar as
allowlists, contexto recebido e hashes, e usar a alegação proporcional
`executed_without_isolation_attestation`.

## 8. Contrato observável congelável

Nos quatro perfis, exigir bilateralmente:

- `repr(type(format)) == "module"`;
- `repr(format) == "<module format>"`;
- `repr(std.format) == "<module format>"`;
- alias lexical conserva a mesma identidade e repr;
- `format.absent` produz a classe e mensagem de campo ausente de módulo
  nomeado `format`, com span no field;
- a raiz não depende de target, `html` ou `a11y-extras`;
- nenhuma rota filha ou alias global novo aparece neste passo;
- os bindings globais existentes `html`, `pdf`, `png`/`svg` CLI e seus feature
  gates permanecem byte a byte nos controles congelados.

O contrato compara valores, diagnóstico público, Unicode e spans. Não compara
endereço de `Arc`, layout de `Scope`, ordem de hash, bytes do binário ou
estrutura Rust.

Qualquer timeout, OOM, swap não permitido, truncamento, identidade de binário
incorreta, baseline drift, parser opaco ou divergência de ordem é `Unknown` e
bloqueia. `Unknown` nunca vira `Preserved` por default.

## 9. RED, ataques e implementação autorizada

O RED atribuível deve demonstrar que o baseline falha porque `format` é
variável desconhecida, enquanto controles globais adjacentes continuam
válidos. Depois do selo, a implementação limita-se ao novo owner, reexport da
fachada, registro no eval, testes e headers/hashes correspondentes.

Mutantes obrigatórios:

1. omitir o binding;
2. registrar dictionary, função ou `none` em vez de módulo;
3. construir módulo com nome `global`, `std` ou vazio;
4. instalar a raiz apenas em um perfil/feature;
5. fazer `std.format` guardar identidade distinta;
6. criar placeholder `svg`, `png`, `pdf` ou `html`;
7. criar alias global de filho;
8. fabricar o `repr` por condição textual sem corrigir o valor.

Todos os mutantes válidos devem ser `Violated`, positivos devem ser
`Preserved` e opacos deliberados devem ser `Unknown`. Exigir mutation score
`1.0`, vetores idênticos em ordem direta, direta repetida e reversa. Duas
revisões consecutivas sem ganho na mesma causa acionam redesenho antes de uma
terceira campanha completa.

## 10. Contabilidade e gates de fecho

Se e somente se o fragmento passar, o inventário P1357 deve produzir:

| Perfil | BOTH_PRESENT esperado | VANILLA_ONLY esperado | CRYSTALLINE_ONLY esperado |
|---|---:|---:|---:|
| default | 1999 | 22 | 42 |
| a11y-extras | 2002 | 25 | 42 |
| html | 2114 | 137 | 42 |
| html+a11y | 2117 | 140 | 42 |

União esperada: 140 caminhos únicos e 324 células `VANILLA_ONLY`. Qualquer
queda maior significa filho/alias não autorizado; qualquer queda menor impede
o fecho. A contagem é prova de superfície, não equivalência funcional geral.

Gates finais após confirmação e implementação:

1. verificar que prompts, contrato, oráculos, mutantes e preseal protegidos não
   mudaram;
2. executar testes focais de root/identity/diagnóstico/perfis e controles
   adjacentes;
3. repetir o inventário P1357 com binários rebuildados e identidades SHA-256;
4. `cargo fmt --all -- --check`;
5. `cargo test --workspace` — falha só pode ser classificada fora do recorte
   se reproduzida e congelada no baseline; não recebe crédito verde;
6. `cargo build --workspace`;
7. `crystalline-lint .` com zero violations/drift;
8. `git diff --check` e árvore/proveniência final registradas.

O certificado final pode afirmar somente: raiz pública `format` preservada no
fragmento P1461. Não pode afirmar fecho de `format.*`, equivalência geral de
exportadores ou paridade total da linguagem.
