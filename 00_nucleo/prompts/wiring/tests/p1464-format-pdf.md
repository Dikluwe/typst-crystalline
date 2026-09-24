# Prompt L0 — integração de `format.pdf`
Hash do Código: 42bd80e3

**Estado:** MATERIALIZADO APÓS CONFIRMAÇÃO ADR-0127.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Camada:** L4 (testes de integração)
**Ficheiro alvo:** `04_wiring/tests/p1464_format_pdf.rs`
**Consumer exclusivo:** `04_wiring/tests/p1464_format_pdf.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Responsabilidade

Executar o binário cristalino como processo externo e provar somente a
superfície pública `format.pdf`: função não construtível, quatro defaults de
namespace, cinco membros reutilizados e gates de `a11y-extras`. O teste não
importa módulos privados, não implementa opções documentais e não inspeciona
identidade de ponteiro, hashes ou ordem interna de inserção.

## Matriz obrigatória

Executar bilateralmente os perfis:

| Perfil | Features explícitas |
|---|---|
| `default` | nenhuma |
| `a11y-extras` | `a11y-extras` |
| `html` | `html` |
| `html+a11y` | `html,a11y-extras` |

Em todos os perfis, verificar exatamente:

- `type(format.pdf) == function`, `repr(format.pdf) == "pdf"`;
- `pages == none`, `standard == ("1.7",)`, `tagged == auto` e
  `pretty == false`;
- `format.pdf()` falha com `cannot be constructed manually`;
- alias lexical e `std.format.pdf` preservam tipo, nome e namespace;
- `attach` e `artifact` existem, têm nomes curtos e comparam iguais a
  `pdf.attach` e `pdf.artifact`;
- `format.pdf.with(pretty: true)` conserva os membros funcionais sem projetar
  como suportada a aplicação de `pretty` ao exportador.

Sem `a11y-extras`, cada acesso a `table-summary`, `header-cell` e `data-cell`
falha com exit de erro, stdout vazio, mensagem integral de feature desativada,
os dois hints na ordem vanilla e span cobrindo somente o field. Repetir ao
menos por acesso direto e por `.with`; alias e `std` não podem mudar o
resultado.

Com `a11y-extras`, o trio existe em conjunto, cada valor é `function`, tem
nome curto e compara igual ao correspondente `pdf.*`. Os cinco membros devem
permanecer descobertos através de `.with`.

## Controles de fronteira

- `format.svg` e `format.png` mantêm tipo, nome e defaults vigentes;
- a função global `pdf` com namespace, seus dois membros permanentes e o gate do trio não
  mudam;
- sucessão P1465: sem `html`, `format.html` recebe o gate público dessa
  feature; com `html`, o teste apenas confirma que o sibling existe como
  função e deixa namespace, set rule e export ao owner P1465;
- uma closure de utilizador chamada `pdf`, com ou sem fields homônimos, não
  recebe o diagnóstico especial de feature;
- nenhum caso usa `#set format.pdf(...)` como comportamento suportado;
- não alegar export PDF, PDF/A, PDF/UA, seleção de páginas, tagging ou
  serialização pretty.

Os testes com subprocesso devem ter timeout finito. Diretórios, caches e
arquivos descartáveis usam exclusivamente a raiz de tarefa fornecida sob
`/dev/shm`; indisponibilidade ou falta de espaço é falha `Unknown`, sem
fallback ao disco.

## Discriminação adversarial

O conjunto deve rejeitar implementações que criem uma segunda factory PDF,
copiem funções apenas pelo nome, exponham parte do trio sem a feature,
propaguem `html` como `a11y-extras`, percam namespace após `.with`, convertam
`format.pdf` em módulo, aceitem construção manual, mudem algum default ou
generalizem o diagnóstico a closures homônimas. Casos comportamentais úteis
permanecem na suíte; mutantes e resultados intermediários ficam somente em
`/dev/shm` e não são versionados.

## Gate

A confirmação humana explícita foi recebida em 2026-09-20, depois do commit
causal `dffb6b91ceb96503375b71e1afab5699c4eafc12`. O consumer foi escrito pelo
papel B sem acesso ao código do papel A. Contra o baseline causal, o primeiro
RED teve `1` teste aprovado e `3` reprovados; após a integração, os `4` testes
passaram. O header e o hash bidirecional são selados no commit de
materialização.

O controle de sibling foi sucedido pelo contrato P1465 confirmado depois do
commit causal `e300314e2f87e90e543c4923a8cae97f9ad978a8`: ausência estrita
deixou de ser expectativa válida e foi substituída por gate/presença conforme
o perfil, sem ampliar a responsabilidade PDF deste teste.

## P1487 — retificação do controle global obsoleto

### Medição anterior à decisão

Em 2026-09-24, o vanilla pinado (`/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`)
e o cristalino da árvore de trabalho devolveram ambos
`["function","pdf","function","attach","function","artifact"]` para
`(repr(type(pdf)),repr(pdf),repr(type(pdf.attach)),repr(pdf.attach),repr(type(pdf.artifact)),repr(pdf.artifact))`.
O consumer `04_wiring/tests/p1464_format_pdf.rs:376` ainda exige
`["module","<module pdf>",...]` nos quatro perfis; o RED reproduz essa única
divergência por perfil. `compiler/eval.md` §P1471 já decidiu a forma vigente.
Trata-se de morfologia pública da linguagem, não de mecânica. A inferência
de oráculo obsoleto seria refutada se a sonda vanilla pinada ou os membros
do namespace divergissem.

### Decisão

O controle global exige `type(pdf) == function` e `repr(pdf) == "pdf"`,
mantendo `attach` e `artifact` e os gates existentes nos quatro perfis.
Não alterar código produtivo, expectativa de `format.pdf` nem os controles
de closures homônimas. A frase histórica anterior sobre módulo global é
superada somente quanto à forma de `pdf`.
