# Tarefa P318 — Lote 3 (lista/termos) pelo modelo + Medições pré-F (PropMap/StyleChain)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P318 (confirmar livre; 314–317 ocupados, 315 reaproveitado
pelo P317-c).
**Pré-condição dura**: a **Fase B do Lote 2 (P317) fechada** — relatório
existente, `crystalline-lint .` = 0, suíte verde. Se o Lote 2 não fechou,
**parar e reportar** (este passo não corre por cima de lote aberto).
**Tipo**: (Parte 1) Lote 3 da migração D — **instância do modelo**, sem desenho
novo + (Parte 2) medições preparatórias da decisão F — **diagnóstico-primeiro,
zero código de produto, zero decisão**.
**Fontes**: `00_nucleo/modelo-lote-migracao-d.md` (a receita), tabela de
largura do P317, ADR-0105 (F como destino com o DEBT 99.E), DEBT da StyleChain,
DEBT-58 (primitivos), `diagnostico-modelo-elemento-passo-313.md` §4.
**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` não
autorizadas. `lab/typst-original/` (quarentena) é **fonte de leitura
autorizada** para a Parte 2 (medir o vanilla; nunca importar dele).
**Commits**: "Passo 318 — lote 3" e "Passo 318 — medições pré-F" (isoláveis).

---

## Parte 1 — Lote 3, instanciando o modelo

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 3`
- `LOTE` (proposta do dono, a confirmar no checkpoint da Fase A; ordem por
  largura crescente da tabela P317):
  **`EnumItem`(4) · `Link`(4) · `ListItem`(6) · `TermItem`(6) · `Terms`(11)**
  — a família de lista/termos + Link, todos element-shaped (campos próprios),
  ~31 sites totais; lote barato que valida o preditor numa família nova.
- `NOTAS_FAMÍLIA`:
  - Itens de lista carregam corpo (`Box<Content>` → desboxar para `Content`
    no `NomeElem`, convenção do Lote 2) e campos próprios (número do
    EnumItem, termo+descrição do TermItem); `map_content` desce no corpo.
  - **Não incluir** as variantes `Set*` (`SetFigureNumbering`(5),
    `SetEquationNumbering`(16), `SetPage`(8), `SetHeadingNumbering`(62)) —
    são marcadores de set-rule, a superfície da StyleChain: o destino delas
    depende da decisão F (Parte 2 mede; decidir agora seria desenhar o F por
    acidente). Registrar a exclusão no relatório com esta razão.
  - Verificar locatabilidade de cada uma (esperado: nenhuma; se alguma for,
    precedente Heading).

Tudo o mais — Fase A, checkpoint humano, Fase B em ordem crescente,
validação, medições, proposta do Lote 4 — **como está no modelo**, sem
repetir aqui. Lembrete único (regra do modelo): se precisar editar prompt
grosso, fatiar primeiro.

---

## Parte 2 — Medições pré-F (alimentam a decisão futura; não a tomam)

Saída: `00_nucleo/diagnosticos/medicao-pre-f-passo-318.md` — números e
comandos, **sem recomendação**. O diagnóstico de decisão do F (passo futuro,
junto do DEBT 99.E) consome este documento. Quatro medições:

### M1 — O tamanho da superfície de propriedades (o que a PropMap teria que carregar)

- **No vanilla** (`lab/typst-original`): contar os campos "settable" que a
  StyleChain de referência resolve — nº de elementos `#[elem]` e nº total de
  campos com default/settable (grep nos derives de `typst-macros` uso:
  `#[elem]`, atributos de campo; registrar o método e a imprecisão dele).
  Este é o teto da superfície.
- **No cristalino hoje**: as propriedades hardcoded em `Style`/`StyleDelta`
  (o 313 §4 contou 10 — confirmar e listar); as variantes `Set*` do
  `Content` (listar, com largura de uso da tabela P317); os usos do wrapper
  `Content::Styled` (largura 58 — onde, por camada). Este é o piso atual.
- O delta teto−piso é o tamanho honesto do que o F/99.E teria pela frente.

### M2 — Os sites que o F converteria (o custo da trava de verificação)

Contar os sites onde o cristalino lê propriedade/campo de elemento de forma
que o F trocaria por lookup: matches sobre `Style`/`StyleDelta`, acessos a
campos de `*Elem` a partir de `rules/` (agora mensurável: os módulos
`elements/` existem desde P316/P317 — contar acessos externos a campos por
elemento migrado). Este número dimensiona a regra de lint / o
teste-varre-tabela que a trava da ADR-0105 exige antes do F.

### M3 — Baseline de performance do caminho quente (o antes do F)

O F troca campo de struct por lookup em PropMap — o custo disso só será
avaliável com um **antes** medido. Estabelecer baseline reproduzível:
tempo de `eval`+`layout` sobre um corpo fixo (usar os fixtures de teste
maiores que existirem; se não houver bench, medir via teste cronometrado
marcado `#[ignore]` ou `hyperfine` no binário, registrando corpo, comando e
máquina). **Três execuções, mediana.** Guardar o corpo usado (path no repo)
para a medição do "depois" ser comparável. Não otimizar nada.

### M4 — O censo da necessidade de scoping (o valor do F)

Do DEBT da StyleChain e do código: listar as features que hoje divergem do
vanilla por falta de scoping léxico (numbering_active via StateRegistry;
set rules globais; as `Set*` do M1) e **contar os testes/fixtures que
exercem o comportamento divergente** (quantos testes fixam o comportamento
global atual — é o custo de migração comportamental do F, que o 99.E vai
pagar). Sem juízo de valor; contagem e lista.

---

## Relatório (`typst-passo-318-relatorio.md` + resumo no chat)

- Parte 1: o que o modelo manda (medições vs preditor; `content.rs`
  antes/depois; suíte; proposta do Lote 4 derivada da tabela).
- Parte 2: os 4 números-síntese (teto/piso da superfície; sites a converter;
  baseline de performance com corpo e comando; censo de scoping) e o path do
  documento de medição.
- A exclusão das `Set*` registrada com a razão.
- `git log --oneline` dos dois commits; lint 0; `git status` limpo.

## Fora de escopo

- Decidir o F ou o destino das `Set*` (a Parte 2 mede; a decisão é do passo
  do 99.E, com diagnóstico próprio).
- DEBT-58 (primitivos — gatilho não disparou).
- Lotes 4+ (instância do modelo, decisão por lote).
- Qualquer otimização sugerida pelo baseline M3 (medir ≠ mexer).
