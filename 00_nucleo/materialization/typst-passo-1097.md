# L0 — Passo 1097: Investigar Generalização dos Resíduos de Kerning/Espaçamento de Classe

**Gate**: nenhum — investigação, sem código alterado.

**Base**: P1096 mediu dois resíduos pequenos (`~0.0025-0.0037pt`) mas
consistentes: kerning vírgula→λ (`+0.00367pt`)/λ→μ (`-0.00366pt`) em
`L(x,λ,μ)`, e espaçamento classe relacional→dígito (`+0.00244` a
`+0.00246pt`) em três ocorrências (`=0`, `≤0`, `=0`). Nenhum dos dois teve
proveniência numérica citada (tabela real, valor bruto em `du`) — só descrito
em prosa. Preocupação levantada: pode ser sintoma pequeno aqui, mas maior
noutro contexto (tamanho de fonte diferente, par de classes diferente).

---

## 1. Obter a proveniência numérica real antes de generalizar

Mesma disciplina desta investigação inteira — não aceitar "kerning
contextual" sem número bruto:

- Para o par vírgula→λ: dump da tabela `GPOS`/`kern` da fonte
  (`NewCMMath-Regular.otf`) para o par de glifos `comma`+`u1D6CC`. Confirmar
  se existe um valor de kern real, e qual, em `du`.
- Para relacional→dígito: localizar a tabela de espaçamento por classe
  matemática no vanilla (`spacing.rs`, já referenciado nesta investigação
  para outros achados) — encontrar a entrada `Rel→Ord`/`Rel→Number` (nomes
  exactos de classe a confirmar) e o valor bruto associado.

## 2. Testar se a magnitude escala com o tamanho de fonte

Se for kerning/espaçamento genuíno em `du`, o valor em `pt` deve escalar
linearmente com `size`. Testar os mesmos pares a pelo menos 2 tamanhos
diferentes (ex.: 11pt e 22pt, ou 11pt e um tamanho de sub/sobrescrito como
7.7pt):

- Se o resíduo em pt dobrar com o tamanho dobrado → confirma proporção `du`
  correcta, aponta para um valor `du` ligeiramente errado no cristalino.
- Se o resíduo **não** escalar (ficar constante em pt independente do
  tamanho) → não é kerning tipográfico normal, é outra coisa (ex.: erro de
  arredondamento numa soma de larguras, não relacionado à fonte).

## 3. Testar se o padrão aparece fora de matemática

Kerning de vírgula é mecanismo de texto normal, não exclusivo de modo
matemático. Testar o mesmo par de glifos (`,` seguido de itálico) em texto
corrido fora de `$...$` — se o resíduo aparecer lá também, a causa é no
shaper/kerning geral (`03_infra/src/shaper.rs`), não específica ao layout
matemático; se só aparecer em modo matemático, é específico a
`math/layout/`.

## 4. Testar outros pares de classe matemática, não só os já vistos

Se a causa for uma tabela de espaçamento por classe (§1, segunda hipótese),
testar pelo menos mais 2-3 pares de classe diferentes dos já vistos
(`Rel→Ord`, `Bin→Ord`, `Ord→Rel`, etc. — nomes reais a confirmar) para saber
se o desvio de `~0.0025pt` é uniforme em toda a tabela (sinal de erro
sistemático numa constante de base, tipo o mesmo tipo de truncamento já
achado em P1093) ou específico a um par.

## Critério de conclusão

- Proveniência numérica bruta obtida para os dois casos do P1096 (§1).
- Escala com tamanho de fonte confirmada ou refutada (§2).
- Presença/ausência fora de contexto matemático confirmada (§3).
- Pelo menos 3 pares de classe adicionais testados (§4), com veredicto sobre
  se o desvio é uniforme (candidato a constante-base truncada) ou disperso
  (candidato a vários pequenos erros independentes, ou não-problema).
