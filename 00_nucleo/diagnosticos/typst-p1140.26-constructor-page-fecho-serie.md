# P1140.26 — Constructor público `page` e decisão da série

**Data:** 2026-08-24  
**Estado do passo:** fechado  
**Estado da série P1140:** aberta

## Resultado

`page` e `std.page` são agora funções públicas equivalentes. Ambas constroem
`Content::PageRun`, aceitam `paper` posicional opcional, body posicional
obrigatório e os 18 deltas tipados já suportados por `#set page`. O constructor
isola a configuração, conserva body vazio e restaura a configuração exterior.

O antigo caminho P335 não foi restaurado: nenhum `Content::SetPage` isolado é
produzido. O owner de aplicação/restauração permanece
`compiler/layout/page_run.rs`.

## RED → GREEN

O primeiro filtro `p1140_26` teve 2 falhas e 1 teste de rejeição já verde:
`page`/`std.page` eram desconhecidos e o constructor completo não compilava.

Depois da implementação, uma probe de morfologia revelou um segundo RED:

```text
cristalino anterior: repr(page([x])) -> "[x]"
vanilla ratificado:  repr(page([x])) -> sequência delimitada
```

O L0 foi corrigido antes do código. O GREEN final executou 6 testes, todos
aprovados, cobrindo bindings, argumentos, erros, body vazio,
isolamento/restauração e `repr` simples/estilizado.

## Matriz de argumentos

| Grupo | Transporte |
|---|---|
| `paper`, `flipped`, `binding`, `width`, `height`, `margin` | `PageRunElem` → geometria lexical |
| `bleed`, `fill`, `background`, `foreground` | `PageRunElem` → canvas lexical |
| `numbering`, `number-align`, `header`, `header-ascent`, `footer`, `footer-descent` | `PageRunElem` → running matter |
| `supplement` | `PageRunElem` → snapshot/store de página |
| `columns` | `PageRunElem` → configuração local de colunas |
| `body` | conteúdo obrigatório delimitado pelo page-run |

Named desconhecido, tipo inválido, body ausente, `body:` named e posicionais
excedentes são rejeitados. Nenhum named reconhecido é aceito como no-op.

## Rebaseline diferencial

Artefatos:

- `superficie-linguagem-p1140.26.json`;
- `superficie-linguagem-p1140.26-probes.json`.

Inventário estrutural final:

| Classe | Contagem |
|---|---:|
| MATCH | 799 |
| UNVERIFIED_METADATA | 178 |
| MISSING_BINDING | 2 |
| MISSING_MEMBER | 1.155 |
| EXTRA_BINDING | 45 |

O cristalino forneceu 1.022 entradas e o vanilla 2.134. `page` deixou de ser
`MISSING_BINDING`; sua assinatura continua `UNVERIFIED_METADATA` porque o
inventário cristalino não representa metadados genéricos de parâmetros.

As probes passaram de 16/28 antes deste passo para 18/29. As duas provas novas
e relevantes coincidem exatamente:

```text
repr(type(page))     -> "function"
repr(type(std.page)) -> "function"
```

## Gates

- `cargo test -p typst-core p1140_26 -- --nocapture`: 6 passed, 0 failed.
- `cargo test --workspace --quiet -- --test-threads=1`: suites com 5.199,
  835, 53, 2, 55 e 2 testes passaram; zero falhas; 3 testes documentais
  permaneceram ignorados.
- `cargo build --workspace --quiet`: exit 0.
- `cargo build --release --workspace --quiet`: exit 0.
- `crystalline-lint .`: exit 0, sem violations.
- `git diff --check`: exit 0.

## Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`.
- Estado: working tree não commitado.
- Hora final: `2026-08-24T17:51:59-03:00`.
- `git diff HEAD --stat`: `127 files changed, 4118 insertions(+), 655 deletions(-)`.
- Vanilla: upstream/main ratificado `a51e02804`.
- Inventário gerado em `2026-08-24T20:51:44+00:00`.

O stat cobre a working tree acumulada da série; não atribui todos os 127
ficheiros a P1140.26.

Hashes SHA-256 finais dos L0 tocados:

- `compiler/stdlib/layout.md`:
  `cac14cd48edcfa37b9679748acfe13cf103334e3141b091993c86bdcfd25566e`;
- `compiler/eval.md`:
  `43676254270689d516a1835fb9a66fd50108651279cfbe2b9c6583697fe6a71a`;
- `entities/elements/page_run.md`:
  `964774c6dc47e305b1771428711c145315629ecf1f53cfb839f2649abe61a245`;
- `compiler/stdlib/foundations/repr.md`:
  `6ee8ea424c08e004eb205b9289a4628179b37ae0d7fc8ad425f2f8f269f67a32`;
- `entities/content.md`:
  `fae525a519957d4f67b67da8e8cf8ad625f0627f7d1ca56bca6e4794c45dcb7b`.

## Por que a série P1140 permanece aberta

O constructor de página está fechado, mas o rebaseline ainda mede 11 probes
divergentes:

- globais: `html` e `path`;
- membros: `array.all`, `str.clusters`, `color.black`, `gradient.kind`,
  `datetime.day`, `int.bit-and`, `counter.get` e `content.fields`;
- alias: `emoji.heart`.

`html` pertence à frente feature-gated separada. `path` exige entidade/tipo
público próprio. Os membros devem continuar atomizados por owner. Há ainda uma
limitação interna da frente page: o vanilla documenta numbering por função,
enquanto `PageRunElem.numbering` e `PageConfig.numbering` continuam restritos a
string/none. Isso exige contrato próprio de callback e não foi aceito como
no-op neste passo.

Portanto P1140.26 fecha, mas não há base empírica para declarar toda a série
P1140 encerrada.
