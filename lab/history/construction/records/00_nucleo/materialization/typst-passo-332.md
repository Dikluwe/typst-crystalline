# Tarefa P332 — Experimento da fronteira de extensão (spikes E1/E2/E3) — a decisão do F espera a medição

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P332 (confirmar livre; a versão anterior deste prompt
foi descartada antes de executar).
**Pré-condição**: P331 fechado — dossiê, inventários 1a/1b/1c, rede de
caracterização (+11, suíte 2708), lint 0, baseline 10× (P330). Se não, parar.
**Tipo**: diagnóstico **experimental** — spikes descartáveis + medição
comparativa. **Zero código de produto. Zero decisão de desenho.** Execução
longa autônoma permitida (regras do P331: bloqueio vira pergunta; progresso
em disco; commits por fase; ordem por valor).
**Fontes**: `f-dossie-opcoes-passo-331.md` + inventários 1a/1b/1c;
`medicao-pre-f-passo-318.md` (M1 ~273 propriedades settable no vanilla; M4
juro de adiar); `medicao-pre-f-passo-330.md` (baseline 10×: 0.6518 s ±
0.0057); `lab/typst-original/` (leitura autorizada; nunca importar);
trait `Element` + os 65 módulos `entities/elements/` (o precedente vivo).
**Commits**: "Passo 332 — requisito de extensibilidade (registro)",
"Passo 332 — spikes E1/E2/E3", "Passo 332 — medição comparativa".

---

## Parte 0 — Registro do requisito (decisão do dono, zero código)

Gravar no DEBT 99.E + adendo no dossiê P331 (adendo, não fecho — a decisão
A/B/C/D **reabriu** e espera este experimento):

1. **Extensibilidade é requisito do projeto, e total**: usuários podem
   definir elementos novos que são cidadãos plenos — recebem `#set`,
   `#show`, `query` e renderizam — sem tocar o core.
2. **Dois públicos**: (a) programador Rust (define elementos em código);
   (b) autor typst (pacotes/show rules na linguagem, sem Rust).
3. **Critérios de engenharia da escolha** (do dono): atomização, separação
   de camadas, economia para IA escrever e manter, mantendo a qualidade
   construída. Fidelidade ao vanilla continua **comportamental** (P329).
4. **Implicações registradas**: o alvo de escala é a superfície M1 (~273
   propriedades de paridade final, crescendo incrementalmente); o juro M4
   continua correndo; a Opção A está **rejeitada** (congelaria contra o
   requisito); B/C/D são re-avaliadas sob o requisito **pelo experimento
   deste passo**, não por argumento.

## Parte 1 — Desenho dos candidatos (papel, curto)

Três candidatos de partida — o executor ajusta/funde com o inventário na
mão, registrando o porquê; mínimo 2, máximo 4 spikes:

- **E1 — Fronteira por trait (híbrido estático/dinâmico)**:
  `Content::Dynamic(Arc<dyn Element>)` como variante de extensão; os 6
  matches do hub despacham pelo trait existente; os 65 nativos continuam
  estáticos. Propriedades: os 10 campos nativos fechados + mapa aberto
  (chave de propriedade dinâmica) para elementos/props de usuário.
  `ElementKind`/payload: variante dinâmica por nome/id registrado.
- **E2 — Type-erased à vanilla (C honesta, mínima)**: chain erased
  (`Property`/`Recipe` com `Box<dyn>`), registro de elemento por id,
  resolução por fallback — a estrutura do vanilla em versão mínima
  suficiente para o elemento-brinquedo, para medir o custo real em vez de
  rejeitar por impressão.
- **E3 — Registro aberto por kind (B destravada)**: `ElementKind` extensível
  (id dinâmico) + payload genérico + PropMap aberto tipado-por-chave; sem
  `dyn` no Content (dados, não vtable) — o comportamento de usuário entra
  por dados + funções registradas.

Para cada candidato, 1 página: como entrega os dois públicos (o caminho
Rust e o caminho typst/pacote), como escala até ~273 props, onde mora cada
camada.

## Parte 2 — Spikes (descartáveis, fora do produto)

- **Onde**: `lab/spikes/f-extensao/{e1,e2,e3}/` — crates/árvores isoladas.
  **Nunca** importadas pelo produto; fora do gate de lint de produto
  (registrar a exclusão); apagáveis sem dó. Podem copiar trechos do produto
  para dentro do spike (o contrário é proibido; da quarentena vanilla também
  é proibido).
- **O que cada spike implementa** (o mesmo, para comparar): o
  **elemento-brinquedo de usuário** `callout` (corpo + título + uma
  propriedade própria `tone`) de ponta a ponta: definição fora do core →
  registro → `#set callout(tone: …)` → `#show callout: …` (forma mínima
  que o spike suportar) → `query(callout)` → renderização em texto/frame.
  Mais um doc sintético com N callouts misturados a nativos (para o perf).
- **Profundidade**: mínima para medir — não é produção. Onde o spike
  precisar de um pedaço do pipeline real (eval/layout), pode simular com
  stub declarado; a simulação é registrada como limite da medição.

## Parte 3 — Medição comparativa (critérios fixados ANTES de medir)

Tabela única, os 3+ spikes nas mesmas colunas. Colunas e método:

1. **Atomização**: nº de arquivos e linhas que o *usuário* toca para criar
   o `callout` (sem contar o spike-harness); nº de arquivos do *core* que
   mudariam por elemento novo (alvo: 0).
2. **Separação de camadas**: grafo de dependência do elemento-usuário →
   core (quais módulos ele importa; existe ciclo?); o estilo atravessa
   camada por contrato declarado ou por acoplamento?
3. **Custo-IA**: (a) quantos arquivos/linhas uma IA precisa **ler** para
   escrever o `callout` correto (contexto mínimo: trait? macro? convenções
   implícitas?); (b) presença de raciocínio não-local (macro opaca,
   dispatch invisível, invariantes implícitas) — listar, contar; (c) o
   precedente: os 65 módulos existentes servem de exemplo direto? 
4. **Performance**: doc 10× do baseline **sem** callouts (overhead da
   infraestrutura sobre nativos — comparar com 0.6518 s ± 0.0057 quando o
   spike reaproveitar o pipeline real; quando for stub, medir relativo
   entre spikes e declarar a limitação) + doc sintético **com** N callouts
   (custo do caminho dinâmico). ≥10 execuções, média ± σ, comandos
   registrados.
5. **Escala até M1 (~273)**: o que muda no desenho quando as props vão de
   1 para 273 (nada? tabela cresce? macro por prop?); custo marginal por
   propriedade nova.
6. **Fidelidade comportamental**: C1–C8 aplicáveis + o caminho do `#show`
   — o que o spike demonstra vs o que fica argumentado.
7. **Custo de migração do estado atual**: pelo preditor (largura por grep
   do 1c), quantos sites o produto converteria para adotar cada desenho —
   incluindo o destino das 4 `Set*`, de `Styled` e das 3 folhas
   provisórias em cada candidato.

## Parte 4 — Relatório comparativo + checkpoint (a decisão é do dono)

`f-experimento-extensao-passo-332.md`: a tabela, os spikes (paths), as
limitações de cada medição, e **leitura do executor claramente marcada**
(não vinculativa). **§Perguntas ao dono** numeradas. O passo TERMINA no
checkpoint — a escolha do desenho da fronteira (e, com ela, a forma final
de F-D/F-B) é do dono, na conversa, com a tabela na mão.

---

## Relatório (`typst-passo-332-relatorio.md` + resumo no chat)

Estado por fase; o registro da Parte 0 (onde mora); os candidatos da Parte
1 (resumo 2 linhas cada); os spikes (o que cada um demonstrou e o que
stubou); a tabela da Parte 3 inteira; as perguntas. `git log` (3 commits);
produto intocado (`git status` em `01_core/` limpo); suíte 2708 verde e
lint 0 **no produto** (os spikes ficam fora do gate, registrado). Caveat do
stack (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

A decisão da fronteira (dono, no checkpoint); F-D/F-B (re-desenham depois
da decisão); qualquer código de produto; consertos (B1/B2/B3 mantêm os
registros do P331, destino decide-se com a fronteira); otimizações.
