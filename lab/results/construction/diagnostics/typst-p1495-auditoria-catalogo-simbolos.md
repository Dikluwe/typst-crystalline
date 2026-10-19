# P1495 — auditoria do catálogo de símbolos e compatibilidade

## Proveniência da medição

- HEAD cristalino: `3d61023e46475cd6e5df52021164523c84e9f074` (P1494).
  A primeira auditoria, em 2026-09-24 23:51–23:56 UTC, ocorreu com working
  tree limpa. A execução final do harness revisado ocorreu na janela
  2026-09-25 00:42:42–00:44:05 UTC, em working tree não commitada.
  O inventário `git status --short` foi idêntico antes e depois da sonda:
```text
 M 00_nucleo/materialization/typst-passo-1495.md
 M 00_nucleo/materialization/typst-passo-1496.md
 M 00_nucleo/prompts/compiler/eval/bindings/field_access.md
 M 00_nucleo/prompts/compiler/eval/math.md
 M 00_nucleo/prompts/compiler/stdlib/emoji.md
 M 00_nucleo/prompts/compiler/stdlib/structural/math.md
 M 00_nucleo/prompts/compiler/stdlib/sym.md
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/math.rs
 M 01_core/src/compiler/stdlib/emoji.rs
 M 01_core/src/compiler/stdlib/structural/math.rs
 M 01_core/src/compiler/stdlib/sym.rs
?? 00_nucleo/diagnosticos/typst-p1495-auditoria-catalogo-simbolos.md
?? lab/parity/tools/p1495_symbols_probe.py
```
  `git diff HEAD --stat` nos dois extremos: 12 tracked files changed,
  221 insertions(+), 8 deletions(-); os dois untracked não entram nesse
  total. Os binários usados precedem as edições L0 e não foram rebuildados.
- Vanilla: upstream/main `9f2b6e8715237cb086899a42873660fe744622e8`,
  `lab/typst-original/target/release/typst`, SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Cristalino: `target/release/typst`, SHA-256
  `b692397823a0e85c9c945feda329fb9687eef0cd6f30e91fc77caf99b04f6394`.
  O pin de dependência atual é `codex = 0.3.0` (`Cargo.toml:75`,
  `Cargo.lock:413-418`), checksum crates.io
  `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`.
- O vanilla usa `codex` git `2e43ab90079cf9f9fa670d421c8c38233143b376`
  (`lab/typst-original/Cargo.toml:51`, checkout local confirmado por
  `git rev-parse HEAD`). SHA-256 de `sym.txt` antigo/novo:
  `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16` /
  `60c79540af8ebe590c80d672b55a41f337bc8d61ebd42d9be8879fce9b18f8bc`.
  Os dois `emoji.txt` são byte-idênticos, SHA-256
  `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`.

Comando final, executado com saída 1 esperada (RED), sem alterar o repositório:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/tools/p1495_symbols_probe.py
```

Resultado final: `total=198`, `RED=162`, `EXTENSION=36`, zero
baseline defect. Saída 1 esperada. Os 162 RED são 146 consultas
qualificadas (73 em cada namespace) e 16 consultas bare. Os doze
aliases existem nos três acessos (36), com valor e `repr` golden
estáveis. O warning histórico de `join` ocorre em `sym` e bare;
`math.join` permanece silencioso. Nenhum desses extras recebe
crédito de paridade contra o vanilla novo.
O script exige também o erro esperado do vanilla e compara
warnings em todos os casos de paridade, inclusive os cinco `tack`
des-depreciados. Para bare, exige `text` presente nos dois bodies;
quando o glifo e os warnings convergirem, mas os carriers públicos
`symbol`/`text` de `eval(..., mode: "math").body` divergirem,
reportará `TEXT_MATCH_CARRIER_DIFF`, nunca `MATCH` integral.
O carrier histórico `text` dos aliases bare é golden de extensão.
Ambos os SHAs binários são verificados antes e depois da suíte, para
não misturar builds durante a medição.

## Catálogo: língua medida antes da classificação

`codex/src/modules/sym.txt:765-793,1377-1400` define os novos pais e filhos:

| Pai | Filhos imediatos (cada um em `sym` e `math`) | Variantes adicionais |
|---|---|---|
| `chess` | `king, queen, rook, bishop, knight, pawn` | `.white/.black` para cada peça (12) |
| `keyboard` | `tab, pageup, pagedown, shift, capslock, space, erase, cmd, option, clear, alt, esc, enter` | `tab.r`, `pageup.alt`, `pagedown.alt`, `capslock.pedestal`, `erase.l/.r`, `enter.alt` (7) |
| `ordinal` | `feminine, masculine` | nenhuma |

Assim, são 3 módulos por namespace (6 pais), 21 filhos por namespace (42
descendentes imediatos) e 19 variantes nomeadas por namespace. O vanilla
devolveu `repr` para 43 paths (3 pais + 40 descendentes) por namespace, com pares
`sym`/`math` idênticos; o cristalino falha já no pai. `chess.pawn.black`
retém VS15 (`♟\u{fe0e}`), portanto não reduzir glifos a `char`.

Doze famílias de metadados foram sondadas pelo valor/modifier e `repr`,
não por presença de binding. Cada path vale em `sym` e `math` no vanilla:

| Família | Testemunha RED | Valor vanilla |
|---|---|---|
| eq | `eq.gt.slant` | ⪖ |
| equiv | `equiv.lt` | ⪙ |
| gt | `gt.eq.lt.slant` | ⋛ + VS1 |
| lat | `lat.eq.slant` | ⪭ + VS1 |
| lt | `lt.eq.gt.slant` | ⋚ + VS1 |
| not | `not.inv` | ⌙ |
| prec | `prec.eq.slant` | ≼ |
| smt | `smt.eq.slant` | ⪬ + VS1 |
| succ | `succ.eq.slant` | ≽ |
| tack | `tack.r.double` | ⊩ (cristalino ⊨) |
| tilde | `tilde.lt` | ⪝ |
| trademark | `trademark.mc` | 🅪 |

Os cinco valores `tack.*.double` mudaram no novo pin: `r.double`
⊩/⊨, `r.double.not` ⊮/⊭, `l.double` ⫣/⫤, `t.double` ⫨/⫫ e
`b.double` ⫧/⫪ (vanilla/cristalino). O changelog do codex novo
(`CHANGELOG.md:33-40`) chama-os breaking; as sondas de `str` confirmam
mudança na linguagem, não só em bytes de render.

## Rotas antigas, depreciação e causa

O catálogo antigo (`codex-0.3.0/src/modules/sym.txt:336-343,373-380,650-654`)
contém `join{,.r,.l,.l.r}` = ⨝/⟖/⟕/⟗, `gt.tri{,.eq,.eq.not,.not}` =
⊳/⊵/⋭/⋫ e `lt.tri{,.eq,.eq.not,.not}` = ⊲/⊴/⋬/⋪. O novo
`CHANGELOG.md:42-47` remove esses doze paths. A sonda mostra ausência no
vanilla e presença em `sym`, `math` e bare cristalinos (36 extensões).
`#sym.join` e `$join$` já avisam no cristalino; `#math.join` não.
Remover qualquer path é quebra pública, não reparo inocente de tabela.

Uma troca mecânica de crate criaria assimetria: `sym.rs:610-612` retém
`legacy_sym_lookup`, mas `build_sym_module` em `:638-646` só injeta
`registered`; `structural/math.rs:3123-3131` espelha apenas o módulo.
Assim `#sym.join`/`#math.join` sumiriam enquanto o fallback bare de
`join` sobreviveria; acessos bare `gt.tri` passariam pelo field do
grupo novo (`eval/math.rs:161-164,1493-1496`) e também falhariam.

O novo `sym.txt:1374-1375` deprecia `spacebar` para `keyboard.space`.
Sondas do vanilla: `sym.spacebar`, `math.spacebar` e `$spacebar$` valem
␣ com warning; cristalino P1494 vale ␣ sem warning. O owner
`field_access.rs:618-629` hoje avisa só em módulo chamado `sym` e antes
de resolver o campo. Warnings de variantes são débito P1283 já declarado
em `compiler/stdlib/sym.md`; `SymbolVariant` não transporta mensagem.

## Decisão P1495 e ordem para P1496

Preservar os doze aliases como extensões cristalinas, sem crédito de
paridade, de forma coerente em `sym`, `math` e bare. Essa escolha evita
remoção pública, portanto não abre o gate de confirmação ADR-0127.
Os cinco glifos `tack` existentes são classificados como correção de
entradas de tabela para o novo alvo pinado: sem mudança de path,
assinatura, carrier ou fase, seguem L0 primeiro + RED→GREEN. O rótulo
breaking do crate foi considerado, não descartado; outra promessa
de compatibilidade desses valores refutaria a classificação e exigiria
parar antes do código. Não houve troca de pin, lockfile ou código
produtivo neste passo.

Owners de P1496, na ordem: `compiler/stdlib/sym.md` (pin exato, overlay
restrito de aliases, dados/metadata), `compiler/stdlib/structural/math.md`
(espelho), `compiler/eval/math.md` (bare e precedência),
`compiler/eval/bindings/field_access.md` (warnings nos dois módulos).
`compiler/stdlib/emoji.md` é controle de invariância; `entities/symbol.md`
fica intacto enquanto carrier/algoritmo público não mudar. Nenhum Núcleo
Tekt nem pin compartilhado precisa mudar. Testes P1496: repetir as 198
sondas; os 146 RED qualificados devem virar `MATCH`, os 16 bare
devem ter glifo e warning corretos, mas ficam
`TEXT_MATCH_CARRIER_DIFF` enquanto persistir a divergência
morfológica pré-existente. Os 36 aliases permanecem `EXTENSION`,
com a política histórica de warnings. Adicionar controles
`registered`, `sqrt`, `emoji` e negativos de Dict/módulo
homônimo importado como `sym` ou `math` com field `spacebar`/`join`:
nenhum warning pertence ao nome homônimo sem identidade do namespace
efetivo. Testar ausência sem warning e rodar matriz
`P1489-B-002/003/004` antes de rebaseline. Se surgir necessidade de
alterar contrato público ou remover alias, parar no gate ADR-0127.
