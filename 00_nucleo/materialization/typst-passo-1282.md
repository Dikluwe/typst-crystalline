# Passo 1282 — Regerar o inventário bilateral da superfície pública

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Substituir o catálogo histórico por uma enumeração nova e bilateral da superfície de
linguagem do vanilla ratificado e do cristalino no mesmo estado de features.

## Pré-condição

Passo 1281 fechado: harness executável, flags simétricas e proveniência reproduzível.

## Escopo

1. Enumerar bindings, módulos, tipos, membros, símbolos, kinds e metadados acessíveis.
2. Executar separadamente o perfil padrão e o perfil com HTML ativado.
3. Produzir conjuntos `MATCH`, `MISSING_BINDING`, `MISSING_MEMBER`, `WRONG_KIND`,
   `UNVERIFIED_METADATA` e `EXTRA_BINDING`.
4. Amostrar mecanicamente cada classe e confirmar por sondas de runtime que o
   classificador não confunde método de instância, membro estático e feature desativada.
5. Agrupar lacunas por família sem transformar contagem de paths em percentagem de
   paridade da linguagem.

## Disciplina de medição

- Toda contagem registra commit, estado da árvore, hora, comando, feature set e SHA-256
  dos binários.
- Medir antes de classificar, conforme ADR-0108.
- Marcar inferências e a evidência que poderia refutá-las.
- As 45 extensões cristalinas anteriormente observadas são objeto de adjudicação de
  compatibilidade, não crédito automático de paridade nem remoção automática.

## Gates

- Este passo é diagnóstico e não autoriza implementar bindings.
- Se o enumerador exigir mudança de produto, auditar/atualizar o L0 proprietário antes.
- Não ler `00_nucleo/context/` nem outros passos de materialização para preencher
  resultados; a fonte é a execução bilateral atual.

## Critério de fechamento

- Inventário novo reproduzível para vanilla e cristalino.
- HTML não aparece como lacuna por assimetria de feature.
- Cada grupo prioritário possui exemplos RED confirmados por runtime.
- Diagnóstico perene gravado em `00_nucleo/diagnosticos/` com proveniência completa.

## Entrega ao Passo 1283

Fornecer as listas confirmadas de `math`, `sym` e `emoji`, separando símbolos, funções,
módulos, aliases e variantes.
