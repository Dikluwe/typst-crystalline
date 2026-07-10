# P680 — Cadeias de import de três ficheiros e dependências em diamante

**Passo:** 680
**Data:** 2026-07-10
**Foco:** cobrir dois casos que P679 deixou por testar: cadeia de três ficheiros sem ciclo (`A→B→C`) e dependência em diamante (`A` importa `B` e `C`, ambos importam `D`).
**Tipo:** Verificação directa + testes automatizados. Sem alteração de lógica (cobertura de P679).
**Commit base:** `934be6e4a — P679: adiciona hash do commit ao relatório`
**ADR-0108 em vigor** (medir antes de decidir). **ADR-0107 em vigor** (paridade com a linguagem, não com a mecânica).

---

## 1. Por que estes dois casos importam

P679 implementou `eval_module_import` e testou import de um nível (`A` importa `B`) e um ciclo simples de dois ficheiros (`A↔B`). Ficaram por verificar:

- **Cadeia de três+ ficheiros sem ciclo** (`A→B→C`): o valor de `C` tem de propagar através de `B` até `A`, sem que nenhum elo seja confundido com ciclo.
- **Dependência em diamante** (`A` importa `B` e `C`; ambos importam `D`): `D` é "visto" duas vezes, em **ramos diferentes**, não em sequência circular. Um mecanismo de detecção de ciclo mal desenhado (que confundisse "já avaliado nalgum ramo anterior" com "activo na cadeia actual") marcaria `D` como ciclo por engano ao ser importado pelo segundo ramo.

---

## 2. Sonda directa (cristalino vs vanilla)

Binários: vanilla `lab/typst-original/target/release/typst` (`typst 0.15.0 (969087ec)`); cristalino `target/debug/typst` (build do commit base, com P679). Texto via `pdftotext -layout`, whitespace normalizado.

### 2.1 Cadeia `A→B→C`

```typst
// p680-c.typ:  #let valor_c = "de C"
// p680-b.typ:  #import "p680-c.typ": valor_c
//              #let valor_b = "de B, com " + valor_c
// p680-a.typ:  #import "p680-b.typ": valor_b
//              #valor_b
```

| | Texto |
|---|---|
| vanilla | `de B, com de C` |
| cristalino | `de B, com de C` |

**Idêntico. Sem erro/warning no cristalino** — a cadeia de três não é tratada como ciclo.

### 2.2 Diamante `A→{B,C}→D`

```typst
// p680-d.typ:           #let valor_d = "de D"
// p680-diamante-b.typ:  #import "p680-d.typ": valor_d
//                       #let valor_b = "B usa " + valor_d
// p680-diamante-c.typ:  #import "p680-d.typ": valor_d
//                       #let valor_c = "C usa " + valor_d
// p680-diamante-a.typ:  #import "p680-diamante-b.typ": valor_b
//                       #import "p680-diamante-c.typ": valor_c
//                       #valor_b #valor_c
```

| | Texto |
|---|---|
| vanilla | `B usa de D C usa de D` |
| cristalino | `B usa de D C usa de D` |

**Idêntico. Sem falso positivo de ciclo** — `D` é importado com sucesso pelos dois ramos (`B` e `C`).

---

## 3. Por que o diamante não é confundido com ciclo (mecânica confirmada)

A detecção de ciclo em P679 usa `engine.route.contains(src_id)` antes de avaliar cada ficheiro importado (`01_core/src/rules/eval/modules.rs`, `eval_module_import`). O ponto decisivo é **como** o `Route` é estendido:

- Em `eval_imported_file`, o frame filho é construído como `Route::extend(engine.route).with_id(src_id)` e atribuído ao `Engine` **local**. O `Engine` do chamador mantém o seu `route` intacto — `Route::extend` produz um novo `Route`; não muta o do chamador (o campo é `Tracked<Route>`, tratado como valor imutável).
- Quando `eval_imported_file` retorna (sucesso ou erro via `?`), o `Engine` local é descartado e o `route` do chamador continua exactamente como estava antes da importação.

Consequência para o diamante:
1. `A` importa `B`: route do filho = `root→A→B`; `B` importa `D`: route do neto = `root→A→B→D`. `D` termina → volta a `B`; `B` termina → volta a `A`. O `route` de `A` nunca conteve `B` ou `D` (esses segmentos viveram só nos engines locais, já descartados).
2. `A` importa `C`: route do filho = `root→A→C` (sem `B`, sem `D`). `C` importa `D`: no momento do teste, `engine.route = root→A→C`, que **não** contém `D`. Logo `contains(D) == false` → não é ciclo. `D` é avaliado de novo, no ramo de `C`.

Ou seja: o mecanismo distingue naturalmente "activo na cadeia actual" (está no `route` do engine em curso) de "já avaliado em ramo anterior" (o segmento desapareceu quando o engine local desse ramo terminou). **Não foi preciso corrigir `Route`/`Route::contains`** — o caso hipotético de falso positivo não se materializa com a implementação de P679.

**Classificação (ADR-0108):** cadeia e diamante são **morfologia** da linguagem (a forma como imports se encadeiam é paridade); o algoritmo `Route`/`Route::contains` é **mecânica** (diverge de propósito do vanilla, P329). A aceitação é ao nível da linguagem: o cristalino produz o mesmo texto e não levanta falso ciclo — o comportamento observável coincide com o vanilla.

---

## 4. Testes automatizados adicionados

Em `01_core/src/rules/eval/tests.rs`, reutilizando o `ImportMockWorld` de P679 (mapa path→`Source` com `FileId` estável; `include_source` por clone):

- `import_cadeia_tres_ficheiros_sem_ciclo` — `A→B→C`; verifica que o binding final é `Value::Str("de B, com de C")` e que o eval não falha.
- `import_diamante_nao_e_ciclo` — `A→{B,C}→D`; verifica `rb = "B usa de D"` e `rc = "C usa de D"`, e que o eval não falha (não levanta `ciclo de importação detectado`).

`ImportMockWorld` suporta cadeia/diamante sem alteração: quando um ficheiro importado (ex.: `B`) faz `#import "p680-c.typ"`, o `engine.world.include_source` resolve `C` pelo mesmo mapa, com o `current_file` do ficheiro importado — a resolução em cadeia funciona porque cada ficheiro é avaliado com o seu próprio `engine.current_file`.

Validação:
- `cargo test --workspace` → **verde** (`3659` passed no core lib — `3657` de P679 + 2 novos; `0 failed` no workspace).
- `crystalline-lint .` → **No violations found**.

---

## 5. Decisão

Ambos os casos funcionam sem problema (§2). Conforme a cláusula de decisão do passo: confirmar e registar como parte da cobertura de P679, **sem código a mudar**. O mecanismo `Route`/`Route::contains` já distingue correctamente os dois conceitos (§3); o cenário de falso positivo de ciclo no diamante não ocorre.

O trabalho efectivo deste passo é a cobertura automatizada (§4) para que estes dois casos não regridam, mais o registo neste relatório.

---

## 6. Critério de fecho do passo

- [x] Cadeia de três ficheiros testada, texto idêntico ao vanilla — §2.1.
- [x] Diamante testado, texto idêntico ao vanilla, sem falso positivo de ciclo — §2.2.
- [x] Nenhum caso falhou → nada a corrigir em `Route` — §3.
- [x] Testes automatizados adicionados para os dois casos — §4.
- [x] Sem regressão em `cargo test --workspace` (verde) — §4.
- [x] `crystalline-lint .` limpo — §4.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p680.md`, com hash do commit — §7/§8.

---

## 7. Proveniência das medições

- **Commit base:** `934be6e4a — P679: adiciona hash do commit ao relatório`.
- **Hora da sonda/validação:** 2026-07-10T15:51:54Z (working tree = commit base + os 2 testes novos de P680; sem alteração de lógica de eval).
- **Vanilla usado:** `lab/typst-original/target/release/typst` — `typst 0.15.0 (969087ec)`.
- **Cristalino usado:** `target/debug/typst` (build do commit base; P679 incluído).
- **Ficheiros alterados (`git diff HEAD --stat`):** `01_core/src/rules/eval/tests.rs` (+57 linhas; 2 testes). Nenhum outro ficheiro de código ou L0 foi tocado.

---

## 8. Hash do commit

`48bec0480 — P680: cobre cadeia A→B→C e diamante em #import (sem código)`
