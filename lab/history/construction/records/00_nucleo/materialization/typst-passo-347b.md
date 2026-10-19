# Passo 347b — investigação de intenção: a terminação de recursão de `#show` é semântica da linguagem ou acidente da implementação?

> **A pergunta.** A Fase A do P347 mediu que o vanilla termina recursão de `#show` por
> **identidade de instância** (devolver `it` guardado → termina; devolver conteúdo
> fresco `[= Z]` → recursa até o teto → erro). Antes de escolher γ/β/α, precisamos saber
> **se a linguagem quis isso** (semântica intencional → paridade exige reproduzir) **ou
> se a implementação gerou** (consequência do mecanismo anti-loop → o vanilla está "meio
> quebrado" e divergir pode ser mais fiel à intenção). Este passo **mede intenção, não
> comportamento**, e **não escolhe** γ/β/α — entrega o veredito com evidência.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P347b (confirmar livre).
**Pré-condição**: P347 Fase A parada na TRAVA (M3 = multi-passe; relatório
`typst-passo-347-relatorio.md`). HEAD pós-P346; P345 commitado (`3ebb397fb`). Suíte
**2723** / **3242**, lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **investigação / medição de intenção** — content-preserving estrito: **zero
código de produção, zero teste, zero `.rs`/`.toml` tocado**. Termina na **TRAVA** com
o veredito, para a decisão do dono. **NÃO escolher γ/β/α**; **não** escrever a recursão.
**Objetivo**: responder, com evidência da fonte, **"a linguagem quis ou a implementação
gerou"** a regra de terminação por identidade de instância. Saídas possíveis do
veredito: **QUIS** (a distinção `it`-vs-fresco é promessa de design da linguagem) /
**GEROU** (é consequência de um mecanismo anti-loop, sem promessa) / **AMBÍGUO** (a
intenção não está declarada em lugar acessível).
**Fontes**: `lab/typst-original/` (a fonte do vanilla: `typst-realize`, `typst-library`,
comentários, nomes, testes) — a fonte primária acessível. Doc pública do Typst e
histórico (PRs/commits) do repositório do vanilla — **se** alcançáveis pelo ambiente;
se não, marcar como lacuna a buscar por fora. Relatórios P347 (M1–M4), P341b (o teto-64,
o guard por-regra do cristalino), ADR-0107 (o critério: paridade é com a linguagem; a
mecânica diverge).

---

## O que distingue "quis" de "gerou" (os predicados da investigação)

A intenção da linguagem é o que ela **promete** (doc, design, o que autores de show
rules assumem). A mecânica é o que a implementação **faz** para entregar. O veredito
sai de procurar a distinção `it`-guardado-termina / fresco-recursa como **promessa** vs
como **comportamento sem promessa**.

---

## Fase A — a investigação (sem código; evidência com `file:line` / referência)

### I1 — a fonte do guard e do teto (intenção no código)
Em `lab/typst-original/` (`typst-realize/src/lib.rs` e vizinhos):
- O guard por recipe-index (`output.guarded(index)` `~:366`; `is_guarded` `~:471-473`)
  e o teto (`MAX_SHOW_RULE_DEPTH` / `check_show_depth` / `Route`): **como os comentários
  e os nomes os descrevem?** "Backstop contra loop infinito / stack overflow" → indício
  de **mecânica**. "O modelo de realização é um fixpoint; o guard é como a recursão
  converge por design" → indício de **semântica**. Citar o texto dos comentários,
  `file:line`.
- O guard segue a **instância** ou o **conteúdo**? Se segue a instância (o `Arc`/o nó
  carrega o recipe-index), a terminação é uma propriedade de identidade — o que reforça
  que `it`-vs-fresco é estrutural do mecanismo. Registrar como o guard é anexado.

### I2 — a mensagem de erro (aviso ou semântica)
A mensagem `"maximum show rule depth exceeded"` + o hint `"maybe a show rule matches its
own output"`: o **texto** trata isso como **erro do usuário a evitar** (mecânica de
proteção: "você fez algo que não devia") ou como **uma semântica que o usuário deveria
conhecer** (design)? Um hint que diz "talvez você tenha feito X por engano" é aviso de
mecânica; uma doc que diz "para parar a recursão, devolva `it`" é semântica. Citar o
texto exato (`file:line`).

### I3 — os testes do vanilla (intenção revelada por teste)
Os testes do vanilla que exercitam recursão de `#show` (procurar em `lab/.../tests/`):
- Há um teste que **afirma** que devolver `it` termina e devolver fresco recursa, como
  **comportamento desejado**? Um teste que asserta isso de propósito é forte indício de
  **semântica querida** (alguém o protegeu).
- Há um teste que só verifica "não estoura a stack / erra no teto" sem afirmar a
  distinção? Indício de **mecânica** (protege-se o limite, não a distinção).
- Citar os testes encontrados, `file:line`, e o que cada um afirma.

### I4 — casos de borda que separam as hipóteses (medir no binário do vanilla)
Compilar no vanilla (binário do P341) casos que "intenção" e "acidente" previriam
diferente:
- `it` **modificado** (devolver `it` com um campo alterado, não a instância pura):
  termina ou recursa? Se a distinção for "instância exata" → mecânica de identidade; se
  for mais sutil (semântica de "é o mesmo elemento conceitual") → possível design.
- output que é o **mesmo tipo mas claramente novo** vs output que **referencia `it`**:
  o vanilla distingue por proveniência ou por forma?
- Registrar cada caso: `.typ`, saída do vanilla, e o que ele revela sobre o critério
  real de terminação.

### I5 — situar contra outros sistemas (opcional, só se a fonte trouxer)
Se a fonte/comentários do vanilla referenciarem o modelo de realização de outros
sistemas (ou se houver design notes no repo), registrar se o `it`-vs-fresco é uma
escolha de domínio conhecida ou uma idiossincrasia. **Não** pesquisar fora se exigir a
web — marcar como lacuna (I6).

### I6 — a lacuna explícita (o que exige doc/histórico online)
O ambiente tem a **fonte** do vanilla, mas talvez **não** a doc pública nem os PRs/
commits. Marcar explicitamente o que ficaria por confirmar fora:
- a doc oficial do Typst sobre show rules e recursão (promete terminação? descreve
  `it`-vs-fresco?);
- o PR/commit que introduziu o guard/teto (a mensagem do autor é a intenção mais direta).
Listar essas lacunas com a pergunta exata, para o dono ou o agente externo buscarem.

---

## TRAVA — o veredito (parar; não escolher γ/β/α)

Emitir o veredito com a evidência de I1–I4 (+I5/I6):
- **QUIS** — a distinção `it`-vs-fresco é promessa de design (evidência: comentário/
  teste/doc que a afirma como intenção). → implica que a paridade a exige; a decisão
  passa a ser **γ vs β** (fatiar ou multi-passe agora), **não** α.
- **GEROU** — é consequência de um mecanismo anti-loop sem promessa (evidência: guard/
  teto descritos como proteção; testes só verificam o limite; nenhuma afirmação da
  distinção). → implica que reproduzir byte a byte é copiar acidente; **α** (terminação
  mais limpa, ex.: ponto-fixo/morfologia) vira defensável e talvez **preferível**, com a
  divergência do vanilla registrada como consciente e justificada (ADR-0107: a mecânica
  do vanilla não é a linguagem).
- **AMBÍGUO** — a intenção não está declarada onde se alcança. → o critério passa a ser
  o **norte do cristalino** (código que se explica, sem acidente herdado): a forma mais
  limpa ganha por ter intenção **nossa** clara, com a divergência do vanilla registrada
  como "o vanilla não declarou intenção aqui". E I6 lista o que confirmaria fora.

**Parar.** A escolha γ/β/α é do dono, **com** o veredito na mão — não deste passo.

---

## Verificação (gates)
```
content-preserving: zero código de produção, zero teste, zero .rs/.toml tocado.
  Suíte 2723 / 3242 intacta (não re-rodada — nada de código). lab só lido/executado.
lint: crystalline-lint . = 0/0.
evidência: cada indício com file:line (fonte) ou o caso .typ + saída do vanilla
  (binário do P341). Sem "~"; sem inferir intenção de comportamento sem dizer que é
  inferência.
lacuna: I6 lista explicitamente o que exige doc/histórico online fora do alcance.
fronteira: o passo NÃO escolhe γ/β/α; entrega o veredito e para.
```

---

## O que NÃO fazer
- **Não escrever a recursão** nem nenhum código — é investigação.
- **Não escolher γ/β/α** — o veredito informa a escolha do dono; não a faz.
- **Não inferir intenção do comportamento sem marcar como inferência** — "o vanilla faz
  X" não é "o vanilla quis X". A distinção é o objeto do passo.
- **Não pesquisar na web** se o ambiente não permitir — marcar como lacuna (I6), não
  inventar a doc.
- **Não tratar AMBÍGUO como fracasso** — é um resultado legítimo que muda o critério
  (passa para o norte do cristalino).

---

## Relatório (`typst-passo-347b-relatorio.md`)
- I1: o guard/teto na fonte — comentários, nomes, instância-vs-conteúdo (`file:line`).
- I2: a mensagem de erro — aviso de mecânica ou semântica (texto exato).
- I3: os testes do vanilla — afirmam a distinção como desejada, ou só o limite?
- I4: os casos de borda no binário — `it` modificado, fresco-vs-referência (saídas).
- I5/I6: situação contra outros sistemas (se a fonte trouxe); a **lacuna** do que exige
  doc/histórico online, com a pergunta exata.
- **VEREDITO**: QUIS / GEROU / AMBÍGUO, com a evidência que o sustenta e o que ele
  implica para γ/β/α (sem escolher).
- **Mapa de filtro (campo):** o lugar lógico — "antes de reproduzir a terminação de
  recursão, o projeto separou a intenção da linguagem do acidente da implementação; é a
  ADR-0107 aplicada à terminação (paridade é com a linguagem, e a linguagem é o que ela
  promete, não o que a implementação calhou de fazer)" — com o rastro (P347 Fase A mediu
  o comportamento por instância; P347b mede a intenção; a escolha γ/β/α vem depois).
- Item: `content→elements` aponta para o Marco G (P346); não volta como órfão.
```
