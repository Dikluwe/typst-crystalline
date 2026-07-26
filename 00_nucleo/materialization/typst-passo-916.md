# Passo 916 — fechar P912/913/914: prova geométrica, hermeticidade de teste, reconciliação retroativa de L0

**Precede este passo**: o "Relatório de Proveniência" que audita P912/913/914 (achados 1-4). Ler
antes de começar — os quatro problemas já estão isolados, este passo é para resolver, não
redescobrir. **`P915` está reservado para "cramped"** (P914 já anunciou isso) — não usar esse
número aqui.

**Pré-condição de árvore**: `git status`. HEAD confirmado em `dcff44bd2`.

---

## Parte A — a prova que falta desde P912 (prioridade mais alta)

**Sem isto, P912/913 não podem ser considerados fechados** — é a única coisa que confirma que o
bug catalogado por P911 foi de facto corrigido, não só que código foi escrito com essa intenção.

1. Recompilar, com o binário actual (`dcff44bd2`), exactamente os casos que P911 mediu:
   `(1/2)`, `(1/2/3/4)`, `(1/2/3/4/5/6/7/8)`, `mat(1,2;3,4)`, `mat(...)` 6 linhas, `cases(1,2)`,
   `cases(...)` 8 linhas.
2. Medir com `mutool trace` o glyph ID real emitido para o delimitador em cada caso (mesmo método
   de P911 — confirmar mesma fonte `NewCMMath-Book` nos dois lados, `mutool info`).
3. Montar a mesma tabela de P911 (cristalino antes / cristalino agora / vanilla), confirmando que o
   glifo **cresce** com o conteúdo no cristalino agora, e idealmente casa com o vanilla (não precisa
   ser o mesmo glifo exacto — vanilla e cristalino podem escolher variantes diferentes da mesma
   fonte — mas tem de crescer, não ficar `"inalterado"` como antes).
4. Se algum caso ainda não crescer: isto é um achado novo, não um fechamento — registar com o
   mesmo rigor de P911, não tentar remendar apressadamente dentro deste passo sem entender a causa.

## Parte B — hermeticidade do teste `pdf_tounicode_contem_mapeamento_de_delimitador`

Confirmado pelo relatório de proveniência: o teste lê `/usr/share/fonts/...` do sistema **antes**
de cair para uma fixture, e a fixture de fallback (`NimbusSans-Regular.otf`) não tem tabela MATH —
o teste pode passar "pelas razões erradas" nessa máquina.

1. Confirmar se existe (ou criar) uma fixture pinada com tabela MATH real em
   `03_infra/fixtures/fonts/` — candidato óbvio: o próprio `NewCMMath-Book.otf` já usado em todo o
   resto desta frente (P890-914), copiado do checkout de `typst-assets` já confirmado como fonte de
   verdade nesses passos.
2. Reescrever o teste para usar **só** essa fixture, sem `std::fs::read("/usr/share/fonts/...")`
   nem fallback condicional — mesma disciplina de `ADR-0122` (paridade de defeitos nos testes):
   fonte fixa, conhecida, a mesma nos dois lados se o teste também comparar contra vanilla.
3. Confirmar que `world_from_str`/`SystemWorld` usado por este teste regista a fonte correctamente
   (`.with_fonts()` ou equivalente) — o relatório de proveniência já notou que `font_slots` ficava
   vazio; confirmar se isso também precisa de correcção para o teste ser válido de facto, não só
   hermético.
4. Rodar o teste corrigido e confirmar que continua a passar — **pelas razões certas desta vez**
   (glifo com ToUnicode de variante MATH real, não coincidência de mapeamento de texto comum).

## Parte C — reconciliação retroativa dos dois commits sem relatório

Os dois commits (`ffabfd837` — `delim:` em `mat()`/`vec()`; `5847e1ea0` — `axis_height` +
`STRETCHY_BASES`) já têm código em produção, testes verdes, e (parcialmente) L0 actualizado, mas
**sem relatório de passo e, no caso de `ffabfd837`, sem L0 escrito antes do código** — violação
confirmada da trava arquitectural (L0 precede L1).

1. Para `ffabfd837`: escrever agora, retroactivamente, a spec em `prompts/engine/eval.md` (ou onde
   fizer sentido) descrevendo `delim:` como se tivesse sido escrita antes — e um relatório curto
   marcado explicitamente **"reconciliação retroativa"** (mesmo espírito do mecanismo já usado no
   projecto para specs escritas fora de ordem, ex. `ADR-0114` — verificação retroativa em vez de
   pretender que a ordem foi respeitada). Não apagar nem reescrever o commit — só documentar o que
   já existe, com a marca honesta de que veio depois.
2. Para `5847e1ea0`: escrever um relatório curto (pode ser mais breve que um passo completo, já que
   é continuação directa do Achado A de P911/P912) explicando causa, correcção, e por que ficou de
   fora do relatório de P912 original.
3. Confirmar que os hashes de L0 destes dois ficheiros batem com o código actual
   (`crystalline-lint .`, 0 drift) — se já estiverem sincronizados (o relatório de proveniência
   sugere que sim, via commit de P914), só confirmar, não refazer.

## Parte D — revisão "trust but verify" da geometria de P912/913/914

Nenhum dos três relatórios confirma se o protocolo de dois agentes (pedido explicitamente pelos
prompts de P912 e P914, por classificação de risco geométrico) foi seguido. Não dá para voltar
atrás e aplicá-lo agora — mas dá para fazer o que esse protocolo existia para garantir: uma
revisão independente, cética, do que foi escrito, mesmo padrão já usado em P898/901/906/908
("revisão do orquestrador").

1. Ler o diff completo de `attach.rs` (P914) e `covering()`/`stretchy.rs`/`assembly.rs` (P912/913)
   linha a linha, não confiar no resumo dos relatórios.
2. Testar pelo menos um caso composto não coberto pelos testes já escritos — por exemplo, para
   P914: sub+sup simultâneos **com** base de descent extremo, ao mesmo tempo (o próprio prompt de
   P914 sugeria isto como teste da revisão). Para P912/913: um delimitador com conteúdo
   assimétrico em torno do eixo (testa a fórmula `balanced` de `delimited.rs`, não só matrizes).
3. Registar o resultado — confirma que está correcto, ou encontra mais um achado (mesmo padrão de
   P898, que encontrou o bug de `dy` descartado exactamente nesta fase).

## Resultado esperado

- Tabela de `mutool trace` pós-correcção, números reais, comparável à de P911.
- Teste `pdf_tounicode_...` hermético, com fixture pinada, sem leitura de `/usr/share/fonts`.
- Dois relatórios retroativos curtos para `ffabfd837`/`5847e1ea0`, marcados como reconciliação, com
  L0 escrito (mesmo que depois do código, documentado como tal).
- Resultado da revisão cética da Parte D — confirma ou encontra achado novo.
- Suíte completa verde, discriminada por crate, no fim de tudo.
