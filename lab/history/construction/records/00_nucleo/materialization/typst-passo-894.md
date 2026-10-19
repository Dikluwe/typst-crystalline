# Passo 894 — triagem de `typst-math-comprehensive-test.typ` (30 secções): 2 crashes + perda sistemática de conteúdo

**Precede este passo**: os prompts/relatórios de P885–P893 (toda a frente de matemática até aqui).
Ler antes de começar, em especial a lição já repetida três vezes sobre comparar PDFs
não-sincronizados (P885 achados 1/4, P891 Parte B) — **este passo tem de evitar o mesmo erro desde
o início**, gerando os dois lados sempre no mesmo momento, do mesmo `.typ`, com hash confirmado.

**Ficheiro-fonte**: `typst-math-comprehensive-test.typ` (30 secções, anexo a este prompt/no
repositório de trabalho — colar hash sha256 no relatório assim que copiado para o local de
trabalho). Só as secções 1–4 foram compiladas e comparadas até agora (por extração de texto, não
confirmado visualmente); as secções 5–30 nunca foram tocadas.

**Achados desta triagem inicial (por extração de texto, NÃO confirmados visualmente — a Fase A
deste passo existe para confirmar ou refutar cada um)**, catalogados abaixo. Tratar cada achado como
hipótese, não como facto, até confirmação visual/byte-level.

---

## Prioridade 0 — dois crashes de compilação (mais graves que qualquer divergência de render)

- **Secção 22, `lr(\]a/b\[)`**: delimitador com escape (`\]`/`\[` como delimitadores literais dentro
  de `lr(...)`, sintaxe pouco comum). Reproduzir isoladamente
  (`$ lr(\]a/b\[) $` num `.typ` mínimo) nos dois binários. Capturar a mensagem de erro/panic
  completa do cristalino. Confirmar se o vanilla também falha com a mesma sintaxe (se falhar nos
  dois, não é bug do cristalino — é sintaxe realmente inválida, e o teste está mal escrito;
  confirmar antes de tratar como achado).
- **Secção 26, funções `bra`/`ket`/`hbar`** (definidas no topo do ficheiro via `#let`, usando
  `math.chevron.l`/`math.chevron.r`/`math.planck`): reproduzir isoladamente as definições + um uso
  simples (`$ bra(phi) ket(psi) $`) nos dois binários. Capturar erro completo. Confirmar se
  `math.planck`, `math.chevron.l`, `math.chevron.r` existem como identificadores válidos no
  cristalino (`01_core/src/rules/math/symbols.rs` ou equivalente) — se um desses três símbolos não
  estiver implementado, é essa a causa directa, não algo mais profundo em `bra`/`ket`.

Para os dois: se forem bugs reais (não sintaxe inválida), **não implementar correcção neste
passo** — só isolar a causa mínima e registar. A correcção fica para um passo seguinte, escopo a
decidir depois de se saber o tamanho do problema.

## Prioridade 1 — perda sistemática de conteúdo (secções 1, 4) — hipótese unificadora a testar

Tabela de sintomas no topo desta mensagem/prompt (9 casos, secções 1 e 4). **Antes de aceitar
qualquer um como real**: gerar os PDFs de `04-math`-equivalente (secções 1 e 4 isoladas) nos dois
binários, no mesmo momento, mesmo `.typ`, hash confirmado — depois render visual a 150dpi, não só
extração de texto (a extração já se sabe não ser fonte fiável sozinha, mas aqui os 9 casos formam
um padrão semanticamente coerente demais para serem só reordenação de extração — precisa de
confirmação visual para ter certeza, não para descartar de ânimo leve).

**Hipótese a testar primeiro** (não confirmada, testar com casos isolados mínimos antes de
generalizar): identificador não vinculado a função, seguido imediatamente de `(`, desambiguado
incorretamente entre "chamada de função" e "justaposição/concatenação". Casos mínimos a isolar:
- `$ n(n+1) $` sozinho — confirmar se `n` desaparece.
- `$ f(x) = f'(x) $` sozinho — confirmar se o `f` antes do `'(x)` desaparece.
- `$ f(x) dif x $` sozinho — confirmar se o `x` depois de `dif` (renderizado `d`) desaparece.
- `$ a^2 + b^2 = c^2 $` sozinho — confirmar se `b^2`/`c^2` perdem o expoente.
- `$ (sin x)/x $` e `$ 1/x $` isolados — confirmar se a fração inteira desaparece.

Se a hipótese de "identificador+parêntese" não explicar todos os 5 casos isolados acima, não forçar
uma causa única — pode haver 2+ mecanismos distintos com sintoma parecido (ex: o caso da fração
`sin(x)/x` não tem `identificador(` nenhum, então não pode ser a mesma causa do caso de `n(n+1)` —
já é sinal de que a hipótese unificadora provavelmente não cobre tudo; confirmar isso explicitamente
e catalogar por mecanismo, não por sintoma).

## Prioridade 2 — mapeamento de caractere errado (secções 1, 4) — mais simples, provavelmente rápido

- `dot` → `·` (U+00B7) no cristalino, deveria ser `⋅` (U+22C5). Localizar em
  `01_core/src/rules/math/symbols.rs` (ou ficheiro equivalente de mapeamento de símbolo→Unicode) a
  entrada de `dot` e corrigir o codepoint.
- `partial` → `∂` (U+2202) no cristalino, vanilla usa `𝜕` (U+1D715, variante itálica matemática).
  Confirmar se isto é intencional (alguns símbolos não têm variante itálica e o upright é o
  correcto) ou se é o mesmo tipo de mapeamento errado do `dot` — não presumir, confirmar contra a
  tabela de `ident_to_unicode`/`shorthand_to_unicode` do vanilla para este símbolo específico.

Estes dois, se confirmados, são candidatos a correcção directa (sem necessidade de Fase A extensa
tipo instrumentação) — é comparar a tabela de mapeamento do cristalino com a do vanilla para estes
símbolos e corrigir a entrada errada.

## Prioridade 3 — secções 5–30, nunca tocadas

Depois de tratar 0-2, compilar as 26 secções restantes (5 a 30) nos dois binários, mesma disciplina
de sincronização, e catalogar divergências por secção, no mesmo formato usado para as secções 1-4
(tabela vanilla vs cristalino, por linha do `.typ`). Não é preciso Fase A completa para todas —
catalogar primeiro, e priorizar Fase A funda só para os casos que parecerem mais graves (perda de
conteúdo > mapeamento de símbolo > espaçamento).

---

## Metodologia obrigatória (repetida de propósito, por já ter falhado 3 vezes neste projeto)

1. Os dois binários compilam o **mesmo ficheiro `.typ`**, **no mesmo momento**, e o relatório regista
   o hash sha256 do `.typ` usado.
2. Nenhuma conclusão de "diverge"/"está certo" sem confirmação visual (render a 150dpi ou
   equivalente) — extração de texto sozinha só serve para triagem inicial, não para veredicto.
3. Working tree: `git status` antes de começar; decidir e registar se há algo pendente de passos
   anteriores.

## Resultado esperado

- Veredicto dos 2 crashes (Prioridade 0): causa mínima isolada, é bug real ou sintaxe inválida.
- Veredicto dos 5 casos isolados de perda de conteúdo (Prioridade 1): confirmados ou refutados por
  visual, e se partilham ou não mecanismo comum — catalogados por causa, não por sintoma.
- Veredicto dos 2 casos de mapeamento de caractere (Prioridade 2): confirmados, com a linha exacta
  no ficheiro de mapeamento a corrigir (correcção pode ser feita neste mesmo passo se for tão
  simples quanto parece, com TDD — não precisa de esperar por outro prompt só para isto).
- Catálogo das secções 5–30 (Prioridade 3), com divergências reportadas por secção, para
  priorização de passos futuros.
- **Nenhuma implementação de correção** para os itens de Prioridade 0 e 1 neste passo — só
  diagnóstico. Prioridade 2 pode fechar neste mesmo passo se a Fase A confirmar que é mesmo só
  corrigir uma entrada de tabela.
