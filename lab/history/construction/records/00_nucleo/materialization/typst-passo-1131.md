# L0 — Passo 1131: Margem Inferior Colapsa para `5.50pt` Fixo Quando Página Termina em Texto Corrido

**Gate**: `ADR-0127` — mudança de comportamento por defeito. **Prioridade
alta**, per a nota — afecta o caso de uso mais comum (texto explicativo
misturado com matemática), magnitude grande (`20.71pt`), reproduzível
com valor exacto idêntico em 3 ocorrências independentes.

**Base**: nota externa (reteste secções 09/20/27/31/38/43). Duas
correcções confirmadas (`+dif`/largura de página secção 27; `|` secção
9). Achado novo: margem inferior real cai para exactamente `5.50pt` nas
3 secções que terminam em parágrafo de texto corrido (38, 31, 20);
secções que terminam em equação/bloco (27, 43, 09) não têm este
problema — confirma que o gatilho é "último elemento é texto corrido",
não `width/height:auto` em geral (todas as 6 usam auto).

---

## 1. Hipótese forte, já identificada pela nota — confirmar com código real

`5.50pt = 0.5em × 11pt` — valor exacto, não aproximado. A nota já
levanta a hipótese certa: uma constante fixa (`0.5em`, provavelmente
`par.leading` default ou algo semelhante já visto nesta investigação —
`P762`/`P1104` mencionam `leading default 0.65em`, não `0.5em`;
confirmar qual constante exacta é esta, não presumir que é a mesma já
catalogada) está a ser usada como fallback no cálculo de margem inferior
quando a página termina em texto, em vez do descent real da última
linha.

```bash
grep -rn "0\.5" 01_core/src/compiler/layout/mod.rs \
  01_core/src/compiler/layout/word.rs \
  01_core/src/compiler/layout/cursor.rs
```

## 2. Não presumir que é o mesmo mecanismo já corrigido em P1103

P1103 corrigiu `last_block_descent_y` obsoleto quando texto normal segue
um **bloco**. Este achado é sobre a página terminar em texto — pode ser
o caminho **inverso** (não há bloco antes para deixar estado obsoleto;
o problema é que `compute_page_height()` nunca teve, de todo, um
caminho correcto para medir o descent real da última linha de texto
puro, caindo sempre num fallback fixo). Ler o código real de
`compute_page_height` (já citado em P1096, P1103) e confirmar qual dos
dois cenários é.

## 3. Verificar se a página termina em cima de conteúdo real ou de espaço fraco

Confirmar se o parágrafo final tem algum elemento à direita/abaixo que
normalmente contribuiria para o cálculo de altura (ex.: `leading` de uma
linha vazia residual, ou o parágrafo em si sem nenhum bloco a seguir) —
o valor fixo pode vir de um caso especial "sem conteúdo adicional depois
do texto, cair no default" que nunca foi testado com um documento real
(só testado internamente com casos sintéticos que sempre tinham bloco
depois).

## 4. Medição

Reproduzir os 3 casos exactos da nota (secções 38, 31, 20) e confirmar
convergência para a margem real (`26.21pt`/`26.33pt`, per o vanilla).

## 5. Não misturar com o resíduo pequeno da secção 27 (`2.36pt`)

Já catalogado como família recorrente de espaço entre blocos, baixa
prioridade face a este achado — não investigar neste passo.

## Critérios de verificação

1. Margem inferior das secções 38, 31, 20 convergindo para o valor real
   do vanilla (±0.0005pt), não `5.50pt` fixo.
2. Confirmar se a constante encontrada é a mesma já catalogada nesta
   investigação (`0.65em` leading) ou uma nova (`0.5em`, nunca antes
   vista) — registar qual.
3. Secções 27, 43, 09 (já correctas ou com resíduo já catalogado) —
   zero regressão.
4. Re-rodar P1086-1130 — zero regressão.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1: constante `0.5em` localizada com arquivo+linha real, não
  presumida a partir do valor numérico sozinho.
- §2: relação (ou não) com o mecanismo do P1103 esclarecida.
- §3: cenário de "sem conteúdo depois do texto" confirmado como o
  gatilho, ou refutado com outra explicação real.
